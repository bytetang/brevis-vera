pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2247440u32;
pub const PC_MAX: u32 = 2249940u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 180usize] = [
        block_0x00224b10,
        block_0x00224b24,
        block_0x00224b30,
        block_0x00224b4c,
        block_0x00224b54,
        block_0x00224b5c,
        block_0x00224b60,
        block_0x00224b70,
        block_0x00224b88,
        block_0x00224ba0,
        block_0x00224ba4,
        block_0x00224bc0,
        block_0x00224bcc,
        block_0x00224bd0,
        block_0x00224be8,
        block_0x00224c00,
        block_0x00224c18,
        block_0x00224c1c,
        block_0x00224c28,
        block_0x00224c2c,
        block_0x00224c34,
        block_0x00224c38,
        block_0x00224c4c,
        block_0x00224c7c,
        block_0x00224c80,
        block_0x00224c88,
        block_0x00224c9c,
        block_0x00224cac,
        block_0x00224cb0,
        block_0x00224cb8,
        block_0x00224cd4,
        block_0x00224cd8,
        block_0x00224cf0,
        block_0x00224cf4,
        block_0x00224d08,
        block_0x00224d10,
        block_0x00224d18,
        block_0x00224d1c,
        block_0x00224d20,
        block_0x00224d24,
        block_0x00224d38,
        block_0x00224d60,
        block_0x00224d64,
        block_0x00224d6c,
        block_0x00224d74,
        block_0x00224d88,
        block_0x00224d8c,
        block_0x00224d94,
        block_0x00224d98,
        block_0x00224dac,
        block_0x00224dd8,
        block_0x00224ddc,
        block_0x00224de4,
        block_0x00224dec,
        block_0x00224df4,
        block_0x00224df8,
        block_0x00224e00,
        block_0x00224e1c,
        block_0x00224e20,
        block_0x00224e38,
        block_0x00224e4c,
        block_0x00224e50,
        block_0x00224e58,
        block_0x00224e68,
        block_0x00224e74,
        block_0x00224e78,
        block_0x00224e84,
        block_0x00224e8c,
        block_0x00224e9c,
        block_0x00224eb8,
        block_0x00224ec0,
        block_0x00224ed4,
        block_0x00224edc,
        block_0x00224eec,
        block_0x00224f00,
        block_0x00224f14,
        block_0x00224f28,
        block_0x00224f38,
        block_0x00224f4c,
        block_0x00224f5c,
        block_0x00224f90,
        block_0x00224f94,
        block_0x00224fa0,
        block_0x00224fa4,
        block_0x00224fb4,
        block_0x00224fbc,
        block_0x00224fc0,
        block_0x00224fc4,
        block_0x00224fd8,
        block_0x00224fdc,
        block_0x00224ff4,
        block_0x00224ff8,
        block_0x00225008,
        block_0x0022503c,
        block_0x00225040,
        block_0x00225054,
        block_0x00225058,
        block_0x0022505c,
        block_0x00225068,
        block_0x0022506c,
        block_0x00225070,
        block_0x00225084,
        block_0x00225088,
        block_0x002250a0,
        block_0x002250a4,
        block_0x002250a8,
        block_0x002250b8,
        block_0x002250ec,
        block_0x002250f0,
        block_0x00225100,
        block_0x00225104,
        block_0x00225108,
        block_0x00225114,
        block_0x00225118,
        block_0x0022511c,
        block_0x0022512c,
        block_0x00225130,
        block_0x00225148,
        block_0x0022514c,
        block_0x00225150,
        block_0x00225160,
        block_0x00225194,
        block_0x00225198,
        block_0x002251a8,
        block_0x002251ac,
        block_0x002251b0,
        block_0x002251bc,
        block_0x002251c0,
        block_0x002251c4,
        block_0x002251d4,
        block_0x002251d8,
        block_0x002251f0,
        block_0x002251f4,
        block_0x002251f8,
        block_0x00225208,
        block_0x0022523c,
        block_0x00225240,
        block_0x0022524c,
        block_0x00225250,
        block_0x00225254,
        block_0x00225258,
        block_0x0022526c,
        block_0x00225274,
        block_0x00225288,
        block_0x002252b0,
        block_0x002252b4,
        block_0x002252bc,
        block_0x002252c4,
        block_0x002252d4,
        block_0x002252dc,
        block_0x002252e0,
        block_0x002252e4,
        block_0x002252fc,
        block_0x00225308,
        block_0x0022530c,
        block_0x0022531c,
        block_0x00225330,
        block_0x0022533c,
        block_0x00225340,
        block_0x00225348,
        block_0x00225358,
        block_0x0022535c,
        block_0x00225368,
        block_0x0022536c,
        block_0x00225378,
        block_0x0022537c,
        block_0x00225384,
        block_0x00225388,
        block_0x0022538c,
        block_0x002253d0,
        block_0x002253e4,
        block_0x002253fc,
        block_0x00225414,
        block_0x0022542c,
        block_0x00225448,
        block_0x00225464,
        block_0x00225480,
        block_0x0022549c,
        block_0x002254b8,
        block_0x002254d4,
    ];
    const IDX: [u16; 626usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 4u16, 0u16, 5u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16, 8u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 13u16, 14u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 17u16, 18u16, 0u16, 0u16, 19u16, 20u16, 0u16, 21u16, 22u16, 0u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 24u16, 25u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16,
        28u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 32u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16,
        36u16, 0u16, 37u16, 38u16, 39u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 43u16, 0u16, 44u16, 0u16,
        45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 48u16, 49u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        51u16, 52u16, 0u16, 53u16, 0u16, 54u16, 0u16, 55u16, 56u16, 0u16, 57u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 0u16, 0u16, 61u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16,
        0u16, 65u16, 66u16, 0u16, 0u16, 67u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16,
        72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 78u16,
        0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 83u16,
        84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16, 87u16, 88u16, 0u16, 0u16, 0u16,
        0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16, 0u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 95u16, 0u16, 0u16, 0u16, 0u16, 96u16, 97u16, 98u16, 0u16, 0u16, 99u16,
        100u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16, 103u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 104u16, 105u16, 106u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 109u16, 0u16, 0u16, 0u16,
        110u16, 111u16, 112u16, 0u16, 0u16, 113u16, 114u16, 115u16, 0u16, 0u16, 0u16,
        116u16, 117u16, 0u16, 0u16, 0u16, 0u16, 0u16, 118u16, 119u16, 120u16, 0u16, 0u16,
        0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 122u16, 123u16, 0u16, 0u16, 0u16, 124u16, 125u16, 126u16, 0u16, 0u16,
        127u16, 128u16, 129u16, 0u16, 0u16, 0u16, 130u16, 131u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 132u16, 133u16, 134u16, 0u16, 0u16, 0u16, 135u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 136u16, 137u16, 0u16, 0u16,
        138u16, 139u16, 140u16, 141u16, 0u16, 0u16, 0u16, 0u16, 142u16, 0u16, 143u16,
        0u16, 0u16, 0u16, 0u16, 144u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 145u16, 146u16, 0u16, 147u16, 0u16, 148u16, 0u16, 0u16, 0u16, 149u16, 0u16,
        150u16, 151u16, 152u16, 0u16, 0u16, 0u16, 0u16, 0u16, 153u16, 0u16, 0u16, 154u16,
        155u16, 0u16, 0u16, 0u16, 156u16, 0u16, 0u16, 0u16, 0u16, 157u16, 0u16, 0u16,
        158u16, 159u16, 0u16, 160u16, 0u16, 0u16, 0u16, 161u16, 162u16, 0u16, 0u16,
        163u16, 164u16, 0u16, 0u16, 165u16, 166u16, 0u16, 167u16, 168u16, 169u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 170u16, 0u16, 0u16, 0u16, 0u16, 171u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        172u16, 0u16, 0u16, 0u16, 0u16, 0u16, 173u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        174u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 175u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 176u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 177u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 178u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 179u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 180u16,
    ];
    if pc < 2247440u32 || pc > 2249940u32 {
        return None;
    }
    let word_offset = ((pc - 2247440u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00224b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 664u32, 2247444u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2247448u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2247452u32);
    emu.apc_no_count(1usize, 2247452u32, 4294860800u32, 2247456u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 10u32, 2247464u32);
    emu.adi_no_count(24usize, 18usize, 0u32, 2247468u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2247588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ba4));
    } else {
        emu.pc = 2247472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224b30));
    }
}
#[inline(always)]
pub fn block_0x00224b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 2usize, 660u32, 2247476u32);
    emu.adi_no_count(25usize, 0usize, 41u32, 2247480u32);
    emu.adi_no_count(26usize, 0usize, 9u32, 2247484u32);
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2247488u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 4294965760u32, 2247492u32);
    emu.adi_no_count(24usize, 18usize, 0u32, 2247496u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2247500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2247508u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224b54));
}
#[inline(always)]
pub fn block_0x00224b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 24usize, 4294967287u32, 2247504u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2247588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ba4));
    } else {
        emu.pc = 2247508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224b54));
    }
}
#[inline(always)]
pub fn block_0x00224b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 824u32, 2247512u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2249680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253d0));
    } else {
        emu.pc = 2247516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224b5c));
    }
}
#[inline(always)]
pub fn block_0x00224b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2247500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224b4c));
    } else {
        emu.pc = 2247520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224b60));
    }
}
#[inline(always)]
pub fn block_0x00224b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2247524u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2247528u32);
    emu.sbr_no_count(27usize, 0usize, 10usize, 2247532u32);
    emu.adr_no_count(23usize, 8usize, 10usize, 2247536u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2247536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224b70));
}
#[inline(always)]
pub fn block_0x00224b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 23usize, 0u32, 2247540u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2247544u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2247548u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2247552u32);
    emu.apc_no_count(1usize, 2247552u32, 4096u32, 2247556u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(11usize, 10usize, 20usize, 2247564u32);
    emu.sw_no_count(10usize, 23usize, 0u32, 2247568u32)?;
    emu.adi_no_count(27usize, 27usize, 4u32, 2247572u32);
    emu.sbr_no_count(11usize, 21usize, 11usize, 2247576u32);
    emu.adi_no_count(23usize, 23usize, 4294967292u32, 2247580u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2247536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224b70));
    } else {
        emu.pc = 2247584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ba0));
    }
}
#[inline(always)]
pub fn block_0x00224ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2247588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2247500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224b4c));
}
#[inline(always)]
pub fn block_0x00224ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 24usize, 2u32, 2247592u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2247596u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966252u32, 2247600u32);
    emu.adr_no_count(10usize, 10usize, 24usize, 2247604u32);
    emu.lw_no_count(20usize, 10usize, 0u32, 2247608u32)?;
    emu.sli_no_count(20usize, 20usize, 1u32, 2247612u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2249912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002254b8));
    } else {
        emu.pc = 2247616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224bc0));
    }
}
#[inline(always)]
pub fn block_0x00224bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 824u32, 2247620u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2247624u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2249680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253d0));
    } else {
        emu.pc = 2247628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224bcc));
    }
}
#[inline(always)]
pub fn block_0x00224bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2247708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c1c));
    } else {
        emu.pc = 2247632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224bd0));
    }
}
#[inline(always)]
pub fn block_0x00224bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2247636u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2247640u32);
    emu.adi_no_count(12usize, 2usize, 664u32, 2247644u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2247648u32);
    emu.sbr_no_count(8usize, 0usize, 10usize, 2247652u32);
    emu.adi_no_count(23usize, 12usize, 4294967292u32, 2247656u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2247656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224be8));
}
#[inline(always)]
pub fn block_0x00224be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 23usize, 0u32, 2247660u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2247664u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2247668u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2247672u32);
    emu.apc_no_count(1usize, 2247672u32, 4096u32, 2247676u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(11usize, 10usize, 20usize, 2247684u32);
    emu.sw_no_count(10usize, 23usize, 0u32, 2247688u32)?;
    emu.adi_no_count(8usize, 8usize, 4u32, 2247692u32);
    emu.sbr_no_count(11usize, 21usize, 11usize, 2247696u32);
    emu.adi_no_count(23usize, 23usize, 4294967292u32, 2247700u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2247656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224be8));
    } else {
        emu.pc = 2247704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c18));
    }
}
#[inline(always)]
pub fn block_0x00224c18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 824u32, 2247708u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2247708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224c1c));
}
#[inline(always)]
pub fn block_0x00224c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 2usize, 168u32, 2247712u32)?;
    emu.adi_no_count(13usize, 15usize, 0u32, 2247716u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2247724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c2c));
    } else {
        emu.pc = 2247720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c28));
    }
}
#[inline(always)]
pub fn block_0x00224c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2247724u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2247724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224c2c));
}
#[inline(always)]
pub fn block_0x00224c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2247728u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2249700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253e4));
    } else {
        emu.pc = 2247732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c34));
    }
}
#[inline(always)]
pub fn block_0x00224c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2247836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c9c));
    } else {
        emu.pc = 2247736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c38));
    }
}
#[inline(always)]
pub fn block_0x00224c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2247740u32);
    emu.sli_no_count(10usize, 13usize, 2u32, 2247744u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2247748u32);
    emu.adr_no_count(12usize, 11usize, 10usize, 2247752u32);
    emu.adi_no_count(14usize, 2usize, 664u32, 2247756u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2247756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224c4c));
}
#[inline]
pub fn block_0x00224c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 14usize, 0u32, 2247760u32)?;
    emu.lw_no_count(5usize, 11usize, 0u32, 2247764u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2247768u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2247772u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2247776u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2247780u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2247784u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2247788u32);
    emu.sw_no_count(16usize, 14usize, 0u32, 2247792u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2247796u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2247800u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2247756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c4c));
    } else {
        emu.pc = 2247804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c7c));
    }
}
#[inline(always)]
pub fn block_0x00224c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2247836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c9c));
    } else {
        emu.pc = 2247808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c80));
    }
}
#[inline(always)]
pub fn block_0x00224c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2247812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2250060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250060u32));
    } else {
        emu.pc = 2247816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224c88));
    }
}
#[inline(always)]
pub fn block_0x00224c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 664u32, 2247820u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2247824u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2247828u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2247832u32)?;
    emu.adi_no_count(13usize, 13usize, 1u32, 2247836u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2247836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224c9c));
}
#[inline(always)]
pub fn block_0x00224c9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 332u32, 2247840u32)?;
    emu.sw_no_count(13usize, 2usize, 824u32, 2247844u32)?;
    emu.adi_no_count(14usize, 10usize, 0u32, 2247848u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2247856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cb0));
    } else {
        emu.pc = 2247852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cac));
    }
}
#[inline(always)]
pub fn block_0x00224cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 0u32, 2247856u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2247856u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224cb0));
}
#[inline(always)]
pub fn block_0x00224cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 41u32, 2247860u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2249724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253fc));
    } else {
        emu.pc = 2247864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cb8));
    }
}
#[inline(always)]
pub fn block_0x00224cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 14usize, 2u32, 2247868u32);
    emu.adi_no_count(12usize, 2usize, 172u32, 2247872u32);
    emu.adi_no_count(13usize, 2usize, 664u32, 2247876u32);
    emu.sbr_no_count(11usize, 0usize, 14usize, 2247880u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2247884u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2247888u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2247892u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2247892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224cd4));
}
#[inline(always)]
pub fn block_0x00224cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2247964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d1c));
    } else {
        emu.pc = 2247896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cd8));
    }
}
#[inline(always)]
pub fn block_0x00224cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2247900u32)?;
    emu.lw_no_count(16usize, 12usize, 0u32, 2247904u32)?;
    emu.adi_no_count(11usize, 11usize, 4u32, 2247908u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2247912u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2247916u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2247892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cd4));
    } else {
        emu.pc = 2247920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cf0));
    }
}
#[inline(always)]
pub fn block_0x00224cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2247968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d20));
    } else {
        emu.pc = 2247924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cf4));
    }
}
#[inline(always)]
pub fn block_0x00224cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 1u32, 2247928u32);
    emu.sli_no_count(11usize, 22usize, 16u32, 2247932u32);
    emu.sai_no_count(21usize, 11usize, 1040u32, 2247936u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2247940u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2248072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d88));
    } else {
        emu.pc = 2247944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d08));
    }
}
#[inline(always)]
pub fn block_0x00224d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 21usize, 19usize, 2247948u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2248412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224edc));
    } else {
        emu.pc = 2247952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d10));
    }
}
#[inline(always)]
pub fn block_0x00224d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2247956u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2248428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224eec));
    } else {
        emu.pc = 2247960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d18));
    }
}
#[inline(always)]
pub fn block_0x00224d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2247964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2248076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224d8c));
}
#[inline(always)]
pub fn block_0x00224d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2247924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224cf4));
    } else {
        emu.pc = 2247968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d20));
    }
}
#[inline(always)]
pub fn block_0x00224d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2248052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d74));
    } else {
        emu.pc = 2247972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d24));
    }
}
#[inline(always)]
pub fn block_0x00224d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2247976u32);
    emu.sli_no_count(13usize, 15usize, 2u32, 2247980u32);
    emu.adi_no_count(16usize, 2usize, 8u32, 2247984u32);
    emu.adr_no_count(11usize, 16usize, 13usize, 2247988u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2247992u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2247992u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224d38));
}
#[inline]
pub fn block_0x00224d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2247996u32)?;
    emu.mulhu_no_count(5usize, 17usize, 14usize, 2248000u32);
    emu.mul_no_count(17usize, 17usize, 14usize, 2248004u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2248008u32);
    emu.sw_no_count(12usize, 16usize, 0u32, 2248012u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2248016u32);
    emu.sltru_no_count(12usize, 12usize, 17usize, 2248020u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2248024u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2248028u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2247992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d38));
    } else {
        emu.pc = 2248032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d60));
    }
}
#[inline(always)]
pub fn block_0x00224d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2248052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d74));
    } else {
        emu.pc = 2248036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d64));
    }
}
#[inline(always)]
pub fn block_0x00224d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2248040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2250060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250060u32));
    } else {
        emu.pc = 2248044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d6c));
    }
}
#[inline(always)]
pub fn block_0x00224d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2248048u32)?;
    emu.adi_no_count(15usize, 15usize, 1u32, 2248052u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2248052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224d74));
}
#[inline(always)]
pub fn block_0x00224d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 168u32, 2248056u32)?;
    emu.sli_no_count(11usize, 22usize, 16u32, 2248060u32);
    emu.sai_no_count(21usize, 11usize, 1040u32, 2248064u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2248068u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2247944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d08));
    } else {
        emu.pc = 2248072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d88));
    }
}
#[inline(always)]
pub fn block_0x00224d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2248076u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2248076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224d8c));
}
#[inline(always)]
pub fn block_0x00224d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2248080u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2249680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253d0));
    } else {
        emu.pc = 2248084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d94));
    }
}
#[inline(always)]
pub fn block_0x00224d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2248172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224dec));
    } else {
        emu.pc = 2248088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d98));
    }
}
#[inline(always)]
pub fn block_0x00224d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2248092u32);
    emu.sli_no_count(14usize, 10usize, 2u32, 2248096u32);
    emu.adi_no_count(17usize, 2usize, 172u32, 2248100u32);
    emu.adr_no_count(12usize, 17usize, 14usize, 2248104u32);
    emu.adi_no_count(16usize, 0usize, 5u32, 2248108u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2248108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224dac));
}
#[inline]
pub fn block_0x00224dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 17usize, 0u32, 2248112u32)?;
    emu.mulhu_no_count(6usize, 5usize, 16usize, 2248116u32);
    emu.sli_no_count(7usize, 5usize, 2u32, 2248120u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2248124u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2248128u32);
    emu.sw_no_count(13usize, 17usize, 0u32, 2248132u32)?;
    emu.adi_no_count(17usize, 17usize, 4u32, 2248136u32);
    emu.sltru_no_count(13usize, 13usize, 5usize, 2248140u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2248144u32);
    emu.adr_no_count(13usize, 6usize, 13usize, 2248148u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2248108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224dac));
    } else {
        emu.pc = 2248152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224dd8));
    }
}
#[inline(always)]
pub fn block_0x00224dd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2248172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224dec));
    } else {
        emu.pc = 2248156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ddc));
    }
}
#[inline(always)]
pub fn block_0x00224ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2248160u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2250060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250060u32));
    } else {
        emu.pc = 2248164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224de4));
    }
}
#[inline(always)]
pub fn block_0x00224de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 12usize, 0u32, 2248168u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2248172u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2248172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224dec));
}
#[inline(always)]
pub fn block_0x00224dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 332u32, 2248176u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2248184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224df8));
    } else {
        emu.pc = 2248180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224df4));
    }
}
#[inline(always)]
pub fn block_0x00224df4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 15usize, 0u32, 2248184u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2248184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224df8));
}
#[inline(always)]
pub fn block_0x00224df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2248188u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2249680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253d0));
    } else {
        emu.pc = 2248192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e00));
    }
}
#[inline(always)]
pub fn block_0x00224e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 10usize, 2u32, 2248196u32);
    emu.adi_no_count(13usize, 2usize, 172u32, 2248200u32);
    emu.adi_no_count(14usize, 2usize, 8u32, 2248204u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2248208u32);
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2248212u32);
    emu.adr_no_count(12usize, 13usize, 15usize, 2248216u32);
    emu.adr_no_count(13usize, 14usize, 15usize, 2248220u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2248220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224e1c));
}
#[inline(always)]
pub fn block_0x00224e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2248280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e58));
    } else {
        emu.pc = 2248224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e20));
    }
}
#[inline(always)]
pub fn block_0x00224e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2248228u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2248232u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2248236u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2248240u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2248244u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2248220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e1c));
    } else {
        emu.pc = 2248248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e38));
    }
}
#[inline(always)]
pub fn block_0x00224e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2248252u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2248256u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2248260u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2248264u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2248296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e68));
    } else {
        emu.pc = 2248268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e4c));
    }
}
#[inline(always)]
pub fn block_0x00224e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2249536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225340));
    } else {
        emu.pc = 2248272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e50));
    }
}
#[inline(always)]
pub fn block_0x00224e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2248276u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2248280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225388));
}
#[inline(always)]
pub fn block_0x00224e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2248284u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2248288u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2248292u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2248268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e4c));
    } else {
        emu.pc = 2248296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e68));
    }
}
#[inline(always)]
pub fn block_0x00224e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2248300u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2248304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2249604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225384));
    } else {
        emu.pc = 2248308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e74));
    }
}
#[inline(always)]
pub fn block_0x00224e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2250016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250016u32));
    } else {
        emu.pc = 2248312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e78));
    }
}
#[inline(always)]
pub fn block_0x00224e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2248316u32);
    emu.adr_no_count(23usize, 14usize, 20usize, 2248320u32);
    emu.adi_no_count(10usize, 0usize, 57u32, 2248324u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2248324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224e84));
}
#[inline(always)]
pub fn block_0x00224e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 20usize, 11usize, 2248328u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2249480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225308));
    } else {
        emu.pc = 2248332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e8c));
    }
}
#[inline(always)]
pub fn block_0x00224e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 23usize, 11usize, 2248336u32);
    emu.lbu_no_count(12usize, 12usize, 4294967295u32, 2248340u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2248344u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2248324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e84));
    } else {
        emu.pc = 2248348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e9c));
    }
}
#[inline(always)]
pub fn block_0x00224e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 11usize, 2248352u32);
    emu.lbu_no_count(10usize, 23usize, 0u32, 2248356u32);
    emu.adr_no_count(12usize, 20usize, 11usize, 2248360u32);
    emu.adi_no_count(13usize, 10usize, 1u32, 2248364u32);
    emu.adi_no_count(10usize, 12usize, 1u32, 2248368u32);
    emu.sb_no_count(13usize, 23usize, 0u32, 2248372u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2250040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250040u32));
    } else {
        emu.pc = 2248376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224eb8));
    }
}
#[inline(always)]
pub fn block_0x00224eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2248380u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2249604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225384));
    } else {
        emu.pc = 2248384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ec0));
    }
}
#[inline(always)]
pub fn block_0x00224ec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(12usize, 11usize, 4294967295u32, 2248388u32);
    emu.adi_no_count(10usize, 23usize, 1u32, 2248392u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2248396u32);
    emu.apc_no_count(1usize, 2248396u32, 4294856704u32, 2248400u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2248404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 4u32, 2248408u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2248412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225384));
}
#[inline(always)]
pub fn block_0x00224edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 22usize, 19usize, 2248416u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2248420u32);
    emu.sai_no_count(20usize, 12usize, 1040u32, 2248424u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2248076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224d8c));
    } else {
        emu.pc = 2248428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224eec));
    }
}
#[inline(always)]
pub fn block_0x00224eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 336u32, 2248432u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2248436u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2248440u32);
    emu.apc_no_count(1usize, 2248440u32, 4294856704u32, 2248444u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2248448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 336u32, 2248452u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2248456u32);
    emu.adi_no_count(27usize, 0usize, 1u32, 2248460u32);
    emu.apc_no_count(1usize, 2248460u32, 4294938624u32, 2248464u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2248468u32;
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
pub fn block_0x00224f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 500u32, 2248472u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2248476u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2248480u32);
    emu.apc_no_count(1usize, 2248480u32, 4294856704u32, 2248484u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2248488u32;
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
pub fn block_0x00224f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 500u32, 2248492u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2248496u32);
    emu.apc_no_count(1usize, 2248496u32, 4294938624u32, 2248500u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2248504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 664u32, 2248508u32);
    emu.adi_no_count(11usize, 2usize, 172u32, 2248512u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2248516u32);
    emu.apc_no_count(1usize, 2248516u32, 4294856704u32, 2248520u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2248524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 664u32, 2248528u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2248532u32);
    emu.apc_no_count(1usize, 2248532u32, 4294938624u32, 2248536u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2248540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00224f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2248544u32);
    emu.adi_no_count(12usize, 18usize, 1u32, 2248548u32);
    emu.lw_no_count(15usize, 2usize, 168u32, 2248552u32)?;
    emu.lw_no_count(16usize, 2usize, 824u32, 2248556u32)?;
    emu.lw_no_count(17usize, 2usize, 660u32, 2248560u32)?;
    emu.lw_no_count(5usize, 2usize, 496u32, 2248564u32)?;
    emu.lw_no_count(10usize, 2usize, 332u32, 2248568u32)?;
    emu.adi_no_count(1usize, 2usize, 660u32, 2248572u32);
    emu.adi_no_count(25usize, 2usize, 4u32, 2248576u32);
    emu.adi_no_count(29usize, 2usize, 332u32, 2248580u32);
    emu.adi_no_count(30usize, 2usize, 168u32, 2248584u32);
    emu.adi_no_count(31usize, 0usize, 41u32, 2248588u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2248592u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2248592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224f90));
}
#[inline(always)]
pub fn block_0x00224f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2249748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225414));
    } else {
        emu.pc = 2248596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224f94));
    }
}
#[inline(always)]
pub fn block_0x00224f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 27usize, 0u32, 2248600u32);
    emu.sli_no_count(13usize, 15usize, 2u32, 2248604u32);
    emu.adi_no_count(14usize, 2usize, 8u32, 2248608u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2248608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224fa0));
}
#[inline(always)]
pub fn block_0x00224fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2249436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252dc));
    } else {
        emu.pc = 2248612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fa4));
    }
}
#[inline(always)]
pub fn block_0x00224fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 14usize, 0u32, 2248616u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2248620u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2248624u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2248608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fa0));
    } else {
        emu.pc = 2248628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fb4));
    }
}
#[inline(always)]
pub fn block_0x00224fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 16usize, 0u32, 2248632u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2248640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fc0));
    } else {
        emu.pc = 2248636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fbc));
    }
}
#[inline(always)]
pub fn block_0x00224fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 15usize, 0u32, 2248640u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2248640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224fc0));
}
#[inline(always)]
pub fn block_0x00224fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2249700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253e4));
    } else {
        emu.pc = 2248644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fc4));
    }
}
#[inline(always)]
pub fn block_0x00224fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(27usize, 13usize, 2u32, 2248648u32);
    emu.sbr_no_count(14usize, 0usize, 27usize, 2248652u32);
    emu.adi_no_count(28usize, 1usize, 0u32, 2248656u32);
    emu.adr_no_count(8usize, 1usize, 27usize, 2248660u32);
    emu.adr_no_count(1usize, 25usize, 27usize, 2248664u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2248664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224fd8));
}
#[inline(always)]
pub fn block_0x00224fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2248792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225058));
    } else {
        emu.pc = 2248668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fdc));
    }
}
#[inline(always)]
pub fn block_0x00224fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 1usize, 0u32, 2248672u32)?;
    emu.lw_no_count(23usize, 8usize, 0u32, 2248676u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2248680u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2248684u32);
    emu.adi_no_count(1usize, 1usize, 4294967292u32, 2248688u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2248664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224fd8));
    } else {
        emu.pc = 2248692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ff4));
    }
}
#[inline(always)]
pub fn block_0x00224ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2248796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022505c));
    } else {
        emu.pc = 2248696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ff8));
    }
}
#[inline(always)]
pub fn block_0x00224ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 2usize, 664u32, 2248700u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2248704u32);
    emu.adr_no_count(15usize, 14usize, 27usize, 2248708u32);
    emu.adi_no_count(27usize, 2usize, 8u32, 2248712u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2248712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225008));
}
#[inline]
pub fn block_0x00225008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 14usize, 0u32, 2248716u32)?;
    emu.lw_no_count(7usize, 27usize, 0u32, 2248720u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2248724u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2248728u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2248732u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2248736u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2248740u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2248744u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2248748u32);
    emu.sw_no_count(8usize, 27usize, 0u32, 2248752u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2248756u32);
    emu.adi_no_count(27usize, 27usize, 4u32, 2248760u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2248712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225008));
    } else {
        emu.pc = 2248764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022503c));
    }
}
#[inline(always)]
pub fn block_0x0022503c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2249964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249964u32));
    } else {
        emu.pc = 2248768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225040));
    }
}
#[inline(always)]
pub fn block_0x00225040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 168u32, 2248772u32)?;
    emu.adi_no_count(27usize, 0usize, 8u32, 2248776u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2248780u32);
    emu.adi_no_count(13usize, 17usize, 0u32, 2248784u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2248812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022506c));
    } else {
        emu.pc = 2248788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225054));
    }
}
#[inline(always)]
pub fn block_0x00225054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2248792u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2248808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225068));
}
#[inline(always)]
pub fn block_0x00225058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2248696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ff8));
    } else {
        emu.pc = 2248796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022505c));
    }
}
#[inline(always)]
pub fn block_0x0022505c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 0u32, 2248800u32);
    emu.adi_no_count(13usize, 17usize, 0u32, 2248804u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2248812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022506c));
    } else {
        emu.pc = 2248808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225068));
    }
}
#[inline(always)]
pub fn block_0x00225068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 15usize, 0u32, 2248812u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2248812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022506c));
}
#[inline(always)]
pub fn block_0x0022506c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2249700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253e4));
    } else {
        emu.pc = 2248816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225070));
    }
}
#[inline(always)]
pub fn block_0x00225070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 13usize, 2u32, 2248820u32);
    emu.sbr_no_count(14usize, 0usize, 1usize, 2248824u32);
    emu.adi_no_count(8usize, 2usize, 496u32, 2248828u32);
    emu.adr_no_count(8usize, 8usize, 1usize, 2248832u32);
    emu.adr_no_count(7usize, 25usize, 1usize, 2248836u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2248836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225084));
}
#[inline(always)]
pub fn block_0x00225084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2248964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225104));
    } else {
        emu.pc = 2248840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225088));
    }
}
#[inline(always)]
pub fn block_0x00225088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 7usize, 0u32, 2248844u32)?;
    emu.lw_no_count(6usize, 8usize, 0u32, 2248848u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2248852u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2248856u32);
    emu.adi_no_count(7usize, 7usize, 4294967292u32, 2248860u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2248836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225084));
    } else {
        emu.pc = 2248864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250a0));
    }
}
#[inline(always)]
pub fn block_0x002250a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2248968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225108));
    } else {
        emu.pc = 2248868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250a4));
    }
}
#[inline(always)]
pub fn block_0x002250a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2248944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250f0));
    } else {
        emu.pc = 2248872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250a8));
    }
}
#[inline(always)]
pub fn block_0x002250a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 2usize, 500u32, 2248876u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2248880u32);
    emu.adr_no_count(1usize, 14usize, 1usize, 2248884u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2248888u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2248888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002250b8));
}
#[inline]
pub fn block_0x002250b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 14usize, 0u32, 2248892u32)?;
    emu.lw_no_count(7usize, 15usize, 0u32, 2248896u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2248900u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2248904u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2248908u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2248912u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2248916u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2248920u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2248924u32);
    emu.sw_no_count(8usize, 15usize, 0u32, 2248928u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2248932u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2248936u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2248888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250b8));
    } else {
        emu.pc = 2248940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250ec));
    }
}
#[inline(always)]
pub fn block_0x002250ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2249964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249964u32));
    } else {
        emu.pc = 2248944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250f0));
    }
}
#[inline(always)]
pub fn block_0x002250f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 168u32, 2248948u32)?;
    emu.ori_no_count(27usize, 27usize, 4u32, 2248952u32);
    emu.adi_no_count(14usize, 5usize, 0u32, 2248956u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2248984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225118));
    } else {
        emu.pc = 2248960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225100));
    }
}
#[inline(always)]
pub fn block_0x00225100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2248964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2248980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225114));
}
#[inline(always)]
pub fn block_0x00225104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2248868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002250a4));
    } else {
        emu.pc = 2248968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225108));
    }
}
#[inline(always)]
pub fn block_0x00225108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 15usize, 0u32, 2248972u32);
    emu.adi_no_count(14usize, 5usize, 0u32, 2248976u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2248984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225118));
    } else {
        emu.pc = 2248980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225114));
    }
}
#[inline(always)]
pub fn block_0x00225114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 0u32, 2248984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2248984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225118));
}
#[inline(always)]
pub fn block_0x00225118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2249724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002253fc));
    } else {
        emu.pc = 2248988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022511c));
    }
}
#[inline(always)]
pub fn block_0x0022511c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 14usize, 2u32, 2248992u32);
    emu.sbr_no_count(8usize, 0usize, 15usize, 2248996u32);
    emu.adr_no_count(1usize, 29usize, 15usize, 2249000u32);
    emu.adr_no_count(7usize, 25usize, 15usize, 2249004u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2249004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022512c));
}
#[inline(always)]
pub fn block_0x0022512c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2249132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251ac));
    } else {
        emu.pc = 2249008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225130));
    }
}
#[inline(always)]
pub fn block_0x00225130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 7usize, 0u32, 2249012u32)?;
    emu.lw_no_count(6usize, 1usize, 0u32, 2249016u32)?;
    emu.adi_no_count(8usize, 8usize, 4u32, 2249020u32);
    emu.adi_no_count(1usize, 1usize, 4294967292u32, 2249024u32);
    emu.adi_no_count(7usize, 7usize, 4294967292u32, 2249028u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2249004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022512c));
    } else {
        emu.pc = 2249032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225148));
    }
}
#[inline(always)]
pub fn block_0x00225148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2249136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251b0));
    } else {
        emu.pc = 2249036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022514c));
    }
}
#[inline(always)]
pub fn block_0x0022514c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2249112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225198));
    } else {
        emu.pc = 2249040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225150));
    }
}
#[inline(always)]
pub fn block_0x00225150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 336u32, 2249044u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2249048u32);
    emu.adr_no_count(15usize, 13usize, 15usize, 2249052u32);
    emu.adi_no_count(1usize, 2usize, 8u32, 2249056u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2249056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225160));
}
#[inline]
pub fn block_0x00225160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 13usize, 0u32, 2249060u32)?;
    emu.lw_no_count(7usize, 1usize, 0u32, 2249064u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2249068u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2249072u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2249076u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2249080u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2249084u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2249088u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2249092u32);
    emu.sw_no_count(8usize, 1usize, 0u32, 2249096u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2249100u32);
    emu.adi_no_count(1usize, 1usize, 4u32, 2249104u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2249056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225160));
    } else {
        emu.pc = 2249108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225194));
    }
}
#[inline(always)]
pub fn block_0x00225194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2249964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249964u32));
    } else {
        emu.pc = 2249112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225198));
    }
}
#[inline(always)]
pub fn block_0x00225198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 168u32, 2249116u32)?;
    emu.adi_no_count(27usize, 27usize, 2u32, 2249120u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2249124u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2249152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251c0));
    } else {
        emu.pc = 2249128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251a8));
    }
}
#[inline(always)]
pub fn block_0x002251a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2249132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249148u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002251bc));
}
#[inline(always)]
pub fn block_0x002251ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2249036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022514c));
    } else {
        emu.pc = 2249136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251b0));
    }
}
#[inline(always)]
pub fn block_0x002251b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 0u32, 2249140u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2249144u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2249152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251c0));
    } else {
        emu.pc = 2249148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251bc));
    }
}
#[inline(always)]
pub fn block_0x002251bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2249152u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2249152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002251c0));
}
#[inline(always)]
pub fn block_0x002251c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2249748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225414));
    } else {
        emu.pc = 2249156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251c4));
    }
}
#[inline(always)]
pub fn block_0x002251c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 15usize, 2u32, 2249160u32);
    emu.sbr_no_count(13usize, 0usize, 1usize, 2249164u32);
    emu.adr_no_count(8usize, 30usize, 1usize, 2249168u32);
    emu.adr_no_count(7usize, 25usize, 1usize, 2249172u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2249172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002251d4));
}
#[inline(always)]
pub fn block_0x002251d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2249292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022524c));
    } else {
        emu.pc = 2249176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251d8));
    }
}
#[inline(always)]
pub fn block_0x002251d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 7usize, 0u32, 2249180u32)?;
    emu.lw_no_count(6usize, 8usize, 0u32, 2249184u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2249188u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2249192u32);
    emu.adi_no_count(7usize, 7usize, 4294967292u32, 2249196u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2249172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251d4));
    } else {
        emu.pc = 2249200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251f0));
    }
}
#[inline(always)]
pub fn block_0x002251f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2249296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225250));
    } else {
        emu.pc = 2249204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251f4));
    }
}
#[inline(always)]
pub fn block_0x002251f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2249280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225240));
    } else {
        emu.pc = 2249208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251f8));
    }
}
#[inline(always)]
pub fn block_0x002251f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 172u32, 2249212u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2249216u32);
    emu.adr_no_count(1usize, 13usize, 1usize, 2249220u32);
    emu.adi_no_count(14usize, 2usize, 8u32, 2249224u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2249224u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225208));
}
#[inline]
pub fn block_0x00225208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 13usize, 0u32, 2249228u32)?;
    emu.lw_no_count(7usize, 14usize, 0u32, 2249232u32)?;
    emu.ani_no_count(8usize, 8usize, 1u32, 2249236u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2249240u32);
    emu.xri_no_count(6usize, 6usize, 4294967295u32, 2249244u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2249248u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2249252u32);
    emu.adr_no_count(8usize, 6usize, 8usize, 2249256u32);
    emu.sltru_no_count(6usize, 8usize, 6usize, 2249260u32);
    emu.sw_no_count(8usize, 14usize, 0u32, 2249264u32)?;
    emu.orr_no_count(8usize, 7usize, 6usize, 2249268u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2249272u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2249224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225208));
    } else {
        emu.pc = 2249276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022523c));
    }
}
#[inline(always)]
pub fn block_0x0022523c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2249964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249964u32));
    } else {
        emu.pc = 2249280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225240));
    }
}
#[inline(always)]
pub fn block_0x00225240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 168u32, 2249284u32)?;
    emu.adi_no_count(27usize, 27usize, 1u32, 2249288u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2249292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225254));
}
#[inline(always)]
pub fn block_0x0022524c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2249204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002251f4));
    } else {
        emu.pc = 2249296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225250));
    }
}
#[inline(always)]
pub fn block_0x00225250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2249300u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2249300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225254));
}
#[inline(always)]
pub fn block_0x00225254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2250084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250084u32));
    } else {
        emu.pc = 2249304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225258));
    }
}
#[inline(always)]
pub fn block_0x00225258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 27usize, 48u32, 2249308u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2249312u32)?;
    emu.adr_no_count(14usize, 14usize, 11usize, 2249316u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2249320u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2249748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225414));
    } else {
        emu.pc = 2249324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022526c));
    }
}
#[inline(always)]
pub fn block_0x0022526c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 28usize, 0u32, 2249328u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2249412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252c4));
    } else {
        emu.pc = 2249332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225274));
    }
}
#[inline(always)]
pub fn block_0x00225274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2249336u32);
    emu.sli_no_count(8usize, 15usize, 2u32, 2249340u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2249344u32);
    emu.adr_no_count(13usize, 13usize, 8usize, 2249348u32);
    emu.adi_no_count(7usize, 2usize, 8u32, 2249352u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2249352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225288));
}
#[inline]
pub fn block_0x00225288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 7usize, 0u32, 2249356u32)?;
    emu.mulhu_no_count(23usize, 6usize, 24usize, 2249360u32);
    emu.mul_no_count(6usize, 6usize, 24usize, 2249364u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2249368u32);
    emu.sw_no_count(14usize, 7usize, 0u32, 2249372u32)?;
    emu.adi_no_count(7usize, 7usize, 4u32, 2249376u32);
    emu.sltru_no_count(14usize, 14usize, 6usize, 2249380u32);
    emu.adi_no_count(8usize, 8usize, 4294967292u32, 2249384u32);
    emu.adr_no_count(14usize, 23usize, 14usize, 2249388u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2249352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225288));
    } else {
        emu.pc = 2249392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252b0));
    }
}
#[inline(always)]
pub fn block_0x002252b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2249412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252c4));
    } else {
        emu.pc = 2249396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252b4));
    }
}
#[inline(always)]
pub fn block_0x002252b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 40u32, 2249400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2250060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250060u32));
    } else {
        emu.pc = 2249404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252bc));
    }
}
#[inline(always)]
pub fn block_0x002252bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 13usize, 0u32, 2249408u32)?;
    emu.adi_no_count(15usize, 15usize, 1u32, 2249412u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2249412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002252c4));
}
#[inline(always)]
pub fn block_0x002252c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 168u32, 2249416u32)?;
    emu.adi_no_count(27usize, 26usize, 1u32, 2249420u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2249424u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2248592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224f90));
    } else {
        emu.pc = 2249428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252d4));
    }
}
#[inline(always)]
pub fn block_0x002252d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2249432u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2249436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2248076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224d8c));
}
#[inline(always)]
pub fn block_0x002252dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2249992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249992u32));
    } else {
        emu.pc = 2249440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252e0));
    }
}
#[inline(always)]
pub fn block_0x002252e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2249468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252fc));
    } else {
        emu.pc = 2249444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002252e4));
    }
}
#[inline(always)]
pub fn block_0x002252e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2249448u32)?;
    emu.adr_no_count(10usize, 10usize, 11usize, 2249452u32);
    emu.sbr_no_count(12usize, 20usize, 11usize, 2249456u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2249460u32);
    emu.apc_no_count(1usize, 2249460u32, 4294856704u32, 2249464u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002252fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2249472u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2249476u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2249480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022538c));
}
#[inline(always)]
pub fn block_0x00225308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2249564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022535c));
    } else {
        emu.pc = 2249484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022530c));
    }
}
#[inline(always)]
pub fn block_0x0022530c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 49u32, 2249488u32);
    emu.adi_no_count(12usize, 20usize, 4294967295u32, 2249492u32);
    emu.sb_no_count(10usize, 14usize, 0u32, 2249496u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2249580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022536c));
    } else {
        emu.pc = 2249500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022531c));
    }
}
#[inline(always)]
pub fn block_0x0022531c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 1u32, 2249504u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2249508u32);
    emu.adi_no_count(8usize, 0usize, 48u32, 2249512u32);
    emu.apc_no_count(1usize, 2249512u32, 4294856704u32, 2249516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 4u32, 2249524u32)?;
    emu.adi_no_count(22usize, 22usize, 1u32, 2249528u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2249592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225378));
    } else {
        emu.pc = 2249532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022533c));
    }
}
#[inline(always)]
pub fn block_0x0022533c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2249536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225384));
}
#[inline(always)]
pub fn block_0x00225340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 4294967295u32, 2249540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2250108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2250108u32));
    } else {
        emu.pc = 2249544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225348));
    }
}
#[inline(always)]
pub fn block_0x00225348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 14usize, 10usize, 2249548u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2249552u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2249556u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2248308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224e74));
    } else {
        emu.pc = 2249560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225358));
    }
}
#[inline(always)]
pub fn block_0x00225358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2249564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225384));
}
#[inline(always)]
pub fn block_0x0022535c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 49u32, 2249568u32);
    emu.adi_no_count(22usize, 22usize, 1u32, 2249572u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2249592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225378));
    } else {
        emu.pc = 2249576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225368));
    }
}
#[inline(always)]
pub fn block_0x00225368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2249580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2249604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225384));
}
#[inline(always)]
pub fn block_0x0022536c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 48u32, 2249584u32);
    emu.adi_no_count(22usize, 22usize, 1u32, 2249588u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2249604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225384));
    } else {
        emu.pc = 2249592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225378));
    }
}
#[inline(always)]
pub fn block_0x00225378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2249604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225384));
    } else {
        emu.pc = 2249596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022537c));
    }
}
#[inline(always)]
pub fn block_0x0022537c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 23usize, 0u32, 2249600u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2249604u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2249604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00225384));
}
#[inline(always)]
pub fn block_0x00225384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2249940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002254d4));
    } else {
        emu.pc = 2249608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00225388));
    }
}
#[inline(always)]
pub fn block_0x00225388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 9usize, 0u32, 2249612u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2249612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022538c));
}
#[inline]
pub fn block_0x0022538c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 9usize, 4u32, 2249616u32)?;
    emu.sh_no_count(22usize, 9usize, 8u32, 2249620u32)?;
    emu.lw_no_count(1usize, 2usize, 876u32, 2249624u32)?;
    emu.lw_no_count(8usize, 2usize, 872u32, 2249628u32)?;
    emu.lw_no_count(9usize, 2usize, 868u32, 2249632u32)?;
    emu.lw_no_count(18usize, 2usize, 864u32, 2249636u32)?;
    emu.lw_no_count(19usize, 2usize, 860u32, 2249640u32)?;
    emu.lw_no_count(20usize, 2usize, 856u32, 2249644u32)?;
    emu.lw_no_count(21usize, 2usize, 852u32, 2249648u32)?;
    emu.lw_no_count(22usize, 2usize, 848u32, 2249652u32)?;
    emu.lw_no_count(23usize, 2usize, 844u32, 2249656u32)?;
    emu.lw_no_count(24usize, 2usize, 840u32, 2249660u32)?;
    emu.lw_no_count(25usize, 2usize, 836u32, 2249664u32)?;
    emu.lw_no_count(26usize, 2usize, 832u32, 2249668u32)?;
    emu.lw_no_count(27usize, 2usize, 828u32, 2249672u32)?;
    emu.adi_no_count(2usize, 2usize, 880u32, 2249676u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249680u32;
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
pub fn block_0x002253d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249684u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2249688u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2249692u32);
    emu.apc_no_count(1usize, 2249692u32, 0u32, 2249696u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002253e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249704u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2249708u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2249712u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2249716u32);
    emu.apc_no_count(1usize, 2249716u32, 0u32, 2249720u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249724u32;
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
pub fn block_0x002253fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249728u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2249732u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2249736u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2249740u32);
    emu.apc_no_count(1usize, 2249740u32, 0u32, 2249744u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249752u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2249756u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2249760u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2249764u32);
    emu.apc_no_count(1usize, 2249764u32, 0u32, 2249768u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0022542c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2249776u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966448u32, 2249780u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249784u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966840u32, 2249788u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2249792u32);
    emu.apc_no_count(1usize, 2249792u32, 4294934528u32, 2249796u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2249804u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966492u32, 2249808u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249812u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966856u32, 2249816u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2249820u32);
    emu.apc_no_count(1usize, 2249820u32, 4294934528u32, 2249824u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2249832u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966540u32, 2249836u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249840u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2249844u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2249848u32);
    emu.apc_no_count(1usize, 2249848u32, 4294934528u32, 2249852u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00225480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2249860u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966768u32, 2249864u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249868u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966984u32, 2249872u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2249876u32);
    emu.apc_no_count(1usize, 2249876u32, 4294934528u32, 2249880u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249884u32;
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
pub fn block_0x0022549c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2249888u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966696u32, 2249892u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249896u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966968u32, 2249900u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2249904u32);
    emu.apc_no_count(1usize, 2249904u32, 4294934528u32, 2249908u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002254b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2249916u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1699u32, 2249920u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249924u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2249928u32);
    emu.adi_no_count(11usize, 0usize, 27u32, 2249932u32);
    emu.apc_no_count(1usize, 2249932u32, 4294934528u32, 2249936u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002254d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2249944u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966920u32, 2249948u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2249952u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2249956u32);
    emu.apc_no_count(1usize, 2249956u32, 0u32, 2249960u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2249964u32;
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
