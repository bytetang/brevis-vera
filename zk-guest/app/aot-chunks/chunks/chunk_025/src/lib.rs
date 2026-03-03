pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2211260u32;
pub const PC_MAX: u32 = 2214948u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 180usize] = [
        block_0x0021bdbc,
        block_0x0021bdc4,
        block_0x0021bdd4,
        block_0x0021bdd8,
        block_0x0021bdf4,
        block_0x0021be1c,
        block_0x0021be28,
        block_0x0021be3c,
        block_0x0021be44,
        block_0x0021be4c,
        block_0x0021be54,
        block_0x0021be58,
        block_0x0021be60,
        block_0x0021be64,
        block_0x0021be74,
        block_0x0021be78,
        block_0x0021be7c,
        block_0x0021be94,
        block_0x0021be98,
        block_0x0021bea0,
        block_0x0021bea4,
        block_0x0021beac,
        block_0x0021bec0,
        block_0x0021bec4,
        block_0x0021bee8,
        block_0x0021beec,
        block_0x0021bf20,
        block_0x0021bf48,
        block_0x0021bf78,
        block_0x0021bf8c,
        block_0x0021bfb4,
        block_0x0021bfcc,
        block_0x0021bfd8,
        block_0x0021c004,
        block_0x0021c00c,
        block_0x0021c010,
        block_0x0021c018,
        block_0x0021c024,
        block_0x0021c034,
        block_0x0021c044,
        block_0x0021c04c,
        block_0x0021c064,
        block_0x0021c080,
        block_0x0021c084,
        block_0x0021c0a0,
        block_0x0021c0a8,
        block_0x0021c0d4,
        block_0x0021c10c,
        block_0x0021c138,
        block_0x0021c178,
        block_0x0021c17c,
        block_0x0021c194,
        block_0x0021c19c,
        block_0x0021c1a8,
        block_0x0021c1bc,
        block_0x0021c1c0,
        block_0x0021c1cc,
        block_0x0021c1e8,
        block_0x0021c338,
        block_0x0021c48c,
        block_0x0021c490,
        block_0x0021c49c,
        block_0x0021c4a8,
        block_0x0021c4b8,
        block_0x0021c4cc,
        block_0x0021c4dc,
        block_0x0021c4e0,
        block_0x0021c4e8,
        block_0x0021c514,
        block_0x0021c520,
        block_0x0021c574,
        block_0x0021c594,
        block_0x0021c598,
        block_0x0021c59c,
        block_0x0021c5a4,
        block_0x0021c5b0,
        block_0x0021c5c0,
        block_0x0021c5c8,
        block_0x0021c5d4,
        block_0x0021c5d8,
        block_0x0021c5e0,
        block_0x0021c5f0,
        block_0x0021c604,
        block_0x0021c630,
        block_0x0021c638,
        block_0x0021c63c,
        block_0x0021c640,
        block_0x0021c648,
        block_0x0021c650,
        block_0x0021c658,
        block_0x0021c670,
        block_0x0021c678,
        block_0x0021c680,
        block_0x0021c688,
        block_0x0021c690,
        block_0x0021c698,
        block_0x0021c6a0,
        block_0x0021c6a8,
        block_0x0021c6b4,
        block_0x0021c6c4,
        block_0x0021c6cc,
        block_0x0021c6dc,
        block_0x0021c6e4,
        block_0x0021c6fc,
        block_0x0021c704,
        block_0x0021c708,
        block_0x0021c714,
        block_0x0021c724,
        block_0x0021c72c,
        block_0x0021c734,
        block_0x0021c740,
        block_0x0021c754,
        block_0x0021c75c,
        block_0x0021c768,
        block_0x0021c778,
        block_0x0021c780,
        block_0x0021c790,
        block_0x0021c798,
        block_0x0021c7b4,
        block_0x0021c7c8,
        block_0x0021c7d8,
        block_0x0021c7e0,
        block_0x0021c804,
        block_0x0021c80c,
        block_0x0021c814,
        block_0x0021c824,
        block_0x0021c850,
        block_0x0021c854,
        block_0x0021c860,
        block_0x0021c870,
        block_0x0021c87c,
        block_0x0021c88c,
        block_0x0021c894,
        block_0x0021c8d0,
        block_0x0021c8d4,
        block_0x0021c8e8,
        block_0x0021c8f8,
        block_0x0021c900,
        block_0x0021c924,
        block_0x0021c92c,
        block_0x0021c934,
        block_0x0021c944,
        block_0x0021c970,
        block_0x0021c974,
        block_0x0021c980,
        block_0x0021c990,
        block_0x0021c998,
        block_0x0021c9a0,
        block_0x0021c9dc,
        block_0x0021c9e0,
        block_0x0021c9f4,
        block_0x0021ca04,
        block_0x0021ca0c,
        block_0x0021ca14,
        block_0x0021ca28,
        block_0x0021ca3c,
        block_0x0021ca40,
        block_0x0021ca60,
        block_0x0021ca78,
        block_0x0021ca94,
        block_0x0021caa8,
        block_0x0021cab8,
        block_0x0021cabc,
        block_0x0021cad0,
        block_0x0021cae0,
        block_0x0021cae8,
        block_0x0021cb0c,
        block_0x0021cb14,
        block_0x0021cb1c,
        block_0x0021cb24,
        block_0x0021cb58,
        block_0x0021cb5c,
        block_0x0021cb68,
        block_0x0021cb78,
        block_0x0021cb84,
        block_0x0021cb90,
        block_0x0021cbac,
        block_0x0021cbb4,
        block_0x0021cbe0,
        block_0x0021cc24,
    ];
    const IDX: [u16; 923usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 9u16, 0u16, 10u16, 0u16, 11u16,
        12u16, 0u16, 13u16, 14u16, 0u16, 0u16, 0u16, 15u16, 16u16, 17u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 20u16, 21u16, 0u16, 22u16, 0u16, 0u16,
        0u16, 0u16, 23u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16,
        26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 36u16, 0u16, 37u16, 0u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 55u16,
        56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16,
        0u16, 62u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16,
        65u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 73u16,
        74u16, 0u16, 75u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16,
        0u16, 0u16, 79u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16,
        0u16, 85u16, 86u16, 87u16, 0u16, 88u16, 0u16, 89u16, 0u16, 90u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 0u16, 94u16, 0u16, 95u16,
        0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16,
        100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 104u16, 0u16, 105u16, 106u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16,
        108u16, 0u16, 109u16, 0u16, 110u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16,
        112u16, 0u16, 113u16, 0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 115u16, 0u16, 116u16,
        0u16, 0u16, 0u16, 117u16, 0u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        119u16, 0u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 121u16, 0u16, 122u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16, 0u16, 124u16, 0u16,
        125u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 127u16, 128u16, 0u16, 0u16, 129u16, 0u16, 0u16, 0u16, 130u16, 0u16,
        0u16, 131u16, 0u16, 0u16, 0u16, 132u16, 0u16, 133u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 134u16, 135u16, 0u16,
        0u16, 0u16, 0u16, 136u16, 0u16, 0u16, 0u16, 137u16, 0u16, 138u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 139u16, 0u16, 140u16, 0u16, 141u16, 0u16,
        0u16, 0u16, 142u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        143u16, 144u16, 0u16, 0u16, 145u16, 0u16, 0u16, 0u16, 146u16, 0u16, 147u16, 0u16,
        148u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 149u16, 150u16, 0u16, 0u16, 0u16, 0u16, 151u16, 0u16, 0u16, 0u16,
        152u16, 0u16, 153u16, 0u16, 154u16, 0u16, 0u16, 0u16, 0u16, 155u16, 0u16, 0u16,
        0u16, 0u16, 156u16, 157u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 158u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 159u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 160u16,
        0u16, 0u16, 0u16, 0u16, 161u16, 0u16, 0u16, 0u16, 162u16, 163u16, 0u16, 0u16,
        0u16, 0u16, 164u16, 0u16, 0u16, 0u16, 165u16, 0u16, 166u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 167u16, 0u16, 168u16, 0u16, 169u16, 0u16, 170u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 171u16,
        172u16, 0u16, 0u16, 173u16, 0u16, 0u16, 0u16, 174u16, 0u16, 0u16, 175u16, 0u16,
        0u16, 176u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 177u16, 0u16, 178u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 179u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        180u16,
    ];
    if pc < 2211260u32 || pc > 2214948u32 {
        return None;
    }
    let word_offset = ((pc - 2211260u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021bdbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2211264u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2211284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdd4));
    } else {
        emu.pc = 2211268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bdc4));
    }
}
#[inline(always)]
pub fn block_0x0021bdc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2211272u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2211276u32);
    emu.apc_no_count(6usize, 2211276u32, 4294860800u32, 2211280u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2211284u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bdd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211288u32;
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
pub fn block_0x0021bdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2211292u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966992u32, 2211296u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2211300u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2211304u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2211308u32);
    emu.apc_no_count(6usize, 2211308u32, 24576u32, 2211312u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2211316u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021bdf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2211320u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2211324u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2211328u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2211332u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2211336u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2211340u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2211344u32);
    emu.lw_no_count(11usize, 11usize, 4u32, 2211348u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2211352u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2211416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be58));
    } else {
        emu.pc = 2211356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be1c));
    }
}
#[inline(always)]
pub fn block_0x0021be1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2211360u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2211364u32)?;
    emu.adi_no_count(12usize, 10usize, 4u32, 2211368u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2211368u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be28));
}
#[inline(always)]
pub fn block_0x0021be28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2211372u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2211376u32);
    emu.adr_no_count(18usize, 13usize, 18usize, 2211380u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2211384u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2211368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be28));
    } else {
        emu.pc = 2211388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be3c));
    }
}
#[inline(always)]
pub fn block_0x0021be3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 12u32, 2211392u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2211444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be74));
    } else {
        emu.pc = 2211396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be44));
    }
}
#[inline(always)]
pub fn block_0x0021be44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2211400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2211428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be64));
    } else {
        emu.pc = 2211404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be4c));
    }
}
#[inline(always)]
pub fn block_0x0021be4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2211408u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2211476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be94));
    } else {
        emu.pc = 2211412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be54));
    }
}
#[inline(always)]
pub fn block_0x0021be54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2211416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be64));
}
#[inline(always)]
pub fn block_0x0021be58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 12u32, 2211420u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2211476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be94));
    } else {
        emu.pc = 2211424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be60));
    }
}
#[inline(always)]
pub fn block_0x0021be60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2211428u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2211428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be64));
}
#[inline(always)]
pub fn block_0x0021be64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltr_no_count(10usize, 0usize, 18usize, 2211432u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2211436u32);
    emu.anr_no_count(18usize, 10usize, 18usize, 2211440u32);
    emu.sli_no_count(18usize, 18usize, 1u32, 2211444u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2211444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be74));
}
#[inline(always)]
pub fn block_0x0021be74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2211488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bea0));
    } else {
        emu.pc = 2211448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be78));
    }
}
#[inline(always)]
pub fn block_0x0021be78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2211452u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2211452u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be7c));
}
#[inline(always)]
pub fn block_0x0021be7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2211456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967024u32, 2211460u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2211464u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2211468u32);
    emu.apc_no_count(1usize, 2211468u32, 0u32, 2211472u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021be94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2211480u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2211480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021be98));
}
#[inline(always)]
pub fn block_0x0021be98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2211484u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bec4));
}
#[inline(always)]
pub fn block_0x0021bea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2211480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be98));
    } else {
        emu.pc = 2211492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bea4));
    }
}
#[inline(always)]
pub fn block_0x0021bea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2211492u32, 4294893568u32, 2211496u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965928u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021beac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2211504u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2211508u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2211512u32);
    emu.apc_no_count(1usize, 2211512u32, 4294860800u32, 2211516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021be7c));
    } else {
        emu.pc = 2211524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bec4));
    }
}
#[inline]
pub fn block_0x0021bec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 12u32, 2211528u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2211532u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2211536u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2211540u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967000u32, 2211544u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2211548u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2211552u32);
    emu.apc_no_count(1usize, 2211552u32, 20480u32, 2211556u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf20));
    } else {
        emu.pc = 2211564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021beec));
    }
}
#[inline]
pub fn block_0x0021beec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2211568u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2211572u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2211576u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2211580u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2211584u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2211588u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2211592u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2211596u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2211600u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2211604u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2211608u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2211612u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211616u32;
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
pub fn block_0x0021bf20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2211620u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967056u32, 2211624u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2211628u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967040u32, 2211632u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2211636u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967144u32, 2211640u32);
    emu.adi_no_count(11usize, 0usize, 86u32, 2211644u32);
    emu.adi_no_count(12usize, 2usize, 27u32, 2211648u32);
    emu.apc_no_count(1usize, 2211648u32, 20480u32, 2211652u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021bf48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2211660u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2211664u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2211668u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2211672u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2211676u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2211680u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2211684u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2211688u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2211692u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2211696u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2211700u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2211764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bfb4));
    } else {
        emu.pc = 2211704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bf78));
    }
}
#[inline(always)]
pub fn block_0x0021bf78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2211708u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2211712u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2211716u32);
    emu.apc_no_count(1usize, 2211716u32, 4294893568u32, 2211720u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021bf8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2211728u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2211732u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2211736u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2211740u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2211744u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2211748u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2211752u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2211756u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2211760u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211764u32;
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
pub fn block_0x0021bfb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2211768u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2211772u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2211776u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2211780u32);
    emu.apc_no_count(1usize, 2211780u32, 0u32, 2211784u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211788u32;
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
pub fn block_0x0021bfcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2211792u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2211796u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2211800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bf78));
}
#[inline]
pub fn block_0x0021bfd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2211804u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2211808u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2211812u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2211816u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2211820u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2211824u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2211828u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2211832u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2211836u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2211840u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2211852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c00c));
    } else {
        emu.pc = 2211844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c004));
    }
}
#[inline(always)]
pub fn block_0x0021c004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2211848u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211876u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c024));
}
#[inline(always)]
pub fn block_0x0021c00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2211864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c018));
    } else {
        emu.pc = 2211856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c010));
    }
}
#[inline(always)]
pub fn block_0x0021c010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2211860u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211876u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c024));
}
#[inline(always)]
pub fn block_0x0021c018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2211868u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2211872u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2211876u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2211876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c024));
}
#[inline(always)]
pub fn block_0x0021c024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2211880u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2211884u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2211888u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2211916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c04c));
    } else {
        emu.pc = 2211892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c034));
    }
}
#[inline(always)]
pub fn block_0x0021c034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2211896u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2211900u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2211904u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2211968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c080));
    } else {
        emu.pc = 2211908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c044));
    }
}
#[inline(always)]
pub fn block_0x0021c044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2211912u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2212108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c10c));
}
#[inline(always)]
pub fn block_0x0021c04c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2211920u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2211924u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2211928u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2211932u32);
    emu.apc_no_count(1usize, 2211932u32, 0u32, 2211936u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2211944u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2211948u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2211952u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2211956u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2211960u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2211964u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2211908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c044));
    } else {
        emu.pc = 2211968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c080));
    }
}
#[inline(always)]
pub fn block_0x0021c080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2212000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c0a0));
    } else {
        emu.pc = 2211972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c084));
    }
}
#[inline(always)]
pub fn block_0x0021c084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2211976u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2211980u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2211984u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2211988u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2211992u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2211996u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2212000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2212108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c10c));
}
#[inline(always)]
pub fn block_0x0021c0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2212004u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2212052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c0d4));
    } else {
        emu.pc = 2212008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c0a8));
    }
}
#[inline]
pub fn block_0x0021c0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2212012u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2212016u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2212020u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2212024u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2212028u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2212032u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2212036u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2212040u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2212044u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2212048u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2212052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2212108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c10c));
}
#[inline]
pub fn block_0x0021c0d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2212056u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2212060u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2212064u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2212068u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2212072u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2212076u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2212080u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2212084u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2212088u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2212092u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2212096u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2212100u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2212104u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2212108u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2212108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c10c));
}
#[inline]
pub fn block_0x0021c10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2212112u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2212116u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2212120u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2212124u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2212128u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2212132u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2212136u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2212140u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2212144u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2212148u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212152u32;
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
pub fn block_0x0021c138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2212156u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2212160u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2212164u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2212168u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2212172u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2212176u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2212180u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2212184u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2212188u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2212192u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2212196u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2212200u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2212204u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2212208u32)?;
    emu.adi_no_count(20usize, 12usize, 0u32, 2212212u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2212244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c194));
    } else {
        emu.pc = 2212216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c178));
    }
}
#[inline(always)]
pub fn block_0x0021c178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2212220u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2212220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c17c));
}
#[inline(always)]
pub fn block_0x0021c17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2212224u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967160u32, 2212228u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2212232u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2212236u32);
    emu.apc_no_count(1usize, 2212236u32, 0u32, 2212240u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2212248u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2213020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c49c));
    } else {
        emu.pc = 2212252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c19c));
    }
}
#[inline(always)]
pub fn block_0x0021c19c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2212256u32);
    emu.apc_no_count(1usize, 2212256u32, 4294889472u32, 2212260u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212264u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c1a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2212268u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2212272u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2212276u32);
    emu.apc_no_count(1usize, 2212276u32, 4294860800u32, 2212280u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2212284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2212220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c17c));
    } else {
        emu.pc = 2212288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1c0));
    }
}
#[inline(always)]
pub fn block_0x0021c1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2212292u32);
    emu.adi_no_count(10usize, 0usize, 16u32, 2212296u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2213032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4a8));
    } else {
        emu.pc = 2212300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1cc));
    }
}
#[inline(always)]
pub fn block_0x0021c1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 16u32, 2212304u32)?;
    emu.adi_no_count(13usize, 0usize, 0u32, 2212308u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2212312u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2212316u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967280u32, 2212320u32);
    emu.anr_no_count(10usize, 20usize, 10usize, 2212324u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2212328u32)?;
    emu.add_memory_rw_events(7usize);
    emu.pc = 2212328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c1e8));
}
#[inline(never)]
pub fn block_0x0021c1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 84u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 9usize, 11usize, 2212332u32);
    emu.lbu_no_count(26usize, 27usize, 0u32, 2212336u32);
    emu.lbu_no_count(25usize, 27usize, 1u32, 2212340u32);
    emu.lbu_no_count(24usize, 27usize, 2u32, 2212344u32);
    emu.lbu_no_count(23usize, 27usize, 3u32, 2212348u32);
    emu.lbu_no_count(22usize, 27usize, 4u32, 2212352u32);
    emu.lbu_no_count(21usize, 27usize, 5u32, 2212356u32);
    emu.lbu_no_count(19usize, 27usize, 6u32, 2212360u32);
    emu.lbu_no_count(31usize, 27usize, 7u32, 2212364u32);
    emu.lbu_no_count(30usize, 27usize, 8u32, 2212368u32);
    emu.lbu_no_count(29usize, 27usize, 9u32, 2212372u32);
    emu.lbu_no_count(28usize, 27usize, 10u32, 2212376u32);
    emu.lbu_no_count(7usize, 27usize, 11u32, 2212380u32);
    emu.lbu_no_count(6usize, 27usize, 12u32, 2212384u32);
    emu.lbu_no_count(5usize, 27usize, 13u32, 2212388u32);
    emu.lbu_no_count(17usize, 27usize, 14u32, 2212392u32);
    emu.lbu_no_count(15usize, 27usize, 15u32, 2212396u32);
    emu.xri_no_count(16usize, 26usize, 4294967295u32, 2212400u32);
    emu.xri_no_count(1usize, 25usize, 4294967295u32, 2212404u32);
    emu.xri_no_count(8usize, 24usize, 4294967295u32, 2212408u32);
    emu.xri_no_count(10usize, 23usize, 4294967295u32, 2212412u32);
    emu.xri_no_count(14usize, 22usize, 4294967295u32, 2212416u32);
    emu.xri_no_count(12usize, 21usize, 4294967295u32, 2212420u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2212424u32);
    emu.sli_no_count(1usize, 1usize, 24u32, 2212428u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2212432u32);
    emu.sri_no_count(1usize, 1usize, 31u32, 2212436u32);
    emu.adr_no_count(16usize, 1usize, 16usize, 2212440u32);
    emu.xri_no_count(1usize, 31usize, 4294967295u32, 2212444u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2212448u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2212452u32);
    emu.sri_no_count(8usize, 8usize, 31u32, 2212456u32);
    emu.sri_no_count(10usize, 10usize, 31u32, 2212460u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2212464u32);
    emu.xri_no_count(8usize, 30usize, 4294967295u32, 2212468u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2212472u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2212476u32);
    emu.sri_no_count(14usize, 14usize, 31u32, 2212480u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2212484u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2212488u32);
    emu.xri_no_count(14usize, 7usize, 4294967295u32, 2212492u32);
    emu.sli_no_count(1usize, 1usize, 24u32, 2212496u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2212500u32);
    emu.sri_no_count(1usize, 1usize, 31u32, 2212504u32);
    emu.sri_no_count(8usize, 8usize, 31u32, 2212508u32);
    emu.adr_no_count(8usize, 1usize, 8usize, 2212512u32);
    emu.xri_no_count(1usize, 6usize, 4294967295u32, 2212516u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2212520u32);
    emu.sli_no_count(1usize, 1usize, 24u32, 2212524u32);
    emu.sri_no_count(14usize, 14usize, 31u32, 2212528u32);
    emu.sri_no_count(1usize, 1usize, 31u32, 2212532u32);
    emu.adr_no_count(14usize, 14usize, 1usize, 2212536u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2212540u32);
    emu.xri_no_count(16usize, 19usize, 4294967295u32, 2212544u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2212548u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2212552u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2212556u32);
    emu.xri_no_count(16usize, 29usize, 4294967295u32, 2212560u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2212564u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2212568u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2212572u32);
    emu.xri_no_count(8usize, 5usize, 4294967295u32, 2212576u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2212580u32);
    emu.sri_no_count(8usize, 8usize, 31u32, 2212584u32);
    emu.adr_no_count(14usize, 14usize, 8usize, 2212588u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2212592u32);
    emu.xri_no_count(12usize, 28usize, 4294967295u32, 2212596u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2212600u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2212604u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2212608u32);
    emu.xri_no_count(16usize, 17usize, 4294967295u32, 2212612u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2212616u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2212620u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2212624u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2212628u32);
    emu.xri_no_count(12usize, 15usize, 4294967295u32, 2212632u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2212636u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2212640u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2212644u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2212648u32);
    emu.ani_no_count(10usize, 10usize, 255u32, 2212652u32);
    emu.adr_no_count(16usize, 18usize, 11usize, 2212656u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2212660u32);
    emu.add_memory_rw_events(83usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4b8));
    } else {
        emu.pc = 2212664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c338));
    }
}
#[inline(never)]
pub fn block_0x0021c338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 26usize, 4294967199u32, 2212668u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212672u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212676u32);
    emu.xrr_no_count(26usize, 10usize, 26usize, 2212680u32);
    emu.adi_no_count(10usize, 25usize, 4294967199u32, 2212684u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212688u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212692u32);
    emu.xrr_no_count(25usize, 10usize, 25usize, 2212696u32);
    emu.adi_no_count(10usize, 24usize, 4294967199u32, 2212700u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212704u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212708u32);
    emu.xrr_no_count(24usize, 10usize, 24usize, 2212712u32);
    emu.adi_no_count(10usize, 23usize, 4294967199u32, 2212716u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212720u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212724u32);
    emu.xrr_no_count(23usize, 10usize, 23usize, 2212728u32);
    emu.adi_no_count(10usize, 22usize, 4294967199u32, 2212732u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212736u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212740u32);
    emu.xrr_no_count(22usize, 10usize, 22usize, 2212744u32);
    emu.adi_no_count(10usize, 21usize, 4294967199u32, 2212748u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212752u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212756u32);
    emu.xrr_no_count(27usize, 10usize, 21usize, 2212760u32);
    emu.adi_no_count(10usize, 19usize, 4294967199u32, 2212764u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212768u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212772u32);
    emu.xrr_no_count(19usize, 10usize, 19usize, 2212776u32);
    emu.adi_no_count(10usize, 31usize, 4294967199u32, 2212780u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2212784u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2212788u32);
    emu.xrr_no_count(10usize, 10usize, 31usize, 2212792u32);
    emu.adi_no_count(12usize, 30usize, 4294967199u32, 2212796u32);
    emu.sltiu_no_count(12usize, 12usize, 26u32, 2212800u32);
    emu.sli_no_count(12usize, 12usize, 5u32, 2212804u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2212808u32);
    emu.adi_no_count(14usize, 29usize, 4294967199u32, 2212812u32);
    emu.sltiu_no_count(14usize, 14usize, 26u32, 2212816u32);
    emu.sli_no_count(14usize, 14usize, 5u32, 2212820u32);
    emu.xrr_no_count(14usize, 14usize, 29usize, 2212824u32);
    emu.adi_no_count(29usize, 28usize, 4294967199u32, 2212828u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2212832u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2212836u32);
    emu.xrr_no_count(28usize, 29usize, 28usize, 2212840u32);
    emu.adi_no_count(29usize, 7usize, 4294967199u32, 2212844u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2212848u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2212852u32);
    emu.xrr_no_count(7usize, 29usize, 7usize, 2212856u32);
    emu.adi_no_count(29usize, 6usize, 4294967199u32, 2212860u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2212864u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2212868u32);
    emu.xrr_no_count(6usize, 29usize, 6usize, 2212872u32);
    emu.adi_no_count(29usize, 5usize, 4294967199u32, 2212876u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2212880u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2212884u32);
    emu.xrr_no_count(5usize, 29usize, 5usize, 2212888u32);
    emu.adi_no_count(29usize, 17usize, 4294967199u32, 2212892u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2212896u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2212900u32);
    emu.xrr_no_count(17usize, 29usize, 17usize, 2212904u32);
    emu.adi_no_count(29usize, 15usize, 4294967199u32, 2212908u32);
    emu.adi_no_count(13usize, 13usize, 4294967280u32, 2212912u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2212916u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2212920u32);
    emu.xrr_no_count(15usize, 29usize, 15usize, 2212924u32);
    emu.adr_no_count(21usize, 20usize, 13usize, 2212928u32);
    emu.sb_no_count(26usize, 16usize, 0u32, 2212932u32);
    emu.sb_no_count(25usize, 16usize, 1u32, 2212936u32);
    emu.sb_no_count(24usize, 16usize, 2u32, 2212940u32);
    emu.sb_no_count(23usize, 16usize, 3u32, 2212944u32);
    emu.sb_no_count(22usize, 16usize, 4u32, 2212948u32);
    emu.sb_no_count(27usize, 16usize, 5u32, 2212952u32);
    emu.sb_no_count(19usize, 16usize, 6u32, 2212956u32);
    emu.sb_no_count(10usize, 16usize, 7u32, 2212960u32);
    emu.sb_no_count(12usize, 16usize, 8u32, 2212964u32);
    emu.sb_no_count(14usize, 16usize, 9u32, 2212968u32);
    emu.sb_no_count(28usize, 16usize, 10u32, 2212972u32);
    emu.sb_no_count(7usize, 16usize, 11u32, 2212976u32);
    emu.sb_no_count(6usize, 16usize, 12u32, 2212980u32);
    emu.sb_no_count(5usize, 16usize, 13u32, 2212984u32);
    emu.sb_no_count(17usize, 16usize, 14u32, 2212988u32);
    emu.sb_no_count(15usize, 16usize, 15u32, 2212992u32);
    emu.adi_no_count(11usize, 11usize, 16u32, 2212996u32);
    emu.adi_no_count(10usize, 0usize, 15u32, 2213000u32);
    emu.add_memory_rw_events(84usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2212328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c1e8));
    } else {
        emu.pc = 2213004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c48c));
    }
}
#[inline(always)]
pub fn block_0x0021c48c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2213068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4cc));
    } else {
        emu.pc = 2213008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c490));
    }
}
#[inline(always)]
pub fn block_0x0021c490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 16u32, 2213012u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2213016u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2213020u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c514));
}
#[inline(always)]
pub fn block_0x0021c49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2213024u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2213028u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2213032u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c514));
}
#[inline(always)]
pub fn block_0x0021c4a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2213036u32);
    emu.adi_no_count(21usize, 20usize, 0u32, 2213040u32);
    emu.adi_no_count(16usize, 18usize, 0u32, 2213044u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2213048u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213084u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c4dc));
}
#[inline(always)]
pub fn block_0x0021c4b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 20usize, 13usize, 2213052u32);
    emu.adi_no_count(9usize, 27usize, 0u32, 2213056u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2213060u32);
    emu.lw_no_count(8usize, 2usize, 16u32, 2213064u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2213068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213084u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c4dc));
}
#[inline(always)]
pub fn block_0x0021c4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 9usize, 11usize, 2213072u32);
    emu.adr_no_count(16usize, 18usize, 11usize, 2213076u32);
    emu.lw_no_count(8usize, 2usize, 16u32, 2213080u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2213084u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2213084u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c4dc));
}
#[inline(always)]
pub fn block_0x0021c4dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 19usize, 21usize, 2213088u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2213088u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c4e0));
}
#[inline(always)]
pub fn block_0x0021c4e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 9usize, 0u32, 2213092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2213236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c574));
    } else {
        emu.pc = 2213096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4e8));
    }
}
#[inline]
pub fn block_0x0021c4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 255u32, 2213100u32);
    emu.adi_no_count(19usize, 19usize, 1u32, 2213104u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2213108u32);
    emu.adi_no_count(9usize, 9usize, 1u32, 2213112u32);
    emu.adi_no_count(11usize, 10usize, 4294967199u32, 2213116u32);
    emu.sltiu_no_count(11usize, 11usize, 26u32, 2213120u32);
    emu.sli_no_count(11usize, 11usize, 5u32, 2213124u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2213128u32);
    emu.sb_no_count(10usize, 16usize, 0u32, 2213132u32);
    emu.adi_no_count(16usize, 16usize, 1u32, 2213136u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2213088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c4e0));
    } else {
        emu.pc = 2213140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c514));
    }
}
#[inline(always)]
pub fn block_0x0021c514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 20u32, 2213144u32)?;
    emu.sw_no_count(18usize, 2usize, 24u32, 2213148u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2213152u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2213152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c520));
}
#[inline]
pub fn block_0x0021c520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2213156u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2213160u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2213164u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2213168u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2213172u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2213176u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2213180u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2213184u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2213188u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2213192u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2213196u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2213200u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2213204u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2213208u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2213212u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2213216u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2213220u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2213224u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2213228u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2213232u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213236u32;
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
pub fn block_0x0021c574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 20u32, 2213240u32)?;
    emu.sw_no_count(18usize, 2usize, 24u32, 2213244u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2213248u32)?;
    emu.adr_no_count(21usize, 9usize, 21usize, 2213252u32);
    emu.adi_no_count(22usize, 0usize, 128u32, 2213256u32);
    emu.adi_no_count(23usize, 0usize, 224u32, 2213260u32);
    emu.adi_no_count(24usize, 0usize, 240u32, 2213264u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2213268u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c5a4));
}
#[inline(always)]
pub fn block_0x0021c594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(25usize, 10usize, 0u32, 2213272u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2213272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c598));
}
#[inline(always)]
pub fn block_0x0021c598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 20usize, 19usize, 2213276u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2213276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c59c));
}
#[inline(always)]
pub fn block_0x0021c59c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 28u32, 2213280u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2213152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c520));
    } else {
        emu.pc = 2213284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5a4));
    }
}
#[inline(always)]
pub fn block_0x0021c5a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(10usize, 9usize, 0u32, 2213288u32);
    emu.ani_no_count(11usize, 10usize, 255u32, 2213292u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2213344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5e0));
    } else {
        emu.pc = 2213296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5b0));
    }
}
#[inline(always)]
pub fn block_0x0021c5b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 1u32, 2213300u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2213304u32);
    emu.apc_no_count(1usize, 2213304u32, 4096u32, 2213308u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213312u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c5c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2213316u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2213496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c678));
    } else {
        emu.pc = 2213320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5c8));
    }
}
#[inline(always)]
pub fn block_0x0021c5c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 40u32, 2213324u32)?;
    emu.lw_no_count(27usize, 2usize, 32u32, 2213328u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2213436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c63c));
    } else {
        emu.pc = 2213332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5d4));
    }
}
#[inline(always)]
pub fn block_0x0021c5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2213448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c648));
    } else {
        emu.pc = 2213336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5d8));
    }
}
#[inline(always)]
pub fn block_0x0021c5d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2213340u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6b4));
}
#[inline(always)]
pub fn block_0x0021c5e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 9usize, 1u32, 2213348u32);
    emu.ani_no_count(10usize, 11usize, 31u32, 2213352u32);
    emu.ani_no_count(12usize, 12usize, 63u32, 2213356u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2213464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c658));
    } else {
        emu.pc = 2213360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5f0));
    }
}
#[inline(always)]
pub fn block_0x0021c5f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 9usize, 2u32, 2213364u32);
    emu.sli_no_count(12usize, 12usize, 6u32, 2213368u32);
    emu.ani_no_count(13usize, 13usize, 63u32, 2213372u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2213376u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2213604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6e4));
    } else {
        emu.pc = 2213380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c604));
    }
}
#[inline]
pub fn block_0x0021c604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 3u32, 2213384u32);
    emu.adi_no_count(9usize, 9usize, 4u32, 2213388u32);
    emu.sli_no_count(10usize, 10usize, 29u32, 2213392u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2213396u32);
    emu.sli_no_count(12usize, 12usize, 6u32, 2213400u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2213404u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2213408u32);
    emu.orr_no_count(11usize, 11usize, 10usize, 2213412u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2213416u32);
    emu.apc_no_count(1usize, 2213416u32, 4096u32, 2213420u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2213428u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2213320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5c8));
    } else {
        emu.pc = 2213432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c638));
    }
}
#[inline(always)]
pub fn block_0x0021c638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2213436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c678));
}
#[inline(always)]
pub fn block_0x0021c63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2213528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c698));
    } else {
        emu.pc = 2213440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c640));
    }
}
#[inline(always)]
pub fn block_0x0021c640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2213444u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213736u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c768));
}
#[inline(always)]
pub fn block_0x0021c648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2213452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6a8));
    } else {
        emu.pc = 2213456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c650));
    }
}
#[inline(always)]
pub fn block_0x0021c650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2213460u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6b4));
}
#[inline(always)]
pub fn block_0x0021c658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 2u32, 2213468u32);
    emu.sli_no_count(10usize, 10usize, 6u32, 2213472u32);
    emu.orr_no_count(11usize, 10usize, 12usize, 2213476u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2213480u32);
    emu.apc_no_count(1usize, 2213480u32, 4096u32, 2213484u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213488u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2213492u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2213320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5c8));
    } else {
        emu.pc = 2213496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c678));
    }
}
#[inline(always)]
pub fn block_0x0021c678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 32u32, 2213500u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2213512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c688));
    } else {
        emu.pc = 2213504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c680));
    }
}
#[inline(always)]
pub fn block_0x0021c680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2213508u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c714));
}
#[inline(always)]
pub fn block_0x0021c688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 11u32, 2213516u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c708));
    } else {
        emu.pc = 2213520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c690));
    }
}
#[inline(always)]
pub fn block_0x0021c690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2213524u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c714));
}
#[inline(always)]
pub fn block_0x0021c698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2213532u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c75c));
    } else {
        emu.pc = 2213536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6a0));
    }
}
#[inline(always)]
pub fn block_0x0021c6a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2213540u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213736u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c768));
}
#[inline(always)]
pub fn block_0x0021c6a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 16u32, 2213548u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2213552u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2213556u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2213556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6b4));
}
#[inline(always)]
pub fn block_0x0021c6b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2213560u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2213564u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2213568u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2213812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c7b4));
    } else {
        emu.pc = 2213572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6c4));
    }
}
#[inline(always)]
pub fn block_0x0021c6c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 18usize, 10usize, 2213576u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2213848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c7d8));
    } else {
        emu.pc = 2213580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6cc));
    }
}
#[inline(always)]
pub fn block_0x0021c6cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(27usize, 18usize, 0u32, 2213584u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2213588u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2213592u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2213892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c804));
    } else {
        emu.pc = 2213596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6dc));
    }
}
#[inline(always)]
pub fn block_0x0021c6dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2213600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c860));
}
#[inline(always)]
pub fn block_0x0021c6e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 3u32, 2213608u32);
    emu.sli_no_count(10usize, 10usize, 12u32, 2213612u32);
    emu.orr_no_count(11usize, 12usize, 10usize, 2213616u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2213620u32);
    emu.apc_no_count(1usize, 2213620u32, 4096u32, 2213624u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2213632u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2213320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c5c8));
    } else {
        emu.pc = 2213636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c704));
    }
}
#[inline(always)]
pub fn block_0x0021c704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2213640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c678));
}
#[inline(always)]
pub fn block_0x0021c708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 16u32, 2213644u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2213648u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2213652u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2213652u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c714));
}
#[inline(always)]
pub fn block_0x0021c714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2213656u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2213660u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2213664u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2214548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ca94));
    } else {
        emu.pc = 2213668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c724));
    }
}
#[inline(always)]
pub fn block_0x0021c724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 10usize, 2213672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2213268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c594));
    } else {
        emu.pc = 2213676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c72c));
    }
}
#[inline(always)]
pub fn block_0x0021c72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 11u32, 2213680u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2213696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c740));
    } else {
        emu.pc = 2213684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c734));
    }
}
#[inline(always)]
pub fn block_0x0021c734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 6u32, 2213688u32);
    emu.ani_no_count(12usize, 25usize, 63u32, 2213692u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2213696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214420u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ca14));
}
#[inline(always)]
pub fn block_0x0021c740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 16u32, 2213700u32);
    emu.sli_no_count(12usize, 25usize, 20u32, 2213704u32);
    emu.ani_no_count(11usize, 25usize, 63u32, 2213708u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2213712u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2213784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c798));
    } else {
        emu.pc = 2213716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c754));
    }
}
#[inline(always)]
pub fn block_0x0021c754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 12u32, 2213720u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ca40));
}
#[inline(always)]
pub fn block_0x0021c75c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 16u32, 2213728u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2213732u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2213736u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2213736u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c768));
}
#[inline(always)]
pub fn block_0x0021c768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2213740u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2213744u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2213748u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2214588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cabc));
    } else {
        emu.pc = 2213752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c778));
    }
}
#[inline(always)]
pub fn block_0x0021c778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 18usize, 10usize, 2213756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2214624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cae0));
    } else {
        emu.pc = 2213760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c780));
    }
}
#[inline(always)]
pub fn block_0x0021c780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(27usize, 18usize, 0u32, 2213764u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2213768u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2213772u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2214668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb0c));
    } else {
        emu.pc = 2213776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c790));
    }
}
#[inline(always)]
pub fn block_0x0021c790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2213780u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cb68));
}
#[inline(always)]
pub fn block_0x0021c798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 18u32, 2213788u32);
    emu.sli_no_count(25usize, 25usize, 14u32, 2213792u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2213796u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2213800u32);
    emu.adi_no_count(13usize, 13usize, 240u32, 2213804u32);
    emu.sri_no_count(14usize, 25usize, 26u32, 2213808u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2213812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ca78));
}
#[inline(always)]
pub fn block_0x0021c7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2213816u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2213820u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2213824u32);
    emu.apc_no_count(1usize, 2213824u32, 4294963200u32, 2213828u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2213832u32;
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
pub fn block_0x0021c7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2213836u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2213840u32)?;
    emu.adr_no_count(18usize, 18usize, 10usize, 2213844u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a < b {
        emu.pc = 2213580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6cc));
    } else {
        emu.pc = 2213848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c7d8));
    }
}
#[inline(always)]
pub fn block_0x0021c7d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2213852u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c814));
    } else {
        emu.pc = 2213856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c7e0));
    }
}
#[inline]
pub fn block_0x0021c7e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 6u32, 2213860u32);
    emu.ani_no_count(11usize, 27usize, 63u32, 2213864u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2213868u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2213872u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2213876u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2213880u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2213884u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2213888u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2213596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6dc));
    } else {
        emu.pc = 2213892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c804));
    }
}
#[inline(always)]
pub fn block_0x0021c804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 11u32, 2213896u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2213972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c854));
    } else {
        emu.pc = 2213900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c80c));
    }
}
#[inline(always)]
pub fn block_0x0021c80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2213904u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2213908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c860));
}
#[inline(always)]
pub fn block_0x0021c814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 27usize, 16u32, 2213912u32);
    emu.sli_no_count(11usize, 27usize, 20u32, 2213916u32);
    emu.ani_no_count(10usize, 27usize, 63u32, 2213920u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2214036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c894));
    } else {
        emu.pc = 2213924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c824));
    }
}
#[inline]
pub fn block_0x0021c824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 27usize, 12u32, 2213928u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2213932u32);
    emu.adi_no_count(10usize, 10usize, 128u32, 2213936u32);
    emu.ori_no_count(12usize, 12usize, 224u32, 2213940u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2213944u32);
    emu.sb_no_count(12usize, 18usize, 0u32, 2213948u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2213952u32);
    emu.sb_no_count(10usize, 18usize, 2u32, 2213956u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2213960u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2213964u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2213892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c804));
    } else {
        emu.pc = 2213968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c850));
    }
}
#[inline(always)]
pub fn block_0x0021c850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2213972u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c6dc));
}
#[inline(always)]
pub fn block_0x0021c854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 16u32, 2213976u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2213980u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2213984u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2213984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c860));
}
#[inline(always)]
pub fn block_0x0021c860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2213988u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2213992u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2213996u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2214100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c8d4));
    } else {
        emu.pc = 2214000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c870));
    }
}
#[inline(always)]
pub fn block_0x0021c870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2214004u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2214008u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2214136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c8f8));
    } else {
        emu.pc = 2214012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c87c));
    }
}
#[inline(always)]
pub fn block_0x0021c87c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(25usize, 10usize, 0u32, 2214016u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214020u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214024u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2214180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c924));
    } else {
        emu.pc = 2214028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c88c));
    }
}
#[inline(always)]
pub fn block_0x0021c88c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2214032u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2214036u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c980));
}
#[inline]
pub fn block_0x0021c894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 27usize, 18u32, 2214040u32);
    emu.sli_no_count(27usize, 27usize, 14u32, 2214044u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2214048u32);
    emu.adi_no_count(10usize, 10usize, 128u32, 2214052u32);
    emu.adi_no_count(12usize, 12usize, 240u32, 2214056u32);
    emu.sri_no_count(13usize, 27usize, 26u32, 2214060u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214064u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2214068u32);
    emu.sb_no_count(12usize, 18usize, 0u32, 2214072u32);
    emu.sb_no_count(13usize, 18usize, 1u32, 2214076u32);
    emu.sb_no_count(11usize, 18usize, 2u32, 2214080u32);
    emu.sb_no_count(10usize, 18usize, 3u32, 2214084u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214088u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214092u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2213596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c6dc));
    } else {
        emu.pc = 2214096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c8d0));
    }
}
#[inline(always)]
pub fn block_0x0021c8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2214100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c804));
}
#[inline(always)]
pub fn block_0x0021c8d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2214104u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2214108u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2214112u32);
    emu.apc_no_count(1usize, 2214112u32, 4294963200u32, 2214116u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214120u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2214124u32)?;
    emu.lw_no_count(18usize, 2usize, 24u32, 2214128u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2214132u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2214012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c87c));
    } else {
        emu.pc = 2214136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c8f8));
    }
}
#[inline(always)]
pub fn block_0x0021c8f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 11u32, 2214140u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2214196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c934));
    } else {
        emu.pc = 2214144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c900));
    }
}
#[inline]
pub fn block_0x0021c900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 6u32, 2214148u32);
    emu.ani_no_count(12usize, 25usize, 63u32, 2214152u32);
    emu.ori_no_count(11usize, 11usize, 192u32, 2214156u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214160u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2214164u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2214168u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214172u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214176u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2214028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c88c));
    } else {
        emu.pc = 2214180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c924));
    }
}
#[inline(always)]
pub fn block_0x0021c924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 26usize, 11u32, 2214184u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2214260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c974));
    } else {
        emu.pc = 2214188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c92c));
    }
}
#[inline(always)]
pub fn block_0x0021c92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2214192u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2214196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c980));
}
#[inline(always)]
pub fn block_0x0021c934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 16u32, 2214200u32);
    emu.sli_no_count(12usize, 25usize, 20u32, 2214204u32);
    emu.ani_no_count(11usize, 25usize, 63u32, 2214208u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2214304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c9a0));
    } else {
        emu.pc = 2214212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c944));
    }
}
#[inline]
pub fn block_0x0021c944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 12u32, 2214216u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2214220u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214224u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2214228u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214232u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2214236u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2214240u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2214244u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214248u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214252u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2214180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c924));
    } else {
        emu.pc = 2214256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c970));
    }
}
#[inline(always)]
pub fn block_0x0021c970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2214260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c88c));
}
#[inline(always)]
pub fn block_0x0021c974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 26usize, 16u32, 2214264u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2214268u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2214272u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2214272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c980));
}
#[inline(always)]
pub fn block_0x0021c980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2214276u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2214280u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2214284u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2214368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c9e0));
    } else {
        emu.pc = 2214288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c990));
    }
}
#[inline(always)]
pub fn block_0x0021c990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 10usize, 2214292u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2214404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ca04));
    } else {
        emu.pc = 2214296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c998));
    }
}
#[inline(always)]
pub fn block_0x0021c998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(26usize, 10usize, 0u32, 2214300u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2214304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c598));
}
#[inline]
pub fn block_0x0021c9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 18u32, 2214308u32);
    emu.sli_no_count(25usize, 25usize, 14u32, 2214312u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2214316u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214320u32);
    emu.adi_no_count(13usize, 13usize, 240u32, 2214324u32);
    emu.sri_no_count(14usize, 25usize, 26u32, 2214328u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214332u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2214336u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2214340u32);
    emu.sb_no_count(14usize, 10usize, 1u32, 2214344u32);
    emu.sb_no_count(12usize, 10usize, 2u32, 2214348u32);
    emu.sb_no_count(11usize, 10usize, 3u32, 2214352u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214356u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214360u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2214028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c88c));
    } else {
        emu.pc = 2214364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c9dc));
    }
}
#[inline(always)]
pub fn block_0x0021c9dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2214368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214180u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c924));
}
#[inline(always)]
pub fn block_0x0021c9e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2214372u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2214376u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2214380u32);
    emu.apc_no_count(1usize, 2214380u32, 4294963200u32, 2214384u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214388u32;
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
pub fn block_0x0021c9f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2214392u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2214396u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2214400u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2214296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c998));
    } else {
        emu.pc = 2214404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ca04));
    }
}
#[inline(always)]
pub fn block_0x0021ca04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 26usize, 11u32, 2214408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2214440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ca28));
    } else {
        emu.pc = 2214412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ca0c));
    }
}
#[inline(always)]
pub fn block_0x0021ca0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 26usize, 6u32, 2214416u32);
    emu.ani_no_count(12usize, 26usize, 63u32, 2214420u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2214420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ca14));
}
#[inline(always)]
pub fn block_0x0021ca14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ori_no_count(11usize, 11usize, 192u32, 2214424u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214428u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2214432u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2214436u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2214440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213272u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c598));
}
#[inline(always)]
pub fn block_0x0021ca28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 26usize, 16u32, 2214444u32);
    emu.sli_no_count(12usize, 26usize, 20u32, 2214448u32);
    emu.ani_no_count(11usize, 26usize, 63u32, 2214452u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214456u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2214496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ca60));
    } else {
        emu.pc = 2214460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ca3c));
    }
}
#[inline(always)]
pub fn block_0x0021ca3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 26usize, 12u32, 2214464u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2214464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ca40));
}
#[inline(always)]
pub fn block_0x0021ca40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 12usize, 26u32, 2214468u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214472u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2214476u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214480u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2214484u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2214488u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2214492u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2214496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c59c));
}
#[inline(always)]
pub fn block_0x0021ca60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 26usize, 18u32, 2214500u32);
    emu.sli_no_count(26usize, 26usize, 14u32, 2214504u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2214508u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214512u32);
    emu.adi_no_count(13usize, 13usize, 240u32, 2214516u32);
    emu.sri_no_count(14usize, 26usize, 26u32, 2214520u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2214520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ca78));
}
#[inline(always)]
pub fn block_0x0021ca78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 128u32, 2214524u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2214528u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2214532u32);
    emu.sb_no_count(14usize, 10usize, 1u32, 2214536u32);
    emu.sb_no_count(12usize, 10usize, 2u32, 2214540u32);
    emu.sb_no_count(11usize, 10usize, 3u32, 2214544u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2214548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c59c));
}
#[inline(always)]
pub fn block_0x0021ca94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2214552u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2214556u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2214560u32);
    emu.apc_no_count(1usize, 2214560u32, 4294963200u32, 2214564u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021caa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2214572u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2214576u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2214580u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2213676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c72c));
    } else {
        emu.pc = 2214584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cab8));
    }
}
#[inline(always)]
pub fn block_0x0021cab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2214588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c594));
}
#[inline(always)]
pub fn block_0x0021cabc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2214592u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2214596u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2214600u32);
    emu.apc_no_count(1usize, 2214600u32, 4294963200u32, 2214604u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214608u32;
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
pub fn block_0x0021cad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2214612u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2214616u32)?;
    emu.adr_no_count(18usize, 18usize, 10usize, 2214620u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a < b {
        emu.pc = 2213760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c780));
    } else {
        emu.pc = 2214624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cae0));
    }
}
#[inline(always)]
pub fn block_0x0021cae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2214628u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2214684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb1c));
    } else {
        emu.pc = 2214632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cae8));
    }
}
#[inline]
pub fn block_0x0021cae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 6u32, 2214636u32);
    emu.ani_no_count(11usize, 27usize, 63u32, 2214640u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2214644u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214648u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2214652u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2214656u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214660u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214664u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2213776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c790));
    } else {
        emu.pc = 2214668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb0c));
    }
}
#[inline(always)]
pub fn block_0x0021cb0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 11u32, 2214672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2214748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb5c));
    } else {
        emu.pc = 2214676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb14));
    }
}
#[inline(always)]
pub fn block_0x0021cb14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2214680u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2214684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cb68));
}
#[inline(always)]
pub fn block_0x0021cb1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 16u32, 2214688u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2214880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cbe0));
    } else {
        emu.pc = 2214692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb24));
    }
}
#[inline]
pub fn block_0x0021cb24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 12u32, 2214696u32);
    emu.sli_no_count(11usize, 27usize, 20u32, 2214700u32);
    emu.ani_no_count(12usize, 27usize, 63u32, 2214704u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2214708u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2214712u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214716u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214720u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2214724u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2214728u32);
    emu.sb_no_count(12usize, 18usize, 2u32, 2214732u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214736u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214740u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2213776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c790));
    } else {
        emu.pc = 2214744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb58));
    }
}
#[inline(always)]
pub fn block_0x0021cb58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2214748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cb0c));
}
#[inline(always)]
pub fn block_0x0021cb5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 16u32, 2214752u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2214756u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2214760u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2214760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cb68));
}
#[inline(always)]
pub fn block_0x0021cb68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2214764u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2214768u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2214772u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2215012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2215012u32));
    } else {
        emu.pc = 2214776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb78));
    }
}
#[inline(always)]
pub fn block_0x0021cb78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2214780u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2214784u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2213268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c594));
    } else {
        emu.pc = 2214788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb84));
    }
}
#[inline(always)]
pub fn block_0x0021cb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 11u32, 2214792u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214796u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2214828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cbac));
    } else {
        emu.pc = 2214800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb90));
    }
}
#[inline(always)]
pub fn block_0x0021cb90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 6u32, 2214804u32);
    emu.ani_no_count(12usize, 25usize, 63u32, 2214808u32);
    emu.ori_no_count(11usize, 11usize, 192u32, 2214812u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214816u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2214820u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2214824u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2214828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c59c));
}
#[inline(always)]
pub fn block_0x0021cbac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 16u32, 2214832u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2214952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2214952u32));
    } else {
        emu.pc = 2214836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cbb4));
    }
}
#[inline]
pub fn block_0x0021cbb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 12u32, 2214840u32);
    emu.sli_no_count(12usize, 25usize, 20u32, 2214844u32);
    emu.ani_no_count(13usize, 25usize, 63u32, 2214848u32);
    emu.ori_no_count(11usize, 11usize, 224u32, 2214852u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2214856u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2214860u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214864u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2214868u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2214872u32);
    emu.sb_no_count(13usize, 10usize, 2u32, 2214876u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2214880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c59c));
}
#[inline]
pub fn block_0x0021cbe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 18u32, 2214884u32);
    emu.sli_no_count(11usize, 27usize, 14u32, 2214888u32);
    emu.sli_no_count(12usize, 27usize, 20u32, 2214892u32);
    emu.ani_no_count(13usize, 27usize, 63u32, 2214896u32);
    emu.adi_no_count(10usize, 10usize, 240u32, 2214900u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2214904u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2214908u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2214912u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2214916u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214920u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2214924u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2214928u32);
    emu.sb_no_count(12usize, 18usize, 2u32, 2214932u32);
    emu.sb_no_count(13usize, 18usize, 3u32, 2214936u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2214940u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2214944u32)?;
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2214668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb0c));
    } else {
        emu.pc = 2214948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cc24));
    }
}
#[inline(always)]
pub fn block_0x0021cc24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2214952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021c790));
}
