pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2192688u32;
pub const PC_MAX: u32 = 2194984u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x00217530,
        block_0x0021754c,
        block_0x00217564,
        block_0x00217580,
        block_0x00217598,
        block_0x002175b0,
        block_0x002175c4,
        block_0x002175dc,
        block_0x002175f4,
        block_0x00217608,
        block_0x00217610,
        block_0x00217618,
        block_0x00217620,
        block_0x00217634,
        block_0x0021763c,
        block_0x00217640,
        block_0x00217658,
        block_0x0021765c,
        block_0x00217668,
        block_0x00217674,
        block_0x0021767c,
        block_0x00217680,
        block_0x00217698,
        block_0x002176a0,
        block_0x002176a8,
        block_0x002176c4,
        block_0x002176e0,
        block_0x00217714,
        block_0x00217718,
        block_0x0021772c,
        block_0x00217730,
        block_0x00217738,
        block_0x00217744,
        block_0x002177c0,
        block_0x002177c4,
        block_0x002177e8,
        block_0x00217810,
        block_0x00217834,
        block_0x00217894,
        block_0x002178f4,
        block_0x00217954,
        block_0x00217978,
        block_0x0021798c,
        block_0x00217990,
        block_0x00217994,
        block_0x00217998,
        block_0x0021799c,
        block_0x002179b0,
        block_0x002179c4,
        block_0x002179dc,
        block_0x002179f4,
        block_0x002179f8,
        block_0x00217a14,
        block_0x00217a24,
        block_0x00217a3c,
        block_0x00217a40,
        block_0x00217a4c,
        block_0x00217a50,
        block_0x00217a5c,
        block_0x00217a60,
        block_0x00217a74,
        block_0x00217a88,
        block_0x00217aa0,
        block_0x00217ab8,
        block_0x00217abc,
        block_0x00217afc,
        block_0x00217b10,
        block_0x00217b20,
        block_0x00217b28,
        block_0x00217b34,
        block_0x00217b38,
        block_0x00217b44,
        block_0x00217b4c,
        block_0x00217b60,
        block_0x00217b74,
        block_0x00217b94,
        block_0x00217b98,
        block_0x00217bcc,
        block_0x00217be4,
        block_0x00217c00,
        block_0x00217c1c,
        block_0x00217c20,
        block_0x00217c38,
        block_0x00217c54,
        block_0x00217c70,
        block_0x00217c74,
        block_0x00217c88,
        block_0x00217c8c,
        block_0x00217ca4,
        block_0x00217cb8,
        block_0x00217cc8,
        block_0x00217cd4,
        block_0x00217cdc,
        block_0x00217d00,
        block_0x00217d14,
        block_0x00217d6c,
        block_0x00217d70,
        block_0x00217d78,
        block_0x00217d7c,
        block_0x00217d80,
        block_0x00217d94,
        block_0x00217dac,
        block_0x00217dc0,
        block_0x00217dd0,
        block_0x00217ddc,
        block_0x00217de4,
        block_0x00217e08,
        block_0x00217e0c,
        block_0x00217e18,
        block_0x00217e28,
    ];
    const IDX: [u16; 575usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 12u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16,
        14u16, 0u16, 15u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 18u16, 0u16,
        0u16, 19u16, 0u16, 0u16, 20u16, 0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        23u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16,
        0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16, 45u16, 46u16, 47u16,
        0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        55u16, 56u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16, 59u16, 60u16, 0u16, 0u16,
        0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 70u16,
        71u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16,
        0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 92u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 97u16,
        0u16, 98u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 108u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16,
    ];
    if pc < 2192688u32 || pc > 2194984u32 {
        return None;
    }
    let word_offset = ((pc - 2192688u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00217530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2192692u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1571u32, 2192696u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192700u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2192704u32);
    emu.adi_no_count(11usize, 0usize, 27u32, 2192708u32);
    emu.apc_no_count(1usize, 2192708u32, 4294934528u32, 2192712u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021754c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192720u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966792u32, 2192724u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2192728u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2192732u32);
    emu.apc_no_count(1usize, 2192732u32, 0u32, 2192736u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192740u32;
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
pub fn block_0x00217564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2192744u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1516u32, 2192748u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192752u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2192756u32);
    emu.adi_no_count(11usize, 0usize, 26u32, 2192760u32);
    emu.apc_no_count(1usize, 2192760u32, 4294934528u32, 2192764u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192772u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966824u32, 2192776u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2192780u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2192784u32);
    emu.apc_no_count(1usize, 2192784u32, 0u32, 2192788u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192792u32;
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
pub fn block_0x00217598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192796u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966776u32, 2192800u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2192804u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2192808u32);
    emu.apc_no_count(1usize, 2192808u32, 0u32, 2192812u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192816u32;
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
pub fn block_0x002175b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192820u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1648u32, 2192824u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2192828u32);
    emu.apc_no_count(1usize, 2192828u32, 0u32, 2192832u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192836u32;
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
pub fn block_0x002175c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192840u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2192844u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2192848u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2192852u32);
    emu.apc_no_count(1usize, 2192852u32, 4294934528u32, 2192856u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002175dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192864u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966808u32, 2192868u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2192872u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2192876u32);
    emu.apc_no_count(1usize, 2192876u32, 4294934528u32, 2192880u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002175f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2192888u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966760u32, 2192892u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2192896u32);
    emu.apc_no_count(1usize, 2192896u32, 4294934528u32, 2192900u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2192904u32, 0u32, 2192908u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2192912u32, 0u32, 2192916u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2192920u32, 0u32, 2192924u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2192932u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2192936u32);
    emu.ani_no_count(10usize, 10usize, 4294967292u32, 2192940u32);
    emu.sbr_no_count(5usize, 10usize, 12usize, 2192944u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2192988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021765c));
    } else {
        emu.pc = 2192948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217634));
    }
}
#[inline(always)]
pub fn block_0x00217634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2192952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2192984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217658));
    } else {
        emu.pc = 2192956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021763c));
    }
}
#[inline(always)]
pub fn block_0x0021763c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 12usize, 11usize, 2192960u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2192960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217640));
}
#[inline(always)]
pub fn block_0x00217640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2192964u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2192968u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2192972u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2192976u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2192980u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2192960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217640));
    } else {
        emu.pc = 2192984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217658));
    }
}
#[inline(always)]
pub fn block_0x00217658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2192988u32;
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
pub fn block_0x0021765c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 11usize, 5usize, 2192992u32);
    emu.sri_no_count(17usize, 13usize, 2u32, 2192996u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2192948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217634));
    } else {
        emu.pc = 2193000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217668));
    }
}
#[inline(always)]
pub fn block_0x00217668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 12usize, 5usize, 2193004u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2193008u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2193020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021767c));
    } else {
        emu.pc = 2193012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217674));
    }
}
#[inline(always)]
pub fn block_0x00217674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2193016u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2193020u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2193048u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217698));
}
#[inline(always)]
pub fn block_0x0021767c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2193024u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2193024u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217680));
}
#[inline(always)]
pub fn block_0x00217680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 12usize, 0u32, 2193028u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2193032u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2193036u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2193040u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2193044u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2193024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217680));
    } else {
        emu.pc = 2193048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217698));
    }
}
#[inline(always)]
pub fn block_0x00217698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2193052u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2193092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002176c4));
    } else {
        emu.pc = 2193056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002176a0));
    }
}
#[inline(always)]
pub fn block_0x002176a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2193060u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2193064u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2193064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002176a8));
}
#[inline(always)]
pub fn block_0x002176a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2193068u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2193072u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2193076u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2193080u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2193084u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2193088u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2193064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002176a8));
    } else {
        emu.pc = 2193092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002176c4));
    }
}
#[inline(always)]
pub fn block_0x002176c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193096u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16711680u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193100u32;
    emu.update_insn_clock();
    emu.adr_no_count(10usize, 12usize, 10usize, 2193104u32);
    emu.adi_no_count(12usize, 11usize, 257u32, 2193108u32);
    emu.adi_no_count(11usize, 13usize, 255u32, 2193112u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2193116u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2193120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2193172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217714));
}
#[inline]
pub fn block_0x002176e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(6usize, 16usize, 2u32, 2193124u32);
    emu.sbr_no_count(17usize, 13usize, 16usize, 2193128u32);
    emu.ani_no_count(7usize, 16usize, 3u32, 2193132u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2193136u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2193140u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2193144u32);
    emu.anr_no_count(6usize, 29usize, 11usize, 2193148u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2193152u32);
    emu.sli_no_count(28usize, 6usize, 16u32, 2193156u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2193160u32);
    emu.sri_no_count(6usize, 6usize, 16u32, 2193164u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2193168u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2193348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177c4));
    } else {
        emu.pc = 2193172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217714));
    }
}
#[inline(always)]
pub fn block_0x00217714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2192984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217658));
    } else {
        emu.pc = 2193176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217718));
    }
}
#[inline(always)]
pub fn block_0x00217718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 17usize, 0u32, 2193180u32);
    emu.adi_no_count(15usize, 5usize, 0u32, 2193184u32);
    emu.adi_no_count(17usize, 0usize, 192u32, 2193188u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2193192u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2193200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217730));
    } else {
        emu.pc = 2193196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021772c));
    }
}
#[inline(always)]
pub fn block_0x0021772c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 192u32, 2193200u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2193200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217730));
}
#[inline(always)]
pub fn block_0x00217730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2193204u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2193120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002176e0));
    } else {
        emu.pc = 2193208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217738));
    }
}
#[inline(always)]
pub fn block_0x00217738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 2u32, 2193212u32);
    emu.sli_no_count(17usize, 17usize, 4u32, 2193216u32);
    emu.adi_no_count(6usize, 15usize, 0u32, 2193220u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2193220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217744));
}
#[inline(never)]
pub fn block_0x00217744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 6usize, 0u32, 2193224u32)?;
    emu.lw_no_count(28usize, 6usize, 4u32, 2193228u32)?;
    emu.lw_no_count(29usize, 6usize, 8u32, 2193232u32)?;
    emu.lw_no_count(30usize, 6usize, 12u32, 2193236u32)?;
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2193240u32);
    emu.sri_no_count(7usize, 7usize, 6u32, 2193244u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2193248u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2193252u32);
    emu.xri_no_count(31usize, 28usize, 4294967295u32, 2193256u32);
    emu.sri_no_count(28usize, 28usize, 6u32, 2193260u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2193264u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2193268u32);
    emu.xri_no_count(31usize, 29usize, 4294967295u32, 2193272u32);
    emu.sri_no_count(29usize, 29usize, 6u32, 2193276u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2193280u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2193284u32);
    emu.xri_no_count(31usize, 30usize, 4294967295u32, 2193288u32);
    emu.sri_no_count(30usize, 30usize, 6u32, 2193292u32);
    emu.sri_no_count(31usize, 31usize, 7u32, 2193296u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2193300u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2193304u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2193308u32);
    emu.adi_no_count(17usize, 17usize, 4294967280u32, 2193312u32);
    emu.anr_no_count(7usize, 28usize, 12usize, 2193316u32);
    emu.anr_no_count(28usize, 29usize, 12usize, 2193320u32);
    emu.anr_no_count(29usize, 30usize, 12usize, 2193324u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2193328u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2193332u32);
    emu.adr_no_count(5usize, 5usize, 29usize, 2193336u32);
    emu.adi_no_count(6usize, 6usize, 16u32, 2193340u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2193220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217744));
    } else {
        emu.pc = 2193344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177c0));
    }
}
#[inline(always)]
pub fn block_0x002177c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2193348u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2193120u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002176e0));
}
#[inline]
pub fn block_0x002177c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2193352u32);
    emu.ani_no_count(16usize, 16usize, 252u32, 2193356u32);
    emu.sli_no_count(16usize, 16usize, 2u32, 2193360u32);
    emu.adr_no_count(15usize, 15usize, 16usize, 2193364u32);
    emu.sltiu_no_count(16usize, 13usize, 192u32, 2193368u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2193372u32);
    emu.anr_no_count(13usize, 13usize, 16usize, 2193376u32);
    emu.ani_no_count(13usize, 13usize, 3u32, 2193380u32);
    emu.sli_no_count(13usize, 13usize, 2u32, 2193384u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2193384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002177e8));
}
#[inline]
pub fn block_0x002177e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2193388u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2193392u32);
    emu.xri_no_count(17usize, 16usize, 4294967295u32, 2193396u32);
    emu.sri_no_count(16usize, 16usize, 6u32, 2193400u32);
    emu.sri_no_count(17usize, 17usize, 7u32, 2193404u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2193408u32);
    emu.anr_no_count(16usize, 16usize, 12usize, 2193412u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2193416u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2193420u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2193384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002177e8));
    } else {
        emu.pc = 2193424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217810));
    }
}
#[inline]
pub fn block_0x00217810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(12usize, 14usize, 11usize, 2193428u32);
    emu.sri_no_count(14usize, 14usize, 8u32, 2193432u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2193436u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2193440u32);
    emu.sli_no_count(12usize, 11usize, 16u32, 2193444u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2193448u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2193452u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2193456u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193460u32;
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
pub fn block_0x00217834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2193464u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2193468u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2193472u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2193476u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193480u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2193484u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2193488u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2193492u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966924u32, 2193496u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2193500u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2193504u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2193508u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2193512u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2193516u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2193520u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2193524u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2193528u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2193532u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2193536u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2193540u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2193544u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2193548u32);
    emu.apc_no_count(1usize, 2193548u32, 4294934528u32, 2193552u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2193560u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2193564u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2193568u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2193572u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193576u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2193580u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2193584u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2193588u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966940u32, 2193592u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2193596u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2193600u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2193604u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2193608u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2193612u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2193616u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2193620u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2193624u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2193628u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2193632u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2193636u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2193640u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2193644u32);
    emu.apc_no_count(1usize, 2193644u32, 4294934528u32, 2193648u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002178f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2193656u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2193660u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2193664u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2193668u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2193672u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2193676u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2193680u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2193684u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966992u32, 2193688u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2193692u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2193696u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2193700u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2193704u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2193708u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2193712u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2193716u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2193720u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2193724u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2193728u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2193732u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2193736u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2193740u32);
    emu.apc_no_count(1usize, 2193740u32, 4294934528u32, 2193744u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2193752u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2193756u32)?;
    emu.adi_no_count(14usize, 13usize, 0u32, 2193760u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2193764u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2193768u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2193772u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2193776u32);
    emu.apc_no_count(1usize, 2193776u32, 0u32, 2193780u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193784u32;
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
pub fn block_0x00217978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2193788u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2193792u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2193796u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2193800u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193804u32;
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
pub fn block_0x0021798c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2193980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a3c));
    } else {
        emu.pc = 2193808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217990));
    }
}
#[inline(always)]
pub fn block_0x00217990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2193980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a3c));
    } else {
        emu.pc = 2193812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217994));
    }
}
#[inline(always)]
pub fn block_0x00217994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b74));
    } else {
        emu.pc = 2193816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217998));
    }
}
#[inline(always)]
pub fn block_0x00217998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b94));
    } else {
        emu.pc = 2193820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021799c));
    }
}
#[inline(always)]
pub fn block_0x0021799c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2193824u32);
    emu.sltru_no_count(15usize, 17usize, 12usize, 2193828u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2193832u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2193836u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2194464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c20));
    } else {
        emu.pc = 2193840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179b0));
    }
}
#[inline(always)]
pub fn block_0x002179b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2193844u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2193848u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2193852u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2193856u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2194488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c38));
    } else {
        emu.pc = 2193860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179c4));
    }
}
#[inline(always)]
pub fn block_0x002179c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2193864u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2193868u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2193872u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2193876u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2193880u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2194516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c54));
    } else {
        emu.pc = 2193884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179dc));
    }
}
#[inline(always)]
pub fn block_0x002179dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2193888u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2193892u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2193896u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2193900u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2193904u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2193912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179f8));
    } else {
        emu.pc = 2193908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179f4));
    }
}
#[inline(always)]
pub fn block_0x002179f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2193912u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2193912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002179f8));
}
#[inline(always)]
pub fn block_0x002179f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 1u32, 2193916u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2193920u32);
    emu.sltru_no_count(15usize, 15usize, 12usize, 2193924u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2193928u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2193932u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2193936u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2194548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c74));
    } else {
        emu.pc = 2193940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a14));
    }
}
#[inline(always)]
pub fn block_0x00217a14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 31u32, 2193944u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2193948u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2193952u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2194568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c88));
    } else {
        emu.pc = 2193956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a24));
    }
}
#[inline(always)]
pub fn block_0x00217a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(14usize, 14usize, 16usize, 2193960u32);
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2193964u32);
    emu.sri_no_count(6usize, 13usize, 1u32, 2193968u32);
    emu.srr_no_count(15usize, 6usize, 15usize, 2193972u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2193976u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2193980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217c8c));
}
#[inline(always)]
pub fn block_0x00217a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a50));
    } else {
        emu.pc = 2193984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a40));
    }
}
#[inline(always)]
pub fn block_0x00217a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 14usize, 2193988u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2193992u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2194012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a5c));
    } else {
        emu.pc = 2193996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a4c));
    }
}
#[inline(always)]
pub fn block_0x00217a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2194000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b4c));
}
#[inline(always)]
pub fn block_0x00217a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 13usize, 2194004u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2194008u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2194252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b4c));
    } else {
        emu.pc = 2194012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a5c));
    }
}
#[inline(always)]
pub fn block_0x00217a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b4c));
    } else {
        emu.pc = 2194016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a60));
    }
}
#[inline(always)]
pub fn block_0x00217a60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 12usize, 16u32, 2194020u32);
    emu.sltru_no_count(15usize, 17usize, 14usize, 2194024u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2194028u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2194032u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2194380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217bcc));
    } else {
        emu.pc = 2194036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a74));
    }
}
#[inline(always)]
pub fn block_0x00217a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2194040u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2194044u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2194048u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2194052u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2194404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217be4));
    } else {
        emu.pc = 2194056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a88));
    }
}
#[inline(always)]
pub fn block_0x00217a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2194060u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2194064u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2194068u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2194072u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2194076u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2194432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c00));
    } else {
        emu.pc = 2194080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217aa0));
    }
}
#[inline(always)]
pub fn block_0x00217aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2194084u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2194088u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2194092u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2194096u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2194100u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2194108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217abc));
    } else {
        emu.pc = 2194104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ab8));
    }
}
#[inline(always)]
pub fn block_0x00217ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2194108u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217abc));
}
#[inline]
pub fn block_0x00217abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2194112u32);
    emu.sli_no_count(5usize, 5usize, 1u32, 2194116u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2194120u32);
    emu.sltru_no_count(15usize, 15usize, 14usize, 2194124u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2194128u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2194132u32);
    emu.sri_no_count(5usize, 13usize, 1u32, 2194136u32);
    emu.orr_no_count(16usize, 16usize, 15usize, 2194140u32);
    emu.xri_no_count(15usize, 16usize, 31u32, 2194144u32);
    emu.srr_no_count(15usize, 5usize, 15usize, 2194148u32);
    emu.slr_no_count(5usize, 14usize, 16usize, 2194152u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2194156u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2194160u32);
    emu.slr_no_count(5usize, 5usize, 16usize, 2194164u32);
    emu.slr_no_count(6usize, 13usize, 16usize, 2194168u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2194172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194192u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b10));
}
#[inline(always)]
pub fn block_0x00217afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(16usize, 6usize, 1u32, 2194176u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2194180u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2194184u32);
    emu.orr_no_count(6usize, 16usize, 6usize, 2194188u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2194192u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2194192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b10));
}
#[inline(always)]
pub fn block_0x00217b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 11usize, 6usize, 2194196u32);
    emu.sbr_no_count(7usize, 12usize, 15usize, 2194200u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2194204u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217afc));
    } else {
        emu.pc = 2194208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b20));
    }
}
#[inline(always)]
pub fn block_0x00217b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 6usize, 2194212u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2194232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b38));
    } else {
        emu.pc = 2194216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b28));
    }
}
#[inline(always)]
pub fn block_0x00217b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2194220u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2194224u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2194244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b44));
    } else {
        emu.pc = 2194228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b34));
    }
}
#[inline(always)]
pub fn block_0x00217b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2194232u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b60));
}
#[inline(always)]
pub fn block_0x00217b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2194236u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2194240u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2194272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b60));
    } else {
        emu.pc = 2194244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b44));
    }
}
#[inline(always)]
pub fn block_0x00217b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 16usize, 0u32, 2194248u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217afc));
}
#[inline(always)]
pub fn block_0x00217b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2194256u32)?;
    emu.sw_no_count(17usize, 10usize, 4u32, 2194260u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194264u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2194268u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194272u32;
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
pub fn block_0x00217b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2194276u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2194280u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194284u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2194288u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194292u32;
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
pub fn block_0x00217b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(17usize, 11usize, 13usize, 2194296u32);
    emu.mul_no_count(12usize, 17usize, 13usize, 2194300u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2194304u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2194308u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2194312u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194316u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2194320u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194324u32;
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
pub fn block_0x00217b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d00));
    } else {
        emu.pc = 2194328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217b98));
    }
}
#[inline]
pub fn block_0x00217b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(13usize, 11usize, 12usize, 2194332u32);
    emu.mul_no_count(12usize, 13usize, 12usize, 2194336u32);
    emu.sltru_no_count(15usize, 0usize, 13usize, 2194340u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2194344u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2194348u32);
    emu.adi_no_count(17usize, 13usize, 1u32, 2194352u32);
    emu.sltiu_no_count(12usize, 17usize, 1u32, 2194356u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2194360u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2194364u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2194368u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194372u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2194376u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194380u32;
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
pub fn block_0x00217bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2194384u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2194388u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2194392u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2194396u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2194400u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2194056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a88));
    } else {
        emu.pc = 2194404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217be4));
    }
}
#[inline(always)]
pub fn block_0x00217be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2194408u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2194412u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2194416u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2194420u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2194424u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2194428u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2194080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217aa0));
    } else {
        emu.pc = 2194432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c00));
    }
}
#[inline(always)]
pub fn block_0x00217c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2194436u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2194440u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2194444u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2194448u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2194452u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2194456u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2194104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ab8));
    } else {
        emu.pc = 2194460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c1c));
    }
}
#[inline(always)]
pub fn block_0x00217c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2194464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217abc));
}
#[inline(always)]
pub fn block_0x00217c20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2194468u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2194472u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2194476u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2194480u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2194484u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2193860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179c4));
    } else {
        emu.pc = 2194488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c38));
    }
}
#[inline(always)]
pub fn block_0x00217c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2194492u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2194496u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2194500u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2194504u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2194508u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2194512u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2193884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179dc));
    } else {
        emu.pc = 2194516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c54));
    }
}
#[inline(always)]
pub fn block_0x00217c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2194520u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2194524u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2194528u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2194532u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2194536u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2194540u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2193908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002179f4));
    } else {
        emu.pc = 2194544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c70));
    }
}
#[inline(always)]
pub fn block_0x00217c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2194548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2193912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002179f8));
}
#[inline(always)]
pub fn block_0x00217c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 32u32, 2194552u32);
    emu.sbr_no_count(16usize, 16usize, 15usize, 2194556u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2194560u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2194564u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2193956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217a24));
    } else {
        emu.pc = 2194568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c88));
    }
}
#[inline(always)]
pub fn block_0x00217c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2194572u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217c8c));
}
#[inline(always)]
pub fn block_0x00217c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2194576u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2194580u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2194584u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2194588u32);
    emu.slr_no_count(16usize, 5usize, 16usize, 2194592u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2194596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194616u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cb8));
}
#[inline(always)]
pub fn block_0x00217ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 17usize, 1u32, 2194600u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2194604u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2194608u32);
    emu.orr_no_count(17usize, 17usize, 5usize, 2194612u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2194616u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2194616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cb8));
}
#[inline(always)]
pub fn block_0x00217cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 17usize, 2194620u32);
    emu.sbr_no_count(6usize, 12usize, 15usize, 2194624u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2194628u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ca4));
    } else {
        emu.pc = 2194632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cc8));
    }
}
#[inline(always)]
pub fn block_0x00217cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 17usize, 2194636u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2194640u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2194652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cdc));
    } else {
        emu.pc = 2194644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cd4));
    }
}
#[inline(always)]
pub fn block_0x00217cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 5usize, 0u32, 2194648u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217ca4));
}
#[inline]
pub fn block_0x00217cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(12usize, 11usize, 13usize, 2194656u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2194660u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2194664u32);
    emu.orr_no_count(17usize, 12usize, 14usize, 2194668u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2194672u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2194676u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194680u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2194684u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194688u32;
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
pub fn block_0x00217d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2194692u32);
    emu.divu_no_count(15usize, 12usize, 13usize, 2194696u32);
    emu.mul_no_count(16usize, 15usize, 13usize, 2194700u32);
    emu.sbr_no_count(16usize, 12usize, 16usize, 2194704u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2194796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d6c));
    } else {
        emu.pc = 2194708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d14));
    }
}
#[inline]
pub fn block_0x00217d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2194712u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2194716u32);
    emu.sli_no_count(11usize, 11usize, 16u32, 2194720u32);
    emu.orr_no_count(14usize, 16usize, 12usize, 2194724u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2194728u32);
    emu.divu_no_count(14usize, 14usize, 13usize, 2194732u32);
    emu.mul_no_count(16usize, 14usize, 13usize, 2194736u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2194740u32);
    emu.sli_no_count(16usize, 14usize, 16u32, 2194744u32);
    emu.sri_no_count(14usize, 14usize, 16u32, 2194748u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2194752u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2194756u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2194760u32);
    emu.divu_no_count(12usize, 11usize, 13usize, 2194764u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2194768u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2194772u32);
    emu.orr_no_count(17usize, 16usize, 12usize, 2194776u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2194780u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2194784u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194788u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2194792u32)?;
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194796u32;
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
pub fn block_0x00217d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d78));
    } else {
        emu.pc = 2194800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d70));
    }
}
#[inline(always)]
pub fn block_0x00217d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2194804u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194812u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d7c));
}
#[inline(always)]
pub fn block_0x00217d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2194812u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217d7c));
}
#[inline(always)]
pub fn block_0x00217d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d94));
    } else {
        emu.pc = 2194816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d80));
    }
}
#[inline(always)]
pub fn block_0x00217d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2194820u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2194824u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194828u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2194832u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194836u32;
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
pub fn block_0x00217d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 1u32, 2194840u32);
    emu.sli_no_count(14usize, 14usize, 31u32, 2194844u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2194848u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2194852u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2194856u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let return_addr = 2194860u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217dc0));
}
#[inline(always)]
pub fn block_0x00217dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 5usize, 1u32, 2194864u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2194868u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2194872u32);
    emu.orr_no_count(5usize, 5usize, 6usize, 2194876u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2194880u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2194880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217dc0));
}
#[inline(always)]
pub fn block_0x00217dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 11usize, 5usize, 2194884u32);
    emu.sbr_no_count(7usize, 16usize, 14usize, 2194888u32);
    emu.sbr_no_count(6usize, 7usize, 6usize, 2194892u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217dac));
    } else {
        emu.pc = 2194896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217dd0));
    }
}
#[inline(always)]
pub fn block_0x00217dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 5usize, 2194900u32);
    emu.orr_no_count(12usize, 17usize, 12usize, 2194904u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2194916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217de4));
    } else {
        emu.pc = 2194908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ddc));
    }
}
#[inline(always)]
pub fn block_0x00217ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 6usize, 0u32, 2194912u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194860u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217dac));
}
#[inline]
pub fn block_0x00217de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(14usize, 11usize, 13usize, 2194920u32);
    emu.mul_no_count(13usize, 14usize, 13usize, 2194924u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2194928u32);
    emu.orr_no_count(17usize, 14usize, 12usize, 2194932u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2194936u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2194940u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194944u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2194948u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194952u32;
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
pub fn block_0x00217e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2194984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e28));
    } else {
        emu.pc = 2194956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e0c));
    }
}
#[inline(always)]
pub fn block_0x00217e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2194960u32);
    emu.lbu_no_count(14usize, 11usize, 0u32, 2194964u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2194992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2194992u32));
    } else {
        emu.pc = 2194968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e18));
    }
}
#[inline(always)]
pub fn block_0x00217e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2194972u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2194976u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2194980u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2194956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e0c));
    } else {
        emu.pc = 2194984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e28));
    }
}
#[inline(always)]
pub fn block_0x00217e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2194988u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194992u32;
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
