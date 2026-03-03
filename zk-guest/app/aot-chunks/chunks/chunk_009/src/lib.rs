pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2129740u32;
pub const PC_MAX: u32 = 2134052u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 129usize] = [
        block_0x00207f4c,
        block_0x00207fa0,
        block_0x00207fa4,
        block_0x00207fa8,
        block_0x00207fb8,
        block_0x00207fd0,
        block_0x00207fd4,
        block_0x00208008,
        block_0x00208044,
        block_0x00208064,
        block_0x00208070,
        block_0x00208094,
        block_0x002080a4,
        block_0x002080a8,
        block_0x002080b8,
        block_0x002080d4,
        block_0x002080e0,
        block_0x0020811c,
        block_0x00208134,
        block_0x00208154,
        block_0x00208174,
        block_0x00208178,
        block_0x00208194,
        block_0x002081d4,
        block_0x002081dc,
        block_0x002081e4,
        block_0x002081ec,
        block_0x0020820c,
        block_0x00208228,
        block_0x00208240,
        block_0x00208254,
        block_0x0020826c,
        block_0x00208280,
        block_0x002082c0,
        block_0x002082ec,
        block_0x002082f0,
        block_0x00208300,
        block_0x00208314,
        block_0x00208328,
        block_0x00208334,
        block_0x0020839c,
        block_0x002083a8,
        block_0x002083e0,
        block_0x002083ec,
        block_0x0020840c,
        block_0x00208428,
        block_0x00208444,
        block_0x00208460,
        block_0x00208488,
        block_0x00208494,
        block_0x002084b0,
        block_0x002084d4,
        block_0x002084e0,
        block_0x002084fc,
        block_0x00208518,
        block_0x0020854c,
        block_0x00208558,
        block_0x00208584,
        block_0x002085a0,
        block_0x002085bc,
        block_0x002085e4,
        block_0x002085f0,
        block_0x0020860c,
        block_0x00208634,
        block_0x00208640,
        block_0x0020865c,
        block_0x00208690,
        block_0x0020869c,
        block_0x002086b8,
        block_0x002086d4,
        block_0x0020871c,
        block_0x00208750,
        block_0x0020875c,
        block_0x00208778,
        block_0x00208794,
        block_0x002087c8,
        block_0x002087d4,
        block_0x002087f0,
        block_0x00208838,
        block_0x00208854,
        block_0x00208888,
        block_0x00208894,
        block_0x002088b0,
        block_0x002088cc,
        block_0x002088e8,
        block_0x00208910,
        block_0x0020891c,
        block_0x00208960,
        block_0x00208974,
        block_0x00208980,
        block_0x002089b4,
        block_0x002089c0,
        block_0x002089c8,
        block_0x00208c08,
        block_0x00208c24,
        block_0x00208c38,
        block_0x00208c50,
        block_0x00208c68,
        block_0x00208c78,
        block_0x00208c84,
        block_0x00208c94,
        block_0x00208cac,
        block_0x00208cc0,
        block_0x00208cd4,
        block_0x00208ce8,
        block_0x00208cf4,
        block_0x00208d24,
        block_0x00208d30,
        block_0x00208e2c,
        block_0x00208e4c,
        block_0x00208e60,
        block_0x00208e78,
        block_0x00208e90,
        block_0x00208eac,
        block_0x00208eb8,
        block_0x00208ed0,
        block_0x00208ee4,
        block_0x00208ef8,
        block_0x00208f18,
        block_0x00208f24,
        block_0x00208f54,
        block_0x00208f60,
        block_0x00208f90,
        block_0x00208f98,
        block_0x00208fd0,
        block_0x00208fd8,
        block_0x00208fdc,
        block_0x00208fe8,
        block_0x00209024,
    ];
    const IDX: [u16; 1079usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 3u16, 4u16, 0u16, 0u16,
        0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 13u16, 14u16, 0u16, 0u16, 0u16,
        15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 17u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 22u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 25u16, 0u16, 26u16, 0u16, 27u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16,
        0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16,
        50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        64u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16,
        68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16,
        77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 86u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16,
        0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16,
        0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        0u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16,
        0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16,
        115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 117u16,
        0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 119u16,
        0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 121u16, 0u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 123u16, 0u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 125u16, 0u16, 126u16, 127u16, 0u16, 0u16,
        128u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 129u16,
    ];
    if pc < 2129740u32 || pc > 2134052u32 {
        return None;
    }
    let word_offset = ((pc - 2129740u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00207f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2129744u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2129748u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2129752u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2129756u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2129760u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2129764u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2129768u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2129772u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2129776u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2129780u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2129784u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2129788u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2129792u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2129796u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2129800u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2129804u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2129808u32);
    let a = 0u32.wrapping_add(53248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129812u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966476u32, 2129816u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2129820u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2129828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207fa4));
    } else {
        emu.pc = 2129824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207fa0));
    }
}
#[inline(always)]
pub fn block_0x00207fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2129828u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2129828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207fa4));
}
#[inline(always)]
pub fn block_0x00207fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2130088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002080a8));
    } else {
        emu.pc = 2129832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207fa8));
    }
}
#[inline(always)]
pub fn block_0x00207fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2129836u32);
    emu.mul_no_count(19usize, 20usize, 10usize, 2129840u32);
    emu.apc_no_count(1usize, 2129840u32, 8192u32, 2129844u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129848u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129852u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2129856u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2129860u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2129864u32);
    emu.apc_no_count(1usize, 2129864u32, 12288u32, 2129868u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129872u32;
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
pub fn block_0x00207fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2130204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020811c));
    } else {
        emu.pc = 2129876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207fd4));
    }
}
#[inline]
pub fn block_0x00207fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 0u32, 2129880u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2129884u32)?;
    emu.sw_no_count(0usize, 2usize, 8u32, 2129888u32)?;
    emu.adi_no_count(22usize, 2usize, 32u32, 2129892u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2129896u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 4294966092u32, 2129900u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2129904u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966112u32, 2129908u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2129912u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2129916u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294965792u32, 2129920u32);
    emu.adi_no_count(24usize, 0usize, 20u32, 2129924u32);
    emu.add_memory_rw_events(13usize);
    let return_addr = 2129928u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208044));
}
#[inline]
pub fn block_0x00208008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2129932u32)?;
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2129936u32);
    emu.mul_no_count(11usize, 27usize, 24usize, 2129940u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2129944u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2129948u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2129952u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2129956u32)?;
    emu.adi_no_count(27usize, 27usize, 1u32, 2129960u32);
    emu.sw_no_count(26usize, 10usize, 0u32, 2129964u32)?;
    emu.sw_no_count(25usize, 10usize, 4u32, 2129968u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2129972u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2129976u32)?;
    emu.sw_no_count(13usize, 10usize, 16u32, 2129980u32)?;
    emu.sw_no_count(27usize, 2usize, 8u32, 2129984u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2130104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002080b8));
    } else {
        emu.pc = 2129988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208044));
    }
}
#[inline(always)]
pub fn block_0x00208044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 24u32, 2129992u32);
    emu.adi_no_count(13usize, 0usize, 12u32, 2129996u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2130000u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2130004u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2130008u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2130012u32);
    emu.apc_no_count(1usize, 2130012u32, 4294946816u32, 2130016u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 24u32, 2130024u32)?;
    emu.lw_no_count(25usize, 2usize, 28u32, 2130028u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2130132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002080d4));
    } else {
        emu.pc = 2130032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208070));
    }
}
#[inline]
pub fn block_0x00208070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 22usize, 0u32, 2130036u32)?;
    emu.lw_no_count(11usize, 22usize, 4u32, 2130040u32)?;
    emu.lw_no_count(12usize, 22usize, 8u32, 2130044u32)?;
    emu.lw_no_count(13usize, 2usize, 0u32, 2130048u32)?;
    emu.lw_no_count(27usize, 2usize, 8u32, 2130052u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2130056u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2130060u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2130064u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2129928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208008));
    } else {
        emu.pc = 2130068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208094));
    }
}
#[inline(always)]
pub fn block_0x00208094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2130072u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2130076u32);
    emu.apc_no_count(1usize, 2130076u32, 4294950912u32, 2130080u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002080a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2130088u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208008));
}
#[inline(always)]
pub fn block_0x002080a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2130092u32);
    emu.sw_no_count(0usize, 2usize, 0u32, 2130096u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2130100u32)?;
    emu.sw_no_count(0usize, 2usize, 8u32, 2130104u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2130104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002080b8));
}
#[inline(always)]
pub fn block_0x002080b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2130108u32)?;
    emu.lw_no_count(11usize, 2usize, 4u32, 2130112u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2130116u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2130120u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2130124u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2130128u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2130132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130144u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002080e0));
}
#[inline(always)]
pub fn block_0x002080d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130136u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2130140u32)?;
    emu.sw_no_count(25usize, 8usize, 4u32, 2130144u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2130144u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002080e0));
}
#[inline]
pub fn block_0x002080e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 92u32, 2130148u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2130152u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2130156u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2130160u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2130164u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2130168u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2130172u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2130176u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2130180u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2130184u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2130188u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2130192u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2130196u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2130200u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130204u32;
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
pub fn block_0x0020811c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130208u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965776u32, 2130212u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2130216u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2130220u32);
    emu.apc_no_count(1usize, 2130220u32, 81920u32, 2130224u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2130232u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2130236u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2130240u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2130244u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2130248u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2130252u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2130256u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2130324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208194));
    } else {
        emu.pc = 2130260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208154));
    }
}
#[inline(always)]
pub fn block_0x00208154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2130264u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130268u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2130272u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2130276u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2130280u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2130284u32);
    emu.apc_no_count(1usize, 2130284u32, 12288u32, 2130288u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130292u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966040u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2130296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2130296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208178));
}
#[inline(always)]
pub fn block_0x00208178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2130300u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2130304u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2130308u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2130312u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2130316u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2130320u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130324u32;
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
pub fn block_0x00208194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2130328u32)?;
    emu.lbu_no_count(14usize, 10usize, 0u32, 2130332u32);
    emu.lbu_no_count(15usize, 10usize, 1u32, 2130336u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2130340u32);
    emu.lbu_no_count(16usize, 10usize, 2u32, 2130344u32);
    emu.lbu_no_count(17usize, 10usize, 3u32, 2130348u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2130352u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2130356u32);
    emu.adi_no_count(15usize, 10usize, 4u32, 2130360u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2130364u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2130368u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2130372u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2130376u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2130380u32)?;
    emu.sw_no_count(13usize, 11usize, 4u32, 2130384u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2130516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208254));
    } else {
        emu.pc = 2130388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002081d4));
    }
}
#[inline(always)]
pub fn block_0x002081d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2130392u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2130472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208228));
    } else {
        emu.pc = 2130396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002081dc));
    }
}
#[inline(always)]
pub fn block_0x002081dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2130400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2130624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002082c0));
    } else {
        emu.pc = 2130404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002081e4));
    }
}
#[inline(always)]
pub fn block_0x002081e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 3u32, 2130408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2130560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208280));
    } else {
        emu.pc = 2130412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002081ec));
    }
}
#[inline(always)]
pub fn block_0x002081ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2130416u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130420u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2130424u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2130428u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2130432u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2130436u32);
    emu.apc_no_count(1usize, 2130436u32, 12288u32, 2130440u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020820c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2130448u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2130452u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2130456u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2130460u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2130464u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2130468u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130472u32;
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
pub fn block_0x00208228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130476u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965884u32, 2130480u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2130484u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2130488u32);
    emu.apc_no_count(1usize, 2130488u32, 4294942720u32, 2130492u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(44u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2130500u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2130504u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2130508u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2130512u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130516u32;
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
pub fn block_0x00208254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130520u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965824u32, 2130524u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2130528u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2130532u32);
    emu.apc_no_count(1usize, 2130532u32, 4294942720u32, 2130536u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020826c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2130544u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2130548u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2130552u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2130556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130560u32;
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
pub fn block_0x00208280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 8u32, 2130564u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2130568u32);
    emu.lbu_no_count(14usize, 10usize, 4u32, 2130572u32);
    emu.lbu_no_count(15usize, 10usize, 5u32, 2130576u32);
    emu.lbu_no_count(16usize, 10usize, 6u32, 2130580u32);
    emu.lbu_no_count(10usize, 10usize, 7u32, 2130584u32);
    emu.sw_no_count(13usize, 11usize, 0u32, 2130588u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2130592u32)?;
    emu.adi_no_count(11usize, 0usize, 2u32, 2130596u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2130600u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2130604u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2130608u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2130612u32);
    emu.orr_no_count(10usize, 10usize, 16usize, 2130616u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2130620u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2130624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208178));
}
#[inline]
pub fn block_0x002082c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2130628u32);
    emu.sb_no_count(10usize, 2usize, 0u32, 2130632u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2130636u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2130640u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130644u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965940u32, 2130648u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2130652u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965948u32, 2130656u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2130660u32);
    emu.apc_no_count(1usize, 2130660u32, 4294955008u32, 2130664u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130668u32;
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
pub fn block_0x002082ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2130672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2130292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208174));
}
#[inline(always)]
pub fn block_0x002082f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2130676u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2130680u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130684u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2130708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208314));
    } else {
        emu.pc = 2130688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208300));
    }
}
#[inline(always)]
pub fn block_0x00208300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 15u32, 2130692u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130696u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966780u32, 2130700u32);
    emu.apc_no_count(6usize, 2130700u32, 106496u32, 2130704u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130708u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 13u32, 2130712u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130716u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966795u32, 2130720u32);
    emu.apc_no_count(6usize, 2130720u32, 106496u32, 2130724u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130728u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2130732u32)?;
    emu.apc_no_count(6usize, 2130732u32, 65536u32, 2130736u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130740u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00208334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2130744u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2130748u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2130752u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2130756u32)?;
    emu.adi_no_count(15usize, 10usize, 8u32, 2130760u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2130764u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2130768u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966284u32, 2130772u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2130776u32);
    emu.adi_no_count(7usize, 0usize, 8u32, 2130780u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130784u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966300u32, 2130788u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130792u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 584u32, 2130796u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2130800u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966268u32, 2130804u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2130808u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966740u32, 2130812u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2130816u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2130820u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2130824u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2130828u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2130832u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2130836u32);
    emu.apc_no_count(1usize, 2130836u32, 106496u32, 2130840u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130844u32;
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
#[inline(always)]
pub fn block_0x0020839c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2130848u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2130852u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130856u32;
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
pub fn block_0x002083a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2130860u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2130864u32)?;
    emu.adi_no_count(15usize, 11usize, 0u32, 2130868u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2130872u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2130876u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130880u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966392u32, 2130884u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2130888u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966376u32, 2130892u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2130896u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2130900u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2130904u32);
    emu.apc_no_count(1usize, 2130904u32, 106496u32, 2130908u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002083e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2130916u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2130920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130924u32;
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
pub fn block_0x002083ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2130928u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2130932u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2130936u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2130940u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2130944u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2130948u32);
    emu.apc_no_count(6usize, 2130948u32, 65536u32, 2130952u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2130956u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020840c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2130960u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2130964u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2130968u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2130972u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2130976u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2130980u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2131012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208444));
    } else {
        emu.pc = 2130984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208428));
    }
}
#[inline(always)]
pub fn block_0x00208428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2130988u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2130992u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2130996u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965660u32, 2131000u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2131004u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2131008u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2131012u32;
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
pub fn block_0x00208444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 13usize, 4294967292u32, 2131016u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2131020u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131024u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965660u32, 2131028u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2131032u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2131036u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2131040u32;
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
pub fn block_0x00208460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2131044u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2131048u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131052u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 620u32, 2131056u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966308u32, 2131064u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2131068u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2131072u32);
    emu.apc_no_count(1usize, 2131072u32, 106496u32, 2131076u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2131084u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131088u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131092u32;
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
pub fn block_0x00208494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131096u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966366u32, 2131100u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2131104u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2131108u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131112u32);
    emu.apc_no_count(6usize, 2131112u32, 106496u32, 2131116u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131120u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002084b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 8u32, 2131124u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131128u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966348u32, 2131132u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131136u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966332u32, 2131140u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2131144u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2131148u32);
    emu.apc_no_count(1usize, 2131148u32, 106496u32, 2131152u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131156u32;
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
pub fn block_0x002084d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2131160u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131164u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131168u32;
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
pub fn block_0x002084e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131172u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966353u32, 2131176u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2131180u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2131184u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131188u32);
    emu.apc_no_count(6usize, 2131188u32, 106496u32, 2131192u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131196u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002084fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131200u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966324u32, 2131204u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2131208u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2131212u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131216u32);
    emu.apc_no_count(6usize, 2131216u32, 106496u32, 2131220u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131224u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2131228u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2131232u32)?;
    emu.adi_no_count(15usize, 11usize, 0u32, 2131236u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2131240u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131244u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966392u32, 2131248u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131252u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966376u32, 2131256u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2131260u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2131264u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2131268u32);
    emu.apc_no_count(1usize, 2131268u32, 106496u32, 2131272u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(408u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020854c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2131280u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2131284u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131288u32;
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
pub fn block_0x00208558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2131292u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2131296u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2131300u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2131304u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2131308u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131312u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965680u32, 2131316u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2131320u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2131324u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2131328u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2131332u32;
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
pub fn block_0x00208584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131336u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966748u32, 2131340u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2131344u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131348u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131352u32);
    emu.apc_no_count(6usize, 2131352u32, 106496u32, 2131356u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131360u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002085a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966627u32, 2131368u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2131372u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131376u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131380u32);
    emu.apc_no_count(6usize, 2131380u32, 106496u32, 2131384u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131388u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002085bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2131392u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2131396u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131400u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966624u32, 2131404u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131408u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966608u32, 2131412u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2131416u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2131420u32);
    emu.apc_no_count(1usize, 2131420u32, 106496u32, 2131424u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131428u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002085e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2131432u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131436u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131440u32;
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
pub fn block_0x002085f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131444u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966585u32, 2131448u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2131452u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131456u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131460u32);
    emu.apc_no_count(6usize, 2131460u32, 106496u32, 2131464u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131468u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020860c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2131472u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2131476u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131480u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966496u32, 2131484u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131488u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966480u32, 2131492u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2131496u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2131500u32);
    emu.apc_no_count(1usize, 2131500u32, 106496u32, 2131504u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2131512u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131516u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131520u32;
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
pub fn block_0x00208640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131524u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966708u32, 2131528u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2131532u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131536u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131540u32);
    emu.apc_no_count(6usize, 2131540u32, 106496u32, 2131544u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131548u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020865c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2131552u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2131556u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131560u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966560u32, 2131564u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131568u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966570u32, 2131572u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2131576u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966544u32, 2131580u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2131584u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2131588u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2131592u32);
    emu.apc_no_count(1usize, 2131592u32, 106496u32, 2131596u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2131604u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131608u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131612u32;
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
pub fn block_0x0020869c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131616u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966404u32, 2131620u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2131624u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131628u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131632u32);
    emu.apc_no_count(6usize, 2131632u32, 106496u32, 2131636u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131640u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002086b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131644u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1115u32, 2131648u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2131652u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131656u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131660u32);
    emu.apc_no_count(6usize, 2131660u32, 106496u32, 2131664u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131668u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002086d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4u32, 2131672u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2131676u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2131680u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2131684u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966432u32, 2131688u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2131692u32);
    emu.adi_no_count(7usize, 0usize, 10u32, 2131696u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966448u32, 2131704u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131708u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966458u32, 2131712u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2131716u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966416u32, 2131720u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2131724u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966470u32, 2131728u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2131732u32);
    emu.adi_no_count(14usize, 0usize, 12u32, 2131736u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2131740u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2132320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208960));
}
#[inline]
pub fn block_0x0020871c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2131744u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2131748u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131752u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966519u32, 2131756u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131760u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966516u32, 2131764u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2131768u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966500u32, 2131772u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2131776u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2131780u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2131784u32);
    emu.apc_no_count(1usize, 2131784u32, 106496u32, 2131788u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2131796u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131800u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131804u32;
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
pub fn block_0x0020875c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131808u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966398u32, 2131812u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2131816u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131820u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131824u32);
    emu.apc_no_count(6usize, 2131824u32, 106496u32, 2131828u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131832u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131836u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966633u32, 2131840u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2131844u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131848u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131852u32);
    emu.apc_no_count(6usize, 2131852u32, 106496u32, 2131856u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131860u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2131864u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2131868u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131872u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966700u32, 2131876u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131880u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 588u32, 2131884u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2131888u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966684u32, 2131892u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2131896u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2131900u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2131904u32);
    emu.apc_no_count(1usize, 2131904u32, 106496u32, 2131908u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131912u32;
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
pub fn block_0x002087c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2131916u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131924u32;
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
pub fn block_0x002087d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131928u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1131u32, 2131932u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2131936u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2131940u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2131944u32);
    emu.apc_no_count(6usize, 2131944u32, 106496u32, 2131948u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2131952u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002087f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4u32, 2131956u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2131960u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2131964u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2131968u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966432u32, 2131972u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2131976u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2131980u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131984u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966710u32, 2131988u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131992u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966722u32, 2131996u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2132000u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966416u32, 2132004u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2132008u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966729u32, 2132012u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2132016u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2132020u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2132024u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2132320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208960));
}
#[inline(always)]
pub fn block_0x00208838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132028u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966596u32, 2132032u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2132036u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2132040u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132044u32);
    emu.apc_no_count(6usize, 2132044u32, 106496u32, 2132048u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2132052u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2132056u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2132060u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132064u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966392u32, 2132068u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2132072u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966516u32, 2132076u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2132080u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966500u32, 2132084u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2132088u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2132092u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2132096u32);
    emu.apc_no_count(1usize, 2132096u32, 106496u32, 2132100u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2132108u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132112u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132116u32;
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
pub fn block_0x00208894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132120u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1083u32, 2132124u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2132128u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2132132u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132136u32);
    emu.apc_no_count(6usize, 2132136u32, 106496u32, 2132140u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2132144u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002088b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966573u32, 2132152u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2132156u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2132160u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132164u32);
    emu.apc_no_count(6usize, 2132164u32, 106496u32, 2132168u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2132172u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002088cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966531u32, 2132180u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2132184u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2132188u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132192u32);
    emu.apc_no_count(6usize, 2132192u32, 106496u32, 2132196u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2132200u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002088e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2132204u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2132208u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132212u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 624u32, 2132216u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2132220u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966740u32, 2132224u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2132228u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2132232u32);
    emu.apc_no_count(1usize, 2132232u32, 106496u32, 2132236u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132240u32;
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
pub fn block_0x00208910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2132244u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132248u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132252u32;
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
pub fn block_0x0020891c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 1u32, 2132256u32);
    emu.adi_no_count(12usize, 12usize, 4u32, 2132260u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2132264u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2132268u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966500u32, 2132272u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2132276u32);
    emu.adi_no_count(7usize, 0usize, 6u32, 2132280u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132284u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966664u32, 2132288u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2132292u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966756u32, 2132296u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2132300u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966648u32, 2132304u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2132308u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966677u32, 2132312u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2132316u32);
    emu.adi_no_count(14usize, 0usize, 8u32, 2132320u32);
    emu.add_memory_rw_events(17usize);
    emu.pc = 2132320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208960));
}
#[inline(always)]
pub fn block_0x00208960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(7usize, 2usize, 0u32, 2132324u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2132328u32)?;
    emu.sw_no_count(5usize, 2usize, 8u32, 2132332u32)?;
    emu.apc_no_count(1usize, 2132332u32, 106496u32, 2132336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2132344u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132348u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132352u32;
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
pub fn block_0x00208980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2132356u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2132360u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2132364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966756u32, 2132368u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2132372u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966516u32, 2132376u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2132380u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966500u32, 2132384u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2132388u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2132392u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2132396u32);
    emu.apc_no_count(1usize, 2132396u32, 106496u32, 2132400u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002089b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2132408u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2132412u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2132416u32;
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
pub fn block_0x002089c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 64u32, 2132420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2133124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c84));
    } else {
        emu.pc = 2132424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002089c8));
    }
}
#[inline(never)]
pub fn block_0x002089c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 144u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967056u32, 2132428u32);
    emu.sw_no_count(1usize, 2usize, 236u32, 2132432u32)?;
    emu.sw_no_count(8usize, 2usize, 232u32, 2132436u32)?;
    emu.sw_no_count(9usize, 2usize, 228u32, 2132440u32)?;
    emu.sw_no_count(18usize, 2usize, 224u32, 2132444u32)?;
    emu.sw_no_count(19usize, 2usize, 220u32, 2132448u32)?;
    emu.sw_no_count(20usize, 2usize, 216u32, 2132452u32)?;
    emu.sw_no_count(21usize, 2usize, 212u32, 2132456u32)?;
    emu.sw_no_count(22usize, 2usize, 208u32, 2132460u32)?;
    emu.sw_no_count(23usize, 2usize, 204u32, 2132464u32)?;
    emu.sw_no_count(24usize, 2usize, 200u32, 2132468u32)?;
    emu.sw_no_count(25usize, 2usize, 196u32, 2132472u32)?;
    emu.sw_no_count(26usize, 2usize, 192u32, 2132476u32)?;
    emu.sw_no_count(27usize, 2usize, 188u32, 2132480u32)?;
    emu.sw_no_count(10usize, 2usize, 120u32, 2132484u32)?;
    emu.lbu_no_count(26usize, 11usize, 0u32, 2132488u32);
    emu.lbu_no_count(22usize, 11usize, 1u32, 2132492u32);
    emu.lbu_no_count(23usize, 11usize, 2u32, 2132496u32);
    emu.lbu_no_count(24usize, 11usize, 3u32, 2132500u32);
    emu.lbu_no_count(25usize, 11usize, 4u32, 2132504u32);
    emu.lbu_no_count(18usize, 11usize, 5u32, 2132508u32);
    emu.lbu_no_count(19usize, 11usize, 6u32, 2132512u32);
    emu.lbu_no_count(20usize, 11usize, 7u32, 2132516u32);
    emu.lbu_no_count(21usize, 11usize, 8u32, 2132520u32);
    emu.lbu_no_count(29usize, 11usize, 9u32, 2132524u32);
    emu.lbu_no_count(30usize, 11usize, 10u32, 2132528u32);
    emu.lbu_no_count(31usize, 11usize, 11u32, 2132532u32);
    emu.lbu_no_count(9usize, 11usize, 12u32, 2132536u32);
    emu.lbu_no_count(5usize, 11usize, 13u32, 2132540u32);
    emu.lbu_no_count(6usize, 11usize, 14u32, 2132544u32);
    emu.lbu_no_count(7usize, 11usize, 15u32, 2132548u32);
    emu.lbu_no_count(28usize, 11usize, 16u32, 2132552u32);
    emu.lbu_no_count(14usize, 11usize, 17u32, 2132556u32);
    emu.lbu_no_count(15usize, 11usize, 18u32, 2132560u32);
    emu.lbu_no_count(16usize, 11usize, 19u32, 2132564u32);
    emu.lbu_no_count(17usize, 11usize, 20u32, 2132568u32);
    emu.lbu_no_count(10usize, 11usize, 21u32, 2132572u32);
    emu.sw_no_count(10usize, 2usize, 116u32, 2132576u32)?;
    emu.lbu_no_count(12usize, 11usize, 22u32, 2132580u32);
    emu.lbu_no_count(13usize, 11usize, 23u32, 2132584u32);
    emu.sb_no_count(26usize, 2usize, 124u32, 2132588u32);
    emu.lbu_no_count(26usize, 11usize, 24u32, 2132592u32);
    emu.lbu_no_count(27usize, 11usize, 25u32, 2132596u32);
    emu.lbu_no_count(8usize, 11usize, 26u32, 2132600u32);
    emu.lbu_no_count(10usize, 11usize, 27u32, 2132604u32);
    emu.sb_no_count(22usize, 2usize, 125u32, 2132608u32);
    emu.sb_no_count(23usize, 2usize, 126u32, 2132612u32);
    emu.sb_no_count(24usize, 2usize, 127u32, 2132616u32);
    emu.sb_no_count(25usize, 2usize, 128u32, 2132620u32);
    emu.lbu_no_count(23usize, 11usize, 28u32, 2132624u32);
    emu.lbu_no_count(22usize, 11usize, 29u32, 2132628u32);
    emu.lbu_no_count(24usize, 11usize, 30u32, 2132632u32);
    emu.lbu_no_count(1usize, 11usize, 31u32, 2132636u32);
    emu.sb_no_count(18usize, 2usize, 129u32, 2132640u32);
    emu.sb_no_count(19usize, 2usize, 130u32, 2132644u32);
    emu.sb_no_count(20usize, 2usize, 131u32, 2132648u32);
    emu.sb_no_count(21usize, 2usize, 132u32, 2132652u32);
    emu.lbu_no_count(18usize, 11usize, 32u32, 2132656u32);
    emu.sw_no_count(18usize, 2usize, 112u32, 2132660u32)?;
    emu.lbu_no_count(18usize, 11usize, 33u32, 2132664u32);
    emu.sw_no_count(18usize, 2usize, 108u32, 2132668u32)?;
    emu.lbu_no_count(18usize, 11usize, 34u32, 2132672u32);
    emu.sw_no_count(18usize, 2usize, 104u32, 2132676u32)?;
    emu.lbu_no_count(18usize, 11usize, 35u32, 2132680u32);
    emu.sw_no_count(18usize, 2usize, 100u32, 2132684u32)?;
    emu.sb_no_count(29usize, 2usize, 133u32, 2132688u32);
    emu.sb_no_count(30usize, 2usize, 134u32, 2132692u32);
    emu.sb_no_count(31usize, 2usize, 135u32, 2132696u32);
    emu.sb_no_count(9usize, 2usize, 136u32, 2132700u32);
    emu.lbu_no_count(29usize, 11usize, 36u32, 2132704u32);
    emu.sw_no_count(29usize, 2usize, 96u32, 2132708u32)?;
    emu.lbu_no_count(29usize, 11usize, 37u32, 2132712u32);
    emu.sw_no_count(29usize, 2usize, 92u32, 2132716u32)?;
    emu.lbu_no_count(29usize, 11usize, 38u32, 2132720u32);
    emu.sw_no_count(29usize, 2usize, 88u32, 2132724u32)?;
    emu.lbu_no_count(29usize, 11usize, 39u32, 2132728u32);
    emu.sw_no_count(29usize, 2usize, 84u32, 2132732u32)?;
    emu.sb_no_count(5usize, 2usize, 137u32, 2132736u32);
    emu.sb_no_count(6usize, 2usize, 138u32, 2132740u32);
    emu.sb_no_count(7usize, 2usize, 139u32, 2132744u32);
    emu.sb_no_count(28usize, 2usize, 140u32, 2132748u32);
    emu.lbu_no_count(5usize, 11usize, 40u32, 2132752u32);
    emu.sw_no_count(5usize, 2usize, 80u32, 2132756u32)?;
    emu.lbu_no_count(5usize, 11usize, 41u32, 2132760u32);
    emu.sw_no_count(5usize, 2usize, 76u32, 2132764u32)?;
    emu.lbu_no_count(5usize, 11usize, 42u32, 2132768u32);
    emu.sw_no_count(5usize, 2usize, 72u32, 2132772u32)?;
    emu.lbu_no_count(5usize, 11usize, 43u32, 2132776u32);
    emu.sw_no_count(5usize, 2usize, 68u32, 2132780u32)?;
    emu.sb_no_count(14usize, 2usize, 141u32, 2132784u32);
    emu.sb_no_count(15usize, 2usize, 142u32, 2132788u32);
    emu.sb_no_count(16usize, 2usize, 143u32, 2132792u32);
    emu.sb_no_count(17usize, 2usize, 144u32, 2132796u32);
    emu.lbu_no_count(14usize, 11usize, 44u32, 2132800u32);
    emu.sw_no_count(14usize, 2usize, 64u32, 2132804u32)?;
    emu.lbu_no_count(14usize, 11usize, 45u32, 2132808u32);
    emu.sw_no_count(14usize, 2usize, 60u32, 2132812u32)?;
    emu.lbu_no_count(14usize, 11usize, 46u32, 2132816u32);
    emu.sw_no_count(14usize, 2usize, 56u32, 2132820u32)?;
    emu.lbu_no_count(14usize, 11usize, 47u32, 2132824u32);
    emu.sw_no_count(14usize, 2usize, 52u32, 2132828u32)?;
    emu.lw_no_count(14usize, 2usize, 116u32, 2132832u32)?;
    emu.sb_no_count(14usize, 2usize, 145u32, 2132836u32);
    emu.sb_no_count(12usize, 2usize, 146u32, 2132840u32);
    emu.sb_no_count(13usize, 2usize, 147u32, 2132844u32);
    emu.sb_no_count(26usize, 2usize, 148u32, 2132848u32);
    emu.lbu_no_count(12usize, 11usize, 48u32, 2132852u32);
    emu.sw_no_count(12usize, 2usize, 116u32, 2132856u32)?;
    emu.lbu_no_count(12usize, 11usize, 49u32, 2132860u32);
    emu.sw_no_count(12usize, 2usize, 48u32, 2132864u32)?;
    emu.lbu_no_count(12usize, 11usize, 50u32, 2132868u32);
    emu.sw_no_count(12usize, 2usize, 44u32, 2132872u32)?;
    emu.lbu_no_count(12usize, 11usize, 51u32, 2132876u32);
    emu.sw_no_count(12usize, 2usize, 40u32, 2132880u32)?;
    emu.sb_no_count(27usize, 2usize, 149u32, 2132884u32);
    emu.sb_no_count(8usize, 2usize, 150u32, 2132888u32);
    emu.sb_no_count(10usize, 2usize, 151u32, 2132892u32);
    emu.sb_no_count(23usize, 2usize, 152u32, 2132896u32);
    emu.lbu_no_count(9usize, 11usize, 52u32, 2132900u32);
    emu.lbu_no_count(25usize, 11usize, 53u32, 2132904u32);
    emu.lbu_no_count(26usize, 11usize, 54u32, 2132908u32);
    emu.lbu_no_count(27usize, 11usize, 55u32, 2132912u32);
    emu.lbu_no_count(23usize, 11usize, 56u32, 2132916u32);
    emu.lbu_no_count(10usize, 11usize, 57u32, 2132920u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2132924u32)?;
    emu.lbu_no_count(10usize, 11usize, 58u32, 2132928u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2132932u32)?;
    emu.lbu_no_count(10usize, 11usize, 59u32, 2132936u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2132940u32)?;
    emu.lbu_no_count(10usize, 11usize, 60u32, 2132944u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2132948u32)?;
    emu.lbu_no_count(10usize, 11usize, 61u32, 2132952u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2132956u32)?;
    emu.lbu_no_count(10usize, 11usize, 62u32, 2132960u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2132964u32)?;
    emu.lbu_no_count(10usize, 11usize, 63u32, 2132968u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2132972u32)?;
    emu.sb_no_count(22usize, 2usize, 153u32, 2132976u32);
    emu.sb_no_count(24usize, 2usize, 154u32, 2132980u32);
    emu.sb_no_count(1usize, 2usize, 155u32, 2132984u32);
    emu.adi_no_count(10usize, 2usize, 156u32, 2132988u32);
    emu.adi_no_count(11usize, 2usize, 124u32, 2132992u32);
    emu.apc_no_count(1usize, 2132992u32, 40960u32, 2132996u32);
    emu.add_memory_rw_events(144usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208c08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2133004u32;
    emu.update_insn_clock();
    emu.lw_no_count(8usize, 2usize, 156u32, 2133008u32)?;
    emu.lw_no_count(18usize, 2usize, 160u32, 2133012u32)?;
    emu.lw_no_count(19usize, 2usize, 164u32, 2133016u32)?;
    emu.adi_no_count(10usize, 10usize, 1361u32, 2133020u32);
    emu.sltru_no_count(10usize, 8usize, 10usize, 2133024u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2133140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c94));
    } else {
        emu.pc = 2133028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c24));
    }
}
#[inline(always)]
pub fn block_0x00208c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 10usize, 2133032u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133036u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133040u32);
    emu.lw_no_count(20usize, 2usize, 168u32, 2133044u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2133164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cac));
    } else {
        emu.pc = 2133048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c38));
    }
}
#[inline(always)]
pub fn block_0x00208c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 19usize, 10usize, 2133052u32);
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2133056u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2133060u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2133064u32);
    emu.lw_no_count(21usize, 2usize, 172u32, 2133068u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2133184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cc0));
    } else {
        emu.pc = 2133072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c50));
    }
}
#[inline(always)]
pub fn block_0x00208c50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 20usize, 10usize, 2133076u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2133080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965933u32, 2133084u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2133088u32);
    emu.lw_no_count(22usize, 2usize, 176u32, 2133092u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2133204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cd4));
    } else {
        emu.pc = 2133096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c68));
    }
}
#[inline(always)]
pub fn block_0x00208c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 21usize, 10usize, 2133100u32);
    emu.sltiu_no_count(10usize, 10usize, 4294967295u32, 2133104u32);
    emu.lw_no_count(11usize, 2usize, 180u32, 2133108u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2133224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ce8));
    } else {
        emu.pc = 2133112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c78));
    }
}
#[inline(always)]
pub fn block_0x00208c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 22usize, 10usize, 2133116u32);
    emu.sltiu_no_count(10usize, 10usize, 4294967295u32, 2133120u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2133124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208cf4));
}
#[inline(always)]
pub fn block_0x00208c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2133128u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2133132u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2133136u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133140u32;
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
pub fn block_0x00208c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 18usize, 10usize, 2133144u32);
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2133148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2133152u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2133156u32);
    emu.lw_no_count(20usize, 2usize, 168u32, 2133160u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2133048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c38));
    } else {
        emu.pc = 2133164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cac));
    }
}
#[inline(always)]
pub fn block_0x00208cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 19usize, 10usize, 2133168u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133172u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133176u32);
    emu.lw_no_count(21usize, 2usize, 172u32, 2133180u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2133072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c50));
    } else {
        emu.pc = 2133184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cc0));
    }
}
#[inline(always)]
pub fn block_0x00208cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 20usize, 10usize, 2133188u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133192u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133196u32);
    emu.lw_no_count(22usize, 2usize, 176u32, 2133200u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a >= b {
        emu.pc = 2133096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c68));
    } else {
        emu.pc = 2133204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208cd4));
    }
}
#[inline(always)]
pub fn block_0x00208cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 10usize, 2133208u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133212u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133216u32);
    emu.lw_no_count(11usize, 2usize, 180u32, 2133220u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2133112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208c78));
    } else {
        emu.pc = 2133224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ce8));
    }
}
#[inline(always)]
pub fn block_0x00208ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 22usize, 10usize, 2133228u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133232u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133236u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2133236u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208cf4));
}
#[inline]
pub fn block_0x00208cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 184u32, 2133240u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2133244u32)?;
    emu.sltru_no_count(10usize, 11usize, 10usize, 2133248u32);
    emu.sbr_no_count(11usize, 24usize, 10usize, 2133252u32);
    emu.sltru_no_count(12usize, 11usize, 24usize, 2133256u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2133260u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2133264u32);
    emu.sltiu_no_count(10usize, 11usize, 1u32, 2133268u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2133272u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2133276u32);
    emu.apc_no_count(1usize, 2133276u32, 61440u32, 2133280u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2133288u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2133292u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2133976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fd8));
    } else {
        emu.pc = 2133296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208d30));
    }
}
#[inline(never)]
pub fn block_0x00208d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 63u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 112u32, 2133300u32)?;
    emu.sb_no_count(10usize, 2usize, 124u32, 2133304u32);
    emu.lw_no_count(10usize, 2usize, 108u32, 2133308u32)?;
    emu.sb_no_count(10usize, 2usize, 125u32, 2133312u32);
    emu.lw_no_count(10usize, 2usize, 104u32, 2133316u32)?;
    emu.sb_no_count(10usize, 2usize, 126u32, 2133320u32);
    emu.lw_no_count(10usize, 2usize, 100u32, 2133324u32)?;
    emu.sb_no_count(10usize, 2usize, 127u32, 2133328u32);
    emu.lw_no_count(10usize, 2usize, 96u32, 2133332u32)?;
    emu.sb_no_count(10usize, 2usize, 128u32, 2133336u32);
    emu.lw_no_count(10usize, 2usize, 92u32, 2133340u32)?;
    emu.sb_no_count(10usize, 2usize, 129u32, 2133344u32);
    emu.lw_no_count(10usize, 2usize, 88u32, 2133348u32)?;
    emu.sb_no_count(10usize, 2usize, 130u32, 2133352u32);
    emu.lw_no_count(10usize, 2usize, 84u32, 2133356u32)?;
    emu.sb_no_count(10usize, 2usize, 131u32, 2133360u32);
    emu.lw_no_count(10usize, 2usize, 80u32, 2133364u32)?;
    emu.sb_no_count(10usize, 2usize, 132u32, 2133368u32);
    emu.lw_no_count(10usize, 2usize, 76u32, 2133372u32)?;
    emu.sb_no_count(10usize, 2usize, 133u32, 2133376u32);
    emu.lw_no_count(10usize, 2usize, 72u32, 2133380u32)?;
    emu.sb_no_count(10usize, 2usize, 134u32, 2133384u32);
    emu.lw_no_count(10usize, 2usize, 68u32, 2133388u32)?;
    emu.sb_no_count(10usize, 2usize, 135u32, 2133392u32);
    emu.lw_no_count(10usize, 2usize, 64u32, 2133396u32)?;
    emu.sb_no_count(10usize, 2usize, 136u32, 2133400u32);
    emu.lw_no_count(10usize, 2usize, 60u32, 2133404u32)?;
    emu.sb_no_count(10usize, 2usize, 137u32, 2133408u32);
    emu.lw_no_count(10usize, 2usize, 56u32, 2133412u32)?;
    emu.sb_no_count(10usize, 2usize, 138u32, 2133416u32);
    emu.lw_no_count(10usize, 2usize, 52u32, 2133420u32)?;
    emu.sb_no_count(10usize, 2usize, 139u32, 2133424u32);
    emu.lw_no_count(10usize, 2usize, 116u32, 2133428u32)?;
    emu.sb_no_count(10usize, 2usize, 140u32, 2133432u32);
    emu.lw_no_count(10usize, 2usize, 48u32, 2133436u32)?;
    emu.sb_no_count(10usize, 2usize, 141u32, 2133440u32);
    emu.lw_no_count(10usize, 2usize, 44u32, 2133444u32)?;
    emu.sb_no_count(10usize, 2usize, 142u32, 2133448u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2133452u32)?;
    emu.sb_no_count(10usize, 2usize, 143u32, 2133456u32);
    emu.sb_no_count(9usize, 2usize, 144u32, 2133460u32);
    emu.sb_no_count(25usize, 2usize, 145u32, 2133464u32);
    emu.sb_no_count(26usize, 2usize, 146u32, 2133468u32);
    emu.sb_no_count(27usize, 2usize, 147u32, 2133472u32);
    emu.sb_no_count(23usize, 2usize, 148u32, 2133476u32);
    emu.lw_no_count(10usize, 2usize, 36u32, 2133480u32)?;
    emu.sb_no_count(10usize, 2usize, 149u32, 2133484u32);
    emu.lw_no_count(10usize, 2usize, 32u32, 2133488u32)?;
    emu.sb_no_count(10usize, 2usize, 150u32, 2133492u32);
    emu.lw_no_count(10usize, 2usize, 28u32, 2133496u32)?;
    emu.sb_no_count(10usize, 2usize, 151u32, 2133500u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2133504u32)?;
    emu.sb_no_count(10usize, 2usize, 152u32, 2133508u32);
    emu.lw_no_count(10usize, 2usize, 20u32, 2133512u32)?;
    emu.sb_no_count(10usize, 2usize, 153u32, 2133516u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2133520u32)?;
    emu.sb_no_count(10usize, 2usize, 154u32, 2133524u32);
    emu.lw_no_count(10usize, 2usize, 12u32, 2133528u32)?;
    emu.sb_no_count(10usize, 2usize, 155u32, 2133532u32);
    emu.adi_no_count(10usize, 2usize, 156u32, 2133536u32);
    emu.adi_no_count(11usize, 2usize, 124u32, 2133540u32);
    emu.apc_no_count(1usize, 2133540u32, 40960u32, 2133544u32);
    emu.add_memory_rw_events(63usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2133552u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 156u32, 2133556u32)?;
    emu.lw_no_count(12usize, 2usize, 160u32, 2133560u32)?;
    emu.lw_no_count(13usize, 2usize, 164u32, 2133564u32)?;
    emu.adi_no_count(10usize, 10usize, 1361u32, 2133568u32);
    emu.sw_no_count(11usize, 2usize, 116u32, 2133572u32)?;
    emu.sltru_no_count(10usize, 11usize, 10usize, 2133576u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2133688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208eb8));
    } else {
        emu.pc = 2133580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e4c));
    }
}
#[inline(always)]
pub fn block_0x00208e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 10usize, 2133584u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133588u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133592u32);
    emu.lw_no_count(23usize, 2usize, 168u32, 2133596u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2133712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ed0));
    } else {
        emu.pc = 2133600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e60));
    }
}
#[inline(always)]
pub fn block_0x00208e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 10usize, 2133604u32);
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2133608u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2133612u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2133616u32);
    emu.lw_no_count(27usize, 2usize, 172u32, 2133620u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2133732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ee4));
    } else {
        emu.pc = 2133624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e78));
    }
}
#[inline(always)]
pub fn block_0x00208e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 23usize, 10usize, 2133628u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2133632u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965933u32, 2133636u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2133640u32);
    emu.lw_no_count(9usize, 2usize, 176u32, 2133644u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a < b {
        emu.pc = 2133752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ef8));
    } else {
        emu.pc = 2133648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e90));
    }
}
#[inline(always)]
pub fn block_0x00208e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 27usize, 10usize, 2133652u32);
    emu.sltiu_no_count(11usize, 10usize, 4294967295u32, 2133656u32);
    emu.lw_no_count(25usize, 2usize, 180u32, 2133660u32)?;
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2133664u32);
    emu.sw_no_count(12usize, 2usize, 112u32, 2133668u32)?;
    emu.sw_no_count(13usize, 2usize, 108u32, 2133672u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2133784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f18));
    } else {
        emu.pc = 2133676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208eac));
    }
}
#[inline(always)]
pub fn block_0x00208eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 9usize, 11usize, 2133680u32);
    emu.sltiu_no_count(11usize, 11usize, 4294967295u32, 2133684u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2133688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133796u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208f24));
}
#[inline(always)]
pub fn block_0x00208eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 12usize, 10usize, 2133692u32);
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2133696u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2133700u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2133704u32);
    emu.lw_no_count(23usize, 2usize, 168u32, 2133708u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2133600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e60));
    } else {
        emu.pc = 2133712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ed0));
    }
}
#[inline(always)]
pub fn block_0x00208ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 13usize, 10usize, 2133716u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133720u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133724u32);
    emu.lw_no_count(27usize, 2usize, 172u32, 2133728u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2133624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e78));
    } else {
        emu.pc = 2133732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ee4));
    }
}
#[inline(always)]
pub fn block_0x00208ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 23usize, 10usize, 2133736u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133740u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2133744u32);
    emu.lw_no_count(9usize, 2usize, 176u32, 2133748u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2133648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208e90));
    } else {
        emu.pc = 2133752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208ef8));
    }
}
#[inline(always)]
pub fn block_0x00208ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 27usize, 10usize, 2133756u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2133760u32);
    emu.slti_no_count(11usize, 10usize, 0u32, 2133764u32);
    emu.lw_no_count(25usize, 2usize, 180u32, 2133768u32)?;
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2133772u32);
    emu.sw_no_count(12usize, 2usize, 112u32, 2133776u32)?;
    emu.sw_no_count(13usize, 2usize, 108u32, 2133780u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2133676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208eac));
    } else {
        emu.pc = 2133784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f18));
    }
}
#[inline(always)]
pub fn block_0x00208f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 9usize, 11usize, 2133788u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2133792u32);
    emu.slti_no_count(11usize, 11usize, 0u32, 2133796u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2133796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208f24));
}
#[inline]
pub fn block_0x00208f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 184u32, 2133800u32)?;
    emu.sltru_no_count(11usize, 25usize, 11usize, 2133804u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2133808u32);
    emu.adr_no_count(12usize, 26usize, 12usize, 2133812u32);
    emu.sltru_no_count(13usize, 12usize, 26usize, 2133816u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2133820u32);
    emu.sbr_no_count(13usize, 12usize, 11usize, 2133824u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2133828u32);
    emu.sltru_no_count(11usize, 13usize, 12usize, 2133832u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2133836u32);
    emu.apc_no_count(1usize, 2133836u32, 61440u32, 2133840u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208f54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2133848u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2133852u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2133980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fdc));
    } else {
        emu.pc = 2133856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f60));
    }
}
#[inline]
pub fn block_0x00208f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(10usize, 18usize, 8usize, 2133860u32);
    emu.orr_no_count(11usize, 19usize, 20usize, 2133864u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2133868u32);
    emu.orr_no_count(11usize, 21usize, 22usize, 2133872u32);
    emu.lw_no_count(12usize, 2usize, 8u32, 2133876u32)?;
    emu.orr_no_count(11usize, 11usize, 12usize, 2133880u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2133884u32);
    emu.orr_no_count(10usize, 10usize, 24usize, 2133888u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2133892u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2133896u32);
    emu.apc_no_count(1usize, 2133896u32, 61440u32, 2133900u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2133908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2133976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fd8));
    } else {
        emu.pc = 2133912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208f98));
    }
}
#[inline]
pub fn block_0x00208f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 116u32, 2133916u32)?;
    emu.lw_no_count(11usize, 2usize, 112u32, 2133920u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2133924u32);
    emu.lw_no_count(11usize, 2usize, 108u32, 2133928u32)?;
    emu.orr_no_count(11usize, 11usize, 23usize, 2133932u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2133936u32);
    emu.orr_no_count(11usize, 27usize, 9usize, 2133940u32);
    emu.orr_no_count(11usize, 11usize, 25usize, 2133944u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2133948u32);
    emu.orr_no_count(10usize, 10usize, 26usize, 2133952u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2133956u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2133960u32);
    emu.apc_no_count(1usize, 2133960u32, 61440u32, 2133964u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2133968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2133972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2134052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209024));
    } else {
        emu.pc = 2133976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208fd8));
    }
}
#[inline(always)]
pub fn block_0x00208fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2133980u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2133980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208fdc));
}
#[inline(always)]
pub fn block_0x00208fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 120u32, 2133984u32)?;
    emu.sw_no_count(10usize, 11usize, 0u32, 2133988u32)?;
    emu.sw_no_count(0usize, 11usize, 4u32, 2133992u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2133992u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208fe8));
}
#[inline]
pub fn block_0x00208fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 236u32, 2133996u32)?;
    emu.lw_no_count(8usize, 2usize, 232u32, 2134000u32)?;
    emu.lw_no_count(9usize, 2usize, 228u32, 2134004u32)?;
    emu.lw_no_count(18usize, 2usize, 224u32, 2134008u32)?;
    emu.lw_no_count(19usize, 2usize, 220u32, 2134012u32)?;
    emu.lw_no_count(20usize, 2usize, 216u32, 2134016u32)?;
    emu.lw_no_count(21usize, 2usize, 212u32, 2134020u32)?;
    emu.lw_no_count(22usize, 2usize, 208u32, 2134024u32)?;
    emu.lw_no_count(23usize, 2usize, 204u32, 2134028u32)?;
    emu.lw_no_count(24usize, 2usize, 200u32, 2134032u32)?;
    emu.lw_no_count(25usize, 2usize, 196u32, 2134036u32)?;
    emu.lw_no_count(26usize, 2usize, 192u32, 2134040u32)?;
    emu.lw_no_count(27usize, 2usize, 188u32, 2134044u32)?;
    emu.adi_no_count(2usize, 2usize, 240u32, 2134048u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134052u32;
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
pub fn block_0x00209024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 120u32, 2134056u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2134060u32)?;
    emu.sw_no_count(8usize, 10usize, 4u32, 2134064u32)?;
    emu.sw_no_count(18usize, 10usize, 8u32, 2134068u32)?;
    emu.sw_no_count(19usize, 10usize, 12u32, 2134072u32)?;
    emu.sw_no_count(20usize, 10usize, 16u32, 2134076u32)?;
    emu.sw_no_count(21usize, 10usize, 20u32, 2134080u32)?;
    emu.sw_no_count(22usize, 10usize, 24u32, 2134084u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2134088u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2134092u32)?;
    emu.sw_no_count(24usize, 10usize, 32u32, 2134096u32)?;
    emu.lw_no_count(11usize, 2usize, 116u32, 2134100u32)?;
    emu.sw_no_count(11usize, 10usize, 36u32, 2134104u32)?;
    emu.lw_no_count(11usize, 2usize, 112u32, 2134108u32)?;
    emu.sw_no_count(11usize, 10usize, 40u32, 2134112u32)?;
    emu.lw_no_count(11usize, 2usize, 108u32, 2134116u32)?;
    emu.sw_no_count(11usize, 10usize, 44u32, 2134120u32)?;
    emu.sw_no_count(23usize, 10usize, 48u32, 2134124u32)?;
    emu.sw_no_count(27usize, 10usize, 52u32, 2134128u32)?;
    emu.sw_no_count(9usize, 10usize, 56u32, 2134132u32)?;
    emu.sw_no_count(25usize, 10usize, 60u32, 2134136u32)?;
    emu.sw_no_count(26usize, 10usize, 64u32, 2134140u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2134144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2133992u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208fe8));
}
