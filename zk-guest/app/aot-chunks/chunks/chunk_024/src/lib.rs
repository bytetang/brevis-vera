pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2175524u32;
pub const PC_MAX: u32 = 2178176u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x00213224,
        block_0x00213294,
        block_0x002132cc,
        block_0x002132ec,
        block_0x00213304,
        block_0x00213314,
        block_0x0021331c,
        block_0x0021332c,
        block_0x00213334,
        block_0x0021333c,
        block_0x0021334c,
        block_0x00213354,
        block_0x0021348c,
        block_0x0021349c,
        block_0x002134a4,
        block_0x002134b4,
        block_0x002134c0,
        block_0x002134d0,
        block_0x002134dc,
        block_0x002134ec,
        block_0x002134f0,
        block_0x00213500,
        block_0x00213634,
        block_0x00213648,
        block_0x0021364c,
        block_0x00213654,
        block_0x00213658,
        block_0x0021366c,
        block_0x0021368c,
        block_0x002136a8,
        block_0x002136fc,
        block_0x00213704,
        block_0x00213740,
        block_0x0021374c,
        block_0x00213760,
        block_0x00213764,
        block_0x0021376c,
        block_0x00213770,
        block_0x00213788,
        block_0x0021378c,
        block_0x002137a0,
        block_0x002137ac,
        block_0x002137b4,
        block_0x002137c0,
        block_0x002137c4,
        block_0x002137c8,
        block_0x002137f4,
        block_0x002137f8,
        block_0x0021381c,
        block_0x00213820,
        block_0x00213828,
        block_0x00213854,
        block_0x00213860,
        block_0x00213874,
        block_0x00213878,
        block_0x00213888,
        block_0x0021388c,
        block_0x002138ac,
        block_0x002138b4,
        block_0x002138b8,
        block_0x002138c4,
        block_0x002138e8,
        block_0x002138ec,
        block_0x002138f4,
        block_0x002138f8,
        block_0x0021392c,
        block_0x0021397c,
        block_0x00213990,
        block_0x00213994,
        block_0x002139a4,
        block_0x002139ac,
        block_0x002139b4,
        block_0x002139bc,
        block_0x002139d4,
        block_0x002139e0,
        block_0x002139e8,
        block_0x00213a04,
        block_0x00213a08,
        block_0x00213a10,
        block_0x00213a24,
        block_0x00213a30,
        block_0x00213a54,
        block_0x00213a58,
        block_0x00213aa4,
        block_0x00213ab4,
        block_0x00213ac0,
        block_0x00213ac4,
        block_0x00213b08,
        block_0x00213b10,
        block_0x00213b24,
        block_0x00213b2c,
        block_0x00213b40,
        block_0x00213b44,
        block_0x00213b48,
        block_0x00213b54,
        block_0x00213b60,
        block_0x00213b74,
        block_0x00213b78,
        block_0x00213b88,
        block_0x00213b8c,
        block_0x00213ba8,
        block_0x00213bb0,
        block_0x00213bc4,
        block_0x00213bc8,
        block_0x00213bcc,
        block_0x00213c0c,
        block_0x00213c28,
        block_0x00213c30,
        block_0x00213c44,
        block_0x00213c48,
        block_0x00213c5c,
        block_0x00213c68,
        block_0x00213c7c,
        block_0x00213c80,
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
    if pc < 2175524u32 || pc > 2178176u32 {
        return None;
    }
    let word_offset = ((pc - 2175524u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x00213224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2175528u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2175532u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2175536u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2175540u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2175544u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2175548u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2175552u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 144u32, 2175556u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2175560u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2175564u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 128u32, 2175568u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2175572u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 852u32, 2175576u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2175580u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2175584u32)?;
    emu.adi_no_count(17usize, 2usize, 48u32, 2175588u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2175592u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2175596u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2175600u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2175604u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2175608u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2175612u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2175616u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2175620u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2175624u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2175628u32);
    emu.apc_no_count(1usize, 2175628u32, 4294955008u32, 2175632u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175636u32;
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
pub fn block_0x00213294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2175640u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2175644u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175648u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 896u32, 2175652u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2175656u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2175660u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2175664u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2175668u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2175672u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2175676u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2175680u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2175684u32);
    emu.apc_no_count(1usize, 2175684u32, 4294955008u32, 2175688u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175692u32;
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
pub fn block_0x002132cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2175696u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2175700u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2175704u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2175708u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2175712u32)?;
    emu.adi_no_count(13usize, 0usize, 39u32, 2175716u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2175720u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2175764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213314));
    } else {
        emu.pc = 2175724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002132ec));
    }
}
#[inline(always)]
pub fn block_0x002132ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 11usize, 2u32, 2175728u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2175732u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 904u32, 2175736u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2175740u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2175744u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(10usize);
    let return_addr = 2175748u32;
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
pub fn block_0x00213304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2175752u32);
    let a = 0u32.wrapping_add(12288u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175756u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 92u32, 2175760u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2175764u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213658));
}
#[inline(always)]
pub fn block_0x00213314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 92u32, 2175768u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2175788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021332c));
    } else {
        emu.pc = 2175772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021331c));
    }
}
#[inline(always)]
pub fn block_0x0021331c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2175776u32);
    let a = 0u32.wrapping_add(24576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175780u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966364u32, 2175784u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2175788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213658));
}
#[inline(always)]
pub fn block_0x0021332c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 1u32, 2175792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2176220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134dc));
    } else {
        emu.pc = 2175796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213334));
    }
}
#[inline(always)]
pub fn block_0x00213334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 767u32, 2175800u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2176220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134dc));
    } else {
        emu.pc = 2175804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021333c));
    }
}
#[inline(always)]
pub fn block_0x0021333c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2175808u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2175812u32);
    emu.apc_no_count(1usize, 2175812u32, 4294959104u32, 2175816u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175820u32;
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
pub fn block_0x0021334c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2175824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2176220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134dc));
    } else {
        emu.pc = 2175828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213354));
    }
}
#[inline(never)]
pub fn block_0x00213354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 11usize, 1u32, 2175832u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2175836u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2175840u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2175844u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 11usize, 20u32, 2175848u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2175852u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 644u32, 2175856u32);
    emu.sli_no_count(17usize, 11usize, 12u32, 2175860u32);
    emu.sli_no_count(5usize, 11usize, 16u32, 2175864u32);
    emu.sli_no_count(6usize, 11usize, 20u32, 2175868u32);
    emu.sli_no_count(7usize, 11usize, 24u32, 2175872u32);
    emu.orr_no_count(14usize, 11usize, 14usize, 2175876u32);
    emu.ani_no_count(11usize, 11usize, 15u32, 2175880u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2175884u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2175888u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2175892u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2175896u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2175900u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2175904u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2175908u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2175912u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2175916u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2175920u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2175924u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2175928u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2175932u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2175936u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2175940u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2175944u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2175948u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2175952u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2175956u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2175960u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2175964u32);
    emu.sh_no_count(0usize, 2usize, 12u32, 2175968u32)?;
    emu.sb_no_count(0usize, 2usize, 14u32, 2175972u32);
    emu.sb_no_count(15usize, 2usize, 15u32, 2175976u32);
    emu.sb_no_count(17usize, 2usize, 16u32, 2175980u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2175984u32);
    emu.adi_no_count(13usize, 13usize, 1365u32, 2175988u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2175992u32);
    emu.lbu_no_count(11usize, 11usize, 0u32, 2175996u32);
    emu.sb_no_count(5usize, 2usize, 17u32, 2176000u32);
    emu.sb_no_count(6usize, 2usize, 18u32, 2176004u32);
    emu.sb_no_count(16usize, 2usize, 19u32, 2176008u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2176012u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2176016u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2176020u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2176024u32);
    emu.anr_no_count(13usize, 16usize, 13usize, 2176028u32);
    emu.adi_no_count(16usize, 2usize, 12u32, 2176032u32);
    emu.adi_no_count(12usize, 12usize, 819u32, 2176036u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2176040u32);
    emu.sbr_no_count(14usize, 14usize, 13usize, 2176044u32);
    emu.anr_no_count(13usize, 14usize, 12usize, 2176048u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2176052u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2176056u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2176060u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2176064u32);
    emu.sri_no_count(13usize, 12usize, 4u32, 2176068u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2176072u32);
    emu.adi_no_count(13usize, 0usize, 117u32, 2176076u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2176080u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2176084u32);
    emu.adi_no_count(12usize, 0usize, 123u32, 2176088u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2176092u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2176096u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2176100u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2176104u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2176108u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2176112u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2176116u32);
    emu.sb_no_count(13usize, 10usize, 4294967295u32, 2176120u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2176124u32);
    emu.sb_no_count(11usize, 2usize, 20u32, 2176128u32);
    emu.sb_no_count(15usize, 2usize, 21u32, 2176132u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2176136u32);
    emu.add_memory_rw_events(78usize);
    let return_addr = 2176140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176564u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213634));
}
#[inline(always)]
pub fn block_0x0021348c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2176144u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176148u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966876u32, 2176152u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2176156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213658));
}
#[inline(always)]
pub fn block_0x0021349c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 256u32, 2176160u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2176220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134dc));
    } else {
        emu.pc = 2176164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134a4));
    }
}
#[inline(always)]
pub fn block_0x002134a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2176168u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176172u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1884u32, 2176176u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2176180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213658));
}
#[inline(always)]
pub fn block_0x002134b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2176184u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176188u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let return_addr = 2176192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213654));
}
#[inline(always)]
pub fn block_0x002134c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2176196u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176200u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1116u32, 2176204u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2176208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213658));
}
#[inline(always)]
pub fn block_0x002134d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 8u32, 2176212u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2176216u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2176588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021364c));
    } else {
        emu.pc = 2176220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134dc));
    }
}
#[inline(always)]
pub fn block_0x002134dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2176224u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2176228u32);
    emu.apc_no_count(1usize, 2176228u32, 8192u32, 2176232u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176236u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002134ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213500));
    } else {
        emu.pc = 2176240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002134f0));
    }
}
#[inline(always)]
pub fn block_0x002134f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2176244u32)?;
    emu.adi_no_count(18usize, 0usize, 129u32, 2176248u32);
    emu.adi_no_count(9usize, 0usize, 128u32, 2176252u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2176256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021366c));
}
#[inline(never)]
pub fn block_0x00213500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 77u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 9usize, 1u32, 2176260u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176264u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2176268u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176272u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 9usize, 20u32, 2176276u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2176280u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 644u32, 2176284u32);
    emu.sli_no_count(17usize, 9usize, 12u32, 2176288u32);
    emu.sli_no_count(5usize, 9usize, 16u32, 2176292u32);
    emu.sli_no_count(6usize, 9usize, 20u32, 2176296u32);
    emu.sli_no_count(7usize, 9usize, 24u32, 2176300u32);
    emu.orr_no_count(14usize, 9usize, 14usize, 2176304u32);
    emu.ani_no_count(13usize, 9usize, 15u32, 2176308u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2176312u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2176316u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2176320u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2176324u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2176328u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2176332u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2176336u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2176340u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2176344u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2176348u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2176352u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2176356u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2176360u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2176364u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2176368u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2176372u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2176376u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2176380u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2176384u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2176388u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2176392u32);
    emu.sh_no_count(0usize, 2usize, 22u32, 2176396u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2176400u32);
    emu.sb_no_count(15usize, 2usize, 25u32, 2176404u32);
    emu.sb_no_count(17usize, 2usize, 26u32, 2176408u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2176412u32);
    emu.adi_no_count(12usize, 12usize, 1365u32, 2176416u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2176420u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2176424u32);
    emu.sb_no_count(5usize, 2usize, 27u32, 2176428u32);
    emu.sb_no_count(6usize, 2usize, 28u32, 2176432u32);
    emu.sb_no_count(16usize, 2usize, 29u32, 2176436u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2176440u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2176444u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2176448u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2176452u32);
    emu.anr_no_count(12usize, 16usize, 12usize, 2176456u32);
    emu.adi_no_count(16usize, 2usize, 22u32, 2176460u32);
    emu.adi_no_count(11usize, 11usize, 819u32, 2176464u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2176468u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2176472u32);
    emu.anr_no_count(12usize, 14usize, 11usize, 2176476u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2176480u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2176484u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2176488u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2176492u32);
    emu.sri_no_count(12usize, 11usize, 4u32, 2176496u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2176500u32);
    emu.adi_no_count(12usize, 0usize, 117u32, 2176504u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2176508u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2176512u32);
    emu.adi_no_count(11usize, 0usize, 123u32, 2176516u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2176520u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2176524u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2176528u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2176532u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2176536u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2176540u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2176544u32);
    emu.sb_no_count(12usize, 10usize, 4294967295u32, 2176548u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2176552u32);
    emu.sb_no_count(13usize, 2usize, 30u32, 2176556u32);
    emu.sb_no_count(15usize, 2usize, 31u32, 2176560u32);
    emu.adi_no_count(11usize, 2usize, 22u32, 2176564u32);
    emu.add_memory_rw_events(77usize);
    emu.pc = 2176564u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213634));
}
#[inline(always)]
pub fn block_0x00213634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 10u32, 2176568u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2176572u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2176576u32);
    emu.apc_no_count(1usize, 2176576u32, 4294909952u32, 2176580u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176584u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(92u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2176588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021366c));
}
#[inline(always)]
pub fn block_0x0021364c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2176592u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176596u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2176596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213654));
}
#[inline(always)]
pub fn block_0x00213654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 604u32, 2176600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2176600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213658));
}
#[inline(always)]
pub fn block_0x00213658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 0u32, 2176604u32)?;
    emu.sh_no_count(0usize, 8usize, 4u32, 2176608u32)?;
    emu.sh_no_count(0usize, 8usize, 6u32, 2176612u32)?;
    emu.sh_no_count(0usize, 8usize, 8u32, 2176616u32)?;
    emu.adi_no_count(18usize, 0usize, 2u32, 2176620u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2176620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021366c));
}
#[inline(always)]
pub fn block_0x0021366c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 12u32, 2176624u32);
    emu.sb_no_count(18usize, 8usize, 13u32, 2176628u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2176632u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2176636u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2176640u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2176644u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2176648u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176652u32;
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
pub fn block_0x0021368c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2176656u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2176660u32)?;
    emu.adi_no_count(13usize, 10usize, 0u32, 2176664u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2176668u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2176672u32);
    emu.apc_no_count(6usize, 2176672u32, 0u32, 2176676u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2176680u32;
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
pub fn block_0x002136a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2176684u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2176688u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2176692u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2176696u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2176700u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2176704u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2176708u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2176712u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2176716u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2176720u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2176724u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2176728u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2176732u32);
    let a = 0u32.wrapping_add(3758096384u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176736u32;
    emu.update_insn_clock();
    emu.lw_no_count(21usize, 8usize, 16u32, 2176740u32)?;
    emu.adi_no_count(12usize, 12usize, 32u32, 2176744u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2176748u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2176752u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2176756u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2176760u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2177056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213820));
    } else {
        emu.pc = 2176764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002136fc));
    }
}
#[inline(always)]
pub fn block_0x002136fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 20u32, 2176768u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2177208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138b8));
    } else {
        emu.pc = 2176772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213704));
    }
}
#[inline]
pub fn block_0x00213704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2176776u32);
    emu.sli_no_count(12usize, 11usize, 3u32, 2176780u32);
    emu.sli_no_count(13usize, 11usize, 5u32, 2176784u32);
    emu.adi_no_count(10usize, 21usize, 24u32, 2176788u32);
    emu.lw_no_count(23usize, 8usize, 0u32, 2176792u32)?;
    emu.lw_no_count(19usize, 8usize, 8u32, 2176796u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2176800u32);
    emu.adi_no_count(20usize, 0usize, 2u32, 2176804u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2176808u32);
    emu.sli_no_count(11usize, 11usize, 3u32, 2176812u32);
    emu.adr_no_count(22usize, 21usize, 13usize, 2176816u32);
    emu.sri_no_count(11usize, 11usize, 3u32, 2176820u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2176824u32);
    emu.adi_no_count(23usize, 23usize, 4u32, 2176828u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2176832u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2176832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213740));
}
#[inline(always)]
pub fn block_0x00213740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 0u32, 2176836u32)?;
    emu.adi_no_count(25usize, 10usize, 0u32, 2176840u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2176868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213764));
    } else {
        emu.pc = 2176844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021374c));
    }
}
#[inline(always)]
pub fn block_0x0021374c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2176848u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2176852u32)?;
    emu.lw_no_count(11usize, 23usize, 4294967292u32, 2176856u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2176860u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2176864u32;
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
pub fn block_0x00213760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138ec));
    } else {
        emu.pc = 2176868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213764));
    }
}
#[inline(always)]
pub fn block_0x00213764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 21usize, 8u32, 2176872u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2176928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137a0));
    } else {
        emu.pc = 2176876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021376c));
    }
}
#[inline(always)]
pub fn block_0x0021376c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137b4));
    } else {
        emu.pc = 2176880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213770));
    }
}
#[inline(always)]
pub fn block_0x00213770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 12u32, 2176884u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2176888u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2176892u32);
    emu.lhu_no_count(11usize, 10usize, 4u32, 2176896u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2176900u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2176940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137ac));
    } else {
        emu.pc = 2176904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213788));
    }
}
#[inline(always)]
pub fn block_0x00213788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2176964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137c4));
    } else {
        emu.pc = 2176908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021378c));
    }
}
#[inline(always)]
pub fn block_0x0021378c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2176912u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2176916u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2176920u32);
    emu.lhu_no_count(12usize, 10usize, 4u32, 2176924u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2176928u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176968u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002137c8));
}
#[inline(always)]
pub fn block_0x002137a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 21usize, 10u32, 2176932u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2176936u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2176904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213788));
    } else {
        emu.pc = 2176940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137ac));
    }
}
#[inline(always)]
pub fn block_0x002137ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2176944u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2176948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176968u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002137c8));
}
#[inline(always)]
pub fn block_0x002137b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2176952u32);
    emu.lhu_no_count(10usize, 21usize, 0u32, 2176956u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2176904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213788));
    } else {
        emu.pc = 2176960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137c0));
    }
}
#[inline(always)]
pub fn block_0x002137c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2176964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176940u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002137ac));
}
#[inline(always)]
pub fn block_0x002137c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 21usize, 2u32, 2176968u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2176968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002137c8));
}
#[inline]
pub fn block_0x002137c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 16u32, 2176972u32)?;
    emu.lw_no_count(13usize, 21usize, 20u32, 2176976u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2176980u32);
    emu.adr_no_count(14usize, 19usize, 10usize, 2176984u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2176988u32)?;
    emu.lw_no_count(14usize, 14usize, 4u32, 2176992u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2176996u32)?;
    emu.sh_no_count(11usize, 2usize, 16u32, 2177000u32)?;
    emu.sh_no_count(12usize, 2usize, 18u32, 2177004u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2177008u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2177012u32;
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
pub fn block_0x002137f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138ec));
    } else {
        emu.pc = 2177016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002137f8));
    }
}
#[inline]
pub fn block_0x002137f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2177020u32);
    emu.xrr_no_count(10usize, 25usize, 22usize, 2177024u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2177028u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2177032u32);
    emu.ani_no_count(10usize, 10usize, 24u32, 2177036u32);
    emu.adr_no_count(10usize, 25usize, 10usize, 2177040u32);
    emu.adi_no_count(23usize, 23usize, 8u32, 2177044u32);
    emu.adi_no_count(21usize, 25usize, 0u32, 2177048u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2176832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213740));
    } else {
        emu.pc = 2177052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021381c));
    }
}
#[inline(always)]
pub fn block_0x0021381c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002138ac));
}
#[inline(always)]
pub fn block_0x00213820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2177060u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2177208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138b8));
    } else {
        emu.pc = 2177064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213828));
    }
}
#[inline]
pub fn block_0x00213828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2177068u32);
    emu.lw_no_count(20usize, 8usize, 0u32, 2177072u32)?;
    emu.lw_no_count(21usize, 8usize, 8u32, 2177076u32)?;
    emu.sli_no_count(19usize, 10usize, 3u32, 2177080u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2177084u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2177088u32);
    emu.sri_no_count(10usize, 10usize, 3u32, 2177092u32);
    emu.adi_no_count(9usize, 10usize, 1u32, 2177096u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2177100u32);
    emu.adi_no_count(10usize, 21usize, 8u32, 2177104u32);
    emu.adi_no_count(20usize, 20usize, 4u32, 2177108u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2177108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213854));
}
#[inline(always)]
pub fn block_0x00213854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2177112u32)?;
    emu.adi_no_count(22usize, 10usize, 0u32, 2177116u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2177144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213878));
    } else {
        emu.pc = 2177120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213860));
    }
}
#[inline(always)]
pub fn block_0x00213860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2177124u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2177128u32)?;
    emu.lw_no_count(11usize, 20usize, 4294967292u32, 2177132u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2177136u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2177140u32;
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
pub fn block_0x00213874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138ec));
    } else {
        emu.pc = 2177144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213878));
    }
}
#[inline(always)]
pub fn block_0x00213878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 0u32, 2177148u32)?;
    emu.lw_no_count(12usize, 21usize, 4u32, 2177152u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2177156u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2177160u32;
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
pub fn block_0x00213888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138ec));
    } else {
        emu.pc = 2177164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021388c));
    }
}
#[inline(always)]
pub fn block_0x0021388c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2177168u32);
    emu.xrr_no_count(10usize, 22usize, 19usize, 2177172u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2177176u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2177180u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2177184u32);
    emu.adi_no_count(20usize, 20usize, 8u32, 2177188u32);
    emu.adi_no_count(21usize, 22usize, 0u32, 2177192u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2177108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213854));
    } else {
        emu.pc = 2177196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138ac));
    }
}
#[inline(always)]
pub fn block_0x002138ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2177200u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2177220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138c4));
    } else {
        emu.pc = 2177204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138b4));
    }
}
#[inline(always)]
pub fn block_0x002138b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002138f4));
}
#[inline(always)]
pub fn block_0x002138b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2177212u32);
    emu.lw_no_count(10usize, 8usize, 4u32, 2177216u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(0usize);
    if a >= b {
        emu.pc = 2177268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138f4));
    } else {
        emu.pc = 2177220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138c4));
    }
}
#[inline]
pub fn block_0x002138c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 0u32, 2177224u32)?;
    emu.sli_no_count(9usize, 9usize, 3u32, 2177228u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2177232u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2177236u32)?;
    emu.adr_no_count(9usize, 11usize, 9usize, 2177240u32);
    emu.lw_no_count(11usize, 9usize, 0u32, 2177244u32)?;
    emu.lw_no_count(12usize, 9usize, 4u32, 2177248u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2177252u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2177256u32;
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
pub fn block_0x002138e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138f4));
    } else {
        emu.pc = 2177260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002138ec));
    }
}
#[inline(always)]
pub fn block_0x002138ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2177264u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2177268u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002138f8));
}
#[inline(always)]
pub fn block_0x002138f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2177272u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002138f8));
}
#[inline]
pub fn block_0x002138f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2177276u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2177280u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2177284u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2177288u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2177292u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2177296u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2177300u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2177304u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2177308u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2177312u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2177316u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2177320u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177324u32;
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
pub fn block_0x0021392c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2177328u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2177332u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2177336u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2177340u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2177344u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2177348u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2177352u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2177356u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2177360u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2177364u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2177368u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2177372u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2177376u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2177380u32)?;
    emu.adi_no_count(8usize, 15usize, 0u32, 2177384u32);
    emu.adi_no_count(9usize, 14usize, 0u32, 2177388u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2177392u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2177396u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2177400u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2177552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a10));
    } else {
        emu.pc = 2177404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021397c));
    }
}
#[inline(always)]
pub fn block_0x0021397c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2177408u32)?;
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177412u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 22usize, 10usize, 2177416u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2177420u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2177428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213994));
    } else {
        emu.pc = 2177424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213990));
    }
}
#[inline(always)]
pub fn block_0x00213990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 43u32, 2177428u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213994));
}
#[inline(always)]
pub fn block_0x00213994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 21u32, 2177432u32);
    emu.adr_no_count(24usize, 10usize, 8usize, 2177436u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2177440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2177572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a24));
    } else {
        emu.pc = 2177444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139a4));
    }
}
#[inline(always)]
pub fn block_0x002139a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 16u32, 2177448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2177700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213aa4));
    } else {
        emu.pc = 2177452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139ac));
    }
}
#[inline(always)]
pub fn block_0x002139ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2177456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2177492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139d4));
    } else {
        emu.pc = 2177460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139b4));
    }
}
#[inline(always)]
pub fn block_0x002139b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 20usize, 19usize, 2177464u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2177468u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2177468u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002139bc));
}
#[inline(always)]
pub fn block_0x002139bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2177472u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2177476u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2177480u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2177484u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2177488u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2177468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139bc));
    } else {
        emu.pc = 2177492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139d4));
    }
}
#[inline(always)]
pub fn block_0x002139d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2177496u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2177500u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2177584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a30));
    } else {
        emu.pc = 2177504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139e0));
    }
}
#[inline(always)]
pub fn block_0x002139e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 22usize, 7u32, 2177508u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2177732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ac4));
    } else {
        emu.pc = 2177512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139e8));
    }
}
#[inline(always)]
pub fn block_0x002139e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(24usize, 27usize, 24usize, 2177516u32);
    emu.sli_no_count(10usize, 22usize, 1u32, 2177520u32);
    emu.sri_no_count(10usize, 10usize, 30u32, 2177524u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2177528u32);
    emu.sli_no_count(22usize, 22usize, 11u32, 2177532u32);
    emu.sw_no_count(9usize, 2usize, 8u32, 2177536u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2177864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b48));
    } else {
        emu.pc = 2177540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a04));
    }
}
#[inline(always)]
pub fn block_0x00213a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b88));
    } else {
        emu.pc = 2177544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a08));
    }
}
#[inline(always)]
pub fn block_0x00213a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2177548u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2177552u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177932u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213b8c));
}
#[inline(always)]
pub fn block_0x00213a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2177556u32)?;
    emu.adi_no_count(24usize, 8usize, 1u32, 2177560u32);
    emu.adi_no_count(21usize, 0usize, 45u32, 2177564u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2177568u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2177444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139a4));
    } else {
        emu.pc = 2177572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a24));
    }
}
#[inline(always)]
pub fn block_0x00213a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2177576u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2177580u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a < b {
        emu.pc = 2177504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002139e0));
    } else {
        emu.pc = 2177584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a30));
    }
}
#[inline]
pub fn block_0x00213a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2177588u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2177592u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2177596u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2177600u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2177604u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2177608u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2177612u32);
    emu.apc_no_count(1usize, 2177612u32, 0u32, 2177616u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177620u32;
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
pub fn block_0x00213a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bc8));
    } else {
        emu.pc = 2177624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a58));
    }
}
#[inline]
pub fn block_0x00213a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2177628u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2177632u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2177636u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2177640u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2177644u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2177648u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2177652u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2177656u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2177660u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2177664u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2177668u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2177672u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2177676u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2177680u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2177684u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2177688u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2177692u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2177696u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2177700u32;
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
pub fn block_0x00213aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2177704u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2177708u32);
    emu.apc_no_count(1usize, 2177708u32, 16384u32, 2177712u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177716u32;
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
pub fn block_0x00213ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2177720u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2177724u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2177584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213a30));
    } else {
        emu.pc = 2177728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ac0));
    }
}
#[inline(always)]
pub fn block_0x00213ac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002139e0));
}
#[inline]
pub fn block_0x00213ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2177736u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2177740u32)?;
    emu.lw_no_count(25usize, 18usize, 8u32, 2177744u32)?;
    emu.lw_no_count(26usize, 18usize, 12u32, 2177748u32)?;
    let a = 0u32.wrapping_add(2682257408u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2177752u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2177756u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 25usize, 10usize, 2177760u32);
    emu.adi_no_count(11usize, 11usize, 48u32, 2177764u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2177768u32);
    emu.sw_no_count(10usize, 18usize, 8u32, 2177772u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2177776u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2177780u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2177784u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2177788u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2177792u32);
    emu.apc_no_count(1usize, 2177792u32, 0u32, 2177796u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2177800u32;
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
pub fn block_0x00213b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2177804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2177996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bcc));
    } else {
        emu.pc = 2177808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b10));
    }
}
#[inline(always)]
pub fn block_0x00213b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2177812u32);
    emu.sbr_no_count(10usize, 27usize, 24usize, 2177816u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2177820u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2177824u32);
    emu.anr_no_count(24usize, 10usize, 21usize, 2177828u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2177828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213b24));
}
#[inline(always)]
pub fn block_0x00213b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 20usize, 21usize, 2177832u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2177888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b60));
    } else {
        emu.pc = 2177836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b2c));
    }
}
#[inline(always)]
pub fn block_0x00213b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 16u32, 2177840u32)?;
    emu.adi_no_count(20usize, 20usize, 1u32, 2177844u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2177848u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2177852u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2177856u32;
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
pub fn block_0x00213b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b24));
    } else {
        emu.pc = 2177860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b44));
    }
}
#[inline(always)]
pub fn block_0x00213b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2177864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213bcc));
}
#[inline(always)]
pub fn block_0x00213b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2177868u32);
    emu.adi_no_count(25usize, 24usize, 0u32, 2177872u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2177932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b8c));
    } else {
        emu.pc = 2177876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b54));
    }
}
#[inline(always)]
pub fn block_0x00213b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 24usize, 16u32, 2177880u32);
    emu.sri_no_count(25usize, 10usize, 17u32, 2177884u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2177888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177932u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213b8c));
}
#[inline(always)]
pub fn block_0x00213b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 23usize, 12u32, 2177892u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2177896u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2177900u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2177904u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2177908u32;
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
pub fn block_0x00213b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bcc));
    } else {
        emu.pc = 2177912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213b78));
    }
}
#[inline(always)]
pub fn block_0x00213b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2177916u32);
    emu.sw_no_count(25usize, 18usize, 8u32, 2177920u32)?;
    emu.sw_no_count(26usize, 18usize, 12u32, 2177924u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2177928u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213bcc));
}
#[inline(always)]
pub fn block_0x00213b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2177932u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177932u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213b8c));
}
#[inline(always)]
pub fn block_0x00213b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 0u32, 2177936u32);
    emu.sri_no_count(22usize, 22usize, 11u32, 2177940u32);
    emu.lw_no_count(23usize, 18usize, 0u32, 2177944u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2177948u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2177952u32;
    emu.update_insn_clock();
    emu.adi_no_count(27usize, 27usize, 4294967295u32, 2177956u32);
    emu.anr_no_count(9usize, 25usize, 27usize, 2177960u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2177960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213ba8));
}
#[inline(always)]
pub fn block_0x00213ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 26usize, 27usize, 2177964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2178060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c0c));
    } else {
        emu.pc = 2177968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bb0));
    }
}
#[inline(always)]
pub fn block_0x00213bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2177972u32)?;
    emu.adi_no_count(26usize, 26usize, 1u32, 2177976u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2177980u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2177984u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2177988u32;
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
pub fn block_0x00213bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213ba8));
    } else {
        emu.pc = 2177992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bc8));
    }
}
#[inline(always)]
pub fn block_0x00213bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2177996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2177996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213bcc));
}
#[inline]
pub fn block_0x00213bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2178000u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2178004u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2178008u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2178012u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2178016u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2178020u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2178024u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2178028u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2178032u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2178036u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2178040u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2178044u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2178048u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2178052u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2178056u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178060u32;
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
pub fn block_0x00213c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 23usize, 0u32, 2178064u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2178068u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2178072u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2178076u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2178080u32);
    emu.apc_no_count(1usize, 2178080u32, 0u32, 2178084u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2178088u32;
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
pub fn block_0x00213c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2178092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2177996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bcc));
    } else {
        emu.pc = 2178096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c30));
    }
}
#[inline(always)]
pub fn block_0x00213c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 12u32, 2178100u32)?;
    emu.adi_no_count(10usize, 23usize, 0u32, 2178104u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2178108u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2178112u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2178116u32;
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
pub fn block_0x00213c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2177996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bcc));
    } else {
        emu.pc = 2178120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c48));
    }
}
#[inline(always)]
pub fn block_0x00213c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2178124u32);
    emu.sbr_no_count(10usize, 24usize, 25usize, 2178128u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2178132u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2178136u32);
    emu.anr_no_count(20usize, 10usize, 9usize, 2178140u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2178140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213c5c));
}
#[inline(always)]
pub fn block_0x00213c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 8usize, 9usize, 2178144u32);
    emu.sltru_no_count(19usize, 10usize, 20usize, 2178148u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2177996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213bcc));
    } else {
        emu.pc = 2178152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c68));
    }
}
#[inline(always)]
pub fn block_0x00213c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2178156u32)?;
    emu.adi_no_count(8usize, 8usize, 1u32, 2178160u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2178164u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2178168u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2178172u32;
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
pub fn block_0x00213c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2178140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c5c));
    } else {
        emu.pc = 2178176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213c80));
    }
}
#[inline(always)]
pub fn block_0x00213c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2178180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2177996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213bcc));
}
