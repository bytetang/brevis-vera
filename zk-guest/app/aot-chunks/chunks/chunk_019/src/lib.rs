pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2197004u32;
pub const PC_MAX: u32 = 2199296u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 113usize] = [
        block_0x0021860c,
        block_0x00218630,
        block_0x00218654,
        block_0x00218678,
        block_0x0021869c,
        block_0x002186b4,
        block_0x002186c0,
        block_0x002186e0,
        block_0x00218728,
        block_0x00218738,
        block_0x00218790,
        block_0x00218794,
        block_0x002187a8,
        block_0x002187b0,
        block_0x002187c8,
        block_0x002187cc,
        block_0x002187e0,
        block_0x00218800,
        block_0x00218804,
        block_0x00218810,
        block_0x00218820,
        block_0x00218830,
        block_0x00218834,
        block_0x00218848,
        block_0x0021885c,
        block_0x00218860,
        block_0x0021888c,
        block_0x002188b0,
        block_0x002188cc,
        block_0x002188dc,
        block_0x002188e4,
        block_0x002188f8,
        block_0x002188fc,
        block_0x0021890c,
        block_0x00218910,
        block_0x00218918,
        block_0x00218928,
        block_0x0021892c,
        block_0x00218938,
        block_0x0021893c,
        block_0x00218940,
        block_0x0021895c,
        block_0x00218964,
        block_0x00218968,
        block_0x00218994,
        block_0x00218998,
        block_0x002189a4,
        block_0x002189ac,
        block_0x002189b0,
        block_0x002189b8,
        block_0x002189c4,
        block_0x002189d8,
        block_0x002189f0,
        block_0x002189f8,
        block_0x00218a1c,
        block_0x00218a24,
        block_0x00218a30,
        block_0x00218a44,
        block_0x00218a50,
        block_0x00218a58,
        block_0x00218a78,
        block_0x00218a80,
        block_0x00218ab0,
        block_0x00218aec,
        block_0x00218afc,
        block_0x00218b0c,
        block_0x00218b40,
        block_0x00218b4c,
        block_0x00218b54,
        block_0x00218b74,
        block_0x00218b7c,
        block_0x00218bac,
        block_0x00218be8,
        block_0x00218c10,
        block_0x00218c14,
        block_0x00218c38,
        block_0x00218c3c,
        block_0x00218c4c,
        block_0x00218c64,
        block_0x00218c78,
        block_0x00218c80,
        block_0x00218c90,
        block_0x00218c98,
        block_0x00218ca4,
        block_0x00218ccc,
        block_0x00218cdc,
        block_0x00218ce4,
        block_0x00218cec,
        block_0x00218cfc,
        block_0x00218d10,
        block_0x00218d14,
        block_0x00218d34,
        block_0x00218d40,
        block_0x00218d48,
        block_0x00218d68,
        block_0x00218d70,
        block_0x00218da0,
        block_0x00218ddc,
        block_0x00218df0,
        block_0x00218e08,
        block_0x00218e2c,
        block_0x00218e48,
        block_0x00218e50,
        block_0x00218e64,
        block_0x00218e78,
        block_0x00218e8c,
        block_0x00218eb0,
        block_0x00218eb8,
        block_0x00218ed0,
        block_0x00218ee0,
        block_0x00218ee8,
        block_0x00218ef0,
        block_0x00218f00,
    ];
    const IDX: [u16; 574usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16,
        0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 16u16, 0u16, 0u16, 0u16, 0u16,
        17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 0u16, 20u16,
        0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16, 0u16, 0u16,
        24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16, 0u16, 34u16, 35u16,
        0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 39u16, 40u16, 41u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 43u16, 44u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 47u16, 0u16,
        48u16, 49u16, 0u16, 50u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 52u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 55u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        61u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 66u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16,
        0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16,
        71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 75u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16,
        78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16,
        81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 86u16, 0u16, 87u16,
        0u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 91u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 93u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16,
        0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16,
        0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
        0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16,
        0u16, 111u16, 0u16, 112u16, 0u16, 0u16, 0u16, 113u16,
    ];
    if pc < 2197004u32 || pc > 2199296u32 {
        return None;
    }
    let word_offset = ((pc - 2197004u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021860c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197008u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1275u32, 2197012u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2197016u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2197020u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2197024u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2197028u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197032u32);
    emu.apc_no_count(6usize, 2197032u32, 40960u32, 2197036u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197040u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197044u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1112u32, 2197048u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2197052u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2197056u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2197060u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2197064u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197068u32);
    emu.apc_no_count(6usize, 2197068u32, 40960u32, 2197072u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197076u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197080u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966684u32, 2197084u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2197088u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2197092u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2197096u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2197100u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197104u32);
    emu.apc_no_count(6usize, 2197104u32, 40960u32, 2197108u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197112u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197116u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1246u32, 2197120u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2197124u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2197128u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2197132u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2197136u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197140u32);
    emu.apc_no_count(6usize, 2197140u32, 40960u32, 2197144u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197148u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021869c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2197152u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2197156u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2197160u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2197164u32);
    emu.apc_no_count(6usize, 2197164u32, 40960u32, 2197168u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197172u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002186b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 12u32, 2197176u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2197180u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197184u32;
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
pub fn block_0x002186c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2197188u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2197192u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2197196u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2197200u32)?;
    emu.sli_no_count(12usize, 12usize, 1u32, 2197204u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2197208u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197212u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2197304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218738));
    } else {
        emu.pc = 2197216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002186e0));
    }
}
#[inline]
pub fn block_0x002186e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2197220u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966148u32, 2197224u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2197228u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966716u32, 2197232u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2197236u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2197240u32)?;
    emu.adi_no_count(15usize, 2usize, 32u32, 2197244u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2197248u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2197252u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2197256u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2197260u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2197264u32)?;
    emu.sw_no_count(14usize, 2usize, 12u32, 2197268u32)?;
    emu.sw_no_count(15usize, 2usize, 16u32, 2197272u32)?;
    emu.sw_no_count(14usize, 2usize, 20u32, 2197276u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2197280u32);
    emu.apc_no_count(1usize, 2197280u32, 36864u32, 2197284u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197288u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2197292u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2197296u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197300u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197304u32;
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
pub fn block_0x00218738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2197308u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2197312u32)?;
    emu.sb_no_count(0usize, 2usize, 4u32, 2197316u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2197320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966148u32, 2197324u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2197328u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966716u32, 2197332u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2197336u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2197340u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2197344u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2197348u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2197352u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2197356u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2197360u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2197364u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2197368u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2197372u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1088u32, 2197376u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2197380u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2197384u32);
    emu.apc_no_count(1usize, 2197384u32, 36864u32, 2197388u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197392u32;
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
pub fn block_0x00218790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187a8));
    } else {
        emu.pc = 2197396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218794));
    }
}
#[inline(always)]
pub fn block_0x00218794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2197400u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2197404u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2197408u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197412u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197416u32;
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
pub fn block_0x002187a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 4u32, 2197420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2197452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187cc));
    } else {
        emu.pc = 2197424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187b0));
    }
}
#[inline(always)]
pub fn block_0x002187b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2197428u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1329u32, 2197432u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2197436u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2197440u32);
    emu.apc_no_count(1usize, 2197440u32, 40960u32, 2197444u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002187c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218794));
    } else {
        emu.pc = 2197452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002187cc));
    }
}
#[inline(always)]
pub fn block_0x002187cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2197456u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2197460u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2197464u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197472u32;
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
pub fn block_0x002187e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2197476u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2197480u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2197484u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2197488u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2197492u32)?;
    emu.adi_no_count(13usize, 0usize, 7u32, 2197496u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2197500u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2197556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218834));
    } else {
        emu.pc = 2197504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218800));
    }
}
#[inline(always)]
pub fn block_0x00218800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021885c));
    } else {
        emu.pc = 2197508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218804));
    }
}
#[inline(always)]
pub fn block_0x00218804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4294967295u32, 2197512u32);
    emu.adi_no_count(10usize, 0usize, 46u32, 2197516u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2197520u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2197520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218810));
}
#[inline(always)]
pub fn block_0x00218810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 14usize, 0u32, 2197524u32);
    emu.adi_no_count(13usize, 17usize, 4294967250u32, 2197528u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2197532u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2197600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218860));
    } else {
        emu.pc = 2197536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218820));
    }
}
#[inline(always)]
pub fn block_0x00218820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 15usize, 0u32, 2197540u32);
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2197544u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2197548u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2197520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218810));
    } else {
        emu.pc = 2197552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218830));
    }
}
#[inline(always)]
pub fn block_0x00218830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2197556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218860));
}
#[inline(always)]
pub fn block_0x00218834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 46u32, 2197560u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2197564u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2197568u32);
    emu.apc_no_count(1usize, 2197568u32, 28672u32, 2197572u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197576u32;
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
pub fn block_0x00218848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2197580u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2197584u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2197588u32);
    emu.sltiu_no_count(13usize, 10usize, 1u32, 2197592u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2197596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218860));
}
#[inline(always)]
pub fn block_0x0021885c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2197600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2197600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218860));
}
#[inline]
pub fn block_0x00218860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 8usize, 4u32, 2197604u32);
    emu.lw_no_count(10usize, 8usize, 0u32, 2197608u32)?;
    emu.orr_no_count(13usize, 13usize, 14usize, 2197612u32);
    emu.sb_no_count(13usize, 8usize, 4u32, 2197616u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2197620u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2197624u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2197628u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2197632u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2197636u32);
    emu.apc_no_count(6usize, 2197636u32, 40960u32, 2197640u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197644u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021888c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 4u32, 2197648u32);
    emu.lw_no_count(12usize, 10usize, 0u32, 2197652u32)?;
    emu.adi_no_count(14usize, 11usize, 4294967250u32, 2197656u32);
    emu.sltiu_no_count(14usize, 14usize, 1u32, 2197660u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2197664u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2197668u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2197672u32);
    emu.apc_no_count(6usize, 2197672u32, 40960u32, 2197676u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2197680u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002188b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2197684u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2197688u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2197692u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2197696u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2197700u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2197704u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2197816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218938));
    } else {
        emu.pc = 2197708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002188cc));
    }
}
#[inline(always)]
pub fn block_0x002188cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2197712u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2197716u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2197720u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2197772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021890c));
    } else {
        emu.pc = 2197724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002188dc));
    }
}
#[inline(always)]
pub fn block_0x002188dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2197728u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2197772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021890c));
    } else {
        emu.pc = 2197732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002188e4));
    }
}
#[inline(always)]
pub fn block_0x002188e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2197736u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2197740u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2197744u32);
    emu.apc_no_count(1usize, 2197744u32, 4294873088u32, 2197748u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002188f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021892c));
    } else {
        emu.pc = 2197756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002188fc));
    }
}
#[inline(always)]
pub fn block_0x002188fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2197760u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2197764u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2197768u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2197772u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218940));
}
#[inline(always)]
pub fn block_0x0021890c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021895c));
    } else {
        emu.pc = 2197776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218910));
    }
}
#[inline(always)]
pub fn block_0x00218910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2197776u32, 4294905856u32, 2197780u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197784u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2197788u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2197792u32);
    emu.apc_no_count(1usize, 2197792u32, 4294873088u32, 2197796u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197800u32;
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
pub fn block_0x00218928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002188fc));
    } else {
        emu.pc = 2197804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021892c));
    }
}
#[inline(always)]
pub fn block_0x0021892c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2197808u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2197812u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2197816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021893c));
}
#[inline(always)]
pub fn block_0x00218938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2197820u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2197820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021893c));
}
#[inline(always)]
pub fn block_0x0021893c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2197824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2197824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218940));
}
#[inline(always)]
pub fn block_0x00218940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2197828u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2197832u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2197836u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2197840u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2197844u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2197848u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197852u32;
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
pub fn block_0x0021895c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2197856u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2197756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002188fc));
    } else {
        emu.pc = 2197860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218964));
    }
}
#[inline(always)]
pub fn block_0x00218964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2197864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021892c));
}
#[inline]
pub fn block_0x00218968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2197868u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2197872u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2197876u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2197880u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2197884u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2197888u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2197892u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2197896u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2197900u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2197904u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2197912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218998));
    } else {
        emu.pc = 2197908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218994));
    }
}
#[inline(always)]
pub fn block_0x00218994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2197912u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2197912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218998));
}
#[inline(always)]
pub fn block_0x00218998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 13usize, 26u32, 2197916u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2197920u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2197944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002189b8));
    } else {
        emu.pc = 2197924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002189a4));
    }
}
#[inline(always)]
pub fn block_0x002189a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 18usize, 5u32, 2197928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2198052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a24));
    } else {
        emu.pc = 2197932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002189ac));
    }
}
#[inline(always)]
pub fn block_0x002189ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2197956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002189c4));
    } else {
        emu.pc = 2197936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002189b0));
    }
}
#[inline(always)]
pub fn block_0x002189b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2197940u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2197944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2197976u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002189d8));
}
#[inline(always)]
pub fn block_0x002189b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2197948u32);
    emu.apc_no_count(1usize, 2197948u32, 12288u32, 2197952u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002189c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2197960u32)?;
    emu.sli_no_count(13usize, 13usize, 5u32, 2197964u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2197968u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2197972u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2197976u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2197976u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002189d8));
}
#[inline(always)]
pub fn block_0x002189d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2197980u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2197984u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2197988u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2197992u32);
    emu.apc_no_count(1usize, 2197992u32, 0u32, 2197996u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002189f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2198004u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2198044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a1c));
    } else {
        emu.pc = 2198008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002189f8));
    }
}
#[inline]
pub fn block_0x002189f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2198012u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2198016u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2198020u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2198024u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2198028u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2198032u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2198036u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2198040u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198044u32;
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
pub fn block_0x00218a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2198048u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2198052u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2198052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218a24));
}
#[inline(always)]
pub fn block_0x00218a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2198056u32);
    emu.apc_no_count(1usize, 2198056u32, 12288u32, 2198060u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198064u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00218a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2198068u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2198072u32)?;
    emu.adi_no_count(10usize, 0usize, 128u32, 2198076u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2198080u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2198096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a50));
    } else {
        emu.pc = 2198084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a44));
    }
}
#[inline(always)]
pub fn block_0x00218a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 8u32, 2198088u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2198092u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2198096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218aec));
}
#[inline(always)]
pub fn block_0x00218a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2198100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2198136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a78));
    } else {
        emu.pc = 2198104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a58));
    }
}
#[inline(always)]
pub fn block_0x00218a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2198108u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198112u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2198116u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198120u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2198124u32);
    emu.sb_no_count(11usize, 2usize, 9u32, 2198128u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2198132u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2198136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218aec));
}
#[inline(always)]
pub fn block_0x00218a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2198140u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2198192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ab0));
    } else {
        emu.pc = 2198144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218a80));
    }
}
#[inline]
pub fn block_0x00218a80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2198148u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2198152u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198156u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2198160u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2198164u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198168u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2198172u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2198176u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2198180u32);
    emu.sb_no_count(11usize, 2usize, 10u32, 2198184u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2198188u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2198192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218aec));
}
#[inline]
pub fn block_0x00218ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2198196u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2198200u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2198204u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198208u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2198212u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2198216u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2198220u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198224u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2198228u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2198232u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2198236u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2198240u32);
    emu.sb_no_count(13usize, 2usize, 10u32, 2198244u32);
    emu.sb_no_count(11usize, 2usize, 11u32, 2198248u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2198252u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2198252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218aec));
}
#[inline(always)]
pub fn block_0x00218aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2198256u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2198260u32);
    emu.apc_no_count(1usize, 2198260u32, 4294909952u32, 2198264u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198268u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2198272u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2198276u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2198280u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198284u32;
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
pub fn block_0x00218b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2198288u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2198292u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2198296u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2198300u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2198304u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2198308u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2198312u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2198316u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2198320u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2198324u32);
    emu.adi_no_count(10usize, 0usize, 128u32, 2198328u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2198332u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2198348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b4c));
    } else {
        emu.pc = 2198336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b40));
    }
}
#[inline(always)]
pub fn block_0x00218b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2198340u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2198344u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2198348u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218be8));
}
#[inline(always)]
pub fn block_0x00218b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2198352u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2198388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b74));
    } else {
        emu.pc = 2198356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b54));
    }
}
#[inline(always)]
pub fn block_0x00218b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2198360u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198364u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2198368u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198372u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2198376u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2198380u32);
    emu.adi_no_count(18usize, 0usize, 2u32, 2198384u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2198388u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218be8));
}
#[inline(always)]
pub fn block_0x00218b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2198392u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2198444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218bac));
    } else {
        emu.pc = 2198396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218b7c));
    }
}
#[inline]
pub fn block_0x00218b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2198400u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2198404u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198408u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2198412u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2198416u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198420u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2198424u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2198428u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2198432u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2198436u32);
    emu.adi_no_count(18usize, 0usize, 3u32, 2198440u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2198444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218be8));
}
#[inline]
pub fn block_0x00218bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2198448u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2198452u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2198456u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198460u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2198464u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2198468u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2198472u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198476u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2198480u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2198484u32);
    emu.sb_no_count(10usize, 2usize, 12u32, 2198488u32);
    emu.sb_no_count(12usize, 2usize, 13u32, 2198492u32);
    emu.sb_no_count(13usize, 2usize, 14u32, 2198496u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2198500u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2198504u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2198504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218be8));
}
#[inline]
pub fn block_0x00218be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 8usize, 8u32, 2198508u32)?;
    emu.lw_no_count(10usize, 19usize, 4u32, 2198512u32)?;
    emu.lw_no_count(21usize, 19usize, 8u32, 2198516u32)?;
    emu.lw_no_count(20usize, 19usize, 12u32, 2198520u32)?;
    emu.lw_no_count(11usize, 19usize, 0u32, 2198524u32)?;
    emu.sltru_no_count(12usize, 21usize, 10usize, 2198528u32);
    emu.sltiu_no_count(13usize, 20usize, 1u32, 2198532u32);
    emu.anr_no_count(14usize, 13usize, 12usize, 2198536u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2198540u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2198548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c14));
    } else {
        emu.pc = 2198544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c10));
    }
}
#[inline(always)]
pub fn block_0x00218c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2198548u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2198548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218c14));
}
#[inline]
pub fn block_0x00218c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2198552u32);
    emu.orr_no_count(13usize, 13usize, 21usize, 2198556u32);
    emu.sbr_no_count(13usize, 10usize, 13usize, 2198560u32);
    emu.sltru_no_count(10usize, 10usize, 13usize, 2198564u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2198568u32);
    emu.anr_no_count(22usize, 10usize, 13usize, 2198572u32);
    emu.adr_no_count(10usize, 11usize, 12usize, 2198576u32);
    emu.adi_no_count(9usize, 22usize, 0u32, 2198580u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2198588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c3c));
    } else {
        emu.pc = 2198584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c38));
    }
}
#[inline(always)]
pub fn block_0x00218c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 18usize, 0u32, 2198588u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2198588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218c3c));
}
#[inline(always)]
pub fn block_0x00218c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 12u32, 2198592u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2198596u32);
    emu.apc_no_count(1usize, 2198596u32, 4294909952u32, 2198600u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198604u32;
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
pub fn block_0x00218c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 9usize, 2198608u32);
    emu.sltru_no_count(10usize, 9usize, 21usize, 2198612u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2198616u32);
    emu.sw_no_count(9usize, 19usize, 8u32, 2198620u32)?;
    emu.sw_no_count(10usize, 19usize, 12u32, 2198624u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2198648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c78));
    } else {
        emu.pc = 2198628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c64));
    }
}
#[inline(always)]
pub fn block_0x00218c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2198632u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 10usize, 1448u32, 2198636u32)?;
    emu.ani_no_count(12usize, 19usize, 255u32, 2198640u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2198644u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2198656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c80));
    } else {
        emu.pc = 2198648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c78));
    }
}
#[inline(always)]
pub fn block_0x00218c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2198652u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2198656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218ca4));
}
#[inline(always)]
pub fn block_0x00218c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 1452u32, 2198660u32)?;
    emu.lbu_no_count(10usize, 8usize, 0u32, 2198664u32);
    emu.lw_no_count(9usize, 8usize, 4u32, 2198668u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2198732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ccc));
    } else {
        emu.pc = 2198672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c90));
    }
}
#[inline(always)]
pub fn block_0x00218c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2198676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2198732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ccc));
    } else {
        emu.pc = 2198680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218c98));
    }
}
#[inline(always)]
pub fn block_0x00218c98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2198684u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2198688u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2198692u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2198692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218ca4));
}
#[inline]
pub fn block_0x00218ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2198696u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2198700u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2198704u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2198708u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2198712u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2198716u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2198720u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2198724u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2198728u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198732u32;
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
pub fn block_0x00218ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 9usize, 4u32, 2198736u32)?;
    emu.lw_no_count(11usize, 21usize, 0u32, 2198740u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2198744u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2198756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ce4));
    } else {
        emu.pc = 2198748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218cdc));
    }
}
#[inline(always)]
pub fn block_0x00218cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2198752u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2198756u32;
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
pub fn block_0x00218ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 21usize, 4u32, 2198760u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2198780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218cfc));
    } else {
        emu.pc = 2198764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218cec));
    }
}
#[inline(always)]
pub fn block_0x00218cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2198768u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2198772u32);
    emu.apc_no_count(1usize, 2198772u32, 4294873088u32, 2198776u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198780u32;
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
pub fn block_0x00218cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2198784u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2198788u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2198792u32);
    emu.apc_no_count(1usize, 2198792u32, 4294873088u32, 2198796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2198800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2198804u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2198680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218c98));
}
#[inline(always)]
pub fn block_0x00218d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2198808u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2198812u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2198816u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2198820u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2198824u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2198828u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2198832u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2198848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d40));
    } else {
        emu.pc = 2198836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d34));
    }
}
#[inline(always)]
pub fn block_0x00218d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 12u32, 2198840u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2198844u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2198848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218ddc));
}
#[inline(always)]
pub fn block_0x00218d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 11u32, 2198852u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2198888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d68));
    } else {
        emu.pc = 2198856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d48));
    }
}
#[inline(always)]
pub fn block_0x00218d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 6u32, 2198860u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198864u32);
    emu.ori_no_count(12usize, 12usize, 192u32, 2198868u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198872u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2198876u32);
    emu.sb_no_count(11usize, 2usize, 13u32, 2198880u32);
    emu.adi_no_count(8usize, 0usize, 2u32, 2198884u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2198888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218ddc));
}
#[inline(always)]
pub fn block_0x00218d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2198892u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2198944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218da0));
    } else {
        emu.pc = 2198896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218d70));
    }
}
#[inline]
pub fn block_0x00218d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 12u32, 2198900u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2198904u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198908u32);
    emu.ori_no_count(12usize, 12usize, 224u32, 2198912u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2198916u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198920u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2198924u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2198928u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2198932u32);
    emu.sb_no_count(11usize, 2usize, 14u32, 2198936u32);
    emu.adi_no_count(8usize, 0usize, 3u32, 2198940u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2198944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218ddc));
}
#[inline]
pub fn block_0x00218da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 18u32, 2198948u32);
    emu.sli_no_count(13usize, 11usize, 14u32, 2198952u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2198956u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2198960u32);
    emu.ori_no_count(12usize, 12usize, 240u32, 2198964u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2198968u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2198972u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2198976u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2198980u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2198984u32);
    emu.sb_no_count(12usize, 2usize, 12u32, 2198988u32);
    emu.sb_no_count(13usize, 2usize, 13u32, 2198992u32);
    emu.sb_no_count(14usize, 2usize, 14u32, 2198996u32);
    emu.sb_no_count(11usize, 2usize, 15u32, 2199000u32);
    emu.adi_no_count(8usize, 0usize, 4u32, 2199004u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2199004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218ddc));
}
#[inline(always)]
pub fn block_0x00218ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 10usize, 8u32, 2199008u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2199012u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2199016u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2199020u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2199084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218e2c));
    } else {
        emu.pc = 2199024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218df0));
    }
}
#[inline(always)]
pub fn block_0x00218df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2199028u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2199032u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2199036u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2199040u32);
    emu.apc_no_count(1usize, 2199040u32, 4294905856u32, 2199044u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2199052u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2199056u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2199060u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2199064u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2199068u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2199072u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2199076u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2199080u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199084u32;
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
pub fn block_0x00218e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2199088u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2199092u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2199096u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2199100u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2199104u32);
    emu.apc_no_count(1usize, 2199104u32, 0u32, 2199108u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 9usize, 8u32, 2199116u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2199120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2199024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218df0));
}
#[inline(always)]
pub fn block_0x00218e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2199124u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199128u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1332u32, 2199132u32);
    emu.apc_no_count(6usize, 2199132u32, 32768u32, 2199136u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199140u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2199144u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1380u32, 2199152u32);
    emu.apc_no_count(6usize, 2199152u32, 32768u32, 2199156u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199160u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2199164u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199168u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1356u32, 2199172u32);
    emu.apc_no_count(6usize, 2199172u32, 32768u32, 2199176u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199180u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2199184u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2199188u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2199192u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2199196u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2199200u32)?;
    emu.lbu_no_count(11usize, 10usize, 0u32, 2199204u32);
    emu.lw_no_count(8usize, 10usize, 4u32, 2199208u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2199212u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2199248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ed0));
    } else {
        emu.pc = 2199216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218eb0));
    }
}
#[inline(always)]
pub fn block_0x00218eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2199220u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ed0));
    } else {
        emu.pc = 2199224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218eb8));
    }
}
#[inline(always)]
pub fn block_0x00218eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2199228u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2199232u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2199236u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2199240u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2199244u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199248u32;
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
pub fn block_0x00218ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2199252u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2199256u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2199260u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ee8));
    } else {
        emu.pc = 2199264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ee0));
    }
}
#[inline(always)]
pub fn block_0x00218ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2199268u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2199272u32;
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
pub fn block_0x00218ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2199276u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f00));
    } else {
        emu.pc = 2199280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218ef0));
    }
}
#[inline(always)]
pub fn block_0x00218ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2199284u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2199288u32);
    emu.apc_no_count(1usize, 2199288u32, 4294873088u32, 2199292u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2199300u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2199304u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2199308u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2199312u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2199316u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2199320u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2199324u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2199328u32);
    emu.apc_no_count(6usize, 2199328u32, 4294873088u32, 2199332u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2199336u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
