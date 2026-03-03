pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2136396u32;
pub const PC_MAX: u32 = 2139352u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x0020994c,
        block_0x00209950,
        block_0x00209964,
        block_0x00209978,
        block_0x002099c8,
        block_0x002099dc,
        block_0x002099e0,
        block_0x002099ec,
        block_0x00209a3c,
        block_0x00209a50,
        block_0x00209a64,
        block_0x00209a70,
        block_0x00209a84,
        block_0x00209a8c,
        block_0x00209a98,
        block_0x00209ac0,
        block_0x00209ac8,
        block_0x00209ad0,
        block_0x00209ae0,
        block_0x00209b08,
        block_0x00209b0c,
        block_0x00209b1c,
        block_0x00209b28,
        block_0x00209b68,
        block_0x00209b88,
        block_0x00209b94,
        block_0x00209b98,
        block_0x00209bb0,
        block_0x00209bb8,
        block_0x00209bc0,
        block_0x00209bc4,
        block_0x00209bd0,
        block_0x00209bd4,
        block_0x00209bdc,
        block_0x00209be0,
        block_0x00209bec,
        block_0x00209bfc,
        block_0x00209c04,
        block_0x00209c0c,
        block_0x00209c1c,
        block_0x00209c30,
        block_0x00209c34,
        block_0x00209c38,
        block_0x00209c3c,
        block_0x00209c44,
        block_0x00209c48,
        block_0x00209c4c,
        block_0x00209c5c,
        block_0x00209c90,
        block_0x00209ca4,
        block_0x00209cd8,
        block_0x00209d10,
        block_0x00209d28,
        block_0x00209db4,
        block_0x00209dc8,
        block_0x00209dd4,
        block_0x00209de8,
        block_0x00209dfc,
        block_0x00209e44,
        block_0x00209e58,
        block_0x00209e60,
        block_0x00209fb4,
        block_0x00209fc4,
        block_0x00209fd4,
        block_0x00209fe4,
        block_0x00209ff4,
        block_0x0020a004,
        block_0x0020a014,
        block_0x0020a024,
        block_0x0020a030,
        block_0x0020a080,
        block_0x0020a0b8,
        block_0x0020a0f0,
        block_0x0020a128,
        block_0x0020a160,
        block_0x0020a198,
        block_0x0020a1d0,
        block_0x0020a208,
        block_0x0020a214,
        block_0x0020a230,
        block_0x0020a240,
        block_0x0020a250,
        block_0x0020a278,
        block_0x0020a280,
        block_0x0020a290,
        block_0x0020a2a8,
        block_0x0020a2bc,
        block_0x0020a2c4,
        block_0x0020a2c8,
        block_0x0020a2e0,
        block_0x0020a310,
        block_0x0020a314,
        block_0x0020a328,
        block_0x0020a350,
        block_0x0020a364,
        block_0x0020a368,
        block_0x0020a388,
        block_0x0020a398,
        block_0x0020a3a0,
        block_0x0020a3a8,
        block_0x0020a3b0,
        block_0x0020a3b4,
        block_0x0020a3d0,
        block_0x0020a3dc,
        block_0x0020a3e4,
        block_0x0020a3e8,
        block_0x0020a3fc,
        block_0x0020a414,
        block_0x0020a424,
        block_0x0020a458,
        block_0x0020a470,
        block_0x0020a4a4,
        block_0x0020a4ac,
        block_0x0020a4d8,
    ];
    const IDX: [u16; 740usize] = [
        1u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16,
        0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16,
        10u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16,
        13u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 16u16, 0u16, 17u16, 0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 21u16, 0u16, 0u16, 0u16, 22u16,
        0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        25u16, 0u16, 0u16, 26u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16,
        29u16, 0u16, 30u16, 31u16, 0u16, 0u16, 32u16, 33u16, 0u16, 34u16, 35u16, 0u16,
        0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 38u16, 0u16, 39u16, 0u16, 0u16, 0u16,
        40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 42u16, 43u16, 44u16, 0u16, 45u16, 46u16,
        47u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16,
        0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16,
        0u16, 60u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16,
        0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16,
        0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        80u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 95u16,
        96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 99u16, 0u16, 100u16, 0u16, 101u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 103u16, 0u16, 0u16, 104u16, 0u16, 105u16, 106u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 109u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 113u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 114u16,
    ];
    if pc < 2136396u32 || pc > 2139352u32 {
        return None;
    }
    let word_offset = ((pc - 2136396u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020994c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136400u32;
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
pub fn block_0x00209950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2136404u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2136408u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2136412u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2136416u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2136420u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00209964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 11usize, 12u32, 2136424u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2136428u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2136432u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2136436u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2136440u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00209978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2136444u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2136448u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2136452u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2136456u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136460u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2136464u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2136468u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136472u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2136476u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2136480u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136484u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2136488u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2136492u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136496u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2136500u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2136504u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2136508u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2136512u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2136516u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2136544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099e0));
    } else {
        emu.pc = 2136520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099c8));
    }
}
#[inline(always)]
pub fn block_0x002099c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2136524u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2136528u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2136532u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2136536u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a50));
    } else {
        emu.pc = 2136540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002099dc));
    }
}
#[inline(always)]
pub fn block_0x002099dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2136544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209a64));
}
#[inline(always)]
pub fn block_0x002099e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2136548u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2136552u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2136556u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002099ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2136560u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2136564u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2136568u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2136572u32)?;
    let a = 0u32.wrapping_add(4151074816u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136576u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965826u32, 2136580u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2136584u32);
    let a = 0u32.wrapping_add(228253696u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136588u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 203u32, 2136592u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2136596u32);
    let a = 0u32.wrapping_add(1618087936u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136600u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1443u32, 2136604u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2136608u32);
    let a = 0u32.wrapping_add(4257644544u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2136612u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 861u32, 2136616u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2136620u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2136624u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2136628u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2136632u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2136656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a50));
    } else {
        emu.pc = 2136636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a3c));
    }
}
#[inline(always)]
pub fn block_0x00209a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2136640u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2136644u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2136648u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2136652u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a64));
    } else {
        emu.pc = 2136656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a50));
    }
}
#[inline(always)]
pub fn block_0x00209a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2136660u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2136664u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2136668u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2136672u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136676u32;
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
pub fn block_0x00209a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2136680u32)?;
    emu.apc_no_count(1usize, 2136680u32, 4096u32, 2136684u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2136692u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2136696u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2136700u32)?;
    emu.apc_no_count(1usize, 2136700u32, 0u32, 2136704u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136708u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2136708u32, 69632u32, 2136712u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136716u32;
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
pub fn block_0x00209a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 0u32, 2136720u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2136724u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2136840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b08));
    } else {
        emu.pc = 2136728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209a98));
    }
}
#[inline]
pub fn block_0x00209a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2136732u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2136736u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2136740u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2136744u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2136748u32)?;
    emu.lw_no_count(8usize, 10usize, 4u32, 2136752u32)?;
    emu.lw_no_count(18usize, 8usize, 4u32, 2136756u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2136760u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2136764u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ac8));
    } else {
        emu.pc = 2136768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ac0));
    }
}
#[inline(always)]
pub fn block_0x00209ac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2136772u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2136776u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00209ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2136780u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ae0));
    } else {
        emu.pc = 2136784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ad0));
    }
}
#[inline(always)]
pub fn block_0x00209ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2136788u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2136792u32);
    emu.apc_no_count(1usize, 2136792u32, 4294934528u32, 2136796u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2136804u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2136808u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2136812u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2136816u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2136820u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2136824u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2136828u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2136832u32);
    emu.apc_no_count(6usize, 2136832u32, 4294934528u32, 2136836u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2136840u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136844u32;
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
pub fn block_0x00209b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2136848u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2136852u32)?;
    emu.apc_no_count(1usize, 2136852u32, 4096u32, 2136856u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2136864u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2136868u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136872u32;
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
pub fn block_0x00209b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2136876u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2136880u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2136884u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2136888u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2136892u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2136896u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2136900u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2136904u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2136908u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2136912u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2136916u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2136920u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2136924u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2136928u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2136932u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2137180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c5c));
    } else {
        emu.pc = 2136936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b68));
    }
}
#[inline(always)]
pub fn block_0x00209b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2136940u32);
    emu.adi_no_count(21usize, 0usize, 4u32, 2136944u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2136948u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294967024u32, 2136952u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2136956u32);
    emu.adi_no_count(23usize, 0usize, 35u32, 2136960u32);
    emu.adi_no_count(24usize, 0usize, 2u32, 2136964u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2136968u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209b98));
}
#[inline(always)]
pub fn block_0x00209b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2136972u32)?;
    emu.lbu_no_count(11usize, 11usize, 8u32, 2136976u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c4c));
    } else {
        emu.pc = 2136980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b94));
    }
}
#[inline(always)]
pub fn block_0x00209b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2137180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c5c));
    } else {
        emu.pc = 2136984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b98));
    }
}
#[inline(always)]
pub fn block_0x00209b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2136988u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2136992u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2136996u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2137000u32);
    emu.apc_no_count(1usize, 2137000u32, 4096u32, 2137004u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137008u32;
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
pub fn block_0x00209bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 8u32, 2137012u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bd4));
    } else {
        emu.pc = 2137016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bb8));
    }
}
#[inline(always)]
pub fn block_0x00209bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2137020u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2137160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c48));
    } else {
        emu.pc = 2137024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bc0));
    }
}
#[inline(always)]
pub fn block_0x00209bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2137232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c90));
    } else {
        emu.pc = 2137028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bc4));
    }
}
#[inline(always)]
pub fn block_0x00209bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 10usize, 2137032u32);
    emu.adr_no_count(9usize, 9usize, 10usize, 2137036u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2136984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b98));
    } else {
        emu.pc = 2137040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bd0));
    }
}
#[inline(always)]
pub fn block_0x00209bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2137044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137180u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209c5c));
}
#[inline(always)]
pub fn block_0x00209bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2137048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2137144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c38));
    } else {
        emu.pc = 2137052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bdc));
    }
}
#[inline(always)]
pub fn block_0x00209bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b88));
    } else {
        emu.pc = 2137056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209be0));
    }
}
#[inline(always)]
pub fn block_0x00209be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 12u32, 2137060u32)?;
    emu.lbu_no_count(11usize, 18usize, 8u32, 2137064u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c4c));
    } else {
        emu.pc = 2137068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bec));
    }
}
#[inline(always)]
pub fn block_0x00209bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 18usize, 4u32, 2137072u32)?;
    emu.lw_no_count(11usize, 25usize, 0u32, 2137076u32)?;
    emu.lw_no_count(19usize, 18usize, 0u32, 2137080u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2137092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c04));
    } else {
        emu.pc = 2137084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209bfc));
    }
}
#[inline(always)]
pub fn block_0x00209bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2137088u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2137092u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00209c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 25usize, 4u32, 2137096u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2137116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c1c));
    } else {
        emu.pc = 2137100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c0c));
    }
}
#[inline(always)]
pub fn block_0x00209c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 25usize, 8u32, 2137104u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2137108u32);
    emu.apc_no_count(1usize, 2137108u32, 4294934528u32, 2137112u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2137120u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2137124u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2137128u32);
    emu.apc_no_count(1usize, 2137128u32, 4294934528u32, 2137132u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137136u32;
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
pub fn block_0x00209c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2136984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b98));
    } else {
        emu.pc = 2137140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c34));
    }
}
#[inline(always)]
pub fn block_0x00209c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2137144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137180u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209c5c));
}
#[inline(always)]
pub fn block_0x00209c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c4c));
    } else {
        emu.pc = 2137148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c3c));
    }
}
#[inline(always)]
pub fn block_0x00209c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 9u32, 2137152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2136980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209b94));
    } else {
        emu.pc = 2137156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c44));
    }
}
#[inline(always)]
pub fn block_0x00209c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2137160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209c4c));
}
#[inline(always)]
pub fn block_0x00209c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2137164u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209c4c));
}
#[inline(always)]
pub fn block_0x00209c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2137168u32)?;
    emu.ani_no_count(12usize, 11usize, 255u32, 2137172u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2137176u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2137252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ca4));
    } else {
        emu.pc = 2137180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209c5c));
    }
}
#[inline]
pub fn block_0x00209c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2137184u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2137188u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2137192u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2137196u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2137200u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2137204u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2137208u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2137212u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2137216u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2137220u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2137224u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2137228u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137232u32;
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
pub fn block_0x00209c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137236u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967032u32, 2137240u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2137244u32);
    emu.apc_no_count(1usize, 2137244u32, 114688u32, 2137248u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2137256u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2137260u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2137264u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137268u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966900u32, 2137272u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2137276u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967048u32, 2137280u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2137284u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967064u32, 2137288u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2137292u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2137296u32);
    emu.apc_no_count(1usize, 2137296u32, 94208u32, 2137300u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2137308u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2137312u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2137316u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2137320u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2137324u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2137328u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2137332u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2137336u32);
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2137340u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 96u32, 2137344u32)?;
    emu.sw_no_count(0usize, 10usize, 100u32, 2137348u32)?;
    emu.ani_no_count(11usize, 11usize, 1u32, 2137352u32);
    emu.sw_no_count(0usize, 10usize, 96u32, 2137356u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2138672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a230));
    } else {
        emu.pc = 2137360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d10));
    }
}
#[inline(always)]
pub fn block_0x00209d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 104u32, 2137368u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2137372u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2137376u32);
    emu.apc_no_count(1usize, 2137376u32, 0u32, 2137380u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00209d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 48u32, 2137388u32);
    emu.lbu_no_count(18usize, 2usize, 112u32, 2137392u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2137396u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2137400u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137404u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2137408u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2137412u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2137416u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2137420u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2137424u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2137428u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2137432u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2137436u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2137440u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2137444u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2137448u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2137452u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2137456u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2137460u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2137464u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2137468u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2137472u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2137476u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2137480u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2137484u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2137488u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2137492u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2137496u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2137500u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2137504u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2137508u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2137512u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2137516u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2137520u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2137556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209dd4));
    } else {
        emu.pc = 2137524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209db4));
    }
}
#[inline(always)]
pub fn block_0x00209db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2137528u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2137532u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2137536u32);
    emu.apc_no_count(1usize, 2137536u32, 0u32, 2137540u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137544u32;
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
#[inline(always)]
pub fn block_0x00209dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2137548u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2137552u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2137668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e44));
    } else {
        emu.pc = 2137556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209dd4));
    }
}
#[inline(always)]
pub fn block_0x00209dd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2137560u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2137564u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2137568u32);
    emu.apc_no_count(1usize, 2137568u32, 36864u32, 2137572u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2137580u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2137584u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2137588u32);
    emu.apc_no_count(1usize, 2137588u32, 0u32, 2137592u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2137600u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2137604u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2137608u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2137612u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2137616u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2137620u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2137624u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2137628u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2137632u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2137636u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2137640u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2137644u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2137648u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2137652u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2137656u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2137660u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2137664u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2137668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2137688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e58));
}
#[inline(always)]
pub fn block_0x00209e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 104u32, 2137672u32)?;
    emu.sw_no_count(20usize, 2usize, 108u32, 2137676u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2137680u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2137684u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2137688u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2137688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e58));
}
#[inline(always)]
pub fn block_0x00209e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2137688u32, 36864u32, 2137692u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137696u32;
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
#[inline(never)]
pub fn block_0x00209e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2137700u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137704u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 8u32, 2137708u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2137712u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2137716u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2137720u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2137724u32)?;
    emu.lw_no_count(5usize, 2usize, 28u32, 2137728u32)?;
    emu.lw_no_count(6usize, 2usize, 32u32, 2137732u32)?;
    emu.lw_no_count(14usize, 2usize, 36u32, 2137736u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2137740u32);
    emu.sri_no_count(7usize, 11usize, 8u32, 2137744u32);
    emu.sri_no_count(28usize, 11usize, 24u32, 2137748u32);
    emu.anr_no_count(29usize, 11usize, 12usize, 2137752u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2137756u32);
    emu.sri_no_count(30usize, 13usize, 8u32, 2137760u32);
    emu.sri_no_count(31usize, 13usize, 24u32, 2137764u32);
    emu.anr_no_count(9usize, 13usize, 12usize, 2137768u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2137772u32);
    emu.sri_no_count(18usize, 15usize, 8u32, 2137776u32);
    emu.sri_no_count(19usize, 15usize, 24u32, 2137780u32);
    emu.anr_no_count(20usize, 15usize, 12usize, 2137784u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2137788u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2137792u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2137796u32);
    emu.sri_no_count(28usize, 16usize, 8u32, 2137800u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2137804u32);
    emu.orr_no_count(11usize, 11usize, 29usize, 2137808u32);
    emu.sri_no_count(29usize, 16usize, 24u32, 2137812u32);
    emu.anr_no_count(30usize, 30usize, 12usize, 2137816u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2137820u32);
    emu.anr_no_count(31usize, 16usize, 12usize, 2137824u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2137828u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2137832u32);
    emu.orr_no_count(13usize, 13usize, 9usize, 2137836u32);
    emu.sri_no_count(9usize, 17usize, 8u32, 2137840u32);
    emu.anr_no_count(18usize, 18usize, 12usize, 2137844u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2137848u32);
    emu.sri_no_count(19usize, 17usize, 24u32, 2137852u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2137856u32);
    emu.orr_no_count(15usize, 15usize, 20usize, 2137860u32);
    emu.anr_no_count(20usize, 17usize, 12usize, 2137864u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2137868u32);
    emu.anr_no_count(28usize, 28usize, 12usize, 2137872u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2137876u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2137880u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2137884u32);
    emu.orr_no_count(16usize, 16usize, 31usize, 2137888u32);
    emu.sri_no_count(31usize, 5usize, 24u32, 2137892u32);
    emu.anr_no_count(9usize, 9usize, 12usize, 2137896u32);
    emu.orr_no_count(9usize, 9usize, 19usize, 2137900u32);
    emu.anr_no_count(19usize, 5usize, 12usize, 2137904u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2137908u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2137912u32);
    emu.orr_no_count(17usize, 17usize, 20usize, 2137916u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2137920u32);
    emu.anr_no_count(29usize, 29usize, 12usize, 2137924u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2137928u32);
    emu.sri_no_count(31usize, 6usize, 24u32, 2137932u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2137936u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2137940u32);
    emu.anr_no_count(19usize, 6usize, 12usize, 2137944u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2137948u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2137952u32);
    emu.orr_no_count(31usize, 20usize, 31usize, 2137956u32);
    emu.sri_no_count(20usize, 14usize, 8u32, 2137960u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2137964u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2137968u32);
    emu.sri_no_count(19usize, 14usize, 24u32, 2137972u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2137976u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2137980u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2137984u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2137988u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2137992u32);
    emu.orr_no_count(20usize, 14usize, 12usize, 2137996u32);
    emu.orr_no_count(11usize, 11usize, 7usize, 2138000u32);
    emu.orr_no_count(12usize, 13usize, 30usize, 2138004u32);
    emu.orr_no_count(13usize, 15usize, 18usize, 2138008u32);
    emu.orr_no_count(14usize, 16usize, 28usize, 2138012u32);
    emu.orr_no_count(15usize, 17usize, 9usize, 2138016u32);
    emu.orr_no_count(16usize, 5usize, 29usize, 2138020u32);
    emu.orr_no_count(17usize, 6usize, 31usize, 2138024u32);
    emu.adi_no_count(5usize, 0usize, 16u32, 2138028u32);
    emu.orr_no_count(6usize, 20usize, 19usize, 2138032u32);
    emu.add_memory_rw_events(85usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2138040u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2138044u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2138048u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2138056u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2138060u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2138064u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2138072u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2138076u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2138080u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209fe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2138088u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2138092u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2138096u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00209ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2138104u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2138108u32);
    emu.adi_no_count(11usize, 16usize, 0u32, 2138112u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2138120u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2138124u32);
    emu.adi_no_count(11usize, 17usize, 0u32, 2138128u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2138136u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2138140u32);
    emu.adi_no_count(11usize, 6usize, 0u32, 2138144u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138152u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 56u32, 2138156u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2138688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a240));
    } else {
        emu.pc = 2138160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a030));
    }
}
#[inline]
pub fn block_0x0020a030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2138164u32);
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138168u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 56u32, 2138172u32);
    let a = 0u32.wrapping_add(2281701376u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2138176u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 14usize, 4u32, 2138180u32)?;
    let a = 0u32.wrapping_add(2013265920u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138184u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1u32, 2138188u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2138192u32);
    emu.mul_no_count(15usize, 11usize, 13usize, 2138196u32);
    emu.mulhu_no_count(16usize, 15usize, 12usize, 2138200u32);
    emu.mul_no_count(15usize, 15usize, 12usize, 2138204u32);
    emu.sltru_no_count(11usize, 11usize, 15usize, 2138208u32);
    emu.sltru_no_count(15usize, 0usize, 16usize, 2138212u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2138216u32);
    emu.orr_no_count(11usize, 11usize, 15usize, 2138220u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2138224u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2138228u32);
    emu.sbr_no_count(11usize, 11usize, 16usize, 2138232u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2138236u32);
    emu.add_memory_rw_events(20usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x0020a080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 8u32, 2138244u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2138248u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2138252u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2138256u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2138260u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2138264u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2138268u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138272u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2138276u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2138280u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2138284u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2138288u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2138292u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x0020a0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 12u32, 2138300u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2138304u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2138308u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2138312u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2138316u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2138320u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2138324u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138328u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2138332u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2138336u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2138340u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2138344u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2138348u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x0020a0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 16u32, 2138356u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2138360u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2138364u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2138368u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2138372u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2138376u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2138380u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138384u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2138388u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2138392u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2138396u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2138400u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2138404u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x0020a128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 20u32, 2138412u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2138416u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2138420u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2138424u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2138428u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2138432u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2138436u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138440u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2138444u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2138448u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2138452u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2138456u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2138460u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x0020a160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 24u32, 2138468u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2138472u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2138476u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2138480u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2138484u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2138488u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2138492u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138496u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2138500u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2138504u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2138508u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2138512u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2138516u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x0020a198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 28u32, 2138524u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2138528u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2138532u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2138536u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2138540u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2138544u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2138548u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138552u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2138556u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2138560u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2138564u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2138568u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2138572u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x0020a1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 32u32, 2138580u32)?;
    emu.adi_no_count(5usize, 0usize, 26u32, 2138584u32);
    emu.mul_no_count(11usize, 10usize, 13usize, 2138588u32);
    emu.mulhu_no_count(13usize, 11usize, 12usize, 2138592u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2138596u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2138600u32);
    emu.sltru_no_count(11usize, 0usize, 13usize, 2138604u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2138608u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138612u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2138616u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2138620u32);
    emu.sbr_no_count(11usize, 10usize, 13usize, 2138624u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2138628u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2138636u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2138640u32);
    emu.add_memory_rw_events(3usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138648u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966943u32, 2138652u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138656u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967112u32, 2138660u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2138664u32);
    emu.apc_no_count(1usize, 2138664u32, 77824u32, 2138668u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138676u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967080u32, 2138680u32);
    emu.apc_no_count(1usize, 2138680u32, 86016u32, 2138684u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138692u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967096u32, 2138696u32);
    emu.apc_no_count(1usize, 2138696u32, 86016u32, 2138700u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2138708u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2138712u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2138716u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2138720u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2138724u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2138728u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2138732u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2138736u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2138740u32);
    emu.add_memory_rw_events(10usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2138748u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a368));
    } else {
        emu.pc = 2138752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a280));
    }
}
#[inline(always)]
pub fn block_0x0020a280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138756u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 96u32, 2138760u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2138764u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2139016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a388));
    } else {
        emu.pc = 2138768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a290));
    }
}
#[inline(always)]
pub fn block_0x0020a290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2138772u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 96u32, 2138776u32);
    emu.lbu_no_count(19usize, 18usize, 112u32, 2138780u32);
    emu.adi_no_count(10usize, 0usize, 64u32, 2138784u32);
    emu.sbr_no_count(12usize, 10usize, 19usize, 2138788u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2138820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2c4));
    } else {
        emu.pc = 2138792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2a8));
    }
}
#[inline(always)]
pub fn block_0x0020a2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 19usize, 2138796u32);
    emu.adi_no_count(10usize, 10usize, 48u32, 2138800u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2138804u32);
    emu.apc_no_count(1usize, 2138804u32, 0u32, 2138808u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a2bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 8usize, 19usize, 2138816u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2138820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a364));
}
#[inline(always)]
pub fn block_0x0020a2c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2138900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a314));
    } else {
        emu.pc = 2138824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2c8));
    }
}
#[inline(always)]
pub fn block_0x0020a2c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 12usize, 2138828u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2138832u32);
    emu.adi_no_count(9usize, 18usize, 48u32, 2138836u32);
    emu.adr_no_count(10usize, 9usize, 19usize, 2138840u32);
    emu.apc_no_count(1usize, 2138840u32, 0u32, 2138844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138848u32;
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
#[inline]
pub fn block_0x0020a2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2138852u32)?;
    emu.lw_no_count(11usize, 18usize, 44u32, 2138856u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2138860u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2138864u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2138868u32);
    emu.sw_no_count(10usize, 18usize, 40u32, 2138872u32)?;
    emu.sw_no_count(11usize, 18usize, 44u32, 2138876u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2138880u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2138884u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2138888u32);
    emu.apc_no_count(1usize, 2138888u32, 36864u32, 2138892u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2138900u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a314));
}
#[inline(always)]
pub fn block_0x0020a314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 4294967232u32, 2138904u32);
    emu.ani_no_count(9usize, 8usize, 63u32, 2138908u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2138912u32);
    emu.adr_no_count(8usize, 11usize, 10usize, 2138916u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2138960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a350));
    } else {
        emu.pc = 2138920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a328));
    }
}
#[inline]
pub fn block_0x0020a328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2138924u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2138928u32)?;
    emu.adr_no_count(14usize, 10usize, 12usize, 2138932u32);
    emu.sltru_no_count(10usize, 14usize, 10usize, 2138936u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2138940u32);
    emu.sw_no_count(14usize, 18usize, 40u32, 2138944u32)?;
    emu.sw_no_count(10usize, 18usize, 44u32, 2138948u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2138952u32);
    emu.apc_no_count(1usize, 2138952u32, 36864u32, 2138956u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138960u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020a350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 48u32, 2138964u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2138968u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2138972u32);
    emu.apc_no_count(1usize, 2138972u32, 0u32, 2138976u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 18usize, 112u32, 2138984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a368));
}
#[inline(always)]
pub fn block_0x0020a368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2138988u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2138992u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2138996u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2139000u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2139004u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2139008u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139012u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139016u32;
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
pub fn block_0x0020a388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139020u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967128u32, 2139024u32);
    emu.apc_no_count(1usize, 2139024u32, 86016u32, 2139028u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 240u32, 2139036u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 5usize, 0u32, 2139044u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139048u32;
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
pub fn block_0x0020a3a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 241u32, 2139052u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x0020a3b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139060u32;
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
pub fn block_0x0020a3b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2139064u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2139068u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2139072u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2139076u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2139080u32);
    emu.apc_no_count(1usize, 2139080u32, 0u32, 2139084u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(20u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2139092u32);
    emu.apc_no_count(1usize, 2139092u32, 0u32, 2139096u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a3dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2139100u32, 0u32, 2139104u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139108u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a3e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3e8));
    }
}
#[inline(always)]
pub fn block_0x0020a3e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 0u32, 2139116u32);
    emu.adr_no_count(13usize, 12usize, 10usize, 2139120u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2139124u32);
    emu.sb_no_count(11usize, 13usize, 4294967295u32, 2139128u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3fc));
    }
}
#[inline(always)]
pub fn block_0x0020a3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 1u32, 2139136u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2139140u32);
    emu.sb_no_count(11usize, 13usize, 4294967294u32, 2139144u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2139148u32);
    emu.sb_no_count(11usize, 13usize, 4294967293u32, 2139152u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a414));
    }
}
#[inline(always)]
pub fn block_0x0020a414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 3u32, 2139160u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2139164u32);
    emu.sb_no_count(11usize, 13usize, 4294967292u32, 2139168u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a424));
    }
}
#[inline]
pub fn block_0x0020a424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2139176u32);
    emu.ani_no_count(14usize, 13usize, 3u32, 2139180u32);
    emu.adr_no_count(13usize, 10usize, 14usize, 2139184u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2139188u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2139192u32);
    emu.ani_no_count(11usize, 11usize, 255u32, 2139196u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139200u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 257u32, 2139204u32);
    emu.mul_no_count(11usize, 11usize, 14usize, 2139208u32);
    emu.sw_no_count(11usize, 13usize, 0u32, 2139212u32)?;
    emu.adr_no_count(14usize, 13usize, 12usize, 2139216u32);
    emu.sw_no_count(11usize, 14usize, 4294967292u32, 2139220u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a458));
    }
}
#[inline(always)]
pub fn block_0x0020a458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 4u32, 2139228u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2139232u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967284u32, 2139236u32)?;
    emu.adi_no_count(15usize, 0usize, 25u32, 2139240u32);
    emu.sw_no_count(11usize, 14usize, 4294967288u32, 2139244u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a470));
    }
}
#[inline]
pub fn block_0x0020a470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 12u32, 2139252u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2139256u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2139260u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2139264u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967268u32, 2139268u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967272u32, 2139272u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967276u32, 2139276u32)?;
    emu.ani_no_count(15usize, 13usize, 4u32, 2139280u32);
    emu.ori_no_count(15usize, 15usize, 24u32, 2139284u32);
    emu.sbr_no_count(12usize, 12usize, 15usize, 2139288u32);
    emu.adi_no_count(16usize, 0usize, 32u32, 2139292u32);
    emu.sw_no_count(11usize, 14usize, 4294967280u32, 2139296u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    } else {
        emu.pc = 2139300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4a4));
    }
}
#[inline(always)]
pub fn block_0x0020a4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 15usize, 2139304u32);
    emu.adi_no_count(14usize, 0usize, 31u32, 2139308u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a4ac));
}
#[inline]
pub fn block_0x0020a4ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 0u32, 2139312u32)?;
    emu.sw_no_count(11usize, 13usize, 4u32, 2139316u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2139320u32)?;
    emu.sw_no_count(11usize, 13usize, 12u32, 2139324u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2139328u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2139332u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2139336u32)?;
    emu.sw_no_count(11usize, 13usize, 28u32, 2139340u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967264u32, 2139344u32);
    emu.adi_no_count(13usize, 13usize, 32u32, 2139348u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2139308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4ac));
    } else {
        emu.pc = 2139352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4d8));
    }
}
#[inline(always)]
pub fn block_0x0020a4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139356u32;
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
