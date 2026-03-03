pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2116660u32;
pub const PC_MAX: u32 = 2124256u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00204c34,
        block_0x00204c74,
        block_0x00204cd8,
        block_0x00204ce4,
        block_0x00204d50,
        block_0x00204d5c,
        block_0x00204d9c,
        block_0x00204dac,
        block_0x00204dec,
        block_0x00204df4,
        block_0x00204dfc,
        block_0x00204e14,
        block_0x00204e1c,
        block_0x00204f80,
        block_0x00204f8c,
        block_0x00204fa4,
        block_0x002050cc,
        block_0x002051d8,
        block_0x002051e4,
        block_0x00205200,
        block_0x00205218,
        block_0x00205230,
        block_0x00205248,
        block_0x00205280,
        block_0x00205298,
        block_0x002052ac,
        block_0x00205378,
        block_0x0020540c,
        block_0x002054a0,
        block_0x00205534,
        block_0x002055c8,
        block_0x002055dc,
        block_0x00205670,
        block_0x00205684,
        block_0x00205718,
        block_0x002057ac,
        block_0x002057c0,
        block_0x00205854,
        block_0x00205868,
        block_0x002058fc,
        block_0x00205990,
        block_0x002059a4,
        block_0x00205a38,
        block_0x00205a4c,
        block_0x00205b00,
        block_0x00205b54,
        block_0x00205b64,
        block_0x00205bb8,
        block_0x00205c4c,
        block_0x00205c60,
        block_0x00205c70,
        block_0x00205cc4,
        block_0x00205cf8,
        block_0x00205d8c,
        block_0x00205da0,
        block_0x00205db0,
        block_0x00205e04,
        block_0x00205e14,
        block_0x00205e68,
        block_0x00205e7c,
        block_0x00205f10,
        block_0x00205fa4,
        block_0x00205fb8,
        block_0x0020600c,
        block_0x00206060,
        block_0x00206074,
        block_0x00206088,
        block_0x0020609c,
        block_0x002060b0,
        block_0x002060e0,
        block_0x00206124,
        block_0x00206134,
        block_0x00206144,
        block_0x002061d8,
        block_0x002061e8,
        block_0x0020627c,
        block_0x0020628c,
        block_0x00206340,
        block_0x00206394,
        block_0x002063a4,
        block_0x002063f8,
        block_0x0020648c,
        block_0x002064e0,
        block_0x00206534,
        block_0x00206548,
        block_0x00206558,
        block_0x002065ac,
        block_0x002065e0,
        block_0x00206674,
        block_0x00206688,
        block_0x00206698,
        block_0x002066ec,
        block_0x002066fc,
        block_0x00206750,
        block_0x00206764,
        block_0x002067b8,
        block_0x002067cc,
        block_0x00206860,
        block_0x00206870,
        block_0x002068c4,
        block_0x002068d8,
        block_0x002068ec,
        block_0x002068fc,
        block_0x0020690c,
        block_0x00206980,
        block_0x002069ac,
        block_0x002069d4,
        block_0x002069e0,
    ];
    #[repr(C)]
    struct Run {
        start_word: u32,
        len: u16,
        fn_offset: u16,
    }
    const RUNS: [Run; 108usize] = [
        Run {
            start_word: 0u32,
            len: 1i32 as u16,
            fn_offset: 0usize as u16,
        },
        Run {
            start_word: 16u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 41u32,
            len: 1i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 44u32,
            len: 1i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 71u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 74u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 90u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 94u32,
            len: 1i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 110u32,
            len: 1i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 112u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 114u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 120u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 122u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 211u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 214u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 220u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 294u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 361u32,
            len: 1i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 364u32,
            len: 1i32 as u16,
            fn_offset: 18usize as u16,
        },
        Run {
            start_word: 371u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 377u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 383u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 389u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 403u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 409u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 414u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 465u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 502u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 539u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 576u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 613u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 618u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 655u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 660u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 697u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 734u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 739u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 776u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 781u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 818u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 855u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 860u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 897u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 902u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 947u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 968u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 972u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 993u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 1030u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 1035u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 1039u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 1060u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 1073u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 1110u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 1115u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 1119u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 1140u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 1144u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 1165u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 1170u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 1207u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 1244u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 1249u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 1270u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 1291u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 1296u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 1301u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 1306u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 1311u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 1323u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 1340u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 1344u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 1348u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 1385u32,
            len: 1i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 1389u32,
            len: 1i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 1426u32,
            len: 1i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 1430u32,
            len: 1i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 1475u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 1496u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 1500u32,
            len: 1i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 1521u32,
            len: 1i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 1558u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 1579u32,
            len: 1i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 1600u32,
            len: 1i32 as u16,
            fn_offset: 83usize as u16,
        },
        Run {
            start_word: 1605u32,
            len: 1i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 1609u32,
            len: 1i32 as u16,
            fn_offset: 85usize as u16,
        },
        Run {
            start_word: 1630u32,
            len: 1i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 1643u32,
            len: 1i32 as u16,
            fn_offset: 87usize as u16,
        },
        Run {
            start_word: 1680u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 1685u32,
            len: 1i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 1689u32,
            len: 1i32 as u16,
            fn_offset: 90usize as u16,
        },
        Run {
            start_word: 1710u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 1714u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 1735u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 1740u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 1761u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 1766u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 1803u32,
            len: 1i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 1807u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 1828u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 1833u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 1838u32,
            len: 1i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 1842u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 1846u32,
            len: 1i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 1875u32,
            len: 1i32 as u16,
            fn_offset: 104usize as u16,
        },
        Run {
            start_word: 1886u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 1896u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
        Run {
            start_word: 1899u32,
            len: 1i32 as u16,
            fn_offset: 107usize as u16,
        },
    ];
    if pc < 2116660u32 || pc > 2124256u32 {
        return None;
    }
    let word_offset = ((pc - 2116660u32) >> 2) as u32;
    let mut lo = 0usize;
    let mut hi = RUNS.len();
    while lo < hi {
        let mid = (lo + hi) >> 1;
        let run = &RUNS[mid];
        if word_offset < run.start_word {
            hi = mid;
        } else if word_offset >= run.start_word + run.len as u32 {
            lo = mid + 1;
        } else {
            let fn_idx = (run.fn_offset as usize)
                + (word_offset - run.start_word) as usize;
            return Some(FN[fn_idx]);
        }
    }
    None
}
#[inline]
pub fn block_0x00204c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 144u32, 2116664u32);
    emu.lw_no_count(1usize, 2usize, 2028u32, 2116668u32)?;
    emu.lw_no_count(8usize, 2usize, 2024u32, 2116672u32)?;
    emu.lw_no_count(9usize, 2usize, 2020u32, 2116676u32)?;
    emu.lw_no_count(18usize, 2usize, 2016u32, 2116680u32)?;
    emu.lw_no_count(19usize, 2usize, 2012u32, 2116684u32)?;
    emu.lw_no_count(20usize, 2usize, 2008u32, 2116688u32)?;
    emu.lw_no_count(21usize, 2usize, 2004u32, 2116692u32)?;
    emu.lw_no_count(22usize, 2usize, 2000u32, 2116696u32)?;
    emu.lw_no_count(23usize, 2usize, 1996u32, 2116700u32)?;
    emu.lw_no_count(24usize, 2usize, 1992u32, 2116704u32)?;
    emu.lw_no_count(25usize, 2usize, 1988u32, 2116708u32)?;
    emu.lw_no_count(26usize, 2usize, 1984u32, 2116712u32)?;
    emu.lw_no_count(27usize, 2usize, 1980u32, 2116716u32)?;
    emu.adi_no_count(2usize, 2usize, 2032u32, 2116720u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116724u32;
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
#[inline(never)]
pub fn block_0x00204c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2116728u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2116732u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2116736u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2116740u32)?;
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2116744u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 728u32, 2116748u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2116752u32);
    let a = 0u32.wrapping_add(2129920u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2116756u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1004u32, 2116760u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116764u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 460u32, 2116768u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2116772u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2116776u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2116780u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2116784u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2116788u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2116792u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2116796u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2116800u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2116804u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2116808u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2116812u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2116816u32);
    emu.apc_no_count(1usize, 2116816u32, 12288u32, 2116820u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2116828u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2116832u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116836u32;
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
#[inline(never)]
pub fn block_0x00204ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2116840u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2116844u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2116848u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2116852u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2116856u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2116860u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2116864u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2116868u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2116872u32);
    let a = 0u32.wrapping_add(2129920u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2116876u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1004u32, 2116880u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116884u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 492u32, 2116888u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2116892u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2116896u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2116900u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2116904u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2116908u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2116912u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2116916u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2116920u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2116924u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2116928u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2116932u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2116936u32);
    emu.apc_no_count(1usize, 2116936u32, 12288u32, 2116940u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966012u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2116948u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2116952u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116956u32;
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
pub fn block_0x00204d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966928u32, 2116960u32);
    emu.sw_no_count(1usize, 2usize, 364u32, 2116964u32)?;
    emu.sw_no_count(8usize, 2usize, 360u32, 2116968u32)?;
    emu.sw_no_count(9usize, 2usize, 356u32, 2116972u32)?;
    emu.sw_no_count(18usize, 2usize, 352u32, 2116976u32)?;
    emu.sw_no_count(19usize, 2usize, 348u32, 2116980u32)?;
    emu.sw_no_count(20usize, 2usize, 344u32, 2116984u32)?;
    emu.sw_no_count(21usize, 2usize, 340u32, 2116988u32)?;
    emu.sw_no_count(22usize, 2usize, 336u32, 2116992u32)?;
    emu.sw_no_count(23usize, 2usize, 332u32, 2116996u32)?;
    emu.sw_no_count(24usize, 2usize, 328u32, 2117000u32)?;
    emu.sw_no_count(25usize, 2usize, 324u32, 2117004u32)?;
    emu.sw_no_count(26usize, 2usize, 320u32, 2117008u32)?;
    emu.adi_no_count(18usize, 11usize, 0u32, 2117012u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2117016u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2117036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dac));
    } else {
        emu.pc = 2117020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d9c));
    }
}
#[inline(always)]
pub fn block_0x00204d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2117024u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2117028u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2117032u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2117036u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2118216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205248));
}
#[inline]
pub fn block_0x00204dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 14usize, 0u32, 2117040u32);
    emu.sw_no_count(0usize, 2usize, 240u32, 2117044u32)?;
    emu.sw_no_count(0usize, 2usize, 244u32, 2117048u32)?;
    emu.sw_no_count(0usize, 2usize, 248u32, 2117052u32)?;
    emu.sw_no_count(0usize, 2usize, 252u32, 2117056u32)?;
    emu.sltiu_no_count(11usize, 13usize, 32u32, 2117060u32);
    emu.sltiu_no_count(14usize, 13usize, 33u32, 2117064u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2117068u32);
    emu.sbr_no_count(14usize, 14usize, 11usize, 2117072u32);
    emu.ani_no_count(11usize, 14usize, 255u32, 2117076u32);
    emu.sw_no_count(0usize, 2usize, 224u32, 2117080u32)?;
    emu.sw_no_count(0usize, 2usize, 228u32, 2117084u32)?;
    emu.sw_no_count(0usize, 2usize, 232u32, 2117088u32)?;
    emu.sw_no_count(0usize, 2usize, 236u32, 2117092u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2117096u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2117140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e14));
    } else {
        emu.pc = 2117100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dec));
    }
}
#[inline(always)]
pub fn block_0x00204dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 1u32, 2117104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f80));
    } else {
        emu.pc = 2117108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204df4));
    }
}
#[inline(always)]
pub fn block_0x00204df4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 31u32, 2117112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2117148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e1c));
    } else {
        emu.pc = 2117116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dfc));
    }
}
#[inline(always)]
pub fn block_0x00204dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2117120u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 752u32, 2117124u32);
    emu.adi_no_count(10usize, 0usize, 32u32, 2117128u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2117132u32);
    emu.apc_no_count(1usize, 2117132u32, 131072u32, 2117136u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 32u32, 2117144u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2118272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205280));
    } else {
        emu.pc = 2117148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e1c));
    }
}
#[inline(never)]
pub fn block_0x00204e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 89u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 12usize, 28u32, 2117152u32);
    emu.lbu_no_count(11usize, 12usize, 29u32, 2117156u32);
    emu.lbu_no_count(13usize, 12usize, 30u32, 2117160u32);
    emu.lbu_no_count(14usize, 12usize, 31u32, 2117164u32);
    emu.lbu_no_count(15usize, 12usize, 24u32, 2117168u32);
    emu.lbu_no_count(16usize, 12usize, 25u32, 2117172u32);
    emu.lbu_no_count(17usize, 12usize, 26u32, 2117176u32);
    emu.lbu_no_count(5usize, 12usize, 27u32, 2117180u32);
    emu.lbu_no_count(6usize, 12usize, 20u32, 2117184u32);
    emu.lbu_no_count(7usize, 12usize, 21u32, 2117188u32);
    emu.lbu_no_count(28usize, 12usize, 22u32, 2117192u32);
    emu.lbu_no_count(29usize, 12usize, 23u32, 2117196u32);
    emu.lbu_no_count(30usize, 12usize, 16u32, 2117200u32);
    emu.lbu_no_count(31usize, 12usize, 17u32, 2117204u32);
    emu.lbu_no_count(19usize, 12usize, 18u32, 2117208u32);
    emu.lbu_no_count(20usize, 12usize, 19u32, 2117212u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2117216u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2117220u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2117224u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2117228u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2117232u32);
    emu.orr_no_count(11usize, 14usize, 13usize, 2117236u32);
    emu.orr_no_count(13usize, 16usize, 15usize, 2117240u32);
    emu.lbu_no_count(21usize, 12usize, 12u32, 2117244u32);
    emu.lbu_no_count(22usize, 12usize, 13u32, 2117248u32);
    emu.lbu_no_count(23usize, 12usize, 14u32, 2117252u32);
    emu.lbu_no_count(24usize, 12usize, 15u32, 2117256u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2117260u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2117264u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2117268u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2117272u32);
    emu.sli_no_count(29usize, 29usize, 24u32, 2117276u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2117280u32);
    emu.orr_no_count(14usize, 5usize, 17usize, 2117284u32);
    emu.orr_no_count(15usize, 7usize, 6usize, 2117288u32);
    emu.orr_no_count(16usize, 29usize, 28usize, 2117292u32);
    emu.orr_no_count(17usize, 31usize, 30usize, 2117296u32);
    emu.lbu_no_count(5usize, 12usize, 8u32, 2117300u32);
    emu.lbu_no_count(6usize, 12usize, 9u32, 2117304u32);
    emu.lbu_no_count(7usize, 12usize, 10u32, 2117308u32);
    emu.lbu_no_count(28usize, 12usize, 11u32, 2117312u32);
    emu.sli_no_count(19usize, 19usize, 16u32, 2117316u32);
    emu.sli_no_count(20usize, 20usize, 24u32, 2117320u32);
    emu.sli_no_count(22usize, 22usize, 8u32, 2117324u32);
    emu.sli_no_count(23usize, 23usize, 16u32, 2117328u32);
    emu.sli_no_count(24usize, 24usize, 24u32, 2117332u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2117336u32);
    emu.orr_no_count(29usize, 20usize, 19usize, 2117340u32);
    emu.orr_no_count(30usize, 22usize, 21usize, 2117344u32);
    emu.orr_no_count(31usize, 24usize, 23usize, 2117348u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2117352u32);
    emu.lbu_no_count(6usize, 12usize, 4u32, 2117356u32);
    emu.lbu_no_count(19usize, 12usize, 5u32, 2117360u32);
    emu.lbu_no_count(20usize, 12usize, 6u32, 2117364u32);
    emu.lbu_no_count(21usize, 12usize, 7u32, 2117368u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2117372u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2117376u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2117380u32);
    emu.sli_no_count(20usize, 20usize, 16u32, 2117384u32);
    emu.sli_no_count(21usize, 21usize, 24u32, 2117388u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2117392u32);
    emu.orr_no_count(6usize, 19usize, 6usize, 2117396u32);
    emu.lbu_no_count(28usize, 12usize, 0u32, 2117400u32);
    emu.lbu_no_count(19usize, 12usize, 1u32, 2117404u32);
    emu.orr_no_count(20usize, 21usize, 20usize, 2117408u32);
    emu.lbu_no_count(21usize, 12usize, 2u32, 2117412u32);
    emu.lbu_no_count(12usize, 12usize, 3u32, 2117416u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2117420u32);
    emu.orr_no_count(28usize, 19usize, 28usize, 2117424u32);
    emu.sli_no_count(21usize, 21usize, 16u32, 2117428u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2117432u32);
    emu.orr_no_count(12usize, 12usize, 21usize, 2117436u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2117440u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2117444u32);
    emu.orr_no_count(11usize, 16usize, 15usize, 2117448u32);
    emu.orr_no_count(14usize, 29usize, 17usize, 2117452u32);
    emu.orr_no_count(15usize, 31usize, 30usize, 2117456u32);
    emu.orr_no_count(16usize, 7usize, 5usize, 2117460u32);
    emu.orr_no_count(17usize, 20usize, 6usize, 2117464u32);
    emu.orr_no_count(12usize, 12usize, 28usize, 2117468u32);
    emu.sw_no_count(14usize, 2usize, 240u32, 2117472u32)?;
    emu.sw_no_count(11usize, 2usize, 244u32, 2117476u32)?;
    emu.sw_no_count(13usize, 2usize, 248u32, 2117480u32)?;
    emu.sw_no_count(10usize, 2usize, 252u32, 2117484u32)?;
    emu.sw_no_count(12usize, 2usize, 224u32, 2117488u32)?;
    emu.sw_no_count(17usize, 2usize, 228u32, 2117492u32)?;
    emu.sw_no_count(16usize, 2usize, 232u32, 2117496u32)?;
    emu.sw_no_count(15usize, 2usize, 236u32, 2117500u32)?;
    emu.add_memory_rw_events(89usize);
    let return_addr = 2117504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2117540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204fa4));
}
#[inline(always)]
pub fn block_0x00204f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 32u32, 2117508u32);
    emu.sbr_no_count(10usize, 11usize, 13usize, 2117512u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2118296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205298));
    } else {
        emu.pc = 2117516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f8c));
    }
}
#[inline(always)]
pub fn block_0x00204f8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 224u32, 2117520u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2117524u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2117528u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2117532u32);
    emu.apc_no_count(1usize, 2117532u32, 20480u32, 2117536u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117540u32;
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
#[inline(never)]
pub fn block_0x00204fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 74u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(10usize, 2usize, 224u32, 2117544u32)?;
    emu.lbu_no_count(11usize, 2usize, 226u32, 2117548u32);
    emu.lbu_no_count(21usize, 2usize, 227u32, 2117552u32);
    emu.lbu_no_count(22usize, 2usize, 228u32, 2117556u32);
    emu.lbu_no_count(19usize, 2usize, 233u32, 2117560u32);
    emu.lbu_no_count(20usize, 2usize, 234u32, 2117564u32);
    emu.lbu_no_count(12usize, 2usize, 235u32, 2117568u32);
    emu.lbu_no_count(13usize, 2usize, 236u32, 2117572u32);
    emu.lbu_no_count(14usize, 2usize, 237u32, 2117576u32);
    emu.lbu_no_count(15usize, 2usize, 238u32, 2117580u32);
    emu.lbu_no_count(16usize, 2usize, 239u32, 2117584u32);
    emu.lbu_no_count(17usize, 2usize, 240u32, 2117588u32);
    emu.lbu_no_count(5usize, 2usize, 241u32, 2117592u32);
    emu.lbu_no_count(6usize, 2usize, 242u32, 2117596u32);
    emu.lbu_no_count(7usize, 2usize, 243u32, 2117600u32);
    emu.lbu_no_count(28usize, 2usize, 244u32, 2117604u32);
    emu.sh_no_count(10usize, 2usize, 0u32, 2117608u32)?;
    emu.sb_no_count(11usize, 2usize, 2u32, 2117612u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2117616u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2117620u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2117624u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2117628u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2117632u32);
    emu.lbu_no_count(10usize, 2usize, 245u32, 2117636u32);
    emu.lbu_no_count(11usize, 2usize, 246u32, 2117640u32);
    emu.lbu_no_count(13usize, 2usize, 247u32, 2117644u32);
    emu.lbu_no_count(15usize, 2usize, 248u32, 2117648u32);
    emu.sli_no_count(17usize, 17usize, 8u32, 2117652u32);
    emu.sli_no_count(5usize, 5usize, 16u32, 2117656u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2117660u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2117664u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2117668u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2117672u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2117676u32);
    emu.orr_no_count(17usize, 6usize, 5usize, 2117680u32);
    emu.orr_no_count(5usize, 28usize, 7usize, 2117684u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2117688u32);
    emu.lbu_no_count(11usize, 2usize, 249u32, 2117692u32);
    emu.lbu_no_count(6usize, 2usize, 250u32, 2117696u32);
    emu.lbu_no_count(7usize, 2usize, 251u32, 2117700u32);
    emu.lbu_no_count(28usize, 2usize, 252u32, 2117704u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2117708u32);
    emu.sli_no_count(11usize, 11usize, 16u32, 2117712u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2117716u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2117720u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2117724u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2117728u32);
    emu.lbu_no_count(15usize, 2usize, 253u32, 2117732u32);
    emu.lbu_no_count(6usize, 2usize, 254u32, 2117736u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2117740u32);
    emu.lbu_no_count(28usize, 2usize, 255u32, 2117744u32);
    emu.sli_no_count(15usize, 15usize, 16u32, 2117748u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2117752u32);
    emu.orr_no_count(15usize, 6usize, 15usize, 2117756u32);
    emu.orr_no_count(12usize, 14usize, 12usize, 2117760u32);
    emu.orr_no_count(14usize, 17usize, 16usize, 2117764u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2117768u32);
    emu.orr_no_count(11usize, 11usize, 13usize, 2117772u32);
    emu.orr_no_count(13usize, 15usize, 7usize, 2117776u32);
    emu.sw_no_count(13usize, 2usize, 144u32, 2117780u32)?;
    emu.sb_no_count(28usize, 2usize, 148u32, 2117784u32);
    emu.lbu_no_count(23usize, 2usize, 229u32, 2117788u32);
    emu.lbu_no_count(24usize, 2usize, 230u32, 2117792u32);
    emu.lbu_no_count(25usize, 2usize, 231u32, 2117796u32);
    emu.lbu_no_count(26usize, 2usize, 232u32, 2117800u32);
    emu.sw_no_count(12usize, 2usize, 128u32, 2117804u32)?;
    emu.sw_no_count(14usize, 2usize, 132u32, 2117808u32)?;
    emu.sw_no_count(10usize, 2usize, 136u32, 2117812u32)?;
    emu.sw_no_count(11usize, 2usize, 140u32, 2117816u32)?;
    emu.adi_no_count(10usize, 2usize, 11u32, 2117820u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2117824u32);
    emu.adi_no_count(12usize, 0usize, 21u32, 2117828u32);
    emu.apc_no_count(1usize, 2117828u32, 20480u32, 2117832u32);
    emu.add_memory_rw_events(74usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1048u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002050cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 67u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(21usize, 2usize, 3u32, 2117840u32);
    emu.sb_no_count(22usize, 2usize, 4u32, 2117844u32);
    emu.sb_no_count(23usize, 2usize, 5u32, 2117848u32);
    emu.sb_no_count(24usize, 2usize, 6u32, 2117852u32);
    emu.sb_no_count(25usize, 2usize, 7u32, 2117856u32);
    emu.sb_no_count(26usize, 2usize, 8u32, 2117860u32);
    emu.sb_no_count(19usize, 2usize, 9u32, 2117864u32);
    emu.sb_no_count(20usize, 2usize, 10u32, 2117868u32);
    emu.lbu_no_count(20usize, 18usize, 64u32, 2117872u32);
    emu.adi_no_count(19usize, 2usize, 160u32, 2117876u32);
    emu.lw_no_count(10usize, 18usize, 48u32, 2117880u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2117884u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2117888u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2117892u32)?;
    emu.lw_no_count(14usize, 18usize, 32u32, 2117896u32)?;
    emu.lw_no_count(15usize, 18usize, 36u32, 2117900u32)?;
    emu.lw_no_count(16usize, 18usize, 40u32, 2117904u32)?;
    emu.lw_no_count(17usize, 18usize, 44u32, 2117908u32)?;
    emu.lw_no_count(5usize, 18usize, 0u32, 2117912u32)?;
    emu.lw_no_count(6usize, 18usize, 4u32, 2117916u32)?;
    emu.lw_no_count(7usize, 18usize, 8u32, 2117920u32)?;
    emu.lw_no_count(28usize, 18usize, 12u32, 2117924u32)?;
    emu.lw_no_count(29usize, 18usize, 16u32, 2117928u32)?;
    emu.lw_no_count(30usize, 18usize, 20u32, 2117932u32)?;
    emu.lw_no_count(31usize, 18usize, 24u32, 2117936u32)?;
    emu.lw_no_count(21usize, 18usize, 28u32, 2117940u32)?;
    emu.adi_no_count(18usize, 2usize, 192u32, 2117944u32);
    emu.sw_no_count(10usize, 2usize, 176u32, 2117948u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2117952u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2117956u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2117960u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2117964u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 2usize, 160u32, 2117968u32)?;
    emu.sw_no_count(15usize, 2usize, 164u32, 2117972u32)?;
    emu.sw_no_count(16usize, 2usize, 168u32, 2117976u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2117980u32)?;
    emu.adi_no_count(11usize, 10usize, 92u32, 2117984u32);
    emu.lw_no_count(10usize, 10usize, 92u32, 2117988u32)?;
    emu.sw_no_count(5usize, 2usize, 128u32, 2117992u32)?;
    emu.sw_no_count(6usize, 2usize, 132u32, 2117996u32)?;
    emu.sw_no_count(7usize, 2usize, 136u32, 2118000u32)?;
    emu.sw_no_count(28usize, 2usize, 140u32, 2118004u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2118008u32)?;
    emu.lw_no_count(13usize, 11usize, 8u32, 2118012u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2118016u32)?;
    emu.lw_no_count(15usize, 11usize, 16u32, 2118020u32)?;
    emu.sw_no_count(29usize, 2usize, 144u32, 2118024u32)?;
    emu.sw_no_count(30usize, 2usize, 148u32, 2118028u32)?;
    emu.sw_no_count(31usize, 2usize, 152u32, 2118032u32)?;
    emu.sw_no_count(21usize, 2usize, 156u32, 2118036u32)?;
    emu.lw_no_count(16usize, 11usize, 20u32, 2118040u32)?;
    emu.lw_no_count(17usize, 11usize, 24u32, 2118044u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2118048u32)?;
    emu.sw_no_count(15usize, 2usize, 208u32, 2118052u32)?;
    emu.sw_no_count(16usize, 2usize, 212u32, 2118056u32)?;
    emu.sw_no_count(17usize, 2usize, 216u32, 2118060u32)?;
    emu.sw_no_count(11usize, 2usize, 220u32, 2118064u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2118068u32)?;
    emu.sw_no_count(12usize, 2usize, 196u32, 2118072u32)?;
    emu.sw_no_count(13usize, 2usize, 200u32, 2118076u32)?;
    emu.sw_no_count(14usize, 2usize, 204u32, 2118080u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2118084u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 192u32, 2118088u32);
    emu.adi_no_count(10usize, 2usize, 224u32, 2118092u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2118096u32);
    emu.apc_no_count(1usize, 2118096u32, 20480u32, 2118100u32);
    emu.add_memory_rw_events(67usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2118108u32);
    emu.apc_no_count(1usize, 2118108u32, 24576u32, 2118112u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 10usize, 255u32, 2118120u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2118124u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2118128u32);
    emu.adi_no_count(12usize, 2usize, 224u32, 2118132u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2118136u32);
    emu.apc_no_count(1usize, 2118136u32, 53248u32, 2118140u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118144u32;
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
pub fn block_0x00205200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 256u32, 2118148u32);
    emu.adi_no_count(10usize, 2usize, 64u32, 2118152u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2118156u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2118160u32);
    emu.apc_no_count(1usize, 2118160u32, 53248u32, 2118164u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 288u32, 2118172u32);
    emu.adi_no_count(10usize, 2usize, 96u32, 2118176u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2118180u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2118184u32);
    emu.apc_no_count(1usize, 2118184u32, 53248u32, 2118188u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 32u32, 2118196u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2118200u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2118204u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2118208u32);
    emu.apc_no_count(1usize, 2118208u32, 8192u32, 2118212u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118216u32;
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
#[inline]
pub fn block_0x00205248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 364u32, 2118220u32)?;
    emu.lw_no_count(8usize, 2usize, 360u32, 2118224u32)?;
    emu.lw_no_count(9usize, 2usize, 356u32, 2118228u32)?;
    emu.lw_no_count(18usize, 2usize, 352u32, 2118232u32)?;
    emu.lw_no_count(19usize, 2usize, 348u32, 2118236u32)?;
    emu.lw_no_count(20usize, 2usize, 344u32, 2118240u32)?;
    emu.lw_no_count(21usize, 2usize, 340u32, 2118244u32)?;
    emu.lw_no_count(22usize, 2usize, 336u32, 2118248u32)?;
    emu.lw_no_count(23usize, 2usize, 332u32, 2118252u32)?;
    emu.lw_no_count(24usize, 2usize, 328u32, 2118256u32)?;
    emu.lw_no_count(25usize, 2usize, 324u32, 2118260u32)?;
    emu.lw_no_count(26usize, 2usize, 320u32, 2118264u32)?;
    emu.adi_no_count(2usize, 2usize, 368u32, 2118268u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118272u32;
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
pub fn block_0x00205280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2118276u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 736u32, 2118280u32);
    emu.adi_no_count(10usize, 0usize, 32u32, 2118284u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2118288u32);
    emu.apc_no_count(1usize, 2118288u32, 131072u32, 2118292u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2118300u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 720u32, 2118304u32);
    emu.adi_no_count(11usize, 0usize, 32u32, 2118308u32);
    emu.apc_no_count(1usize, 2118308u32, 131072u32, 2118312u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002052ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 51u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966672u32, 2118320u32);
    emu.sw_no_count(1usize, 2usize, 620u32, 2118324u32)?;
    emu.sw_no_count(8usize, 2usize, 616u32, 2118328u32)?;
    emu.sw_no_count(9usize, 2usize, 612u32, 2118332u32)?;
    emu.sw_no_count(18usize, 2usize, 608u32, 2118336u32)?;
    emu.sw_no_count(19usize, 2usize, 604u32, 2118340u32)?;
    emu.sw_no_count(20usize, 2usize, 600u32, 2118344u32)?;
    emu.sw_no_count(21usize, 2usize, 596u32, 2118348u32)?;
    emu.sw_no_count(22usize, 2usize, 592u32, 2118352u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2118356u32)?;
    emu.sw_no_count(24usize, 2usize, 584u32, 2118360u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2118364u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2118368u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2118372u32);
    emu.lw_no_count(10usize, 11usize, 16u32, 2118376u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2118380u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2118384u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2118388u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2118392u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2118396u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2118400u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2118404u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2118408u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2118412u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2118416u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2118420u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2118424u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2118428u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2118432u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2118436u32)?;
    emu.lw_no_count(10usize, 9usize, 16u32, 2118440u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2118444u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2118448u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2118452u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2118456u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2118460u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2118464u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2118468u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2118472u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2118476u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2118480u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2118484u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2118488u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2118492u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2118496u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2118500u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2118504u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2118508u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2118512u32);
    emu.apc_no_count(1usize, 2118512u32, 24576u32, 2118516u32);
    emu.add_memory_rw_events(51usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2118524u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2118528u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2118532u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2118536u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2118540u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2118544u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2118548u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2118552u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2118556u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2118560u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2118564u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2118568u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2118572u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2118576u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2118580u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2118584u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2118588u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2118592u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2118596u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2118600u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2118604u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2118608u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2118612u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2118616u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2118620u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2118624u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2118628u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2118632u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2118636u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2118640u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2118644u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2118648u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2118652u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2118656u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2118660u32);
    emu.apc_no_count(1usize, 2118660u32, 24576u32, 2118664u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118668u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020540c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 80u32, 2118672u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2118676u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2118680u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2118684u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2118688u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2118692u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2118696u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2118700u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2118704u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2118708u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2118712u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2118716u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2118720u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2118724u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2118728u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2118732u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2118736u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2118740u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2118744u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2118748u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2118752u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2118756u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2118760u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2118764u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2118768u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2118772u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2118776u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2118780u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2118784u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2118788u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2118792u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2118796u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2118800u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2118804u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2118808u32);
    emu.apc_no_count(1usize, 2118808u32, 24576u32, 2118812u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118816u32;
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
#[inline(never)]
pub fn block_0x002054a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2118820u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2118824u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2118828u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2118832u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2118836u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2118840u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2118844u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2118848u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2118852u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2118856u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2118860u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2118864u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2118868u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2118872u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2118876u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2118880u32)?;
    emu.lw_no_count(10usize, 18usize, 48u32, 2118884u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2118888u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2118892u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2118896u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2118900u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2118904u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2118908u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2118912u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2118916u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2118920u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2118924u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2118928u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2118932u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2118936u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2118940u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2118944u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2118948u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2118952u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2118956u32);
    emu.apc_no_count(1usize, 2118956u32, 24576u32, 2118960u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118964u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2118968u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2118972u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2118976u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2118980u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2118984u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2118988u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2118992u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2118996u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2119000u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2119004u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2119008u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2119012u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2119016u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2119020u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2119024u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2119028u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2119032u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2119036u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2119040u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2119044u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2119048u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2119052u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2119056u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2119060u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2119064u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2119068u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2119072u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2119076u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2119080u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2119084u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2119088u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2119092u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2119096u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2119100u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2119104u32);
    emu.apc_no_count(1usize, 2119104u32, 24576u32, 2119108u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002055c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2119116u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2119120u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2119124u32);
    emu.apc_no_count(1usize, 2119124u32, 24576u32, 2119128u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002055dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2119136u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2119140u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2119144u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2119148u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2119152u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2119156u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2119160u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2119164u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2119168u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2119172u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2119176u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2119180u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2119184u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2119188u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2119192u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2119196u32)?;
    emu.lw_no_count(10usize, 2usize, 56u32, 2119200u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2119204u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2119208u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2119212u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2119216u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2119220u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2119224u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2119228u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2119232u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2119236u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2119240u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2119244u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2119248u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2119252u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2119256u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2119260u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2119264u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2119268u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2119272u32);
    emu.apc_no_count(1usize, 2119272u32, 24576u32, 2119276u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 104u32, 2119284u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2119288u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2119292u32);
    emu.apc_no_count(1usize, 2119292u32, 24576u32, 2119296u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2119304u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2119308u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2119312u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2119316u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2119320u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2119324u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2119328u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2119332u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2119336u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2119340u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2119344u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2119348u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2119352u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2119356u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2119360u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2119364u32)?;
    emu.lw_no_count(10usize, 18usize, 80u32, 2119368u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2119372u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2119376u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2119380u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2119384u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2119388u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2119392u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2119396u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2119400u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2119404u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2119408u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2119412u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2119416u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2119420u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2119424u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2119428u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2119432u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2119436u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2119440u32);
    emu.apc_no_count(1usize, 2119440u32, 24576u32, 2119444u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119448u32;
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
#[inline(never)]
pub fn block_0x00205718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 48u32, 2119452u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2119456u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2119460u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2119464u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2119468u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2119472u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2119476u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2119480u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2119484u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2119488u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2119492u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2119496u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2119500u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2119504u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2119508u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2119512u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2119516u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2119520u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2119524u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2119528u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2119532u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2119536u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2119540u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2119544u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2119548u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2119552u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2119556u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2119560u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2119564u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2119568u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2119572u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2119576u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2119580u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2119584u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2119588u32);
    emu.apc_no_count(1usize, 2119588u32, 24576u32, 2119592u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002057ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2119600u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2119604u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2119608u32);
    emu.apc_no_count(1usize, 2119608u32, 24576u32, 2119612u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002057c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2119620u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2119624u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2119628u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2119632u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2119636u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2119640u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2119644u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2119648u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2119652u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2119656u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2119660u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2119664u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2119668u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2119672u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2119676u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2119680u32)?;
    emu.lw_no_count(10usize, 2usize, 56u32, 2119684u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2119688u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2119692u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2119696u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2119700u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2119704u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2119708u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2119712u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2119716u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2119720u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2119724u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2119728u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2119732u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2119736u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2119740u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2119744u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2119748u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2119752u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2119756u32);
    emu.apc_no_count(1usize, 2119756u32, 24576u32, 2119760u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 136u32, 2119768u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2119772u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2119776u32);
    emu.apc_no_count(1usize, 2119776u32, 24576u32, 2119780u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119784u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2119788u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2119792u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2119796u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2119800u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2119804u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2119808u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2119812u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2119816u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2119820u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2119824u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2119828u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2119832u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2119836u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2119840u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2119844u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2119848u32)?;
    emu.lw_no_count(10usize, 18usize, 80u32, 2119852u32)?;
    emu.lw_no_count(11usize, 18usize, 84u32, 2119856u32)?;
    emu.lw_no_count(12usize, 18usize, 88u32, 2119860u32)?;
    emu.lw_no_count(13usize, 18usize, 92u32, 2119864u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2119868u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2119872u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2119876u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2119880u32)?;
    emu.lw_no_count(10usize, 18usize, 64u32, 2119884u32)?;
    emu.lw_no_count(11usize, 18usize, 68u32, 2119888u32)?;
    emu.lw_no_count(12usize, 18usize, 72u32, 2119892u32)?;
    emu.lw_no_count(13usize, 18usize, 76u32, 2119896u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2119900u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2119904u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2119908u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2119912u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2119916u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2119920u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2119924u32);
    emu.apc_no_count(1usize, 2119924u32, 20480u32, 2119928u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002058fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2119936u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2119940u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2119944u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2119948u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2119952u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2119956u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2119960u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2119964u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2119968u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2119972u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2119976u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2119980u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2119984u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2119988u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2119992u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2119996u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2120000u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2120004u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2120008u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2120012u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2120016u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2120020u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2120024u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2120028u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2120032u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2120036u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2120040u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2120044u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2120048u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2120052u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2120056u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2120060u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2120064u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2120068u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2120072u32);
    emu.apc_no_count(1usize, 2120072u32, 20480u32, 2120076u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2120084u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2120088u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2120092u32);
    emu.apc_no_count(1usize, 2120092u32, 24576u32, 2120096u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002059a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2120104u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2120108u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2120112u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2120116u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2120120u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2120124u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2120128u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2120132u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2120136u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2120140u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2120144u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2120148u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2120152u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2120156u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2120160u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2120164u32)?;
    emu.lw_no_count(10usize, 2usize, 88u32, 2120168u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2120172u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2120176u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2120180u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2120184u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2120188u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2120192u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2120196u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2120200u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2120204u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2120208u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2120212u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2120216u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2120220u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2120224u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2120228u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2120232u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2120236u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2120240u32);
    emu.apc_no_count(1usize, 2120240u32, 20480u32, 2120244u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205a38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 168u32, 2120252u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2120256u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2120260u32);
    emu.apc_no_count(1usize, 2120260u32, 24576u32, 2120264u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120268u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 45u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2120272u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2120276u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2120280u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2120284u32)?;
    emu.lw_no_count(14usize, 2usize, 72u32, 2120288u32)?;
    emu.lw_no_count(15usize, 2usize, 76u32, 2120292u32)?;
    emu.lw_no_count(16usize, 2usize, 80u32, 2120296u32)?;
    emu.lw_no_count(17usize, 2usize, 84u32, 2120300u32)?;
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2120304u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2120308u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2120312u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2120316u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2120320u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(30usize, a);
    emu.pc = 2120324u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 536u32, 2120328u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2120332u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2120336u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2120340u32)?;
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120344u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 2usize, 520u32, 2120348u32)?;
    emu.sw_no_count(15usize, 2usize, 524u32, 2120352u32)?;
    emu.sw_no_count(16usize, 2usize, 528u32, 2120356u32)?;
    emu.sw_no_count(17usize, 2usize, 532u32, 2120360u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120364u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 5usize, 1565u32, 2120368u32);
    emu.adi_no_count(21usize, 6usize, 4294965300u32, 2120372u32);
    emu.adi_no_count(23usize, 7usize, 171u32, 2120376u32);
    emu.adi_no_count(24usize, 28usize, 4294966998u32, 2120380u32);
    emu.adi_no_count(9usize, 29usize, 1485u32, 2120384u32);
    emu.adi_no_count(18usize, 30usize, 144u32, 2120388u32);
    emu.adi_no_count(20usize, 10usize, 4294967138u32, 2120392u32);
    emu.adi_no_count(22usize, 11usize, 4294966751u32, 2120396u32);
    emu.sw_no_count(24usize, 2usize, 568u32, 2120400u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2120404u32)?;
    emu.sw_no_count(21usize, 2usize, 576u32, 2120408u32)?;
    emu.sw_no_count(19usize, 2usize, 580u32, 2120412u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2120416u32)?;
    emu.sw_no_count(20usize, 2usize, 556u32, 2120420u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2120424u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2120428u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2120432u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2120436u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2120440u32);
    emu.apc_no_count(1usize, 2120440u32, 24576u32, 2120444u32);
    emu.add_memory_rw_events(45usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 184u32, 2120452u32)?;
    emu.lw_no_count(11usize, 2usize, 188u32, 2120456u32)?;
    emu.lw_no_count(12usize, 2usize, 192u32, 2120460u32)?;
    emu.lw_no_count(13usize, 2usize, 196u32, 2120464u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2120468u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2120472u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2120476u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2120480u32)?;
    emu.lw_no_count(10usize, 2usize, 168u32, 2120484u32)?;
    emu.lw_no_count(11usize, 2usize, 172u32, 2120488u32)?;
    emu.lw_no_count(12usize, 2usize, 176u32, 2120492u32)?;
    emu.lw_no_count(13usize, 2usize, 180u32, 2120496u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2120500u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2120504u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2120508u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2120512u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2120516u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2120520u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2120524u32);
    emu.apc_no_count(1usize, 2120524u32, 20480u32, 2120528u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2120536u32);
    emu.adi_no_count(11usize, 2usize, 200u32, 2120540u32);
    emu.apc_no_count(1usize, 2120540u32, 53248u32, 2120544u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 216u32, 2120552u32)?;
    emu.lw_no_count(11usize, 2usize, 220u32, 2120556u32)?;
    emu.lw_no_count(12usize, 2usize, 224u32, 2120560u32)?;
    emu.lw_no_count(13usize, 2usize, 228u32, 2120564u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2120568u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2120572u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2120576u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2120580u32)?;
    emu.lw_no_count(10usize, 2usize, 200u32, 2120584u32)?;
    emu.lw_no_count(11usize, 2usize, 204u32, 2120588u32)?;
    emu.lw_no_count(12usize, 2usize, 208u32, 2120592u32)?;
    emu.lw_no_count(13usize, 2usize, 212u32, 2120596u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2120600u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2120604u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2120608u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2120612u32)?;
    emu.adi_no_count(10usize, 2usize, 232u32, 2120616u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2120620u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2120624u32);
    emu.apc_no_count(1usize, 2120624u32, 20480u32, 2120628u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 56u32, 2120636u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2120640u32)?;
    emu.lw_no_count(12usize, 2usize, 64u32, 2120644u32)?;
    emu.lw_no_count(13usize, 2usize, 68u32, 2120648u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2120652u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2120656u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2120660u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2120664u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2120668u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2120672u32)?;
    emu.lw_no_count(12usize, 2usize, 48u32, 2120676u32)?;
    emu.lw_no_count(13usize, 2usize, 52u32, 2120680u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2120684u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2120688u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2120692u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2120696u32)?;
    emu.lw_no_count(10usize, 2usize, 248u32, 2120700u32)?;
    emu.lw_no_count(11usize, 2usize, 252u32, 2120704u32)?;
    emu.lw_no_count(12usize, 2usize, 256u32, 2120708u32)?;
    emu.lw_no_count(13usize, 2usize, 260u32, 2120712u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2120716u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2120720u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2120724u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2120728u32)?;
    emu.lw_no_count(10usize, 2usize, 232u32, 2120732u32)?;
    emu.lw_no_count(11usize, 2usize, 236u32, 2120736u32)?;
    emu.lw_no_count(12usize, 2usize, 240u32, 2120740u32)?;
    emu.lw_no_count(13usize, 2usize, 244u32, 2120744u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2120748u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2120752u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2120756u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2120760u32)?;
    emu.adi_no_count(10usize, 2usize, 264u32, 2120764u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2120768u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2120772u32);
    emu.apc_no_count(1usize, 2120772u32, 20480u32, 2120776u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 296u32, 2120784u32);
    emu.adi_no_count(11usize, 2usize, 40u32, 2120788u32);
    emu.adi_no_count(12usize, 2usize, 232u32, 2120792u32);
    emu.apc_no_count(1usize, 2120792u32, 20480u32, 2120796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2120804u32);
    emu.adi_no_count(11usize, 2usize, 72u32, 2120808u32);
    emu.apc_no_count(1usize, 2120808u32, 53248u32, 2120812u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 88u32, 2120820u32)?;
    emu.lw_no_count(11usize, 2usize, 92u32, 2120824u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2120828u32)?;
    emu.lw_no_count(13usize, 2usize, 100u32, 2120832u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2120836u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2120840u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2120844u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2120848u32)?;
    emu.lw_no_count(10usize, 2usize, 72u32, 2120852u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2120856u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2120860u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2120864u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2120868u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2120872u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2120876u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2120880u32)?;
    emu.adi_no_count(10usize, 2usize, 328u32, 2120884u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2120888u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2120892u32);
    emu.apc_no_count(1usize, 2120892u32, 20480u32, 2120896u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(928u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 2usize, 568u32, 2120904u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2120908u32)?;
    emu.sw_no_count(21usize, 2usize, 576u32, 2120912u32)?;
    emu.sw_no_count(19usize, 2usize, 580u32, 2120916u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2120920u32)?;
    emu.sw_no_count(20usize, 2usize, 556u32, 2120924u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2120928u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2120932u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2120936u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2120940u32);
    emu.adi_no_count(12usize, 2usize, 168u32, 2120944u32);
    emu.apc_no_count(1usize, 2120944u32, 24576u32, 2120948u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2120956u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2120960u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2120964u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2120968u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2120972u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2120976u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2120980u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2120984u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2120988u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2120992u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2120996u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2121000u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2121004u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2121008u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2121012u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2121016u32)?;
    emu.lw_no_count(10usize, 2usize, 344u32, 2121020u32)?;
    emu.lw_no_count(11usize, 2usize, 348u32, 2121024u32)?;
    emu.lw_no_count(12usize, 2usize, 352u32, 2121028u32)?;
    emu.lw_no_count(13usize, 2usize, 356u32, 2121032u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2121036u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2121040u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2121044u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2121048u32)?;
    emu.lw_no_count(10usize, 2usize, 328u32, 2121052u32)?;
    emu.lw_no_count(11usize, 2usize, 332u32, 2121056u32)?;
    emu.lw_no_count(12usize, 2usize, 336u32, 2121060u32)?;
    emu.lw_no_count(13usize, 2usize, 340u32, 2121064u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2121068u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2121072u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2121076u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2121080u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2121084u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2121088u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2121092u32);
    emu.apc_no_count(1usize, 2121092u32, 20480u32, 2121096u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121100u32;
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
pub fn block_0x00205d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 360u32, 2121104u32);
    emu.adi_no_count(11usize, 2usize, 456u32, 2121108u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2121112u32);
    emu.apc_no_count(1usize, 2121112u32, 20480u32, 2121116u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121120u32;
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
pub fn block_0x00205da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2121124u32);
    emu.adi_no_count(11usize, 2usize, 360u32, 2121128u32);
    emu.apc_no_count(1usize, 2121128u32, 53248u32, 2121132u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 376u32, 2121140u32)?;
    emu.lw_no_count(11usize, 2usize, 380u32, 2121144u32)?;
    emu.lw_no_count(12usize, 2usize, 384u32, 2121148u32)?;
    emu.lw_no_count(13usize, 2usize, 388u32, 2121152u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2121156u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2121160u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2121164u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2121168u32)?;
    emu.lw_no_count(10usize, 2usize, 360u32, 2121172u32)?;
    emu.lw_no_count(11usize, 2usize, 364u32, 2121176u32)?;
    emu.lw_no_count(12usize, 2usize, 368u32, 2121180u32)?;
    emu.lw_no_count(13usize, 2usize, 372u32, 2121184u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2121188u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2121192u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2121196u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2121200u32)?;
    emu.adi_no_count(10usize, 2usize, 392u32, 2121204u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2121208u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2121212u32);
    emu.apc_no_count(1usize, 2121212u32, 20480u32, 2121216u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(608u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2121224u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2121228u32);
    emu.apc_no_count(1usize, 2121228u32, 53248u32, 2121232u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121236u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2121240u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2121244u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2121248u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2121252u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2121256u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2121260u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2121264u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2121268u32)?;
    emu.lw_no_count(10usize, 2usize, 8u32, 2121272u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2121276u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2121280u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2121284u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2121288u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2121292u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2121296u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2121300u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2121304u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2121308u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2121312u32);
    emu.apc_no_count(1usize, 2121312u32, 20480u32, 2121316u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 424u32, 2121324u32);
    emu.adi_no_count(11usize, 2usize, 488u32, 2121328u32);
    emu.adi_no_count(12usize, 2usize, 328u32, 2121332u32);
    emu.apc_no_count(1usize, 2121332u32, 20480u32, 2121336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205e7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 312u32, 2121344u32)?;
    emu.lw_no_count(11usize, 2usize, 316u32, 2121348u32)?;
    emu.lw_no_count(12usize, 2usize, 320u32, 2121352u32)?;
    emu.lw_no_count(13usize, 2usize, 324u32, 2121356u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2121360u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2121364u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2121368u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2121372u32)?;
    emu.lw_no_count(10usize, 2usize, 296u32, 2121376u32)?;
    emu.lw_no_count(11usize, 2usize, 300u32, 2121380u32)?;
    emu.lw_no_count(12usize, 2usize, 304u32, 2121384u32)?;
    emu.lw_no_count(13usize, 2usize, 308u32, 2121388u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2121392u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2121396u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2121400u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2121404u32)?;
    emu.lw_no_count(10usize, 2usize, 120u32, 2121408u32)?;
    emu.lw_no_count(11usize, 2usize, 124u32, 2121412u32)?;
    emu.lw_no_count(12usize, 2usize, 128u32, 2121416u32)?;
    emu.lw_no_count(13usize, 2usize, 132u32, 2121420u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2121424u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2121428u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2121432u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2121436u32)?;
    emu.lw_no_count(10usize, 2usize, 104u32, 2121440u32)?;
    emu.lw_no_count(11usize, 2usize, 108u32, 2121444u32)?;
    emu.lw_no_count(12usize, 2usize, 112u32, 2121448u32)?;
    emu.lw_no_count(13usize, 2usize, 116u32, 2121452u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2121456u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2121460u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2121464u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2121468u32)?;
    emu.adi_no_count(10usize, 2usize, 456u32, 2121472u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2121476u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2121480u32);
    emu.apc_no_count(1usize, 2121480u32, 20480u32, 2121484u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121488u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1632u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205f10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 152u32, 2121492u32)?;
    emu.lw_no_count(11usize, 2usize, 156u32, 2121496u32)?;
    emu.lw_no_count(12usize, 2usize, 160u32, 2121500u32)?;
    emu.lw_no_count(13usize, 2usize, 164u32, 2121504u32)?;
    emu.sw_no_count(10usize, 2usize, 536u32, 2121508u32)?;
    emu.sw_no_count(11usize, 2usize, 540u32, 2121512u32)?;
    emu.sw_no_count(12usize, 2usize, 544u32, 2121516u32)?;
    emu.sw_no_count(13usize, 2usize, 548u32, 2121520u32)?;
    emu.lw_no_count(10usize, 2usize, 136u32, 2121524u32)?;
    emu.lw_no_count(11usize, 2usize, 140u32, 2121528u32)?;
    emu.lw_no_count(12usize, 2usize, 144u32, 2121532u32)?;
    emu.lw_no_count(13usize, 2usize, 148u32, 2121536u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2121540u32)?;
    emu.sw_no_count(11usize, 2usize, 524u32, 2121544u32)?;
    emu.sw_no_count(12usize, 2usize, 528u32, 2121548u32)?;
    emu.sw_no_count(13usize, 2usize, 532u32, 2121552u32)?;
    emu.lw_no_count(10usize, 2usize, 408u32, 2121556u32)?;
    emu.lw_no_count(11usize, 2usize, 412u32, 2121560u32)?;
    emu.lw_no_count(12usize, 2usize, 416u32, 2121564u32)?;
    emu.lw_no_count(13usize, 2usize, 420u32, 2121568u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2121572u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2121576u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2121580u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2121584u32)?;
    emu.lw_no_count(10usize, 2usize, 392u32, 2121588u32)?;
    emu.lw_no_count(11usize, 2usize, 396u32, 2121592u32)?;
    emu.lw_no_count(12usize, 2usize, 400u32, 2121596u32)?;
    emu.lw_no_count(13usize, 2usize, 404u32, 2121600u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2121604u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2121608u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2121612u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2121616u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2121620u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2121624u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2121628u32);
    emu.apc_no_count(1usize, 2121628u32, 20480u32, 2121632u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 456u32, 2121640u32);
    emu.adi_no_count(12usize, 2usize, 488u32, 2121644u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2121648u32);
    emu.apc_no_count(1usize, 2121648u32, 20480u32, 2121652u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 280u32, 2121660u32)?;
    emu.lw_no_count(11usize, 2usize, 284u32, 2121664u32)?;
    emu.lw_no_count(12usize, 2usize, 288u32, 2121668u32)?;
    emu.lw_no_count(13usize, 2usize, 292u32, 2121672u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2121676u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2121680u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2121684u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2121688u32)?;
    emu.lw_no_count(10usize, 2usize, 264u32, 2121692u32)?;
    emu.lw_no_count(11usize, 2usize, 268u32, 2121696u32)?;
    emu.lw_no_count(12usize, 2usize, 272u32, 2121700u32)?;
    emu.lw_no_count(13usize, 2usize, 276u32, 2121704u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2121708u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2121712u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2121716u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2121720u32)?;
    emu.adi_no_count(10usize, 2usize, 488u32, 2121724u32);
    emu.adi_no_count(11usize, 2usize, 296u32, 2121728u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2121732u32);
    emu.apc_no_count(1usize, 2121732u32, 20480u32, 2121736u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020600c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 440u32, 2121744u32)?;
    emu.lw_no_count(11usize, 2usize, 444u32, 2121748u32)?;
    emu.lw_no_count(12usize, 2usize, 448u32, 2121752u32)?;
    emu.lw_no_count(13usize, 2usize, 452u32, 2121756u32)?;
    emu.sw_no_count(10usize, 2usize, 568u32, 2121760u32)?;
    emu.sw_no_count(11usize, 2usize, 572u32, 2121764u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2121768u32)?;
    emu.sw_no_count(13usize, 2usize, 580u32, 2121772u32)?;
    emu.lw_no_count(10usize, 2usize, 424u32, 2121776u32)?;
    emu.lw_no_count(11usize, 2usize, 428u32, 2121780u32)?;
    emu.lw_no_count(12usize, 2usize, 432u32, 2121784u32)?;
    emu.lw_no_count(13usize, 2usize, 436u32, 2121788u32)?;
    emu.sw_no_count(10usize, 2usize, 552u32, 2121792u32)?;
    emu.sw_no_count(11usize, 2usize, 556u32, 2121796u32)?;
    emu.sw_no_count(12usize, 2usize, 560u32, 2121800u32)?;
    emu.sw_no_count(13usize, 2usize, 564u32, 2121804u32)?;
    emu.adi_no_count(10usize, 2usize, 520u32, 2121808u32);
    emu.adi_no_count(11usize, 2usize, 552u32, 2121812u32);
    emu.adi_no_count(12usize, 2usize, 392u32, 2121816u32);
    emu.apc_no_count(1usize, 2121816u32, 20480u32, 2121820u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 32u32, 2121828u32);
    emu.adi_no_count(11usize, 2usize, 488u32, 2121832u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2121836u32);
    emu.apc_no_count(1usize, 2121836u32, 20480u32, 2121840u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121844u32;
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
pub fn block_0x00206074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2121848u32);
    emu.adi_no_count(11usize, 2usize, 264u32, 2121852u32);
    emu.adi_no_count(12usize, 2usize, 136u32, 2121856u32);
    emu.apc_no_count(1usize, 2121856u32, 20480u32, 2121860u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 552u32, 2121868u32);
    emu.adi_no_count(11usize, 2usize, 104u32, 2121872u32);
    emu.adi_no_count(12usize, 2usize, 424u32, 2121876u32);
    emu.apc_no_count(1usize, 2121876u32, 20480u32, 2121880u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020609c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 64u32, 2121888u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2121892u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2121896u32);
    emu.apc_no_count(1usize, 2121896u32, 20480u32, 2121900u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002060b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 620u32, 2121908u32)?;
    emu.lw_no_count(8usize, 2usize, 616u32, 2121912u32)?;
    emu.lw_no_count(9usize, 2usize, 612u32, 2121916u32)?;
    emu.lw_no_count(18usize, 2usize, 608u32, 2121920u32)?;
    emu.lw_no_count(19usize, 2usize, 604u32, 2121924u32)?;
    emu.lw_no_count(20usize, 2usize, 600u32, 2121928u32)?;
    emu.lw_no_count(21usize, 2usize, 596u32, 2121932u32)?;
    emu.lw_no_count(22usize, 2usize, 592u32, 2121936u32)?;
    emu.lw_no_count(23usize, 2usize, 588u32, 2121940u32)?;
    emu.lw_no_count(24usize, 2usize, 584u32, 2121944u32)?;
    emu.adi_no_count(2usize, 2usize, 624u32, 2121948u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121952u32;
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
pub fn block_0x002060e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966576u32, 2121956u32);
    emu.sw_no_count(1usize, 2usize, 716u32, 2121960u32)?;
    emu.sw_no_count(8usize, 2usize, 712u32, 2121964u32)?;
    emu.sw_no_count(9usize, 2usize, 708u32, 2121968u32)?;
    emu.sw_no_count(18usize, 2usize, 704u32, 2121972u32)?;
    emu.sw_no_count(19usize, 2usize, 700u32, 2121976u32)?;
    emu.sw_no_count(20usize, 2usize, 696u32, 2121980u32)?;
    emu.sw_no_count(21usize, 2usize, 692u32, 2121984u32)?;
    emu.sw_no_count(22usize, 2usize, 688u32, 2121988u32)?;
    emu.sw_no_count(23usize, 2usize, 684u32, 2121992u32)?;
    emu.sw_no_count(24usize, 2usize, 680u32, 2121996u32)?;
    emu.sw_no_count(25usize, 2usize, 676u32, 2122000u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2122004u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2122008u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2122012u32);
    emu.apc_no_count(1usize, 2122012u32, 49152u32, 2122016u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122020u32;
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
pub fn block_0x00206124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 32u32, 2122024u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2122028u32);
    emu.apc_no_count(1usize, 2122028u32, 49152u32, 2122032u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122036u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 64u32, 2122040u32);
    emu.adi_no_count(10usize, 2usize, 68u32, 2122044u32);
    emu.apc_no_count(1usize, 2122044u32, 49152u32, 2122048u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1800u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00206144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2122056u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2122060u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2122064u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2122068u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2122072u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2122076u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2122080u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2122084u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2122088u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2122092u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2122096u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2122100u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2122104u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2122108u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2122112u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2122116u32)?;
    emu.lw_no_count(10usize, 9usize, 48u32, 2122120u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2122124u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2122128u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2122132u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2122136u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2122140u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2122144u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2122148u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2122152u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2122156u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2122160u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2122164u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2122168u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2122172u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2122176u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2122180u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2122184u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2122188u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2122192u32);
    emu.apc_no_count(1usize, 2122192u32, 20480u32, 2122196u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002061d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 100u32, 2122204u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2122208u32);
    emu.apc_no_count(1usize, 2122208u32, 49152u32, 2122212u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002061e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 16u32, 2122220u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2122224u32)?;
    emu.lw_no_count(12usize, 9usize, 24u32, 2122228u32)?;
    emu.lw_no_count(13usize, 9usize, 28u32, 2122232u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2122236u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2122240u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2122244u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2122248u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2122252u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2122256u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2122260u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2122264u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2122268u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2122272u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2122276u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2122280u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2122284u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2122288u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2122292u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2122296u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2122300u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2122304u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2122308u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2122312u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2122316u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2122320u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2122324u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2122328u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2122332u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2122336u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2122340u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2122344u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2122348u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2122352u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2122356u32);
    emu.apc_no_count(1usize, 2122356u32, 20480u32, 2122360u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020627c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 132u32, 2122368u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2122372u32);
    emu.apc_no_count(1usize, 2122372u32, 49152u32, 2122376u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020628c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 45u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 84u32, 2122384u32)?;
    emu.lw_no_count(11usize, 2usize, 88u32, 2122388u32)?;
    emu.lw_no_count(12usize, 2usize, 92u32, 2122392u32)?;
    emu.lw_no_count(13usize, 2usize, 96u32, 2122396u32)?;
    emu.lw_no_count(14usize, 2usize, 68u32, 2122400u32)?;
    emu.lw_no_count(15usize, 2usize, 72u32, 2122404u32)?;
    emu.lw_no_count(16usize, 2usize, 76u32, 2122408u32)?;
    emu.lw_no_count(17usize, 2usize, 80u32, 2122412u32)?;
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2122416u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2122420u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2122424u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2122428u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2122432u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(30usize, a);
    emu.pc = 2122436u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 628u32, 2122440u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2122444u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2122448u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2122452u32)?;
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122456u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 2usize, 612u32, 2122460u32)?;
    emu.sw_no_count(15usize, 2usize, 616u32, 2122464u32)?;
    emu.sw_no_count(16usize, 2usize, 620u32, 2122468u32)?;
    emu.sw_no_count(17usize, 2usize, 624u32, 2122472u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122476u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 5usize, 1565u32, 2122480u32);
    emu.adi_no_count(22usize, 6usize, 4294965300u32, 2122484u32);
    emu.adi_no_count(24usize, 7usize, 171u32, 2122488u32);
    emu.adi_no_count(25usize, 28usize, 4294966998u32, 2122492u32);
    emu.adi_no_count(18usize, 29usize, 1485u32, 2122496u32);
    emu.adi_no_count(19usize, 30usize, 144u32, 2122500u32);
    emu.adi_no_count(21usize, 10usize, 4294967138u32, 2122504u32);
    emu.adi_no_count(23usize, 11usize, 4294966751u32, 2122508u32);
    emu.sw_no_count(25usize, 2usize, 660u32, 2122512u32)?;
    emu.sw_no_count(24usize, 2usize, 664u32, 2122516u32)?;
    emu.sw_no_count(22usize, 2usize, 668u32, 2122520u32)?;
    emu.sw_no_count(20usize, 2usize, 672u32, 2122524u32)?;
    emu.sw_no_count(23usize, 2usize, 644u32, 2122528u32)?;
    emu.sw_no_count(21usize, 2usize, 648u32, 2122532u32)?;
    emu.sw_no_count(19usize, 2usize, 652u32, 2122536u32)?;
    emu.sw_no_count(18usize, 2usize, 656u32, 2122540u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2122544u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2122548u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2122552u32);
    emu.apc_no_count(1usize, 2122552u32, 20480u32, 2122556u32);
    emu.add_memory_rw_events(45usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 148u32, 2122564u32)?;
    emu.lw_no_count(11usize, 2usize, 152u32, 2122568u32)?;
    emu.lw_no_count(12usize, 2usize, 156u32, 2122572u32)?;
    emu.lw_no_count(13usize, 2usize, 160u32, 2122576u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2122580u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2122584u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2122588u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2122592u32)?;
    emu.lw_no_count(10usize, 2usize, 132u32, 2122596u32)?;
    emu.lw_no_count(11usize, 2usize, 136u32, 2122600u32)?;
    emu.lw_no_count(12usize, 2usize, 140u32, 2122604u32)?;
    emu.lw_no_count(13usize, 2usize, 144u32, 2122608u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2122612u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2122616u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2122620u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2122624u32)?;
    emu.adi_no_count(10usize, 2usize, 164u32, 2122628u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2122632u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2122636u32);
    emu.apc_no_count(1usize, 2122636u32, 20480u32, 2122640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2122648u32);
    emu.adi_no_count(11usize, 2usize, 164u32, 2122652u32);
    emu.apc_no_count(1usize, 2122652u32, 49152u32, 2122656u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122660u32;
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
#[inline]
pub fn block_0x002063a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 180u32, 2122664u32)?;
    emu.lw_no_count(11usize, 2usize, 184u32, 2122668u32)?;
    emu.lw_no_count(12usize, 2usize, 188u32, 2122672u32)?;
    emu.lw_no_count(13usize, 2usize, 192u32, 2122676u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2122680u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2122684u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2122688u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2122692u32)?;
    emu.lw_no_count(10usize, 2usize, 164u32, 2122696u32)?;
    emu.lw_no_count(11usize, 2usize, 168u32, 2122700u32)?;
    emu.lw_no_count(12usize, 2usize, 172u32, 2122704u32)?;
    emu.lw_no_count(13usize, 2usize, 176u32, 2122708u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2122712u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2122716u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2122720u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2122724u32)?;
    emu.adi_no_count(10usize, 2usize, 196u32, 2122728u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2122732u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2122736u32);
    emu.apc_no_count(1usize, 2122736u32, 20480u32, 2122740u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002063f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 52u32, 2122748u32)?;
    emu.lw_no_count(11usize, 2usize, 56u32, 2122752u32)?;
    emu.lw_no_count(12usize, 2usize, 60u32, 2122756u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2122760u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2122764u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2122768u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2122772u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2122776u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2122780u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2122784u32)?;
    emu.lw_no_count(12usize, 2usize, 44u32, 2122788u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2122792u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2122796u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2122800u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2122804u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2122808u32)?;
    emu.lw_no_count(10usize, 2usize, 212u32, 2122812u32)?;
    emu.lw_no_count(11usize, 2usize, 216u32, 2122816u32)?;
    emu.lw_no_count(12usize, 2usize, 220u32, 2122820u32)?;
    emu.lw_no_count(13usize, 2usize, 224u32, 2122824u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2122828u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2122832u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2122836u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2122840u32)?;
    emu.lw_no_count(10usize, 2usize, 196u32, 2122844u32)?;
    emu.lw_no_count(11usize, 2usize, 200u32, 2122848u32)?;
    emu.lw_no_count(12usize, 2usize, 204u32, 2122852u32)?;
    emu.lw_no_count(13usize, 2usize, 208u32, 2122856u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2122860u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2122864u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2122868u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2122872u32)?;
    emu.adi_no_count(10usize, 2usize, 228u32, 2122876u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2122880u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2122884u32);
    emu.apc_no_count(1usize, 2122884u32, 20480u32, 2122888u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020648c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 52u32, 2122896u32)?;
    emu.lw_no_count(11usize, 2usize, 56u32, 2122900u32)?;
    emu.lw_no_count(12usize, 2usize, 60u32, 2122904u32)?;
    emu.lw_no_count(13usize, 2usize, 64u32, 2122908u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2122912u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2122916u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2122920u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2122924u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2122928u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2122932u32)?;
    emu.lw_no_count(12usize, 2usize, 44u32, 2122936u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2122940u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2122944u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2122948u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2122952u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2122956u32)?;
    emu.adi_no_count(10usize, 2usize, 260u32, 2122960u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2122964u32);
    emu.adi_no_count(12usize, 2usize, 196u32, 2122968u32);
    emu.apc_no_count(1usize, 2122968u32, 20480u32, 2122972u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002064e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 244u32, 2122980u32)?;
    emu.lw_no_count(11usize, 2usize, 248u32, 2122984u32)?;
    emu.lw_no_count(12usize, 2usize, 252u32, 2122988u32)?;
    emu.lw_no_count(13usize, 2usize, 256u32, 2122992u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2122996u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123000u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123004u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123008u32)?;
    emu.lw_no_count(10usize, 2usize, 228u32, 2123012u32)?;
    emu.lw_no_count(11usize, 2usize, 232u32, 2123016u32)?;
    emu.lw_no_count(12usize, 2usize, 236u32, 2123020u32)?;
    emu.lw_no_count(13usize, 2usize, 240u32, 2123024u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123028u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123032u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123036u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123040u32)?;
    emu.adi_no_count(10usize, 2usize, 292u32, 2123044u32);
    emu.adi_no_count(11usize, 2usize, 260u32, 2123048u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2123052u32);
    emu.apc_no_count(1usize, 2123052u32, 20480u32, 2123056u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 324u32, 2123064u32);
    emu.adi_no_count(11usize, 2usize, 228u32, 2123068u32);
    emu.adi_no_count(12usize, 2usize, 100u32, 2123072u32);
    emu.apc_no_count(1usize, 2123072u32, 20480u32, 2123076u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(40u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2123084u32);
    emu.adi_no_count(11usize, 2usize, 68u32, 2123088u32);
    emu.apc_no_count(1usize, 2123088u32, 49152u32, 2123092u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 84u32, 2123100u32)?;
    emu.lw_no_count(11usize, 2usize, 88u32, 2123104u32)?;
    emu.lw_no_count(12usize, 2usize, 92u32, 2123108u32)?;
    emu.lw_no_count(13usize, 2usize, 96u32, 2123112u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2123116u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123120u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123124u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123128u32)?;
    emu.lw_no_count(10usize, 2usize, 68u32, 2123132u32)?;
    emu.lw_no_count(11usize, 2usize, 72u32, 2123136u32)?;
    emu.lw_no_count(12usize, 2usize, 76u32, 2123140u32)?;
    emu.lw_no_count(13usize, 2usize, 80u32, 2123144u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123148u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123152u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123156u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123160u32)?;
    emu.adi_no_count(10usize, 2usize, 356u32, 2123164u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2123168u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2123172u32);
    emu.apc_no_count(1usize, 2123172u32, 20480u32, 2123176u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002065ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 660u32, 2123184u32)?;
    emu.sw_no_count(24usize, 2usize, 664u32, 2123188u32)?;
    emu.sw_no_count(22usize, 2usize, 668u32, 2123192u32)?;
    emu.sw_no_count(20usize, 2usize, 672u32, 2123196u32)?;
    emu.sw_no_count(23usize, 2usize, 644u32, 2123200u32)?;
    emu.sw_no_count(21usize, 2usize, 648u32, 2123204u32)?;
    emu.sw_no_count(19usize, 2usize, 652u32, 2123208u32)?;
    emu.sw_no_count(18usize, 2usize, 656u32, 2123212u32)?;
    emu.adi_no_count(10usize, 2usize, 420u32, 2123216u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2123220u32);
    emu.adi_no_count(12usize, 2usize, 132u32, 2123224u32);
    emu.apc_no_count(1usize, 2123224u32, 20480u32, 2123228u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002065e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2123236u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2123240u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2123244u32)?;
    emu.lw_no_count(13usize, 2usize, 32u32, 2123248u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2123252u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2123256u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2123260u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2123264u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2123268u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2123272u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2123276u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2123280u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2123284u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2123288u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2123292u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2123296u32)?;
    emu.lw_no_count(10usize, 2usize, 372u32, 2123300u32)?;
    emu.lw_no_count(11usize, 2usize, 376u32, 2123304u32)?;
    emu.lw_no_count(12usize, 2usize, 380u32, 2123308u32)?;
    emu.lw_no_count(13usize, 2usize, 384u32, 2123312u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2123316u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123320u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123324u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123328u32)?;
    emu.lw_no_count(10usize, 2usize, 356u32, 2123332u32)?;
    emu.lw_no_count(11usize, 2usize, 360u32, 2123336u32)?;
    emu.lw_no_count(12usize, 2usize, 364u32, 2123340u32)?;
    emu.lw_no_count(13usize, 2usize, 368u32, 2123344u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123348u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123352u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123356u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123360u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2123364u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2123368u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2123372u32);
    emu.apc_no_count(1usize, 2123372u32, 20480u32, 2123376u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 388u32, 2123384u32);
    emu.adi_no_count(11usize, 2usize, 420u32, 2123388u32);
    emu.adi_no_count(12usize, 2usize, 580u32, 2123392u32);
    emu.apc_no_count(1usize, 2123392u32, 20480u32, 2123396u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2123404u32);
    emu.adi_no_count(11usize, 2usize, 388u32, 2123408u32);
    emu.apc_no_count(1usize, 2123408u32, 49152u32, 2123412u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(16u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 404u32, 2123420u32)?;
    emu.lw_no_count(11usize, 2usize, 408u32, 2123424u32)?;
    emu.lw_no_count(12usize, 2usize, 412u32, 2123428u32)?;
    emu.lw_no_count(13usize, 2usize, 416u32, 2123432u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2123436u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123440u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123444u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123448u32)?;
    emu.lw_no_count(10usize, 2usize, 388u32, 2123452u32)?;
    emu.lw_no_count(11usize, 2usize, 392u32, 2123456u32)?;
    emu.lw_no_count(12usize, 2usize, 396u32, 2123460u32)?;
    emu.lw_no_count(13usize, 2usize, 400u32, 2123464u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123468u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123472u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123476u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123480u32)?;
    emu.adi_no_count(10usize, 2usize, 452u32, 2123484u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2123488u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2123492u32);
    emu.apc_no_count(1usize, 2123492u32, 20480u32, 2123496u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002066ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2123504u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2123508u32);
    emu.apc_no_count(1usize, 2123508u32, 49152u32, 2123512u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002066fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2123520u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2123524u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2123528u32)?;
    emu.lw_no_count(13usize, 2usize, 32u32, 2123532u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2123536u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123540u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123544u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123548u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2123552u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2123556u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2123560u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2123564u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123568u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123572u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123576u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123580u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2123584u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2123588u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2123592u32);
    emu.apc_no_count(1usize, 2123592u32, 20480u32, 2123596u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 484u32, 2123604u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2123608u32);
    emu.adi_no_count(12usize, 2usize, 356u32, 2123612u32);
    emu.apc_no_count(1usize, 2123612u32, 20480u32, 2123616u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123620u32;
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
#[inline]
pub fn block_0x00206764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 468u32, 2123624u32)?;
    emu.lw_no_count(11usize, 2usize, 472u32, 2123628u32)?;
    emu.lw_no_count(12usize, 2usize, 476u32, 2123632u32)?;
    emu.lw_no_count(13usize, 2usize, 480u32, 2123636u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2123640u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123644u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123648u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123652u32)?;
    emu.lw_no_count(10usize, 2usize, 452u32, 2123656u32)?;
    emu.lw_no_count(11usize, 2usize, 456u32, 2123660u32)?;
    emu.lw_no_count(12usize, 2usize, 460u32, 2123664u32)?;
    emu.lw_no_count(13usize, 2usize, 464u32, 2123668u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123672u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123676u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123680u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123684u32)?;
    emu.adi_no_count(10usize, 2usize, 612u32, 2123688u32);
    emu.adi_no_count(11usize, 2usize, 484u32, 2123692u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2123696u32);
    emu.apc_no_count(1usize, 2123696u32, 20480u32, 2123700u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002067b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 516u32, 2123708u32);
    emu.adi_no_count(11usize, 2usize, 292u32, 2123712u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2123716u32);
    emu.apc_no_count(1usize, 2123716u32, 20480u32, 2123720u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002067cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 48u32, 2123728u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2123732u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2123736u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2123740u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2123744u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2123748u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2123752u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2123756u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2123760u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2123764u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2123768u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2123772u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2123776u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2123780u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2123784u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2123788u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2123792u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2123796u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2123800u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2123804u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2123808u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123812u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123816u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123820u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2123824u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2123828u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2123832u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2123836u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123840u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123844u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123848u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123852u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2123856u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2123860u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2123864u32);
    emu.apc_no_count(1usize, 2123864u32, 20480u32, 2123868u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123872u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 548u32, 2123876u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2123880u32);
    emu.apc_no_count(1usize, 2123880u32, 49152u32, 2123884u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123888u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00206870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 564u32, 2123892u32)?;
    emu.lw_no_count(11usize, 2usize, 568u32, 2123896u32)?;
    emu.lw_no_count(12usize, 2usize, 572u32, 2123900u32)?;
    emu.lw_no_count(13usize, 2usize, 576u32, 2123904u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2123908u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2123912u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2123916u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2123920u32)?;
    emu.lw_no_count(10usize, 2usize, 548u32, 2123924u32)?;
    emu.lw_no_count(11usize, 2usize, 552u32, 2123928u32)?;
    emu.lw_no_count(12usize, 2usize, 556u32, 2123932u32)?;
    emu.lw_no_count(13usize, 2usize, 560u32, 2123936u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2123940u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2123944u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2123948u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2123952u32)?;
    emu.adi_no_count(10usize, 2usize, 612u32, 2123956u32);
    emu.adi_no_count(11usize, 2usize, 452u32, 2123960u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2123964u32);
    emu.apc_no_count(1usize, 2123964u32, 20480u32, 2123968u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123972u32;
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
pub fn block_0x002068c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 324u32, 2123976u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2123980u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2123984u32);
    emu.apc_no_count(1usize, 2123984u32, 20480u32, 2123988u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002068d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 644u32, 2123996u32);
    emu.adi_no_count(11usize, 2usize, 548u32, 2124000u32);
    emu.adi_no_count(12usize, 2usize, 36u32, 2124004u32);
    emu.apc_no_count(1usize, 2124004u32, 20480u32, 2124008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002068ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2124016u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2124020u32);
    emu.apc_no_count(1usize, 2124020u32, 49152u32, 2124024u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002068fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 64u32, 2124032u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2124036u32);
    emu.apc_no_count(1usize, 2124036u32, 49152u32, 2124040u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020690c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 532u32, 2124048u32)?;
    emu.lw_no_count(11usize, 2usize, 536u32, 2124052u32)?;
    emu.lw_no_count(12usize, 2usize, 540u32, 2124056u32)?;
    emu.lw_no_count(13usize, 2usize, 544u32, 2124060u32)?;
    emu.sw_no_count(10usize, 8usize, 48u32, 2124064u32)?;
    emu.sw_no_count(11usize, 8usize, 52u32, 2124068u32)?;
    emu.sw_no_count(12usize, 8usize, 56u32, 2124072u32)?;
    emu.sw_no_count(13usize, 8usize, 60u32, 2124076u32)?;
    emu.lw_no_count(10usize, 2usize, 516u32, 2124080u32)?;
    emu.lw_no_count(11usize, 2usize, 520u32, 2124084u32)?;
    emu.lw_no_count(12usize, 2usize, 524u32, 2124088u32)?;
    emu.lw_no_count(13usize, 2usize, 528u32, 2124092u32)?;
    emu.sw_no_count(10usize, 8usize, 32u32, 2124096u32)?;
    emu.sw_no_count(11usize, 8usize, 36u32, 2124100u32)?;
    emu.sw_no_count(12usize, 8usize, 40u32, 2124104u32)?;
    emu.sw_no_count(13usize, 8usize, 44u32, 2124108u32)?;
    emu.lw_no_count(1usize, 2usize, 716u32, 2124112u32)?;
    emu.lw_no_count(8usize, 2usize, 712u32, 2124116u32)?;
    emu.lw_no_count(9usize, 2usize, 708u32, 2124120u32)?;
    emu.lw_no_count(18usize, 2usize, 704u32, 2124124u32)?;
    emu.lw_no_count(19usize, 2usize, 700u32, 2124128u32)?;
    emu.lw_no_count(20usize, 2usize, 696u32, 2124132u32)?;
    emu.lw_no_count(21usize, 2usize, 692u32, 2124136u32)?;
    emu.lw_no_count(22usize, 2usize, 688u32, 2124140u32)?;
    emu.lw_no_count(23usize, 2usize, 684u32, 2124144u32)?;
    emu.lw_no_count(24usize, 2usize, 680u32, 2124148u32)?;
    emu.lw_no_count(25usize, 2usize, 676u32, 2124152u32)?;
    emu.adi_no_count(2usize, 2usize, 720u32, 2124156u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124160u32;
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
pub fn block_0x00206980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2124164u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2124168u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2124172u32)?;
    emu.lbu_no_count(10usize, 12usize, 0u32, 2124176u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2124180u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2124184u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 336u32, 2124188u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2124192u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2124196u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2124200u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2124204u32;
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
pub fn block_0x002069ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2124208u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2124212u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124216u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1012u32, 2124220u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2124224u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 996u32, 2124228u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2124232u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2124236u32);
    emu.apc_no_count(1usize, 2124236u32, 114688u32, 2124240u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002069d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2124248u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124252u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124256u32;
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
pub fn block_0x002069e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124260u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1243u32, 2124264u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2124268u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124272u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124276u32);
    emu.apc_no_count(6usize, 2124276u32, 114688u32, 2124280u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124284u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
