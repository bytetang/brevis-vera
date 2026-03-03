pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2230008u32;
pub const PC_MAX: u32 = 2232488u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 126usize] = [
        block_0x002206f8,
        block_0x00220700,
        block_0x00220704,
        block_0x00220730,
        block_0x00220790,
        block_0x002207ec,
        block_0x00220828,
        block_0x00220834,
        block_0x0022083c,
        block_0x0022084c,
        block_0x002208ec,
        block_0x002208f4,
        block_0x00220904,
        block_0x00220944,
        block_0x00220950,
        block_0x00220958,
        block_0x00220964,
        block_0x00220980,
        block_0x00220984,
        block_0x0022098c,
        block_0x002209a4,
        block_0x002209ac,
        block_0x002209b8,
        block_0x002209c8,
        block_0x002209d0,
        block_0x002209e8,
        block_0x002209f4,
        block_0x002209fc,
        block_0x00220a08,
        block_0x00220a0c,
        block_0x00220a24,
        block_0x00220a8c,
        block_0x00220a94,
        block_0x00220aa0,
        block_0x00220ab0,
        block_0x00220ab8,
        block_0x00220ad0,
        block_0x00220ad4,
        block_0x00220adc,
        block_0x00220af8,
        block_0x00220b00,
        block_0x00220b20,
        block_0x00220b28,
        block_0x00220b3c,
        block_0x00220b4c,
        block_0x00220b54,
        block_0x00220b5c,
        block_0x00220b64,
        block_0x00220b7c,
        block_0x00220ba4,
        block_0x00220ba8,
        block_0x00220bd4,
        block_0x00220bdc,
        block_0x00220bec,
        block_0x00220bf0,
        block_0x00220c0c,
        block_0x00220c10,
        block_0x00220c24,
        block_0x00220c38,
        block_0x00220c40,
        block_0x00220c60,
        block_0x00220c68,
        block_0x00220c70,
        block_0x00220c84,
        block_0x00220ca0,
        block_0x00220ca4,
        block_0x00220cb4,
        block_0x00220cb8,
        block_0x00220cd0,
        block_0x00220cd4,
        block_0x00220cf0,
        block_0x00220cf8,
        block_0x00220d30,
        block_0x00220d4c,
        block_0x00220d68,
        block_0x00220d84,
        block_0x00220d9c,
        block_0x00220db4,
        block_0x00220dcc,
        block_0x00220df8,
        block_0x00220e00,
        block_0x00220e04,
        block_0x00220e0c,
        block_0x00220e1c,
        block_0x00220e24,
        block_0x00220e28,
        block_0x00220e34,
        block_0x00220e44,
        block_0x00220e4c,
        block_0x00220e50,
        block_0x00220e5c,
        block_0x00220e88,
        block_0x00220e90,
        block_0x00220e94,
        block_0x00220eb0,
        block_0x00220ec0,
        block_0x00220ec4,
        block_0x00220ecc,
        block_0x00220ed0,
        block_0x00220ed8,
        block_0x00220ef8,
        block_0x00220f00,
        block_0x00220f04,
        block_0x00220f10,
        block_0x00220f18,
        block_0x00220f28,
        block_0x00220f44,
        block_0x00220f4c,
        block_0x00220f7c,
        block_0x00220f94,
        block_0x00220fa0,
        block_0x00220fa4,
        block_0x00220fd0,
        block_0x00220fd4,
        block_0x00220fe4,
        block_0x00221018,
        block_0x00221034,
        block_0x0022103c,
        block_0x00221040,
        block_0x00221050,
        block_0x00221054,
        block_0x0022105c,
        block_0x00221060,
        block_0x00221070,
        block_0x0022108c,
        block_0x002210a8,
    ];
    const IDX: [u16; 621usize] = [
        1u16, 0u16, 2u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16,
        0u16, 8u16, 0u16, 9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 12u16, 0u16, 0u16,
        0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 17u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 22u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 0u16,
        25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 27u16, 0u16, 28u16, 0u16,
        0u16, 29u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 33u16, 0u16, 0u16,
        34u16, 0u16, 0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16,
        38u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        44u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 0u16, 47u16, 0u16, 48u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 59u16,
        0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 62u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16,
        66u16, 0u16, 0u16, 0u16, 67u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16,
        70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 82u16, 0u16,
        83u16, 0u16, 0u16, 0u16, 84u16, 0u16, 85u16, 86u16, 0u16, 0u16, 87u16, 0u16,
        0u16, 0u16, 88u16, 0u16, 89u16, 90u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 96u16, 97u16, 0u16, 98u16, 99u16,
        0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16,
        103u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16,
        0u16, 111u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        113u16, 114u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        117u16, 0u16, 118u16, 119u16, 0u16, 0u16, 0u16, 120u16, 121u16, 0u16, 122u16,
        123u16, 0u16, 0u16, 0u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 125u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16,
    ];
    if pc < 2230008u32 || pc > 2232488u32 {
        return None;
    }
    let word_offset = ((pc - 2230008u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002206f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 16usize, 29u32, 2230012u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2231628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220d4c));
    } else {
        emu.pc = 2230016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220700));
    }
}
#[inline(always)]
pub fn block_0x00220700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2231656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220d68));
    } else {
        emu.pc = 2230020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220704));
    }
}
#[inline]
pub fn block_0x00220704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(14usize, 11usize, 24u32, 2230024u32)?;
    emu.sri_no_count(11usize, 17usize, 1u32, 2230028u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2230032u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2230036u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2230040u32;
    emu.update_insn_clock();
    emu.adi_no_count(28usize, 5usize, 1365u32, 2230044u32);
    emu.adi_no_count(7usize, 6usize, 819u32, 2230048u32);
    emu.adi_no_count(5usize, 29usize, 4294967055u32, 2230052u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2230056u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 257u32, 2230060u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2230160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220790));
    } else {
        emu.pc = 2230064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220730));
    }
}
#[inline]
pub fn block_0x00220730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(29usize, 17usize, 11usize, 2230068u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2230072u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230076u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2230080u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230084u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2230088u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230092u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2230096u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230100u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2230104u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2230108u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2230112u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2230116u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2230120u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2230124u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2230128u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2230132u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2230136u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2230140u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2230144u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2230148u32);
    emu.sri_no_count(5usize, 5usize, 24u32, 2230152u32);
    emu.adi_no_count(6usize, 5usize, 32u32, 2230156u32);
    emu.add_memory_rw_events(24usize);
    let return_addr = 2230160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002207ec));
}
#[inline]
pub fn block_0x00220790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(29usize, 16usize, 1u32, 2230164u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2230168u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2230172u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230176u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2230180u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230184u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2230188u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230192u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2230196u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2230200u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2230204u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2230208u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2230212u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2230216u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2230220u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2230224u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2230228u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2230232u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2230236u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2230240u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2230244u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2230248u32);
    emu.sri_no_count(6usize, 5usize, 24u32, 2230252u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2230252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002207ec));
}
#[inline]
pub fn block_0x002207ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 14usize, 6usize, 2230256u32);
    emu.adi_no_count(14usize, 0usize, 4294967200u32, 2230260u32);
    emu.adi_no_count(7usize, 0usize, 80u32, 2230264u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2230268u32;
    emu.update_insn_clock();
    emu.sbr_no_count(14usize, 14usize, 5usize, 2230272u32);
    emu.adi_no_count(28usize, 28usize, 4294965651u32, 2230276u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2230280u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2230284u32);
    emu.adi_no_count(14usize, 14usize, 1087u32, 2230288u32);
    emu.mul_no_count(14usize, 14usize, 7usize, 2230292u32);
    emu.mulh_no_count(14usize, 14usize, 28usize, 2230296u32);
    emu.sri_no_count(28usize, 14usize, 31u32, 2230300u32);
    emu.sai_no_count(14usize, 14usize, 1034u32, 2230304u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2230308u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2231732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220db4));
    } else {
        emu.pc = 2230312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220828));
    }
}
#[inline(always)]
pub fn block_0x00220828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 6usize, 4294967264u32, 2230316u32);
    emu.slr_no_count(17usize, 17usize, 6usize, 2230320u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2230332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022083c));
    } else {
        emu.pc = 2230324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220834));
    }
}
#[inline(always)]
pub fn block_0x00220834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 17usize, 0u32, 2230328u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2230332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022084c));
}
#[inline(always)]
pub fn block_0x0022083c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(28usize, 6usize, 4294967295u32, 2230336u32);
    emu.srr_no_count(11usize, 11usize, 28usize, 2230340u32);
    emu.slr_no_count(16usize, 16usize, 6usize, 2230344u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2230348u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2230348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022084c));
}
#[inline(never)]
pub fn block_0x0022084c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(16usize, 7usize, 1055u32, 2230352u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2230356u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2230360u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 4294966248u32, 2230364u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2230368u32);
    emu.lw_no_count(6usize, 14usize, 0u32, 2230372u32)?;
    emu.lw_no_count(7usize, 14usize, 4u32, 2230376u32)?;
    emu.anr_no_count(16usize, 16usize, 17usize, 2230380u32);
    emu.lh_no_count(17usize, 14usize, 8u32, 2230384u32)?;
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2230388u32);
    emu.mul_no_count(29usize, 7usize, 16usize, 2230392u32);
    emu.mulhu_no_count(30usize, 7usize, 16usize, 2230396u32);
    emu.mul_no_count(31usize, 6usize, 11usize, 2230400u32);
    emu.mulhu_no_count(6usize, 6usize, 11usize, 2230404u32);
    emu.mul_no_count(8usize, 7usize, 11usize, 2230408u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2230412u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2230416u32);
    emu.adi_no_count(16usize, 0usize, 4294967232u32, 2230420u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2230424u32);
    emu.sbr_no_count(5usize, 16usize, 17usize, 2230428u32);
    emu.sbr_no_count(16usize, 0usize, 17usize, 2230432u32);
    emu.sltru_no_count(17usize, 28usize, 29usize, 2230436u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2230440u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2230444u32);
    emu.ani_no_count(29usize, 5usize, 63u32, 2230448u32);
    emu.sltru_no_count(7usize, 28usize, 31usize, 2230452u32);
    emu.sri_no_count(30usize, 28usize, 31u32, 2230456u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2230460u32);
    emu.adi_no_count(28usize, 29usize, 4294967264u32, 2230464u32);
    emu.adr_no_count(6usize, 17usize, 6usize, 2230468u32);
    emu.sltru_no_count(17usize, 6usize, 17usize, 2230472u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2230476u32);
    emu.sltru_no_count(7usize, 6usize, 8usize, 2230480u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2230484u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2230488u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2230492u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2230496u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2230500u32);
    emu.xri_no_count(8usize, 29usize, 4294967295u32, 2230504u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2230516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002208f4));
    } else {
        emu.pc = 2230508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002208ec));
    }
}
#[inline(always)]
pub fn block_0x002208ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(11usize, 7usize, 29usize, 2230512u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2230516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220904));
}
#[inline(always)]
pub fn block_0x002208f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 7usize, 1u32, 2230520u32);
    emu.slr_no_count(11usize, 11usize, 8usize, 2230524u32);
    emu.srr_no_count(17usize, 6usize, 16usize, 2230528u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2230532u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2230532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220904));
}
#[inline]
pub fn block_0x00220904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(14usize, 14usize, 10u32, 2230536u32)?;
    emu.slti_no_count(17usize, 28usize, 0u32, 2230540u32);
    emu.adi_no_count(30usize, 0usize, 1u32, 2230544u32);
    emu.slr_no_count(31usize, 30usize, 29usize, 2230548u32);
    emu.adi_no_count(9usize, 17usize, 4294967295u32, 2230552u32);
    emu.sbr_no_count(17usize, 0usize, 17usize, 2230556u32);
    emu.slr_no_count(16usize, 30usize, 16usize, 2230560u32);
    emu.anr_no_count(31usize, 9usize, 31usize, 2230564u32);
    emu.anr_no_count(30usize, 17usize, 16usize, 2230568u32);
    emu.sltiu_no_count(16usize, 30usize, 1u32, 2230572u32);
    emu.adi_no_count(9usize, 30usize, 4294967295u32, 2230576u32);
    emu.sbr_no_count(18usize, 31usize, 16usize, 2230580u32);
    emu.anr_no_count(17usize, 18usize, 7usize, 2230584u32);
    emu.anr_no_count(16usize, 9usize, 6usize, 2230588u32);
    emu.orr_no_count(19usize, 16usize, 17usize, 2230592u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2230660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220984));
    } else {
        emu.pc = 2230596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220944));
    }
}
#[inline(always)]
pub fn block_0x00220944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(19usize, 11usize, 4u32, 2230600u32);
    emu.adi_no_count(20usize, 0usize, 625u32, 2230604u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2230700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209ac));
    } else {
        emu.pc = 2230608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220950));
    }
}
#[inline(always)]
pub fn block_0x00220950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 100u32, 2230612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2230780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209fc));
    } else {
        emu.pc = 2230616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220958));
    }
}
#[inline(always)]
pub fn block_0x00220958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 9u32, 2230620u32);
    emu.sltiu_no_count(20usize, 11usize, 10u32, 2230624u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2231428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c84));
    } else {
        emu.pc = 2230628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220964));
    }
}
#[inline(always)]
pub fn block_0x00220964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2230632u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2230636u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2230640u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2230644u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2230648u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2230652u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2230820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a24));
    } else {
        emu.pc = 2230656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220980));
    }
}
#[inline(always)]
pub fn block_0x00220980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2230660u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002209e8));
}
#[inline(always)]
pub fn block_0x00220984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2230664u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2230692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209a4));
    } else {
        emu.pc = 2230668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022098c));
    }
}
#[inline(always)]
pub fn block_0x0022098c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(19usize, 13usize, 2u32, 2230672u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2230676u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 840u32, 2230680u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2230684u32);
    emu.lw_no_count(19usize, 19usize, 4294967292u32, 2230688u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2230596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220944));
    } else {
        emu.pc = 2230692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209a4));
    }
}
#[inline(always)]
pub fn block_0x002209a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2230696u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2230700u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220cf8));
}
#[inline(always)]
pub fn block_0x002209ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2230704u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2230708u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2230932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a94));
    } else {
        emu.pc = 2230712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209b8));
    }
}
#[inline(always)]
pub fn block_0x002209b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2230716u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1696u32, 2230720u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2230724u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2230736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209d0));
    } else {
        emu.pc = 2230728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209c8));
    }
}
#[inline(always)]
pub fn block_0x002209c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2230732u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1808u32, 2230736u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2230736u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002209d0));
}
#[inline(always)]
pub fn block_0x002209d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 5u32, 2230740u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2230744u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2230748u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2230752u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2230756u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2230820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a24));
    } else {
        emu.pc = 2230760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209e8));
    }
}
#[inline(always)]
pub fn block_0x002209e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(7usize, 14usize, 15usize, 2230764u32);
    emu.sli_no_count(6usize, 5usize, 16u32, 2230768u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2230996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ad4));
    } else {
        emu.pc = 2230772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209f4));
    }
}
#[inline(always)]
pub fn block_0x002209f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 13usize, 0u32, 2230776u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2230780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220adc));
}
#[inline(always)]
pub fn block_0x002209fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 1000u32, 2230784u32);
    emu.sltiu_no_count(20usize, 11usize, 1000u32, 2230788u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2230796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a0c));
    } else {
        emu.pc = 2230792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a08));
    }
}
#[inline(always)]
pub fn block_0x00220a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1000u32, 2230796u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2230796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220a0c));
}
#[inline(always)]
pub fn block_0x00220a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 3u32, 2230800u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2230804u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2230808u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2230812u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2230816u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2230760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209e8));
    } else {
        emu.pc = 2230820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a24));
    }
}
#[inline(never)]
pub fn block_0x00220a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 6usize, 1u32, 2230824u32);
    emu.sli_no_count(16usize, 7usize, 31u32, 2230828u32);
    emu.sri_no_count(17usize, 7usize, 1u32, 2230832u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2230836u32;
    emu.update_insn_clock();
    emu.orr_no_count(11usize, 11usize, 16usize, 2230840u32);
    emu.adi_no_count(6usize, 5usize, 4294966477u32, 2230844u32);
    emu.adi_no_count(16usize, 5usize, 4294966476u32, 2230848u32);
    emu.adr_no_count(5usize, 11usize, 17usize, 2230852u32);
    emu.sltru_no_count(7usize, 5usize, 11usize, 2230856u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2230860u32);
    emu.mulhu_no_count(7usize, 5usize, 6usize, 2230864u32);
    emu.sri_no_count(9usize, 7usize, 2u32, 2230868u32);
    emu.ani_no_count(7usize, 7usize, 4294967292u32, 2230872u32);
    emu.adr_no_count(7usize, 7usize, 9usize, 2230876u32);
    emu.sbr_no_count(5usize, 5usize, 7usize, 2230880u32);
    emu.sbr_no_count(7usize, 11usize, 5usize, 2230884u32);
    emu.sltru_no_count(11usize, 11usize, 5usize, 2230888u32);
    emu.mul_no_count(5usize, 7usize, 16usize, 2230892u32);
    emu.mulhu_no_count(9usize, 7usize, 6usize, 2230896u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2230900u32);
    emu.mul_no_count(16usize, 7usize, 6usize, 2230904u32);
    emu.adr_no_count(17usize, 9usize, 5usize, 2230908u32);
    emu.mul_no_count(11usize, 11usize, 6usize, 2230912u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2230916u32);
    emu.slr_no_count(11usize, 19usize, 29usize, 2230920u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2231132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b5c));
    } else {
        emu.pc = 2230924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a8c));
    }
}
#[inline(always)]
pub fn block_0x00220a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 11usize, 0u32, 2230928u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2230932u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220b64));
}
#[inline(always)]
pub fn block_0x00220a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2230936u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 256u32, 2230940u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2231460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ca4));
    } else {
        emu.pc = 2230944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220aa0));
    }
}
#[inline(always)]
pub fn block_0x00220aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2230948u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1664u32, 2230952u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2230956u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2230968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ab8));
    } else {
        emu.pc = 2230960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ab0));
    }
}
#[inline(always)]
pub fn block_0x00220ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2230964u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2230968u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2230968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220ab8));
}
#[inline(always)]
pub fn block_0x00220ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 7u32, 2230972u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2230976u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2230980u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2230984u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2230988u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2230760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209e8));
    } else {
        emu.pc = 2230992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ad0));
    }
}
#[inline(always)]
pub fn block_0x00220ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2230996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220a24));
}
#[inline(always)]
pub fn block_0x00220ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(7usize, 7usize, 16u32, 2231000u32);
    emu.sai_no_count(5usize, 7usize, 1040u32, 2231004u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2231004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220adc));
}
#[inline(always)]
pub fn block_0x00220adc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(7usize, 6usize, 16u32, 2231008u32);
    emu.adi_no_count(22usize, 0usize, 4294967295u32, 2231012u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2231016u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 0usize, 10u32, 2231020u32);
    emu.adi_no_count(24usize, 6usize, 4294966477u32, 2231024u32);
    emu.adi_no_count(21usize, 0usize, 4294967295u32, 2231028u32);
    emu.adi_no_count(25usize, 12usize, 0u32, 2231032u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2231032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220af8));
}
#[inline(always)]
pub fn block_0x00220af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 13usize, 21usize, 2231036u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2231684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220d84));
    } else {
        emu.pc = 2231040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b00));
    }
}
#[inline(always)]
pub fn block_0x00220b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 19usize, 0u32, 2231044u32);
    emu.divu_no_count(19usize, 11usize, 19usize, 2231048u32);
    emu.mul_no_count(26usize, 19usize, 6usize, 2231052u32);
    emu.sbr_no_count(11usize, 11usize, 26usize, 2231056u32);
    emu.adr_no_count(26usize, 5usize, 21usize, 2231060u32);
    emu.adi_no_count(19usize, 19usize, 48u32, 2231064u32);
    emu.sb_no_count(19usize, 25usize, 0u32, 2231068u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2231116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b4c));
    } else {
        emu.pc = 2231072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b20));
    }
}
#[inline(always)]
pub fn block_0x00220b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 20usize, 21usize, 2231076u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2231164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b7c));
    } else {
        emu.pc = 2231080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b28));
    }
}
#[inline(always)]
pub fn block_0x00220b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(19usize, 6usize, 24usize, 2231084u32);
    emu.adi_no_count(25usize, 25usize, 1u32, 2231088u32);
    emu.sri_no_count(19usize, 19usize, 3u32, 2231092u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2231096u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2231032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220af8));
    } else {
        emu.pc = 2231100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b3c));
    }
}
#[inline(always)]
pub fn block_0x00220b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2231104u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 880u32, 2231108u32);
    emu.apc_no_count(1usize, 2231108u32, 0u32, 2231112u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231116u32;
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
pub fn block_0x00220b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(7usize, 11usize, 29usize, 2231120u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2231352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c38));
    } else {
        emu.pc = 2231124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220b54));
    }
}
#[inline(always)]
pub fn block_0x00220b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 7usize, 0u32, 2231128u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2231132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220c40));
}
#[inline(always)]
pub fn block_0x00220b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 19usize, 1u32, 2231136u32);
    emu.srr_no_count(29usize, 5usize, 8usize, 2231140u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2231140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220b64));
}
#[inline(always)]
pub fn block_0x00220b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 28usize, 1055u32, 2231144u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2231148u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2231152u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2231156u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2231160u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2231164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220cf0));
}
#[inline]
pub fn block_0x00220b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2231168u32);
    emu.adi_no_count(19usize, 7usize, 4294967295u32, 2231172u32);
    emu.sbr_no_count(11usize, 0usize, 21usize, 2231176u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2231180u32);
    emu.ani_no_count(19usize, 19usize, 63u32, 2231184u32);
    emu.xri_no_count(20usize, 19usize, 4294967295u32, 2231188u32);
    emu.adi_no_count(21usize, 19usize, 4294967264u32, 2231192u32);
    emu.sai_no_count(22usize, 21usize, 1055u32, 2231196u32);
    emu.adi_no_count(23usize, 0usize, 10u32, 2231200u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2231204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220bd4));
}
#[inline(always)]
pub fn block_0x00220ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 17usize, 29usize, 2231208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2231208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220ba8));
}
#[inline]
pub fn block_0x00220ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(17usize, 17usize, 18usize, 2231212u32);
    emu.anr_no_count(16usize, 16usize, 9usize, 2231216u32);
    emu.mulhu_no_count(25usize, 7usize, 23usize, 2231220u32);
    emu.mul_no_count(6usize, 6usize, 23usize, 2231224u32);
    emu.mul_no_count(7usize, 7usize, 23usize, 2231228u32);
    emu.adi_no_count(24usize, 24usize, 48u32, 2231232u32);
    emu.adr_no_count(6usize, 25usize, 6usize, 2231236u32);
    emu.adr_no_count(25usize, 12usize, 11usize, 2231240u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2231244u32);
    emu.sb_no_count(24usize, 25usize, 0u32, 2231248u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2231508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220cd4));
    } else {
        emu.pc = 2231252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220bd4));
    }
}
#[inline(always)]
pub fn block_0x00220bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 6usize, 19usize, 2231256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2231280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220bf0));
    } else {
        emu.pc = 2231260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220bdc));
    }
}
#[inline(always)]
pub fn block_0x00220bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2231264u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2231268u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2231272u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2231308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c0c));
    } else {
        emu.pc = 2231276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220bec));
    }
}
#[inline(always)]
pub fn block_0x00220bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2231280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002209a4));
}
#[inline(always)]
pub fn block_0x00220bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(25usize, 7usize, 19usize, 2231284u32);
    emu.sli_no_count(26usize, 6usize, 1u32, 2231288u32);
    emu.slr_no_count(26usize, 26usize, 20usize, 2231292u32);
    emu.orr_no_count(25usize, 25usize, 26usize, 2231296u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2231300u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2231304u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2230692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209a4));
    } else {
        emu.pc = 2231308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c0c));
    }
}
#[inline(always)]
pub fn block_0x00220c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2231708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220d9c));
    } else {
        emu.pc = 2231312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c10));
    }
}
#[inline(always)]
pub fn block_0x00220c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(24usize, 16usize, 23usize, 2231316u32);
    emu.mul_no_count(17usize, 17usize, 23usize, 2231320u32);
    emu.adr_no_count(17usize, 24usize, 17usize, 2231324u32);
    emu.mul_no_count(16usize, 16usize, 23usize, 2231328u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2231204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ba4));
    } else {
        emu.pc = 2231332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c24));
    }
}
#[inline(always)]
pub fn block_0x00220c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 17usize, 1u32, 2231336u32);
    emu.slr_no_count(24usize, 24usize, 8usize, 2231340u32);
    emu.srr_no_count(25usize, 16usize, 29usize, 2231344u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2231348u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2231352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220ba8));
}
#[inline(always)]
pub fn block_0x00220c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 11usize, 1u32, 2231356u32);
    emu.srr_no_count(9usize, 11usize, 8usize, 2231360u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2231360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220c40));
}
#[inline(always)]
pub fn block_0x00220c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(11usize, 28usize, 1055u32, 2231364u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2231368u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2231372u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2231376u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2231380u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2231384u32);
    emu.slr_no_count(7usize, 6usize, 29usize, 2231388u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2231400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c68));
    } else {
        emu.pc = 2231392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220c60));
    }
}
#[inline(always)]
pub fn block_0x00220c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 7usize, 0u32, 2231396u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2231400u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231408u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220c70));
}
#[inline(always)]
pub fn block_0x00220c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 6usize, 1u32, 2231404u32);
    emu.srr_no_count(29usize, 6usize, 8usize, 2231408u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2231408u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220c70));
}
#[inline(always)]
pub fn block_0x00220c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 11usize, 7usize, 2231412u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2231416u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2231420u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2231424u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2231428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2231536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220cf0));
}
#[inline(always)]
pub fn block_0x00220c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2231432u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2231436u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2231440u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2231444u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2231448u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2231452u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2230820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220a24));
    } else {
        emu.pc = 2231456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ca0));
    }
}
#[inline(always)]
pub fn block_0x00220ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2231460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002209e8));
}
#[inline(always)]
pub fn block_0x00220ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2231464u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 20usize, 4294965760u32, 2231468u32);
    emu.sltru_no_count(20usize, 11usize, 21usize, 2231472u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2231480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220cb8));
    } else {
        emu.pc = 2231476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220cb4));
    }
}
#[inline(always)]
pub fn block_0x00220cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 0u32, 2231480u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2231480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220cb8));
}
#[inline(always)]
pub fn block_0x00220cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 9u32, 2231484u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2231488u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2231492u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2231496u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2231500u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2230760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002209e8));
    } else {
        emu.pc = 2231504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220cd0));
    }
}
#[inline(always)]
pub fn block_0x00220cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2231508u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2230820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220a24));
}
#[inline(always)]
pub fn block_0x00220cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2231512u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2231516u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2231520u32);
    emu.adi_no_count(28usize, 30usize, 0u32, 2231524u32);
    emu.adi_no_count(29usize, 31usize, 0u32, 2231528u32);
    emu.adi_no_count(30usize, 7usize, 0u32, 2231532u32);
    emu.adi_no_count(31usize, 6usize, 0u32, 2231536u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2231536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220cf0));
}
#[inline(always)]
pub fn block_0x00220cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2231536u32, 0u32, 2231540u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231544u32;
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
pub fn block_0x00220cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2231548u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2231552u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2231556u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2231560u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2231564u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2231568u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2231572u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2231576u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2231580u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2231584u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2231588u32)?;
    emu.lw_no_count(26usize, 2usize, 0u32, 2231592u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2231596u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231600u32;
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
pub fn block_0x00220d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2231604u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 264u32, 2231608u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2231612u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 756u32, 2231616u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2231620u32);
    emu.apc_no_count(1usize, 2231620u32, 4294955008u32, 2231624u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231628u32;
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
pub fn block_0x00220d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2231632u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 772u32, 2231636u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2231640u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 808u32, 2231644u32);
    emu.adi_no_count(11usize, 0usize, 36u32, 2231648u32);
    emu.apc_no_count(1usize, 2231648u32, 4294955008u32, 2231652u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231656u32;
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
pub fn block_0x00220d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2231660u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 720u32, 2231664u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2231668u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 824u32, 2231672u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2231676u32);
    emu.apc_no_count(1usize, 2231676u32, 4294955008u32, 2231680u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231684u32;
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
pub fn block_0x00220d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2231688u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 896u32, 2231692u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2231696u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2231700u32);
    emu.apc_no_count(1usize, 2231700u32, 4294955008u32, 2231704u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231708u32;
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
pub fn block_0x00220d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2231712u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 912u32, 2231716u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2231720u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2231724u32);
    emu.apc_no_count(1usize, 2231724u32, 4294955008u32, 2231728u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231732u32;
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
pub fn block_0x00220db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2231736u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 248u32, 2231740u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2231744u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2231748u32);
    emu.apc_no_count(1usize, 2231748u32, 4294955008u32, 2231752u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231756u32;
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
pub fn block_0x00220dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2231760u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2231764u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2231768u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2231772u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2231776u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2231780u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2231784u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2231788u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2231792u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2231796u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2231812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e04));
    } else {
        emu.pc = 2231800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220df8));
    }
}
#[inline(always)]
pub fn block_0x00220df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 29usize, 2231804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2231820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e0c));
    } else {
        emu.pc = 2231808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e00));
    }
}
#[inline(always)]
pub fn block_0x00220e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2231812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232224u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220fa0));
}
#[inline(always)]
pub fn block_0x00220e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 28usize, 2231816u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2232224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fa0));
    } else {
        emu.pc = 2231820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e0c));
    }
}
#[inline(always)]
pub fn block_0x00220e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 30usize, 2231824u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2231828u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2231832u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2231848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e28));
    } else {
        emu.pc = 2231836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e1c));
    }
}
#[inline(always)]
pub fn block_0x00220e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 5usize, 2231840u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2231860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e34));
    } else {
        emu.pc = 2231844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e24));
    }
}
#[inline(always)]
pub fn block_0x00220e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2231848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232224u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220fa0));
}
#[inline(always)]
pub fn block_0x00220e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 30usize, 2231852u32);
    emu.sltru_no_count(5usize, 30usize, 5usize, 2231856u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2232224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fa0));
    } else {
        emu.pc = 2231860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e34));
    }
}
#[inline(always)]
pub fn block_0x00220e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 16usize, 2231864u32);
    emu.sbr_no_count(6usize, 29usize, 17usize, 2231868u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2231872u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2231888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e50));
    } else {
        emu.pc = 2231876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e44));
    }
}
#[inline(always)]
pub fn block_0x00220e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 5usize, 2231880u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2231900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e5c));
    } else {
        emu.pc = 2231884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e4c));
    }
}
#[inline(always)]
pub fn block_0x00220e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2231888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232000u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220ec0));
}
#[inline(always)]
pub fn block_0x00220e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 16usize, 2231892u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2231896u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2232000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ec0));
    } else {
        emu.pc = 2231900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e5c));
    }
}
#[inline]
pub fn block_0x00220e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 16usize, 31u32, 2231904u32);
    emu.sli_no_count(7usize, 17usize, 1u32, 2231908u32);
    emu.sli_no_count(5usize, 16usize, 1u32, 2231912u32);
    emu.sri_no_count(8usize, 30usize, 31u32, 2231916u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2231920u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2231924u32);
    emu.sbr_no_count(6usize, 29usize, 6usize, 2231928u32);
    emu.sbr_no_count(6usize, 6usize, 7usize, 2231932u32);
    emu.sli_no_count(7usize, 31usize, 1u32, 2231936u32);
    emu.orr_no_count(7usize, 7usize, 8usize, 2231940u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2231984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220eb0));
    } else {
        emu.pc = 2231944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e88));
    }
}
#[inline(always)]
pub fn block_0x00220e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 7usize, 2231948u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2232000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ec0));
    } else {
        emu.pc = 2231952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e90));
    }
}
#[inline(always)]
pub fn block_0x00220e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2232416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221060));
    } else {
        emu.pc = 2231956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e94));
    }
}
#[inline(always)]
pub fn block_0x00220e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2231960u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 960u32, 2231964u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2231968u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2231972u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2231976u32);
    emu.apc_no_count(1usize, 2231976u32, 16384u32, 2231980u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2231984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00220eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 5usize, 2231988u32);
    emu.sli_no_count(6usize, 30usize, 1u32, 2231992u32);
    emu.sltru_no_count(5usize, 5usize, 6usize, 2231996u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2231952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220e90));
    } else {
        emu.pc = 2232000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ec0));
    }
}
#[inline(always)]
pub fn block_0x00220ec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2232016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ed0));
    } else {
        emu.pc = 2232004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ec4));
    }
}
#[inline(always)]
pub fn block_0x00220ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 17usize, 2232008u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2232024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ed8));
    } else {
        emu.pc = 2232012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ecc));
    }
}
#[inline(always)]
pub fn block_0x00220ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2232016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232224u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220fa0));
}
#[inline(always)]
pub fn block_0x00220ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 16usize, 2232020u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2232224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fa0));
    } else {
        emu.pc = 2232024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ed8));
    }
}
#[inline(always)]
pub fn block_0x00220ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 30usize, 2232028u32);
    emu.sbr_no_count(17usize, 17usize, 31usize, 2232032u32);
    emu.sbr_no_count(16usize, 16usize, 30usize, 2232036u32);
    emu.sbr_no_count(17usize, 17usize, 5usize, 2232040u32);
    emu.sbr_no_count(5usize, 29usize, 17usize, 2232044u32);
    emu.sltru_no_count(6usize, 28usize, 16usize, 2232048u32);
    emu.sbr_no_count(5usize, 5usize, 6usize, 2232052u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2232212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f94));
    } else {
        emu.pc = 2232056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220ef8));
    }
}
#[inline(always)]
pub fn block_0x00220ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 17usize, 5usize, 2232060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2232224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fa0));
    } else {
        emu.pc = 2232064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f00));
    }
}
#[inline(always)]
pub fn block_0x00220f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2232432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221070));
    } else {
        emu.pc = 2232068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f04));
    }
}
#[inline(always)]
pub fn block_0x00220f04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2232072u32);
    emu.adr_no_count(8usize, 11usize, 13usize, 2232076u32);
    emu.adi_no_count(17usize, 0usize, 57u32, 2232080u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2232080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220f10));
}
#[inline(always)]
pub fn block_0x00220f10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 13usize, 16usize, 2232084u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2232272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fd0));
    } else {
        emu.pc = 2232088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f18));
    }
}
#[inline(always)]
pub fn block_0x00220f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 8usize, 16usize, 2232092u32);
    emu.lbu_no_count(5usize, 5usize, 4294967295u32, 2232096u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2232100u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2232080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f10));
    } else {
        emu.pc = 2232104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f28));
    }
}
#[inline(always)]
pub fn block_0x00220f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 8usize, 16usize, 2232108u32);
    emu.lbu_no_count(15usize, 8usize, 0u32, 2232112u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2232116u32);
    emu.adi_no_count(5usize, 15usize, 1u32, 2232120u32);
    emu.adi_no_count(15usize, 17usize, 1u32, 2232124u32);
    emu.sb_no_count(5usize, 8usize, 0u32, 2232128u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2232488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002210a8));
    } else {
        emu.pc = 2232132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f44));
    }
}
#[inline(always)]
pub fn block_0x00220f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2232136u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2232412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022105c));
    } else {
        emu.pc = 2232140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f4c));
    }
}
#[inline]
pub fn block_0x00220f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2232144u32);
    emu.adi_no_count(16usize, 8usize, 1u32, 2232148u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2232152u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2232156u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2232160u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2232164u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2232168u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2232172u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2232176u32);
    emu.adi_no_count(20usize, 14usize, 0u32, 2232180u32);
    emu.apc_no_count(1usize, 2232180u32, 4294873088u32, 2232184u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232188u32;
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
pub fn block_0x00220f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2232192u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2232196u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2232200u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2232204u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2232208u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2232212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022105c));
}
#[inline(always)]
pub fn block_0x00220f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(17usize, 28usize, 16usize, 2232216u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2232220u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2232064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220f00));
    } else {
        emu.pc = 2232224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fa0));
    }
}
#[inline(always)]
pub fn block_0x00220fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2232228u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2232228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220fa4));
}
#[inline]
pub fn block_0x00220fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2232232u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2232236u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2232240u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2232244u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2232248u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2232252u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2232256u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2232260u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2232264u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2232268u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232272u32;
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
pub fn block_0x00220fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2232372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221034));
    } else {
        emu.pc = 2232276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fd4));
    }
}
#[inline(always)]
pub fn block_0x00220fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 49u32, 2232280u32);
    emu.adi_no_count(16usize, 13usize, 4294967295u32, 2232284u32);
    emu.sb_no_count(17usize, 11usize, 0u32, 2232288u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2232380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022103c));
    } else {
        emu.pc = 2232292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220fe4));
    }
}
#[inline]
pub fn block_0x00220fe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 11usize, 1u32, 2232296u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2232300u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2232304u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2232308u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2232312u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2232316u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2232320u32);
    emu.adi_no_count(12usize, 16usize, 0u32, 2232324u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2232328u32);
    emu.adi_no_count(23usize, 14usize, 0u32, 2232332u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2232336u32);
    emu.apc_no_count(1usize, 2232336u32, 4294873088u32, 2232340u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00221018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 22usize, 0u32, 2232348u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2232352u32);
    emu.adi_no_count(14usize, 23usize, 0u32, 2232356u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2232360u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2232364u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2232368u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2232372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232384u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221040));
}
#[inline(always)]
pub fn block_0x00221034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2232376u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2232380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232384u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221040));
}
#[inline(always)]
pub fn block_0x0022103c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2232384u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2232384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221040));
}
#[inline(always)]
pub fn block_0x00221040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 1u32, 2232388u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2232392u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2232396u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2232412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022105c));
    } else {
        emu.pc = 2232400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221050));
    }
}
#[inline(always)]
pub fn block_0x00221050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2232412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022105c));
    } else {
        emu.pc = 2232404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221054));
    }
}
#[inline(always)]
pub fn block_0x00221054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2232408u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2232412u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2232412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022105c));
}
#[inline(always)]
pub fn block_0x0022105c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2232460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022108c));
    } else {
        emu.pc = 2232416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221060));
    }
}
#[inline(always)]
pub fn block_0x00221060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2232420u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2232424u32)?;
    emu.sh_no_count(14usize, 10usize, 8u32, 2232428u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2232432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2232228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220fa4));
}
#[inline(always)]
pub fn block_0x00221070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2232436u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 928u32, 2232440u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2232444u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2232448u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2232452u32);
    emu.apc_no_count(1usize, 2232452u32, 16384u32, 2232456u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232460u32;
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
pub fn block_0x0022108c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2232464u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 944u32, 2232468u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2232472u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2232476u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2232480u32);
    emu.apc_no_count(1usize, 2232480u32, 16384u32, 2232484u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232488u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002210a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2232492u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1776u32, 2232496u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2232500u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2232504u32);
    emu.apc_no_count(1usize, 2232504u32, 16384u32, 2232508u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2232512u32;
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
