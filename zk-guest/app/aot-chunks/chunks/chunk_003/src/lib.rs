pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2107260u32;
pub const PC_MAX: u32 = 2110740u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 145usize] = [
        block_0x0020277c,
        block_0x00202798,
        block_0x002027b4,
        block_0x002027bc,
        block_0x002027cc,
        block_0x002027d8,
        block_0x002027fc,
        block_0x0020281c,
        block_0x00202828,
        block_0x00202844,
        block_0x0020284c,
        block_0x00202864,
        block_0x00202868,
        block_0x0020287c,
        block_0x00202898,
        block_0x002028ac,
        block_0x002028c0,
        block_0x002028d0,
        block_0x00202900,
        block_0x00202920,
        block_0x00202930,
        block_0x00202934,
        block_0x002029a0,
        block_0x002029ac,
        block_0x002029b0,
        block_0x002029c8,
        block_0x002029d8,
        block_0x002029f0,
        block_0x002029f4,
        block_0x002029fc,
        block_0x00202a08,
        block_0x00202a14,
        block_0x00202a34,
        block_0x00202a4c,
        block_0x00202a50,
        block_0x00202a68,
        block_0x00202a6c,
        block_0x00202a74,
        block_0x00202a90,
        block_0x00202a94,
        block_0x00202aa0,
        block_0x00202aa8,
        block_0x00202abc,
        block_0x00202ad0,
        block_0x00202ad8,
        block_0x00202ae8,
        block_0x00202b00,
        block_0x00202b04,
        block_0x00202b14,
        block_0x00202b24,
        block_0x00202b34,
        block_0x00202b50,
        block_0x00202b64,
        block_0x00202b84,
        block_0x00202b94,
        block_0x00202b98,
        block_0x00202c04,
        block_0x00202c10,
        block_0x00202c20,
        block_0x00202c30,
        block_0x00202c54,
        block_0x00202c5c,
        block_0x00202c68,
        block_0x00202c7c,
        block_0x00202cac,
        block_0x00202cb4,
        block_0x00202cc4,
        block_0x00202cd0,
        block_0x00202cf4,
        block_0x00202d10,
        block_0x00202d18,
        block_0x00202d94,
        block_0x00202dac,
        block_0x00202dc0,
        block_0x00202dc8,
        block_0x00202dcc,
        block_0x00202e04,
        block_0x00202e18,
        block_0x00202e28,
        block_0x00202e38,
        block_0x00202e40,
        block_0x00202e54,
        block_0x00202e64,
        block_0x00202e74,
        block_0x00202e94,
        block_0x00202ea4,
        block_0x00202eb4,
        block_0x00202ec4,
        block_0x00202ecc,
        block_0x00202ee0,
        block_0x00202ef0,
        block_0x00202f00,
        block_0x00202f1c,
        block_0x00202f24,
        block_0x00202f6c,
        block_0x00202fa0,
        block_0x00202fbc,
        block_0x00202fc4,
        block_0x00202fe0,
        block_0x00202fe8,
        block_0x00203004,
        block_0x00203008,
        block_0x00203048,
        block_0x0020305c,
        block_0x00203070,
        block_0x00203080,
        block_0x002030e0,
        block_0x00203104,
        block_0x00203114,
        block_0x00203124,
        block_0x0020312c,
        block_0x00203144,
        block_0x00203164,
        block_0x00203174,
        block_0x00203178,
        block_0x002031e4,
        block_0x002031f0,
        block_0x00203204,
        block_0x00203214,
        block_0x00203220,
        block_0x0020325c,
        block_0x00203278,
        block_0x00203280,
        block_0x00203288,
        block_0x002032a0,
        block_0x002032c0,
        block_0x002032d0,
        block_0x002032d4,
        block_0x002032f0,
        block_0x002032f8,
        block_0x00203364,
        block_0x00203370,
        block_0x00203380,
        block_0x00203390,
        block_0x002033a0,
        block_0x002033bc,
        block_0x002033c4,
        block_0x002033cc,
        block_0x002033ec,
        block_0x00203400,
        block_0x00203410,
        block_0x002034d4,
        block_0x002034f0,
        block_0x002034f8,
        block_0x00203514,
    ];
    const IDX: [u16; 871usize] = [
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
        0u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16,
        0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 0u16, 0u16, 55u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16,
        0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16,
        0u16, 62u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 67u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16,
        0u16, 75u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 79u16,
        0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        85u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 88u16,
        0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 94u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 98u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16,
        0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        108u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16, 0u16, 111u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16,
        0u16, 0u16, 0u16, 114u16, 115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 0u16, 117u16, 0u16, 0u16, 0u16, 0u16,
        118u16, 0u16, 0u16, 0u16, 119u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 122u16, 0u16, 123u16, 0u16, 124u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 125u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16,
        0u16, 127u16, 128u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 129u16, 0u16, 130u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        131u16, 0u16, 0u16, 132u16, 0u16, 0u16, 0u16, 133u16, 0u16, 0u16, 0u16, 134u16,
        0u16, 0u16, 0u16, 135u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 136u16, 0u16,
        137u16, 0u16, 138u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 139u16, 0u16,
        0u16, 0u16, 0u16, 140u16, 0u16, 0u16, 0u16, 141u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 142u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 143u16, 0u16,
        144u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 145u16,
    ];
    if pc < 2107260u32 || pc > 2110740u32 {
        return None;
    }
    let word_offset = ((pc - 2107260u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020277c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2107264u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2107268u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2107272u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2107276u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2107280u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2107284u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2107388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027fc));
    } else {
        emu.pc = 2107288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202798));
    }
}
#[inline(always)]
pub fn block_0x00202798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2107292u32)?;
    emu.lbu_no_count(9usize, 10usize, 0u32, 2107296u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2107300u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2107304u32);
    emu.sw_no_count(10usize, 11usize, 0u32, 2107308u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2107312u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2107432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202828));
    } else {
        emu.pc = 2107316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027b4));
    }
}
#[inline(always)]
pub fn block_0x002027b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2107320u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2107460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202844));
    } else {
        emu.pc = 2107324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027bc));
    }
}
#[inline(always)]
pub fn block_0x002027bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2107328u32);
    emu.adi_no_count(15usize, 0usize, 5u32, 2107332u32);
    emu.apc_no_count(1usize, 2107332u32, 4096u32, 2107336u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002027cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2107344u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107348u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2107544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202898));
    } else {
        emu.pc = 2107352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027d8));
    }
}
#[inline]
pub fn block_0x002027d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2107356u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2107360u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2107364u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2107368u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2107372u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2107376u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2107380u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2107384u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107388u32;
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
pub fn block_0x002027fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2107392u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107396u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2107400u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2107404u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2107408u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107412u32);
    emu.apc_no_count(1usize, 2107412u32, 32768u32, 2107416u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107420u32;
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
#[inline(always)]
pub fn block_0x0020281c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107424u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2107428u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2107432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020287c));
}
#[inline(always)]
pub fn block_0x00202828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107436u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2107440u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2107444u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2107448u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2107452u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2107456u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107460u32;
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
pub fn block_0x00202844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107460u32, 28672u32, 2107464u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107468u32;
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
pub fn block_0x0020284c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107472u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2107476u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2107480u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2107484u32);
    emu.apc_no_count(1usize, 2107484u32, 32768u32, 2107488u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107492u32;
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
pub fn block_0x00202864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002028c0));
    } else {
        emu.pc = 2107496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202868));
    }
}
#[inline(always)]
pub fn block_0x00202868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107500u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2107504u32);
    emu.adi_no_count(12usize, 11usize, 3u32, 2107508u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2107512u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2107516u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2107516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020287c));
}
#[inline(always)]
pub fn block_0x0020287c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2107520u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2107524u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2107528u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2107532u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2107536u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2107540u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107544u32;
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
pub fn block_0x00202898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 8u32, 2107548u32);
    emu.adi_no_count(12usize, 0usize, 72u32, 2107552u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2107556u32);
    emu.apc_no_count(1usize, 2107556u32, 32768u32, 2107560u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107564u32;
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
pub fn block_0x002028ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 92u32, 2107568u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2107572u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2107576u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2107580u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107584u32;
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
pub fn block_0x002028c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2107588u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2107592u32);
    emu.apc_no_count(1usize, 2107592u32, 102400u32, 2107596u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002028d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2107604u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2107608u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2107612u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2107616u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2107620u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2107624u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2107628u32)?;
    emu.adi_no_count(18usize, 11usize, 0u32, 2107632u32);
    emu.lw_no_count(20usize, 11usize, 4u32, 2107636u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2107640u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2107644u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2107700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202934));
    } else {
        emu.pc = 2107648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202900));
    }
}
#[inline(always)]
pub fn block_0x00202900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2107652u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107656u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2107660u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2107664u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2107668u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107672u32);
    emu.apc_no_count(1usize, 2107672u32, 32768u32, 2107676u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107680u32;
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
pub fn block_0x00202920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107684u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2107688u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2107692u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2107820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029ac));
    } else {
        emu.pc = 2107696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202930));
    }
}
#[inline(always)]
pub fn block_0x00202930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2107700u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202a08));
}
#[inline(never)]
pub fn block_0x00202934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2107704u32)?;
    emu.lbu_no_count(11usize, 10usize, 1u32, 2107708u32);
    emu.lbu_no_count(12usize, 10usize, 2u32, 2107712u32);
    emu.lbu_no_count(13usize, 10usize, 3u32, 2107716u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2107720u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2107724u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2107728u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2107732u32);
    emu.orr_no_count(11usize, 11usize, 14usize, 2107736u32);
    emu.lbu_no_count(14usize, 10usize, 4u32, 2107740u32);
    emu.lbu_no_count(15usize, 10usize, 5u32, 2107744u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2107748u32);
    emu.lbu_no_count(13usize, 10usize, 6u32, 2107752u32);
    emu.lbu_no_count(16usize, 10usize, 7u32, 2107756u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2107760u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2107764u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2107768u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2107772u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2107776u32);
    emu.adi_no_count(20usize, 20usize, 4294967288u32, 2107780u32);
    emu.adi_no_count(15usize, 10usize, 8u32, 2107784u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2107788u32);
    emu.orr_no_count(11usize, 13usize, 14usize, 2107792u32);
    emu.sw_no_count(15usize, 18usize, 0u32, 2107796u32)?;
    emu.sw_no_count(20usize, 18usize, 4u32, 2107800u32)?;
    emu.apc_no_count(1usize, 2107800u32, 32768u32, 2107804u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107808u32;
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
#[inline(always)]
pub fn block_0x002029a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2107812u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2107816u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2107912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a08));
    } else {
        emu.pc = 2107820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029ac));
    }
}
#[inline(always)]
pub fn block_0x002029ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a34));
    } else {
        emu.pc = 2107824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029b0));
    }
}
#[inline(always)]
pub fn block_0x002029b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2107828u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2107832u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107836u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2107840u32);
    emu.apc_no_count(1usize, 2107840u32, 32768u32, 2107844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107848u32;
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
pub fn block_0x002029c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 8u32, 2107852u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2107856u32)?;
    emu.apc_no_count(1usize, 2107856u32, 28672u32, 2107860u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107864u32;
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
pub fn block_0x002029d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107868u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2107872u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2107876u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2107880u32);
    emu.apc_no_count(1usize, 2107880u32, 32768u32, 2107884u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002029f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2108196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b24));
    } else {
        emu.pc = 2107892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029f4));
    }
}
#[inline(always)]
pub fn block_0x002029f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107896u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107900u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2107900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002029fc));
}
#[inline(always)]
pub fn block_0x002029fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 9usize, 0u32, 2107904u32)?;
    emu.sw_no_count(18usize, 9usize, 4u32, 2107908u32)?;
    emu.sw_no_count(19usize, 9usize, 8u32, 2107912u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2107912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202a08));
}
#[inline(always)]
pub fn block_0x00202a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107916u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2107920u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2107924u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2107924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202a14));
}
#[inline(always)]
pub fn block_0x00202a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2107928u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2107932u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2107936u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2107940u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2107944u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2107948u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2107952u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107956u32;
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
pub fn block_0x00202a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 0u32, 2107960u32)?;
    emu.sbr_no_count(10usize, 20usize, 9usize, 2107964u32);
    emu.adr_no_count(11usize, 19usize, 9usize, 2107968u32);
    emu.sw_no_count(11usize, 18usize, 0u32, 2107972u32)?;
    emu.sw_no_count(10usize, 18usize, 4u32, 2107976u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2108008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a68));
    } else {
        emu.pc = 2107980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a4c));
    }
}
#[inline(always)]
pub fn block_0x00202a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2107984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2107984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202a50));
}
#[inline(always)]
pub fn block_0x00202a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107988u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967148u32, 2107992u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2107996u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2108000u32);
    emu.apc_no_count(1usize, 2108000u32, 102400u32, 2108004u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108008u32;
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
#[inline(always)]
pub fn block_0x00202a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2108064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202aa0));
    } else {
        emu.pc = 2108012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a6c));
    }
}
#[inline(always)]
pub fn block_0x00202a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2108012u32, 28672u32, 2108016u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108020u32;
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
pub fn block_0x00202a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108024u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2108028u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2108032u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2108036u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2108040u32);
    emu.apc_no_count(1usize, 2108040u32, 32768u32, 2108044u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108048u32;
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
pub fn block_0x00202a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a50));
    } else {
        emu.pc = 2108052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a94));
    }
}
#[inline(always)]
pub fn block_0x00202a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2108056u32);
    emu.adi_no_count(20usize, 9usize, 0u32, 2108060u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2108064u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108072u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202aa8));
}
#[inline(always)]
pub fn block_0x00202aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2108068u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2108072u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2108072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202aa8));
}
#[inline(always)]
pub fn block_0x00202aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2108076u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2108080u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2108084u32);
    emu.apc_no_count(1usize, 2108084u32, 32768u32, 2108088u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965800u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2108096u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2108100u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2108104u32);
    emu.apc_no_count(1usize, 2108104u32, 131072u32, 2108108u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2108116u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2108180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b14));
    } else {
        emu.pc = 2108120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ad8));
    }
}
#[inline(always)]
pub fn block_0x00202ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 12u32, 2108124u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2108128u32)?;
    emu.apc_no_count(1usize, 2108128u32, 28672u32, 2108132u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108140u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2108144u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2108148u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2108152u32);
    emu.apc_no_count(1usize, 2108152u32, 32768u32, 2108156u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108160u32;
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
pub fn block_0x00202b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2108196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b24));
    } else {
        emu.pc = 2108164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b04));
    }
}
#[inline(always)]
pub fn block_0x00202b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2108168u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108172u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2108176u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2108180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107900u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002029fc));
}
#[inline(always)]
pub fn block_0x00202b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 8usize, 0u32, 2108184u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2108188u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2108192u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2108196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202a14));
}
#[inline(always)]
pub fn block_0x00202b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2108200u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2108204u32);
    emu.apc_no_count(1usize, 2108204u32, 102400u32, 2108208u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108212u32;
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
pub fn block_0x00202b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2108216u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2108220u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2108224u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2108228u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2108232u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2108236u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2108660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cf4));
    } else {
        emu.pc = 2108240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b50));
    }
}
#[inline(always)]
pub fn block_0x00202b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 15usize, 0u32, 2108244u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2108248u32);
    emu.lw_no_count(10usize, 11usize, 4u32, 2108252u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2108256u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2108312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b98));
    } else {
        emu.pc = 2108260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b64));
    }
}
#[inline(always)]
pub fn block_0x00202b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2108264u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108268u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2108272u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2108276u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2108280u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2108284u32);
    emu.apc_no_count(1usize, 2108284u32, 32768u32, 2108288u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108292u32;
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
pub fn block_0x00202b84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2108296u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108300u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2108304u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2108432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c10));
    } else {
        emu.pc = 2108308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b94));
    }
}
#[inline(always)]
pub fn block_0x00202b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2108312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c30));
}
#[inline(never)]
pub fn block_0x00202b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2108316u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2108320u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2108324u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2108328u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2108332u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2108336u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2108340u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2108344u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2108348u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2108352u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2108356u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2108360u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2108364u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2108368u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2108372u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2108376u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2108380u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2108384u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2108388u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2108392u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2108396u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2108400u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2108404u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2108408u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2108412u32)?;
    emu.apc_no_count(1usize, 2108412u32, 32768u32, 2108416u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108420u32;
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
pub fn block_0x00202c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2108424u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2108428u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2108464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c30));
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
    emu.adi_no_count(10usize, 2usize, 8u32, 2108436u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2108440u32);
    emu.apc_no_count(1usize, 2108440u32, 20480u32, 2108444u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108448u32;
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
pub fn block_0x00202c20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2108452u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2108456u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108460u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2108500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c54));
    } else {
        emu.pc = 2108464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c30));
    }
}
#[inline]
pub fn block_0x00202c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108468u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2108472u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2108476u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2108480u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2108484u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2108488u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2108492u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2108496u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108500u32;
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
pub fn block_0x00202c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2108504u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2108820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d94));
    } else {
        emu.pc = 2108508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c5c));
    }
}
#[inline(always)]
pub fn block_0x00202c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 9usize, 4u32, 2108512u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2108516u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2108540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c7c));
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108524u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2108528u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2108532u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2108536u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2108540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202cc4));
}
#[inline]
pub fn block_0x00202c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 9usize, 0u32, 2108544u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2108548u32)?;
    emu.adi_no_count(6usize, 5usize, 4294967292u32, 2108552u32);
    emu.adi_no_count(7usize, 16usize, 4u32, 2108556u32);
    emu.lbu_no_count(13usize, 16usize, 0u32, 2108560u32);
    emu.lbu_no_count(17usize, 16usize, 1u32, 2108564u32);
    emu.lbu_no_count(14usize, 16usize, 2u32, 2108568u32);
    emu.lbu_no_count(15usize, 16usize, 3u32, 2108572u32);
    emu.adi_no_count(28usize, 0usize, 2u32, 2108576u32);
    emu.sw_no_count(7usize, 9usize, 0u32, 2108580u32)?;
    emu.sw_no_count(6usize, 9usize, 4u32, 2108584u32)?;
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2108844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dac));
    } else {
        emu.pc = 2108588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cac));
    }
}
#[inline(always)]
pub fn block_0x00202cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 0usize, 4u32, 2108592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2108696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d18));
    } else {
        emu.pc = 2108596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cb4));
    }
}
#[inline(always)]
pub fn block_0x00202cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108600u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2108604u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2108608u32)?;
    emu.sw_no_count(7usize, 2usize, 12u32, 2108612u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2108612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202cc4));
}
#[inline(always)]
pub fn block_0x00202cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2108616u32);
    emu.apc_no_count(1usize, 2108616u32, 32768u32, 2108620u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108624u32;
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
#[inline]
pub fn block_0x00202cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108628u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2108632u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2108636u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2108640u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2108644u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2108648u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2108652u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2108656u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108660u32;
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
pub fn block_0x00202cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108664u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966924u32, 2108668u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108672u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2108676u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2108680u32);
    emu.apc_no_count(1usize, 2108680u32, 8192u32, 2108684u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2108692u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c30));
}
#[inline(never)]
pub fn block_0x00202d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 16usize, 8u32, 2108700u32);
    emu.adi_no_count(5usize, 5usize, 4294967288u32, 2108704u32);
    emu.lbu_no_count(7usize, 16usize, 4u32, 2108708u32);
    emu.lbu_no_count(28usize, 16usize, 5u32, 2108712u32);
    emu.lbu_no_count(29usize, 16usize, 6u32, 2108716u32);
    emu.lbu_no_count(16usize, 16usize, 7u32, 2108720u32);
    emu.sw_no_count(6usize, 9usize, 0u32, 2108724u32)?;
    emu.sw_no_count(5usize, 9usize, 4u32, 2108728u32)?;
    emu.sli_no_count(17usize, 17usize, 8u32, 2108732u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2108736u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2108740u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2108744u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2108748u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2108752u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2108756u32);
    emu.sli_no_count(29usize, 29usize, 16u32, 2108760u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2108764u32);
    emu.orr_no_count(14usize, 28usize, 7usize, 2108768u32);
    emu.orr_no_count(15usize, 16usize, 29usize, 2108772u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2108776u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2108780u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2108784u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2108788u32)?;
    emu.sw_no_count(13usize, 8usize, 12u32, 2108792u32)?;
    emu.sw_no_count(14usize, 8usize, 16u32, 2108796u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2108800u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2108804u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2108808u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2108812u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2108816u32);
    emu.add_memory_rw_events(31usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108820u32;
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
pub fn block_0x00202d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108824u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966924u32, 2108828u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108832u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2108836u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108840u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2108844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108864u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dc0));
}
#[inline(always)]
pub fn block_0x00202dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108848u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966924u32, 2108852u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108856u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2108860u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108864u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2108864u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dc0));
}
#[inline(always)]
pub fn block_0x00202dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2108864u32, 8192u32, 2108868u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108872u32;
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
pub fn block_0x00202dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2108876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202cd0));
}
#[inline]
pub fn block_0x00202dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2108880u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2108884u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2108888u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2108892u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2108896u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2108900u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2108904u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2108908u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2108912u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2108916u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2108920u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2108924u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2108928u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2109184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f00));
    } else {
        emu.pc = 2108932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e04));
    }
}
#[inline(always)]
pub fn block_0x00202e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 15usize, 0u32, 2108936u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2108940u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2108944u32);
    emu.apc_no_count(1usize, 2108944u32, 0u32, 2108948u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 2usize, 16u32, 2108956u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2108960u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108964u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2108984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e38));
    } else {
        emu.pc = 2108968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e28));
    }
}
#[inline(always)]
pub fn block_0x00202e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2108972u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2108976u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2108980u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2108984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f6c));
}
#[inline(always)]
pub fn block_0x00202e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2108988u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2109344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fa0));
    } else {
        emu.pc = 2108992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e40));
    }
}
#[inline(always)]
pub fn block_0x00202e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 24u32, 2108996u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2109000u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2109004u32);
    emu.apc_no_count(1usize, 2109004u32, 20480u32, 2109008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 16u32, 2109016u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2109020u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2109024u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2109044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e74));
    } else {
        emu.pc = 2109028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e64));
    }
}
#[inline(always)]
pub fn block_0x00202e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2109032u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2109036u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2109040u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2109044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f6c));
}
#[inline(always)]
pub fn block_0x00202e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2109048u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2109052u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2109056u32)?;
    emu.adi_no_count(25usize, 20usize, 4294967294u32, 2109060u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2109064u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2109068u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2109072u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2109380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fc4));
    } else {
        emu.pc = 2109076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e94));
    }
}
#[inline(always)]
pub fn block_0x00202e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 16u32, 2109080u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2109084u32);
    emu.apc_no_count(1usize, 2109084u32, 0u32, 2109088u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202ea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2109096u32)?;
    emu.lw_no_count(20usize, 2usize, 20u32, 2109100u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109104u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2109124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ec4));
    } else {
        emu.pc = 2109108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202eb4));
    }
}
#[inline(always)]
pub fn block_0x00202eb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2109112u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2109116u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2109120u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2109124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f6c));
}
#[inline(always)]
pub fn block_0x00202ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2109128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2109416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fe8));
    } else {
        emu.pc = 2109132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ecc));
    }
}
#[inline(always)]
pub fn block_0x00202ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 24u32, 2109136u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2109140u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2109144u32);
    emu.apc_no_count(1usize, 2109144u32, 0u32, 2109148u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109152u32;
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
pub fn block_0x00202ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 16u32, 2109156u32)?;
    emu.lw_no_count(10usize, 2usize, 20u32, 2109160u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109164u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2109220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f24));
    } else {
        emu.pc = 2109168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ef0));
    }
}
#[inline(always)]
pub fn block_0x00202ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2109172u32);
    emu.sw_no_count(11usize, 8usize, 0u32, 2109176u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2109180u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2109184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f6c));
}
#[inline(always)]
pub fn block_0x00202f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109188u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967016u32, 2109192u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109196u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2109200u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2109204u32);
    emu.apc_no_count(1usize, 2109204u32, 8192u32, 2109208u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2109216u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108968u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202e28));
}
#[inline]
pub fn block_0x00202f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 24u32, 2109224u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2109228u32)?;
    emu.lw_no_count(14usize, 2usize, 8u32, 2109232u32)?;
    emu.lw_no_count(15usize, 2usize, 12u32, 2109236u32)?;
    emu.sw_no_count(24usize, 8usize, 32u32, 2109240u32)?;
    emu.sw_no_count(20usize, 8usize, 36u32, 2109244u32)?;
    emu.sw_no_count(25usize, 8usize, 40u32, 2109248u32)?;
    emu.sw_no_count(11usize, 8usize, 44u32, 2109252u32)?;
    emu.sw_no_count(15usize, 8usize, 16u32, 2109256u32)?;
    emu.sw_no_count(21usize, 8usize, 20u32, 2109260u32)?;
    emu.sw_no_count(9usize, 8usize, 24u32, 2109264u32)?;
    emu.sw_no_count(22usize, 8usize, 28u32, 2109268u32)?;
    emu.sw_no_count(23usize, 8usize, 0u32, 2109272u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2109276u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2109280u32)?;
    emu.sw_no_count(14usize, 8usize, 12u32, 2109284u32)?;
    emu.sw_no_count(10usize, 8usize, 48u32, 2109288u32)?;
    emu.sw_no_count(12usize, 8usize, 52u32, 2109292u32)?;
    emu.add_memory_rw_events(18usize);
    emu.pc = 2109292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f6c));
}
#[inline]
pub fn block_0x00202f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2109296u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2109300u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2109304u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2109308u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2109312u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2109316u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2109320u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2109324u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2109328u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2109332u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2109336u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2109340u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109344u32;
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
pub fn block_0x00202fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109348u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967016u32, 2109352u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109356u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2109360u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2109364u32);
    emu.apc_no_count(1usize, 2109364u32, 8192u32, 2109368u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2109376u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202e64));
}
#[inline(always)]
pub fn block_0x00202fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109384u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967016u32, 2109388u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109392u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2109396u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2109400u32);
    emu.apc_no_count(1usize, 2109400u32, 8192u32, 2109404u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109408u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2109412u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202eb4));
}
#[inline(always)]
pub fn block_0x00202fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109420u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967016u32, 2109424u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109428u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2109432u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2109436u32);
    emu.apc_no_count(1usize, 2109436u32, 8192u32, 2109440u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2109448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ef0));
}
#[inline]
pub fn block_0x00203008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2109452u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2109456u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2109460u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2109464u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2109468u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2109472u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2109476u32)?;
    emu.sw_no_count(21usize, 2usize, 180u32, 2109480u32)?;
    emu.sw_no_count(22usize, 2usize, 176u32, 2109484u32)?;
    emu.sw_no_count(23usize, 2usize, 172u32, 2109488u32)?;
    emu.sw_no_count(24usize, 2usize, 168u32, 2109492u32)?;
    emu.sw_no_count(25usize, 2usize, 164u32, 2109496u32)?;
    emu.sw_no_count(26usize, 2usize, 160u32, 2109500u32)?;
    emu.sw_no_count(27usize, 2usize, 156u32, 2109504u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2109508u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2110044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020325c));
    } else {
        emu.pc = 2109512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203048));
    }
}
#[inline(always)]
pub fn block_0x00203048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 15usize, 0u32, 2109516u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2109520u32);
    emu.adi_no_count(10usize, 2usize, 72u32, 2109524u32);
    emu.apc_no_count(1usize, 2109524u32, 4294963200u32, 2109528u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020305c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 72u32, 2109536u32)?;
    emu.lw_no_count(9usize, 2usize, 76u32, 2109540u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109544u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2109548u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a != b {
        emu.pc = 2109568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203080));
    } else {
        emu.pc = 2109552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203070));
    }
}
#[inline(always)]
pub fn block_0x00203070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109556u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2109560u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2109564u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2109568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203220));
}
#[inline]
pub fn block_0x00203080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 80u32, 2109572u32)?;
    emu.lw_no_count(11usize, 2usize, 84u32, 2109576u32)?;
    emu.lw_no_count(12usize, 2usize, 88u32, 2109580u32)?;
    emu.lw_no_count(13usize, 2usize, 92u32, 2109584u32)?;
    emu.lw_no_count(20usize, 2usize, 96u32, 2109588u32)?;
    emu.lw_no_count(26usize, 2usize, 100u32, 2109592u32)?;
    emu.lw_no_count(5usize, 2usize, 104u32, 2109596u32)?;
    emu.lw_no_count(6usize, 2usize, 108u32, 2109600u32)?;
    emu.lw_no_count(14usize, 2usize, 112u32, 2109604u32)?;
    emu.lw_no_count(15usize, 2usize, 116u32, 2109608u32)?;
    emu.lw_no_count(16usize, 2usize, 120u32, 2109612u32)?;
    emu.lw_no_count(17usize, 2usize, 124u32, 2109616u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2109620u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2109624u32)?;
    emu.sw_no_count(12usize, 2usize, 64u32, 2109628u32)?;
    emu.sw_no_count(13usize, 2usize, 68u32, 2109632u32)?;
    emu.lw_no_count(10usize, 2usize, 128u32, 2109636u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2109640u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2109644u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2109648u32)?;
    emu.sw_no_count(17usize, 2usize, 48u32, 2109652u32)?;
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2109656u32);
    emu.sw_no_count(10usize, 2usize, 52u32, 2109660u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032d4));
    } else {
        emu.pc = 2109664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030e0));
    }
}
#[inline]
pub fn block_0x002030e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(6usize, 2usize, 28u32, 2109668u32)?;
    emu.sw_no_count(5usize, 2usize, 32u32, 2109672u32)?;
    emu.lw_no_count(27usize, 2usize, 132u32, 2109676u32)?;
    emu.lw_no_count(21usize, 2usize, 136u32, 2109680u32)?;
    emu.lw_no_count(23usize, 2usize, 140u32, 2109684u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2109688u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2109692u32);
    emu.apc_no_count(1usize, 2109692u32, 4294963200u32, 2109696u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 72u32, 2109704u32)?;
    emu.lw_no_count(25usize, 2usize, 76u32, 2109708u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109712u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2109732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203124));
    } else {
        emu.pc = 2109716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203114));
    }
}
#[inline(always)]
pub fn block_0x00203114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109720u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2109724u32)?;
    emu.sw_no_count(25usize, 8usize, 4u32, 2109728u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2109732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203220));
}
#[inline(always)]
pub fn block_0x00203124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2109736u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002033a0));
    } else {
        emu.pc = 2109740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020312c));
    }
}
#[inline(always)]
pub fn block_0x0020312c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 24u32, 2109744u32)?;
    emu.lw_no_count(10usize, 18usize, 4u32, 2109748u32)?;
    emu.lw_no_count(11usize, 2usize, 80u32, 2109752u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2109756u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2109760u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2109816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203178));
    } else {
        emu.pc = 2109764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203144));
    }
}
#[inline(always)]
pub fn block_0x00203144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2109768u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2109776u32);
    emu.sw_no_count(11usize, 2usize, 144u32, 2109780u32)?;
    emu.sw_no_count(10usize, 2usize, 148u32, 2109784u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2109788u32);
    emu.apc_no_count(1usize, 2109788u32, 32768u32, 2109792u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966056u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2109800u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2109804u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2109808u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2109936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031f0));
    } else {
        emu.pc = 2109812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203174));
    }
}
#[inline(always)]
pub fn block_0x00203174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2109816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109972u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203214));
}
#[inline(never)]
pub fn block_0x00203178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 0u32, 2109820u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2109824u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2109828u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2109832u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2109836u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2109840u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2109844u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2109848u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2109852u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2109856u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2109860u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2109864u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2109868u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2109872u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2109876u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2109880u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2109884u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2109888u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2109892u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2109896u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2109900u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2109904u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2109908u32);
    emu.sw_no_count(17usize, 18usize, 0u32, 2109912u32)?;
    emu.sw_no_count(16usize, 18usize, 4u32, 2109916u32)?;
    emu.apc_no_count(1usize, 2109916u32, 32768u32, 2109920u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109924u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002031e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 11usize, 0u32, 2109928u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2109932u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2109972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203214));
    } else {
        emu.pc = 2109936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031f0));
    }
}
#[inline(always)]
pub fn block_0x002031f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 72u32, 2109940u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2109944u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2109948u32);
    emu.apc_no_count(1usize, 2109948u32, 20480u32, 2109952u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109956u32;
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
pub fn block_0x00203204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 72u32, 2109960u32)?;
    emu.lw_no_count(20usize, 2usize, 76u32, 2109964u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109968u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203280));
    } else {
        emu.pc = 2109972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203214));
    }
}
#[inline(always)]
pub fn block_0x00203214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109976u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2109980u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2109984u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2109984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203220));
}
#[inline]
pub fn block_0x00203220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 204u32, 2109988u32)?;
    emu.lw_no_count(8usize, 2usize, 200u32, 2109992u32)?;
    emu.lw_no_count(9usize, 2usize, 196u32, 2109996u32)?;
    emu.lw_no_count(18usize, 2usize, 192u32, 2110000u32)?;
    emu.lw_no_count(19usize, 2usize, 188u32, 2110004u32)?;
    emu.lw_no_count(20usize, 2usize, 184u32, 2110008u32)?;
    emu.lw_no_count(21usize, 2usize, 180u32, 2110012u32)?;
    emu.lw_no_count(22usize, 2usize, 176u32, 2110016u32)?;
    emu.lw_no_count(23usize, 2usize, 172u32, 2110020u32)?;
    emu.lw_no_count(24usize, 2usize, 168u32, 2110024u32)?;
    emu.lw_no_count(25usize, 2usize, 164u32, 2110028u32)?;
    emu.lw_no_count(26usize, 2usize, 160u32, 2110032u32)?;
    emu.lw_no_count(27usize, 2usize, 156u32, 2110036u32)?;
    emu.adi_no_count(2usize, 2usize, 208u32, 2110040u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110044u32;
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
pub fn block_0x0020325c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110048u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966864u32, 2110052u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110056u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2110060u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2110064u32);
    emu.apc_no_count(1usize, 2110064u32, 8192u32, 2110068u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2110076u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110080u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203070));
}
#[inline(always)]
pub fn block_0x00203280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2110084u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002034d4));
    } else {
        emu.pc = 2110088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203288));
    }
}
#[inline(always)]
pub fn block_0x00203288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 16u32, 2110092u32)?;
    emu.lw_no_count(10usize, 18usize, 4u32, 2110096u32)?;
    emu.lw_no_count(11usize, 2usize, 80u32, 2110100u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2110104u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2110108u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2110200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032f8));
    } else {
        emu.pc = 2110112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032a0));
    }
}
#[inline(always)]
pub fn block_0x002032a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2110116u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110120u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2110124u32);
    emu.sw_no_count(11usize, 2usize, 144u32, 2110128u32)?;
    emu.sw_no_count(10usize, 2usize, 148u32, 2110132u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2110136u32);
    emu.apc_no_count(1usize, 2110136u32, 32768u32, 2110140u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2110148u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2110152u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2110156u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2110320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203370));
    } else {
        emu.pc = 2110160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032d0));
    }
}
#[inline(always)]
pub fn block_0x002032d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110352u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203390));
}
#[inline(always)]
pub fn block_0x002032d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110168u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966864u32, 2110172u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110176u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2110180u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2110184u32);
    emu.apc_no_count(1usize, 2110184u32, 8192u32, 2110188u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 10usize, 0u32, 2110196u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109716u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203114));
}
#[inline(never)]
pub fn block_0x002032f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 0u32, 2110204u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2110208u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2110212u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2110216u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2110220u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2110224u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2110228u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2110232u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2110236u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2110240u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2110244u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2110248u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2110252u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2110256u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2110260u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2110264u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2110268u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2110272u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2110276u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2110280u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2110284u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2110288u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2110292u32);
    emu.sw_no_count(17usize, 18usize, 0u32, 2110296u32)?;
    emu.sw_no_count(16usize, 18usize, 4u32, 2110300u32)?;
    emu.apc_no_count(1usize, 2110300u32, 32768u32, 2110304u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110308u32;
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
pub fn block_0x00203364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2110312u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2110316u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2110352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203390));
    } else {
        emu.pc = 2110320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203370));
    }
}
#[inline(always)]
pub fn block_0x00203370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 72u32, 2110324u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2110328u32);
    emu.apc_no_count(1usize, 2110328u32, 20480u32, 2110332u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 72u32, 2110340u32)?;
    emu.lw_no_count(12usize, 2usize, 76u32, 2110344u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110348u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002033c4));
    } else {
        emu.pc = 2110352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203390));
    }
}
#[inline(always)]
pub fn block_0x00203390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110356u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2110360u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2110364u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2110368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203220));
}
#[inline(always)]
pub fn block_0x002033a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110372u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966864u32, 2110376u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110380u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2110384u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2110388u32);
    emu.apc_no_count(1usize, 2110388u32, 8192u32, 2110392u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110396u32;
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
pub fn block_0x002033bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2110400u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109972u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203214));
}
#[inline(always)]
pub fn block_0x002033c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2110408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002034f8));
    } else {
        emu.pc = 2110412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002033cc));
    }
}
#[inline(always)]
pub fn block_0x002033cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 11usize, 0u32, 2110416u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2110420u32)?;
    emu.lw_no_count(10usize, 2usize, 80u32, 2110424u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2110428u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2110432u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2110436u32);
    emu.apc_no_count(1usize, 2110436u32, 4294963200u32, 2110440u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002033ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 72u32, 2110448u32)?;
    emu.lw_no_count(10usize, 2usize, 76u32, 2110452u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110456u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2110460u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2110480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203410));
    } else {
        emu.pc = 2110464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203400));
    }
}
#[inline(always)]
pub fn block_0x00203400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110468u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2110472u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2110476u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2110480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203220));
}
#[inline(never)]
pub fn block_0x00203410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 49u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 80u32, 2110484u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2110488u32)?;
    emu.lw_no_count(14usize, 2usize, 60u32, 2110492u32)?;
    emu.lw_no_count(15usize, 2usize, 64u32, 2110496u32)?;
    emu.lw_no_count(16usize, 2usize, 68u32, 2110500u32)?;
    emu.sw_no_count(24usize, 8usize, 0u32, 2110504u32)?;
    emu.sw_no_count(25usize, 8usize, 4u32, 2110508u32)?;
    emu.lw_no_count(17usize, 2usize, 20u32, 2110512u32)?;
    emu.sw_no_count(17usize, 8usize, 8u32, 2110516u32)?;
    emu.lw_no_count(17usize, 2usize, 16u32, 2110520u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2110524u32)?;
    emu.lw_no_count(17usize, 2usize, 36u32, 2110528u32)?;
    emu.lw_no_count(5usize, 2usize, 40u32, 2110532u32)?;
    emu.lw_no_count(6usize, 2usize, 44u32, 2110536u32)?;
    emu.lw_no_count(7usize, 2usize, 48u32, 2110540u32)?;
    emu.sw_no_count(20usize, 8usize, 16u32, 2110544u32)?;
    emu.lw_no_count(28usize, 2usize, 12u32, 2110548u32)?;
    emu.sw_no_count(28usize, 8usize, 20u32, 2110552u32)?;
    emu.sw_no_count(19usize, 8usize, 24u32, 2110556u32)?;
    emu.lw_no_count(28usize, 2usize, 4u32, 2110560u32)?;
    emu.sw_no_count(28usize, 8usize, 28u32, 2110564u32)?;
    emu.lw_no_count(28usize, 2usize, 52u32, 2110568u32)?;
    emu.sw_no_count(27usize, 8usize, 96u32, 2110572u32)?;
    emu.sw_no_count(21usize, 8usize, 100u32, 2110576u32)?;
    emu.sw_no_count(23usize, 8usize, 104u32, 2110580u32)?;
    emu.sw_no_count(12usize, 8usize, 108u32, 2110584u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2110588u32)?;
    emu.sw_no_count(12usize, 8usize, 32u32, 2110592u32)?;
    emu.sw_no_count(22usize, 8usize, 36u32, 2110596u32)?;
    emu.sw_no_count(9usize, 8usize, 40u32, 2110600u32)?;
    emu.sw_no_count(13usize, 8usize, 44u32, 2110604u32)?;
    emu.sw_no_count(26usize, 8usize, 64u32, 2110608u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2110612u32)?;
    emu.sw_no_count(12usize, 8usize, 68u32, 2110616u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2110620u32)?;
    emu.sw_no_count(12usize, 8usize, 72u32, 2110624u32)?;
    emu.sw_no_count(17usize, 8usize, 76u32, 2110628u32)?;
    emu.sw_no_count(5usize, 8usize, 80u32, 2110632u32)?;
    emu.sw_no_count(6usize, 8usize, 84u32, 2110636u32)?;
    emu.sw_no_count(7usize, 8usize, 88u32, 2110640u32)?;
    emu.sw_no_count(28usize, 8usize, 92u32, 2110644u32)?;
    emu.sw_no_count(14usize, 8usize, 48u32, 2110648u32)?;
    emu.sw_no_count(15usize, 8usize, 52u32, 2110652u32)?;
    emu.sw_no_count(16usize, 8usize, 56u32, 2110656u32)?;
    emu.lw_no_count(12usize, 2usize, 24u32, 2110660u32)?;
    emu.sw_no_count(12usize, 8usize, 60u32, 2110664u32)?;
    emu.sw_no_count(10usize, 8usize, 112u32, 2110668u32)?;
    emu.sw_no_count(11usize, 8usize, 116u32, 2110672u32)?;
    emu.add_memory_rw_events(49usize);
    let return_addr = 2110676u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203220));
}
#[inline(always)]
pub fn block_0x002034d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110680u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966864u32, 2110684u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110688u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2110692u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2110696u32);
    emu.apc_no_count(1usize, 2110696u32, 4096u32, 2110700u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2110708u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110712u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110352u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203390));
}
#[inline(always)]
pub fn block_0x002034f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110716u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966864u32, 2110720u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110724u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2110728u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2110732u32);
    emu.apc_no_count(1usize, 2110732u32, 4096u32, 2110736u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110744u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203400));
}
