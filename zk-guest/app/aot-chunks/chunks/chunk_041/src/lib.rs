pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2249964u32;
pub const PC_MAX: u32 = 2252328u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x002254ec,
        block_0x00225508,
        block_0x00225520,
        block_0x00225538,
        block_0x0022554c,
        block_0x00225564,
        block_0x0022557c,
        block_0x00225590,
        block_0x00225598,
        block_0x002255a0,
        block_0x002255a8,
        block_0x002255bc,
        block_0x002255d0,
        block_0x002255d8,
        block_0x002255dc,
        block_0x002255f4,
        block_0x002255f8,
        block_0x00225604,
        block_0x00225610,
        block_0x00225618,
        block_0x0022561c,
        block_0x00225634,
        block_0x0022563c,
        block_0x00225644,
        block_0x00225660,
        block_0x0022567c,
        block_0x002256b0,
        block_0x002256b4,
        block_0x002256c8,
        block_0x002256cc,
        block_0x002256d4,
        block_0x002256e0,
        block_0x0022575c,
        block_0x00225760,
        block_0x00225784,
        block_0x002257ac,
        block_0x002257d0,
        block_0x00225830,
        block_0x00225890,
        block_0x002258f0,
        block_0x00225954,
        block_0x00225978,
        block_0x0022598c,
        block_0x00225990,
        block_0x00225994,
        block_0x00225998,
        block_0x0022599c,
        block_0x002259b0,
        block_0x002259c4,
        block_0x002259dc,
        block_0x002259f4,
        block_0x002259f8,
        block_0x00225a14,
        block_0x00225a24,
        block_0x00225a3c,
        block_0x00225a40,
        block_0x00225a4c,
        block_0x00225a50,
        block_0x00225a5c,
        block_0x00225a60,
        block_0x00225a74,
        block_0x00225a88,
        block_0x00225aa0,
        block_0x00225ab8,
        block_0x00225abc,
        block_0x00225afc,
        block_0x00225b10,
        block_0x00225b20,
        block_0x00225b28,
        block_0x00225b34,
        block_0x00225b38,
        block_0x00225b44,
        block_0x00225b4c,
        block_0x00225b60,
        block_0x00225b74,
        block_0x00225b94,
        block_0x00225b98,
        block_0x00225bcc,
        block_0x00225be4,
        block_0x00225c00,
        block_0x00225c1c,
        block_0x00225c20,
        block_0x00225c38,
        block_0x00225c54,
        block_0x00225c70,
        block_0x00225c74,
        block_0x00225c88,
        block_0x00225c8c,
        block_0x00225ca4,
        block_0x00225cb8,
        block_0x00225cc8,
        block_0x00225cd4,
        block_0x00225cdc,
        block_0x00225d00,
        block_0x00225d14,
        block_0x00225d6c,
        block_0x00225d70,
        block_0x00225d78,
        block_0x00225d7c,
        block_0x00225d80,
        block_0x00225d94,
        block_0x00225dac,
        block_0x00225dc0,
        block_0x00225dd0,
        block_0x00225ddc,
        block_0x00225de4,
        block_0x00225e08,
        block_0x00225e0c,
        block_0x00225e18,
        block_0x00225e28,
    ];
    const IDX: [u16; 592usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16,
        0u16, 0u16, 8u16, 0u16, 9u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16,
        12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 16u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 20u16, 21u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 0u16, 24u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 28u16, 0u16,
        0u16, 0u16, 0u16, 29u16, 30u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16,
        45u16, 46u16, 47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16,
        49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16,
        52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 55u16, 56u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16,
        59u16, 60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 65u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16,
        0u16, 0u16, 70u16, 71u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16,
        74u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        76u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 91u16,
        0u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 96u16, 97u16, 0u16, 98u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16, 101u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16,
        0u16, 104u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 107u16, 108u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16,
    ];
    if pc < 2249964u32 || pc > 2252328u32 {
        return None;
    }
    let word_offset = ((pc - 2249964u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002254ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2249968u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1644u32, 2249972u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249976u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2249980u32);
    emu.adi_no_count(11usize, 0usize, 26u32, 2249984u32);
    emu.apc_no_count(1usize, 2249984u32, 4294934528u32, 2249988u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249992u32;
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
pub fn block_0x00225508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249996u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966952u32, 2250000u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2250004u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2250008u32);
    emu.apc_no_count(1usize, 2250008u32, 0u32, 2250012u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250016u32;
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
pub fn block_0x00225520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2250020u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966904u32, 2250024u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2250028u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2250032u32);
    emu.apc_no_count(1usize, 2250032u32, 0u32, 2250036u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250040u32;
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
pub fn block_0x00225538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2250044u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1776u32, 2250048u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2250052u32);
    emu.apc_no_count(1usize, 2250052u32, 0u32, 2250056u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(76u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0022554c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2250064u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2250068u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2250072u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2250076u32);
    emu.apc_no_count(1usize, 2250076u32, 4294934528u32, 2250080u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2250088u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966936u32, 2250092u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2250096u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2250100u32);
    emu.apc_no_count(1usize, 2250100u32, 4294934528u32, 2250104u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0022557c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2250112u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966888u32, 2250116u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2250120u32);
    emu.apc_no_count(1usize, 2250120u32, 4294934528u32, 2250124u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2250128u32, 0u32, 2250132u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2250136u32, 0u32, 2250140u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002255a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2250144u32, 0u32, 2250148u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250152u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002255a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2250156u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2250160u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2250164u32);
    emu.apc_no_count(1usize, 2250164u32, 0u32, 2250168u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002255bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2250176u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2250180u32);
    emu.ani_no_count(10usize, 10usize, 4294967292u32, 2250184u32);
    emu.sbr_no_count(5usize, 10usize, 12usize, 2250188u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2250232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255f8));
    } else {
        emu.pc = 2250192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255d0));
    }
}
#[inline(always)]
pub fn block_0x002255d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2250196u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2250228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255f4));
    } else {
        emu.pc = 2250200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255d8));
    }
}
#[inline(always)]
pub fn block_0x002255d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 12usize, 11usize, 2250204u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2250204u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002255dc));
}
#[inline(always)]
pub fn block_0x002255dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2250208u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2250212u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2250216u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2250220u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2250224u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2250204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255dc));
    } else {
        emu.pc = 2250228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255f4));
    }
}
#[inline(always)]
pub fn block_0x002255f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250232u32;
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
pub fn block_0x002255f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 11usize, 5usize, 2250236u32);
    emu.sri_no_count(17usize, 13usize, 2u32, 2250240u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2250192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255d0));
    } else {
        emu.pc = 2250244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225604));
    }
}
#[inline(always)]
pub fn block_0x00225604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 12usize, 5usize, 2250248u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2250252u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2250264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225618));
    } else {
        emu.pc = 2250256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225610));
    }
}
#[inline(always)]
pub fn block_0x00225610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2250260u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2250264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2250292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225634));
}
#[inline(always)]
pub fn block_0x00225618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2250268u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2250268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022561c));
}
#[inline(always)]
pub fn block_0x0022561c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 12usize, 0u32, 2250272u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2250276u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2250280u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2250284u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2250288u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2250268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022561c));
    } else {
        emu.pc = 2250292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225634));
    }
}
#[inline(always)]
pub fn block_0x00225634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2250296u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2250336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225660));
    } else {
        emu.pc = 2250300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022563c));
    }
}
#[inline(always)]
pub fn block_0x0022563c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2250304u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2250308u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2250308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225644));
}
#[inline(always)]
pub fn block_0x00225644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2250312u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2250316u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2250320u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2250324u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2250328u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2250332u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2250308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225644));
    } else {
        emu.pc = 2250336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225660));
    }
}
#[inline(always)]
pub fn block_0x00225660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2250340u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16711680u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2250344u32;
    emu.update_insn_clock();
    emu.adr_no_count(10usize, 12usize, 10usize, 2250348u32);
    emu.adi_no_count(12usize, 11usize, 257u32, 2250352u32);
    emu.adi_no_count(11usize, 13usize, 255u32, 2250356u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2250360u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2250364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2250416u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002256b0));
}
#[inline]
pub fn block_0x0022567c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(6usize, 16usize, 2u32, 2250368u32);
    emu.sbr_no_count(17usize, 13usize, 16usize, 2250372u32);
    emu.ani_no_count(7usize, 16usize, 3u32, 2250376u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2250380u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2250384u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2250388u32);
    emu.anr_no_count(6usize, 29usize, 11usize, 2250392u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2250396u32);
    emu.sli_no_count(28usize, 6usize, 16u32, 2250400u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2250404u32);
    emu.sri_no_count(6usize, 6usize, 16u32, 2250408u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2250412u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2250592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225760));
    } else {
        emu.pc = 2250416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002256b0));
    }
}
#[inline(always)]
pub fn block_0x002256b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2250228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002255f4));
    } else {
        emu.pc = 2250420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002256b4));
    }
}
#[inline(always)]
pub fn block_0x002256b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 17usize, 0u32, 2250424u32);
    emu.adi_no_count(15usize, 5usize, 0u32, 2250428u32);
    emu.adi_no_count(17usize, 0usize, 192u32, 2250432u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2250436u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2250444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002256cc));
    } else {
        emu.pc = 2250440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002256c8));
    }
}
#[inline(always)]
pub fn block_0x002256c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 192u32, 2250444u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2250444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002256cc));
}
#[inline(always)]
pub fn block_0x002256cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2250448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2250364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022567c));
    } else {
        emu.pc = 2250452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002256d4));
    }
}
#[inline(always)]
pub fn block_0x002256d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 2u32, 2250456u32);
    emu.sli_no_count(17usize, 17usize, 4u32, 2250460u32);
    emu.adi_no_count(6usize, 15usize, 0u32, 2250464u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2250464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002256e0));
}
#[inline(never)]
pub fn block_0x002256e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 6usize, 0u32, 2250468u32)?;
    emu.lw_no_count(28usize, 6usize, 4u32, 2250472u32)?;
    emu.lw_no_count(29usize, 6usize, 8u32, 2250476u32)?;
    emu.lw_no_count(30usize, 6usize, 12u32, 2250480u32)?;
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2250484u32);
    emu.sri_no_count(7usize, 7usize, 6u32, 2250488u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2250492u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2250496u32);
    emu.xri_no_count(31usize, 28usize, 4294967295u32, 2250500u32);
    emu.sri_no_count(28usize, 28usize, 6u32, 2250504u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2250508u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2250512u32);
    emu.xri_no_count(31usize, 29usize, 4294967295u32, 2250516u32);
    emu.sri_no_count(29usize, 29usize, 6u32, 2250520u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2250524u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2250528u32);
    emu.xri_no_count(31usize, 30usize, 4294967295u32, 2250532u32);
    emu.sri_no_count(30usize, 30usize, 6u32, 2250536u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2250540u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2250544u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2250548u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2250552u32);
    emu.adi_no_count(17usize, 17usize, 4294967280u32, 2250556u32);
    emu.anr_no_count(7usize, 28usize, 12usize, 2250560u32);
    emu.anr_no_count(28usize, 29usize, 12usize, 2250564u32);
    emu.anr_no_count(29usize, 30usize, 12usize, 2250568u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2250572u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2250576u32);
    emu.adr_no_count(5usize, 5usize, 29usize, 2250580u32);
    emu.adi_no_count(6usize, 6usize, 16u32, 2250584u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2250464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002256e0));
    } else {
        emu.pc = 2250588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022575c));
    }
}
#[inline(always)]
pub fn block_0x0022575c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2250592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2250364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022567c));
}
#[inline]
pub fn block_0x00225760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2250596u32);
    emu.ani_no_count(16usize, 16usize, 252u32, 2250600u32);
    emu.sli_no_count(16usize, 16usize, 2u32, 2250604u32);
    emu.adr_no_count(15usize, 15usize, 16usize, 2250608u32);
    emu.sltiu_no_count(16usize, 13usize, 192u32, 2250612u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2250616u32);
    emu.anr_no_count(13usize, 13usize, 16usize, 2250620u32);
    emu.ani_no_count(13usize, 13usize, 3u32, 2250624u32);
    emu.sli_no_count(13usize, 13usize, 2u32, 2250628u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2250628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225784));
}
#[inline]
pub fn block_0x00225784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2250632u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2250636u32);
    emu.xri_no_count(17usize, 16usize, 4294967295u32, 2250640u32);
    emu.sri_no_count(16usize, 16usize, 6u32, 2250644u32);
    emu.sri_no_count(17usize, 17usize, 7u32, 2250648u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2250652u32);
    emu.anr_no_count(16usize, 16usize, 12usize, 2250656u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2250660u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2250664u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2250628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225784));
    } else {
        emu.pc = 2250668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002257ac));
    }
}
#[inline]
pub fn block_0x002257ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(12usize, 14usize, 11usize, 2250672u32);
    emu.sri_no_count(14usize, 14usize, 8u32, 2250676u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2250680u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2250684u32);
    emu.sli_no_count(12usize, 11usize, 16u32, 2250688u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2250692u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2250696u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2250700u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250704u32;
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
pub fn block_0x002257d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2250708u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2250712u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2250716u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2250720u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2250724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2250728u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2250732u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2250736u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967052u32, 2250740u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2250744u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2250748u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2250752u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2250756u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2250760u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2250764u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2250768u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2250772u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2250776u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2250780u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2250784u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2250788u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2250792u32);
    emu.apc_no_count(1usize, 2250792u32, 4294934528u32, 2250796u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00225830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2250804u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2250808u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2250812u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2250816u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2250820u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2250824u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2250828u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2250832u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967068u32, 2250836u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2250840u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2250844u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2250848u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2250852u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2250856u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2250860u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2250864u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2250868u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2250872u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2250876u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2250880u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2250884u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2250888u32);
    emu.apc_no_count(1usize, 2250888u32, 4294934528u32, 2250892u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250896u32;
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
#[inline]
pub fn block_0x00225890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2250900u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2250904u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2250908u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2250912u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2250916u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2250920u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2250924u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2250928u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967120u32, 2250932u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2250936u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2250940u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2250944u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2250948u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2250952u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2250956u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2250960u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2250964u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2250968u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2250972u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2250976u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2250980u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2250984u32);
    emu.apc_no_count(1usize, 2250984u32, 4294934528u32, 2250988u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2250992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002258f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2250996u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2251000u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2251004u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2251008u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2251012u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2251016u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2251020u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2251024u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967220u32, 2251028u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2251032u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2251036u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2251040u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2251044u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2251048u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2251052u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2251056u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2251060u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2251064u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2251068u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2251072u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2251076u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2251080u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2251084u32);
    emu.apc_no_count(1usize, 2251084u32, 4294934528u32, 2251088u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2251092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00225954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2251096u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2251100u32)?;
    emu.adi_no_count(14usize, 13usize, 0u32, 2251104u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2251108u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2251112u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2251116u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2251120u32);
    emu.apc_no_count(1usize, 2251120u32, 0u32, 2251124u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2251128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(28u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2251132u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2251136u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2251140u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2251144u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2251148u32;
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
pub fn block_0x0022598c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2251324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a3c));
    } else {
        emu.pc = 2251152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225990));
    }
}
#[inline(always)]
pub fn block_0x00225990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2251324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a3c));
    } else {
        emu.pc = 2251156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225994));
    }
}
#[inline(always)]
pub fn block_0x00225994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2251636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b74));
    } else {
        emu.pc = 2251160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225998));
    }
}
#[inline(always)]
pub fn block_0x00225998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2251668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b94));
    } else {
        emu.pc = 2251164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022599c));
    }
}
#[inline(always)]
pub fn block_0x0022599c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2251168u32);
    emu.sltru_no_count(15usize, 17usize, 12usize, 2251172u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2251176u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2251180u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2251808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c20));
    } else {
        emu.pc = 2251184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259b0));
    }
}
#[inline(always)]
pub fn block_0x002259b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2251188u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2251192u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2251196u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2251200u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2251832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c38));
    } else {
        emu.pc = 2251204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259c4));
    }
}
#[inline(always)]
pub fn block_0x002259c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2251208u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2251212u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2251216u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2251220u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251224u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2251860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c54));
    } else {
        emu.pc = 2251228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259dc));
    }
}
#[inline(always)]
pub fn block_0x002259dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2251232u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2251236u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2251240u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2251244u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251248u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2251256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259f8));
    } else {
        emu.pc = 2251252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259f4));
    }
}
#[inline(always)]
pub fn block_0x002259f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2251256u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2251256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002259f8));
}
#[inline(always)]
pub fn block_0x002259f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 1u32, 2251260u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2251264u32);
    emu.sltru_no_count(15usize, 15usize, 12usize, 2251268u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2251272u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2251276u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2251280u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2251892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c74));
    } else {
        emu.pc = 2251284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a14));
    }
}
#[inline(always)]
pub fn block_0x00225a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 31u32, 2251288u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2251292u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2251296u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2251912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c88));
    } else {
        emu.pc = 2251300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a24));
    }
}
#[inline(always)]
pub fn block_0x00225a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(14usize, 14usize, 16usize, 2251304u32);
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2251308u32);
    emu.sri_no_count(6usize, 13usize, 1u32, 2251312u32);
    emu.srr_no_count(15usize, 6usize, 15usize, 2251316u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2251320u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2251324u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251916u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225c8c));
}
#[inline(always)]
pub fn block_0x00225a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2251344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a50));
    } else {
        emu.pc = 2251328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a40));
    }
}
#[inline(always)]
pub fn block_0x00225a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 14usize, 2251332u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2251336u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2251356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a5c));
    } else {
        emu.pc = 2251340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a4c));
    }
}
#[inline(always)]
pub fn block_0x00225a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2251344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225b4c));
}
#[inline(always)]
pub fn block_0x00225a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 13usize, 2251348u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2251352u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2251596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b4c));
    } else {
        emu.pc = 2251356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a5c));
    }
}
#[inline(always)]
pub fn block_0x00225a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2251596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b4c));
    } else {
        emu.pc = 2251360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a60));
    }
}
#[inline(always)]
pub fn block_0x00225a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 12usize, 16u32, 2251364u32);
    emu.sltru_no_count(15usize, 17usize, 14usize, 2251368u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2251372u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2251376u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2251724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225bcc));
    } else {
        emu.pc = 2251380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a74));
    }
}
#[inline(always)]
pub fn block_0x00225a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2251384u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2251388u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2251392u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2251396u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2251748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225be4));
    } else {
        emu.pc = 2251400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a88));
    }
}
#[inline(always)]
pub fn block_0x00225a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2251404u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2251408u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2251412u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2251416u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251420u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2251776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c00));
    } else {
        emu.pc = 2251424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225aa0));
    }
}
#[inline(always)]
pub fn block_0x00225aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2251428u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2251432u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2251436u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2251440u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251444u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2251452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225abc));
    } else {
        emu.pc = 2251448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225ab8));
    }
}
#[inline(always)]
pub fn block_0x00225ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2251452u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2251452u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225abc));
}
#[inline]
pub fn block_0x00225abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2251456u32);
    emu.sli_no_count(5usize, 5usize, 1u32, 2251460u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2251464u32);
    emu.sltru_no_count(15usize, 15usize, 14usize, 2251468u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2251472u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2251476u32);
    emu.sri_no_count(5usize, 13usize, 1u32, 2251480u32);
    emu.orr_no_count(16usize, 16usize, 15usize, 2251484u32);
    emu.xri_no_count(15usize, 16usize, 31u32, 2251488u32);
    emu.srr_no_count(15usize, 5usize, 15usize, 2251492u32);
    emu.slr_no_count(5usize, 14usize, 16usize, 2251496u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2251500u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2251504u32);
    emu.slr_no_count(5usize, 5usize, 16usize, 2251508u32);
    emu.slr_no_count(6usize, 13usize, 16usize, 2251512u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2251516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225b10));
}
#[inline(always)]
pub fn block_0x00225afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(16usize, 6usize, 1u32, 2251520u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2251524u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2251528u32);
    emu.orr_no_count(6usize, 16usize, 6usize, 2251532u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2251536u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2251536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225b10));
}
#[inline(always)]
pub fn block_0x00225b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 11usize, 6usize, 2251540u32);
    emu.sbr_no_count(7usize, 12usize, 15usize, 2251544u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2251548u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2251516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225afc));
    } else {
        emu.pc = 2251552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b20));
    }
}
#[inline(always)]
pub fn block_0x00225b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 6usize, 2251556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2251576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b38));
    } else {
        emu.pc = 2251560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b28));
    }
}
#[inline(always)]
pub fn block_0x00225b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2251564u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2251568u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2251588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b44));
    } else {
        emu.pc = 2251572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b34));
    }
}
#[inline(always)]
pub fn block_0x00225b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2251576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225b60));
}
#[inline(always)]
pub fn block_0x00225b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2251580u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2251584u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2251616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b60));
    } else {
        emu.pc = 2251588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b44));
    }
}
#[inline(always)]
pub fn block_0x00225b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 16usize, 0u32, 2251592u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2251596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225afc));
}
#[inline(always)]
pub fn block_0x00225b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2251600u32)?;
    emu.sw_no_count(17usize, 10usize, 4u32, 2251604u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2251608u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2251612u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2251616u32;
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
pub fn block_0x00225b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2251620u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2251624u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2251628u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2251632u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2251636u32;
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
pub fn block_0x00225b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(17usize, 11usize, 13usize, 2251640u32);
    emu.mul_no_count(12usize, 17usize, 13usize, 2251644u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2251648u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2251652u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2251656u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2251660u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2251664u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2251668u32;
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
pub fn block_0x00225b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2252032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225d00));
    } else {
        emu.pc = 2251672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225b98));
    }
}
#[inline]
pub fn block_0x00225b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(13usize, 11usize, 12usize, 2251676u32);
    emu.mul_no_count(12usize, 13usize, 12usize, 2251680u32);
    emu.sltru_no_count(15usize, 0usize, 13usize, 2251684u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2251688u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2251692u32);
    emu.adi_no_count(17usize, 13usize, 1u32, 2251696u32);
    emu.sltiu_no_count(12usize, 17usize, 1u32, 2251700u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2251704u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2251708u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2251712u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2251716u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2251720u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2251724u32;
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
pub fn block_0x00225bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2251728u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2251732u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2251736u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2251740u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2251744u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2251400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a88));
    } else {
        emu.pc = 2251748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225be4));
    }
}
#[inline(always)]
pub fn block_0x00225be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2251752u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2251756u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2251760u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2251764u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2251768u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251772u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2251424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225aa0));
    } else {
        emu.pc = 2251776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c00));
    }
}
#[inline(always)]
pub fn block_0x00225c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2251780u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2251784u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2251788u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2251792u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2251796u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251800u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2251448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225ab8));
    } else {
        emu.pc = 2251804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c1c));
    }
}
#[inline(always)]
pub fn block_0x00225c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2251808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251452u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225abc));
}
#[inline(always)]
pub fn block_0x00225c20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2251812u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2251816u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2251820u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2251824u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2251828u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2251204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259c4));
    } else {
        emu.pc = 2251832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c38));
    }
}
#[inline(always)]
pub fn block_0x00225c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2251836u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2251840u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2251844u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2251848u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2251852u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251856u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2251228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259dc));
    } else {
        emu.pc = 2251860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c54));
    }
}
#[inline(always)]
pub fn block_0x00225c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2251864u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2251868u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2251872u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2251876u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2251880u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2251884u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2251252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002259f4));
    } else {
        emu.pc = 2251888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c70));
    }
}
#[inline(always)]
pub fn block_0x00225c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2251892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002259f8));
}
#[inline(always)]
pub fn block_0x00225c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 32u32, 2251896u32);
    emu.sbr_no_count(16usize, 16usize, 15usize, 2251900u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2251904u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2251908u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2251300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225a24));
    } else {
        emu.pc = 2251912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225c88));
    }
}
#[inline(always)]
pub fn block_0x00225c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2251916u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2251916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225c8c));
}
#[inline(always)]
pub fn block_0x00225c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2251920u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2251924u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2251928u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2251932u32);
    emu.slr_no_count(16usize, 5usize, 16usize, 2251936u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2251940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225cb8));
}
#[inline(always)]
pub fn block_0x00225ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 17usize, 1u32, 2251944u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2251948u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2251952u32);
    emu.orr_no_count(17usize, 17usize, 5usize, 2251956u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2251960u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2251960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225cb8));
}
#[inline(always)]
pub fn block_0x00225cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 17usize, 2251964u32);
    emu.sbr_no_count(6usize, 12usize, 15usize, 2251968u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2251972u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2251940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225ca4));
    } else {
        emu.pc = 2251976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225cc8));
    }
}
#[inline(always)]
pub fn block_0x00225cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 17usize, 2251980u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2251984u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2251996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225cdc));
    } else {
        emu.pc = 2251988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225cd4));
    }
}
#[inline(always)]
pub fn block_0x00225cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 5usize, 0u32, 2251992u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2251996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2251940u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225ca4));
}
#[inline]
pub fn block_0x00225cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(12usize, 11usize, 13usize, 2252000u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2252004u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2252008u32);
    emu.orr_no_count(17usize, 12usize, 14usize, 2252012u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2252016u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2252020u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2252024u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2252028u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2252032u32;
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
pub fn block_0x00225d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2252036u32);
    emu.divu_no_count(15usize, 12usize, 13usize, 2252040u32);
    emu.mul_no_count(16usize, 15usize, 13usize, 2252044u32);
    emu.sbr_no_count(16usize, 12usize, 16usize, 2252048u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2252140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225d6c));
    } else {
        emu.pc = 2252052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225d14));
    }
}
#[inline]
pub fn block_0x00225d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2252056u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2252060u32);
    emu.sli_no_count(11usize, 11usize, 16u32, 2252064u32);
    emu.orr_no_count(14usize, 16usize, 12usize, 2252068u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2252072u32);
    emu.divu_no_count(14usize, 14usize, 13usize, 2252076u32);
    emu.mul_no_count(16usize, 14usize, 13usize, 2252080u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2252084u32);
    emu.sli_no_count(16usize, 14usize, 16u32, 2252088u32);
    emu.sri_no_count(14usize, 14usize, 16u32, 2252092u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2252096u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2252100u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2252104u32);
    emu.divu_no_count(12usize, 11usize, 13usize, 2252108u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2252112u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2252116u32);
    emu.orr_no_count(17usize, 16usize, 12usize, 2252120u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2252124u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2252128u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2252132u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2252136u32)?;
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2252140u32;
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
pub fn block_0x00225d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2252152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225d78));
    } else {
        emu.pc = 2252144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225d70));
    }
}
#[inline(always)]
pub fn block_0x00225d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2252148u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2252152u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2252156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225d7c));
}
#[inline(always)]
pub fn block_0x00225d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2252156u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2252156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225d7c));
}
#[inline(always)]
pub fn block_0x00225d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2252180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225d94));
    } else {
        emu.pc = 2252160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225d80));
    }
}
#[inline(always)]
pub fn block_0x00225d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2252164u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2252168u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2252172u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2252176u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2252180u32;
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
pub fn block_0x00225d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 1u32, 2252184u32);
    emu.sli_no_count(14usize, 14usize, 31u32, 2252188u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2252192u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2252196u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2252200u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let return_addr = 2252204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2252224u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225dc0));
}
#[inline(always)]
pub fn block_0x00225dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 5usize, 1u32, 2252208u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2252212u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2252216u32);
    emu.orr_no_count(5usize, 5usize, 6usize, 2252220u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2252224u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2252224u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225dc0));
}
#[inline(always)]
pub fn block_0x00225dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 11usize, 5usize, 2252228u32);
    emu.sbr_no_count(7usize, 16usize, 14usize, 2252232u32);
    emu.sbr_no_count(6usize, 7usize, 6usize, 2252236u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2252204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225dac));
    } else {
        emu.pc = 2252240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225dd0));
    }
}
#[inline(always)]
pub fn block_0x00225dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 5usize, 2252244u32);
    emu.orr_no_count(12usize, 17usize, 12usize, 2252248u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2252260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225de4));
    } else {
        emu.pc = 2252252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225ddc));
    }
}
#[inline(always)]
pub fn block_0x00225ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 6usize, 0u32, 2252256u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2252260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2252204u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225dac));
}
#[inline]
pub fn block_0x00225de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(14usize, 11usize, 13usize, 2252264u32);
    emu.mul_no_count(13usize, 14usize, 13usize, 2252268u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2252272u32);
    emu.orr_no_count(17usize, 14usize, 12usize, 2252276u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2252280u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2252284u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2252288u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2252292u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2252296u32;
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
pub fn block_0x00225e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2252328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225e28));
    } else {
        emu.pc = 2252300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225e0c));
    }
}
#[inline(always)]
pub fn block_0x00225e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2252304u32);
    emu.lbu_no_count(14usize, 11usize, 0u32, 2252308u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2252336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2252336u32));
    } else {
        emu.pc = 2252312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225e18));
    }
}
#[inline(always)]
pub fn block_0x00225e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2252316u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2252320u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2252324u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2252300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225e0c));
    } else {
        emu.pc = 2252328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225e28));
    }
}
#[inline(always)]
pub fn block_0x00225e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2252332u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2252336u32;
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
