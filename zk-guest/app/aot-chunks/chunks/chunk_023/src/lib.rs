pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2172948u32;
pub const PC_MAX: u32 = 2175500u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 127usize] = [
        block_0x00212814,
        block_0x0021285c,
        block_0x00212864,
        block_0x00212868,
        block_0x00212894,
        block_0x002128f4,
        block_0x00212950,
        block_0x0021298c,
        block_0x00212998,
        block_0x002129a0,
        block_0x002129b0,
        block_0x00212a50,
        block_0x00212a58,
        block_0x00212a68,
        block_0x00212aa8,
        block_0x00212ab4,
        block_0x00212abc,
        block_0x00212ac8,
        block_0x00212ae4,
        block_0x00212ae8,
        block_0x00212af0,
        block_0x00212b08,
        block_0x00212b10,
        block_0x00212b1c,
        block_0x00212b2c,
        block_0x00212b34,
        block_0x00212b4c,
        block_0x00212b58,
        block_0x00212b60,
        block_0x00212b6c,
        block_0x00212b70,
        block_0x00212b88,
        block_0x00212bf0,
        block_0x00212bf8,
        block_0x00212c04,
        block_0x00212c14,
        block_0x00212c1c,
        block_0x00212c34,
        block_0x00212c38,
        block_0x00212c40,
        block_0x00212c5c,
        block_0x00212c64,
        block_0x00212c84,
        block_0x00212c8c,
        block_0x00212ca0,
        block_0x00212cb0,
        block_0x00212cb8,
        block_0x00212cc0,
        block_0x00212cc8,
        block_0x00212ce0,
        block_0x00212d08,
        block_0x00212d0c,
        block_0x00212d38,
        block_0x00212d40,
        block_0x00212d50,
        block_0x00212d54,
        block_0x00212d70,
        block_0x00212d74,
        block_0x00212d88,
        block_0x00212d9c,
        block_0x00212da4,
        block_0x00212dc4,
        block_0x00212dcc,
        block_0x00212dd4,
        block_0x00212de8,
        block_0x00212e04,
        block_0x00212e08,
        block_0x00212e18,
        block_0x00212e1c,
        block_0x00212e34,
        block_0x00212e38,
        block_0x00212e54,
        block_0x00212e5c,
        block_0x00212e94,
        block_0x00212eb0,
        block_0x00212ecc,
        block_0x00212ee8,
        block_0x00212f00,
        block_0x00212f18,
        block_0x00212f30,
        block_0x00212f5c,
        block_0x00212f64,
        block_0x00212f68,
        block_0x00212f70,
        block_0x00212f80,
        block_0x00212f88,
        block_0x00212f8c,
        block_0x00212f98,
        block_0x00212fa8,
        block_0x00212fb0,
        block_0x00212fb4,
        block_0x00212fc0,
        block_0x00212fec,
        block_0x00212ff4,
        block_0x00212ff8,
        block_0x00213014,
        block_0x00213024,
        block_0x00213028,
        block_0x00213030,
        block_0x00213034,
        block_0x0021303c,
        block_0x0021305c,
        block_0x00213064,
        block_0x00213068,
        block_0x00213074,
        block_0x0021307c,
        block_0x0021308c,
        block_0x002130a8,
        block_0x002130b0,
        block_0x002130e0,
        block_0x002130f8,
        block_0x00213104,
        block_0x00213108,
        block_0x00213134,
        block_0x00213138,
        block_0x00213148,
        block_0x0021317c,
        block_0x00213198,
        block_0x002131a0,
        block_0x002131a4,
        block_0x002131b4,
        block_0x002131b8,
        block_0x002131c0,
        block_0x002131c4,
        block_0x002131d4,
        block_0x002131f0,
        block_0x0021320c,
    ];
    const IDX: [u16; 639usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 4u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        12u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 16u16,
        0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 20u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 0u16, 0u16, 24u16,
        0u16, 0u16, 0u16, 25u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16,
        0u16, 28u16, 0u16, 29u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        33u16, 0u16, 34u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16,
        0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16,
        0u16, 48u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 55u16, 56u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16, 0u16, 0u16, 59u16,
        0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 62u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 70u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16,
        0u16, 82u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16, 87u16,
        0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16, 90u16, 91u16, 0u16, 0u16,
        92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16,
        94u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16,
        98u16, 0u16, 99u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 102u16, 0u16, 103u16, 104u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 109u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 112u16, 113u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 115u16, 0u16, 0u16, 0u16, 116u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 119u16, 120u16, 0u16, 0u16,
        0u16, 121u16, 122u16, 0u16, 123u16, 124u16, 0u16, 0u16, 0u16, 125u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 127u16,
    ];
    if pc < 2172948u32 || pc > 2175500u32 {
        return None;
    }
    let word_offset = ((pc - 2172948u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00212814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2172952u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2172956u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2172960u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2172964u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2172968u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2172972u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2172976u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2172980u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2172984u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2172988u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2172992u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2172996u32)?;
    emu.sw_no_count(26usize, 2usize, 0u32, 2173000u32)?;
    emu.adi_no_count(15usize, 14usize, 0u32, 2173004u32);
    emu.lw_no_count(17usize, 11usize, 0u32, 2173008u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2173012u32)?;
    emu.orr_no_count(14usize, 17usize, 16usize, 2173016u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2174612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e94));
    } else {
        emu.pc = 2173020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021285c));
    }
}
#[inline(always)]
pub fn block_0x0021285c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 16usize, 29u32, 2173024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2174640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212eb0));
    } else {
        emu.pc = 2173028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212864));
    }
}
#[inline(always)]
pub fn block_0x00212864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2174668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ecc));
    } else {
        emu.pc = 2173032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212868));
    }
}
#[inline]
pub fn block_0x00212868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(14usize, 11usize, 24u32, 2173036u32)?;
    emu.sri_no_count(11usize, 17usize, 1u32, 2173040u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2173044u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2173048u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2173052u32;
    emu.update_insn_clock();
    emu.adi_no_count(28usize, 5usize, 1365u32, 2173056u32);
    emu.adi_no_count(7usize, 6usize, 819u32, 2173060u32);
    emu.adi_no_count(5usize, 29usize, 4294967055u32, 2173064u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2173068u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 257u32, 2173072u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2173172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002128f4));
    } else {
        emu.pc = 2173076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212894));
    }
}
#[inline]
pub fn block_0x00212894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(29usize, 17usize, 11usize, 2173080u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2173084u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173088u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2173092u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173096u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2173100u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173104u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2173108u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173112u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2173116u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2173120u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2173124u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2173128u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2173132u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2173136u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2173140u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2173144u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2173148u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2173152u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2173156u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2173160u32);
    emu.sri_no_count(5usize, 5usize, 24u32, 2173164u32);
    emu.adi_no_count(6usize, 5usize, 32u32, 2173168u32);
    emu.add_memory_rw_events(24usize);
    let return_addr = 2173172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212950));
}
#[inline]
pub fn block_0x002128f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(29usize, 16usize, 1u32, 2173176u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2173180u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2173184u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173188u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2173192u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173196u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2173200u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173204u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2173208u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2173212u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2173216u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2173220u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2173224u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2173228u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2173232u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2173236u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2173240u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2173244u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2173248u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2173252u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2173256u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2173260u32);
    emu.sri_no_count(6usize, 5usize, 24u32, 2173264u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2173264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212950));
}
#[inline]
pub fn block_0x00212950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 14usize, 6usize, 2173268u32);
    emu.adi_no_count(14usize, 0usize, 4294967200u32, 2173272u32);
    emu.adi_no_count(7usize, 0usize, 80u32, 2173276u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2173280u32;
    emu.update_insn_clock();
    emu.sbr_no_count(14usize, 14usize, 5usize, 2173284u32);
    emu.adi_no_count(28usize, 28usize, 4294965651u32, 2173288u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2173292u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2173296u32);
    emu.adi_no_count(14usize, 14usize, 1087u32, 2173300u32);
    emu.mul_no_count(14usize, 14usize, 7usize, 2173304u32);
    emu.mulh_no_count(14usize, 14usize, 28usize, 2173308u32);
    emu.sri_no_count(28usize, 14usize, 31u32, 2173312u32);
    emu.sai_no_count(14usize, 14usize, 1034u32, 2173316u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2173320u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2174744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f18));
    } else {
        emu.pc = 2173324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021298c));
    }
}
#[inline(always)]
pub fn block_0x0021298c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 6usize, 4294967264u32, 2173328u32);
    emu.slr_no_count(17usize, 17usize, 6usize, 2173332u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2173344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129a0));
    } else {
        emu.pc = 2173336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212998));
    }
}
#[inline(always)]
pub fn block_0x00212998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 17usize, 0u32, 2173340u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002129b0));
}
#[inline(always)]
pub fn block_0x002129a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(28usize, 6usize, 4294967295u32, 2173348u32);
    emu.srr_no_count(11usize, 11usize, 28usize, 2173352u32);
    emu.slr_no_count(16usize, 16usize, 6usize, 2173356u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2173360u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2173360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002129b0));
}
#[inline(never)]
pub fn block_0x002129b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(16usize, 7usize, 1055u32, 2173364u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2173368u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2173372u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 4294966120u32, 2173376u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2173380u32);
    emu.lw_no_count(6usize, 14usize, 0u32, 2173384u32)?;
    emu.lw_no_count(7usize, 14usize, 4u32, 2173388u32)?;
    emu.anr_no_count(16usize, 16usize, 17usize, 2173392u32);
    emu.lh_no_count(17usize, 14usize, 8u32, 2173396u32)?;
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2173400u32);
    emu.mul_no_count(29usize, 7usize, 16usize, 2173404u32);
    emu.mulhu_no_count(30usize, 7usize, 16usize, 2173408u32);
    emu.mul_no_count(31usize, 6usize, 11usize, 2173412u32);
    emu.mulhu_no_count(6usize, 6usize, 11usize, 2173416u32);
    emu.mul_no_count(8usize, 7usize, 11usize, 2173420u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2173424u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2173428u32);
    emu.adi_no_count(16usize, 0usize, 4294967232u32, 2173432u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2173436u32);
    emu.sbr_no_count(5usize, 16usize, 17usize, 2173440u32);
    emu.sbr_no_count(16usize, 0usize, 17usize, 2173444u32);
    emu.sltru_no_count(17usize, 28usize, 29usize, 2173448u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2173452u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2173456u32);
    emu.ani_no_count(29usize, 5usize, 63u32, 2173460u32);
    emu.sltru_no_count(7usize, 28usize, 31usize, 2173464u32);
    emu.sri_no_count(30usize, 28usize, 31u32, 2173468u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2173472u32);
    emu.adi_no_count(28usize, 29usize, 4294967264u32, 2173476u32);
    emu.adr_no_count(6usize, 17usize, 6usize, 2173480u32);
    emu.sltru_no_count(17usize, 6usize, 17usize, 2173484u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2173488u32);
    emu.sltru_no_count(7usize, 6usize, 8usize, 2173492u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2173496u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2173500u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2173504u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2173508u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2173512u32);
    emu.xri_no_count(8usize, 29usize, 4294967295u32, 2173516u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2173528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a58));
    } else {
        emu.pc = 2173520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a50));
    }
}
#[inline(always)]
pub fn block_0x00212a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(11usize, 7usize, 29usize, 2173524u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a68));
}
#[inline(always)]
pub fn block_0x00212a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 7usize, 1u32, 2173532u32);
    emu.slr_no_count(11usize, 11usize, 8usize, 2173536u32);
    emu.srr_no_count(17usize, 6usize, 16usize, 2173540u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2173544u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2173544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a68));
}
#[inline]
pub fn block_0x00212a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(14usize, 14usize, 10u32, 2173548u32)?;
    emu.slti_no_count(17usize, 28usize, 0u32, 2173552u32);
    emu.adi_no_count(30usize, 0usize, 1u32, 2173556u32);
    emu.slr_no_count(31usize, 30usize, 29usize, 2173560u32);
    emu.adi_no_count(9usize, 17usize, 4294967295u32, 2173564u32);
    emu.sbr_no_count(17usize, 0usize, 17usize, 2173568u32);
    emu.slr_no_count(16usize, 30usize, 16usize, 2173572u32);
    emu.anr_no_count(31usize, 9usize, 31usize, 2173576u32);
    emu.anr_no_count(30usize, 17usize, 16usize, 2173580u32);
    emu.sltiu_no_count(16usize, 30usize, 1u32, 2173584u32);
    emu.adi_no_count(9usize, 30usize, 4294967295u32, 2173588u32);
    emu.sbr_no_count(18usize, 31usize, 16usize, 2173592u32);
    emu.anr_no_count(17usize, 18usize, 7usize, 2173596u32);
    emu.anr_no_count(16usize, 9usize, 6usize, 2173600u32);
    emu.orr_no_count(19usize, 16usize, 17usize, 2173604u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2173672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ae8));
    } else {
        emu.pc = 2173608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212aa8));
    }
}
#[inline(always)]
pub fn block_0x00212aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(19usize, 11usize, 4u32, 2173612u32);
    emu.adi_no_count(20usize, 0usize, 625u32, 2173616u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2173712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b10));
    } else {
        emu.pc = 2173620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ab4));
    }
}
#[inline(always)]
pub fn block_0x00212ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 100u32, 2173624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2173792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b60));
    } else {
        emu.pc = 2173628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212abc));
    }
}
#[inline(always)]
pub fn block_0x00212abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 9u32, 2173632u32);
    emu.sltiu_no_count(20usize, 11usize, 10u32, 2173636u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2174440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212de8));
    } else {
        emu.pc = 2173640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ac8));
    }
}
#[inline(always)]
pub fn block_0x00212ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2173644u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2173648u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2173652u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2173656u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2173660u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2173664u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2173832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b88));
    } else {
        emu.pc = 2173668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ae4));
    }
}
#[inline(always)]
pub fn block_0x00212ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2173672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b4c));
}
#[inline(always)]
pub fn block_0x00212ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2173676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2173704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b08));
    } else {
        emu.pc = 2173680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212af0));
    }
}
#[inline(always)]
pub fn block_0x00212af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(19usize, 13usize, 2u32, 2173684u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2173688u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 712u32, 2173692u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2173696u32);
    emu.lw_no_count(19usize, 19usize, 4294967292u32, 2173700u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2173608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212aa8));
    } else {
        emu.pc = 2173704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b08));
    }
}
#[inline(always)]
pub fn block_0x00212b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2173708u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173712u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212e5c));
}
#[inline(always)]
pub fn block_0x00212b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2173716u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2173720u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2173944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bf8));
    } else {
        emu.pc = 2173724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b1c));
    }
}
#[inline(always)]
pub fn block_0x00212b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2173728u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1696u32, 2173732u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2173736u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2173748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b34));
    } else {
        emu.pc = 2173740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b2c));
    }
}
#[inline(always)]
pub fn block_0x00212b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2173744u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1808u32, 2173748u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2173748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b34));
}
#[inline(always)]
pub fn block_0x00212b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 5u32, 2173752u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2173756u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2173760u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2173764u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2173768u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2173832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b88));
    } else {
        emu.pc = 2173772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b4c));
    }
}
#[inline(always)]
pub fn block_0x00212b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(7usize, 14usize, 15usize, 2173776u32);
    emu.sli_no_count(6usize, 5usize, 16u32, 2173780u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2174008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c38));
    } else {
        emu.pc = 2173784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b58));
    }
}
#[inline(always)]
pub fn block_0x00212b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 13usize, 0u32, 2173788u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173792u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212c40));
}
#[inline(always)]
pub fn block_0x00212b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 1000u32, 2173796u32);
    emu.sltiu_no_count(20usize, 11usize, 1000u32, 2173800u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2173808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b70));
    } else {
        emu.pc = 2173804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b6c));
    }
}
#[inline(always)]
pub fn block_0x00212b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1000u32, 2173808u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b70));
}
#[inline(always)]
pub fn block_0x00212b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 3u32, 2173812u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2173816u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2173820u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2173824u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2173828u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2173772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b4c));
    } else {
        emu.pc = 2173832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b88));
    }
}
#[inline(never)]
pub fn block_0x00212b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 6usize, 1u32, 2173836u32);
    emu.sli_no_count(16usize, 7usize, 31u32, 2173840u32);
    emu.sri_no_count(17usize, 7usize, 1u32, 2173844u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2173848u32;
    emu.update_insn_clock();
    emu.orr_no_count(11usize, 11usize, 16usize, 2173852u32);
    emu.adi_no_count(6usize, 5usize, 4294966477u32, 2173856u32);
    emu.adi_no_count(16usize, 5usize, 4294966476u32, 2173860u32);
    emu.adr_no_count(5usize, 11usize, 17usize, 2173864u32);
    emu.sltru_no_count(7usize, 5usize, 11usize, 2173868u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2173872u32);
    emu.mulhu_no_count(7usize, 5usize, 6usize, 2173876u32);
    emu.sri_no_count(9usize, 7usize, 2u32, 2173880u32);
    emu.ani_no_count(7usize, 7usize, 4294967292u32, 2173884u32);
    emu.adr_no_count(7usize, 7usize, 9usize, 2173888u32);
    emu.sbr_no_count(5usize, 5usize, 7usize, 2173892u32);
    emu.sbr_no_count(7usize, 11usize, 5usize, 2173896u32);
    emu.sltru_no_count(11usize, 11usize, 5usize, 2173900u32);
    emu.mul_no_count(5usize, 7usize, 16usize, 2173904u32);
    emu.mulhu_no_count(9usize, 7usize, 6usize, 2173908u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2173912u32);
    emu.mul_no_count(16usize, 7usize, 6usize, 2173916u32);
    emu.adr_no_count(17usize, 9usize, 5usize, 2173920u32);
    emu.mul_no_count(11usize, 11usize, 6usize, 2173924u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2173928u32);
    emu.slr_no_count(11usize, 19usize, 29usize, 2173932u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2174144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cc0));
    } else {
        emu.pc = 2173936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bf0));
    }
}
#[inline(always)]
pub fn block_0x00212bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 11usize, 0u32, 2173940u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212cc8));
}
#[inline(always)]
pub fn block_0x00212bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2173948u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 256u32, 2173952u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2174472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e08));
    } else {
        emu.pc = 2173956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c04));
    }
}
#[inline(always)]
pub fn block_0x00212c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2173960u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1664u32, 2173964u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2173968u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2173980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c1c));
    } else {
        emu.pc = 2173972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c14));
    }
}
#[inline(always)]
pub fn block_0x00212c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2173976u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2173980u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2173980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212c1c));
}
#[inline(always)]
pub fn block_0x00212c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 7u32, 2173984u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2173988u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2173992u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2173996u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2174000u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2173772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b4c));
    } else {
        emu.pc = 2174004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c34));
    }
}
#[inline(always)]
pub fn block_0x00212c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174008u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b88));
}
#[inline(always)]
pub fn block_0x00212c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(7usize, 7usize, 16u32, 2174012u32);
    emu.sai_no_count(5usize, 7usize, 1040u32, 2174016u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2174016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212c40));
}
#[inline(always)]
pub fn block_0x00212c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(7usize, 6usize, 16u32, 2174020u32);
    emu.adi_no_count(22usize, 0usize, 4294967295u32, 2174024u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2174028u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 0usize, 10u32, 2174032u32);
    emu.adi_no_count(24usize, 6usize, 4294966477u32, 2174036u32);
    emu.adi_no_count(21usize, 0usize, 4294967295u32, 2174040u32);
    emu.adi_no_count(25usize, 12usize, 0u32, 2174044u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2174044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212c5c));
}
#[inline(always)]
pub fn block_0x00212c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 13usize, 21usize, 2174048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2174696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ee8));
    } else {
        emu.pc = 2174052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c64));
    }
}
#[inline(always)]
pub fn block_0x00212c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 19usize, 0u32, 2174056u32);
    emu.divu_no_count(19usize, 11usize, 19usize, 2174060u32);
    emu.mul_no_count(26usize, 19usize, 6usize, 2174064u32);
    emu.sbr_no_count(11usize, 11usize, 26usize, 2174068u32);
    emu.adr_no_count(26usize, 5usize, 21usize, 2174072u32);
    emu.adi_no_count(19usize, 19usize, 48u32, 2174076u32);
    emu.sb_no_count(19usize, 25usize, 0u32, 2174080u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2174128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cb0));
    } else {
        emu.pc = 2174084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c84));
    }
}
#[inline(always)]
pub fn block_0x00212c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 20usize, 21usize, 2174088u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2174176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ce0));
    } else {
        emu.pc = 2174092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c8c));
    }
}
#[inline(always)]
pub fn block_0x00212c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(19usize, 6usize, 24usize, 2174096u32);
    emu.adi_no_count(25usize, 25usize, 1u32, 2174100u32);
    emu.sri_no_count(19usize, 19usize, 3u32, 2174104u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2174108u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2174044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c5c));
    } else {
        emu.pc = 2174112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ca0));
    }
}
#[inline(always)]
pub fn block_0x00212ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174116u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 752u32, 2174120u32);
    emu.apc_no_count(1usize, 2174120u32, 0u32, 2174124u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(7usize, 11usize, 29usize, 2174132u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2174364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d9c));
    } else {
        emu.pc = 2174136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212cb8));
    }
}
#[inline(always)]
pub fn block_0x00212cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 7usize, 0u32, 2174140u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2174144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212da4));
}
#[inline(always)]
pub fn block_0x00212cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 19usize, 1u32, 2174148u32);
    emu.srr_no_count(29usize, 5usize, 8usize, 2174152u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2174152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212cc8));
}
#[inline(always)]
pub fn block_0x00212cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 28usize, 1055u32, 2174156u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2174160u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2174164u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2174168u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2174172u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2174176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212e54));
}
#[inline]
pub fn block_0x00212ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2174180u32);
    emu.adi_no_count(19usize, 7usize, 4294967295u32, 2174184u32);
    emu.sbr_no_count(11usize, 0usize, 21usize, 2174188u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2174192u32);
    emu.ani_no_count(19usize, 19usize, 63u32, 2174196u32);
    emu.xri_no_count(20usize, 19usize, 4294967295u32, 2174200u32);
    emu.adi_no_count(21usize, 19usize, 4294967264u32, 2174204u32);
    emu.sai_no_count(22usize, 21usize, 1055u32, 2174208u32);
    emu.adi_no_count(23usize, 0usize, 10u32, 2174212u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2174216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d38));
}
#[inline(always)]
pub fn block_0x00212d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 17usize, 29usize, 2174220u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2174220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d0c));
}
#[inline]
pub fn block_0x00212d0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(17usize, 17usize, 18usize, 2174224u32);
    emu.anr_no_count(16usize, 16usize, 9usize, 2174228u32);
    emu.mulhu_no_count(25usize, 7usize, 23usize, 2174232u32);
    emu.mul_no_count(6usize, 6usize, 23usize, 2174236u32);
    emu.mul_no_count(7usize, 7usize, 23usize, 2174240u32);
    emu.adi_no_count(24usize, 24usize, 48u32, 2174244u32);
    emu.adr_no_count(6usize, 25usize, 6usize, 2174248u32);
    emu.adr_no_count(25usize, 12usize, 11usize, 2174252u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2174256u32);
    emu.sb_no_count(24usize, 25usize, 0u32, 2174260u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2174520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e38));
    } else {
        emu.pc = 2174264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d38));
    }
}
#[inline(always)]
pub fn block_0x00212d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 6usize, 19usize, 2174268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2174292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d54));
    } else {
        emu.pc = 2174272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d40));
    }
}
#[inline(always)]
pub fn block_0x00212d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2174276u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2174280u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2174284u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2174320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d70));
    } else {
        emu.pc = 2174288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d50));
    }
}
#[inline(always)]
pub fn block_0x00212d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b08));
}
#[inline(always)]
pub fn block_0x00212d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(25usize, 7usize, 19usize, 2174296u32);
    emu.sli_no_count(26usize, 6usize, 1u32, 2174300u32);
    emu.slr_no_count(26usize, 26usize, 20usize, 2174304u32);
    emu.orr_no_count(25usize, 25usize, 26usize, 2174308u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2174312u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2174316u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2173704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b08));
    } else {
        emu.pc = 2174320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d70));
    }
}
#[inline(always)]
pub fn block_0x00212d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2174720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f00));
    } else {
        emu.pc = 2174324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d74));
    }
}
#[inline(always)]
pub fn block_0x00212d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(24usize, 16usize, 23usize, 2174328u32);
    emu.mul_no_count(17usize, 17usize, 23usize, 2174332u32);
    emu.adr_no_count(17usize, 24usize, 17usize, 2174336u32);
    emu.mul_no_count(16usize, 16usize, 23usize, 2174340u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2174216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d08));
    } else {
        emu.pc = 2174344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212d88));
    }
}
#[inline(always)]
pub fn block_0x00212d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 17usize, 1u32, 2174348u32);
    emu.slr_no_count(24usize, 24usize, 8usize, 2174352u32);
    emu.srr_no_count(25usize, 16usize, 29usize, 2174356u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2174360u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2174364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212d0c));
}
#[inline(always)]
pub fn block_0x00212d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 11usize, 1u32, 2174368u32);
    emu.srr_no_count(9usize, 11usize, 8usize, 2174372u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2174372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212da4));
}
#[inline(always)]
pub fn block_0x00212da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(11usize, 28usize, 1055u32, 2174376u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2174380u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2174384u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2174388u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2174392u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2174396u32);
    emu.slr_no_count(7usize, 6usize, 29usize, 2174400u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2174412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212dcc));
    } else {
        emu.pc = 2174404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212dc4));
    }
}
#[inline(always)]
pub fn block_0x00212dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 7usize, 0u32, 2174408u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2174412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174420u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212dd4));
}
#[inline(always)]
pub fn block_0x00212dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 6usize, 1u32, 2174416u32);
    emu.srr_no_count(29usize, 6usize, 8usize, 2174420u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2174420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212dd4));
}
#[inline(always)]
pub fn block_0x00212dd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 11usize, 7usize, 2174424u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2174428u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2174432u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2174436u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2174440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212e54));
}
#[inline(always)]
pub fn block_0x00212de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2174444u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2174448u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2174452u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2174456u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2174460u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2174464u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2173832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b88));
    } else {
        emu.pc = 2174468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e04));
    }
}
#[inline(always)]
pub fn block_0x00212e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174472u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b4c));
}
#[inline(always)]
pub fn block_0x00212e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2174476u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 20usize, 4294965760u32, 2174480u32);
    emu.sltru_no_count(20usize, 11usize, 21usize, 2174484u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2174492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e1c));
    } else {
        emu.pc = 2174488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e18));
    }
}
#[inline(always)]
pub fn block_0x00212e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 0u32, 2174492u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2174492u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212e1c));
}
#[inline(always)]
pub fn block_0x00212e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 9u32, 2174496u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2174500u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2174504u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2174508u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2174512u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2173772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b4c));
    } else {
        emu.pc = 2174516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212e34));
    }
}
#[inline(always)]
pub fn block_0x00212e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b88));
}
#[inline(always)]
pub fn block_0x00212e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2174524u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2174528u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2174532u32);
    emu.adi_no_count(28usize, 30usize, 0u32, 2174536u32);
    emu.adi_no_count(29usize, 31usize, 0u32, 2174540u32);
    emu.adi_no_count(30usize, 7usize, 0u32, 2174544u32);
    emu.adi_no_count(31usize, 6usize, 0u32, 2174548u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2174548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212e54));
}
#[inline(always)]
pub fn block_0x00212e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2174548u32, 0u32, 2174552u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00212e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2174560u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2174564u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2174568u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2174572u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2174576u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2174580u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2174584u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2174588u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2174592u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2174596u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2174600u32)?;
    emu.lw_no_count(26usize, 2usize, 0u32, 2174604u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2174608u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174612u32;
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
pub fn block_0x00212e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174616u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 136u32, 2174620u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174624u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 628u32, 2174628u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2174632u32);
    emu.apc_no_count(1usize, 2174632u32, 4294955008u32, 2174636u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174640u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174644u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 644u32, 2174648u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174652u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 680u32, 2174656u32);
    emu.adi_no_count(11usize, 0usize, 36u32, 2174660u32);
    emu.apc_no_count(1usize, 2174660u32, 4294955008u32, 2174664u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174668u32;
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
pub fn block_0x00212ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2174672u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 592u32, 2174676u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174680u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 696u32, 2174684u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2174688u32);
    emu.apc_no_count(1usize, 2174688u32, 4294955008u32, 2174692u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174696u32;
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
pub fn block_0x00212ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174700u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 768u32, 2174704u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2174708u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2174712u32);
    emu.apc_no_count(1usize, 2174712u32, 4294955008u32, 2174716u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174724u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 784u32, 2174728u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2174732u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2174736u32);
    emu.apc_no_count(1usize, 2174736u32, 4294955008u32, 2174740u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2174748u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 120u32, 2174752u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2174756u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2174760u32);
    emu.apc_no_count(1usize, 2174760u32, 4294955008u32, 2174764u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00212f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2174772u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2174776u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2174780u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2174784u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2174788u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2174792u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2174796u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2174800u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2174804u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2174808u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2174824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f68));
    } else {
        emu.pc = 2174812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f5c));
    }
}
#[inline(always)]
pub fn block_0x00212f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 29usize, 2174816u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2174832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f70));
    } else {
        emu.pc = 2174820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f64));
    }
}
#[inline(always)]
pub fn block_0x00212f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213104));
}
#[inline(always)]
pub fn block_0x00212f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 28usize, 2174828u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2175236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213104));
    } else {
        emu.pc = 2174832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f70));
    }
}
#[inline(always)]
pub fn block_0x00212f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 30usize, 2174836u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2174840u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2174844u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2174860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f8c));
    } else {
        emu.pc = 2174848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f80));
    }
}
#[inline(always)]
pub fn block_0x00212f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 5usize, 2174852u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2174872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f98));
    } else {
        emu.pc = 2174856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f88));
    }
}
#[inline(always)]
pub fn block_0x00212f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174860u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213104));
}
#[inline(always)]
pub fn block_0x00212f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 30usize, 2174864u32);
    emu.sltru_no_count(5usize, 30usize, 5usize, 2174868u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2175236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213104));
    } else {
        emu.pc = 2174872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212f98));
    }
}
#[inline(always)]
pub fn block_0x00212f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 16usize, 2174876u32);
    emu.sbr_no_count(6usize, 29usize, 17usize, 2174880u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2174884u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2174900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fb4));
    } else {
        emu.pc = 2174888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fa8));
    }
}
#[inline(always)]
pub fn block_0x00212fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 5usize, 2174892u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2174912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fc0));
    } else {
        emu.pc = 2174896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fb0));
    }
}
#[inline(always)]
pub fn block_0x00212fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2174900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213024));
}
#[inline(always)]
pub fn block_0x00212fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 16usize, 2174904u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2174908u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2175012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213024));
    } else {
        emu.pc = 2174912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fc0));
    }
}
#[inline]
pub fn block_0x00212fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 16usize, 31u32, 2174916u32);
    emu.sli_no_count(7usize, 17usize, 1u32, 2174920u32);
    emu.sli_no_count(5usize, 16usize, 1u32, 2174924u32);
    emu.sri_no_count(8usize, 30usize, 31u32, 2174928u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2174932u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2174936u32);
    emu.sbr_no_count(6usize, 29usize, 6usize, 2174940u32);
    emu.sbr_no_count(6usize, 6usize, 7usize, 2174944u32);
    emu.sli_no_count(7usize, 31usize, 1u32, 2174948u32);
    emu.orr_no_count(7usize, 7usize, 8usize, 2174952u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2174996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213014));
    } else {
        emu.pc = 2174956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212fec));
    }
}
#[inline(always)]
pub fn block_0x00212fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 7usize, 2174960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2175012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213024));
    } else {
        emu.pc = 2174964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ff4));
    }
}
#[inline(always)]
pub fn block_0x00212ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2175428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131c4));
    } else {
        emu.pc = 2174968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ff8));
    }
}
#[inline(always)]
pub fn block_0x00212ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2174972u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 832u32, 2174976u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2174980u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2174984u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2174988u32);
    emu.apc_no_count(1usize, 2174988u32, 16384u32, 2174992u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00213014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 5usize, 2175000u32);
    emu.sli_no_count(6usize, 30usize, 1u32, 2175004u32);
    emu.sltru_no_count(5usize, 5usize, 6usize, 2175008u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2174964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212ff4));
    } else {
        emu.pc = 2175012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213024));
    }
}
#[inline(always)]
pub fn block_0x00213024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2175028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213034));
    } else {
        emu.pc = 2175016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213028));
    }
}
#[inline(always)]
pub fn block_0x00213028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 17usize, 2175020u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2175036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021303c));
    } else {
        emu.pc = 2175024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213030));
    }
}
#[inline(always)]
pub fn block_0x00213030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2175028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213104));
}
#[inline(always)]
pub fn block_0x00213034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 16usize, 2175032u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2175236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213104));
    } else {
        emu.pc = 2175036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021303c));
    }
}
#[inline(always)]
pub fn block_0x0021303c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 30usize, 2175040u32);
    emu.sbr_no_count(17usize, 17usize, 31usize, 2175044u32);
    emu.sbr_no_count(16usize, 16usize, 30usize, 2175048u32);
    emu.sbr_no_count(17usize, 17usize, 5usize, 2175052u32);
    emu.sbr_no_count(5usize, 29usize, 17usize, 2175056u32);
    emu.sltru_no_count(6usize, 28usize, 16usize, 2175060u32);
    emu.sbr_no_count(5usize, 5usize, 6usize, 2175064u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2175224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002130f8));
    } else {
        emu.pc = 2175068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021305c));
    }
}
#[inline(always)]
pub fn block_0x0021305c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 17usize, 5usize, 2175072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2175236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213104));
    } else {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213064));
    }
}
#[inline(always)]
pub fn block_0x00213064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2175444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131d4));
    } else {
        emu.pc = 2175080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213068));
    }
}
#[inline(always)]
pub fn block_0x00213068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2175084u32);
    emu.adr_no_count(8usize, 11usize, 13usize, 2175088u32);
    emu.adi_no_count(17usize, 0usize, 57u32, 2175092u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2175092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213074));
}
#[inline(always)]
pub fn block_0x00213074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 13usize, 16usize, 2175096u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2175284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213134));
    } else {
        emu.pc = 2175100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021307c));
    }
}
#[inline(always)]
pub fn block_0x0021307c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 8usize, 16usize, 2175104u32);
    emu.lbu_no_count(5usize, 5usize, 4294967295u32, 2175108u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2175112u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2175092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213074));
    } else {
        emu.pc = 2175116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021308c));
    }
}
#[inline(always)]
pub fn block_0x0021308c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 8usize, 16usize, 2175120u32);
    emu.lbu_no_count(15usize, 8usize, 0u32, 2175124u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2175128u32);
    emu.adi_no_count(5usize, 15usize, 1u32, 2175132u32);
    emu.adi_no_count(15usize, 17usize, 1u32, 2175136u32);
    emu.sb_no_count(5usize, 8usize, 0u32, 2175140u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2175500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021320c));
    } else {
        emu.pc = 2175144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002130a8));
    }
}
#[inline(always)]
pub fn block_0x002130a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2175148u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2175424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131c0));
    } else {
        emu.pc = 2175152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002130b0));
    }
}
#[inline]
pub fn block_0x002130b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2175156u32);
    emu.adi_no_count(16usize, 8usize, 1u32, 2175160u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2175164u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2175168u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2175172u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2175176u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2175180u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2175184u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2175188u32);
    emu.adi_no_count(20usize, 14usize, 0u32, 2175192u32);
    emu.apc_no_count(1usize, 2175192u32, 4294909952u32, 2175196u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002130e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2175204u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2175208u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2175212u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2175216u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2175220u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2175224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002131c0));
}
#[inline(always)]
pub fn block_0x002130f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(17usize, 28usize, 16usize, 2175228u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2175232u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2175076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213064));
    } else {
        emu.pc = 2175236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213104));
    }
}
#[inline(always)]
pub fn block_0x00213104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2175240u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2175240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213108));
}
#[inline]
pub fn block_0x00213108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2175244u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2175248u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2175252u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2175256u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2175260u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2175264u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2175268u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2175272u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2175276u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2175280u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175284u32;
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
pub fn block_0x00213134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2175384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213198));
    } else {
        emu.pc = 2175288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213138));
    }
}
#[inline(always)]
pub fn block_0x00213138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 49u32, 2175292u32);
    emu.adi_no_count(16usize, 13usize, 4294967295u32, 2175296u32);
    emu.sb_no_count(17usize, 11usize, 0u32, 2175300u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2175392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131a0));
    } else {
        emu.pc = 2175304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213148));
    }
}
#[inline]
pub fn block_0x00213148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 11usize, 1u32, 2175308u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2175312u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2175316u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2175320u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2175324u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2175328u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2175332u32);
    emu.adi_no_count(12usize, 16usize, 0u32, 2175336u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2175340u32);
    emu.adi_no_count(23usize, 14usize, 0u32, 2175344u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2175348u32);
    emu.apc_no_count(1usize, 2175348u32, 4294909952u32, 2175352u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175356u32;
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
pub fn block_0x0021317c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 22usize, 0u32, 2175360u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2175364u32);
    emu.adi_no_count(14usize, 23usize, 0u32, 2175368u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2175372u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2175376u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2175380u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2175384u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002131a4));
}
#[inline(always)]
pub fn block_0x00213198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2175388u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2175392u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002131a4));
}
#[inline(always)]
pub fn block_0x002131a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2175396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2175396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002131a4));
}
#[inline(always)]
pub fn block_0x002131a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 1u32, 2175400u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2175404u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2175408u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2175424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131c0));
    } else {
        emu.pc = 2175412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131b4));
    }
}
#[inline(always)]
pub fn block_0x002131b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2175424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131c0));
    } else {
        emu.pc = 2175416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131b8));
    }
}
#[inline(always)]
pub fn block_0x002131b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2175420u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2175424u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2175424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002131c0));
}
#[inline(always)]
pub fn block_0x002131c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2175472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131f0));
    } else {
        emu.pc = 2175428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002131c4));
    }
}
#[inline(always)]
pub fn block_0x002131c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2175432u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2175436u32)?;
    emu.sh_no_count(14usize, 10usize, 8u32, 2175440u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2175444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2175240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213108));
}
#[inline(always)]
pub fn block_0x002131d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2175448u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 800u32, 2175452u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2175456u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2175460u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2175464u32);
    emu.apc_no_count(1usize, 2175464u32, 16384u32, 2175468u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002131f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2175476u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 816u32, 2175480u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2175484u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2175488u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2175492u32);
    emu.apc_no_count(1usize, 2175492u32, 16384u32, 2175496u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021320c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2175504u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1648u32, 2175508u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2175512u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2175516u32);
    emu.apc_no_count(1usize, 2175516u32, 16384u32, 2175520u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
