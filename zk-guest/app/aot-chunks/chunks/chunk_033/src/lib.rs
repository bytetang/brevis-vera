pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2232512u32;
pub const PC_MAX: u32 = 2235164u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x002210c0,
        block_0x00221130,
        block_0x00221168,
        block_0x00221188,
        block_0x002211a0,
        block_0x002211b0,
        block_0x002211b8,
        block_0x002211c8,
        block_0x002211d0,
        block_0x002211d8,
        block_0x002211e8,
        block_0x002211f0,
        block_0x00221328,
        block_0x00221338,
        block_0x00221340,
        block_0x00221350,
        block_0x0022135c,
        block_0x0022136c,
        block_0x00221378,
        block_0x00221388,
        block_0x0022138c,
        block_0x0022139c,
        block_0x002214d0,
        block_0x002214e4,
        block_0x002214e8,
        block_0x002214f0,
        block_0x002214f4,
        block_0x00221508,
        block_0x00221528,
        block_0x00221544,
        block_0x00221598,
        block_0x002215a0,
        block_0x002215dc,
        block_0x002215e8,
        block_0x002215fc,
        block_0x00221600,
        block_0x00221608,
        block_0x0022160c,
        block_0x00221624,
        block_0x00221628,
        block_0x0022163c,
        block_0x00221648,
        block_0x00221650,
        block_0x0022165c,
        block_0x00221660,
        block_0x00221664,
        block_0x00221690,
        block_0x00221694,
        block_0x002216b8,
        block_0x002216bc,
        block_0x002216c4,
        block_0x002216f0,
        block_0x002216fc,
        block_0x00221710,
        block_0x00221714,
        block_0x00221724,
        block_0x00221728,
        block_0x00221748,
        block_0x00221750,
        block_0x00221754,
        block_0x00221760,
        block_0x00221784,
        block_0x00221788,
        block_0x00221790,
        block_0x00221794,
        block_0x002217c8,
        block_0x00221818,
        block_0x0022182c,
        block_0x00221830,
        block_0x00221840,
        block_0x00221848,
        block_0x00221850,
        block_0x00221858,
        block_0x00221870,
        block_0x0022187c,
        block_0x00221884,
        block_0x002218a0,
        block_0x002218a4,
        block_0x002218ac,
        block_0x002218c0,
        block_0x002218cc,
        block_0x002218f0,
        block_0x002218f4,
        block_0x00221940,
        block_0x00221950,
        block_0x0022195c,
        block_0x00221960,
        block_0x002219a4,
        block_0x002219ac,
        block_0x002219c0,
        block_0x002219c8,
        block_0x002219dc,
        block_0x002219e0,
        block_0x002219e4,
        block_0x002219f0,
        block_0x002219fc,
        block_0x00221a10,
        block_0x00221a14,
        block_0x00221a24,
        block_0x00221a28,
        block_0x00221a44,
        block_0x00221a4c,
        block_0x00221a60,
        block_0x00221a64,
        block_0x00221a68,
        block_0x00221aa8,
        block_0x00221ac4,
        block_0x00221acc,
        block_0x00221ae0,
        block_0x00221ae4,
        block_0x00221af8,
        block_0x00221b04,
        block_0x00221b18,
        block_0x00221b1c,
    ];
    const IDX: [u16; 664usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16,
        0u16, 8u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 11u16, 0u16, 12u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16,
        0u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 17u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 21u16, 0u16, 0u16,
        0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 26u16, 27u16, 0u16, 0u16,
        0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16,
        0u16, 37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 41u16, 0u16, 0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 44u16, 45u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16, 0u16,
        0u16, 0u16, 0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 59u16, 60u16, 0u16, 0u16, 61u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 64u16, 65u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16,
        0u16, 70u16, 0u16, 71u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        74u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16,
        78u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 86u16, 87u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16,
        0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16,
        92u16, 93u16, 94u16, 0u16, 0u16, 95u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 104u16, 105u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 108u16,
        0u16, 0u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16,
        0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 113u16, 114u16,
    ];
    if pc < 2232512u32 || pc > 2235164u32 {
        return None;
    }
    let word_offset = ((pc - 2232512u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x002210c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2232516u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2232520u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2232524u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2232528u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2232532u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2232536u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2232540u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 24u32, 2232544u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2232548u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2232552u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 8u32, 2232556u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2232560u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 980u32, 2232564u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2232568u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2232572u32)?;
    emu.adi_no_count(17usize, 2usize, 48u32, 2232576u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2232580u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2232584u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2232588u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2232592u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2232596u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2232600u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2232604u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2232608u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2232612u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2232616u32);
    emu.apc_no_count(1usize, 2232616u32, 4294955008u32, 2232620u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00221130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2232628u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2232632u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2232636u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1024u32, 2232640u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2232644u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2232648u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2232652u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2232656u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2232660u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2232664u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2232668u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2232672u32);
    emu.apc_no_count(1usize, 2232672u32, 4294955008u32, 2232676u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00221168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2232684u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2232688u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2232692u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2232696u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2232700u32)?;
    emu.adi_no_count(13usize, 0usize, 39u32, 2232704u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2232708u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2232752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002211b0));
    } else {
        emu.pc = 2232712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221188));
    }
}
#[inline(always)]
pub fn block_0x00221188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 11usize, 2u32, 2232716u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2232720u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1032u32, 2232724u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2232728u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2232732u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(10usize);
    let return_addr = 2232736u32;
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
pub fn block_0x002211a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2232740u32);
    let a = 0u32.wrapping_add(12288u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2232744u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 92u32, 2232748u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2232752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f4));
}
#[inline(always)]
pub fn block_0x002211b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 92u32, 2232756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2232776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002211c8));
    } else {
        emu.pc = 2232760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002211b8));
    }
}
#[inline(always)]
pub fn block_0x002211b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2232764u32);
    let a = 0u32.wrapping_add(24576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2232768u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966364u32, 2232772u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2232776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f4));
}
#[inline(always)]
pub fn block_0x002211c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 1u32, 2232780u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2233208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221378));
    } else {
        emu.pc = 2232784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002211d0));
    }
}
#[inline(always)]
pub fn block_0x002211d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 767u32, 2232788u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2233208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221378));
    } else {
        emu.pc = 2232792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002211d8));
    }
}
#[inline(always)]
pub fn block_0x002211d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2232796u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2232800u32);
    emu.apc_no_count(1usize, 2232800u32, 4294959104u32, 2232804u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002211e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2232812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2233208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221378));
    } else {
        emu.pc = 2232816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002211f0));
    }
}
#[inline(never)]
pub fn block_0x002211f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 11usize, 1u32, 2232820u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2232824u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2232828u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2232832u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 11usize, 20u32, 2232836u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2232840u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1211u32, 2232844u32);
    emu.sli_no_count(17usize, 11usize, 12u32, 2232848u32);
    emu.sli_no_count(5usize, 11usize, 16u32, 2232852u32);
    emu.sli_no_count(6usize, 11usize, 20u32, 2232856u32);
    emu.sli_no_count(7usize, 11usize, 24u32, 2232860u32);
    emu.orr_no_count(14usize, 11usize, 14usize, 2232864u32);
    emu.ani_no_count(11usize, 11usize, 15u32, 2232868u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2232872u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2232876u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2232880u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2232884u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2232888u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2232892u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2232896u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2232900u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2232904u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2232908u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2232912u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2232916u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2232920u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2232924u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2232928u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2232932u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2232936u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2232940u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2232944u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2232948u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2232952u32);
    emu.sh_no_count(0usize, 2usize, 12u32, 2232956u32)?;
    emu.sb_no_count(0usize, 2usize, 14u32, 2232960u32);
    emu.sb_no_count(15usize, 2usize, 15u32, 2232964u32);
    emu.sb_no_count(17usize, 2usize, 16u32, 2232968u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2232972u32);
    emu.adi_no_count(13usize, 13usize, 1365u32, 2232976u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2232980u32);
    emu.lbu_no_count(11usize, 11usize, 0u32, 2232984u32);
    emu.sb_no_count(5usize, 2usize, 17u32, 2232988u32);
    emu.sb_no_count(6usize, 2usize, 18u32, 2232992u32);
    emu.sb_no_count(16usize, 2usize, 19u32, 2232996u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2233000u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2233004u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2233008u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2233012u32);
    emu.anr_no_count(13usize, 16usize, 13usize, 2233016u32);
    emu.adi_no_count(16usize, 2usize, 12u32, 2233020u32);
    emu.adi_no_count(12usize, 12usize, 819u32, 2233024u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2233028u32);
    emu.sbr_no_count(14usize, 14usize, 13usize, 2233032u32);
    emu.anr_no_count(13usize, 14usize, 12usize, 2233036u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2233040u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2233044u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2233048u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2233052u32);
    emu.sri_no_count(13usize, 12usize, 4u32, 2233056u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2233060u32);
    emu.adi_no_count(13usize, 0usize, 117u32, 2233064u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2233068u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2233072u32);
    emu.adi_no_count(12usize, 0usize, 123u32, 2233076u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2233080u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2233084u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2233088u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2233092u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2233096u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2233100u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2233104u32);
    emu.sb_no_count(13usize, 10usize, 4294967295u32, 2233108u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2233112u32);
    emu.sb_no_count(11usize, 2usize, 20u32, 2233116u32);
    emu.sb_no_count(15usize, 2usize, 21u32, 2233120u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2233124u32);
    emu.add_memory_rw_events(78usize);
    let return_addr = 2233128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214d0));
}
#[inline(always)]
pub fn block_0x00221328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2233132u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2233136u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966876u32, 2233140u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2233144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f4));
}
#[inline(always)]
pub fn block_0x00221338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 256u32, 2233148u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2233208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221378));
    } else {
        emu.pc = 2233152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221340));
    }
}
#[inline(always)]
pub fn block_0x00221340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2233156u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2233160u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1884u32, 2233164u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2233168u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f4));
}
#[inline(always)]
pub fn block_0x00221350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2233172u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2233176u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let return_addr = 2233180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f0));
}
#[inline(always)]
pub fn block_0x0022135c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2233184u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2233188u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1116u32, 2233192u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2233196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f4));
}
#[inline(always)]
pub fn block_0x0022136c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 8u32, 2233200u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2233204u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2233576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002214e8));
    } else {
        emu.pc = 2233208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221378));
    }
}
#[inline(always)]
pub fn block_0x00221378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2233212u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2233216u32);
    emu.apc_no_count(1usize, 2233216u32, 8192u32, 2233220u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2233224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00221388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2233244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022139c));
    } else {
        emu.pc = 2233228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022138c));
    }
}
#[inline(always)]
pub fn block_0x0022138c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2233232u32)?;
    emu.adi_no_count(18usize, 0usize, 129u32, 2233236u32);
    emu.adi_no_count(9usize, 0usize, 128u32, 2233240u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2233244u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221508));
}
#[inline(never)]
pub fn block_0x0022139c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 77u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 9usize, 1u32, 2233248u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2233252u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2233256u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2233260u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 9usize, 20u32, 2233264u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2233268u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1211u32, 2233272u32);
    emu.sli_no_count(17usize, 9usize, 12u32, 2233276u32);
    emu.sli_no_count(5usize, 9usize, 16u32, 2233280u32);
    emu.sli_no_count(6usize, 9usize, 20u32, 2233284u32);
    emu.sli_no_count(7usize, 9usize, 24u32, 2233288u32);
    emu.orr_no_count(14usize, 9usize, 14usize, 2233292u32);
    emu.ani_no_count(13usize, 9usize, 15u32, 2233296u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2233300u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2233304u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2233308u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2233312u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2233316u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2233320u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2233324u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2233328u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2233332u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2233336u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2233340u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2233344u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2233348u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2233352u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2233356u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2233360u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2233364u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2233368u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2233372u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2233376u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2233380u32);
    emu.sh_no_count(0usize, 2usize, 22u32, 2233384u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2233388u32);
    emu.sb_no_count(15usize, 2usize, 25u32, 2233392u32);
    emu.sb_no_count(17usize, 2usize, 26u32, 2233396u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2233400u32);
    emu.adi_no_count(12usize, 12usize, 1365u32, 2233404u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2233408u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2233412u32);
    emu.sb_no_count(5usize, 2usize, 27u32, 2233416u32);
    emu.sb_no_count(6usize, 2usize, 28u32, 2233420u32);
    emu.sb_no_count(16usize, 2usize, 29u32, 2233424u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2233428u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2233432u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2233436u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2233440u32);
    emu.anr_no_count(12usize, 16usize, 12usize, 2233444u32);
    emu.adi_no_count(16usize, 2usize, 22u32, 2233448u32);
    emu.adi_no_count(11usize, 11usize, 819u32, 2233452u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2233456u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2233460u32);
    emu.anr_no_count(12usize, 14usize, 11usize, 2233464u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2233468u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2233472u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2233476u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2233480u32);
    emu.sri_no_count(12usize, 11usize, 4u32, 2233484u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2233488u32);
    emu.adi_no_count(12usize, 0usize, 117u32, 2233492u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2233496u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2233500u32);
    emu.adi_no_count(11usize, 0usize, 123u32, 2233504u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2233508u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2233512u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2233516u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2233520u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2233524u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2233528u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2233532u32);
    emu.sb_no_count(12usize, 10usize, 4294967295u32, 2233536u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2233540u32);
    emu.sb_no_count(13usize, 2usize, 30u32, 2233544u32);
    emu.sb_no_count(15usize, 2usize, 31u32, 2233548u32);
    emu.adi_no_count(11usize, 2usize, 22u32, 2233552u32);
    emu.add_memory_rw_events(77usize);
    emu.pc = 2233552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214d0));
}
#[inline(always)]
pub fn block_0x002214d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 10u32, 2233556u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2233560u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2233564u32);
    emu.apc_no_count(1usize, 2233564u32, 4294873088u32, 2233568u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2233572u32;
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
pub fn block_0x002214e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2233576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221508));
}
#[inline(always)]
pub fn block_0x002214e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2233580u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2233584u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2233584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f0));
}
#[inline(always)]
pub fn block_0x002214f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 604u32, 2233588u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2233588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002214f4));
}
#[inline(always)]
pub fn block_0x002214f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 0u32, 2233592u32)?;
    emu.sh_no_count(0usize, 8usize, 4u32, 2233596u32)?;
    emu.sh_no_count(0usize, 8usize, 6u32, 2233600u32)?;
    emu.sh_no_count(0usize, 8usize, 8u32, 2233604u32)?;
    emu.adi_no_count(18usize, 0usize, 2u32, 2233608u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2233608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221508));
}
#[inline(always)]
pub fn block_0x00221508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 12u32, 2233612u32);
    emu.sb_no_count(18usize, 8usize, 13u32, 2233616u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2233620u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2233624u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2233628u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2233632u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2233636u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2233640u32;
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
pub fn block_0x00221528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2233644u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2233648u32)?;
    emu.adi_no_count(13usize, 10usize, 0u32, 2233652u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2233656u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2233660u32);
    emu.apc_no_count(6usize, 2233660u32, 0u32, 2233664u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2233668u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00221544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2233672u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2233676u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2233680u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2233684u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2233688u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2233692u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2233696u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2233700u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2233704u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2233708u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2233712u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2233716u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2233720u32);
    let a = 0u32.wrapping_add(3758096384u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2233724u32;
    emu.update_insn_clock();
    emu.lw_no_count(21usize, 8usize, 16u32, 2233728u32)?;
    emu.adi_no_count(12usize, 12usize, 32u32, 2233732u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2233736u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2233740u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2233744u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2233748u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2234044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002216bc));
    } else {
        emu.pc = 2233752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221598));
    }
}
#[inline(always)]
pub fn block_0x00221598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 20u32, 2233756u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2234196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221754));
    } else {
        emu.pc = 2233760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002215a0));
    }
}
#[inline]
pub fn block_0x002215a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2233764u32);
    emu.sli_no_count(12usize, 11usize, 3u32, 2233768u32);
    emu.sli_no_count(13usize, 11usize, 5u32, 2233772u32);
    emu.adi_no_count(10usize, 21usize, 24u32, 2233776u32);
    emu.lw_no_count(23usize, 8usize, 0u32, 2233780u32)?;
    emu.lw_no_count(19usize, 8usize, 8u32, 2233784u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2233788u32);
    emu.adi_no_count(20usize, 0usize, 2u32, 2233792u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2233796u32);
    emu.sli_no_count(11usize, 11usize, 3u32, 2233800u32);
    emu.adr_no_count(22usize, 21usize, 13usize, 2233804u32);
    emu.sri_no_count(11usize, 11usize, 3u32, 2233808u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2233812u32);
    emu.adi_no_count(23usize, 23usize, 4u32, 2233816u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2233820u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2233820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002215dc));
}
#[inline(always)]
pub fn block_0x002215dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 0u32, 2233824u32)?;
    emu.adi_no_count(25usize, 10usize, 0u32, 2233828u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2233856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221600));
    } else {
        emu.pc = 2233832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002215e8));
    }
}
#[inline(always)]
pub fn block_0x002215e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2233836u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2233840u32)?;
    emu.lw_no_count(11usize, 23usize, 4294967292u32, 2233844u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2233848u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2233852u32;
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
pub fn block_0x002215fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221788));
    } else {
        emu.pc = 2233856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221600));
    }
}
#[inline(always)]
pub fn block_0x00221600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 21usize, 8u32, 2233860u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2233916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022163c));
    } else {
        emu.pc = 2233864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221608));
    }
}
#[inline(always)]
pub fn block_0x00221608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2233936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221650));
    } else {
        emu.pc = 2233868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022160c));
    }
}
#[inline(always)]
pub fn block_0x0022160c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 12u32, 2233872u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2233876u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2233880u32);
    emu.lhu_no_count(11usize, 10usize, 4u32, 2233884u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2233888u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2233928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221648));
    } else {
        emu.pc = 2233892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221624));
    }
}
#[inline(always)]
pub fn block_0x00221624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2233952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221660));
    } else {
        emu.pc = 2233896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221628));
    }
}
#[inline(always)]
pub fn block_0x00221628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2233900u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2233904u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2233908u32);
    emu.lhu_no_count(12usize, 10usize, 4u32, 2233912u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2233916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233956u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221664));
}
#[inline(always)]
pub fn block_0x0022163c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 21usize, 10u32, 2233920u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2233924u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2233892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221624));
    } else {
        emu.pc = 2233928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221648));
    }
}
#[inline(always)]
pub fn block_0x00221648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2233932u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2233936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233956u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221664));
}
#[inline(always)]
pub fn block_0x00221650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2233940u32);
    emu.lhu_no_count(10usize, 21usize, 0u32, 2233944u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2233892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221624));
    } else {
        emu.pc = 2233948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022165c));
    }
}
#[inline(always)]
pub fn block_0x0022165c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2233952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2233928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221648));
}
#[inline(always)]
pub fn block_0x00221660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 21usize, 2u32, 2233956u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2233956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221664));
}
#[inline]
pub fn block_0x00221664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 16u32, 2233960u32)?;
    emu.lw_no_count(13usize, 21usize, 20u32, 2233964u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2233968u32);
    emu.adr_no_count(14usize, 19usize, 10usize, 2233972u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2233976u32)?;
    emu.lw_no_count(14usize, 14usize, 4u32, 2233980u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2233984u32)?;
    emu.sh_no_count(11usize, 2usize, 16u32, 2233988u32)?;
    emu.sh_no_count(12usize, 2usize, 18u32, 2233992u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2233996u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2234000u32;
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
pub fn block_0x00221690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221788));
    } else {
        emu.pc = 2234004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221694));
    }
}
#[inline]
pub fn block_0x00221694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2234008u32);
    emu.xrr_no_count(10usize, 25usize, 22usize, 2234012u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2234016u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2234020u32);
    emu.ani_no_count(10usize, 10usize, 24u32, 2234024u32);
    emu.adr_no_count(10usize, 25usize, 10usize, 2234028u32);
    emu.adi_no_count(23usize, 23usize, 8u32, 2234032u32);
    emu.adi_no_count(21usize, 25usize, 0u32, 2234036u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2233820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002215dc));
    } else {
        emu.pc = 2234040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002216b8));
    }
}
#[inline(always)]
pub fn block_0x002216b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2234044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221748));
}
#[inline(always)]
pub fn block_0x002216bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2234048u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2234196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221754));
    } else {
        emu.pc = 2234052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002216c4));
    }
}
#[inline]
pub fn block_0x002216c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2234056u32);
    emu.lw_no_count(20usize, 8usize, 0u32, 2234060u32)?;
    emu.lw_no_count(21usize, 8usize, 8u32, 2234064u32)?;
    emu.sli_no_count(19usize, 10usize, 3u32, 2234068u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2234072u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2234076u32);
    emu.sri_no_count(10usize, 10usize, 3u32, 2234080u32);
    emu.adi_no_count(9usize, 10usize, 1u32, 2234084u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2234088u32);
    emu.adi_no_count(10usize, 21usize, 8u32, 2234092u32);
    emu.adi_no_count(20usize, 20usize, 4u32, 2234096u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2234096u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002216f0));
}
#[inline(always)]
pub fn block_0x002216f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2234100u32)?;
    emu.adi_no_count(22usize, 10usize, 0u32, 2234104u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2234132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221714));
    } else {
        emu.pc = 2234108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002216fc));
    }
}
#[inline(always)]
pub fn block_0x002216fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2234112u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2234116u32)?;
    emu.lw_no_count(11usize, 20usize, 4294967292u32, 2234120u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2234124u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2234128u32;
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
pub fn block_0x00221710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221788));
    } else {
        emu.pc = 2234132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221714));
    }
}
#[inline(always)]
pub fn block_0x00221714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 0u32, 2234136u32)?;
    emu.lw_no_count(12usize, 21usize, 4u32, 2234140u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2234144u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2234148u32;
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
pub fn block_0x00221724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221788));
    } else {
        emu.pc = 2234152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221728));
    }
}
#[inline(always)]
pub fn block_0x00221728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2234156u32);
    emu.xrr_no_count(10usize, 22usize, 19usize, 2234160u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2234164u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2234168u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2234172u32);
    emu.adi_no_count(20usize, 20usize, 8u32, 2234176u32);
    emu.adi_no_count(21usize, 22usize, 0u32, 2234180u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2234096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002216f0));
    } else {
        emu.pc = 2234184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221748));
    }
}
#[inline(always)]
pub fn block_0x00221748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2234188u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2234208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221760));
    } else {
        emu.pc = 2234192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221750));
    }
}
#[inline(always)]
pub fn block_0x00221750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2234196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221790));
}
#[inline(always)]
pub fn block_0x00221754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2234200u32);
    emu.lw_no_count(10usize, 8usize, 4u32, 2234204u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(0usize);
    if a >= b {
        emu.pc = 2234256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221790));
    } else {
        emu.pc = 2234208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221760));
    }
}
#[inline]
pub fn block_0x00221760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 0u32, 2234212u32)?;
    emu.sli_no_count(9usize, 9usize, 3u32, 2234216u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2234220u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2234224u32)?;
    emu.adr_no_count(9usize, 11usize, 9usize, 2234228u32);
    emu.lw_no_count(11usize, 9usize, 0u32, 2234232u32)?;
    emu.lw_no_count(12usize, 9usize, 4u32, 2234236u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2234240u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2234244u32;
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
pub fn block_0x00221784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221790));
    } else {
        emu.pc = 2234248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221788));
    }
}
#[inline(always)]
pub fn block_0x00221788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2234252u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2234256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221794));
}
#[inline(always)]
pub fn block_0x00221790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2234260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2234260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221794));
}
#[inline]
pub fn block_0x00221794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2234264u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2234268u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2234272u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2234276u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2234280u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2234284u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2234288u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2234292u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2234296u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2234300u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2234304u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2234308u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2234312u32;
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
pub fn block_0x002217c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2234316u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2234320u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2234324u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2234328u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2234332u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2234336u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2234340u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2234344u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2234348u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2234352u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2234356u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2234360u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2234364u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2234368u32)?;
    emu.adi_no_count(8usize, 15usize, 0u32, 2234372u32);
    emu.adi_no_count(9usize, 14usize, 0u32, 2234376u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2234380u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2234384u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2234388u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2234540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218ac));
    } else {
        emu.pc = 2234392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221818));
    }
}
#[inline(always)]
pub fn block_0x00221818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2234396u32)?;
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2234400u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 22usize, 10usize, 2234404u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2234408u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2234416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221830));
    } else {
        emu.pc = 2234412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022182c));
    }
}
#[inline(always)]
pub fn block_0x0022182c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 43u32, 2234416u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2234416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221830));
}
#[inline(always)]
pub fn block_0x00221830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 21u32, 2234420u32);
    emu.adr_no_count(24usize, 10usize, 8usize, 2234424u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2234428u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2234560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218c0));
    } else {
        emu.pc = 2234432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221840));
    }
}
#[inline(always)]
pub fn block_0x00221840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 16u32, 2234436u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2234688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221940));
    } else {
        emu.pc = 2234440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221848));
    }
}
#[inline(always)]
pub fn block_0x00221848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2234444u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2234480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221870));
    } else {
        emu.pc = 2234448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221850));
    }
}
#[inline(always)]
pub fn block_0x00221850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 20usize, 19usize, 2234452u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2234456u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2234456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221858));
}
#[inline(always)]
pub fn block_0x00221858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2234460u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2234464u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2234468u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2234472u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2234476u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2234456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221858));
    } else {
        emu.pc = 2234480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221870));
    }
}
#[inline(always)]
pub fn block_0x00221870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2234484u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2234488u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2234572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218cc));
    } else {
        emu.pc = 2234492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022187c));
    }
}
#[inline(always)]
pub fn block_0x0022187c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 22usize, 7u32, 2234496u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2234720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221960));
    } else {
        emu.pc = 2234500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221884));
    }
}
#[inline(always)]
pub fn block_0x00221884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(24usize, 27usize, 24usize, 2234504u32);
    emu.sli_no_count(10usize, 22usize, 1u32, 2234508u32);
    emu.sri_no_count(10usize, 10usize, 30u32, 2234512u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2234516u32);
    emu.sli_no_count(22usize, 22usize, 11u32, 2234520u32);
    emu.sw_no_count(9usize, 2usize, 8u32, 2234524u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2234852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002219e4));
    } else {
        emu.pc = 2234528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218a0));
    }
}
#[inline(always)]
pub fn block_0x002218a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a24));
    } else {
        emu.pc = 2234532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218a4));
    }
}
#[inline(always)]
pub fn block_0x002218a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2234536u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2234540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234920u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a28));
}
#[inline(always)]
pub fn block_0x002218ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2234544u32)?;
    emu.adi_no_count(24usize, 8usize, 1u32, 2234548u32);
    emu.adi_no_count(21usize, 0usize, 45u32, 2234552u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2234556u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2234432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221840));
    } else {
        emu.pc = 2234560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218c0));
    }
}
#[inline(always)]
pub fn block_0x002218c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2234564u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2234568u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a < b {
        emu.pc = 2234492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022187c));
    } else {
        emu.pc = 2234572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218cc));
    }
}
#[inline]
pub fn block_0x002218cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2234576u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2234580u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2234584u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2234588u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2234592u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2234596u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2234600u32);
    emu.apc_no_count(1usize, 2234600u32, 0u32, 2234604u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2234608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002218f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a64));
    } else {
        emu.pc = 2234612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218f4));
    }
}
#[inline]
pub fn block_0x002218f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2234616u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2234620u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2234624u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2234628u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2234632u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2234636u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2234640u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2234644u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2234648u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2234652u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2234656u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2234660u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2234664u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2234668u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2234672u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2234676u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2234680u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2234684u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2234688u32;
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
pub fn block_0x00221940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2234692u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2234696u32);
    emu.apc_no_count(1usize, 2234696u32, 16384u32, 2234700u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2234704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00221950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2234708u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2234712u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2234572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002218cc));
    } else {
        emu.pc = 2234716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022195c));
    }
}
#[inline(always)]
pub fn block_0x0022195c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2234720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234492u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022187c));
}
#[inline]
pub fn block_0x00221960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2234724u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2234728u32)?;
    emu.lw_no_count(25usize, 18usize, 8u32, 2234732u32)?;
    emu.lw_no_count(26usize, 18usize, 12u32, 2234736u32)?;
    let a = 0u32.wrapping_add(2682257408u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2234740u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2234744u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 25usize, 10usize, 2234748u32);
    emu.adi_no_count(11usize, 11usize, 48u32, 2234752u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2234756u32);
    emu.sw_no_count(10usize, 18usize, 8u32, 2234760u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2234764u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2234768u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2234772u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2234776u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2234780u32);
    emu.apc_no_count(1usize, 2234780u32, 0u32, 2234784u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2234788u32;
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
#[inline(always)]
pub fn block_0x002219a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2234792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2234984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a68));
    } else {
        emu.pc = 2234796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002219ac));
    }
}
#[inline(always)]
pub fn block_0x002219ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2234800u32);
    emu.sbr_no_count(10usize, 27usize, 24usize, 2234804u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2234808u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2234812u32);
    emu.anr_no_count(24usize, 10usize, 21usize, 2234816u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2234816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002219c0));
}
#[inline(always)]
pub fn block_0x002219c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 20usize, 21usize, 2234820u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2234876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002219fc));
    } else {
        emu.pc = 2234824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002219c8));
    }
}
#[inline(always)]
pub fn block_0x002219c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 16u32, 2234828u32)?;
    emu.adi_no_count(20usize, 20usize, 1u32, 2234832u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2234836u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2234840u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2234844u32;
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
pub fn block_0x002219dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002219c0));
    } else {
        emu.pc = 2234848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002219e0));
    }
}
#[inline(always)]
pub fn block_0x002219e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2234852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a68));
}
#[inline(always)]
pub fn block_0x002219e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2234856u32);
    emu.adi_no_count(25usize, 24usize, 0u32, 2234860u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2234920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a28));
    } else {
        emu.pc = 2234864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002219f0));
    }
}
#[inline(always)]
pub fn block_0x002219f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 24usize, 16u32, 2234868u32);
    emu.sri_no_count(25usize, 10usize, 17u32, 2234872u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2234876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234920u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a28));
}
#[inline(always)]
pub fn block_0x002219fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 23usize, 12u32, 2234880u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2234884u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2234888u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2234892u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2234896u32;
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
pub fn block_0x00221a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a68));
    } else {
        emu.pc = 2234900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a14));
    }
}
#[inline(always)]
pub fn block_0x00221a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2234904u32);
    emu.sw_no_count(25usize, 18usize, 8u32, 2234908u32)?;
    emu.sw_no_count(26usize, 18usize, 12u32, 2234912u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2234916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a68));
}
#[inline(always)]
pub fn block_0x00221a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2234920u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2234920u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a28));
}
#[inline(always)]
pub fn block_0x00221a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 0u32, 2234924u32);
    emu.sri_no_count(22usize, 22usize, 11u32, 2234928u32);
    emu.lw_no_count(23usize, 18usize, 0u32, 2234932u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2234936u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2234940u32;
    emu.update_insn_clock();
    emu.adi_no_count(27usize, 27usize, 4294967295u32, 2234944u32);
    emu.anr_no_count(9usize, 25usize, 27usize, 2234948u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2234948u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a44));
}
#[inline(always)]
pub fn block_0x00221a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 26usize, 27usize, 2234952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2235048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221aa8));
    } else {
        emu.pc = 2234956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a4c));
    }
}
#[inline(always)]
pub fn block_0x00221a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2234960u32)?;
    emu.adi_no_count(26usize, 26usize, 1u32, 2234964u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2234968u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2234972u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2234976u32;
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
pub fn block_0x00221a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a44));
    } else {
        emu.pc = 2234980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a64));
    }
}
#[inline(always)]
pub fn block_0x00221a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2234984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2234984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a68));
}
#[inline]
pub fn block_0x00221a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2234988u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2234992u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2234996u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2235000u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2235004u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2235008u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2235012u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2235016u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2235020u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2235024u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2235028u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2235032u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2235036u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2235040u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2235044u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2235048u32;
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
pub fn block_0x00221aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 23usize, 0u32, 2235052u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2235056u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2235060u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2235064u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2235068u32);
    emu.apc_no_count(1usize, 2235068u32, 0u32, 2235072u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2235076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00221ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2235080u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2234984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a68));
    } else {
        emu.pc = 2235084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221acc));
    }
}
#[inline(always)]
pub fn block_0x00221acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 12u32, 2235088u32)?;
    emu.adi_no_count(10usize, 23usize, 0u32, 2235092u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2235096u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2235100u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2235104u32;
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
pub fn block_0x00221ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2234984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a68));
    } else {
        emu.pc = 2235108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ae4));
    }
}
#[inline(always)]
pub fn block_0x00221ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2235112u32);
    emu.sbr_no_count(10usize, 24usize, 25usize, 2235116u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2235120u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2235124u32);
    emu.anr_no_count(20usize, 10usize, 9usize, 2235128u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2235128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221af8));
}
#[inline(always)]
pub fn block_0x00221af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 8usize, 9usize, 2235132u32);
    emu.sltru_no_count(19usize, 10usize, 20usize, 2235136u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2234984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221a68));
    } else {
        emu.pc = 2235140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b04));
    }
}
#[inline(always)]
pub fn block_0x00221b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2235144u32)?;
    emu.adi_no_count(8usize, 8usize, 1u32, 2235148u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2235152u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2235156u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2235160u32;
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
pub fn block_0x00221b18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2235128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221af8));
    } else {
        emu.pc = 2235164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b1c));
    }
}
#[inline(always)]
pub fn block_0x00221b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2235168u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2234984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221a68));
}
