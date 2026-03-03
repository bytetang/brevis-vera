pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2100928u32;
pub const PC_MAX: u32 = 2103328u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 137usize] = [
        block_0x00200ec0,
        block_0x00200ec4,
        block_0x00200ed8,
        block_0x00200ee0,
        block_0x00200ee4,
        block_0x00200ee8,
        block_0x00200efc,
        block_0x00200f14,
        block_0x00200f1c,
        block_0x00200f30,
        block_0x00200f34,
        block_0x00200f3c,
        block_0x00200f44,
        block_0x00200f48,
        block_0x00200f50,
        block_0x00200f68,
        block_0x00200f6c,
        block_0x00200f80,
        block_0x00200f8c,
        block_0x00200f90,
        block_0x00200fa0,
        block_0x00200fb8,
        block_0x00200fbc,
        block_0x00200fc0,
        block_0x00200fcc,
        block_0x00200fdc,
        block_0x00200fec,
        block_0x00200ff4,
        block_0x00201008,
        block_0x0020100c,
        block_0x00201010,
        block_0x00201018,
        block_0x0020104c,
        block_0x00201060,
        block_0x00201074,
        block_0x00201080,
        block_0x00201094,
        block_0x002010b8,
        block_0x002010bc,
        block_0x002010c4,
        block_0x002010d8,
        block_0x002010dc,
        block_0x002010e4,
        block_0x002010f4,
        block_0x002010f8,
        block_0x00201100,
        block_0x00201110,
        block_0x00201114,
        block_0x00201118,
        block_0x00201124,
        block_0x00201128,
        block_0x0020114c,
        block_0x00201150,
        block_0x00201168,
        block_0x0020117c,
        block_0x00201180,
        block_0x0020118c,
        block_0x002011c8,
        block_0x002011cc,
        block_0x002011ec,
        block_0x00201208,
        block_0x0020121c,
        block_0x00201228,
        block_0x00201238,
        block_0x00201240,
        block_0x00201244,
        block_0x00201248,
        block_0x00201264,
        block_0x00201280,
        block_0x00201288,
        block_0x0020128c,
        block_0x00201298,
        block_0x002012ac,
        block_0x002012c8,
        block_0x002012cc,
        block_0x002012e8,
        block_0x002012f4,
        block_0x002012fc,
        block_0x00201300,
        block_0x00201310,
        block_0x0020131c,
        block_0x00201330,
        block_0x00201334,
        block_0x00201354,
        block_0x00201358,
        block_0x00201360,
        block_0x00201378,
        block_0x0020138c,
        block_0x002013a0,
        block_0x002013a4,
        block_0x002013bc,
        block_0x002013d4,
        block_0x002013f0,
        block_0x00201404,
        block_0x0020141c,
        block_0x00201424,
        block_0x00201480,
        block_0x0020148c,
        block_0x00201490,
        block_0x002014a0,
        block_0x002014b0,
        block_0x002014d4,
        block_0x00201534,
        block_0x00201540,
        block_0x00201544,
        block_0x00201554,
        block_0x00201568,
        block_0x00201580,
        block_0x0020159c,
        block_0x002015b4,
        block_0x002015cc,
        block_0x002015e0,
        block_0x00201600,
        block_0x00201618,
        block_0x0020161c,
        block_0x00201628,
        block_0x00201644,
        block_0x00201648,
        block_0x00201674,
        block_0x00201688,
        block_0x002016c4,
        block_0x002016dc,
        block_0x002016f4,
        block_0x00201708,
        block_0x00201720,
        block_0x00201734,
        block_0x0020174c,
        block_0x00201754,
        block_0x00201758,
        block_0x00201764,
        block_0x00201778,
        block_0x00201790,
        block_0x002017bc,
        block_0x002017d4,
        block_0x002017ec,
        block_0x00201804,
        block_0x00201820,
    ];
    const IDX: [u16; 601usize] = [
        1u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 4u16, 5u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 0u16, 10u16, 11u16, 0u16, 12u16, 0u16, 13u16, 14u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 16u16, 17u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16,
        19u16, 20u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16,
        23u16, 24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16,
        27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 29u16, 30u16, 31u16, 0u16, 32u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16,
        0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        38u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 42u16, 0u16, 43u16,
        0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 48u16,
        49u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 55u16,
        56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 65u16, 66u16, 67u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        69u16, 0u16, 70u16, 71u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 76u16, 0u16, 0u16, 77u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 80u16,
        0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16,
        0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        97u16, 0u16, 0u16, 98u16, 99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 104u16, 105u16,
        0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16,
        112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 114u16, 115u16, 0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        117u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        119u16, 0u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16, 0u16, 0u16, 0u16, 0u16, 124u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 125u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 127u16, 0u16, 128u16, 129u16, 0u16, 0u16, 130u16, 0u16, 0u16,
        0u16, 0u16, 131u16, 0u16, 0u16, 0u16, 0u16, 0u16, 132u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 133u16, 0u16, 0u16, 0u16, 0u16, 0u16, 134u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 135u16, 0u16, 0u16, 0u16, 0u16, 0u16, 136u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 137u16,
    ];
    if pc < 2100928u32 || pc > 2103328u32 {
        return None;
    }
    let word_offset = ((pc - 2100928u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00200ec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2101496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f8));
    } else {
        emu.pc = 2100932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ec4));
    }
}
#[inline(always)]
pub fn block_0x00200ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2100936u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2100940u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2100944u32);
    emu.lw_no_count(25usize, 10usize, 8u32, 2100948u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2101496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f8));
    } else {
        emu.pc = 2100952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ed8));
    }
}
#[inline(always)]
pub fn block_0x00200ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 23usize, 4u32, 2100956u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2100968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ee8));
    } else {
        emu.pc = 2100960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ee0));
    }
}
#[inline(always)]
pub fn block_0x00200ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2100968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ee8));
    } else {
        emu.pc = 2100964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ee4));
    }
}
#[inline(always)]
pub fn block_0x00200ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2102300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020141c));
    } else {
        emu.pc = 2100968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ee8));
    }
}
#[inline(always)]
pub fn block_0x00200ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2100972u32)?;
    emu.lw_no_count(22usize, 10usize, 16u32, 2100976u32)?;
    emu.sli_no_count(23usize, 18usize, 2u32, 2100980u32);
    emu.mul_no_count(11usize, 23usize, 22usize, 2100984u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2102300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020141c));
    } else {
        emu.pc = 2100988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200efc));
    }
}
#[inline(always)]
pub fn block_0x00200efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 4u32, 2100992u32)?;
    emu.adi_no_count(10usize, 2usize, 348u32, 2100996u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2101000u32);
    emu.adi_no_count(12usize, 25usize, 0u32, 2101004u32);
    emu.apc_no_count(1usize, 2101004u32, 4096u32, 2101008u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 356u32, 2101016u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2103128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201758));
    } else {
        emu.pc = 2101020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f1c));
    }
}
#[inline(always)]
pub fn block_0x00200f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 352u32, 2101024u32)?;
    emu.adi_no_count(11usize, 27usize, 0u32, 2101028u32);
    emu.adi_no_count(12usize, 26usize, 0u32, 2101032u32);
    emu.apc_no_count(1usize, 2101032u32, 94208u32, 2101036u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201758));
    } else {
        emu.pc = 2101044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f34));
    }
}
#[inline(always)]
pub fn block_0x00200f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 270u32, 2101048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2101060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f44));
    } else {
        emu.pc = 2101052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f3c));
    }
}
#[inline(always)]
pub fn block_0x00200f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 23usize, 0u32, 2101056u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2101064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f48));
    } else {
        emu.pc = 2101060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f44));
    }
}
#[inline(always)]
pub fn block_0x00200f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(27usize, 22usize, 2u32, 2101064u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200f48));
}
#[inline(always)]
pub fn block_0x00200f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2101064u32, 16384u32, 2101068u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101072u32;
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
pub fn block_0x00200f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2101076u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101080u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2101084u32);
    emu.adi_no_count(12usize, 25usize, 0u32, 2101088u32);
    emu.apc_no_count(1usize, 2101088u32, 20480u32, 2101092u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017ec));
    } else {
        emu.pc = 2101100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f6c));
    }
}
#[inline(always)]
pub fn block_0x00200f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 10usize, 0u32, 2101104u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2101108u32);
    emu.adi_no_count(12usize, 25usize, 0u32, 2101112u32);
    emu.apc_no_count(1usize, 2101112u32, 16384u32, 2101116u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101120u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 0usize, 270u32, 2101124u32);
    emu.adi_no_count(1usize, 0usize, 180u32, 2101128u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2101344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201060));
    } else {
        emu.pc = 2101132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f8c));
    }
}
#[inline(always)]
pub fn block_0x00200f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2101344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201060));
    } else {
        emu.pc = 2101136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f90));
    }
}
#[inline(always)]
pub fn block_0x00200f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2101140u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2101144u32);
    emu.adi_no_count(13usize, 0usize, 4294967292u32, 2101148u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2101152u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2101152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200fa0));
}
#[inline(always)]
pub fn block_0x00200fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2101156u32);
    emu.sri_no_count(5usize, 13usize, 2u32, 2101160u32);
    emu.adi_no_count(16usize, 15usize, 1u32, 2101164u32);
    emu.xri_no_count(6usize, 15usize, 4294967295u32, 2101168u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2101172u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2101176u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2101176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200fb8));
}
#[inline(always)]
pub fn block_0x00200fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2101228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fec));
    } else {
        emu.pc = 2101180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fbc));
    }
}
#[inline(always)]
pub fn block_0x00200fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2101212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fdc));
    } else {
        emu.pc = 2101184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fc0));
    }
}
#[inline(always)]
pub fn block_0x00200fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 17usize, 0u32, 2101188u32);
    emu.adi_no_count(7usize, 15usize, 0u32, 2101192u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2101236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ff4));
    } else {
        emu.pc = 2101196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fcc));
    }
}
#[inline(always)]
pub fn block_0x00200fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(7usize, 17usize, 4294967295u32, 2101200u32);
    emu.adr_no_count(7usize, 18usize, 7usize, 2101204u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2101208u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ff4));
}
#[inline(always)]
pub fn block_0x00200fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(11usize, 17usize, 4294967295u32, 2101216u32);
    emu.adr_no_count(11usize, 18usize, 11usize, 2101220u32);
    emu.adi_no_count(7usize, 6usize, 0u32, 2101224u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ff4));
}
#[inline(always)]
pub fn block_0x00200fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 6usize, 0u32, 2101232u32);
    emu.adi_no_count(7usize, 17usize, 0u32, 2101236u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2101236u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200ff4));
}
#[inline(always)]
pub fn block_0x00200ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(7usize, 7usize, 27usize, 2101240u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2101244u32);
    emu.adr_no_count(7usize, 7usize, 11usize, 2101248u32);
    emu.adi_no_count(11usize, 7usize, 4u32, 2101252u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2103028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016f4));
    } else {
        emu.pc = 2101256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201008));
    }
}
#[inline(always)]
pub fn block_0x00201008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2103048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201708));
    } else {
        emu.pc = 2101260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020100c));
    }
}
#[inline(always)]
pub fn block_0x0020100c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2103072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201720));
    } else {
        emu.pc = 2101264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201010));
    }
}
#[inline(always)]
pub fn block_0x00201010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 4u32, 2101268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2103092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201734));
    } else {
        emu.pc = 2101272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201018));
    }
}
#[inline]
pub fn block_0x00201018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 17usize, 1u32, 2101276u32);
    emu.adr_no_count(7usize, 26usize, 7usize, 2101280u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2101284u32);
    emu.lbu_no_count(28usize, 10usize, 0u32, 2101288u32);
    emu.lbu_no_count(29usize, 10usize, 1u32, 2101292u32);
    emu.lbu_no_count(30usize, 10usize, 2u32, 2101296u32);
    emu.lbu_no_count(10usize, 10usize, 3u32, 2101300u32);
    emu.sb_no_count(28usize, 7usize, 0u32, 2101304u32);
    emu.sb_no_count(29usize, 7usize, 1u32, 2101308u32);
    emu.sb_no_count(30usize, 7usize, 2u32, 2101312u32);
    emu.sb_no_count(10usize, 7usize, 3u32, 2101316u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2101320u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2101176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fb8));
    } else {
        emu.pc = 2101324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020104c));
    }
}
#[inline(always)]
pub fn block_0x0020104c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 23usize, 2101328u32);
    emu.adr_no_count(14usize, 14usize, 23usize, 2101332u32);
    emu.adr_no_count(12usize, 12usize, 23usize, 2101336u32);
    emu.adi_no_count(15usize, 16usize, 0u32, 2101340u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2101152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fa0));
    } else {
        emu.pc = 2101344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201060));
    }
}
#[inline(always)]
pub fn block_0x00201060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2101348u32);
    emu.adi_no_count(11usize, 26usize, 0u32, 2101352u32);
    emu.adi_no_count(12usize, 25usize, 0u32, 2101356u32);
    emu.apc_no_count(1usize, 2101356u32, 0u32, 2101360u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101364u32;
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
pub fn block_0x00201074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 100u32, 2101368u32)?;
    emu.lw_no_count(25usize, 2usize, 48u32, 2101372u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2103128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201758));
    } else {
        emu.pc = 2101376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201080));
    }
}
#[inline(always)]
pub fn block_0x00201080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 96u32, 2101380u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2101384u32)?;
    emu.adi_no_count(12usize, 25usize, 0u32, 2101388u32);
    emu.apc_no_count(1usize, 2101388u32, 94208u32, 2101392u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101396u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00201094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 64u32, 2101400u32)?;
    emu.lw_no_count(12usize, 2usize, 68u32, 2101404u32)?;
    emu.lw_no_count(26usize, 2usize, 60u32, 2101408u32)?;
    emu.lw_no_count(27usize, 2usize, 56u32, 2101412u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2101416u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2101420u32)?;
    emu.adi_no_count(14usize, 0usize, 270u32, 2101424u32);
    emu.adi_no_count(15usize, 0usize, 180u32, 2101428u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201358));
    } else {
        emu.pc = 2101432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010b8));
    }
}
#[inline(always)]
pub fn block_0x002010b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020141c));
}
#[inline(always)]
pub fn block_0x002010bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 23usize, 4u32, 2101440u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020141c));
    } else {
        emu.pc = 2101444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010c4));
    }
}
#[inline(always)]
pub fn block_0x002010c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 23usize, 8u32, 2101448u32)?;
    emu.lw_no_count(26usize, 2usize, 60u32, 2101452u32)?;
    emu.lw_no_count(27usize, 2usize, 56u32, 2101456u32)?;
    emu.lw_no_count(25usize, 2usize, 48u32, 2101460u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201358));
    } else {
        emu.pc = 2101464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010d8));
    }
}
#[inline(always)]
pub fn block_0x002010d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020141c));
}
#[inline(always)]
pub fn block_0x002010dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 23usize, 12u32, 2101472u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020141c));
    } else {
        emu.pc = 2101476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010e4));
    }
}
#[inline(always)]
pub fn block_0x002010e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 23usize, 16u32, 2101480u32)?;
    emu.lw_no_count(26usize, 2usize, 60u32, 2101484u32)?;
    emu.lw_no_count(27usize, 2usize, 56u32, 2101488u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201358));
    } else {
        emu.pc = 2101492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f4));
    }
}
#[inline(always)]
pub fn block_0x002010f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020141c));
}
#[inline(always)]
pub fn block_0x002010f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 23usize, 4u32, 2101500u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2101528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201118));
    } else {
        emu.pc = 2101504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201100));
    }
}
#[inline(always)]
pub fn block_0x00201100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 60u32, 2101508u32)?;
    emu.lw_no_count(27usize, 2usize, 56u32, 2101512u32)?;
    emu.lw_no_count(25usize, 2usize, 48u32, 2101516u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201358));
    } else {
        emu.pc = 2101520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201110));
    }
}
#[inline(always)]
pub fn block_0x00201110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201358));
    } else {
        emu.pc = 2101524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201114));
    }
}
#[inline(always)]
pub fn block_0x00201114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020141c));
}
#[inline(always)]
pub fn block_0x00201118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 60u32, 2101532u32)?;
    emu.lw_no_count(27usize, 2usize, 56u32, 2101536u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2101540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102100u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201354));
}
#[inline(always)]
pub fn block_0x00201124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 1u32, 2101544u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201128));
}
#[inline]
pub fn block_0x00201128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 2usize, 24u32, 2101548u32)?;
    emu.lw_no_count(7usize, 2usize, 20u32, 2101552u32)?;
    emu.adi_no_count(15usize, 0usize, 0u32, 2101556u32);
    emu.adi_no_count(23usize, 0usize, 0u32, 2101560u32);
    emu.sw_no_count(21usize, 2usize, 92u32, 2101564u32)?;
    emu.sw_no_count(17usize, 2usize, 96u32, 2101568u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2101572u32)?;
    emu.adi_no_count(11usize, 17usize, 0u32, 2101576u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2101580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201150));
}
#[inline(always)]
pub fn block_0x0020114c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2101768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201208));
    } else {
        emu.pc = 2101584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201150));
    }
}
#[inline(always)]
pub fn block_0x00201150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 0u32, 2101588u32);
    emu.mul_no_count(10usize, 23usize, 6usize, 2101592u32);
    emu.adi_no_count(23usize, 23usize, 1u32, 2101596u32);
    emu.divu_no_count(10usize, 10usize, 5usize, 2101600u32);
    emu.mul_no_count(9usize, 10usize, 7usize, 2101604u32);
    emu.adi_no_count(27usize, 22usize, 0u32, 2101608u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2101608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201168));
}
#[inline(always)]
pub fn block_0x00201168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(10usize, 21usize, 22usize, 2101612u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2101616u32);
    emu.adr_no_count(26usize, 10usize, 9usize, 2101620u32);
    emu.adi_no_count(13usize, 26usize, 4u32, 2101624u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2102980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016c4));
    } else {
        emu.pc = 2101628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020117c));
    }
}
#[inline(always)]
pub fn block_0x0020117c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2103004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016dc));
    } else {
        emu.pc = 2101632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201180));
    }
}
#[inline(always)]
pub fn block_0x00201180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 92u32, 2101636u32)?;
    emu.sbr_no_count(10usize, 10usize, 15usize, 2101640u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2101708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011cc));
    } else {
        emu.pc = 2101644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020118c));
    }
}
#[inline]
pub fn block_0x0020118c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 20usize, 26usize, 2101648u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2101652u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2101656u32);
    emu.adi_no_count(27usize, 27usize, 4294967295u32, 2101660u32);
    emu.lbu_no_count(12usize, 26usize, 0u32, 2101664u32);
    emu.lbu_no_count(13usize, 26usize, 1u32, 2101668u32);
    emu.lbu_no_count(14usize, 26usize, 2u32, 2101672u32);
    emu.lbu_no_count(16usize, 26usize, 3u32, 2101676u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2101680u32);
    emu.sb_no_count(13usize, 10usize, 1u32, 2101684u32);
    emu.sb_no_count(14usize, 10usize, 2u32, 2101688u32);
    emu.sb_no_count(16usize, 10usize, 3u32, 2101692u32);
    emu.sw_no_count(15usize, 2usize, 100u32, 2101696u32)?;
    emu.adr_no_count(21usize, 21usize, 18usize, 2101700u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2101608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201168));
    } else {
        emu.pc = 2101704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011c8));
    }
}
#[inline(always)]
pub fn block_0x002011c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101708u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101580u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020114c));
}
#[inline(always)]
pub fn block_0x002011cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2101712u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2101716u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2101720u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2101724u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2101728u32);
    emu.sw_no_count(17usize, 2usize, 12u32, 2101732u32)?;
    emu.apc_no_count(1usize, 2101732u32, 12288u32, 2101736u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101740u32;
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
#[inline(always)]
pub fn block_0x002011ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 2usize, 20u32, 2101744u32)?;
    emu.lw_no_count(6usize, 2usize, 24u32, 2101748u32)?;
    emu.lw_no_count(5usize, 2usize, 44u32, 2101752u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2101756u32)?;
    emu.lw_no_count(11usize, 2usize, 96u32, 2101760u32)?;
    emu.lw_no_count(15usize, 2usize, 100u32, 2101764u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2101768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020118c));
}
#[inline(always)]
pub fn block_0x00201208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 2usize, 12u32, 2101772u32)?;
    emu.adi_no_count(10usize, 2usize, 92u32, 2101776u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2101780u32);
    emu.apc_no_count(1usize, 2101780u32, 0u32, 2101784u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101788u32;
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
pub fn block_0x0020121c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 100u32, 2101792u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2101796u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2103124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201754));
    } else {
        emu.pc = 2101800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201228));
    }
}
#[inline(always)]
pub fn block_0x00201228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 96u32, 2101804u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2101808u32)?;
    emu.apc_no_count(1usize, 2101808u32, 94208u32, 2101812u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101816u32;
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
pub fn block_0x00201238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 72u32, 2101820u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201334));
    } else {
        emu.pc = 2101824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201240));
    }
}
#[inline(always)]
pub fn block_0x00201240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201758));
}
#[inline(always)]
pub fn block_0x00201244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 1u32, 2101832u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201248));
}
#[inline(always)]
pub fn block_0x00201248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 22usize, 0u32, 2101836u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2101840u32)?;
    emu.sw_no_count(21usize, 2usize, 92u32, 2101844u32)?;
    emu.sw_no_count(25usize, 2usize, 96u32, 2101848u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2101852u32)?;
    emu.adi_no_count(27usize, 0usize, 0u32, 2101856u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2102004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012f4));
    } else {
        emu.pc = 2101860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201264));
    }
}
#[inline(always)]
pub fn block_0x00201264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2101864u32)?;
    emu.mul_no_count(10usize, 10usize, 11usize, 2101868u32);
    emu.lw_no_count(21usize, 2usize, 20u32, 2101872u32)?;
    emu.sli_no_count(21usize, 21usize, 2u32, 2101876u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2101880u32);
    emu.adr_no_count(21usize, 10usize, 21usize, 2101884u32);
    emu.adi_no_count(23usize, 25usize, 0u32, 2101888u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2101888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201280));
}
#[inline(always)]
pub fn block_0x00201280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 26usize, 21usize, 2101892u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2103140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201764));
    } else {
        emu.pc = 2101896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201288));
    }
}
#[inline(always)]
pub fn block_0x00201288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2103160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201778));
    } else {
        emu.pc = 2101900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020128c));
    }
}
#[inline(always)]
pub fn block_0x0020128c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 92u32, 2101904u32)?;
    emu.sbr_no_count(10usize, 10usize, 27usize, 2101908u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2101964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012cc));
    } else {
        emu.pc = 2101912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201298));
    }
}
#[inline(always)]
pub fn block_0x00201298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 20usize, 21usize, 2101916u32);
    emu.adr_no_count(10usize, 23usize, 27usize, 2101920u32);
    emu.adi_no_count(12usize, 26usize, 0u32, 2101924u32);
    emu.apc_no_count(1usize, 2101924u32, 16384u32, 2101928u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101932u32;
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
pub fn block_0x002012ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 27usize, 26usize, 2101936u32);
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2101940u32);
    emu.sw_no_count(27usize, 2usize, 100u32, 2101944u32)?;
    emu.lw_no_count(10usize, 2usize, 44u32, 2101948u32)?;
    emu.adr_no_count(21usize, 21usize, 10usize, 2101952u32);
    emu.adi_no_count(13usize, 22usize, 0u32, 2101956u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2101888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201280));
    } else {
        emu.pc = 2101960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012c8));
    }
}
#[inline(always)]
pub fn block_0x002012c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002012fc));
}
#[inline(always)]
pub fn block_0x002012cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2101968u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2101972u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2101976u32);
    emu.adi_no_count(11usize, 27usize, 0u32, 2101980u32);
    emu.adi_no_count(12usize, 26usize, 0u32, 2101984u32);
    emu.apc_no_count(1usize, 2101984u32, 12288u32, 2101988u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101992u32;
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
pub fn block_0x002012e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 96u32, 2101996u32)?;
    emu.lw_no_count(27usize, 2usize, 100u32, 2102000u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2102004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201298));
}
#[inline(always)]
pub fn block_0x002012f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 25usize, 0u32, 2102008u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2102012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201300));
}
#[inline(always)]
pub fn block_0x002012fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 96u32, 2102016u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201300));
}
#[inline(always)]
pub fn block_0x00201300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2102020u32);
    emu.adi_no_count(12usize, 27usize, 0u32, 2102024u32);
    emu.apc_no_count(1usize, 2102024u32, 0u32, 2102028u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 100u32, 2102036u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2102040u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2103128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201758));
    } else {
        emu.pc = 2102044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020131c));
    }
}
#[inline(always)]
pub fn block_0x0020131c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 8u32, 2102048u32)?;
    emu.lw_no_count(10usize, 2usize, 96u32, 2102052u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2102056u32)?;
    emu.apc_no_count(1usize, 2102056u32, 94208u32, 2102060u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201758));
    } else {
        emu.pc = 2102068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201334));
    }
}
#[inline(always)]
pub fn block_0x00201334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 64u32, 2102072u32)?;
    emu.lw_no_count(12usize, 2usize, 68u32, 2102076u32)?;
    emu.lw_no_count(26usize, 2usize, 60u32, 2102080u32)?;
    emu.lw_no_count(27usize, 2usize, 56u32, 2102084u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2102088u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2102092u32)?;
    emu.adi_no_count(14usize, 0usize, 270u32, 2102096u32);
    emu.adi_no_count(15usize, 0usize, 180u32, 2102100u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2102100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201354));
}
#[inline(always)]
pub fn block_0x00201354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 48u32, 2102104u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201358));
}
#[inline(always)]
pub fn block_0x00201358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 8usize, 1u32, 2102108u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2100348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100348u32));
    } else {
        emu.pc = 2102112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201360));
    }
}
#[inline(always)]
pub fn block_0x00201360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 8usize, 3u32, 2102116u32);
    emu.sli_no_count(11usize, 8usize, 6u32, 2102120u32);
    emu.sbr_no_count(10usize, 11usize, 10usize, 2102124u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2102128u32);
    emu.lw_no_count(11usize, 10usize, 40u32, 2102132u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2102300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020141c));
    } else {
        emu.pc = 2102136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201378));
    }
}
#[inline(always)]
pub fn block_0x00201378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 36u32, 2102140u32)?;
    emu.lw_no_count(10usize, 2usize, 52u32, 2102144u32)?;
    emu.adi_no_count(12usize, 25usize, 0u32, 2102148u32);
    emu.apc_no_count(1usize, 2102148u32, 94208u32, 2102152u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020138c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 180u32, 2102160u32);
    emu.adi_no_count(14usize, 0usize, 270u32, 2102164u32);
    emu.lw_no_count(12usize, 2usize, 68u32, 2102168u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2102172u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100348u32));
    } else {
        emu.pc = 2102176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013a0));
    }
}
#[inline(always)]
pub fn block_0x002013a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020141c));
}
#[inline(always)]
pub fn block_0x002013a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2102184u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102188u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965876u32, 2102192u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2102196u32)?;
    emu.apc_no_count(1usize, 2102196u32, 53248u32, 2102200u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102204u32;
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
pub fn block_0x002013bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2102208u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102212u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965908u32, 2102216u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2102220u32)?;
    emu.apc_no_count(1usize, 2102220u32, 53248u32, 2102224u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 26usize, 3u32, 2102232u32);
    emu.sli_no_count(11usize, 26usize, 6u32, 2102236u32);
    emu.sbr_no_count(10usize, 11usize, 10usize, 2102240u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2102244u32);
    emu.lw_no_count(11usize, 10usize, 4294967292u32, 2102248u32)?;
    emu.lw_no_count(14usize, 2usize, 32u32, 2102252u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2102300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020141c));
    } else {
        emu.pc = 2102256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013f0));
    }
}
#[inline(always)]
pub fn block_0x002013f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4294967288u32, 2102260u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2102264u32)?;
    emu.adi_no_count(12usize, 14usize, 0u32, 2102268u32);
    emu.apc_no_count(1usize, 2102268u32, 94208u32, 2102272u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 68u32, 2102280u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2102284u32)?;
    emu.sltiu_no_count(21usize, 10usize, 1u32, 2102288u32);
    emu.sbr_no_count(10usize, 0usize, 21usize, 2102292u32);
    emu.anr_no_count(20usize, 10usize, 26usize, 2102296u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2102300u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102308u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201424));
}
#[inline(always)]
pub fn block_0x0020141c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2102304u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2102308u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2102308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201424));
}
#[inline]
pub fn block_0x00201424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 284u32, 2102312u32)?;
    emu.sw_no_count(0usize, 2usize, 288u32, 2102316u32)?;
    emu.sw_no_count(0usize, 2usize, 292u32, 2102320u32)?;
    emu.sw_no_count(0usize, 2usize, 296u32, 2102324u32)?;
    emu.sw_no_count(0usize, 2usize, 300u32, 2102328u32)?;
    emu.sw_no_count(0usize, 2usize, 304u32, 2102332u32)?;
    emu.sw_no_count(0usize, 2usize, 308u32, 2102336u32)?;
    emu.sw_no_count(0usize, 2usize, 312u32, 2102340u32)?;
    emu.sri_no_count(10usize, 12usize, 1u32, 2102344u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2102348u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2102352u32);
    emu.sw_no_count(13usize, 2usize, 92u32, 2102356u32)?;
    emu.sw_no_count(12usize, 2usize, 96u32, 2102360u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2102364u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2102368u32)?;
    emu.sw_no_count(11usize, 2usize, 108u32, 2102372u32)?;
    emu.sb_no_count(11usize, 2usize, 112u32, 2102376u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102380u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966076u32, 2102384u32);
    emu.adi_no_count(10usize, 2usize, 348u32, 2102388u32);
    emu.adi_no_count(11usize, 2usize, 92u32, 2102392u32);
    emu.apc_no_count(1usize, 2102392u32, 8192u32, 2102396u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 356u32, 2102404u32)?;
    emu.adi_no_count(10usize, 0usize, 32u32, 2102408u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2102416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201490));
    } else {
        emu.pc = 2102412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020148c));
    }
}
#[inline(always)]
pub fn block_0x0020148c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 32u32, 2102416u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201490));
}
#[inline(always)]
pub fn block_0x00201490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 352u32, 2102420u32)?;
    emu.adi_no_count(10usize, 2usize, 284u32, 2102424u32);
    emu.apc_no_count(1usize, 2102424u32, 16384u32, 2102428u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102432u32;
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
pub fn block_0x002014a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102436u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 88u32, 2102440u32)?;
    emu.lw_no_count(18usize, 2usize, 84u32, 2102444u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2102484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014d4));
    } else {
        emu.pc = 2102448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014b0));
    }
}
#[inline]
pub fn block_0x002014b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 332u32, 2102452u32)?;
    emu.sw_no_count(0usize, 2usize, 336u32, 2102456u32)?;
    emu.sw_no_count(0usize, 2usize, 340u32, 2102460u32)?;
    emu.sw_no_count(0usize, 2usize, 344u32, 2102464u32)?;
    emu.sw_no_count(0usize, 2usize, 316u32, 2102468u32)?;
    emu.sw_no_count(0usize, 2usize, 320u32, 2102472u32)?;
    emu.sw_no_count(0usize, 2usize, 324u32, 2102476u32)?;
    emu.sw_no_count(0usize, 2usize, 328u32, 2102480u32)?;
    emu.add_memory_rw_events(9usize);
    let return_addr = 2102484u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201554));
}
#[inline]
pub fn block_0x002014d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 316u32, 2102488u32)?;
    emu.sw_no_count(0usize, 2usize, 320u32, 2102492u32)?;
    emu.sw_no_count(0usize, 2usize, 324u32, 2102496u32)?;
    emu.sw_no_count(0usize, 2usize, 328u32, 2102500u32)?;
    emu.sw_no_count(0usize, 2usize, 332u32, 2102504u32)?;
    emu.sw_no_count(0usize, 2usize, 336u32, 2102508u32)?;
    emu.sw_no_count(0usize, 2usize, 340u32, 2102512u32)?;
    emu.sw_no_count(0usize, 2usize, 344u32, 2102516u32)?;
    emu.sri_no_count(10usize, 9usize, 1u32, 2102520u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2102524u32);
    emu.sbr_no_count(10usize, 9usize, 10usize, 2102528u32);
    emu.lw_no_count(12usize, 2usize, 76u32, 2102532u32)?;
    emu.sw_no_count(12usize, 2usize, 92u32, 2102536u32)?;
    emu.sw_no_count(9usize, 2usize, 96u32, 2102540u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2102544u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2102548u32)?;
    emu.sw_no_count(11usize, 2usize, 108u32, 2102552u32)?;
    emu.sb_no_count(11usize, 2usize, 112u32, 2102556u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102560u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966076u32, 2102564u32);
    emu.adi_no_count(10usize, 2usize, 348u32, 2102568u32);
    emu.adi_no_count(11usize, 2usize, 92u32, 2102572u32);
    emu.apc_no_count(1usize, 2102572u32, 8192u32, 2102576u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 356u32, 2102584u32)?;
    emu.adi_no_count(10usize, 0usize, 32u32, 2102588u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2102596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201544));
    } else {
        emu.pc = 2102592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201540));
    }
}
#[inline(always)]
pub fn block_0x00201540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 32u32, 2102596u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201544));
}
#[inline(always)]
pub fn block_0x00201544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 352u32, 2102600u32)?;
    emu.adi_no_count(10usize, 2usize, 316u32, 2102604u32);
    emu.apc_no_count(1usize, 2102604u32, 16384u32, 2102608u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 220u32, 2102616u32);
    emu.adi_no_count(11usize, 2usize, 316u32, 2102620u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2102624u32);
    emu.apc_no_count(1usize, 2102624u32, 16384u32, 2102628u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(8usize, 20usize, 24u32, 2102636u32);
    emu.adi_no_count(10usize, 2usize, 92u32, 2102640u32);
    emu.adi_no_count(12usize, 0usize, 31u32, 2102644u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2102648u32);
    emu.apc_no_count(1usize, 2102648u32, 16384u32, 2102652u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(44u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 80u32, 2102660u32)?;
    emu.sb_no_count(10usize, 2usize, 123u32, 2102664u32);
    emu.adi_no_count(10usize, 2usize, 124u32, 2102668u32);
    emu.adi_no_count(12usize, 0usize, 31u32, 2102672u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2102676u32);
    emu.apc_no_count(1usize, 2102676u32, 16384u32, 2102680u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(16u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020159c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(18usize, 2usize, 155u32, 2102688u32);
    emu.adi_no_count(10usize, 2usize, 156u32, 2102692u32);
    emu.adi_no_count(12usize, 0usize, 31u32, 2102696u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2102700u32);
    emu.apc_no_count(1usize, 2102700u32, 16384u32, 2102704u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102708u32;
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
pub fn block_0x002015b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(21usize, 2usize, 187u32, 2102712u32);
    emu.adi_no_count(10usize, 2usize, 188u32, 2102716u32);
    emu.adi_no_count(11usize, 2usize, 284u32, 2102720u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2102724u32);
    emu.apc_no_count(1usize, 2102724u32, 16384u32, 2102728u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102732u32;
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
pub fn block_0x002015cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 252u32, 2102736u32);
    emu.adi_no_count(12usize, 0usize, 28u32, 2102740u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2102744u32);
    emu.apc_no_count(1usize, 2102744u32, 16384u32, 2102748u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002015e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 8u32, 2102756u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2102760u32);
    emu.sb_no_count(8usize, 2usize, 280u32, 2102764u32);
    emu.sb_no_count(11usize, 2usize, 281u32, 2102768u32);
    emu.sb_no_count(10usize, 2usize, 282u32, 2102772u32);
    emu.sb_no_count(20usize, 2usize, 283u32, 2102776u32);
    emu.apc_no_count(1usize, 2102776u32, 12288u32, 2102780u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102784u32;
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
pub fn block_0x00201600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102788u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2102792u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2102796u32);
    emu.adi_no_count(12usize, 0usize, 192u32, 2102800u32);
    emu.apc_no_count(1usize, 2102800u32, 16384u32, 2102804u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102808u32;
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
pub fn block_0x00201618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017bc));
    } else {
        emu.pc = 2102812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020161c));
    }
}
#[inline(always)]
pub fn block_0x0020161c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2102816u32);
    emu.apc_no_count(1usize, 2102816u32, 12288u32, 2102820u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102824u32;
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
pub fn block_0x00201628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102828u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2102832u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2102836u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2102840u32);
    emu.adi_no_count(9usize, 0usize, 4u32, 2102844u32);
    emu.apc_no_count(1usize, 2102844u32, 16384u32, 2102848u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017d4));
    } else {
        emu.pc = 2102856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201648));
    }
}
#[inline]
pub fn block_0x00201648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 6u32, 2102860u32);
    emu.sw_no_count(11usize, 2usize, 348u32, 2102864u32)?;
    emu.sw_no_count(8usize, 2usize, 352u32, 2102868u32)?;
    emu.sw_no_count(0usize, 2usize, 356u32, 2102872u32)?;
    emu.sw_no_count(9usize, 2usize, 360u32, 2102876u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2102880u32)?;
    emu.sw_no_count(0usize, 2usize, 368u32, 2102884u32)?;
    emu.adi_no_count(10usize, 2usize, 92u32, 2102888u32);
    emu.adi_no_count(11usize, 2usize, 348u32, 2102892u32);
    emu.apc_no_count(1usize, 2102892u32, 12288u32, 2102896u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102900u32;
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
pub fn block_0x00201674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 356u32, 2102904u32)?;
    emu.lw_no_count(10usize, 2usize, 352u32, 2102908u32)?;
    emu.sli_no_count(11usize, 11usize, 5u32, 2102912u32);
    emu.apc_no_count(1usize, 2102912u32, 12288u32, 2102916u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 428u32, 2102924u32)?;
    emu.lw_no_count(8usize, 2usize, 424u32, 2102928u32)?;
    emu.lw_no_count(9usize, 2usize, 420u32, 2102932u32)?;
    emu.lw_no_count(18usize, 2usize, 416u32, 2102936u32)?;
    emu.lw_no_count(19usize, 2usize, 412u32, 2102940u32)?;
    emu.lw_no_count(20usize, 2usize, 408u32, 2102944u32)?;
    emu.lw_no_count(21usize, 2usize, 404u32, 2102948u32)?;
    emu.lw_no_count(22usize, 2usize, 400u32, 2102952u32)?;
    emu.lw_no_count(23usize, 2usize, 396u32, 2102956u32)?;
    emu.lw_no_count(24usize, 2usize, 392u32, 2102960u32)?;
    emu.lw_no_count(25usize, 2usize, 388u32, 2102964u32)?;
    emu.lw_no_count(26usize, 2usize, 384u32, 2102968u32)?;
    emu.lw_no_count(27usize, 2usize, 380u32, 2102972u32)?;
    emu.adi_no_count(2usize, 2usize, 432u32, 2102976u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102980u32;
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
pub fn block_0x002016c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102984u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965924u32, 2102988u32);
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2102992u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2102996u32);
    emu.apc_no_count(1usize, 2102996u32, 90112u32, 2103000u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002016dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103008u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965924u32, 2103012u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2103016u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2103020u32);
    emu.apc_no_count(1usize, 2103020u32, 90112u32, 2103024u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002016f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103032u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965956u32, 2103036u32);
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2103040u32);
    emu.apc_no_count(1usize, 2103040u32, 90112u32, 2103044u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103052u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965956u32, 2103056u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2103060u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2103064u32);
    emu.apc_no_count(1usize, 2103064u32, 90112u32, 2103068u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 13usize, 14usize, 2103076u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103080u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965972u32, 2103084u32);
    emu.apc_no_count(1usize, 2103084u32, 90112u32, 2103088u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4u32, 2103096u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103100u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965972u32, 2103104u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2103108u32);
    emu.apc_no_count(1usize, 2103108u32, 90112u32, 2103112u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020174c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 20usize, 0u32, 2103120u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2103124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102308u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201424));
}
#[inline(always)]
pub fn block_0x00201754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 72u32, 2103128u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2103128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201758));
}
#[inline(always)]
pub fn block_0x00201758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2103132u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2103136u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2103140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2100244u32));
}
#[inline(always)]
pub fn block_0x00201764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103144u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965892u32, 2103148u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2103152u32);
    emu.apc_no_count(1usize, 2103152u32, 90112u32, 2103156u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103164u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965892u32, 2103168u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2103172u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2103176u32);
    emu.apc_no_count(1usize, 2103176u32, 90112u32, 2103180u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 92u32, 2103188u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103192u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967048u32, 2103196u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2103200u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967032u32, 2103204u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2103208u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967072u32, 2103212u32);
    emu.adi_no_count(11usize, 0usize, 22u32, 2103216u32);
    emu.adi_no_count(12usize, 2usize, 92u32, 2103220u32);
    emu.apc_no_count(1usize, 2103220u32, 73728u32, 2103224u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103228u32;
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
pub fn block_0x002017bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103232u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965804u32, 2103236u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103240u32);
    emu.adi_no_count(11usize, 0usize, 192u32, 2103244u32);
    emu.apc_no_count(1usize, 2103244u32, 49152u32, 2103248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103252u32;
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
pub fn block_0x002017d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103256u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965820u32, 2103260u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2103264u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2103268u32);
    emu.apc_no_count(1usize, 2103268u32, 49152u32, 2103272u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002017ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103280u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965940u32, 2103284u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103288u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2103292u32);
    emu.apc_no_count(1usize, 2103292u32, 49152u32, 2103296u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 8u32, 2103304u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2103308u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103312u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965876u32, 2103316u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2103320u32)?;
    emu.apc_no_count(1usize, 2103320u32, 49152u32, 2103324u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103328u32;
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
#[inline(always)]
pub fn block_0x00201820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 12u32, 2103332u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2103336u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103340u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965908u32, 2103344u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2103348u32)?;
    emu.apc_no_count(1usize, 2103348u32, 49152u32, 2103352u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
