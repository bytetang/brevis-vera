pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2103868u32;
pub const PC_MAX: u32 = 2107244u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 116usize] = [
        block_0x00201a3c,
        block_0x00201a98,
        block_0x00201b08,
        block_0x00201b2c,
        block_0x00201b38,
        block_0x00201b40,
        block_0x00201b4c,
        block_0x00201b64,
        block_0x00201bf0,
        block_0x00201c04,
        block_0x00201c10,
        block_0x00201c24,
        block_0x00201c38,
        block_0x00201c80,
        block_0x00201c94,
        block_0x00201c9c,
        block_0x00201e0c,
        block_0x00201e2c,
        block_0x00201e30,
        block_0x00201e6c,
        block_0x00201ea0,
        block_0x00201eb4,
        block_0x00201ec4,
        block_0x00201ed8,
        block_0x00201edc,
        block_0x00201efc,
        block_0x00201f0c,
        block_0x00201f60,
        block_0x00201f78,
        block_0x00201f94,
        block_0x00201f98,
        block_0x00201fd0,
        block_0x00201fd4,
        block_0x00201fd8,
        block_0x00201fdc,
        block_0x00202000,
        block_0x00202018,
        block_0x0020203c,
        block_0x00202048,
        block_0x00202060,
        block_0x0020208c,
        block_0x00202094,
        block_0x002020a8,
        block_0x002020d0,
        block_0x002020d8,
        block_0x002020ec,
        block_0x00202114,
        block_0x0020211c,
        block_0x0020212c,
        block_0x00202138,
        block_0x00202144,
        block_0x00202164,
        block_0x0020217c,
        block_0x00202214,
        block_0x0020222c,
        block_0x00202244,
        block_0x00202258,
        block_0x00202260,
        block_0x00202264,
        block_0x00202288,
        block_0x00202294,
        block_0x002022ac,
        block_0x002022d8,
        block_0x002022e0,
        block_0x002022f4,
        block_0x0020231c,
        block_0x00202324,
        block_0x00202338,
        block_0x00202360,
        block_0x00202368,
        block_0x00202378,
        block_0x00202384,
        block_0x00202390,
        block_0x002023b0,
        block_0x002023c8,
        block_0x00202464,
        block_0x0020247c,
        block_0x00202494,
        block_0x002024a8,
        block_0x002024b0,
        block_0x002024b4,
        block_0x002024d0,
        block_0x002024ec,
        block_0x002024f4,
        block_0x00202500,
        block_0x0020250c,
        block_0x00202530,
        block_0x00202550,
        block_0x0020255c,
        block_0x00202578,
        block_0x00202580,
        block_0x00202598,
        block_0x0020259c,
        block_0x002025b0,
        block_0x002025cc,
        block_0x002025f8,
        block_0x00202608,
        block_0x00202634,
        block_0x00202650,
        block_0x00202658,
        block_0x00202668,
        block_0x00202678,
        block_0x0020268c,
        block_0x00202698,
        block_0x0020269c,
        block_0x002026b0,
        block_0x002026d0,
        block_0x002026dc,
        block_0x002026e8,
        block_0x002026f0,
        block_0x00202708,
        block_0x0020270c,
        block_0x00202720,
        block_0x00202728,
        block_0x0020274c,
        block_0x0020276c,
    ];
    const IDX: [u16; 845usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16,
        6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16,
        11u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16,
        22u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 33u16, 34u16, 35u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 48u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16,
        0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16,
        0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 65u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16, 0u16,
        73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        78u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 84u16, 0u16,
        0u16, 85u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 89u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        92u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16,
        0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16,
        104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 0u16, 109u16, 0u16, 110u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 111u16, 112u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16,
        114u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 116u16,
    ];
    if pc < 2103868u32 || pc > 2107244u32 {
        return None;
    }
    let word_offset = ((pc - 2103868u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00201a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966912u32, 2103872u32);
    emu.sw_no_count(1usize, 2usize, 380u32, 2103876u32)?;
    emu.sw_no_count(8usize, 2usize, 376u32, 2103880u32)?;
    emu.sw_no_count(9usize, 2usize, 372u32, 2103884u32)?;
    emu.sw_no_count(18usize, 2usize, 368u32, 2103888u32)?;
    emu.sw_no_count(19usize, 2usize, 364u32, 2103892u32)?;
    emu.sw_no_count(20usize, 2usize, 360u32, 2103896u32)?;
    emu.sw_no_count(21usize, 2usize, 356u32, 2103900u32)?;
    emu.sw_no_count(22usize, 2usize, 352u32, 2103904u32)?;
    emu.sw_no_count(23usize, 2usize, 348u32, 2103908u32)?;
    emu.sw_no_count(24usize, 2usize, 344u32, 2103912u32)?;
    emu.sw_no_count(25usize, 2usize, 340u32, 2103916u32)?;
    emu.sw_no_count(26usize, 2usize, 336u32, 2103920u32)?;
    emu.sw_no_count(27usize, 2usize, 332u32, 2103924u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2103928u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2103932u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2103936u32);
    emu.adi_no_count(18usize, 2usize, 48u32, 2103940u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2103944u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2103948u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2103952u32);
    emu.apc_no_count(1usize, 2103952u32, 36864u32, 2103956u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00201a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103964u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2103968u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103972u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2103976u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2103980u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2103984u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2103988u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2103992u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2103996u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2104000u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2104004u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2104008u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2104012u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2104016u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2104020u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104024u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2104028u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2104032u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2104036u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2104040u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2104044u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2104048u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2104052u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2104056u32)?;
    emu.sri_no_count(12usize, 9usize, 6u32, 2104060u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2104064u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2104068u32)?;
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2104120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201b38));
    } else {
        emu.pc = 2104072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201b08));
    }
}
#[inline]
pub fn block_0x00201b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 9usize, 4294967232u32, 2104076u32);
    emu.ani_no_count(9usize, 9usize, 63u32, 2104080u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2104084u32);
    emu.sw_no_count(12usize, 2usize, 40u32, 2104088u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2104092u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2104096u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2104100u32);
    emu.apc_no_count(1usize, 2104100u32, 73728u32, 2104104u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2104112u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2104116u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2104120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201b40));
}
#[inline(always)]
pub fn block_0x00201b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2104124u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2104128u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2104128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201b40));
}
#[inline(always)]
pub fn block_0x00201b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 9usize, 0u32, 2104132u32);
    emu.apc_no_count(1usize, 2104132u32, 36864u32, 2104136u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 2usize, 112u32, 2104144u32);
    emu.adi_no_count(10usize, 2usize, 120u32, 2104148u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2104152u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2104156u32);
    emu.apc_no_count(1usize, 2104156u32, 36864u32, 2104160u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965632u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00201b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 160u32, 2104168u32);
    emu.lbu_no_count(18usize, 2usize, 224u32, 2104172u32);
    emu.lw_no_count(10usize, 2usize, 152u32, 2104176u32)?;
    emu.lw_no_count(11usize, 2usize, 156u32, 2104180u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2104184u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2104188u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2104192u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2104196u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2104200u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2104204u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2104208u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2104212u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2104216u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2104220u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2104224u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2104228u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2104232u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2104236u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2104240u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2104244u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2104248u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2104252u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2104256u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2104260u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2104264u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2104268u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2104272u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2104276u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2104280u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2104284u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2104288u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2104292u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2104296u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2104300u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2104336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c10));
    } else {
        emu.pc = 2104304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201bf0));
    }
}
#[inline(always)]
pub fn block_0x00201bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2104308u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2104312u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2104316u32);
    emu.apc_no_count(1usize, 2104316u32, 32768u32, 2104320u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2104328u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2104332u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2104448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c80));
    } else {
        emu.pc = 2104336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201c10));
    }
}
#[inline(always)]
pub fn block_0x00201c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2104340u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2104344u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2104348u32);
    emu.apc_no_count(1usize, 2104348u32, 69632u32, 2104352u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 268u32, 2104360u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2104364u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2104368u32);
    emu.apc_no_count(1usize, 2104368u32, 32768u32, 2104372u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2104380u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2104384u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2104388u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2104392u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2104396u32);
    emu.sb_no_count(20usize, 2usize, 328u32, 2104400u32);
    emu.sb_no_count(12usize, 2usize, 329u32, 2104404u32);
    emu.sb_no_count(11usize, 2usize, 330u32, 2104408u32);
    emu.sb_no_count(10usize, 2usize, 331u32, 2104412u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2104416u32);
    emu.sb_no_count(19usize, 2usize, 324u32, 2104420u32);
    emu.sb_no_count(10usize, 2usize, 325u32, 2104424u32);
    emu.sb_no_count(14usize, 2usize, 326u32, 2104428u32);
    emu.sb_no_count(13usize, 2usize, 327u32, 2104432u32);
    emu.adi_no_count(10usize, 2usize, 120u32, 2104436u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2104440u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2104444u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2104448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2104468u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c94));
}
#[inline(always)]
pub fn block_0x00201c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 216u32, 2104452u32)?;
    emu.sw_no_count(20usize, 2usize, 220u32, 2104456u32)?;
    emu.adi_no_count(10usize, 2usize, 120u32, 2104460u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2104464u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2104468u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2104468u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201c94));
}
#[inline(always)]
pub fn block_0x00201c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2104468u32, 69632u32, 2104472u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00201c9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 92u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2104480u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 2usize, 120u32, 2104484u32)?;
    emu.lw_no_count(12usize, 2usize, 124u32, 2104488u32)?;
    emu.lw_no_count(14usize, 2usize, 128u32, 2104492u32)?;
    emu.lw_no_count(15usize, 2usize, 132u32, 2104496u32)?;
    emu.lw_no_count(16usize, 2usize, 136u32, 2104500u32)?;
    emu.lw_no_count(17usize, 2usize, 140u32, 2104504u32)?;
    emu.lw_no_count(5usize, 2usize, 144u32, 2104508u32)?;
    emu.lw_no_count(13usize, 2usize, 148u32, 2104512u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967040u32, 2104516u32);
    emu.sri_no_count(6usize, 10usize, 8u32, 2104520u32);
    emu.sri_no_count(7usize, 10usize, 24u32, 2104524u32);
    emu.anr_no_count(28usize, 10usize, 11usize, 2104528u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2104532u32);
    emu.sri_no_count(29usize, 12usize, 8u32, 2104536u32);
    emu.sri_no_count(30usize, 12usize, 24u32, 2104540u32);
    emu.anr_no_count(31usize, 12usize, 11usize, 2104544u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2104548u32);
    emu.sri_no_count(9usize, 14usize, 8u32, 2104552u32);
    emu.sri_no_count(18usize, 14usize, 24u32, 2104556u32);
    emu.anr_no_count(19usize, 14usize, 11usize, 2104560u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2104564u32);
    emu.anr_no_count(6usize, 6usize, 11usize, 2104568u32);
    emu.orr_no_count(6usize, 6usize, 7usize, 2104572u32);
    emu.sri_no_count(7usize, 15usize, 8u32, 2104576u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2104580u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2104584u32);
    emu.sri_no_count(28usize, 15usize, 24u32, 2104588u32);
    emu.anr_no_count(29usize, 29usize, 11usize, 2104592u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2104596u32);
    emu.anr_no_count(30usize, 15usize, 11usize, 2104600u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2104604u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2104608u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2104612u32);
    emu.sri_no_count(31usize, 16usize, 8u32, 2104616u32);
    emu.anr_no_count(9usize, 9usize, 11usize, 2104620u32);
    emu.orr_no_count(9usize, 9usize, 18usize, 2104624u32);
    emu.sri_no_count(18usize, 16usize, 24u32, 2104628u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2104632u32);
    emu.orr_no_count(14usize, 14usize, 19usize, 2104636u32);
    emu.anr_no_count(19usize, 16usize, 11usize, 2104640u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2104644u32);
    emu.anr_no_count(7usize, 7usize, 11usize, 2104648u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2104652u32);
    emu.sri_no_count(28usize, 17usize, 8u32, 2104656u32);
    emu.sli_no_count(30usize, 30usize, 8u32, 2104660u32);
    emu.orr_no_count(15usize, 15usize, 30usize, 2104664u32);
    emu.sri_no_count(30usize, 17usize, 24u32, 2104668u32);
    emu.anr_no_count(31usize, 31usize, 11usize, 2104672u32);
    emu.orr_no_count(31usize, 31usize, 18usize, 2104676u32);
    emu.anr_no_count(18usize, 17usize, 11usize, 2104680u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2104684u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2104688u32);
    emu.orr_no_count(16usize, 16usize, 19usize, 2104692u32);
    emu.sri_no_count(19usize, 5usize, 8u32, 2104696u32);
    emu.anr_no_count(28usize, 28usize, 11usize, 2104700u32);
    emu.orr_no_count(28usize, 28usize, 30usize, 2104704u32);
    emu.sri_no_count(30usize, 5usize, 24u32, 2104708u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2104712u32);
    emu.orr_no_count(17usize, 17usize, 18usize, 2104716u32);
    emu.anr_no_count(18usize, 5usize, 11usize, 2104720u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2104724u32);
    emu.anr_no_count(19usize, 19usize, 11usize, 2104728u32);
    emu.orr_no_count(30usize, 19usize, 30usize, 2104732u32);
    emu.sri_no_count(19usize, 13usize, 8u32, 2104736u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2104740u32);
    emu.orr_no_count(5usize, 5usize, 18usize, 2104744u32);
    emu.sri_no_count(18usize, 13usize, 24u32, 2104748u32);
    emu.anr_no_count(19usize, 19usize, 11usize, 2104752u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2104756u32);
    emu.anr_no_count(11usize, 13usize, 11usize, 2104760u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2104764u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2104768u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2104772u32);
    emu.orr_no_count(10usize, 10usize, 6usize, 2104776u32);
    emu.orr_no_count(12usize, 12usize, 29usize, 2104780u32);
    emu.orr_no_count(14usize, 14usize, 9usize, 2104784u32);
    emu.orr_no_count(13usize, 15usize, 7usize, 2104788u32);
    emu.orr_no_count(15usize, 16usize, 31usize, 2104792u32);
    emu.orr_no_count(16usize, 17usize, 28usize, 2104796u32);
    emu.orr_no_count(17usize, 5usize, 30usize, 2104800u32);
    emu.orr_no_count(11usize, 11usize, 18usize, 2104804u32);
    emu.sw_no_count(10usize, 2usize, 268u32, 2104808u32)?;
    emu.sw_no_count(12usize, 2usize, 272u32, 2104812u32)?;
    emu.sw_no_count(14usize, 2usize, 276u32, 2104816u32)?;
    emu.sw_no_count(13usize, 2usize, 280u32, 2104820u32)?;
    emu.sw_no_count(15usize, 2usize, 284u32, 2104824u32)?;
    emu.sw_no_count(16usize, 2usize, 288u32, 2104828u32)?;
    emu.sw_no_count(17usize, 2usize, 292u32, 2104832u32)?;
    emu.sw_no_count(11usize, 2usize, 296u32, 2104836u32)?;
    emu.apc_no_count(1usize, 2104836u32, 32768u32, 2104840u32);
    emu.add_memory_rw_events(92usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2104848u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2104852u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2104856u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2104860u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2104864u32);
    emu.adi_no_count(19usize, 0usize, 64u32, 2104868u32);
    emu.apc_no_count(1usize, 2104868u32, 36864u32, 2104872u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2105184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201f60));
    } else {
        emu.pc = 2104880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e30));
    }
}
#[inline]
pub fn block_0x00201e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2104884u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2104888u32);
    emu.sw_no_count(19usize, 2usize, 232u32, 2104892u32)?;
    emu.sw_no_count(10usize, 2usize, 236u32, 2104896u32)?;
    emu.sw_no_count(0usize, 2usize, 240u32, 2104900u32)?;
    emu.adi_no_count(21usize, 0usize, 32u32, 2104904u32);
    emu.adi_no_count(22usize, 2usize, 268u32, 2104908u32);
    emu.adi_no_count(23usize, 2usize, 244u32, 2104912u32);
    let a = 0u32.wrapping_add(2125824u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2104916u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294967040u32, 2104920u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(25usize, a);
    emu.pc = 2104924u32;
    emu.update_insn_clock();
    emu.adi_no_count(25usize, 25usize, 4294966716u32, 2104928u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(26usize, a);
    emu.pc = 2104932u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 26usize, 4294966772u32, 2104936u32);
    emu.adi_no_count(27usize, 2usize, 260u32, 2104940u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2104940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201e6c));
}
#[inline]
pub fn block_0x00201e6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 2usize, 244u32, 2104944u32)?;
    emu.sw_no_count(23usize, 2usize, 260u32, 2104948u32)?;
    emu.sw_no_count(24usize, 2usize, 264u32, 2104952u32)?;
    emu.sw_no_count(26usize, 2usize, 136u32, 2104956u32)?;
    emu.sw_no_count(20usize, 2usize, 140u32, 2104960u32)?;
    emu.sw_no_count(25usize, 2usize, 120u32, 2104964u32)?;
    emu.sw_no_count(20usize, 2usize, 124u32, 2104968u32)?;
    emu.sw_no_count(27usize, 2usize, 128u32, 2104972u32)?;
    emu.sw_no_count(20usize, 2usize, 132u32, 2104976u32)?;
    emu.adi_no_count(10usize, 2usize, 248u32, 2104980u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2104984u32);
    emu.apc_no_count(1usize, 2104984u32, 106496u32, 2104988u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2104992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 232u32, 2104996u32)?;
    emu.lw_no_count(19usize, 2usize, 256u32, 2105000u32)?;
    emu.lw_no_count(11usize, 2usize, 252u32, 2105004u32)?;
    emu.sbr_no_count(10usize, 10usize, 9usize, 2105008u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2105052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201edc));
    } else {
        emu.pc = 2105012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201eb4));
    }
}
#[inline(always)]
pub fn block_0x00201eb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 9usize, 2105016u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2105020u32);
    emu.apc_no_count(1usize, 2105020u32, 32768u32, 2105024u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 9usize, 19usize, 2105032u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2105036u32);
    emu.sw_no_count(9usize, 2usize, 240u32, 2105040u32)?;
    emu.adi_no_count(22usize, 22usize, 1u32, 2105044u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2104940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201e6c));
    } else {
        emu.pc = 2105048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201ed8));
    }
}
#[inline(always)]
pub fn block_0x00201ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105100u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201f0c));
}
#[inline(always)]
pub fn block_0x00201edc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 232u32, 2105056u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2105060u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2105064u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2105068u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2105072u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2105076u32);
    emu.apc_no_count(1usize, 2105076u32, 12288u32, 2105080u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 18usize, 0u32, 2105088u32);
    emu.lw_no_count(18usize, 2usize, 236u32, 2105092u32)?;
    emu.lw_no_count(9usize, 2usize, 240u32, 2105096u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2105100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201eb4));
}
#[inline]
pub fn block_0x00201f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 232u32, 2105104u32)?;
    emu.lw_no_count(11usize, 2usize, 236u32, 2105108u32)?;
    emu.lw_no_count(12usize, 2usize, 240u32, 2105112u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2105116u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2105120u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2105124u32)?;
    emu.lw_no_count(1usize, 2usize, 380u32, 2105128u32)?;
    emu.lw_no_count(8usize, 2usize, 376u32, 2105132u32)?;
    emu.lw_no_count(9usize, 2usize, 372u32, 2105136u32)?;
    emu.lw_no_count(18usize, 2usize, 368u32, 2105140u32)?;
    emu.lw_no_count(19usize, 2usize, 364u32, 2105144u32)?;
    emu.lw_no_count(20usize, 2usize, 360u32, 2105148u32)?;
    emu.lw_no_count(21usize, 2usize, 356u32, 2105152u32)?;
    emu.lw_no_count(22usize, 2usize, 352u32, 2105156u32)?;
    emu.lw_no_count(23usize, 2usize, 348u32, 2105160u32)?;
    emu.lw_no_count(24usize, 2usize, 344u32, 2105164u32)?;
    emu.lw_no_count(25usize, 2usize, 340u32, 2105168u32)?;
    emu.lw_no_count(26usize, 2usize, 336u32, 2105172u32)?;
    emu.lw_no_count(27usize, 2usize, 332u32, 2105176u32)?;
    emu.adi_no_count(2usize, 2usize, 384u32, 2105180u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105184u32;
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
pub fn block_0x00201f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105188u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966660u32, 2105192u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2105196u32);
    emu.adi_no_count(11usize, 0usize, 64u32, 2105200u32);
    emu.apc_no_count(1usize, 2105200u32, 106496u32, 2105204u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105212u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966828u32, 2105216u32);
    emu.adi_no_count(13usize, 10usize, 0u32, 2105220u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2105224u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2105228u32);
    emu.apc_no_count(6usize, 2105228u32, 36864u32, 2105232u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105236u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105240u32;
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
pub fn block_0x00201f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2105244u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2105248u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2105252u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2105256u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2105260u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2105264u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2105268u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2105272u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105276u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2105280u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2105284u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2105288u32);
    emu.apc_no_count(1usize, 2105288u32, 36864u32, 2105292u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2105344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202000));
    } else {
        emu.pc = 2105300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201fd4));
    }
}
#[inline(always)]
pub fn block_0x00201fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2105308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201fdc));
    } else {
        emu.pc = 2105304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201fd8));
    }
}
#[inline(always)]
pub fn block_0x00201fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 18usize, 0u32, 2105308u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2105308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201fdc));
}
#[inline]
pub fn block_0x00201fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2105312u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2105316u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2105320u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2105324u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2105328u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2105332u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2105336u32);
    emu.apc_no_count(6usize, 2105336u32, 32768u32, 2105340u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2105344u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2105348u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2105352u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2105356u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2105360u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2105364u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105368u32;
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
pub fn block_0x00202018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2105372u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2105376u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2105380u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2105384u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2105388u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2105392u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2105396u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2105400u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2105700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202164));
    } else {
        emu.pc = 2105404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020203c));
    }
}
#[inline(always)]
pub fn block_0x0020203c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2105408u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2105412u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2105440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202060));
    } else {
        emu.pc = 2105416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202048));
    }
}
#[inline(always)]
pub fn block_0x00202048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2105420u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105424u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2105428u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2105432u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2105436u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2105440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020212c));
}
#[inline]
pub fn block_0x00202060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2105444u32)?;
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2105448u32);
    emu.adi_no_count(6usize, 10usize, 4u32, 2105452u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2105456u32);
    emu.lbu_no_count(5usize, 10usize, 1u32, 2105460u32);
    emu.lbu_no_count(16usize, 10usize, 2u32, 2105464u32);
    emu.lbu_no_count(17usize, 10usize, 3u32, 2105468u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2105472u32);
    emu.sw_no_count(6usize, 11usize, 0u32, 2105476u32)?;
    emu.sw_no_count(15usize, 11usize, 4u32, 2105480u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2105876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202214));
    } else {
        emu.pc = 2105484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020208c));
    }
}
#[inline(always)]
pub fn block_0x0020208c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 4u32, 2105488u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2105512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020a8));
    } else {
        emu.pc = 2105492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202094));
    }
}
#[inline(always)]
pub fn block_0x00202094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105496u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2105500u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2105504u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2105508u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2105512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020212c));
}
#[inline]
pub fn block_0x002020a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 10usize, 8u32, 2105516u32);
    emu.adi_no_count(6usize, 12usize, 4294967288u32, 2105520u32);
    emu.lbu_no_count(15usize, 10usize, 4u32, 2105524u32);
    emu.lbu_no_count(29usize, 10usize, 5u32, 2105528u32);
    emu.lbu_no_count(7usize, 10usize, 6u32, 2105532u32);
    emu.lbu_no_count(28usize, 10usize, 7u32, 2105536u32);
    emu.adi_no_count(31usize, 0usize, 2u32, 2105540u32);
    emu.sw_no_count(30usize, 11usize, 0u32, 2105544u32)?;
    emu.sw_no_count(6usize, 11usize, 4u32, 2105548u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2105900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020222c));
    } else {
        emu.pc = 2105552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020d0));
    }
}
#[inline(always)]
pub fn block_0x002020d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 0usize, 4u32, 2105556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2105580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020ec));
    } else {
        emu.pc = 2105560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020d8));
    }
}
#[inline(always)]
pub fn block_0x002020d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105564u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2105568u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2105572u32)?;
    emu.sw_no_count(30usize, 2usize, 4u32, 2105576u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2105580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020212c));
}
#[inline]
pub fn block_0x002020ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 12u32, 2105584u32);
    emu.adi_no_count(18usize, 12usize, 4294967284u32, 2105588u32);
    emu.lbu_no_count(6usize, 10usize, 8u32, 2105592u32);
    emu.lbu_no_count(9usize, 10usize, 9u32, 2105596u32);
    emu.lbu_no_count(30usize, 10usize, 10u32, 2105600u32);
    emu.lbu_no_count(31usize, 10usize, 11u32, 2105604u32);
    emu.adi_no_count(20usize, 0usize, 3u32, 2105608u32);
    emu.sw_no_count(19usize, 11usize, 0u32, 2105612u32)?;
    emu.sw_no_count(18usize, 11usize, 4u32, 2105616u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2105924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202244));
    } else {
        emu.pc = 2105620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202114));
    }
}
#[inline(always)]
pub fn block_0x00202114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 4u32, 2105624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2105724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020217c));
    } else {
        emu.pc = 2105628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020211c));
    }
}
#[inline(always)]
pub fn block_0x0020211c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105632u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2105636u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2105640u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2105644u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2105644u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020212c));
}
#[inline(always)]
pub fn block_0x0020212c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2105648u32);
    emu.apc_no_count(1usize, 2105648u32, 36864u32, 2105652u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2105660u32);
    emu.sw_no_count(11usize, 8usize, 0u32, 2105664u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2105668u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2105668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202144));
}
#[inline(always)]
pub fn block_0x00202144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2105672u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2105676u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2105680u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2105684u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2105688u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2105692u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2105696u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105700u32;
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
pub fn block_0x00202164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105704u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967140u32, 2105708u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105712u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2105716u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2105720u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2105724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105944u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202258));
}
#[inline(never)]
pub fn block_0x0020217c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 8u32, 2105728u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2105732u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2105736u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2105740u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2105744u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2105748u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2105752u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2105756u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2105760u32);
    emu.orr_no_count(13usize, 5usize, 14usize, 2105764u32);
    emu.adi_no_count(14usize, 10usize, 16u32, 2105768u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2105772u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2105776u32);
    emu.orr_no_count(15usize, 29usize, 15usize, 2105780u32);
    emu.orr_no_count(17usize, 28usize, 7usize, 2105784u32);
    emu.lbu_no_count(5usize, 10usize, 12u32, 2105788u32);
    emu.lbu_no_count(7usize, 10usize, 13u32, 2105792u32);
    emu.lbu_no_count(28usize, 10usize, 14u32, 2105796u32);
    emu.lbu_no_count(10usize, 10usize, 15u32, 2105800u32);
    emu.orr_no_count(6usize, 9usize, 6usize, 2105804u32);
    emu.orr_no_count(29usize, 31usize, 30usize, 2105808u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2105812u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2105816u32)?;
    emu.orr_no_count(11usize, 16usize, 13usize, 2105820u32);
    emu.orr_no_count(12usize, 17usize, 15usize, 2105824u32);
    emu.orr_no_count(13usize, 29usize, 6usize, 2105828u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2105832u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2105836u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2105840u32);
    emu.orr_no_count(14usize, 7usize, 5usize, 2105844u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2105848u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2105852u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2105856u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2105860u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2105864u32)?;
    emu.sw_no_count(13usize, 8usize, 12u32, 2105868u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2105872u32)?;
    emu.add_memory_rw_events(38usize);
    let return_addr = 2105876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202144));
}
#[inline(always)]
pub fn block_0x00202214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105880u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967140u32, 2105884u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105888u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2105892u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2105896u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2105900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105944u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202258));
}
#[inline(always)]
pub fn block_0x0020222c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105904u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967140u32, 2105908u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105912u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2105916u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2105920u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2105924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105944u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202258));
}
#[inline(always)]
pub fn block_0x00202244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105928u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967140u32, 2105932u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105936u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2105940u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2105944u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2105944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202258));
}
#[inline(always)]
pub fn block_0x00202258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2105944u32, 12288u32, 2105948u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105956u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202138));
}
#[inline]
pub fn block_0x00202264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2105960u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2105964u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2105968u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2105972u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2105976u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2105980u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2105984u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2105988u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2106288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023b0));
    } else {
        emu.pc = 2105992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202288));
    }
}
#[inline(always)]
pub fn block_0x00202288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2105996u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2106000u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2106028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022ac));
    } else {
        emu.pc = 2106004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202294));
    }
}
#[inline(always)]
pub fn block_0x00202294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2106008u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106012u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2106016u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2106020u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2106024u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2106028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202378));
}
#[inline]
pub fn block_0x002022ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2106032u32)?;
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2106036u32);
    emu.adi_no_count(6usize, 10usize, 4u32, 2106040u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2106044u32);
    emu.lbu_no_count(5usize, 10usize, 1u32, 2106048u32);
    emu.lbu_no_count(16usize, 10usize, 2u32, 2106052u32);
    emu.lbu_no_count(17usize, 10usize, 3u32, 2106056u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2106060u32);
    emu.sw_no_count(6usize, 11usize, 0u32, 2106064u32)?;
    emu.sw_no_count(15usize, 11usize, 4u32, 2106068u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2106468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202464));
    } else {
        emu.pc = 2106072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022d8));
    }
}
#[inline(always)]
pub fn block_0x002022d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 4u32, 2106076u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2106100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022f4));
    } else {
        emu.pc = 2106080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022e0));
    }
}
#[inline(always)]
pub fn block_0x002022e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106084u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2106088u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2106092u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2106096u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2106100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202378));
}
#[inline]
pub fn block_0x002022f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 10usize, 8u32, 2106104u32);
    emu.adi_no_count(6usize, 12usize, 4294967288u32, 2106108u32);
    emu.lbu_no_count(15usize, 10usize, 4u32, 2106112u32);
    emu.lbu_no_count(29usize, 10usize, 5u32, 2106116u32);
    emu.lbu_no_count(7usize, 10usize, 6u32, 2106120u32);
    emu.lbu_no_count(28usize, 10usize, 7u32, 2106124u32);
    emu.adi_no_count(31usize, 0usize, 2u32, 2106128u32);
    emu.sw_no_count(30usize, 11usize, 0u32, 2106132u32)?;
    emu.sw_no_count(6usize, 11usize, 4u32, 2106136u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2106492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020247c));
    } else {
        emu.pc = 2106140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020231c));
    }
}
#[inline(always)]
pub fn block_0x0020231c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 0usize, 4u32, 2106144u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2106168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202338));
    } else {
        emu.pc = 2106148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202324));
    }
}
#[inline(always)]
pub fn block_0x00202324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106152u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2106156u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2106160u32)?;
    emu.sw_no_count(30usize, 2usize, 4u32, 2106164u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2106168u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202378));
}
#[inline]
pub fn block_0x00202338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 12u32, 2106172u32);
    emu.adi_no_count(18usize, 12usize, 4294967284u32, 2106176u32);
    emu.lbu_no_count(6usize, 10usize, 8u32, 2106180u32);
    emu.lbu_no_count(9usize, 10usize, 9u32, 2106184u32);
    emu.lbu_no_count(30usize, 10usize, 10u32, 2106188u32);
    emu.lbu_no_count(31usize, 10usize, 11u32, 2106192u32);
    emu.adi_no_count(20usize, 0usize, 3u32, 2106196u32);
    emu.sw_no_count(19usize, 11usize, 0u32, 2106200u32)?;
    emu.sw_no_count(18usize, 11usize, 4u32, 2106204u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2106516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202494));
    } else {
        emu.pc = 2106208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202360));
    }
}
#[inline(always)]
pub fn block_0x00202360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 4u32, 2106212u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2106312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023c8));
    } else {
        emu.pc = 2106216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202368));
    }
}
#[inline(always)]
pub fn block_0x00202368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106220u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2106224u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2106228u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2106232u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2106232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202378));
}
#[inline(always)]
pub fn block_0x00202378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2106236u32);
    emu.apc_no_count(1usize, 2106236u32, 36864u32, 2106240u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106244u32;
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
pub fn block_0x00202384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2106248u32);
    emu.sw_no_count(11usize, 8usize, 0u32, 2106252u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2106256u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2106256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202390));
}
#[inline(always)]
pub fn block_0x00202390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2106260u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2106264u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2106268u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2106272u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2106276u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2106280u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2106284u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106288u32;
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
pub fn block_0x002023b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106292u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967080u32, 2106296u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106300u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2106304u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2106308u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2106312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002024a8));
}
#[inline(never)]
pub fn block_0x002023c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 8u32, 2106316u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2106320u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2106324u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2106328u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2106332u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2106336u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2106340u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2106344u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2106348u32);
    emu.orr_no_count(13usize, 5usize, 14usize, 2106352u32);
    emu.adi_no_count(14usize, 10usize, 16u32, 2106356u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2106360u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2106364u32);
    emu.orr_no_count(15usize, 29usize, 15usize, 2106368u32);
    emu.orr_no_count(17usize, 28usize, 7usize, 2106372u32);
    emu.lbu_no_count(5usize, 10usize, 12u32, 2106376u32);
    emu.lbu_no_count(7usize, 10usize, 13u32, 2106380u32);
    emu.lbu_no_count(28usize, 10usize, 14u32, 2106384u32);
    emu.lbu_no_count(10usize, 10usize, 15u32, 2106388u32);
    emu.orr_no_count(6usize, 9usize, 6usize, 2106392u32);
    emu.adi_no_count(29usize, 0usize, 1u32, 2106396u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2106400u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2106404u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2106408u32)?;
    emu.orr_no_count(11usize, 16usize, 13usize, 2106412u32);
    emu.orr_no_count(12usize, 17usize, 15usize, 2106416u32);
    emu.orr_no_count(13usize, 30usize, 6usize, 2106420u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2106424u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2106428u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2106432u32);
    emu.orr_no_count(14usize, 7usize, 5usize, 2106436u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2106440u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2106444u32);
    emu.sw_no_count(29usize, 8usize, 0u32, 2106448u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2106452u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2106456u32)?;
    emu.sw_no_count(13usize, 8usize, 12u32, 2106460u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2106464u32)?;
    emu.add_memory_rw_events(39usize);
    let return_addr = 2106468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202390));
}
#[inline(always)]
pub fn block_0x00202464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106472u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967080u32, 2106476u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106480u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2106484u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2106488u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2106492u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002024a8));
}
#[inline(always)]
pub fn block_0x0020247c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106496u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967080u32, 2106500u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106504u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2106508u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2106512u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2106516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002024a8));
}
#[inline(always)]
pub fn block_0x00202494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106520u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967080u32, 2106524u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106528u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2106532u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2106536u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2106536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002024a8));
}
#[inline(always)]
pub fn block_0x002024a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2106536u32, 12288u32, 2106540u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106544u32;
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
pub fn block_0x002024b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2106548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202384));
}
#[inline(always)]
pub fn block_0x002024b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2106552u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2106556u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2106560u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2106564u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2106568u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2106572u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2106672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202530));
    } else {
        emu.pc = 2106576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024d0));
    }
}
#[inline(always)]
pub fn block_0x002024d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2106580u32)?;
    emu.lbu_no_count(9usize, 10usize, 0u32, 2106584u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2106588u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2106592u32);
    emu.sw_no_count(10usize, 11usize, 0u32, 2106596u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2106600u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2106716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020255c));
    } else {
        emu.pc = 2106604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024ec));
    }
}
#[inline(always)]
pub fn block_0x002024ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2106608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2106744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202578));
    } else {
        emu.pc = 2106612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002024f4));
    }
}
#[inline(always)]
pub fn block_0x002024f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2106616u32);
    emu.apc_no_count(1usize, 2106616u32, 0u32, 2106620u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2106628u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106632u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2106828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025cc));
    } else {
        emu.pc = 2106636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020250c));
    }
}
#[inline]
pub fn block_0x0020250c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2106640u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2106644u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2106648u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2106652u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2106656u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2106660u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2106664u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2106668u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106672u32;
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
pub fn block_0x00202530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2106676u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106680u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2106684u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2106688u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2106692u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2106696u32);
    emu.apc_no_count(1usize, 2106696u32, 32768u32, 2106700u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106708u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2106712u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2106716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002025b0));
}
#[inline(always)]
pub fn block_0x0020255c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106720u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2106724u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2106728u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2106732u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2106736u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2106740u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106744u32;
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
pub fn block_0x00202578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2106744u32, 28672u32, 2106748u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106752u32;
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
pub fn block_0x00202580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106756u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2106760u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2106764u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2106768u32);
    emu.apc_no_count(1usize, 2106768u32, 32768u32, 2106772u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025f8));
    } else {
        emu.pc = 2106780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020259c));
    }
}
#[inline(always)]
pub fn block_0x0020259c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106784u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2106788u32);
    emu.adi_no_count(12usize, 11usize, 3u32, 2106792u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2106796u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2106800u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2106800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002025b0));
}
#[inline(always)]
pub fn block_0x002025b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2106804u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2106808u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2106812u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2106816u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2106820u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2106824u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106828u32;
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
pub fn block_0x002025cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2106832u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2106836u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2106840u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2106844u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2106848u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2106852u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2106856u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2106860u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2106864u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2106868u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106872u32;
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
pub fn block_0x002025f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2106876u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2106880u32);
    emu.apc_no_count(1usize, 2106880u32, 102400u32, 2106884u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106888u32;
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
#[inline]
pub fn block_0x00202608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2106892u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2106896u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2106900u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2106904u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2106908u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2106912u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2106916u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2106920u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2106924u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2106928u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2107056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026b0));
    } else {
        emu.pc = 2106932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202634));
    }
}
#[inline(always)]
pub fn block_0x00202634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2106936u32)?;
    emu.lbu_no_count(9usize, 10usize, 0u32, 2106940u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2106944u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2106948u32);
    emu.sw_no_count(10usize, 11usize, 0u32, 2106952u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2106956u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2107100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026dc));
    } else {
        emu.pc = 2106960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202650));
    }
}
#[inline(always)]
pub fn block_0x00202650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2106964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2107112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026e8));
    } else {
        emu.pc = 2106968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202658));
    }
}
#[inline(always)]
pub fn block_0x00202658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2106972u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2106976u32);
    emu.apc_no_count(1usize, 2106976u32, 0u32, 2106980u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 8u32, 2106988u32)?;
    emu.lw_no_count(18usize, 2usize, 12u32, 2106992u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2106996u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2107036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020269c));
    } else {
        emu.pc = 2107000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202678));
    }
}
#[inline(always)]
pub fn block_0x00202678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 2usize, 16u32, 2107004u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107008u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107012u32);
    emu.apc_no_count(1usize, 2107012u32, 0u32, 2107016u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020268c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2107024u32)?;
    emu.lw_no_count(10usize, 2usize, 12u32, 2107028u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2107212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020274c));
    } else {
        emu.pc = 2107032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202698));
    }
}
#[inline(always)]
pub fn block_0x00202698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2107036u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2107036u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020269c));
}
#[inline(always)]
pub fn block_0x0020269c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107040u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2107044u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2107048u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2107052u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2107056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202728));
}
#[inline(always)]
pub fn block_0x002026b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2107060u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107064u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2107068u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2107072u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2107076u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107080u32);
    emu.apc_no_count(1usize, 2107080u32, 32768u32, 2107084u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107088u32;
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
pub fn block_0x002026d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107092u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2107096u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2107100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202720));
}
#[inline(always)]
pub fn block_0x002026dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107104u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2107108u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2107112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202728));
}
#[inline(always)]
pub fn block_0x002026e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107112u32, 28672u32, 2107116u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107120u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107124u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2107128u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2107132u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2107136u32);
    emu.apc_no_count(1usize, 2107136u32, 32768u32, 2107140u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020276c));
    } else {
        emu.pc = 2107148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020270c));
    }
}
#[inline(always)]
pub fn block_0x0020270c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107152u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2107156u32);
    emu.adi_no_count(12usize, 11usize, 3u32, 2107160u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2107164u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2107168u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2107168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202720));
}
#[inline(always)]
pub fn block_0x00202720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2107172u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2107176u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2107176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202728));
}
#[inline]
pub fn block_0x00202728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2107180u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2107184u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2107188u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2107192u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2107196u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2107200u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2107204u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2107208u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107212u32;
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
pub fn block_0x0020274c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 16u32, 2107216u32)?;
    emu.sw_no_count(19usize, 8usize, 0u32, 2107220u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2107224u32)?;
    emu.sw_no_count(20usize, 8usize, 8u32, 2107228u32)?;
    emu.sw_no_count(11usize, 8usize, 12u32, 2107232u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2107236u32)?;
    emu.sw_no_count(12usize, 8usize, 20u32, 2107240u32)?;
    emu.add_memory_rw_events(8usize);
    let return_addr = 2107244u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202728));
}
#[inline(always)]
pub fn block_0x0020276c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2107248u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2107252u32);
    emu.apc_no_count(1usize, 2107252u32, 102400u32, 2107256u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
