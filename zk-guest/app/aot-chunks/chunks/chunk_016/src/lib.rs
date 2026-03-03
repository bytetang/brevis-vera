pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2162440u32;
pub const PC_MAX: u32 = 2171368u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 129usize] = [
        block_0x0020ff08,
        block_0x0020ff5c,
        block_0x0020ffc8,
        block_0x0020ffcc,
        block_0x0020ffe0,
        block_0x00210028,
        block_0x0021007c,
        block_0x00210080,
        block_0x00210094,
        block_0x002100dc,
        block_0x00210130,
        block_0x00210144,
        block_0x00210198,
        block_0x00210204,
        block_0x0021021c,
        block_0x00210248,
        block_0x0021025c,
        block_0x002102f0,
        block_0x00210344,
        block_0x002103b0,
        block_0x00210444,
        block_0x00210498,
        block_0x002104ec,
        block_0x00210540,
        block_0x002105ac,
        block_0x00210600,
        block_0x00210654,
        block_0x002106a8,
        block_0x002106fc,
        block_0x00210750,
        block_0x002107a4,
        block_0x002107f8,
        block_0x0021084c,
        block_0x002108b8,
        block_0x0021090c,
        block_0x00210960,
        block_0x002109b4,
        block_0x00210a08,
        block_0x00210a5c,
        block_0x00210ab0,
        block_0x00210b04,
        block_0x00210b58,
        block_0x00210bac,
        block_0x00210c00,
        block_0x00210c54,
        block_0x00210ca8,
        block_0x00210cfc,
        block_0x00210d50,
        block_0x00210da4,
        block_0x00210df8,
        block_0x00210e64,
        block_0x00210e78,
        block_0x00210ecc,
        block_0x00210f20,
        block_0x00210f74,
        block_0x00210fc8,
        block_0x0021101c,
        block_0x00211070,
        block_0x002110c4,
        block_0x00211118,
        block_0x0021116c,
        block_0x002111c0,
        block_0x00211214,
        block_0x00211268,
        block_0x002112bc,
        block_0x00211310,
        block_0x00211364,
        block_0x002113b8,
        block_0x0021140c,
        block_0x00211460,
        block_0x002114b4,
        block_0x00211508,
        block_0x0021155c,
        block_0x002115b0,
        block_0x00211604,
        block_0x00211658,
        block_0x002116ac,
        block_0x00211700,
        block_0x00211754,
        block_0x002117a8,
        block_0x002117fc,
        block_0x00211850,
        block_0x002118a4,
        block_0x00211910,
        block_0x00211914,
        block_0x00211928,
        block_0x00211970,
        block_0x002119c4,
        block_0x002119c8,
        block_0x002119dc,
        block_0x00211a24,
        block_0x00211a98,
        block_0x00211b24,
        block_0x00211b40,
        block_0x00211d6c,
        block_0x00211da8,
        block_0x00211de0,
        block_0x00211e20,
        block_0x00211e24,
        block_0x00211e68,
        block_0x00211e6c,
        block_0x00211ebc,
        block_0x00211ec0,
        block_0x00211f30,
        block_0x00211f34,
        block_0x00211f84,
        block_0x00211f88,
        block_0x00211fc4,
        block_0x00211fc8,
        block_0x00212018,
        block_0x0021201c,
        block_0x00212054,
        block_0x00212058,
        block_0x002120a8,
        block_0x002120ac,
        block_0x002120d4,
        block_0x002120dc,
        block_0x002120e0,
        block_0x00212114,
        block_0x00212118,
        block_0x0021213c,
        block_0x00212140,
        block_0x0021215c,
        block_0x00212164,
        block_0x00212168,
        block_0x00212198,
        block_0x0021219c,
        block_0x002121e4,
        block_0x002121e8,
    ];
    #[repr(C)]
    struct Run {
        start_word: u32,
        len: u16,
        fn_offset: u16,
    }
    const RUNS: [Run; 110usize] = [
        Run {
            start_word: 0u32,
            len: 1i32 as u16,
            fn_offset: 0usize as u16,
        },
        Run {
            start_word: 21u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 48u32,
            len: 2i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 54u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 72u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 93u32,
            len: 2i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 99u32,
            len: 1i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 117u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 138u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 143u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 164u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 191u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 197u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 208u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 213u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 250u32,
            len: 1i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 271u32,
            len: 1i32 as u16,
            fn_offset: 18usize as u16,
        },
        Run {
            start_word: 298u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 335u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 356u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 377u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 398u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 425u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 446u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 467u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 488u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 509u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 530u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 551u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 572u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 593u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 620u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 641u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 662u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 683u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 704u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 725u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 746u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 767u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 788u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 809u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 830u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 851u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 872u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 893u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 914u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 935u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 956u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 983u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 988u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 1009u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 1030u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 1051u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 1072u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 1093u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 1114u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 1135u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 1156u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 1177u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 1198u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 1219u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 1240u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 1261u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 1282u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 1303u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 1324u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 1345u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 1366u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 1387u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 1408u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 1429u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 1450u32,
            len: 1i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 1471u32,
            len: 1i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 1492u32,
            len: 1i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 1513u32,
            len: 1i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 1534u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 1555u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 1576u32,
            len: 1i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 1597u32,
            len: 1i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 1618u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 1639u32,
            len: 1i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 1666u32,
            len: 2i32 as u16,
            fn_offset: 83usize as u16,
        },
        Run {
            start_word: 1672u32,
            len: 1i32 as u16,
            fn_offset: 85usize as u16,
        },
        Run {
            start_word: 1690u32,
            len: 1i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 1711u32,
            len: 2i32 as u16,
            fn_offset: 87usize as u16,
        },
        Run {
            start_word: 1717u32,
            len: 1i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 1735u32,
            len: 1i32 as u16,
            fn_offset: 90usize as u16,
        },
        Run {
            start_word: 1764u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 1799u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 1806u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 1945u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 1960u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 1974u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 1990u32,
            len: 2i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 2008u32,
            len: 2i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 2029u32,
            len: 2i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 2058u32,
            len: 2i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 2079u32,
            len: 2i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 2095u32,
            len: 2i32 as u16,
            fn_offset: 107usize as u16,
        },
        Run {
            start_word: 2116u32,
            len: 2i32 as u16,
            fn_offset: 109usize as u16,
        },
        Run {
            start_word: 2131u32,
            len: 2i32 as u16,
            fn_offset: 111usize as u16,
        },
        Run {
            start_word: 2152u32,
            len: 2i32 as u16,
            fn_offset: 113usize as u16,
        },
        Run {
            start_word: 2163u32,
            len: 1i32 as u16,
            fn_offset: 115usize as u16,
        },
        Run {
            start_word: 2165u32,
            len: 2i32 as u16,
            fn_offset: 116usize as u16,
        },
        Run {
            start_word: 2179u32,
            len: 2i32 as u16,
            fn_offset: 118usize as u16,
        },
        Run {
            start_word: 2189u32,
            len: 2i32 as u16,
            fn_offset: 120usize as u16,
        },
        Run {
            start_word: 2197u32,
            len: 1i32 as u16,
            fn_offset: 122usize as u16,
        },
        Run {
            start_word: 2199u32,
            len: 2i32 as u16,
            fn_offset: 123usize as u16,
        },
        Run {
            start_word: 2212u32,
            len: 2i32 as u16,
            fn_offset: 125usize as u16,
        },
        Run {
            start_word: 2231u32,
            len: 2i32 as u16,
            fn_offset: 127usize as u16,
        },
    ];
    if pc < 2162440u32 || pc > 2171368u32 {
        return None;
    }
    let word_offset = ((pc - 2162440u32) >> 2) as u32;
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
pub fn block_0x0020ff08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162444u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162448u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162452u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162456u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162460u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162464u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162468u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162472u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162476u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162480u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162484u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162488u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2162492u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162496u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162500u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162504u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2162508u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162512u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162516u32);
    emu.apc_no_count(1usize, 2162516u32, 4294946816u32, 2162520u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ff5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162528u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162532u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162536u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162540u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2162544u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2162548u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2162552u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2162556u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162560u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162564u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2162568u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2162572u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162576u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162580u32)?;
    emu.sw_no_count(14usize, 2usize, 256u32, 2162584u32)?;
    emu.sw_no_count(15usize, 2usize, 260u32, 2162588u32)?;
    emu.sw_no_count(16usize, 2usize, 264u32, 2162592u32)?;
    emu.sw_no_count(17usize, 2usize, 268u32, 2162596u32)?;
    emu.sw_no_count(10usize, 2usize, 272u32, 2162600u32)?;
    emu.sw_no_count(11usize, 2usize, 276u32, 2162604u32)?;
    emu.sw_no_count(12usize, 2usize, 280u32, 2162608u32)?;
    emu.sw_no_count(13usize, 2usize, 284u32, 2162612u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2162616u32);
    emu.adi_no_count(11usize, 2usize, 256u32, 2162620u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2162624u32);
    emu.apc_no_count(1usize, 2162624u32, 4294946816u32, 2162628u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ffc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 143u32, 2162636u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ffcc));
}
#[inline(always)]
pub fn block_0x0020ffcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2162640u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162644u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162648u32);
    emu.apc_no_count(1usize, 2162648u32, 4294946816u32, 2162652u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162656u32;
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
#[inline]
pub fn block_0x0020ffe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162660u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162664u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162668u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162672u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162676u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162680u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162684u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162688u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162692u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162696u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162700u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162704u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2162708u32);
    emu.sw_no_count(10usize, 2usize, 288u32, 2162712u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162716u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162720u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162724u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2162636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ffcc));
    } else {
        emu.pc = 2162728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210028));
    }
}
#[inline]
pub fn block_0x00210028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 304u32, 2162732u32)?;
    emu.lw_no_count(11usize, 2usize, 308u32, 2162736u32)?;
    emu.lw_no_count(12usize, 2usize, 312u32, 2162740u32)?;
    emu.lw_no_count(13usize, 2usize, 316u32, 2162744u32)?;
    emu.sw_no_count(10usize, 2usize, 240u32, 2162748u32)?;
    emu.sw_no_count(11usize, 2usize, 244u32, 2162752u32)?;
    emu.sw_no_count(12usize, 2usize, 248u32, 2162756u32)?;
    emu.sw_no_count(13usize, 2usize, 252u32, 2162760u32)?;
    emu.lw_no_count(10usize, 2usize, 288u32, 2162764u32)?;
    emu.lw_no_count(11usize, 2usize, 292u32, 2162768u32)?;
    emu.lw_no_count(12usize, 2usize, 296u32, 2162772u32)?;
    emu.lw_no_count(13usize, 2usize, 300u32, 2162776u32)?;
    emu.sw_no_count(10usize, 2usize, 224u32, 2162780u32)?;
    emu.sw_no_count(11usize, 2usize, 228u32, 2162784u32)?;
    emu.sw_no_count(12usize, 2usize, 232u32, 2162788u32)?;
    emu.sw_no_count(13usize, 2usize, 236u32, 2162792u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2162796u32);
    emu.adi_no_count(11usize, 2usize, 224u32, 2162800u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2162804u32);
    emu.apc_no_count(1usize, 2162804u32, 4294946816u32, 2162808u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021007c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 47u32, 2162816u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210080));
}
#[inline(always)]
pub fn block_0x00210080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2162820u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162824u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162828u32);
    emu.apc_no_count(1usize, 2162828u32, 4294946816u32, 2162832u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162840u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162844u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162848u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162852u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162856u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162860u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162864u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162868u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162872u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162876u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162880u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162884u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2162888u32);
    emu.sw_no_count(10usize, 2usize, 288u32, 2162892u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162896u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162900u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162904u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2162816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210080));
    } else {
        emu.pc = 2162908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002100dc));
    }
}
#[inline]
pub fn block_0x002100dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 304u32, 2162912u32)?;
    emu.lw_no_count(11usize, 2usize, 308u32, 2162916u32)?;
    emu.lw_no_count(12usize, 2usize, 312u32, 2162920u32)?;
    emu.lw_no_count(13usize, 2usize, 316u32, 2162924u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2162928u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2162932u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2162936u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2162940u32)?;
    emu.lw_no_count(10usize, 2usize, 288u32, 2162944u32)?;
    emu.lw_no_count(11usize, 2usize, 292u32, 2162948u32)?;
    emu.lw_no_count(12usize, 2usize, 296u32, 2162952u32)?;
    emu.lw_no_count(13usize, 2usize, 300u32, 2162956u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2162960u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2162964u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2162968u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2162972u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2162976u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2162980u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2162984u32);
    emu.apc_no_count(1usize, 2162984u32, 4294946816u32, 2162988u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2162996u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2163000u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2163004u32);
    emu.apc_no_count(1usize, 2163004u32, 4294946816u32, 2163008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2163016u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2163020u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2163024u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2163028u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2163032u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2163036u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2163040u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2163044u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2163048u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2163052u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2163056u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2163060u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2163064u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2163068u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2163072u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2163076u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2163080u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2163084u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2163088u32);
    emu.apc_no_count(1usize, 2163088u32, 4294946816u32, 2163092u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163096u32;
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
#[inline(never)]
pub fn block_0x00210198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2163100u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2163104u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2163108u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2163112u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2163116u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2163120u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2163124u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2163128u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2163132u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2163136u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2163140u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2163144u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2163148u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2163152u32)?;
    emu.sw_no_count(14usize, 2usize, 320u32, 2163156u32)?;
    emu.sw_no_count(15usize, 2usize, 324u32, 2163160u32)?;
    emu.sw_no_count(16usize, 2usize, 328u32, 2163164u32)?;
    emu.sw_no_count(17usize, 2usize, 332u32, 2163168u32)?;
    emu.sw_no_count(10usize, 2usize, 336u32, 2163172u32)?;
    emu.sw_no_count(11usize, 2usize, 340u32, 2163176u32)?;
    emu.sw_no_count(12usize, 2usize, 344u32, 2163180u32)?;
    emu.sw_no_count(13usize, 2usize, 348u32, 2163184u32)?;
    emu.adi_no_count(11usize, 2usize, 320u32, 2163188u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2163192u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2163196u32);
    emu.apc_no_count(1usize, 2163196u32, 4294946816u32, 2163200u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163204u32;
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
pub fn block_0x00210204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 364u32, 2163208u32)?;
    emu.lw_no_count(8usize, 2usize, 360u32, 2163212u32)?;
    emu.lw_no_count(9usize, 2usize, 356u32, 2163216u32)?;
    emu.lw_no_count(18usize, 2usize, 352u32, 2163220u32)?;
    emu.adi_no_count(2usize, 2usize, 368u32, 2163224u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163228u32;
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
pub fn block_0x0021021c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967056u32, 2163232u32);
    emu.sw_no_count(1usize, 2usize, 236u32, 2163236u32)?;
    emu.sw_no_count(8usize, 2usize, 232u32, 2163240u32)?;
    emu.sw_no_count(9usize, 2usize, 228u32, 2163244u32)?;
    emu.sw_no_count(18usize, 2usize, 224u32, 2163248u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2163252u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2163256u32);
    emu.adi_no_count(10usize, 2usize, 192u32, 2163260u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2163264u32);
    emu.apc_no_count(1usize, 2163264u32, 4294946816u32, 2163268u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2163276u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2163280u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2163284u32);
    emu.apc_no_count(1usize, 2163284u32, 4294946816u32, 2163288u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163292u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021025c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2163296u32)?;
    emu.lw_no_count(11usize, 2usize, 0u32, 2163300u32)?;
    emu.lw_no_count(12usize, 2usize, 4u32, 2163304u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2163308u32)?;
    emu.lw_no_count(14usize, 2usize, 24u32, 2163312u32)?;
    emu.lw_no_count(15usize, 2usize, 28u32, 2163316u32)?;
    emu.lw_no_count(16usize, 2usize, 28u32, 2163320u32)?;
    emu.lw_no_count(17usize, 2usize, 16u32, 2163324u32)?;
    emu.lw_no_count(5usize, 2usize, 16u32, 2163328u32)?;
    emu.lw_no_count(6usize, 2usize, 20u32, 2163332u32)?;
    emu.lw_no_count(7usize, 2usize, 20u32, 2163336u32)?;
    emu.lw_no_count(28usize, 2usize, 8u32, 2163340u32)?;
    emu.lw_no_count(29usize, 2usize, 8u32, 2163344u32)?;
    emu.lw_no_count(30usize, 2usize, 12u32, 2163348u32)?;
    emu.lw_no_count(31usize, 2usize, 12u32, 2163352u32)?;
    emu.sw_no_count(17usize, 2usize, 144u32, 2163356u32)?;
    emu.sw_no_count(6usize, 2usize, 148u32, 2163360u32)?;
    emu.sw_no_count(13usize, 2usize, 152u32, 2163364u32)?;
    emu.sw_no_count(15usize, 2usize, 156u32, 2163368u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2163372u32)?;
    emu.sw_no_count(10usize, 2usize, 128u32, 2163376u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2163380u32)?;
    emu.sw_no_count(28usize, 2usize, 136u32, 2163384u32)?;
    emu.sw_no_count(30usize, 2usize, 140u32, 2163388u32)?;
    emu.sw_no_count(5usize, 2usize, 176u32, 2163392u32)?;
    emu.sw_no_count(7usize, 2usize, 180u32, 2163396u32)?;
    emu.sw_no_count(14usize, 2usize, 184u32, 2163400u32)?;
    emu.sw_no_count(16usize, 2usize, 188u32, 2163404u32)?;
    emu.sw_no_count(11usize, 2usize, 160u32, 2163408u32)?;
    emu.sw_no_count(13usize, 2usize, 164u32, 2163412u32)?;
    emu.sw_no_count(29usize, 2usize, 168u32, 2163416u32)?;
    emu.sw_no_count(31usize, 2usize, 172u32, 2163420u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163424u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163428u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163432u32);
    emu.apc_no_count(1usize, 2163432u32, 4294946816u32, 2163436u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002102f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163444u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163448u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163452u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163456u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163460u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163464u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163468u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163472u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163476u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163480u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163484u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163488u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163492u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163496u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163500u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163504u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163508u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163512u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163516u32);
    emu.apc_no_count(1usize, 2163516u32, 4294946816u32, 2163520u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163524u32;
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
#[inline(never)]
pub fn block_0x00210344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163528u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163532u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163536u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163540u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2163544u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2163548u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2163552u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2163556u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163560u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163564u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2163568u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2163572u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163576u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163580u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2163584u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2163588u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2163592u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2163596u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2163600u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2163604u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2163608u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2163612u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2163616u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2163620u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2163624u32);
    emu.apc_no_count(1usize, 2163624u32, 4294946816u32, 2163628u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002103b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2163636u32)?;
    emu.lw_no_count(11usize, 2usize, 32u32, 2163640u32)?;
    emu.lw_no_count(12usize, 2usize, 36u32, 2163644u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2163648u32)?;
    emu.lw_no_count(14usize, 2usize, 56u32, 2163652u32)?;
    emu.lw_no_count(15usize, 2usize, 60u32, 2163656u32)?;
    emu.lw_no_count(16usize, 2usize, 60u32, 2163660u32)?;
    emu.lw_no_count(17usize, 2usize, 48u32, 2163664u32)?;
    emu.lw_no_count(5usize, 2usize, 48u32, 2163668u32)?;
    emu.lw_no_count(6usize, 2usize, 52u32, 2163672u32)?;
    emu.lw_no_count(7usize, 2usize, 52u32, 2163676u32)?;
    emu.lw_no_count(28usize, 2usize, 40u32, 2163680u32)?;
    emu.lw_no_count(29usize, 2usize, 40u32, 2163684u32)?;
    emu.lw_no_count(30usize, 2usize, 44u32, 2163688u32)?;
    emu.lw_no_count(31usize, 2usize, 44u32, 2163692u32)?;
    emu.sw_no_count(17usize, 2usize, 144u32, 2163696u32)?;
    emu.sw_no_count(6usize, 2usize, 148u32, 2163700u32)?;
    emu.sw_no_count(13usize, 2usize, 152u32, 2163704u32)?;
    emu.sw_no_count(15usize, 2usize, 156u32, 2163708u32)?;
    emu.lw_no_count(13usize, 2usize, 36u32, 2163712u32)?;
    emu.sw_no_count(10usize, 2usize, 128u32, 2163716u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2163720u32)?;
    emu.sw_no_count(28usize, 2usize, 136u32, 2163724u32)?;
    emu.sw_no_count(30usize, 2usize, 140u32, 2163728u32)?;
    emu.sw_no_count(5usize, 2usize, 176u32, 2163732u32)?;
    emu.sw_no_count(7usize, 2usize, 180u32, 2163736u32)?;
    emu.sw_no_count(14usize, 2usize, 184u32, 2163740u32)?;
    emu.sw_no_count(16usize, 2usize, 188u32, 2163744u32)?;
    emu.sw_no_count(11usize, 2usize, 160u32, 2163748u32)?;
    emu.sw_no_count(13usize, 2usize, 164u32, 2163752u32)?;
    emu.sw_no_count(29usize, 2usize, 168u32, 2163756u32)?;
    emu.sw_no_count(31usize, 2usize, 172u32, 2163760u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163764u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163768u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163772u32);
    emu.apc_no_count(1usize, 2163772u32, 4294946816u32, 2163776u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163784u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163788u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163792u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163796u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163800u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163804u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163808u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163812u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163816u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163820u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163824u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163828u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163832u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163836u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163840u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163844u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163848u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163852u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163856u32);
    emu.apc_no_count(1usize, 2163856u32, 4294946816u32, 2163860u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163868u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163872u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163876u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163880u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163884u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163888u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163892u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163896u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163900u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163904u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163908u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163912u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2163916u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2163920u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2163924u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2163928u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2163932u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2163936u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2163940u32);
    emu.apc_no_count(1usize, 2163940u32, 4294946816u32, 2163944u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163948u32;
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
pub fn block_0x002104ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2163952u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2163956u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2163960u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2163964u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2163968u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2163972u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2163976u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2163980u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2163984u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2163988u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2163992u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2163996u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164000u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164004u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164008u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164012u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164016u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164020u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164024u32);
    emu.apc_no_count(1usize, 2164024u32, 4294946816u32, 2164028u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(48u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00210540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164036u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164040u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164044u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164048u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2164052u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2164056u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2164060u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2164064u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164068u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164072u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2164076u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2164080u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164084u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164088u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2164092u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2164096u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2164100u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2164104u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2164108u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2164112u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2164116u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2164120u32)?;
    emu.adi_no_count(10usize, 2usize, 64u32, 2164124u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2164128u32);
    emu.adi_no_count(12usize, 2usize, 192u32, 2164132u32);
    emu.apc_no_count(1usize, 2164132u32, 4294946816u32, 2164136u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002105ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 80u32, 2164144u32)?;
    emu.lw_no_count(11usize, 2usize, 84u32, 2164148u32)?;
    emu.lw_no_count(12usize, 2usize, 88u32, 2164152u32)?;
    emu.lw_no_count(13usize, 2usize, 92u32, 2164156u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164160u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164164u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164168u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164172u32)?;
    emu.lw_no_count(10usize, 2usize, 64u32, 2164176u32)?;
    emu.lw_no_count(11usize, 2usize, 68u32, 2164180u32)?;
    emu.lw_no_count(12usize, 2usize, 72u32, 2164184u32)?;
    emu.lw_no_count(13usize, 2usize, 76u32, 2164188u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164192u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164196u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164200u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164204u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164208u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164212u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164216u32);
    emu.apc_no_count(1usize, 2164216u32, 4294946816u32, 2164220u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164228u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164232u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164236u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164240u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164244u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164248u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164252u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164256u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164260u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164264u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164268u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164272u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164276u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164280u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164284u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164288u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164292u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164296u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164300u32);
    emu.apc_no_count(1usize, 2164300u32, 4294946816u32, 2164304u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164312u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164316u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164320u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164324u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164328u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164332u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164336u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164340u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164344u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164348u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164352u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164356u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164360u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164364u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164368u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164372u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164376u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164380u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164384u32);
    emu.apc_no_count(1usize, 2164384u32, 4294946816u32, 2164388u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002106a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164396u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164400u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164404u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164408u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164412u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164416u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164420u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164424u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164428u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164432u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164436u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164440u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164444u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164448u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164452u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164456u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164460u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164464u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164468u32);
    emu.apc_no_count(1usize, 2164468u32, 4294946816u32, 2164472u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002106fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164480u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164484u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164488u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164492u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164496u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164500u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164504u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164508u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164512u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164516u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164520u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164524u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164528u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164532u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164536u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164540u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164544u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164548u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164552u32);
    emu.apc_no_count(1usize, 2164552u32, 4294946816u32, 2164556u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164564u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164568u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164572u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164576u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164580u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164584u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164588u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164592u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164596u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164600u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164604u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164608u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164612u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164616u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164620u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164624u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164628u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164632u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164636u32);
    emu.apc_no_count(1usize, 2164636u32, 4294946816u32, 2164640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002107a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164648u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164652u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164656u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164660u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164664u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164668u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164672u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164676u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164680u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164684u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164688u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164692u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164696u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164700u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164704u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164708u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164712u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164716u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164720u32);
    emu.apc_no_count(1usize, 2164720u32, 4294946816u32, 2164724u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002107f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164732u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164736u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164740u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164744u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164748u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164752u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164756u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164760u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2164764u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2164768u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2164772u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2164776u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164780u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164784u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164788u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164792u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164796u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164800u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164804u32);
    emu.apc_no_count(1usize, 2164804u32, 4294946816u32, 2164808u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021084c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2164816u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2164820u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2164824u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2164828u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2164832u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2164836u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2164840u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2164844u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164848u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164852u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2164856u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2164860u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164864u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164868u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2164872u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2164876u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2164880u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2164884u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2164888u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2164892u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2164896u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2164900u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2164904u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2164908u32);
    emu.adi_no_count(12usize, 2usize, 64u32, 2164912u32);
    emu.apc_no_count(1usize, 2164912u32, 4294946816u32, 2164916u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002108b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 112u32, 2164924u32)?;
    emu.lw_no_count(11usize, 2usize, 116u32, 2164928u32)?;
    emu.lw_no_count(12usize, 2usize, 120u32, 2164932u32)?;
    emu.lw_no_count(13usize, 2usize, 124u32, 2164936u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2164940u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2164944u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2164948u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2164952u32)?;
    emu.lw_no_count(10usize, 2usize, 96u32, 2164956u32)?;
    emu.lw_no_count(11usize, 2usize, 100u32, 2164960u32)?;
    emu.lw_no_count(12usize, 2usize, 104u32, 2164964u32)?;
    emu.lw_no_count(13usize, 2usize, 108u32, 2164968u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2164972u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2164976u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2164980u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2164984u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2164988u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2164992u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2164996u32);
    emu.apc_no_count(1usize, 2164996u32, 4294946816u32, 2165000u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021090c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165008u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165012u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165016u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165020u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165024u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165028u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165032u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165036u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165040u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165044u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165048u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165052u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165056u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165060u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165064u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165068u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165072u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165076u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165080u32);
    emu.apc_no_count(1usize, 2165080u32, 4294946816u32, 2165084u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165092u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165096u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165100u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165104u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165108u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165112u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165116u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165120u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165124u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165128u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165132u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165136u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165140u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165144u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165148u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165152u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165156u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165160u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165164u32);
    emu.apc_no_count(1usize, 2165164u32, 4294946816u32, 2165168u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002109b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165176u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165180u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165184u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165188u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165192u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165196u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165200u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165204u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165208u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165212u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165216u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165220u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165224u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165228u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165232u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165236u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165240u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165244u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165248u32);
    emu.apc_no_count(1usize, 2165248u32, 4294946816u32, 2165252u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165256u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165260u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165264u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165268u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165272u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165276u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165280u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165284u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165288u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165292u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165296u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165300u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165304u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165308u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165312u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165316u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165320u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165324u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165328u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165332u32);
    emu.apc_no_count(1usize, 2165332u32, 4294946816u32, 2165336u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165344u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165348u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165352u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165356u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165360u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165364u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165368u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165372u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165376u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165380u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165384u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165388u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165392u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165396u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165400u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165404u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165408u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165412u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165416u32);
    emu.apc_no_count(1usize, 2165416u32, 4294946816u32, 2165420u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165424u32;
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
#[inline]
pub fn block_0x00210ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165428u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165432u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165436u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165440u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165444u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165448u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165452u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165456u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165460u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165464u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165468u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165472u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165476u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165480u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165484u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165488u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165492u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165496u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165500u32);
    emu.apc_no_count(1usize, 2165500u32, 4294946816u32, 2165504u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165512u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165516u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165520u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165524u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165528u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165532u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165536u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165540u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165544u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165548u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165552u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165556u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165560u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165564u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165568u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165572u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165576u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165580u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165584u32);
    emu.apc_no_count(1usize, 2165584u32, 4294946816u32, 2165588u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165596u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165600u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165604u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165608u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165612u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165616u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165620u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165624u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165628u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165632u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165636u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165640u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165644u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165648u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165652u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165656u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165660u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165664u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165668u32);
    emu.apc_no_count(1usize, 2165668u32, 4294946816u32, 2165672u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165680u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165684u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165688u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165692u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165696u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165700u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165704u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165708u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165712u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165716u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165720u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165724u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165728u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165732u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165736u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165740u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165744u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165748u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165752u32);
    emu.apc_no_count(1usize, 2165752u32, 4294946816u32, 2165756u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165764u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165768u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165772u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165776u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165780u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165784u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165788u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165792u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165796u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165800u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165804u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165808u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165812u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165816u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165820u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165824u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165828u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165832u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165836u32);
    emu.apc_no_count(1usize, 2165836u32, 4294946816u32, 2165840u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165848u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165852u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165856u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165860u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165864u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165868u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165872u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165876u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165880u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165884u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165888u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165892u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165896u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165900u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165904u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165908u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165912u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2165916u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2165920u32);
    emu.apc_no_count(1usize, 2165920u32, 4294946816u32, 2165924u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210ca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2165932u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2165936u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2165940u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2165944u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2165948u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2165952u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2165956u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2165960u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2165964u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2165968u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2165972u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2165976u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2165980u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2165984u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2165988u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2165992u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2165996u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166000u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166004u32);
    emu.apc_no_count(1usize, 2166004u32, 4294946816u32, 2166008u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166016u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166020u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166024u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166028u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166032u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166036u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166040u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166044u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166048u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166052u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166056u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166060u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166064u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166068u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166072u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166076u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166080u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166084u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166088u32);
    emu.apc_no_count(1usize, 2166088u32, 4294946816u32, 2166092u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166100u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166104u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166108u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166112u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166116u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166120u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166124u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166128u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166132u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166136u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166140u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166144u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166148u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166152u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166156u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166160u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166164u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166168u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166172u32);
    emu.apc_no_count(1usize, 2166172u32, 4294942720u32, 2166176u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166184u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166188u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166192u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166196u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166200u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166204u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166208u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166212u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166216u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166220u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166224u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166228u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166232u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166236u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166240u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166244u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166248u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166252u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166256u32);
    emu.apc_no_count(1usize, 2166256u32, 4294942720u32, 2166260u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166264u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00210df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166268u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166272u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166276u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166280u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2166284u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2166288u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2166292u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2166296u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166300u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166304u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2166308u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2166312u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166316u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166320u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2166324u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2166328u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2166332u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2166336u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2166340u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2166344u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2166348u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2166352u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2166356u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2166360u32);
    emu.adi_no_count(12usize, 2usize, 96u32, 2166364u32);
    emu.apc_no_count(1usize, 2166364u32, 4294942720u32, 2166368u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2166376u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166380u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166384u32);
    emu.apc_no_count(1usize, 2166384u32, 4294942720u32, 2166388u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166396u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166400u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166404u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166408u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166412u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166416u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166420u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166424u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166428u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166432u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166436u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166440u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166444u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166448u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166452u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166456u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166460u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166464u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166468u32);
    emu.apc_no_count(1usize, 2166468u32, 4294942720u32, 2166472u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166480u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166484u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166488u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166492u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166496u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166500u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166504u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166508u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166512u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166516u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166520u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166524u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166528u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166532u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166536u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166540u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166544u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166548u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166552u32);
    emu.apc_no_count(1usize, 2166552u32, 4294942720u32, 2166556u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166564u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166568u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166572u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166576u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166580u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166584u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166588u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166592u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166596u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166600u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166604u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166608u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166612u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166616u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166620u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166624u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166628u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166632u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166636u32);
    emu.apc_no_count(1usize, 2166636u32, 4294942720u32, 2166640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210f74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166648u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166652u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166656u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166660u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166664u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166668u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166672u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166676u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166680u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166684u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166688u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166692u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166696u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166700u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166704u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166708u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166712u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166716u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166720u32);
    emu.apc_no_count(1usize, 2166720u32, 4294942720u32, 2166724u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166732u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166736u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166740u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166744u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166748u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166752u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166756u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166760u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166764u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166768u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166772u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166776u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166780u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166784u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166788u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166792u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166796u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166800u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166804u32);
    emu.apc_no_count(1usize, 2166804u32, 4294942720u32, 2166808u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021101c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166816u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166820u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166824u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166828u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166832u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166836u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166840u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166844u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166848u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166852u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166856u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166860u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166864u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166868u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166872u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166876u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166880u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166884u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166888u32);
    emu.apc_no_count(1usize, 2166888u32, 4294942720u32, 2166892u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166900u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166904u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166908u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166912u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2166916u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2166920u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2166924u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2166928u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2166932u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2166936u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2166940u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2166944u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2166948u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2166952u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2166956u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2166960u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2166964u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2166968u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2166972u32);
    emu.apc_no_count(1usize, 2166972u32, 4294942720u32, 2166976u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166980u32;
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
#[inline]
pub fn block_0x002110c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2166984u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2166988u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2166992u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2166996u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167000u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167004u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167008u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167012u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167016u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167020u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167024u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167028u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167032u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167036u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167040u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167044u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167048u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167052u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167056u32);
    emu.apc_no_count(1usize, 2167056u32, 4294942720u32, 2167060u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167064u32;
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
#[inline]
pub fn block_0x00211118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167068u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167072u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167076u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167080u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167084u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167088u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167092u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167096u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167100u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167104u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167108u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167112u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167116u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167120u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167124u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167128u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167132u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167136u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167140u32);
    emu.apc_no_count(1usize, 2167140u32, 4294942720u32, 2167144u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167148u32;
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
#[inline]
pub fn block_0x0021116c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167152u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167156u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167160u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167164u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167168u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167172u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167176u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167180u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167184u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167188u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167192u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167196u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167200u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167204u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167208u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167212u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167216u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167220u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167224u32);
    emu.apc_no_count(1usize, 2167224u32, 4294942720u32, 2167228u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002111c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167236u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167240u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167244u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167248u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167252u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167256u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167260u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167264u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167268u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167272u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167276u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167280u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167284u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167288u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167292u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167296u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167300u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167304u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167308u32);
    emu.apc_no_count(1usize, 2167308u32, 4294942720u32, 2167312u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167320u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167324u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167328u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167332u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167336u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167340u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167344u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167348u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167352u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167356u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167360u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167364u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167368u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167372u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167376u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167380u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167384u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167388u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167392u32);
    emu.apc_no_count(1usize, 2167392u32, 4294942720u32, 2167396u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167404u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167408u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167412u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167416u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167420u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167424u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167428u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167432u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167436u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167440u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167444u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167448u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167452u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167456u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167460u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167464u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167468u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167472u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167476u32);
    emu.apc_no_count(1usize, 2167476u32, 4294942720u32, 2167480u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002112bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167488u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167492u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167496u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167500u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167504u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167508u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167512u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167516u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167520u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167524u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167528u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167532u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167536u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167540u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167544u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167548u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167552u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167556u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167560u32);
    emu.apc_no_count(1usize, 2167560u32, 4294942720u32, 2167564u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167568u32;
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
#[inline]
pub fn block_0x00211310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167572u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167576u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167580u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167584u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167588u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167592u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167596u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167600u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167604u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167608u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167612u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167616u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167620u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167624u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167628u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167632u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167636u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167640u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167644u32);
    emu.apc_no_count(1usize, 2167644u32, 4294942720u32, 2167648u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167656u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167660u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167664u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167668u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167672u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167676u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167680u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167684u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167688u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167692u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167696u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167700u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167704u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167708u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167712u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167716u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167720u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167724u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167728u32);
    emu.apc_no_count(1usize, 2167728u32, 4294942720u32, 2167732u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002113b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167740u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167744u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167748u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167752u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167756u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167760u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167764u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167768u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167772u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167776u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167780u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167784u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167788u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167792u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167796u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167800u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167804u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167808u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167812u32);
    emu.apc_no_count(1usize, 2167812u32, 4294942720u32, 2167816u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167820u32;
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
#[inline]
pub fn block_0x0021140c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167824u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167828u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167832u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167836u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167840u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167844u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167848u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167852u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167856u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167860u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167864u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167868u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167872u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167876u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167880u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167884u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167888u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167892u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167896u32);
    emu.apc_no_count(1usize, 2167896u32, 4294942720u32, 2167900u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167904u32;
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
#[inline]
pub fn block_0x00211460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167908u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167912u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2167916u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2167920u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2167924u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2167928u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2167932u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2167936u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2167940u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2167944u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2167948u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2167952u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2167956u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2167960u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2167964u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2167968u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2167972u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2167976u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2167980u32);
    emu.apc_no_count(1usize, 2167980u32, 4294942720u32, 2167984u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167988u32;
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
#[inline]
pub fn block_0x002114b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2167992u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2167996u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168000u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168004u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168008u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168012u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168016u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168020u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168024u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168028u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168032u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168036u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168040u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168044u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168048u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168052u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168056u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168060u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168064u32);
    emu.apc_no_count(1usize, 2168064u32, 4294942720u32, 2168068u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168072u32;
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
#[inline]
pub fn block_0x00211508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168076u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168080u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168084u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168088u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168092u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168096u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168100u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168104u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168108u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168112u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168116u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168120u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168124u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168128u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168132u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168136u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168140u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168144u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168148u32);
    emu.apc_no_count(1usize, 2168148u32, 4294942720u32, 2168152u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(20u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021155c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168160u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168164u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168168u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168172u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168176u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168180u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168184u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168188u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168192u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168196u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168200u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168204u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168208u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168212u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168216u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168220u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168224u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168228u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168232u32);
    emu.apc_no_count(1usize, 2168232u32, 4294942720u32, 2168236u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168240u32;
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
pub fn block_0x002115b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168244u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168248u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168252u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168256u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168260u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168264u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168268u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168272u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168276u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168280u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168284u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168288u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168292u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168296u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168300u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168304u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168308u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168312u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168316u32);
    emu.apc_no_count(1usize, 2168316u32, 4294942720u32, 2168320u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168328u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168332u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168336u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168340u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168344u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168348u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168352u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168356u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168360u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168364u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168368u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168372u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168376u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168380u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168384u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168388u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168392u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168396u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168400u32);
    emu.apc_no_count(1usize, 2168400u32, 4294942720u32, 2168404u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168408u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168412u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168416u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168420u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168424u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168428u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168432u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168436u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168440u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168444u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168448u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168452u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168456u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168460u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168464u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168468u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168472u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168476u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168480u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168484u32);
    emu.apc_no_count(1usize, 2168484u32, 4294942720u32, 2168488u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002116ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168496u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168500u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168504u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168508u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168512u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168516u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168520u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168524u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168528u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168532u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168536u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168540u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168544u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168548u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168552u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168556u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168560u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168564u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168568u32);
    emu.apc_no_count(1usize, 2168568u32, 4294942720u32, 2168572u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168580u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168584u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168588u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168592u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168596u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168600u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168604u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168608u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168612u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168616u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168620u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168624u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168628u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168632u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168636u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168640u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168644u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168648u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168652u32);
    emu.apc_no_count(1usize, 2168652u32, 4294942720u32, 2168656u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168664u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168668u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168672u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168676u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168680u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168684u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168688u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168692u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168696u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168700u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168704u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168708u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168712u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168716u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168720u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168724u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168728u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168732u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168736u32);
    emu.apc_no_count(1usize, 2168736u32, 4294942720u32, 2168740u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168744u32;
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
#[inline]
pub fn block_0x002117a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168748u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168752u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168756u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168760u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168764u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168768u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168772u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168776u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168780u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168784u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168788u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168792u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168796u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168800u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168804u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168808u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168812u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168816u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168820u32);
    emu.apc_no_count(1usize, 2168820u32, 4294942720u32, 2168824u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002117fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168832u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168836u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168840u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168844u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168848u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168852u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168856u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168860u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168864u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168868u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168872u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168876u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168880u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168884u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168888u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168892u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168896u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168900u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168904u32);
    emu.apc_no_count(1usize, 2168904u32, 4294942720u32, 2168908u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168912u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00211850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2168916u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2168920u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2168924u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2168928u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2168932u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2168936u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2168940u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2168944u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2168948u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2168952u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2168956u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2168960u32)?;
    emu.sw_no_count(10usize, 2usize, 160u32, 2168964u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2168968u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2168972u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2168976u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2168980u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2168984u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2168988u32);
    emu.apc_no_count(1usize, 2168988u32, 4294942720u32, 2168992u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002118a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2169000u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2169004u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2169008u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2169012u32)?;
    emu.lw_no_count(14usize, 2usize, 192u32, 2169016u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2169020u32)?;
    emu.lw_no_count(16usize, 2usize, 200u32, 2169024u32)?;
    emu.lw_no_count(17usize, 2usize, 204u32, 2169028u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2169032u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2169036u32)?;
    emu.sw_no_count(14usize, 2usize, 160u32, 2169040u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2169044u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2169048u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2169052u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2169056u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2169060u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2169064u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2169068u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2169072u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2169076u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2169080u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2169084u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2169088u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2169092u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2169096u32);
    emu.apc_no_count(1usize, 2169096u32, 4294942720u32, 2169100u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 96u32, 2169108u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211914));
}
#[inline(always)]
pub fn block_0x00211914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2169112u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2169116u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2169120u32);
    emu.apc_no_count(1usize, 2169120u32, 4294942720u32, 2169124u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169128u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2169132u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2169136u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2169140u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2169144u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2169148u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2169152u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2169156u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2169160u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2169164u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2169168u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2169172u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2169176u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2169180u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2169184u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2169188u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2169192u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2169196u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2169108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211914));
    } else {
        emu.pc = 2169200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211970));
    }
}
#[inline]
pub fn block_0x00211970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 176u32, 2169204u32)?;
    emu.lw_no_count(11usize, 2usize, 180u32, 2169208u32)?;
    emu.lw_no_count(12usize, 2usize, 184u32, 2169212u32)?;
    emu.lw_no_count(13usize, 2usize, 188u32, 2169216u32)?;
    emu.sw_no_count(10usize, 2usize, 208u32, 2169220u32)?;
    emu.sw_no_count(11usize, 2usize, 212u32, 2169224u32)?;
    emu.sw_no_count(12usize, 2usize, 216u32, 2169228u32)?;
    emu.sw_no_count(13usize, 2usize, 220u32, 2169232u32)?;
    emu.lw_no_count(10usize, 2usize, 160u32, 2169236u32)?;
    emu.lw_no_count(11usize, 2usize, 164u32, 2169240u32)?;
    emu.lw_no_count(12usize, 2usize, 168u32, 2169244u32)?;
    emu.lw_no_count(13usize, 2usize, 172u32, 2169248u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2169252u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2169256u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2169260u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2169264u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2169268u32);
    emu.adi_no_count(11usize, 2usize, 192u32, 2169272u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2169276u32);
    emu.apc_no_count(1usize, 2169276u32, 4294942720u32, 2169280u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002119c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 94u32, 2169288u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002119c8));
}
#[inline(always)]
pub fn block_0x002119c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2169292u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2169296u32);
    emu.adi_no_count(12usize, 2usize, 160u32, 2169300u32);
    emu.apc_no_count(1usize, 2169300u32, 4294942720u32, 2169304u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002119dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 208u32, 2169312u32)?;
    emu.lw_no_count(11usize, 2usize, 212u32, 2169316u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2169320u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2169324u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2169328u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2169332u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2169336u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2169340u32)?;
    emu.lw_no_count(10usize, 2usize, 192u32, 2169344u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2169348u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2169352u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2169356u32)?;
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2169360u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2169364u32)?;
    emu.sw_no_count(11usize, 2usize, 164u32, 2169368u32)?;
    emu.sw_no_count(12usize, 2usize, 168u32, 2169372u32)?;
    emu.sw_no_count(13usize, 2usize, 172u32, 2169376u32)?;
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2169288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002119c8));
    } else {
        emu.pc = 2169380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a24));
    }
}
#[inline(never)]
pub fn block_0x00211a24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 176u32, 2169384u32)?;
    emu.lw_no_count(11usize, 2usize, 180u32, 2169388u32)?;
    emu.lw_no_count(12usize, 2usize, 184u32, 2169392u32)?;
    emu.lw_no_count(13usize, 2usize, 188u32, 2169396u32)?;
    emu.lw_no_count(14usize, 2usize, 160u32, 2169400u32)?;
    emu.lw_no_count(15usize, 2usize, 164u32, 2169404u32)?;
    emu.lw_no_count(16usize, 2usize, 168u32, 2169408u32)?;
    emu.lw_no_count(17usize, 2usize, 172u32, 2169412u32)?;
    emu.sw_no_count(10usize, 2usize, 144u32, 2169416u32)?;
    emu.sw_no_count(11usize, 2usize, 148u32, 2169420u32)?;
    emu.sw_no_count(12usize, 2usize, 152u32, 2169424u32)?;
    emu.sw_no_count(13usize, 2usize, 156u32, 2169428u32)?;
    emu.sw_no_count(14usize, 2usize, 128u32, 2169432u32)?;
    emu.sw_no_count(15usize, 2usize, 132u32, 2169436u32)?;
    emu.sw_no_count(16usize, 2usize, 136u32, 2169440u32)?;
    emu.sw_no_count(17usize, 2usize, 140u32, 2169444u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2169448u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2169452u32)?;
    emu.sw_no_count(12usize, 8usize, 24u32, 2169456u32)?;
    emu.sw_no_count(13usize, 8usize, 28u32, 2169460u32)?;
    emu.sw_no_count(14usize, 8usize, 0u32, 2169464u32)?;
    emu.sw_no_count(15usize, 8usize, 4u32, 2169468u32)?;
    emu.sw_no_count(16usize, 8usize, 8u32, 2169472u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2169476u32)?;
    emu.adi_no_count(10usize, 2usize, 192u32, 2169480u32);
    emu.adi_no_count(11usize, 2usize, 128u32, 2169484u32);
    emu.adi_no_count(12usize, 2usize, 128u32, 2169488u32);
    emu.apc_no_count(1usize, 2169488u32, 4294942720u32, 2169492u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169496u32;
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
#[inline(never)]
pub fn block_0x00211a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2169500u32)?;
    emu.lw_no_count(11usize, 2usize, 196u32, 2169504u32)?;
    emu.lw_no_count(12usize, 2usize, 200u32, 2169508u32)?;
    emu.lw_no_count(13usize, 2usize, 204u32, 2169512u32)?;
    emu.lw_no_count(14usize, 2usize, 208u32, 2169516u32)?;
    emu.lw_no_count(15usize, 2usize, 212u32, 2169520u32)?;
    emu.lw_no_count(16usize, 2usize, 216u32, 2169524u32)?;
    emu.lw_no_count(17usize, 2usize, 220u32, 2169528u32)?;
    emu.lw_no_count(5usize, 9usize, 0u32, 2169532u32)?;
    emu.lw_no_count(6usize, 9usize, 4u32, 2169536u32)?;
    emu.lw_no_count(7usize, 9usize, 8u32, 2169540u32)?;
    emu.lw_no_count(28usize, 9usize, 12u32, 2169544u32)?;
    emu.lw_no_count(29usize, 9usize, 16u32, 2169548u32)?;
    emu.lw_no_count(30usize, 9usize, 20u32, 2169552u32)?;
    emu.lw_no_count(31usize, 9usize, 24u32, 2169556u32)?;
    emu.lw_no_count(9usize, 9usize, 28u32, 2169560u32)?;
    emu.xrr_no_count(11usize, 6usize, 11usize, 2169564u32);
    emu.xrr_no_count(10usize, 5usize, 10usize, 2169568u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2169572u32);
    emu.xrr_no_count(13usize, 28usize, 13usize, 2169576u32);
    emu.xrr_no_count(14usize, 29usize, 14usize, 2169580u32);
    emu.xrr_no_count(15usize, 30usize, 15usize, 2169584u32);
    emu.xrr_no_count(16usize, 31usize, 16usize, 2169588u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2169592u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2169596u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2169600u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2169604u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2169608u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2169612u32);
    emu.xrr_no_count(11usize, 9usize, 17usize, 2169616u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2169620u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2169624u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2169628u32);
    emu.apc_no_count(1usize, 2169628u32, 24576u32, 2169632u32);
    emu.add_memory_rw_events(35usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 32u32, 2169640u32);
    emu.lw_no_count(1usize, 2usize, 236u32, 2169644u32)?;
    emu.lw_no_count(8usize, 2usize, 232u32, 2169648u32)?;
    emu.lw_no_count(9usize, 2usize, 228u32, 2169652u32)?;
    emu.lw_no_count(18usize, 2usize, 224u32, 2169656u32)?;
    emu.adi_no_count(2usize, 2usize, 240u32, 2169660u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169664u32;
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
pub fn block_0x00211b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 139u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2169668u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2169672u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2169676u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2169680u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2169684u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2169688u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2169692u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2169696u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2169700u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2169704u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2169708u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2169712u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2169716u32);
    emu.lbu_no_count(10usize, 11usize, 0u32, 2169720u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2169724u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2169728u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2169732u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2169736u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2169740u32);
    emu.lbu_no_count(17usize, 11usize, 6u32, 2169744u32);
    emu.lbu_no_count(5usize, 11usize, 7u32, 2169748u32);
    emu.lbu_no_count(6usize, 11usize, 8u32, 2169752u32);
    emu.lbu_no_count(7usize, 11usize, 9u32, 2169756u32);
    emu.lbu_no_count(28usize, 11usize, 10u32, 2169760u32);
    emu.lbu_no_count(29usize, 11usize, 11u32, 2169764u32);
    emu.lbu_no_count(30usize, 11usize, 12u32, 2169768u32);
    emu.lbu_no_count(31usize, 11usize, 13u32, 2169772u32);
    emu.lbu_no_count(9usize, 11usize, 14u32, 2169776u32);
    emu.lbu_no_count(18usize, 11usize, 15u32, 2169780u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2169784u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2169788u32);
    emu.sli_no_count(19usize, 10usize, 24u32, 2169792u32);
    emu.sli_no_count(17usize, 17usize, 8u32, 2169796u32);
    emu.orr_no_count(10usize, 13usize, 14usize, 2169800u32);
    emu.orr_no_count(12usize, 19usize, 12usize, 2169804u32);
    emu.orr_no_count(13usize, 17usize, 5usize, 2169808u32);
    emu.lbu_no_count(14usize, 11usize, 16u32, 2169812u32);
    emu.lbu_no_count(17usize, 11usize, 17u32, 2169816u32);
    emu.lbu_no_count(5usize, 11usize, 18u32, 2169820u32);
    emu.lbu_no_count(19usize, 11usize, 19u32, 2169824u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2169828u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2169832u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2169836u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2169840u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2169844u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2169848u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2169852u32);
    emu.orr_no_count(16usize, 28usize, 29usize, 2169856u32);
    emu.orr_no_count(6usize, 6usize, 7usize, 2169860u32);
    emu.orr_no_count(7usize, 9usize, 18usize, 2169864u32);
    emu.lbu_no_count(28usize, 11usize, 20u32, 2169868u32);
    emu.lbu_no_count(29usize, 11usize, 21u32, 2169872u32);
    emu.lbu_no_count(9usize, 11usize, 22u32, 2169876u32);
    emu.lbu_no_count(18usize, 11usize, 23u32, 2169880u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2169884u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2169888u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2169892u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2169896u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2169900u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2169904u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2169908u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2169912u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2169916u32);
    emu.orr_no_count(17usize, 9usize, 18usize, 2169920u32);
    emu.lbu_no_count(31usize, 11usize, 24u32, 2169924u32);
    emu.lbu_no_count(9usize, 11usize, 25u32, 2169928u32);
    emu.lbu_no_count(18usize, 11usize, 26u32, 2169932u32);
    emu.lbu_no_count(19usize, 11usize, 27u32, 2169936u32);
    emu.sli_no_count(29usize, 29usize, 16u32, 2169940u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2169944u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2169948u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2169952u32);
    emu.orr_no_count(29usize, 18usize, 19usize, 2169956u32);
    emu.lbu_no_count(18usize, 11usize, 30u32, 2169960u32);
    emu.lbu_no_count(19usize, 11usize, 31u32, 2169964u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2169968u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2169972u32);
    emu.orr_no_count(31usize, 31usize, 9usize, 2169976u32);
    emu.lbu_no_count(9usize, 11usize, 29u32, 2169980u32);
    emu.lbu_no_count(11usize, 11usize, 28u32, 2169984u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2169988u32);
    emu.orr_no_count(24usize, 18usize, 19usize, 2169992u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2169996u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2170000u32);
    emu.orr_no_count(11usize, 11usize, 9usize, 2170004u32);
    emu.orr_no_count(18usize, 12usize, 10usize, 2170008u32);
    emu.orr_no_count(19usize, 15usize, 13usize, 2170012u32);
    emu.orr_no_count(20usize, 6usize, 16usize, 2170016u32);
    emu.orr_no_count(21usize, 30usize, 7usize, 2170020u32);
    emu.orr_no_count(22usize, 14usize, 5usize, 2170024u32);
    emu.orr_no_count(23usize, 28usize, 17usize, 2170028u32);
    emu.orr_no_count(25usize, 31usize, 29usize, 2170032u32);
    emu.orr_no_count(24usize, 11usize, 24usize, 2170036u32);
    emu.adi_no_count(10usize, 24usize, 1u32, 2170040u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2170044u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2170048u32);
    emu.adr_no_count(11usize, 25usize, 10usize, 2170052u32);
    emu.sltru_no_count(12usize, 11usize, 25usize, 2170056u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2170060u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2170064u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2170068u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2170072u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2170076u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2170080u32);
    emu.adr_no_count(11usize, 23usize, 10usize, 2170084u32);
    emu.sltru_no_count(12usize, 11usize, 23usize, 2170088u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2170092u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2170096u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2170100u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2170104u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2170108u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2170112u32);
    emu.adr_no_count(11usize, 10usize, 22usize, 2170116u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2170120u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2170124u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2170128u32);
    emu.adr_no_count(11usize, 10usize, 21usize, 2170132u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2170136u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2170140u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2170144u32);
    emu.adr_no_count(11usize, 10usize, 20usize, 2170148u32);
    emu.sltru_no_count(11usize, 11usize, 10usize, 2170152u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2170156u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2170160u32);
    emu.adr_no_count(11usize, 19usize, 10usize, 2170164u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2170168u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2170172u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2170176u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2170180u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2170184u32);
    emu.adr_no_count(11usize, 18usize, 10usize, 2170188u32);
    emu.sltru_no_count(12usize, 11usize, 18usize, 2170192u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2170196u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2170200u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2170204u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2170208u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2170212u32);
    emu.apc_no_count(1usize, 2170212u32, 24576u32, 2170216u32);
    emu.add_memory_rw_events(139usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2170224u32);
    emu.sw_no_count(24usize, 2usize, 4u32, 2170228u32)?;
    emu.sw_no_count(25usize, 2usize, 8u32, 2170232u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2170236u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2170240u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2170244u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2170248u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2170252u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2170256u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2170260u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 60u32, 2170264u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2170268u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2170272u32);
    emu.apc_no_count(1usize, 2170272u32, 4294938624u32, 2170276u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211da8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 32u32, 2170284u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2170288u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2170292u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2170296u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2170300u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2170304u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2170308u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2170312u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2170316u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2170320u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2170324u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2170328u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2170332u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170336u32;
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
pub fn block_0x00211de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 11usize, 0u32, 2170340u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2170344u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2170348u32)?;
    emu.lw_no_count(12usize, 11usize, 16u32, 2170352u32)?;
    emu.lw_no_count(13usize, 11usize, 20u32, 2170356u32)?;
    emu.lw_no_count(7usize, 11usize, 24u32, 2170360u32)?;
    emu.lw_no_count(6usize, 11usize, 28u32, 2170364u32)?;
    emu.adr_no_count(14usize, 16usize, 14usize, 2170368u32);
    emu.sltru_no_count(15usize, 14usize, 16usize, 2170372u32);
    emu.adr_no_count(17usize, 5usize, 15usize, 2170376u32);
    emu.sltru_no_count(15usize, 17usize, 5usize, 2170380u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2170384u32);
    emu.sltru_no_count(17usize, 12usize, 17usize, 2170388u32);
    emu.adr_no_count(13usize, 15usize, 13usize, 2170392u32);
    emu.adr_no_count(13usize, 13usize, 17usize, 2170396u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2170404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e24));
    } else {
        emu.pc = 2170400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e20));
    }
}
#[inline(always)]
pub fn block_0x00211e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 13usize, 15usize, 2170404u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211e24));
}
#[inline]
pub fn block_0x00211e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 8u32, 2170408u32)?;
    emu.sbr_no_count(28usize, 0usize, 16usize, 2170412u32);
    emu.sbr_no_count(29usize, 5usize, 16usize, 2170416u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2170420u32);
    emu.sltru_no_count(28usize, 29usize, 28usize, 2170424u32);
    emu.mulhu_no_count(30usize, 16usize, 11usize, 2170428u32);
    emu.sbr_no_count(30usize, 30usize, 5usize, 2170432u32);
    emu.adr_no_count(28usize, 30usize, 28usize, 2170436u32);
    emu.sbr_no_count(30usize, 0usize, 5usize, 2170440u32);
    emu.adr_no_count(7usize, 16usize, 7usize, 2170444u32);
    emu.mulhu_no_count(31usize, 5usize, 11usize, 2170448u32);
    emu.adr_no_count(5usize, 29usize, 6usize, 2170452u32);
    emu.sltru_no_count(16usize, 7usize, 16usize, 2170456u32);
    emu.sltru_no_count(6usize, 28usize, 30usize, 2170460u32);
    emu.adr_no_count(5usize, 5usize, 16usize, 2170464u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2170468u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2170476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e6c));
    } else {
        emu.pc = 2170472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211e68));
    }
}
#[inline(always)]
pub fn block_0x00211e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 5usize, 29usize, 2170476u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211e6c));
}
#[inline]
pub fn block_0x00211e6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 28usize, 16usize, 2170480u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2170484u32);
    emu.adr_no_count(16usize, 13usize, 15usize, 2170488u32);
    emu.sltru_no_count(28usize, 30usize, 28usize, 2170492u32);
    emu.sltru_no_count(7usize, 17usize, 7usize, 2170496u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2170500u32);
    emu.adr_no_count(28usize, 6usize, 28usize, 2170504u32);
    emu.adr_no_count(6usize, 5usize, 7usize, 2170508u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2170512u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2170516u32);
    emu.anr_no_count(29usize, 7usize, 5usize, 2170520u32);
    emu.sltru_no_count(7usize, 13usize, 14usize, 2170524u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2170528u32);
    emu.sltru_no_count(5usize, 13usize, 17usize, 2170532u32);
    emu.adr_no_count(17usize, 6usize, 7usize, 2170536u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2170540u32);
    emu.sltru_no_count(7usize, 29usize, 30usize, 2170544u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2170548u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2170552u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2170560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ec0));
    } else {
        emu.pc = 2170556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ebc));
    }
}
#[inline(always)]
pub fn block_0x00211ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2170560u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ec0));
}
#[inline(never)]
pub fn block_0x00211ec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2170564u32);
    emu.sw_no_count(8usize, 2usize, 44u32, 2170568u32)?;
    emu.sw_no_count(9usize, 2usize, 40u32, 2170572u32)?;
    emu.sw_no_count(18usize, 2usize, 36u32, 2170576u32)?;
    emu.sw_no_count(19usize, 2usize, 32u32, 2170580u32)?;
    emu.sw_no_count(20usize, 2usize, 28u32, 2170584u32)?;
    emu.sw_no_count(21usize, 2usize, 24u32, 2170588u32)?;
    emu.sw_no_count(22usize, 2usize, 20u32, 2170592u32)?;
    emu.sw_no_count(23usize, 2usize, 16u32, 2170596u32)?;
    emu.sw_no_count(24usize, 2usize, 12u32, 2170600u32)?;
    emu.sw_no_count(25usize, 2usize, 8u32, 2170604u32)?;
    emu.sw_no_count(26usize, 2usize, 4u32, 2170608u32)?;
    emu.sw_no_count(27usize, 2usize, 0u32, 2170612u32)?;
    emu.sbr_no_count(6usize, 0usize, 15usize, 2170616u32);
    emu.sbr_no_count(30usize, 14usize, 15usize, 2170620u32);
    emu.mulhu_no_count(28usize, 15usize, 11usize, 2170624u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2170628u32);
    emu.sbr_no_count(28usize, 28usize, 14usize, 2170632u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2170636u32);
    emu.sbr_no_count(31usize, 0usize, 14usize, 2170640u32);
    emu.mulhu_no_count(8usize, 14usize, 11usize, 2170644u32);
    emu.adr_no_count(28usize, 29usize, 15usize, 2170648u32);
    emu.sltru_no_count(15usize, 28usize, 29usize, 2170652u32);
    emu.adr_no_count(14usize, 7usize, 30usize, 2170656u32);
    emu.sltru_no_count(29usize, 6usize, 31usize, 2170660u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2170664u32);
    emu.adr_no_count(29usize, 8usize, 29usize, 2170668u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2170676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f34));
    } else {
        emu.pc = 2170672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f30));
    }
}
#[inline(always)]
pub fn block_0x00211f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 14usize, 7usize, 2170676u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170676u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f34));
}
#[inline]
pub fn block_0x00211f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 6usize, 15usize, 2170680u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2170684u32);
    emu.adr_no_count(15usize, 17usize, 12usize, 2170688u32);
    emu.sltru_no_count(6usize, 7usize, 6usize, 2170692u32);
    emu.sltru_no_count(28usize, 5usize, 28usize, 2170696u32);
    emu.sltru_no_count(17usize, 15usize, 17usize, 2170700u32);
    emu.adr_no_count(30usize, 29usize, 6usize, 2170704u32);
    emu.adr_no_count(29usize, 14usize, 28usize, 2170708u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2170712u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2170716u32);
    emu.anr_no_count(28usize, 28usize, 14usize, 2170720u32);
    emu.sltru_no_count(6usize, 17usize, 16usize, 2170724u32);
    emu.adr_no_count(14usize, 17usize, 5usize, 2170728u32);
    emu.sltru_no_count(5usize, 14usize, 17usize, 2170732u32);
    emu.adr_no_count(17usize, 6usize, 29usize, 2170736u32);
    emu.adr_no_count(29usize, 7usize, 28usize, 2170740u32);
    emu.sltru_no_count(7usize, 29usize, 7usize, 2170744u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2170748u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2170752u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2170760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f88));
    } else {
        emu.pc = 2170756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f84));
    }
}
#[inline(always)]
pub fn block_0x00211f84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2170760u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f88));
}
#[inline]
pub fn block_0x00211f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(30usize, 16usize, 12usize, 2170764u32);
    emu.sbr_no_count(6usize, 0usize, 12usize, 2170768u32);
    emu.mulhu_no_count(28usize, 12usize, 11usize, 2170772u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2170776u32);
    emu.sbr_no_count(28usize, 28usize, 16usize, 2170780u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2170784u32);
    emu.sbr_no_count(31usize, 0usize, 16usize, 2170788u32);
    emu.mulhu_no_count(8usize, 16usize, 11usize, 2170792u32);
    emu.adr_no_count(28usize, 29usize, 12usize, 2170796u32);
    emu.sltru_no_count(12usize, 28usize, 29usize, 2170800u32);
    emu.adr_no_count(16usize, 7usize, 30usize, 2170804u32);
    emu.sltru_no_count(29usize, 6usize, 31usize, 2170808u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2170812u32);
    emu.adr_no_count(29usize, 8usize, 29usize, 2170816u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2170824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fc8));
    } else {
        emu.pc = 2170820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fc4));
    }
}
#[inline(always)]
pub fn block_0x00211fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 7usize, 2170824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fc8));
}
#[inline]
pub fn block_0x00211fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 6usize, 12usize, 2170828u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2170832u32);
    emu.adr_no_count(12usize, 17usize, 13usize, 2170836u32);
    emu.sltru_no_count(6usize, 30usize, 6usize, 2170840u32);
    emu.sltru_no_count(7usize, 5usize, 28usize, 2170844u32);
    emu.sltru_no_count(17usize, 12usize, 17usize, 2170848u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2170852u32);
    emu.adr_no_count(29usize, 16usize, 7usize, 2170856u32);
    emu.adr_no_count(31usize, 15usize, 17usize, 2170860u32);
    emu.sltru_no_count(16usize, 29usize, 16usize, 2170864u32);
    emu.sltru_no_count(28usize, 31usize, 15usize, 2170868u32);
    emu.adr_no_count(17usize, 31usize, 5usize, 2170872u32);
    emu.anr_no_count(7usize, 7usize, 16usize, 2170876u32);
    emu.sltru_no_count(5usize, 17usize, 31usize, 2170880u32);
    emu.adr_no_count(16usize, 28usize, 5usize, 2170884u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2170888u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2170892u32);
    emu.sltru_no_count(29usize, 7usize, 30usize, 2170896u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2170900u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2170908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021201c));
    } else {
        emu.pc = 2170904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212018));
    }
}
#[inline(always)]
pub fn block_0x00212018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 28usize, 2170908u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021201c));
}
#[inline]
pub fn block_0x0021201c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(31usize, 15usize, 13usize, 2170912u32);
    emu.sbr_no_count(28usize, 0usize, 13usize, 2170916u32);
    emu.mulhu_no_count(29usize, 13usize, 11usize, 2170920u32);
    emu.sltru_no_count(28usize, 31usize, 28usize, 2170924u32);
    emu.sbr_no_count(29usize, 29usize, 15usize, 2170928u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2170932u32);
    emu.sbr_no_count(8usize, 0usize, 15usize, 2170936u32);
    emu.adr_no_count(29usize, 7usize, 13usize, 2170940u32);
    emu.sltru_no_count(30usize, 29usize, 7usize, 2170944u32);
    emu.adr_no_count(13usize, 6usize, 31usize, 2170948u32);
    emu.adr_no_count(13usize, 13usize, 30usize, 2170952u32);
    emu.sltru_no_count(7usize, 28usize, 8usize, 2170956u32);
    emu.mulhu_no_count(11usize, 15usize, 11usize, 2170960u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2170968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212058));
    } else {
        emu.pc = 2170964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212054));
    }
}
#[inline(always)]
pub fn block_0x00212054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 13usize, 6usize, 2170968u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212058));
}
#[inline]
pub fn block_0x00212058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 11usize, 7usize, 2170972u32);
    emu.adr_no_count(6usize, 28usize, 30usize, 2170976u32);
    emu.adr_no_count(15usize, 29usize, 5usize, 2170980u32);
    emu.adi_no_count(11usize, 14usize, 1u32, 2170984u32);
    emu.sltru_no_count(14usize, 6usize, 28usize, 2170988u32);
    emu.sltru_no_count(28usize, 15usize, 29usize, 2170992u32);
    emu.sltiu_no_count(29usize, 11usize, 1u32, 2170996u32);
    emu.adr_no_count(5usize, 13usize, 28usize, 2171000u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2171004u32);
    emu.sltru_no_count(13usize, 5usize, 13usize, 2171008u32);
    emu.orr_no_count(29usize, 11usize, 12usize, 2171012u32);
    emu.anr_no_count(31usize, 28usize, 13usize, 2171016u32);
    emu.sltiu_no_count(30usize, 29usize, 1u32, 2171020u32);
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2171024u32);
    emu.adr_no_count(28usize, 17usize, 30usize, 2171028u32);
    emu.sltru_no_count(13usize, 28usize, 17usize, 2171032u32);
    emu.adr_no_count(29usize, 16usize, 30usize, 2171036u32);
    emu.adr_no_count(29usize, 29usize, 13usize, 2171040u32);
    emu.adr_no_count(17usize, 6usize, 31usize, 2171044u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2171052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120ac));
    } else {
        emu.pc = 2171048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120a8));
    }
}
#[inline(always)]
pub fn block_0x002120a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 29usize, 16usize, 2171052u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120ac));
}
#[inline]
pub fn block_0x002120ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 7usize, 14usize, 2171056u32);
    emu.adr_no_count(16usize, 30usize, 13usize, 2171060u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2171064u32);
    emu.sltru_no_count(14usize, 16usize, 30usize, 2171068u32);
    emu.adr_no_count(30usize, 30usize, 14usize, 2171072u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2171076u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2171080u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2171084u32);
    emu.sltru_no_count(6usize, 17usize, 6usize, 2171088u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2171100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120dc));
    } else {
        emu.pc = 2171092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120d4));
    }
}
#[inline(always)]
pub fn block_0x002120d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 29usize, 2171096u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171104u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120e0));
}
#[inline(always)]
pub fn block_0x002120dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 28usize, 2171104u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120e0));
}
#[inline]
pub fn block_0x002120e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(29usize, 16usize, 1u32, 2171108u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2171112u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2171116u32);
    emu.adr_no_count(28usize, 16usize, 28usize, 2171120u32);
    emu.sltru_no_count(16usize, 28usize, 16usize, 2171124u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2171128u32);
    emu.sai_no_count(28usize, 16usize, 1055u32, 2171132u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2171136u32);
    emu.sltru_no_count(29usize, 15usize, 28usize, 2171140u32);
    emu.adr_no_count(16usize, 28usize, 5usize, 2171144u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2171148u32);
    emu.adr_no_count(7usize, 7usize, 6usize, 2171152u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2171160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212118));
    } else {
        emu.pc = 2171156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212114));
    }
}
#[inline(always)]
pub fn block_0x00212114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 16usize, 28usize, 2171160u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171160u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212118));
}
#[inline]
pub fn block_0x00212118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 28usize, 29usize, 2171164u32);
    emu.sltru_no_count(5usize, 29usize, 28usize, 2171168u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2171172u32);
    emu.sai_no_count(28usize, 5usize, 1055u32, 2171176u32);
    emu.adr_no_count(30usize, 7usize, 28usize, 2171180u32);
    emu.adr_no_count(6usize, 17usize, 28usize, 2171184u32);
    emu.sltru_no_count(5usize, 6usize, 17usize, 2171188u32);
    emu.adr_no_count(30usize, 30usize, 5usize, 2171192u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2171200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212140));
    } else {
        emu.pc = 2171196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021213c));
    }
}
#[inline(always)]
pub fn block_0x0021213c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 7usize, 2171200u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212140));
}
#[inline(always)]
pub fn block_0x00212140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 28usize, 5usize, 2171204u32);
    emu.sltru_no_count(17usize, 0usize, 6usize, 2171208u32);
    emu.sltru_no_count(7usize, 5usize, 28usize, 2171212u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2171216u32);
    emu.adr_no_count(28usize, 28usize, 7usize, 2171220u32);
    emu.adi_no_count(29usize, 6usize, 4294967295u32, 2171224u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2171236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212164));
    } else {
        emu.pc = 2171228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021215c));
    }
}
#[inline(always)]
pub fn block_0x0021215c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 17usize, 30usize, 2171232u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171236u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212168));
}
#[inline(always)]
pub fn block_0x00212164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 29usize, 6usize, 2171240u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212168));
}
#[inline]
pub fn block_0x00212168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 5usize, 4294967295u32, 2171244u32);
    emu.sltiu_no_count(5usize, 5usize, 1u32, 2171248u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2171252u32);
    emu.sbr_no_count(5usize, 28usize, 5usize, 2171256u32);
    emu.sltru_no_count(6usize, 6usize, 7usize, 2171260u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2171264u32);
    emu.sai_no_count(30usize, 5usize, 1055u32, 2171268u32);
    emu.adr_no_count(11usize, 30usize, 11usize, 2171272u32);
    emu.sltru_no_count(5usize, 11usize, 30usize, 2171276u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2171280u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2171284u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2171292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021219c));
    } else {
        emu.pc = 2171288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212198));
    }
}
#[inline(always)]
pub fn block_0x00212198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 12usize, 30usize, 2171292u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021219c));
}
#[inline]
pub fn block_0x0021219c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 30usize, 13usize, 2171296u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2171300u32);
    emu.adr_no_count(13usize, 6usize, 5usize, 2171304u32);
    emu.adr_no_count(5usize, 14usize, 7usize, 2171308u32);
    emu.sltru_no_count(6usize, 13usize, 6usize, 2171312u32);
    emu.sltru_no_count(14usize, 0usize, 5usize, 2171316u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2171320u32);
    emu.anr_no_count(7usize, 14usize, 7usize, 2171324u32);
    emu.adr_no_count(14usize, 5usize, 6usize, 2171328u32);
    emu.sltru_no_count(5usize, 14usize, 5usize, 2171332u32);
    emu.anr_no_count(5usize, 6usize, 5usize, 2171336u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2171340u32);
    emu.sltru_no_count(7usize, 5usize, 7usize, 2171344u32);
    emu.adr_no_count(6usize, 5usize, 15usize, 2171348u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2171352u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2171356u32);
    emu.adr_no_count(28usize, 16usize, 5usize, 2171360u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2171368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121e8));
    } else {
        emu.pc = 2171364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121e4));
    }
}
#[inline(always)]
pub fn block_0x002121e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2171368u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171368u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002121e8));
}
#[inline(never)]
pub fn block_0x002121e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 30usize, 1u32, 2171372u32);
    emu.sbr_no_count(16usize, 29usize, 30usize, 2171376u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2171380u32);
    emu.sri_no_count(8usize, 28usize, 24u32, 2171384u32);
    emu.sri_no_count(9usize, 6usize, 24u32, 2171388u32);
    emu.sri_no_count(31usize, 14usize, 24u32, 2171392u32);
    emu.sri_no_count(30usize, 13usize, 24u32, 2171396u32);
    emu.sri_no_count(29usize, 12usize, 24u32, 2171400u32);
    emu.sri_no_count(15usize, 11usize, 24u32, 2171404u32);
    emu.sri_no_count(18usize, 28usize, 8u32, 2171408u32);
    emu.sri_no_count(19usize, 28usize, 16u32, 2171412u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2171416u32);
    emu.sri_no_count(21usize, 6usize, 16u32, 2171420u32);
    emu.sri_no_count(22usize, 14usize, 8u32, 2171424u32);
    emu.sri_no_count(23usize, 14usize, 16u32, 2171428u32);
    emu.sri_no_count(24usize, 13usize, 8u32, 2171432u32);
    emu.sri_no_count(25usize, 13usize, 16u32, 2171436u32);
    emu.sri_no_count(26usize, 12usize, 8u32, 2171440u32);
    emu.sri_no_count(27usize, 12usize, 16u32, 2171444u32);
    emu.sb_no_count(8usize, 10usize, 8u32, 2171448u32);
    emu.sb_no_count(19usize, 10usize, 9u32, 2171452u32);
    emu.sb_no_count(18usize, 10usize, 10u32, 2171456u32);
    emu.sb_no_count(28usize, 10usize, 11u32, 2171460u32);
    emu.sri_no_count(28usize, 11usize, 8u32, 2171464u32);
    emu.sb_no_count(9usize, 10usize, 12u32, 2171468u32);
    emu.sb_no_count(21usize, 10usize, 13u32, 2171472u32);
    emu.sb_no_count(20usize, 10usize, 14u32, 2171476u32);
    emu.sb_no_count(6usize, 10usize, 15u32, 2171480u32);
    emu.sri_no_count(6usize, 11usize, 16u32, 2171484u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2171488u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2171492u32);
    emu.sb_no_count(31usize, 10usize, 16u32, 2171496u32);
    emu.sb_no_count(23usize, 10usize, 17u32, 2171500u32);
    emu.sb_no_count(22usize, 10usize, 18u32, 2171504u32);
    emu.sb_no_count(14usize, 10usize, 19u32, 2171508u32);
    emu.sb_no_count(30usize, 10usize, 20u32, 2171512u32);
    emu.sb_no_count(25usize, 10usize, 21u32, 2171516u32);
    emu.sb_no_count(24usize, 10usize, 22u32, 2171520u32);
    emu.sb_no_count(13usize, 10usize, 23u32, 2171524u32);
    emu.sb_no_count(29usize, 10usize, 24u32, 2171528u32);
    emu.sb_no_count(27usize, 10usize, 25u32, 2171532u32);
    emu.sb_no_count(26usize, 10usize, 26u32, 2171536u32);
    emu.sb_no_count(12usize, 10usize, 27u32, 2171540u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2171544u32);
    emu.sltru_no_count(12usize, 5usize, 16usize, 2171548u32);
    emu.sri_no_count(13usize, 5usize, 24u32, 2171552u32);
    emu.sri_no_count(14usize, 5usize, 8u32, 2171556u32);
    emu.sri_no_count(16usize, 5usize, 16u32, 2171560u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2171564u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2171568u32);
    emu.sb_no_count(16usize, 10usize, 5u32, 2171572u32);
    emu.sb_no_count(14usize, 10usize, 6u32, 2171576u32);
    emu.sb_no_count(5usize, 10usize, 7u32, 2171580u32);
    emu.sri_no_count(13usize, 12usize, 24u32, 2171584u32);
    emu.sri_no_count(14usize, 12usize, 8u32, 2171588u32);
    emu.sri_no_count(16usize, 12usize, 16u32, 2171592u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2171596u32);
    emu.sb_no_count(16usize, 10usize, 1u32, 2171600u32);
    emu.sb_no_count(14usize, 10usize, 2u32, 2171604u32);
    emu.sb_no_count(12usize, 10usize, 3u32, 2171608u32);
    emu.sb_no_count(15usize, 10usize, 28u32, 2171612u32);
    emu.sb_no_count(6usize, 10usize, 29u32, 2171616u32);
    emu.sb_no_count(28usize, 10usize, 30u32, 2171620u32);
    emu.sb_no_count(11usize, 10usize, 31u32, 2171624u32);
    emu.lw_no_count(8usize, 2usize, 44u32, 2171628u32)?;
    emu.lw_no_count(9usize, 2usize, 40u32, 2171632u32)?;
    emu.lw_no_count(18usize, 2usize, 36u32, 2171636u32)?;
    emu.lw_no_count(19usize, 2usize, 32u32, 2171640u32)?;
    emu.lw_no_count(20usize, 2usize, 28u32, 2171644u32)?;
    emu.lw_no_count(21usize, 2usize, 24u32, 2171648u32)?;
    emu.lw_no_count(22usize, 2usize, 20u32, 2171652u32)?;
    emu.lw_no_count(23usize, 2usize, 16u32, 2171656u32)?;
    emu.lw_no_count(24usize, 2usize, 12u32, 2171660u32)?;
    emu.lw_no_count(25usize, 2usize, 8u32, 2171664u32)?;
    emu.lw_no_count(26usize, 2usize, 4u32, 2171668u32)?;
    emu.lw_no_count(27usize, 2usize, 0u32, 2171672u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2171676u32);
    emu.add_memory_rw_events(78usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171680u32;
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
