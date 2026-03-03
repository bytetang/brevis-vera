pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2162020u32;
pub const PC_MAX: u32 = 2164328u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 133usize] = [
        block_0x0020fd64,
        block_0x0020fd6c,
        block_0x0020fd80,
        block_0x0020fd94,
        block_0x0020fd9c,
        block_0x0020fda8,
        block_0x0020fdb0,
        block_0x0020fdc0,
        block_0x0020fdc8,
        block_0x0020fdd0,
        block_0x0020fdd8,
        block_0x0020fde4,
        block_0x0020fde8,
        block_0x0020fdec,
        block_0x0020fdf0,
        block_0x0020fdf4,
        block_0x0020fe04,
        block_0x0020fe08,
        block_0x0020fe10,
        block_0x0020fe14,
        block_0x0020fe18,
        block_0x0020fe24,
        block_0x0020fe2c,
        block_0x0020fe30,
        block_0x0020fe3c,
        block_0x0020fe4c,
        block_0x0020fe58,
        block_0x0020fe6c,
        block_0x0020fe84,
        block_0x0020fea8,
        block_0x0020feac,
        block_0x0020feb4,
        block_0x0020fec0,
        block_0x0020fec8,
        block_0x0020fedc,
        block_0x0020fee4,
        block_0x0020feec,
        block_0x0020fef8,
        block_0x0020ff94,
        block_0x0020ff98,
        block_0x00210004,
        block_0x00210078,
        block_0x002100a0,
        block_0x002100ac,
        block_0x002100b4,
        block_0x002100dc,
        block_0x002100e0,
        block_0x002100fc,
        block_0x0021010c,
        block_0x00210120,
        block_0x00210124,
        block_0x00210130,
        block_0x0021013c,
        block_0x0021015c,
        block_0x00210164,
        block_0x00210170,
        block_0x00210178,
        block_0x00210184,
        block_0x002101ac,
        block_0x002101c0,
        block_0x002101c4,
        block_0x002101e8,
        block_0x00210204,
        block_0x0021021c,
        block_0x00210234,
        block_0x0021024c,
        block_0x00210278,
        block_0x002102b0,
        block_0x002102bc,
        block_0x002102c0,
        block_0x002102e0,
        block_0x002102f4,
        block_0x002102f8,
        block_0x0021034c,
        block_0x00210354,
        block_0x0021035c,
        block_0x0021036c,
        block_0x00210374,
        block_0x0021037c,
        block_0x00210384,
        block_0x0021038c,
        block_0x00210390,
        block_0x002103a4,
        block_0x002103ac,
        block_0x002103b0,
        block_0x002103b8,
        block_0x002103dc,
        block_0x002103f4,
        block_0x0021048c,
        block_0x002104a0,
        block_0x002104bc,
        block_0x002104c4,
        block_0x002104cc,
        block_0x002104d0,
        block_0x002104d8,
        block_0x002104e0,
        block_0x002104f0,
        block_0x002104f4,
        block_0x00210500,
        block_0x00210508,
        block_0x00210514,
        block_0x00210518,
        block_0x00210524,
        block_0x0021052c,
        block_0x00210534,
        block_0x00210568,
        block_0x00210570,
        block_0x00210574,
        block_0x0021057c,
        block_0x00210588,
        block_0x00210590,
        block_0x00210598,
        block_0x0021059c,
        block_0x002105ac,
        block_0x002105b4,
        block_0x002105c0,
        block_0x002105c4,
        block_0x002105c8,
        block_0x002105d4,
        block_0x002105d8,
        block_0x002105e8,
        block_0x002105f8,
        block_0x0021060c,
        block_0x00210610,
        block_0x00210614,
        block_0x00210618,
        block_0x00210630,
        block_0x0021064c,
        block_0x00210650,
        block_0x00210654,
        block_0x0021065c,
        block_0x00210664,
        block_0x00210668,
    ];
    const IDX: [u16; 578usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 4u16,
        0u16, 5u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16,
        0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 12u16, 13u16, 14u16, 15u16, 16u16, 0u16,
        0u16, 0u16, 17u16, 18u16, 0u16, 19u16, 20u16, 21u16, 0u16, 0u16, 22u16, 0u16,
        23u16, 24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 27u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16, 32u16, 0u16, 0u16, 33u16,
        0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 43u16, 0u16, 0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16,
        0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 52u16,
        0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 55u16,
        0u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16,
        0u16, 78u16, 0u16, 79u16, 0u16, 80u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 84u16, 85u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 94u16, 0u16,
        95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 99u16, 0u16,
        100u16, 0u16, 0u16, 101u16, 102u16, 0u16, 0u16, 103u16, 0u16, 104u16, 0u16,
        105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        106u16, 0u16, 107u16, 108u16, 0u16, 109u16, 0u16, 0u16, 110u16, 0u16, 111u16,
        0u16, 112u16, 113u16, 0u16, 0u16, 0u16, 114u16, 0u16, 115u16, 0u16, 0u16, 116u16,
        117u16, 118u16, 0u16, 0u16, 119u16, 120u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16,
        0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 123u16, 124u16, 125u16, 126u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 127u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 128u16, 129u16,
        130u16, 0u16, 131u16, 0u16, 132u16, 133u16,
    ];
    if pc < 2162020u32 || pc > 2164328u32 {
        return None;
    }
    let word_offset = ((pc - 2162020u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020fd64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2162020u32, 0u32, 2162024u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162028u32;
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
pub fn block_0x0020fd6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2162032u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2162036u32)?;
    emu.adi_no_count(15usize, 0usize, 257u32, 2162040u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2162044u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2162068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd94));
    } else {
        emu.pc = 2162048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd80));
    }
}
#[inline(always)]
pub fn block_0x0020fd80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2162052u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2162056u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2162060u32)?;
    emu.adi_no_count(15usize, 0usize, 1u32, 2162064u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2162068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fdd8));
}
#[inline(always)]
pub fn block_0x0020fd94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 256u32, 2162072u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2162076u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2162076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fd9c));
}
#[inline(always)]
pub fn block_0x0020fd9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 10usize, 15usize, 2162080u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2162084u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2162096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdb0));
    } else {
        emu.pc = 2162088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fda8));
    }
}
#[inline(always)]
pub fn block_0x0020fda8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2162092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2162076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd9c));
    } else {
        emu.pc = 2162096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdb0));
    }
}
#[inline(always)]
pub fn block_0x0020fdb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 16u32, 2162100u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2162104u32)?;
    emu.sltru_no_count(16usize, 15usize, 11usize, 2162108u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2162120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdc8));
    } else {
        emu.pc = 2162112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdc0));
    }
}
#[inline(always)]
pub fn block_0x0020fdc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2162116u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2162120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fdd0));
}
#[inline(always)]
pub fn block_0x0020fdc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2162124u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1280u32, 2162128u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2162128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fdd0));
}
#[inline(always)]
pub fn block_0x0020fdd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 0usize, 16usize, 2162132u32);
    emu.ani_no_count(16usize, 16usize, 5u32, 2162136u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2162136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fdd8));
}
#[inline(always)]
pub fn block_0x0020fdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 24u32, 2162140u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2162144u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2162584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff98));
    } else {
        emu.pc = 2162148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fde4));
    }
}
#[inline(always)]
pub fn block_0x0020fde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2162580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff94));
    } else {
        emu.pc = 2162152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fde8));
    }
}
#[inline(always)]
pub fn block_0x0020fde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2162692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210004));
    } else {
        emu.pc = 2162156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdec));
    }
}
#[inline(always)]
pub fn block_0x0020fdec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe08));
    } else {
        emu.pc = 2162160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdf0));
    }
}
#[inline(always)]
pub fn block_0x0020fdf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2162184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe08));
    } else {
        emu.pc = 2162164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fdf4));
    }
}
#[inline(always)]
pub fn block_0x0020fdf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2162168u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2162172u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2162176u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2162184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe08));
    } else {
        emu.pc = 2162180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe04));
    }
}
#[inline(always)]
pub fn block_0x0020fe04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 12usize, 0u32, 2162184u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fe08));
}
#[inline(always)]
pub fn block_0x0020fe08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 32u32, 2162188u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2162224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe30));
    } else {
        emu.pc = 2162192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe10));
    }
}
#[inline(always)]
pub fn block_0x0020fe10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2162220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe2c));
    } else {
        emu.pc = 2162196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe14));
    }
}
#[inline(always)]
pub fn block_0x0020fe14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2162200u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fe18));
}
#[inline(always)]
pub fn block_0x0020fe18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 13usize, 2162204u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2162208u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2162220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe2c));
    } else {
        emu.pc = 2162212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe24));
    }
}
#[inline(always)]
pub fn block_0x0020fe24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2162216u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2162200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe18));
    } else {
        emu.pc = 2162220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe2c));
    }
}
#[inline(always)]
pub fn block_0x0020fe2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2162236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe3c));
    } else {
        emu.pc = 2162224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe30));
    }
}
#[inline(always)]
pub fn block_0x0020fe30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2162228u32);
    emu.apc_no_count(1usize, 2162228u32, 8192u32, 2162232u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162236u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fe3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2162240u32);
    emu.lb_no_count(12usize, 10usize, 0u32, 2162244u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2162248u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2162264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe58));
    } else {
        emu.pc = 2162252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe4c));
    }
}
#[inline(always)]
pub fn block_0x0020fe4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 36u32, 2162256u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2162260u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2162264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fef8));
}
#[inline(always)]
pub fn block_0x0020fe58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(15usize, 10usize, 1u32, 2162268u32);
    emu.ani_no_count(12usize, 11usize, 31u32, 2162272u32);
    emu.adi_no_count(16usize, 0usize, 223u32, 2162276u32);
    emu.ani_no_count(15usize, 15usize, 63u32, 2162280u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2162348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020feac));
    } else {
        emu.pc = 2162284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe6c));
    }
}
#[inline(always)]
pub fn block_0x0020fe6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 2u32, 2162288u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2162292u32);
    emu.ani_no_count(16usize, 16usize, 63u32, 2162296u32);
    emu.adi_no_count(17usize, 0usize, 240u32, 2162300u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2162304u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2162376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fec8));
    } else {
        emu.pc = 2162308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe84));
    }
}
#[inline]
pub fn block_0x0020fe84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 3u32, 2162312u32);
    emu.sli_no_count(12usize, 12usize, 29u32, 2162316u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2162320u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2162324u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2162328u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2162332u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2162336u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2162340u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fe30));
    } else {
        emu.pc = 2162344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fea8));
    }
}
#[inline(always)]
pub fn block_0x0020fea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2162348u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162356u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020feb4));
}
#[inline(always)]
pub fn block_0x0020feac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 6u32, 2162352u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2162356u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2162356u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020feb4));
}
#[inline(always)]
pub fn block_0x0020feb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 128u32, 2162360u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2162364u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2162396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fedc));
    } else {
        emu.pc = 2162368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fec0));
    }
}
#[inline(always)]
pub fn block_0x0020fec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2162372u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2162376u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fef8));
}
#[inline(always)]
pub fn block_0x0020fec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 12u32, 2162380u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2162384u32);
    emu.adi_no_count(11usize, 0usize, 128u32, 2162388u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2162392u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2162368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fec0));
    } else {
        emu.pc = 2162396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fedc));
    }
}
#[inline(always)]
pub fn block_0x0020fedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 11u32, 2162400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2162412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020feec));
    } else {
        emu.pc = 2162404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fee4));
    }
}
#[inline(always)]
pub fn block_0x0020fee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2162408u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2162412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fef8));
}
#[inline(always)]
pub fn block_0x0020feec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 16u32, 2162416u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2162420u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2162424u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2162424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fef8));
}
#[inline(never)]
pub fn block_0x0020fef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2162428u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2162432u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2162436u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 348u32, 2162440u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2162444u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2162448u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966580u32, 2162452u32);
    emu.adi_no_count(17usize, 2usize, 40u32, 2162456u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2162460u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966576u32, 2162464u32);
    emu.adi_no_count(6usize, 2usize, 16u32, 2162468u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2162472u32;
    emu.update_insn_clock();
    emu.adi_no_count(7usize, 7usize, 144u32, 2162476u32);
    emu.adi_no_count(28usize, 2usize, 24u32, 2162480u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2162484u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2162488u32)?;
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162492u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1388u32, 2162496u32);
    emu.sw_no_count(11usize, 2usize, 72u32, 2162500u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2162504u32)?;
    emu.sw_no_count(15usize, 2usize, 80u32, 2162508u32)?;
    emu.sw_no_count(16usize, 2usize, 84u32, 2162512u32)?;
    emu.adi_no_count(11usize, 0usize, 5u32, 2162516u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2162520u32)?;
    emu.sw_no_count(17usize, 2usize, 88u32, 2162524u32)?;
    emu.sw_no_count(5usize, 2usize, 92u32, 2162528u32)?;
    emu.sw_no_count(6usize, 2usize, 96u32, 2162532u32)?;
    emu.sw_no_count(7usize, 2usize, 100u32, 2162536u32)?;
    emu.sw_no_count(28usize, 2usize, 104u32, 2162540u32)?;
    emu.sw_no_count(7usize, 2usize, 108u32, 2162544u32)?;
    emu.adi_no_count(12usize, 2usize, 72u32, 2162548u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2162552u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2162556u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2162560u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2162564u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2162568u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2162572u32);
    emu.apc_no_count(1usize, 2162572u32, 0u32, 2162576u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ff94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 0u32, 2162584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ff98));
}
#[inline(never)]
pub fn block_0x0020ff98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 40u32, 2162588u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2162592u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2162596u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2162600u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2162604u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2162608u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 144u32, 2162612u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2162616u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2162620u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1452u32, 2162624u32);
    emu.adi_no_count(17usize, 0usize, 3u32, 2162628u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2162632u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2162636u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2162640u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2162644u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2162648u32)?;
    emu.sw_no_count(15usize, 2usize, 88u32, 2162652u32)?;
    emu.sw_no_count(13usize, 2usize, 92u32, 2162656u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2162660u32);
    emu.sw_no_count(16usize, 2usize, 48u32, 2162664u32)?;
    emu.sw_no_count(17usize, 2usize, 52u32, 2162668u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2162672u32)?;
    emu.sw_no_count(17usize, 2usize, 60u32, 2162676u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2162680u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2162684u32);
    emu.apc_no_count(1usize, 2162684u32, 0u32, 2162688u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162692u32;
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
#[inline(never)]
pub fn block_0x00210004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2162696u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2162700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2162704u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2162708u32);
    emu.adi_no_count(13usize, 2usize, 16u32, 2162712u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2162716u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 144u32, 2162720u32);
    emu.adi_no_count(16usize, 2usize, 24u32, 2162724u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2162728u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1300u32, 2162732u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2162736u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2162740u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2162744u32)?;
    emu.sw_no_count(11usize, 2usize, 84u32, 2162748u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2162752u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2162756u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2162760u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2162764u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2162768u32)?;
    emu.sw_no_count(15usize, 2usize, 100u32, 2162772u32)?;
    emu.adi_no_count(11usize, 2usize, 72u32, 2162776u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2162780u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2162784u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2162788u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2162792u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2162796u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2162800u32);
    emu.apc_no_count(1usize, 2162800u32, 0u32, 2162804u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2162812u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2162816u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2162820u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2162824u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2162828u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2162832u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2162836u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2162840u32);
    emu.adi_no_count(11usize, 0usize, 1280u32, 2162844u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2163176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101e8));
    } else {
        emu.pc = 2162848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100a0));
    }
}
#[inline(always)]
pub fn block_0x002100a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 160u32, 2162852u32)?;
    emu.sri_no_count(19usize, 8usize, 5u32, 2162856u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2162940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100fc));
    } else {
        emu.pc = 2162860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100ac));
    }
}
#[inline(always)]
pub fn block_0x002100ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2162864u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2163228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021021c));
    } else {
        emu.pc = 2162868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100b4));
    }
}
#[inline]
pub fn block_0x002100b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2162872u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2162876u32);
    emu.adr_no_count(12usize, 12usize, 19usize, 2162880u32);
    emu.adr_no_count(14usize, 13usize, 10usize, 2162884u32);
    emu.adi_no_count(13usize, 12usize, 4294967295u32, 2162888u32);
    emu.sli_no_count(15usize, 12usize, 2u32, 2162892u32);
    emu.adi_no_count(12usize, 14usize, 4294967292u32, 2162896u32);
    emu.adr_no_count(14usize, 15usize, 10usize, 2162900u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2162904u32);
    emu.adi_no_count(15usize, 0usize, 39u32, 2162908u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2162908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100dc));
}
#[inline(always)]
pub fn block_0x002100dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2163204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210204));
    } else {
        emu.pc = 2162912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100e0));
    }
}
#[inline(always)]
pub fn block_0x002100e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 12usize, 0u32, 2162916u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2162920u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2162924u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2162928u32);
    emu.sw_no_count(16usize, 14usize, 0u32, 2162932u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2162936u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2162908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100dc));
    } else {
        emu.pc = 2162940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100fc));
    }
}
#[inline(always)]
pub fn block_0x002100fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 8usize, 31u32, 2162944u32);
    emu.adi_no_count(11usize, 0usize, 32u32, 2162948u32);
    emu.sli_no_count(9usize, 19usize, 2u32, 2162952u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2162980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210124));
    } else {
        emu.pc = 2162956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021010c));
    }
}
#[inline(always)]
pub fn block_0x0021010c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2162960u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2162964u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2162968u32);
    emu.apc_no_count(1usize, 2162968u32, 4294922240u32, 2162972u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162976u32;
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
pub fn block_0x00210120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2162980u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210124));
}
#[inline(always)]
pub fn block_0x00210124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 160u32, 2162984u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2162988u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2163136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101c0));
    } else {
        emu.pc = 2162992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210130));
    }
}
#[inline(always)]
pub fn block_0x00210130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2162996u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2163000u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2163204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210204));
    } else {
        emu.pc = 2163004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021013c));
    }
}
#[inline(always)]
pub fn block_0x0021013c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2163008u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2163012u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2163016u32)?;
    emu.sbr_no_count(12usize, 0usize, 8usize, 2163020u32);
    emu.srr_no_count(15usize, 11usize, 12usize, 2163024u32);
    emu.sli_no_count(13usize, 14usize, 2u32, 2163028u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2163032u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2163056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210170));
    } else {
        emu.pc = 2163036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021015c));
    }
}
#[inline(always)]
pub fn block_0x0021015c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2163040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2163252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210234));
    } else {
        emu.pc = 2163044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210164));
    }
}
#[inline(always)]
pub fn block_0x00210164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 13usize, 2163048u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2163052u32)?;
    emu.adi_no_count(11usize, 14usize, 1u32, 2163056u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2163056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210170));
}
#[inline(always)]
pub fn block_0x00210170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2163060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2163116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101ac));
    } else {
        emu.pc = 2163064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210178));
    }
}
#[inline(always)]
pub fn block_0x00210178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 31u32, 2163068u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2163072u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2163076u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2163076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210184));
}
#[inline]
pub fn block_0x00210184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 13usize, 0u32, 2163080u32)?;
    emu.lw_no_count(16usize, 13usize, 4294967292u32, 2163084u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2163088u32);
    emu.slr_no_count(15usize, 15usize, 20usize, 2163092u32);
    emu.srr_no_count(16usize, 16usize, 12usize, 2163096u32);
    emu.adi_no_count(17usize, 13usize, 4294967292u32, 2163100u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2163104u32);
    emu.sw_no_count(15usize, 13usize, 0u32, 2163108u32)?;
    emu.adi_no_count(13usize, 17usize, 0u32, 2163112u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2163076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210184));
    } else {
        emu.pc = 2163116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101ac));
    }
}
#[inline(always)]
pub fn block_0x002101ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 10usize, 9usize, 2163120u32);
    emu.lw_no_count(12usize, 9usize, 0u32, 2163124u32)?;
    emu.slr_no_count(12usize, 12usize, 20usize, 2163128u32);
    emu.sw_no_count(12usize, 9usize, 0u32, 2163132u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2163136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002101c4));
}
#[inline(always)]
pub fn block_0x002101c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 0u32, 2163140u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002101c4));
}
#[inline]
pub fn block_0x002101c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 160u32, 2163144u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2163148u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2163152u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2163156u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2163160u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2163164u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2163168u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2163172u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163176u32;
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
pub fn block_0x002101e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2163180u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1542u32, 2163184u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2163188u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2163192u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2163196u32);
    emu.apc_no_count(1usize, 2163196u32, 0u32, 2163200u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2163208u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2163212u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2163216u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2163220u32);
    emu.apc_no_count(1usize, 2163220u32, 0u32, 2163224u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021021c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 4294967295u32, 2163232u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2163236u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2163240u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2163244u32);
    emu.apc_no_count(1usize, 2163244u32, 0u32, 2163248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163252u32;
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
pub fn block_0x00210234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2163256u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2163260u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2163264u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2163268u32);
    emu.apc_no_count(1usize, 2163268u32, 0u32, 2163272u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163276u32;
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
#[inline]
pub fn block_0x0021024c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2163280u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2163284u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2163288u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2163292u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2163296u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2163300u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2163304u32)?;
    emu.sw_no_count(22usize, 2usize, 4u32, 2163308u32)?;
    emu.sli_no_count(12usize, 12usize, 2u32, 2163312u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2163316u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2163580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021037c));
    } else {
        emu.pc = 2163320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210278));
    }
}
#[inline]
pub fn block_0x00210278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 10usize, 0u32, 2163324u32);
    emu.adi_no_count(5usize, 0usize, 0u32, 2163328u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2163332u32);
    emu.sli_no_count(6usize, 14usize, 2u32, 2163336u32);
    emu.sltru_no_count(15usize, 0usize, 14usize, 2163340u32);
    emu.adi_no_count(17usize, 14usize, 1u32, 2163344u32);
    emu.adi_no_count(7usize, 14usize, 4294967295u32, 2163348u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2163352u32);
    emu.sli_no_count(15usize, 15usize, 2u32, 2163356u32);
    emu.sli_no_count(28usize, 7usize, 2u32, 2163360u32);
    emu.adr_no_count(7usize, 13usize, 15usize, 2163364u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2163368u32);
    emu.adi_no_count(28usize, 28usize, 1u32, 2163372u32);
    emu.adi_no_count(29usize, 0usize, 40u32, 2163376u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2163376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102b0));
}
#[inline(always)]
pub fn block_0x002102b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(9usize, 5usize, 2u32, 2163380u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2163384u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2163388u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2163388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102bc));
}
#[inline(always)]
pub fn block_0x002102bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2163640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103b8));
    } else {
        emu.pc = 2163392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102c0));
    }
}
#[inline(always)]
pub fn block_0x002102c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 5usize, 0u32, 2163396u32);
    emu.adi_no_count(31usize, 9usize, 0u32, 2163400u32);
    emu.lw_no_count(8usize, 15usize, 0u32, 2163404u32)?;
    emu.adi_no_count(5usize, 5usize, 1u32, 2163408u32);
    emu.adi_no_count(11usize, 15usize, 4u32, 2163412u32);
    emu.adi_no_count(9usize, 9usize, 4u32, 2163416u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2163420u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2163388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102bc));
    } else {
        emu.pc = 2163424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102e0));
    }
}
#[inline(always)]
pub fn block_0x002102e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2163428u32);
    emu.adi_no_count(9usize, 28usize, 0u32, 2163432u32);
    emu.adi_no_count(15usize, 30usize, 0u32, 2163436u32);
    emu.adi_no_count(20usize, 7usize, 0u32, 2163440u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2163444u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2163444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102f4));
}
#[inline(always)]
pub fn block_0x002102f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2163676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103dc));
    } else {
        emu.pc = 2163448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102f8));
    }
}
#[inline]
pub fn block_0x002102f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 20usize, 0u32, 2163452u32);
    emu.lw_no_count(21usize, 21usize, 0u32, 2163456u32)?;
    emu.lw_no_count(20usize, 31usize, 0u32, 2163460u32)?;
    emu.mulhu_no_count(22usize, 21usize, 8usize, 2163464u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2163468u32);
    emu.sltru_no_count(20usize, 18usize, 20usize, 2163472u32);
    emu.adr_no_count(22usize, 20usize, 22usize, 2163476u32);
    emu.xrr_no_count(20usize, 19usize, 6usize, 2163480u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2163484u32);
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2163488u32);
    emu.sltru_no_count(20usize, 0usize, 20usize, 2163492u32);
    emu.sli_no_count(20usize, 20usize, 2u32, 2163496u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2163500u32);
    emu.mul_no_count(21usize, 21usize, 8usize, 2163504u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2163508u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2163512u32);
    emu.sw_no_count(21usize, 31usize, 0u32, 2163516u32)?;
    emu.adr_no_count(18usize, 22usize, 18usize, 2163520u32);
    emu.adi_no_count(31usize, 31usize, 4u32, 2163524u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2163528u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2163444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102f4));
    } else {
        emu.pc = 2163532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021034c));
    }
}
#[inline(always)]
pub fn block_0x0021034c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2163536u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2163564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021036c));
    } else {
        emu.pc = 2163540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210354));
    }
}
#[inline(always)]
pub fn block_0x00210354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 30usize, 14usize, 2163544u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2163676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103dc));
    } else {
        emu.pc = 2163548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021035c));
    }
}
#[inline(always)]
pub fn block_0x0021035c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 2u32, 2163552u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2163556u32);
    emu.sw_no_count(18usize, 15usize, 0u32, 2163560u32)?;
    emu.adi_no_count(15usize, 17usize, 0u32, 2163564u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2163564u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021036c));
}
#[inline(always)]
pub fn block_0x0021036c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 15usize, 30usize, 2163568u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2163376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102b0));
    } else {
        emu.pc = 2163572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210374));
    }
}
#[inline(always)]
pub fn block_0x00210374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 15usize, 0u32, 2163576u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102b0));
}
#[inline(always)]
pub fn block_0x0021037c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2163584u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2163588u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2163588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210384));
}
#[inline(always)]
pub fn block_0x00210384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2163592u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2163596u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2163596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021038c));
}
#[inline(always)]
pub fn block_0x0021038c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2163640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103b8));
    } else {
        emu.pc = 2163600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210390));
    }
}
#[inline(always)]
pub fn block_0x00210390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2163604u32)?;
    emu.adi_no_count(11usize, 14usize, 4u32, 2163608u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2163612u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2163616u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2163596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021038c));
    } else {
        emu.pc = 2163620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103a4));
    }
}
#[inline(always)]
pub fn block_0x002103a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(14usize, 13usize, 4294967295u32, 2163624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2163632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103b0));
    } else {
        emu.pc = 2163628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103ac));
    }
}
#[inline(always)]
pub fn block_0x002103ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2163632u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002103b0));
}
#[inline(always)]
pub fn block_0x002103b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2163636u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210384));
}
#[inline]
pub fn block_0x002103b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 28u32, 2163644u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2163648u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2163652u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2163656u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2163660u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2163664u32)?;
    emu.lw_no_count(22usize, 2usize, 4u32, 2163668u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2163672u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163676u32;
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
pub fn block_0x002103dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2163680u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2163684u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2163688u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2163692u32);
    emu.apc_no_count(1usize, 2163692u32, 4294963200u32, 2163696u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002103f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2163704u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2163708u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2163712u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2163716u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2163720u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2163724u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2163728u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2163732u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2163736u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2163740u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2163744u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2163748u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2163752u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2163756u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2163760u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2163764u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2163768u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2163772u32);
    emu.adi_no_count(25usize, 0usize, 0u32, 2163776u32);
    let a = 0u32.wrapping_add(168431616u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2163780u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2163784u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 10usize, 0u32, 2163788u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2163792u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2163796u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2163800u32)?;
    emu.lw_no_count(22usize, 10usize, 8u32, 2163804u32)?;
    emu.adi_no_count(10usize, 9usize, 4294967295u32, 2163808u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2163812u32)?;
    emu.adi_no_count(10usize, 9usize, 4u32, 2163816u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2163820u32)?;
    emu.sbr_no_count(10usize, 0usize, 8usize, 2163824u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2163828u32)?;
    emu.adi_no_count(27usize, 11usize, 4294965770u32, 2163832u32);
    emu.adi_no_count(19usize, 12usize, 256u32, 2163836u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2163840u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2163844u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 10usize, 128u32, 2163848u32);
    emu.add_memory_rw_events(38usize);
    let return_addr = 2163852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163908u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104c4));
}
#[inline(always)]
pub fn block_0x0021048c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2163856u32)?;
    emu.adr_no_count(10usize, 10usize, 26usize, 2163860u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2163864u32);
    emu.adi_no_count(10usize, 10usize, 4294967286u32, 2163868u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2163872u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2163872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104a0));
}
#[inline(always)]
pub fn block_0x002104a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 22usize, 0u32, 2163876u32);
    emu.lw_no_count(10usize, 2usize, 20u32, 2163880u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2163884u32)?;
    emu.sbr_no_count(12usize, 26usize, 20usize, 2163888u32);
    emu.adr_no_count(11usize, 9usize, 20usize, 2163892u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2163896u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2163900u32;
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
pub fn block_0x002104bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2163904u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2164324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210664));
    } else {
        emu.pc = 2163908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104c4));
    }
}
#[inline(always)]
pub fn block_0x002104c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 25usize, 1u32, 2163912u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2164316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021065c));
    } else {
        emu.pc = 2163916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104cc));
    }
}
#[inline(always)]
pub fn block_0x002104cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104e0));
    } else {
        emu.pc = 2163920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104d0));
    }
}
#[inline(always)]
pub fn block_0x002104d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 21usize, 0u32, 2163924u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163928u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210614));
}
#[inline(always)]
pub fn block_0x002104d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 26usize, 0u32, 2163932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2164244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210614));
    } else {
        emu.pc = 2163936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104e0));
    }
}
#[inline(always)]
pub fn block_0x002104e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 8usize, 21usize, 2163940u32);
    emu.adr_no_count(10usize, 9usize, 21usize, 2163944u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2163948u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2163992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210518));
    } else {
        emu.pc = 2163952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104f0));
    }
}
#[inline(always)]
pub fn block_0x002104f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2164240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210610));
    } else {
        emu.pc = 2163956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104f4));
    }
}
#[inline(always)]
pub fn block_0x002104f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2163960u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2163964u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2163968u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2163968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210500));
}
#[inline(always)]
pub fn block_0x00210500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2163972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2164164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105c4));
    } else {
        emu.pc = 2163976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210508));
    }
}
#[inline(always)]
pub fn block_0x00210508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2163980u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2163984u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2163968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210500));
    } else {
        emu.pc = 2163988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210514));
    }
}
#[inline(always)]
pub fn block_0x00210514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163992u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210610));
}
#[inline(always)]
pub fn block_0x00210518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 3u32, 2163996u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2164000u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2164084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210574));
    } else {
        emu.pc = 2164004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210524));
    }
}
#[inline(always)]
pub fn block_0x00210524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2164008u32);
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2164012u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2164012u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021052c));
}
#[inline(always)]
pub fn block_0x0021052c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 8u32, 2164016u32)?;
    emu.adr_no_count(14usize, 14usize, 21usize, 2164020u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2164020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210534));
}
#[inline]
pub fn block_0x00210534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2164024u32);
    emu.adr_no_count(16usize, 14usize, 12usize, 2164028u32);
    emu.lw_no_count(15usize, 15usize, 0u32, 2164032u32)?;
    emu.lw_no_count(16usize, 16usize, 0u32, 2164036u32)?;
    emu.xrr_no_count(17usize, 15usize, 27usize, 2164040u32);
    emu.xrr_no_count(16usize, 16usize, 27usize, 2164044u32);
    emu.sbr_no_count(17usize, 19usize, 17usize, 2164048u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2164052u32);
    emu.sbr_no_count(17usize, 19usize, 16usize, 2164056u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2164060u32);
    emu.anr_no_count(15usize, 15usize, 16usize, 2164064u32);
    emu.anr_no_count(15usize, 15usize, 23usize, 2164068u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2164120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210598));
    } else {
        emu.pc = 2164072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210568));
    }
}
#[inline(always)]
pub fn block_0x00210568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 8u32, 2164076u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2164020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210534));
    } else {
        emu.pc = 2164080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210570));
    }
}
#[inline(always)]
pub fn block_0x00210570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164120u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210598));
}
#[inline(always)]
pub fn block_0x00210574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2164088u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2164092u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2164092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021057c));
}
#[inline(always)]
pub fn block_0x0021057c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 10usize, 13usize, 2164096u32);
    emu.lbu_no_count(14usize, 14usize, 0u32, 2164100u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2164168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105c8));
    } else {
        emu.pc = 2164104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210588));
    }
}
#[inline(always)]
pub fn block_0x00210588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2164108u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2164092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021057c));
    } else {
        emu.pc = 2164112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210590));
    }
}
#[inline(always)]
pub fn block_0x00210590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2164116u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2164012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021052c));
    } else {
        emu.pc = 2164120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210598));
    }
}
#[inline(always)]
pub fn block_0x00210598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2164240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210610));
    } else {
        emu.pc = 2164124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021059c));
    }
}
#[inline(always)]
pub fn block_0x0021059c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 12usize, 2164128u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2164132u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2164136u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2164140u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2164140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002105ac));
}
#[inline(always)]
pub fn block_0x002105ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2164144u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2164184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105d8));
    } else {
        emu.pc = 2164148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105b4));
    }
}
#[inline(always)]
pub fn block_0x002105b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2164152u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2164156u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2164140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105ac));
    } else {
        emu.pc = 2164160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105c0));
    }
}
#[inline(always)]
pub fn block_0x002105c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210610));
}
#[inline(always)]
pub fn block_0x002105c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 11usize, 2164168u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002105c8));
}
#[inline(always)]
pub fn block_0x002105c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 13usize, 2164172u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2164176u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2163928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104d8));
    } else {
        emu.pc = 2164180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105d4));
    }
}
#[inline(always)]
pub fn block_0x002105d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164200u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002105e8));
}
#[inline(always)]
pub fn block_0x002105d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2164188u32);
    emu.adr_no_count(10usize, 21usize, 13usize, 2164192u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2164196u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2163928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104d8));
    } else {
        emu.pc = 2164200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105e8));
    }
}
#[inline(always)]
pub fn block_0x002105e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 9usize, 21usize, 2164204u32);
    emu.adr_no_count(13usize, 21usize, 13usize, 2164208u32);
    emu.lbu_no_count(10usize, 13usize, 0u32, 2164212u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104d8));
    } else {
        emu.pc = 2164216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105f8));
    }
}
#[inline(always)]
pub fn block_0x002105f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2164220u32);
    emu.adi_no_count(18usize, 26usize, 0u32, 2164224u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2164228u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2164232u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2164304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210650));
    } else {
        emu.pc = 2164236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021060c));
    }
}
#[inline(always)]
pub fn block_0x0021060c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210630));
}
#[inline(always)]
pub fn block_0x00210610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 8usize, 0u32, 2164244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210614));
}
#[inline(always)]
pub fn block_0x00210614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2164316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021065c));
    } else {
        emu.pc = 2164248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210618));
    }
}
#[inline(always)]
pub fn block_0x00210618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 1u32, 2164252u32);
    emu.adi_no_count(18usize, 20usize, 0u32, 2164256u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2164260u32);
    emu.adi_no_count(26usize, 8usize, 0u32, 2164264u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2164268u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2164304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210650));
    } else {
        emu.pc = 2164272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210630));
    }
}
#[inline(always)]
pub fn block_0x00210630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2164276u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2164280u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2164284u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2164288u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2164292u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966844u32, 2164296u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2164300u32;
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
pub fn block_0x0021064c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210664));
    } else {
        emu.pc = 2164304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210650));
    }
}
#[inline(always)]
pub fn block_0x00210650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2163852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021048c));
    } else {
        emu.pc = 2164308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210654));
    }
}
#[inline(always)]
pub fn block_0x00210654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2164312u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104a0));
}
#[inline(always)]
pub fn block_0x0021065c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2164320u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164324u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210668));
}
#[inline(always)]
pub fn block_0x00210664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2164328u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210668));
}
#[inline]
pub fn block_0x00210668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2164332u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2164336u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2164340u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2164344u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2164348u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2164352u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2164356u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2164360u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2164364u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2164368u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2164372u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2164376u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2164380u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2164384u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164388u32;
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
