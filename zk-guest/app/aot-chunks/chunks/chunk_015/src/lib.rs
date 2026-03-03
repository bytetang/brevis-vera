pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2153356u32;
pub const PC_MAX: u32 = 2162356u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x0020db8c,
        block_0x0020dca0,
        block_0x0020dcb8,
        block_0x0020dd10,
        block_0x0020dd50,
        block_0x0020dd9c,
        block_0x0020dda4,
        block_0x0020ddb4,
        block_0x0020ddc4,
        block_0x0020ddcc,
        block_0x0020de74,
        block_0x0020df14,
        block_0x0020df2c,
        block_0x0020df84,
        block_0x0020dfc8,
        block_0x0020e070,
        block_0x0020e110,
        block_0x0020e128,
        block_0x0020e180,
        block_0x0020e1c0,
        block_0x0020e1d0,
        block_0x0020e1e0,
        block_0x0020e1e8,
        block_0x0020e2f4,
        block_0x0020e31c,
        block_0x0020e334,
        block_0x0020e38c,
        block_0x0020e3d0,
        block_0x0020e45c,
        block_0x0020e46c,
        block_0x0020e49c,
        block_0x0020e534,
        block_0x0020e54c,
        block_0x0020e55c,
        block_0x0020e568,
        block_0x0020e7fc,
        block_0x0020e80c,
        block_0x0020e8e4,
        block_0x0020e980,
        block_0x0020e9ac,
        block_0x0020e9c0,
        block_0x0020e9d4,
        block_0x0020e9e8,
        block_0x0020ea3c,
        block_0x0020ea90,
        block_0x0020eae4,
        block_0x0020eb50,
        block_0x0020eba4,
        block_0x0020ebf8,
        block_0x0020ec4c,
        block_0x0020eca0,
        block_0x0020ecf4,
        block_0x0020ed48,
        block_0x0020edb4,
        block_0x0020edc8,
        block_0x0020ee1c,
        block_0x0020ee70,
        block_0x0020eedc,
        block_0x0020eef0,
        block_0x0020ef04,
        block_0x0020ef58,
        block_0x0020efac,
        block_0x0020f000,
        block_0x0020f054,
        block_0x0020f0a8,
        block_0x0020f0fc,
        block_0x0020f150,
        block_0x0020f1a4,
        block_0x0020f1f8,
        block_0x0020f24c,
        block_0x0020f2a0,
        block_0x0020f2f4,
        block_0x0020f348,
        block_0x0020f39c,
        block_0x0020f3f0,
        block_0x0020f444,
        block_0x0020f4b0,
        block_0x0020f4c4,
        block_0x0020f518,
        block_0x0020f56c,
        block_0x0020f5c0,
        block_0x0020f614,
        block_0x0020f668,
        block_0x0020f6bc,
        block_0x0020f710,
        block_0x0020f764,
        block_0x0020f7b8,
        block_0x0020f80c,
        block_0x0020f860,
        block_0x0020f8b4,
        block_0x0020f908,
        block_0x0020f95c,
        block_0x0020f9c8,
        block_0x0020fa1c,
        block_0x0020fa70,
        block_0x0020fac4,
        block_0x0020fb18,
        block_0x0020fb6c,
        block_0x0020fbc0,
        block_0x0020fc14,
        block_0x0020fc68,
        block_0x0020fcbc,
        block_0x0020fd10,
        block_0x0020fd64,
        block_0x0020fdb8,
        block_0x0020fe0c,
        block_0x0020fe60,
        block_0x0020feb4,
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
            start_word: 69u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 75u32,
            len: 1i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 97u32,
            len: 1i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 113u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 132u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 134u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 138u32,
            len: 1i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 142u32,
            len: 1i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 144u32,
            len: 1i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 186u32,
            len: 1i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 226u32,
            len: 1i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 232u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 254u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 271u32,
            len: 1i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 313u32,
            len: 1i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 353u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 359u32,
            len: 1i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 381u32,
            len: 1i32 as u16,
            fn_offset: 18usize as u16,
        },
        Run {
            start_word: 397u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 401u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 405u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 407u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 474u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 484u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 490u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 512u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 529u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 564u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 568u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 580u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 618u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 624u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 628u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 631u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 796u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 800u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 854u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 893u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 904u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 909u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 914u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 919u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 940u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 961u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 982u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 1009u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 1030u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 1051u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 1072u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 1093u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 1114u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 1135u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 1162u32,
            len: 1i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 1167u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 1188u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 1209u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 1236u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 1241u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 1246u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 1267u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 1288u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 1309u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 1330u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 1351u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 1372u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 1393u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 1414u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 1435u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 1456u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 1477u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 1498u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 1519u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 1540u32,
            len: 1i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 1561u32,
            len: 1i32 as u16,
            fn_offset: 74usize as u16,
        },
        Run {
            start_word: 1582u32,
            len: 1i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 1609u32,
            len: 1i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 1614u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 1635u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 1656u32,
            len: 1i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 1677u32,
            len: 1i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 1698u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 1719u32,
            len: 1i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 1740u32,
            len: 1i32 as u16,
            fn_offset: 83usize as u16,
        },
        Run {
            start_word: 1761u32,
            len: 1i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 1782u32,
            len: 1i32 as u16,
            fn_offset: 85usize as u16,
        },
        Run {
            start_word: 1803u32,
            len: 1i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 1824u32,
            len: 1i32 as u16,
            fn_offset: 87usize as u16,
        },
        Run {
            start_word: 1845u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 1866u32,
            len: 1i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 1887u32,
            len: 1i32 as u16,
            fn_offset: 90usize as u16,
        },
        Run {
            start_word: 1908u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 1935u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 1956u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 1977u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 1998u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 2019u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 2040u32,
            len: 1i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 2061u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 2082u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 2103u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 2124u32,
            len: 1i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 2145u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 2166u32,
            len: 1i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 2187u32,
            len: 1i32 as u16,
            fn_offset: 104usize as u16,
        },
        Run {
            start_word: 2208u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 2229u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
        Run {
            start_word: 2250u32,
            len: 1i32 as u16,
            fn_offset: 107usize as u16,
        },
    ];
    if pc < 2153356u32 || pc > 2162356u32 {
        return None;
    }
    let word_offset = ((pc - 2153356u32) >> 2) as u32;
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
#[inline(never)]
pub fn block_0x0020db8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 69u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2153360u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2153364u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2153368u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2153372u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2153376u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2153380u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2153384u32)?;
    emu.sw_no_count(21usize, 2usize, 180u32, 2153388u32)?;
    emu.sw_no_count(22usize, 2usize, 176u32, 2153392u32)?;
    emu.sw_no_count(23usize, 2usize, 172u32, 2153396u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2153400u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2153404u32);
    emu.lw_no_count(17usize, 11usize, 16u32, 2153408u32)?;
    emu.lw_no_count(16usize, 11usize, 20u32, 2153412u32)?;
    emu.lw_no_count(15usize, 11usize, 24u32, 2153416u32)?;
    emu.lw_no_count(14usize, 11usize, 28u32, 2153420u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2153424u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2153428u32)?;
    emu.lw_no_count(11usize, 11usize, 8u32, 2153432u32)?;
    emu.lw_no_count(10usize, 9usize, 12u32, 2153436u32)?;
    emu.adi_no_count(5usize, 0usize, 4294967295u32, 2153440u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2153444u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2153448u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2153452u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2153456u32;
    emu.update_insn_clock();
    emu.sw_no_count(0usize, 2usize, 92u32, 2153460u32)?;
    emu.sw_no_count(0usize, 2usize, 96u32, 2153464u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2153468u32)?;
    emu.sw_no_count(0usize, 2usize, 104u32, 2153472u32)?;
    emu.sw_no_count(5usize, 2usize, 60u32, 2153476u32)?;
    emu.sw_no_count(5usize, 2usize, 64u32, 2153480u32)?;
    emu.sw_no_count(0usize, 2usize, 68u32, 2153484u32)?;
    emu.sw_no_count(5usize, 2usize, 72u32, 2153488u32)?;
    emu.adi_no_count(5usize, 0usize, 1u32, 2153492u32);
    emu.sw_no_count(0usize, 2usize, 124u32, 2153496u32)?;
    emu.sw_no_count(0usize, 2usize, 128u32, 2153500u32)?;
    emu.sw_no_count(0usize, 2usize, 132u32, 2153504u32)?;
    emu.sw_no_count(0usize, 2usize, 136u32, 2153508u32)?;
    emu.sw_no_count(0usize, 2usize, 108u32, 2153512u32)?;
    emu.sw_no_count(0usize, 2usize, 112u32, 2153516u32)?;
    emu.sw_no_count(0usize, 2usize, 116u32, 2153520u32)?;
    emu.sw_no_count(0usize, 2usize, 120u32, 2153524u32)?;
    emu.adi_no_count(6usize, 6usize, 4294965933u32, 2153528u32);
    emu.adi_no_count(7usize, 7usize, 4294966916u32, 2153532u32);
    emu.adi_no_count(28usize, 28usize, 4294965954u32, 2153536u32);
    emu.adi_no_count(29usize, 29usize, 1361u32, 2153540u32);
    emu.sw_no_count(29usize, 2usize, 44u32, 2153544u32)?;
    emu.sw_no_count(28usize, 2usize, 48u32, 2153548u32)?;
    emu.sw_no_count(7usize, 2usize, 52u32, 2153552u32)?;
    emu.sw_no_count(6usize, 2usize, 56u32, 2153556u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2153560u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294967164u32, 2153564u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2153568u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2153572u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2153576u32)?;
    emu.sw_no_count(14usize, 2usize, 40u32, 2153580u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2153584u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 4294967196u32, 2153588u32);
    emu.sw_no_count(5usize, 2usize, 76u32, 2153592u32)?;
    emu.sw_no_count(0usize, 2usize, 80u32, 2153596u32)?;
    emu.sw_no_count(0usize, 2usize, 84u32, 2153600u32)?;
    emu.sw_no_count(0usize, 2usize, 88u32, 2153604u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2153608u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2153612u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2153616u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2153620u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2153624u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294967260u32, 2153628u32);
    emu.add_memory_rw_events(69usize);
    let return_addr = 2153632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd50));
}
#[inline(always)]
pub fn block_0x0020dca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2153636u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2153640u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2153644u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2153648u32);
    emu.apc_no_count(1usize, 2153648u32, 20480u32, 2153652u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dcb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2153660u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2153664u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2153668u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2153672u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2153676u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2153680u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2153684u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2153688u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2153692u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2153696u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2153700u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2153704u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2153708u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2153712u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2153716u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2153720u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2153724u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2153728u32);
    emu.adi_no_count(12usize, 2usize, 108u32, 2153732u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2153736u32);
    emu.apc_no_count(1usize, 2153736u32, 20480u32, 2153740u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020dd10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2153748u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2153752u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2153756u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2153760u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2153764u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2153768u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2153772u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2153776u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2153780u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2153784u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2153788u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2153792u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2153796u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2153800u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2153804u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2153808u32)?;
    emu.add_memory_rw_events(16usize);
    emu.pc = 2153808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd50));
}
#[inline]
pub fn block_0x0020dd50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2153812u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2153816u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2153820u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2153824u32)?;
    emu.lw_no_count(14usize, 2usize, 28u32, 2153828u32)?;
    emu.lw_no_count(15usize, 2usize, 32u32, 2153832u32)?;
    emu.lw_no_count(16usize, 2usize, 36u32, 2153836u32)?;
    emu.lw_no_count(17usize, 2usize, 40u32, 2153840u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2153844u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2153848u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2153852u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2153856u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2153860u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2153864u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2153868u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2153872u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2153876u32);
    emu.apc_no_count(1usize, 2153876u32, 40960u32, 2153880u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020dd9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2153888u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2155472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3d0));
    } else {
        emu.pc = 2153892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dda4));
    }
}
#[inline(always)]
pub fn block_0x0020dda4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2153896u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2153900u32);
    emu.apc_no_count(1usize, 2153900u32, 40960u32, 2153904u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153908u32;
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
#[inline(always)]
pub fn block_0x0020ddb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2153912u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2153916u32);
    emu.apc_no_count(1usize, 2153916u32, 40960u32, 2153920u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153924u32;
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
pub fn block_0x0020ddc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2153928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2154944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1c0));
    } else {
        emu.pc = 2153932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ddcc));
    }
}
#[inline(never)]
pub fn block_0x0020ddcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 42u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2153936u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2153940u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2153944u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2153948u32)?;
    emu.lw_no_count(14usize, 2usize, 28u32, 2153952u32)?;
    emu.lw_no_count(15usize, 2usize, 32u32, 2153956u32)?;
    emu.lw_no_count(16usize, 2usize, 36u32, 2153960u32)?;
    emu.lw_no_count(17usize, 2usize, 40u32, 2153964u32)?;
    emu.sri_no_count(10usize, 10usize, 1u32, 2153968u32);
    emu.sli_no_count(5usize, 11usize, 31u32, 2153972u32);
    emu.sri_no_count(11usize, 11usize, 1u32, 2153976u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2153980u32);
    emu.sli_no_count(5usize, 12usize, 31u32, 2153984u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2153988u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2153992u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2153996u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2154000u32);
    emu.orr_no_count(12usize, 5usize, 12usize, 2154004u32);
    emu.sli_no_count(5usize, 14usize, 31u32, 2154008u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2154012u32);
    emu.orr_no_count(13usize, 5usize, 13usize, 2154016u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2154020u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2154024u32);
    emu.orr_no_count(14usize, 5usize, 14usize, 2154028u32);
    emu.sli_no_count(5usize, 16usize, 31u32, 2154032u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2154036u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2154040u32);
    emu.sli_no_count(5usize, 17usize, 31u32, 2154044u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2154048u32);
    emu.lw_no_count(5usize, 2usize, 76u32, 2154052u32)?;
    emu.sri_no_count(17usize, 17usize, 1u32, 2154056u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2154060u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2154064u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2154068u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2154072u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2154076u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2154080u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2154084u32)?;
    emu.sw_no_count(17usize, 2usize, 40u32, 2154088u32)?;
    emu.ani_no_count(10usize, 5usize, 1u32, 2154092u32);
    emu.apc_no_count(1usize, 2154092u32, 40960u32, 2154096u32);
    emu.add_memory_rw_events(42usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154100u32;
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
pub fn block_0x0020de74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2154104u32);
    emu.lw_no_count(11usize, 2usize, 76u32, 2154108u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2154112u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2154116u32)?;
    emu.lw_no_count(14usize, 2usize, 88u32, 2154120u32)?;
    emu.lw_no_count(15usize, 2usize, 92u32, 2154124u32)?;
    emu.lw_no_count(16usize, 2usize, 96u32, 2154128u32)?;
    emu.lw_no_count(17usize, 2usize, 100u32, 2154132u32)?;
    emu.lw_no_count(5usize, 2usize, 104u32, 2154136u32)?;
    emu.sri_no_count(11usize, 11usize, 1u32, 2154140u32);
    emu.sli_no_count(6usize, 12usize, 31u32, 2154144u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2154148u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2154152u32);
    emu.sli_no_count(6usize, 13usize, 31u32, 2154156u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2154160u32);
    emu.orr_no_count(12usize, 6usize, 12usize, 2154164u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2154168u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2154172u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2154176u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2154180u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2154184u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2154188u32);
    emu.sli_no_count(6usize, 16usize, 31u32, 2154192u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2154196u32);
    emu.orr_no_count(15usize, 6usize, 15usize, 2154200u32);
    emu.sli_no_count(6usize, 17usize, 31u32, 2154204u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2154208u32);
    emu.orr_no_count(16usize, 6usize, 16usize, 2154212u32);
    emu.sli_no_count(6usize, 5usize, 31u32, 2154216u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2154220u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2154224u32);
    emu.sw_no_count(11usize, 2usize, 76u32, 2154228u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2154232u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2154236u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2154240u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2154244u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2154248u32)?;
    emu.sw_no_count(17usize, 2usize, 100u32, 2154252u32)?;
    emu.sw_no_count(5usize, 2usize, 104u32, 2154256u32)?;
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2153892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dda4));
    } else {
        emu.pc = 2154260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020df14));
    }
}
#[inline(always)]
pub fn block_0x0020df14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2154264u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2154268u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2154272u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2154276u32);
    emu.apc_no_count(1usize, 2154276u32, 20480u32, 2154280u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020df2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2154288u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2154292u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2154296u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2154300u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2154304u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2154308u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2154312u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2154316u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2154320u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2154324u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2154328u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2154332u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2154336u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2154340u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2154344u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2154348u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2154352u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2154356u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2154360u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2154364u32);
    emu.apc_no_count(1usize, 2154364u32, 20480u32, 2154368u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020df84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2154376u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2154380u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2154384u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2154388u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2154392u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2154396u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2154400u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2154404u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2154408u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2154412u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2154416u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2154420u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2154424u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2154428u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2154432u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2154436u32)?;
    emu.add_memory_rw_events(17usize);
    let return_addr = 2154440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dda4));
}
#[inline(never)]
pub fn block_0x0020dfc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 42u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2154444u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2154448u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2154452u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2154456u32)?;
    emu.lw_no_count(14usize, 2usize, 60u32, 2154460u32)?;
    emu.lw_no_count(15usize, 2usize, 64u32, 2154464u32)?;
    emu.lw_no_count(16usize, 2usize, 68u32, 2154468u32)?;
    emu.lw_no_count(17usize, 2usize, 72u32, 2154472u32)?;
    emu.sri_no_count(10usize, 10usize, 1u32, 2154476u32);
    emu.sli_no_count(5usize, 11usize, 31u32, 2154480u32);
    emu.sri_no_count(11usize, 11usize, 1u32, 2154484u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2154488u32);
    emu.sli_no_count(5usize, 12usize, 31u32, 2154492u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2154496u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2154500u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2154504u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2154508u32);
    emu.orr_no_count(12usize, 5usize, 12usize, 2154512u32);
    emu.sli_no_count(5usize, 14usize, 31u32, 2154516u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2154520u32);
    emu.orr_no_count(13usize, 5usize, 13usize, 2154524u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2154528u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2154532u32);
    emu.orr_no_count(14usize, 5usize, 14usize, 2154536u32);
    emu.sli_no_count(5usize, 16usize, 31u32, 2154540u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2154544u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2154548u32);
    emu.sli_no_count(5usize, 17usize, 31u32, 2154552u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2154556u32);
    emu.lw_no_count(5usize, 2usize, 108u32, 2154560u32)?;
    emu.sri_no_count(17usize, 17usize, 1u32, 2154564u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2154568u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2154572u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2154576u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2154580u32)?;
    emu.sw_no_count(14usize, 2usize, 60u32, 2154584u32)?;
    emu.sw_no_count(15usize, 2usize, 64u32, 2154588u32)?;
    emu.sw_no_count(16usize, 2usize, 68u32, 2154592u32)?;
    emu.sw_no_count(17usize, 2usize, 72u32, 2154596u32)?;
    emu.ani_no_count(10usize, 5usize, 1u32, 2154600u32);
    emu.apc_no_count(1usize, 2154600u32, 40960u32, 2154604u32);
    emu.add_memory_rw_events(42usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020e070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2154612u32);
    emu.lw_no_count(11usize, 2usize, 108u32, 2154616u32)?;
    emu.lw_no_count(12usize, 2usize, 112u32, 2154620u32)?;
    emu.lw_no_count(13usize, 2usize, 116u32, 2154624u32)?;
    emu.lw_no_count(14usize, 2usize, 120u32, 2154628u32)?;
    emu.lw_no_count(15usize, 2usize, 124u32, 2154632u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2154636u32)?;
    emu.lw_no_count(17usize, 2usize, 132u32, 2154640u32)?;
    emu.lw_no_count(5usize, 2usize, 136u32, 2154644u32)?;
    emu.sri_no_count(11usize, 11usize, 1u32, 2154648u32);
    emu.sli_no_count(6usize, 12usize, 31u32, 2154652u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2154656u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2154660u32);
    emu.sli_no_count(6usize, 13usize, 31u32, 2154664u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2154668u32);
    emu.orr_no_count(12usize, 6usize, 12usize, 2154672u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2154676u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2154680u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2154684u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2154688u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2154692u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2154696u32);
    emu.sli_no_count(6usize, 16usize, 31u32, 2154700u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2154704u32);
    emu.orr_no_count(15usize, 6usize, 15usize, 2154708u32);
    emu.sli_no_count(6usize, 17usize, 31u32, 2154712u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2154716u32);
    emu.orr_no_count(16usize, 6usize, 16usize, 2154720u32);
    emu.sli_no_count(6usize, 5usize, 31u32, 2154724u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2154728u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2154732u32);
    emu.sw_no_count(11usize, 2usize, 108u32, 2154736u32)?;
    emu.sw_no_count(12usize, 2usize, 112u32, 2154740u32)?;
    emu.sw_no_count(13usize, 2usize, 116u32, 2154744u32)?;
    emu.sw_no_count(14usize, 2usize, 120u32, 2154748u32)?;
    emu.sw_no_count(15usize, 2usize, 124u32, 2154752u32)?;
    emu.sw_no_count(16usize, 2usize, 128u32, 2154756u32)?;
    emu.sw_no_count(17usize, 2usize, 132u32, 2154760u32)?;
    emu.sw_no_count(5usize, 2usize, 136u32, 2154764u32)?;
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2154944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1c0));
    } else {
        emu.pc = 2154768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e110));
    }
}
#[inline(always)]
pub fn block_0x0020e110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2154772u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2154776u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2154780u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2154784u32);
    emu.apc_no_count(1usize, 2154784u32, 20480u32, 2154788u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2154796u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2154800u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2154804u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2154808u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2154812u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2154816u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2154820u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2154824u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2154828u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2154832u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2154836u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2154840u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2154844u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2154848u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2154852u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2154856u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2154860u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2154864u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2154868u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2154872u32);
    emu.apc_no_count(1usize, 2154872u32, 20480u32, 2154876u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2154884u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2154888u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2154892u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2154896u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2154900u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2154904u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2154908u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2154912u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2154916u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2154920u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2154924u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2154928u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2154932u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2154936u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2154940u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2154944u32)?;
    emu.add_memory_rw_events(16usize);
    emu.pc = 2154944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e1c0));
}
#[inline(always)]
pub fn block_0x0020e1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2154948u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2154952u32);
    emu.apc_no_count(1usize, 2154952u32, 40960u32, 2154956u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154960u32;
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
pub fn block_0x0020e1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2154964u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2154968u32);
    emu.apc_no_count(1usize, 2154968u32, 40960u32, 2154972u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2154980u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2154440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dfc8));
    } else {
        emu.pc = 2154984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e1e8));
    }
}
#[inline(never)]
pub fn block_0x0020e1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 67u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2154988u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2154992u32)?;
    emu.lw_no_count(14usize, 2usize, 52u32, 2154996u32)?;
    emu.lw_no_count(15usize, 2usize, 56u32, 2155000u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2155004u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2155008u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2155012u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2155016u32)?;
    emu.lw_no_count(5usize, 2usize, 60u32, 2155020u32)?;
    emu.lw_no_count(6usize, 2usize, 64u32, 2155024u32)?;
    emu.lw_no_count(7usize, 2usize, 68u32, 2155028u32)?;
    emu.lw_no_count(28usize, 2usize, 72u32, 2155032u32)?;
    emu.lw_no_count(29usize, 2usize, 28u32, 2155036u32)?;
    emu.lw_no_count(30usize, 2usize, 32u32, 2155040u32)?;
    emu.lw_no_count(31usize, 2usize, 36u32, 2155044u32)?;
    emu.lw_no_count(21usize, 2usize, 40u32, 2155048u32)?;
    emu.sltru_no_count(22usize, 10usize, 11usize, 2155052u32);
    emu.sbr_no_count(23usize, 14usize, 16usize, 2155056u32);
    emu.sltru_no_count(14usize, 14usize, 16usize, 2155060u32);
    emu.sbr_no_count(16usize, 15usize, 17usize, 2155064u32);
    emu.sltru_no_count(15usize, 15usize, 17usize, 2155068u32);
    emu.sbr_no_count(17usize, 5usize, 29usize, 2155072u32);
    emu.sltru_no_count(5usize, 5usize, 29usize, 2155076u32);
    emu.sbr_no_count(29usize, 6usize, 30usize, 2155080u32);
    emu.sltru_no_count(6usize, 6usize, 30usize, 2155084u32);
    emu.sbr_no_count(30usize, 7usize, 31usize, 2155088u32);
    emu.sltru_no_count(7usize, 7usize, 31usize, 2155092u32);
    emu.sbr_no_count(31usize, 28usize, 21usize, 2155096u32);
    emu.sltru_no_count(28usize, 28usize, 21usize, 2155100u32);
    emu.sbr_no_count(21usize, 0usize, 22usize, 2155104u32);
    emu.sbr_no_count(13usize, 13usize, 22usize, 2155108u32);
    emu.sltru_no_count(21usize, 13usize, 21usize, 2155112u32);
    emu.sbr_no_count(21usize, 21usize, 22usize, 2155116u32);
    emu.sltru_no_count(22usize, 13usize, 12usize, 2155120u32);
    emu.sbr_no_count(21usize, 21usize, 22usize, 2155124u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2155128u32);
    emu.sbr_no_count(22usize, 21usize, 14usize, 2155132u32);
    emu.adr_no_count(14usize, 23usize, 21usize, 2155136u32);
    emu.sltru_no_count(21usize, 14usize, 23usize, 2155140u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2155144u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2155148u32);
    emu.sbr_no_count(22usize, 21usize, 15usize, 2155152u32);
    emu.adr_no_count(15usize, 16usize, 21usize, 2155156u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2155160u32);
    emu.adr_no_count(16usize, 22usize, 16usize, 2155164u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2155168u32);
    emu.sbr_no_count(5usize, 16usize, 5usize, 2155172u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2155176u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2155180u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2155184u32);
    emu.sai_no_count(17usize, 17usize, 1055u32, 2155188u32);
    emu.sbr_no_count(5usize, 17usize, 6usize, 2155192u32);
    emu.adr_no_count(17usize, 29usize, 17usize, 2155196u32);
    emu.sltru_no_count(6usize, 17usize, 29usize, 2155200u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2155204u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2155208u32);
    emu.sbr_no_count(6usize, 5usize, 7usize, 2155212u32);
    emu.adr_no_count(5usize, 30usize, 5usize, 2155216u32);
    emu.sltru_no_count(7usize, 5usize, 30usize, 2155220u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2155224u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2155228u32);
    emu.sbr_no_count(7usize, 6usize, 28usize, 2155232u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2155236u32);
    emu.sltru_no_count(28usize, 6usize, 31usize, 2155240u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2155244u32);
    emu.ani_no_count(7usize, 7usize, 2u32, 2155248u32);
    emu.add_memory_rw_events(66usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2153632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dca0));
    } else {
        emu.pc = 2155252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2f4));
    }
}
#[inline]
pub fn block_0x0020e2f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 10usize, 11usize, 2155256u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2155260u32);
    emu.orr_no_count(10usize, 13usize, 10usize, 2155264u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2155268u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2155272u32);
    emu.orr_no_count(10usize, 10usize, 16usize, 2155276u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2155280u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2155284u32);
    emu.orr_no_count(10usize, 10usize, 6usize, 2155288u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2153632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dca0));
    } else {
        emu.pc = 2155292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e31c));
    }
}
#[inline(always)]
pub fn block_0x0020e31c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2155296u32);
    emu.adi_no_count(11usize, 2usize, 44u32, 2155300u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2155304u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2155308u32);
    emu.apc_no_count(1usize, 2155308u32, 20480u32, 2155312u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155316u32;
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
pub fn block_0x0020e334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2155320u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2155324u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2155328u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2155332u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2155336u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2155340u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2155344u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2155348u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2155352u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2155356u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2155360u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2155364u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2155368u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2155372u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2155376u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2155380u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2155384u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2155388u32);
    emu.adi_no_count(12usize, 2usize, 76u32, 2155392u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2155396u32);
    emu.apc_no_count(1usize, 2155396u32, 20480u32, 2155400u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e38c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2155408u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2155412u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2155416u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2155420u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2155424u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2155428u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2155432u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2155436u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2155440u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2155444u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2155448u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2155452u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2155456u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2155460u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2155464u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2155468u32)?;
    emu.add_memory_rw_events(17usize);
    let return_addr = 2155472u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2153808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dd50));
}
#[inline(never)]
pub fn block_0x0020e3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 124u32, 2155476u32)?;
    emu.lw_no_count(11usize, 2usize, 128u32, 2155480u32)?;
    emu.lw_no_count(12usize, 2usize, 132u32, 2155484u32)?;
    emu.lw_no_count(13usize, 2usize, 136u32, 2155488u32)?;
    emu.lw_no_count(14usize, 2usize, 108u32, 2155492u32)?;
    emu.lw_no_count(15usize, 2usize, 112u32, 2155496u32)?;
    emu.lw_no_count(16usize, 2usize, 116u32, 2155500u32)?;
    emu.lw_no_count(17usize, 2usize, 120u32, 2155504u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2155508u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2155512u32)?;
    emu.sw_no_count(12usize, 8usize, 24u32, 2155516u32)?;
    emu.sw_no_count(13usize, 8usize, 28u32, 2155520u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2155524u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2155528u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2155532u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2155536u32)?;
    emu.sw_no_count(14usize, 8usize, 0u32, 2155540u32)?;
    emu.sw_no_count(15usize, 8usize, 4u32, 2155544u32)?;
    emu.sw_no_count(16usize, 8usize, 8u32, 2155548u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2155552u32)?;
    emu.lw_no_count(14usize, 9usize, 16u32, 2155556u32)?;
    emu.lw_no_count(15usize, 9usize, 20u32, 2155560u32)?;
    emu.lw_no_count(16usize, 9usize, 24u32, 2155564u32)?;
    emu.lw_no_count(17usize, 9usize, 28u32, 2155568u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2155572u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2155576u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2155580u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2155584u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2155588u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2155592u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2155596u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2155600u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2155604u32);
    emu.apc_no_count(1usize, 2155604u32, 40960u32, 2155608u32);
    emu.add_memory_rw_events(35usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e45c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2155616u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2155620u32);
    emu.apc_no_count(1usize, 2155620u32, 40960u32, 2155624u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155628u32;
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
pub fn block_0x0020e46c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 8usize, 32u32, 2155632u32);
    emu.lw_no_count(1usize, 2usize, 204u32, 2155636u32)?;
    emu.lw_no_count(8usize, 2usize, 200u32, 2155640u32)?;
    emu.lw_no_count(9usize, 2usize, 196u32, 2155644u32)?;
    emu.lw_no_count(18usize, 2usize, 192u32, 2155648u32)?;
    emu.lw_no_count(19usize, 2usize, 188u32, 2155652u32)?;
    emu.lw_no_count(20usize, 2usize, 184u32, 2155656u32)?;
    emu.lw_no_count(21usize, 2usize, 180u32, 2155660u32)?;
    emu.lw_no_count(22usize, 2usize, 176u32, 2155664u32)?;
    emu.lw_no_count(23usize, 2usize, 172u32, 2155668u32)?;
    emu.adi_no_count(2usize, 2usize, 208u32, 2155672u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155676u32;
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
pub fn block_0x0020e49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2155680u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2155684u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2155688u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2155692u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2155696u32)?;
    emu.lw_no_count(14usize, 10usize, 8u32, 2155700u32)?;
    emu.lw_no_count(15usize, 10usize, 12u32, 2155704u32)?;
    emu.lw_no_count(16usize, 11usize, 0u32, 2155708u32)?;
    emu.lw_no_count(17usize, 11usize, 4u32, 2155712u32)?;
    emu.lw_no_count(5usize, 11usize, 8u32, 2155716u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2155720u32)?;
    emu.lw_no_count(7usize, 10usize, 16u32, 2155724u32)?;
    emu.lw_no_count(28usize, 10usize, 20u32, 2155728u32)?;
    emu.lw_no_count(29usize, 10usize, 24u32, 2155732u32)?;
    emu.lw_no_count(10usize, 10usize, 28u32, 2155736u32)?;
    emu.lw_no_count(30usize, 11usize, 16u32, 2155740u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2155744u32)?;
    emu.lw_no_count(8usize, 11usize, 24u32, 2155748u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2155752u32)?;
    emu.xrr_no_count(13usize, 17usize, 13usize, 2155756u32);
    emu.xrr_no_count(12usize, 16usize, 12usize, 2155760u32);
    emu.xrr_no_count(14usize, 5usize, 14usize, 2155764u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2155768u32);
    emu.xrr_no_count(16usize, 30usize, 7usize, 2155772u32);
    emu.xrr_no_count(17usize, 31usize, 28usize, 2155776u32);
    emu.xrr_no_count(5usize, 8usize, 29usize, 2155780u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2155784u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2155788u32);
    emu.orr_no_count(13usize, 16usize, 17usize, 2155792u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2155796u32);
    emu.orr_no_count(13usize, 13usize, 5usize, 2155800u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2155804u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2155808u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2155812u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2155816u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2155820u32);
    emu.apc_no_count(1usize, 2155820u32, 40960u32, 2155824u32);
    emu.add_memory_rw_events(38usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2155832u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2155836u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2155840u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2155844u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2155848u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155852u32;
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
pub fn block_0x0020e54c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2155856u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2155860u32)?;
    emu.apc_no_count(1usize, 2155860u32, 4294963200u32, 2155864u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2155872u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2155876u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155880u32;
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
pub fn block_0x0020e568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 165u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2155884u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2155888u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2155892u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2155896u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2155900u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2155904u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2155908u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2155912u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2155916u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2155920u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2155924u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2155928u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2155932u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2155936u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2155940u32)?;
    emu.lbu_no_count(10usize, 11usize, 0u32, 2155944u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2155948u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2155952u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2155956u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2155960u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2155964u32);
    emu.lbu_no_count(17usize, 11usize, 6u32, 2155968u32);
    emu.lbu_no_count(5usize, 11usize, 7u32, 2155972u32);
    emu.lbu_no_count(6usize, 11usize, 8u32, 2155976u32);
    emu.lbu_no_count(7usize, 11usize, 9u32, 2155980u32);
    emu.lbu_no_count(28usize, 11usize, 10u32, 2155984u32);
    emu.lbu_no_count(29usize, 11usize, 11u32, 2155988u32);
    emu.lbu_no_count(30usize, 11usize, 12u32, 2155992u32);
    emu.lbu_no_count(31usize, 11usize, 13u32, 2155996u32);
    emu.lbu_no_count(18usize, 11usize, 14u32, 2156000u32);
    emu.lbu_no_count(19usize, 11usize, 15u32, 2156004u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2156008u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2156012u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2156016u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2156020u32);
    emu.orr_no_count(9usize, 10usize, 12usize, 2156024u32);
    emu.lbu_no_count(20usize, 11usize, 16u32, 2156028u32);
    emu.lbu_no_count(21usize, 11usize, 17u32, 2156032u32);
    emu.lbu_no_count(22usize, 11usize, 18u32, 2156036u32);
    emu.lbu_no_count(23usize, 11usize, 19u32, 2156040u32);
    emu.sli_no_count(17usize, 17usize, 8u32, 2156044u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2156048u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2156052u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2156056u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2156060u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2156064u32);
    emu.orr_no_count(14usize, 17usize, 5usize, 2156068u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2156072u32);
    emu.orr_no_count(10usize, 28usize, 29usize, 2156076u32);
    emu.orr_no_count(12usize, 6usize, 7usize, 2156080u32);
    emu.lbu_no_count(7usize, 11usize, 20u32, 2156084u32);
    emu.lbu_no_count(28usize, 11usize, 21u32, 2156088u32);
    emu.lbu_no_count(29usize, 11usize, 22u32, 2156092u32);
    emu.lbu_no_count(24usize, 11usize, 23u32, 2156096u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2156100u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2156104u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2156108u32);
    emu.sli_no_count(22usize, 22usize, 8u32, 2156112u32);
    emu.sli_no_count(21usize, 21usize, 16u32, 2156116u32);
    emu.sli_no_count(20usize, 20usize, 24u32, 2156120u32);
    emu.orr_no_count(16usize, 18usize, 19usize, 2156124u32);
    emu.orr_no_count(17usize, 30usize, 31usize, 2156128u32);
    emu.orr_no_count(5usize, 22usize, 23usize, 2156132u32);
    emu.orr_no_count(6usize, 20usize, 21usize, 2156136u32);
    emu.lbu_no_count(30usize, 11usize, 24u32, 2156140u32);
    emu.lbu_no_count(31usize, 11usize, 25u32, 2156144u32);
    emu.lbu_no_count(18usize, 11usize, 26u32, 2156148u32);
    emu.lbu_no_count(19usize, 11usize, 27u32, 2156152u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2156156u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2156160u32);
    emu.sli_no_count(20usize, 7usize, 24u32, 2156164u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2156168u32);
    emu.orr_no_count(7usize, 29usize, 24usize, 2156172u32);
    emu.orr_no_count(28usize, 20usize, 28usize, 2156176u32);
    emu.orr_no_count(29usize, 18usize, 19usize, 2156180u32);
    emu.lbu_no_count(18usize, 11usize, 28u32, 2156184u32);
    emu.lbu_no_count(19usize, 11usize, 29u32, 2156188u32);
    emu.lbu_no_count(20usize, 11usize, 30u32, 2156192u32);
    emu.lbu_no_count(11usize, 11usize, 31u32, 2156196u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2156200u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2156204u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2156208u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2156212u32);
    emu.orr_no_count(11usize, 20usize, 11usize, 2156216u32);
    let a = 0u32.wrapping_add(60612608u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2156220u32;
    emu.update_insn_clock();
    emu.sli_no_count(19usize, 19usize, 16u32, 2156224u32);
    emu.sli_no_count(18usize, 18usize, 24u32, 2156228u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2156232u32);
    let a = 0u32.wrapping_add(205926400u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2156236u32;
    emu.update_insn_clock();
    emu.orr_no_count(1usize, 9usize, 13usize, 2156240u32);
    let a = 0u32.wrapping_add(1491623936u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2156244u32;
    emu.update_insn_clock();
    emu.orr_no_count(15usize, 15usize, 14usize, 2156248u32);
    emu.sw_no_count(15usize, 2usize, 8u32, 2156252u32)?;
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2156256u32;
    emu.update_insn_clock();
    emu.adi_no_count(25usize, 31usize, 4294965935u32, 2156260u32);
    emu.adi_no_count(26usize, 19usize, 1342u32, 2156264u32);
    emu.adi_no_count(27usize, 13usize, 380u32, 2156268u32);
    emu.adi_no_count(13usize, 14usize, 1363u32, 2156272u32);
    emu.orr_no_count(19usize, 12usize, 10usize, 2156276u32);
    emu.orr_no_count(20usize, 17usize, 16usize, 2156280u32);
    emu.orr_no_count(21usize, 6usize, 5usize, 2156284u32);
    emu.orr_no_count(22usize, 28usize, 7usize, 2156288u32);
    emu.orr_no_count(23usize, 30usize, 29usize, 2156292u32);
    emu.orr_no_count(24usize, 18usize, 11usize, 2156296u32);
    emu.adr_no_count(25usize, 24usize, 25usize, 2156300u32);
    emu.sw_no_count(25usize, 2usize, 20u32, 2156304u32)?;
    emu.sltru_no_count(10usize, 25usize, 24usize, 2156308u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156312u32);
    emu.adr_no_count(11usize, 23usize, 10usize, 2156316u32);
    emu.sltru_no_count(12usize, 11usize, 23usize, 2156320u32);
    emu.adr_no_count(26usize, 11usize, 26usize, 2156324u32);
    emu.sw_no_count(26usize, 2usize, 16u32, 2156328u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2156332u32);
    emu.sltru_no_count(11usize, 26usize, 11usize, 2156336u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2156340u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156344u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2156348u32);
    emu.adr_no_count(11usize, 22usize, 10usize, 2156352u32);
    emu.sltru_no_count(12usize, 11usize, 22usize, 2156356u32);
    emu.adr_no_count(27usize, 11usize, 27usize, 2156360u32);
    emu.sw_no_count(27usize, 2usize, 12u32, 2156364u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2156368u32);
    emu.sltru_no_count(11usize, 27usize, 11usize, 2156372u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2156376u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156380u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2156384u32);
    emu.adr_no_count(11usize, 21usize, 10usize, 2156388u32);
    emu.sltru_no_count(12usize, 11usize, 21usize, 2156392u32);
    emu.adr_no_count(18usize, 11usize, 13usize, 2156396u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2156400u32);
    emu.sltru_no_count(11usize, 18usize, 11usize, 2156404u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2156408u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156412u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2156416u32);
    emu.adr_no_count(11usize, 20usize, 10usize, 2156420u32);
    emu.sltru_no_count(12usize, 11usize, 20usize, 2156424u32);
    emu.adi_no_count(8usize, 11usize, 1u32, 2156428u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2156432u32);
    emu.sltiu_no_count(11usize, 8usize, 1u32, 2156436u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2156440u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156444u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2156448u32);
    emu.adr_no_count(11usize, 19usize, 10usize, 2156452u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2156456u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2156460u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2156464u32);
    emu.sltiu_no_count(11usize, 9usize, 1u32, 2156468u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2156472u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156476u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2156480u32);
    emu.adr_no_count(25usize, 10usize, 15usize, 2156484u32);
    emu.sltru_no_count(11usize, 25usize, 10usize, 2156488u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2156492u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2156496u32);
    emu.adi_no_count(27usize, 1usize, 0u32, 2156500u32);
    emu.adr_no_count(11usize, 1usize, 10usize, 2156504u32);
    emu.sltru_no_count(12usize, 11usize, 1usize, 2156508u32);
    emu.adi_no_count(26usize, 11usize, 1u32, 2156512u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2156516u32);
    emu.sltiu_no_count(11usize, 26usize, 1u32, 2156520u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2156524u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156528u32);
    emu.sri_no_count(10usize, 10usize, 31u32, 2156532u32);
    emu.apc_no_count(1usize, 2156532u32, 40960u32, 2156536u32);
    emu.add_memory_rw_events(165usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e7fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2156544u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2156548u32);
    emu.apc_no_count(1usize, 2156548u32, 40960u32, 2156552u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156556u32;
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
#[inline(never)]
pub fn block_0x0020e80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 54u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2156560u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2156564u32)?;
    emu.xrr_no_count(11usize, 24usize, 11usize, 2156568u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2156572u32)?;
    emu.xrr_no_count(12usize, 23usize, 12usize, 2156576u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2156580u32)?;
    emu.xrr_no_count(13usize, 22usize, 13usize, 2156584u32);
    emu.xrr_no_count(14usize, 21usize, 18usize, 2156588u32);
    emu.xrr_no_count(15usize, 20usize, 8usize, 2156592u32);
    emu.xrr_no_count(16usize, 19usize, 9usize, 2156596u32);
    emu.lw_no_count(6usize, 2usize, 8u32, 2156600u32)?;
    emu.xrr_no_count(17usize, 6usize, 25usize, 2156604u32);
    emu.xrr_no_count(5usize, 27usize, 26usize, 2156608u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2156612u32);
    emu.anr_no_count(11usize, 11usize, 10usize, 2156616u32);
    emu.anr_no_count(12usize, 12usize, 10usize, 2156620u32);
    emu.anr_no_count(13usize, 13usize, 10usize, 2156624u32);
    emu.anr_no_count(14usize, 14usize, 10usize, 2156628u32);
    emu.anr_no_count(15usize, 15usize, 10usize, 2156632u32);
    emu.anr_no_count(16usize, 16usize, 10usize, 2156636u32);
    emu.anr_no_count(17usize, 17usize, 10usize, 2156640u32);
    emu.anr_no_count(10usize, 5usize, 10usize, 2156644u32);
    emu.xrr_no_count(11usize, 11usize, 24usize, 2156648u32);
    emu.xrr_no_count(12usize, 12usize, 23usize, 2156652u32);
    emu.xrr_no_count(13usize, 13usize, 22usize, 2156656u32);
    emu.xrr_no_count(14usize, 14usize, 21usize, 2156660u32);
    emu.xrr_no_count(15usize, 15usize, 20usize, 2156664u32);
    emu.xrr_no_count(16usize, 16usize, 19usize, 2156668u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2156672u32);
    emu.xrr_no_count(10usize, 10usize, 27usize, 2156676u32);
    emu.lw_no_count(5usize, 2usize, 24u32, 2156680u32)?;
    emu.sw_no_count(11usize, 5usize, 0u32, 2156684u32)?;
    emu.sw_no_count(12usize, 5usize, 4u32, 2156688u32)?;
    emu.sw_no_count(13usize, 5usize, 8u32, 2156692u32)?;
    emu.sw_no_count(14usize, 5usize, 12u32, 2156696u32)?;
    emu.sw_no_count(15usize, 5usize, 16u32, 2156700u32)?;
    emu.sw_no_count(16usize, 5usize, 20u32, 2156704u32)?;
    emu.sw_no_count(17usize, 5usize, 24u32, 2156708u32)?;
    emu.sw_no_count(10usize, 5usize, 28u32, 2156712u32)?;
    emu.lw_no_count(1usize, 2usize, 76u32, 2156716u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2156720u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2156724u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2156728u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2156732u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2156736u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2156740u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2156744u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2156748u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2156752u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2156756u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2156760u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2156764u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2156768u32);
    emu.add_memory_rw_events(54usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156772u32;
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
pub fn block_0x0020e8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2156776u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2156780u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2156784u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2156788u32)?;
    emu.lw_no_count(14usize, 10usize, 8u32, 2156792u32)?;
    emu.lw_no_count(15usize, 10usize, 12u32, 2156796u32)?;
    emu.lw_no_count(16usize, 11usize, 0u32, 2156800u32)?;
    emu.lw_no_count(17usize, 11usize, 4u32, 2156804u32)?;
    emu.lw_no_count(5usize, 11usize, 8u32, 2156808u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2156812u32)?;
    emu.lw_no_count(7usize, 10usize, 16u32, 2156816u32)?;
    emu.lw_no_count(28usize, 10usize, 20u32, 2156820u32)?;
    emu.lw_no_count(29usize, 10usize, 24u32, 2156824u32)?;
    emu.lw_no_count(10usize, 10usize, 28u32, 2156828u32)?;
    emu.lw_no_count(30usize, 11usize, 16u32, 2156832u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2156836u32)?;
    emu.lw_no_count(8usize, 11usize, 24u32, 2156840u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2156844u32)?;
    emu.xrr_no_count(13usize, 17usize, 13usize, 2156848u32);
    emu.xrr_no_count(12usize, 16usize, 12usize, 2156852u32);
    emu.xrr_no_count(14usize, 5usize, 14usize, 2156856u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2156860u32);
    emu.xrr_no_count(16usize, 30usize, 7usize, 2156864u32);
    emu.xrr_no_count(17usize, 31usize, 28usize, 2156868u32);
    emu.xrr_no_count(5usize, 8usize, 29usize, 2156872u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2156876u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2156880u32);
    emu.orr_no_count(13usize, 16usize, 17usize, 2156884u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2156888u32);
    emu.orr_no_count(13usize, 13usize, 5usize, 2156892u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2156896u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2156900u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2156904u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2156908u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2156912u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2156916u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2156920u32);
    emu.apc_no_count(6usize, 2156920u32, 40960u32, 2156924u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2156928u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966928u32, 2156932u32);
    emu.sw_no_count(1usize, 2usize, 364u32, 2156936u32)?;
    emu.sw_no_count(8usize, 2usize, 360u32, 2156940u32)?;
    emu.sw_no_count(9usize, 2usize, 356u32, 2156944u32)?;
    emu.sw_no_count(18usize, 2usize, 352u32, 2156948u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2156952u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2156956u32);
    emu.adi_no_count(10usize, 2usize, 320u32, 2156960u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2156964u32);
    emu.apc_no_count(1usize, 2156964u32, 4294955008u32, 2156968u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 288u32, 2156976u32);
    emu.adi_no_count(12usize, 2usize, 320u32, 2156980u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2156984u32);
    emu.apc_no_count(1usize, 2156984u32, 4294955008u32, 2156988u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 256u32, 2156996u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157000u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157004u32);
    emu.apc_no_count(1usize, 2157004u32, 4294955008u32, 2157008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2157016u32);
    emu.adi_no_count(12usize, 2usize, 256u32, 2157020u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2157024u32);
    emu.apc_no_count(1usize, 2157024u32, 4294955008u32, 2157028u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020e9e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2157036u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2157040u32)?;
    emu.lw_no_count(12usize, 2usize, 24u32, 2157044u32)?;
    emu.lw_no_count(13usize, 2usize, 28u32, 2157048u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157052u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157056u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157060u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157064u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2157068u32)?;
    emu.lw_no_count(11usize, 2usize, 4u32, 2157072u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2157076u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2157080u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157084u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157088u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157092u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157096u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157100u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157104u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157108u32);
    emu.apc_no_count(1usize, 2157108u32, 4294955008u32, 2157112u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ea3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157120u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157124u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157128u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157132u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157136u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157140u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157144u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157148u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2157152u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2157156u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2157160u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2157164u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157168u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157172u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157176u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157180u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157184u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157188u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157192u32);
    emu.apc_no_count(1usize, 2157192u32, 4294955008u32, 2157196u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ea90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157204u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157208u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157212u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157216u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157220u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157224u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157228u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157232u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2157236u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2157240u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2157244u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2157248u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157252u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157256u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157260u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157264u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157268u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157272u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157276u32);
    emu.apc_no_count(1usize, 2157276u32, 4294955008u32, 2157280u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157284u32;
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
#[inline(never)]
pub fn block_0x0020eae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157288u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157292u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157296u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157300u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2157304u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2157308u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2157312u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2157316u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157320u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157324u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2157328u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2157332u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157336u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157340u32)?;
    emu.sw_no_count(14usize, 2usize, 320u32, 2157344u32)?;
    emu.sw_no_count(15usize, 2usize, 324u32, 2157348u32)?;
    emu.sw_no_count(16usize, 2usize, 328u32, 2157352u32)?;
    emu.sw_no_count(17usize, 2usize, 332u32, 2157356u32)?;
    emu.sw_no_count(10usize, 2usize, 336u32, 2157360u32)?;
    emu.sw_no_count(11usize, 2usize, 340u32, 2157364u32)?;
    emu.sw_no_count(12usize, 2usize, 344u32, 2157368u32)?;
    emu.sw_no_count(13usize, 2usize, 348u32, 2157372u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2157376u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2157380u32);
    emu.adi_no_count(12usize, 2usize, 320u32, 2157384u32);
    emu.apc_no_count(1usize, 2157384u32, 4294955008u32, 2157388u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020eb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2157396u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2157400u32)?;
    emu.lw_no_count(12usize, 2usize, 56u32, 2157404u32)?;
    emu.lw_no_count(13usize, 2usize, 60u32, 2157408u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157412u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157416u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157420u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157424u32)?;
    emu.lw_no_count(10usize, 2usize, 32u32, 2157428u32)?;
    emu.lw_no_count(11usize, 2usize, 36u32, 2157432u32)?;
    emu.lw_no_count(12usize, 2usize, 40u32, 2157436u32)?;
    emu.lw_no_count(13usize, 2usize, 44u32, 2157440u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157444u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157448u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157452u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157456u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157460u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157464u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157468u32);
    emu.apc_no_count(1usize, 2157468u32, 4294955008u32, 2157472u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020eba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157480u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157484u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157488u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157492u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157496u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157500u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157504u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157508u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2157512u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2157516u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2157520u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2157524u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157528u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157532u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157536u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157540u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157544u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157548u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157552u32);
    emu.apc_no_count(1usize, 2157552u32, 4294955008u32, 2157556u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157560u32;
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
#[inline]
pub fn block_0x0020ebf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157564u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157568u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157572u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157576u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157580u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157584u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157588u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157592u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2157596u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2157600u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2157604u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2157608u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157612u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157616u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157620u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157624u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157628u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157632u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157636u32);
    emu.apc_no_count(1usize, 2157636u32, 4294955008u32, 2157640u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ec4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157648u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157652u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157656u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157660u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157664u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157668u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157672u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157676u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2157680u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2157684u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2157688u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2157692u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157696u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157700u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157704u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157708u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157712u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157716u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157720u32);
    emu.apc_no_count(1usize, 2157720u32, 4294955008u32, 2157724u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020eca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157732u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157736u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157740u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157744u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157748u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157752u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157756u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157760u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2157764u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2157768u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2157772u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2157776u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157780u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157784u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157788u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157792u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157796u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157800u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157804u32);
    emu.apc_no_count(1usize, 2157804u32, 4294955008u32, 2157808u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ecf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157816u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157820u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157824u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157828u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157832u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157836u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157840u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157844u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2157848u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2157852u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2157856u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2157860u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2157864u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2157868u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2157872u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2157876u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2157880u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2157884u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2157888u32);
    emu.apc_no_count(1usize, 2157888u32, 4294955008u32, 2157892u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ed48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2157900u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2157904u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2157908u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2157912u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2157916u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2157920u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2157924u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2157928u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2157932u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2157936u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2157940u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2157944u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2157948u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2157952u32)?;
    emu.sw_no_count(14usize, 2usize, 256u32, 2157956u32)?;
    emu.sw_no_count(15usize, 2usize, 260u32, 2157960u32)?;
    emu.sw_no_count(16usize, 2usize, 264u32, 2157964u32)?;
    emu.sw_no_count(17usize, 2usize, 268u32, 2157968u32)?;
    emu.sw_no_count(10usize, 2usize, 272u32, 2157972u32)?;
    emu.sw_no_count(11usize, 2usize, 276u32, 2157976u32)?;
    emu.sw_no_count(12usize, 2usize, 280u32, 2157980u32)?;
    emu.sw_no_count(13usize, 2usize, 284u32, 2157984u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2157988u32);
    emu.adi_no_count(11usize, 2usize, 256u32, 2157992u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2157996u32);
    emu.apc_no_count(1usize, 2157996u32, 4294950912u32, 2158000u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020edb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2158008u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158012u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158016u32);
    emu.apc_no_count(1usize, 2158016u32, 4294950912u32, 2158020u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020edc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158028u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158032u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158036u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158040u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158044u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158048u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158052u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158056u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158060u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158064u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158068u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158072u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158076u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158080u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158084u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158088u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158092u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158096u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158100u32);
    emu.apc_no_count(1usize, 2158100u32, 4294950912u32, 2158104u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ee1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158112u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158116u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158120u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158124u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158128u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158132u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158136u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158140u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158144u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158148u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158152u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158156u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158160u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158164u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158168u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158172u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158176u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158180u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158184u32);
    emu.apc_no_count(1usize, 2158184u32, 4294950912u32, 2158188u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020ee70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158196u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158200u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158204u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158208u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2158212u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2158216u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2158220u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2158224u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158228u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158232u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2158236u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2158240u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158244u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158248u32)?;
    emu.sw_no_count(14usize, 2usize, 320u32, 2158252u32)?;
    emu.sw_no_count(15usize, 2usize, 324u32, 2158256u32)?;
    emu.sw_no_count(16usize, 2usize, 328u32, 2158260u32)?;
    emu.sw_no_count(17usize, 2usize, 332u32, 2158264u32)?;
    emu.sw_no_count(10usize, 2usize, 336u32, 2158268u32)?;
    emu.sw_no_count(11usize, 2usize, 340u32, 2158272u32)?;
    emu.sw_no_count(12usize, 2usize, 344u32, 2158276u32)?;
    emu.sw_no_count(13usize, 2usize, 348u32, 2158280u32)?;
    emu.adi_no_count(10usize, 2usize, 64u32, 2158284u32);
    emu.adi_no_count(11usize, 2usize, 320u32, 2158288u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2158292u32);
    emu.apc_no_count(1usize, 2158292u32, 4294950912u32, 2158296u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1684u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020eedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2158304u32);
    emu.adi_no_count(11usize, 2usize, 64u32, 2158308u32);
    emu.adi_no_count(12usize, 2usize, 64u32, 2158312u32);
    emu.apc_no_count(1usize, 2158312u32, 4294950912u32, 2158316u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020eef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 96u32, 2158324u32);
    emu.adi_no_count(11usize, 2usize, 320u32, 2158328u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2158332u32);
    emu.apc_no_count(1usize, 2158332u32, 4294950912u32, 2158336u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ef04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 112u32, 2158344u32)?;
    emu.lw_no_count(11usize, 2usize, 116u32, 2158348u32)?;
    emu.lw_no_count(12usize, 2usize, 120u32, 2158352u32)?;
    emu.lw_no_count(13usize, 2usize, 124u32, 2158356u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158360u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158364u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158368u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158372u32)?;
    emu.lw_no_count(10usize, 2usize, 96u32, 2158376u32)?;
    emu.lw_no_count(11usize, 2usize, 100u32, 2158380u32)?;
    emu.lw_no_count(12usize, 2usize, 104u32, 2158384u32)?;
    emu.lw_no_count(13usize, 2usize, 108u32, 2158388u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158392u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158396u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158400u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158404u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158408u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158412u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158416u32);
    emu.apc_no_count(1usize, 2158416u32, 4294950912u32, 2158420u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ef58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158428u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158432u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158436u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158440u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158444u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158448u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158452u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158456u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158460u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158464u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158468u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158472u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158476u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158480u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158484u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158488u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158492u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158496u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158500u32);
    emu.apc_no_count(1usize, 2158500u32, 4294950912u32, 2158504u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020efac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158512u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158516u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158520u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158524u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158528u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158532u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158536u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158540u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158544u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158548u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158552u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158556u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158560u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158564u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158568u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158572u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158576u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158580u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158584u32);
    emu.apc_no_count(1usize, 2158584u32, 4294950912u32, 2158588u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158596u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158600u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158604u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158608u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158612u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158616u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158620u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158624u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158628u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158632u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158636u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158640u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158644u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158648u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158652u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158656u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158660u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158664u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158668u32);
    emu.apc_no_count(1usize, 2158668u32, 4294950912u32, 2158672u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158680u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158684u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158688u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158692u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158696u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158700u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158704u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158708u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158712u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158716u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158720u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158724u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158728u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158732u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158736u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158740u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158744u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158748u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158752u32);
    emu.apc_no_count(1usize, 2158752u32, 4294950912u32, 2158756u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158760u32;
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
#[inline]
pub fn block_0x0020f0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158764u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158768u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158772u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158776u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158780u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158784u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158788u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158792u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158796u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158800u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158804u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158808u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158812u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158816u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158820u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158824u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158828u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158832u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158836u32);
    emu.apc_no_count(1usize, 2158836u32, 4294950912u32, 2158840u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f0fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158848u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158852u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158856u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158860u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158864u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158868u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158872u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158876u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158880u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158884u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158888u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158892u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158896u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158900u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158904u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158908u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158912u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2158916u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2158920u32);
    emu.apc_no_count(1usize, 2158920u32, 4294950912u32, 2158924u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1056u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2158932u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2158936u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2158940u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2158944u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2158948u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2158952u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2158956u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2158960u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2158964u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2158968u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2158972u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2158976u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2158980u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2158984u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2158988u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2158992u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2158996u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159000u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159004u32);
    emu.apc_no_count(1usize, 2159004u32, 4294950912u32, 2159008u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159016u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159020u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159024u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159028u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159032u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159036u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159040u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159044u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159048u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159052u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159056u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159060u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159064u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159068u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159072u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159076u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159080u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159084u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159088u32);
    emu.apc_no_count(1usize, 2159088u32, 4294950912u32, 2159092u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f1f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159100u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159104u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159108u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159112u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159116u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159120u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159124u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159128u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159132u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159136u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159140u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159144u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159148u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159152u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159156u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159160u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159164u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159168u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159172u32);
    emu.apc_no_count(1usize, 2159172u32, 4294950912u32, 2159176u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159184u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159188u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159192u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159196u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159200u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159204u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159208u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159212u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159216u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159220u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159224u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159228u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159232u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159236u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159240u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159244u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159248u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159252u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159256u32);
    emu.apc_no_count(1usize, 2159256u32, 4294950912u32, 2159260u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159264u32;
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
#[inline]
pub fn block_0x0020f2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159268u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159272u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159276u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159280u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159284u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159288u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159292u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159296u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159300u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159304u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159308u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159312u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159316u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159320u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159324u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159328u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159332u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159336u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159340u32);
    emu.apc_no_count(1usize, 2159340u32, 4294950912u32, 2159344u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f2f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159352u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159356u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159360u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159364u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159368u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159372u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159376u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159380u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159384u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159388u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159392u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159396u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159400u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159404u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159408u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159412u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159416u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159420u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159424u32);
    emu.apc_no_count(1usize, 2159424u32, 4294950912u32, 2159428u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159436u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159440u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159444u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159448u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159452u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159456u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159460u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159464u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159468u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159472u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159476u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159480u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159484u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159488u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159492u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159496u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159500u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159504u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159508u32);
    emu.apc_no_count(1usize, 2159508u32, 4294950912u32, 2159512u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159520u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159524u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159528u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159532u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159536u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159540u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159544u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159548u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159552u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159556u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159560u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159564u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159568u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159572u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159576u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159580u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159584u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159588u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159592u32);
    emu.apc_no_count(1usize, 2159592u32, 4294950912u32, 2159596u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159600u32;
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
pub fn block_0x0020f3f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159604u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159608u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159612u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159616u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159620u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159624u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159628u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159632u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159636u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159640u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159644u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159648u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159652u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159656u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159660u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159664u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159668u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159672u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159676u32);
    emu.apc_no_count(1usize, 2159676u32, 4294950912u32, 2159680u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159684u32;
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
#[inline(never)]
pub fn block_0x0020f444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159688u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159692u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159696u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159700u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2159704u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2159708u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2159712u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2159716u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159720u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159724u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2159728u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2159732u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159736u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159740u32)?;
    emu.sw_no_count(14usize, 2usize, 256u32, 2159744u32)?;
    emu.sw_no_count(15usize, 2usize, 260u32, 2159748u32)?;
    emu.sw_no_count(16usize, 2usize, 264u32, 2159752u32)?;
    emu.sw_no_count(17usize, 2usize, 268u32, 2159756u32)?;
    emu.sw_no_count(10usize, 2usize, 272u32, 2159760u32)?;
    emu.sw_no_count(11usize, 2usize, 276u32, 2159764u32)?;
    emu.sw_no_count(12usize, 2usize, 280u32, 2159768u32)?;
    emu.sw_no_count(13usize, 2usize, 284u32, 2159772u32)?;
    emu.adi_no_count(10usize, 2usize, 288u32, 2159776u32);
    emu.adi_no_count(11usize, 2usize, 256u32, 2159780u32);
    emu.adi_no_count(12usize, 2usize, 96u32, 2159784u32);
    emu.apc_no_count(1usize, 2159784u32, 4294950912u32, 2159788u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159792u32;
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
pub fn block_0x0020f4b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 320u32, 2159796u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159800u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159804u32);
    emu.apc_no_count(1usize, 2159804u32, 4294950912u32, 2159808u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159816u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159820u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159824u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159828u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159832u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159836u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159840u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159844u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159848u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159852u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159856u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159860u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159864u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159868u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159872u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159876u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159880u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159884u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159888u32);
    emu.apc_no_count(1usize, 2159888u32, 4294950912u32, 2159892u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(88u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159900u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159904u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159908u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159912u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2159916u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2159920u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2159924u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2159928u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2159932u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2159936u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2159940u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2159944u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2159948u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2159952u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2159956u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2159960u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2159964u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2159968u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2159972u32);
    emu.apc_no_count(1usize, 2159972u32, 4294950912u32, 2159976u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f56c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2159984u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2159988u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2159992u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2159996u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160000u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160004u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160008u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160012u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160016u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160020u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160024u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160028u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160032u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160036u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160040u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160044u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160048u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160052u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160056u32);
    emu.apc_no_count(1usize, 2160056u32, 4294950912u32, 2160060u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f5c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160068u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160072u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160076u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160080u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160084u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160088u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160092u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160096u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160100u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160104u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160108u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160112u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160116u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160120u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160124u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160128u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160132u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160136u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160140u32);
    emu.apc_no_count(1usize, 2160140u32, 4294950912u32, 2160144u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160148u32;
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
#[inline]
pub fn block_0x0020f614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160152u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160156u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160160u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160164u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160168u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160172u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160176u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160180u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160184u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160188u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160192u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160196u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160200u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160204u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160208u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160212u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160216u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160220u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160224u32);
    emu.apc_no_count(1usize, 2160224u32, 4294950912u32, 2160228u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967048u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160236u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160240u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160244u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160248u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160252u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160256u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160260u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160264u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160268u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160272u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160276u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160280u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160284u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160288u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160292u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160296u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160300u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160304u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160308u32);
    emu.apc_no_count(1usize, 2160308u32, 4294950912u32, 2160312u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f6bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160320u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160324u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160328u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160332u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160336u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160340u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160344u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160348u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160352u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160356u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160360u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160364u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160368u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160372u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160376u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160380u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160384u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160388u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160392u32);
    emu.apc_no_count(1usize, 2160392u32, 4294950912u32, 2160396u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160404u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160408u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160412u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160416u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160420u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160424u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160428u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160432u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160436u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160440u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160444u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160448u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160452u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160456u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160460u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160464u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160468u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160472u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160476u32);
    emu.apc_no_count(1usize, 2160476u32, 4294950912u32, 2160480u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160488u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160492u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160496u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160500u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160504u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160508u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160512u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160516u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160520u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160524u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160528u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160532u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160536u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160540u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160544u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160548u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160552u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160556u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160560u32);
    emu.apc_no_count(1usize, 2160560u32, 4294950912u32, 2160564u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160568u32;
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
#[inline]
pub fn block_0x0020f7b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160572u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160576u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160580u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160584u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160588u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160592u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160596u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160600u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160604u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160608u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160612u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160616u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160620u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160624u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160628u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160632u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160636u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160640u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160644u32);
    emu.apc_no_count(1usize, 2160644u32, 4294950912u32, 2160648u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160656u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160660u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160664u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160668u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160672u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160676u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160680u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160684u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160688u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160692u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160696u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160700u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160704u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160708u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160712u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160716u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160720u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160724u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160728u32);
    emu.apc_no_count(1usize, 2160728u32, 4294950912u32, 2160732u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160736u32;
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
#[inline]
pub fn block_0x0020f860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160740u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160744u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160748u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160752u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160756u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160760u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160764u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160768u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160772u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160776u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160780u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160784u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160788u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160792u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160796u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160800u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160804u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160808u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160812u32);
    emu.apc_no_count(1usize, 2160812u32, 4294950912u32, 2160816u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160824u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160828u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160832u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160836u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160840u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160844u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160848u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160852u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160856u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160860u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160864u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160868u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160872u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160876u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160880u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160884u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160888u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160892u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160896u32);
    emu.apc_no_count(1usize, 2160896u32, 4294950912u32, 2160900u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160904u32;
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
pub fn block_0x0020f908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160908u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160912u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2160916u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2160920u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2160924u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2160928u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2160932u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2160936u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2160940u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2160944u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2160948u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2160952u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2160956u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2160960u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2160964u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2160968u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2160972u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2160976u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2160980u32);
    emu.apc_no_count(1usize, 2160980u32, 4294950912u32, 2160984u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020f95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2160992u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2160996u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161000u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161004u32)?;
    emu.lw_no_count(14usize, 2usize, 320u32, 2161008u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2161012u32)?;
    emu.lw_no_count(16usize, 2usize, 328u32, 2161016u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2161020u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161024u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161028u32)?;
    emu.sw_no_count(14usize, 2usize, 288u32, 2161032u32)?;
    emu.sw_no_count(17usize, 2usize, 300u32, 2161036u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161040u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161044u32)?;
    emu.sw_no_count(14usize, 2usize, 128u32, 2161048u32)?;
    emu.sw_no_count(15usize, 2usize, 132u32, 2161052u32)?;
    emu.sw_no_count(16usize, 2usize, 136u32, 2161056u32)?;
    emu.sw_no_count(17usize, 2usize, 140u32, 2161060u32)?;
    emu.sw_no_count(10usize, 2usize, 144u32, 2161064u32)?;
    emu.sw_no_count(11usize, 2usize, 148u32, 2161068u32)?;
    emu.sw_no_count(12usize, 2usize, 152u32, 2161072u32)?;
    emu.sw_no_count(13usize, 2usize, 156u32, 2161076u32)?;
    emu.adi_no_count(10usize, 2usize, 160u32, 2161080u32);
    emu.adi_no_count(11usize, 2usize, 64u32, 2161084u32);
    emu.adi_no_count(12usize, 2usize, 128u32, 2161088u32);
    emu.apc_no_count(1usize, 2161088u32, 4294950912u32, 2161092u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020f9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 144u32, 2161100u32)?;
    emu.lw_no_count(11usize, 2usize, 148u32, 2161104u32)?;
    emu.lw_no_count(12usize, 2usize, 152u32, 2161108u32)?;
    emu.lw_no_count(13usize, 2usize, 156u32, 2161112u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161116u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161120u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161124u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161128u32)?;
    emu.lw_no_count(10usize, 2usize, 128u32, 2161132u32)?;
    emu.lw_no_count(11usize, 2usize, 132u32, 2161136u32)?;
    emu.lw_no_count(12usize, 2usize, 136u32, 2161140u32)?;
    emu.lw_no_count(13usize, 2usize, 140u32, 2161144u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161148u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161152u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161156u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161160u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161164u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161168u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161172u32);
    emu.apc_no_count(1usize, 2161172u32, 4294950912u32, 2161176u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161180u32;
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
#[inline]
pub fn block_0x0020fa1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161184u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161188u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161192u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161196u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161200u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161204u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161208u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161212u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161216u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161220u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161224u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161228u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161232u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161236u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161240u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161244u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161248u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161252u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161256u32);
    emu.apc_no_count(1usize, 2161256u32, 4294950912u32, 2161260u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161264u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fa70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161268u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161272u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161276u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161280u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161284u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161288u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161292u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161296u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161300u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161304u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161308u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161312u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161316u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161320u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161324u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161328u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161332u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161336u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161340u32);
    emu.apc_no_count(1usize, 2161340u32, 4294950912u32, 2161344u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161352u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161356u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161360u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161364u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161368u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161372u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161376u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161380u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161384u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161388u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161392u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161396u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161400u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161404u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161408u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161412u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161416u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161420u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161424u32);
    emu.apc_no_count(1usize, 2161424u32, 4294950912u32, 2161428u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fb18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161436u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161440u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161444u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161448u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161452u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161456u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161460u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161464u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161468u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161472u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161476u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161480u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161484u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161488u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161492u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161496u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161500u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161504u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161508u32);
    emu.apc_no_count(1usize, 2161508u32, 4294950912u32, 2161512u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161520u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161524u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161528u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161532u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161536u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161540u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161544u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161548u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161552u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161556u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161560u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161564u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161568u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161572u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161576u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161580u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161584u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161588u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161592u32);
    emu.apc_no_count(1usize, 2161592u32, 4294950912u32, 2161596u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fbc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161604u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161608u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161612u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161616u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161620u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161624u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161628u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161632u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161636u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161640u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161644u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161648u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161652u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161656u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161660u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161664u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161668u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161672u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161676u32);
    emu.apc_no_count(1usize, 2161676u32, 4294950912u32, 2161680u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fc14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161688u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161692u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161696u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161700u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161704u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161708u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161712u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161716u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161720u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161724u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161728u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161732u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161736u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161740u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161744u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161748u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161752u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161756u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161760u32);
    emu.apc_no_count(1usize, 2161760u32, 4294950912u32, 2161764u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161768u32;
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
#[inline]
pub fn block_0x0020fc68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161772u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161776u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161780u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161784u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161788u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161792u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161796u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161800u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161804u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161808u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161812u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161816u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161820u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161824u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161828u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161832u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161836u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161840u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161844u32);
    emu.apc_no_count(1usize, 2161844u32, 4294950912u32, 2161848u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fcbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161856u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161860u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161864u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161868u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161872u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161876u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161880u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161884u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161888u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161892u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161896u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161900u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161904u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161908u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161912u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2161916u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2161920u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2161924u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2161928u32);
    emu.apc_no_count(1usize, 2161928u32, 4294950912u32, 2161932u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fd10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2161940u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2161944u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2161948u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2161952u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2161956u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2161960u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2161964u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2161968u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2161972u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2161976u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2161980u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2161984u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2161988u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2161992u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2161996u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162000u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2162004u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162008u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162012u32);
    emu.apc_no_count(1usize, 2162012u32, 4294950912u32, 2162016u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fd64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162024u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162028u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162032u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162036u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162040u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162044u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162048u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162052u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162056u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162060u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162064u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162068u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2162072u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162076u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162080u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162084u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2162088u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162092u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162096u32);
    emu.apc_no_count(1usize, 2162096u32, 4294946816u32, 2162100u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fdb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162108u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162112u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162116u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162120u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162124u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162128u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162132u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162136u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162140u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162144u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162148u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162152u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2162156u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162160u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162164u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162168u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2162172u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162176u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162180u32);
    emu.apc_no_count(1usize, 2162180u32, 4294946816u32, 2162184u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fe0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162192u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162196u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162200u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162204u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162208u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162212u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162216u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162220u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162224u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162228u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162232u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162236u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2162240u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162244u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162248u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162252u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2162256u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162260u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162264u32);
    emu.apc_no_count(1usize, 2162264u32, 4294946816u32, 2162268u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fe60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162276u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162280u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162284u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162288u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162292u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162296u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162300u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162304u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162308u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162312u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162316u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162320u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2162324u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162328u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162332u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162336u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2162340u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162344u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162348u32);
    emu.apc_no_count(1usize, 2162348u32, 4294946816u32, 2162352u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020feb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 336u32, 2162360u32)?;
    emu.lw_no_count(11usize, 2usize, 340u32, 2162364u32)?;
    emu.lw_no_count(12usize, 2usize, 344u32, 2162368u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2162372u32)?;
    emu.sw_no_count(10usize, 2usize, 304u32, 2162376u32)?;
    emu.sw_no_count(11usize, 2usize, 308u32, 2162380u32)?;
    emu.sw_no_count(12usize, 2usize, 312u32, 2162384u32)?;
    emu.sw_no_count(13usize, 2usize, 316u32, 2162388u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2162392u32)?;
    emu.lw_no_count(11usize, 2usize, 324u32, 2162396u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2162400u32)?;
    emu.lw_no_count(13usize, 2usize, 332u32, 2162404u32)?;
    emu.sw_no_count(10usize, 2usize, 288u32, 2162408u32)?;
    emu.sw_no_count(11usize, 2usize, 292u32, 2162412u32)?;
    emu.sw_no_count(12usize, 2usize, 296u32, 2162416u32)?;
    emu.sw_no_count(13usize, 2usize, 300u32, 2162420u32)?;
    emu.adi_no_count(10usize, 2usize, 320u32, 2162424u32);
    emu.adi_no_count(11usize, 2usize, 288u32, 2162428u32);
    emu.adi_no_count(12usize, 2usize, 288u32, 2162432u32);
    emu.apc_no_count(1usize, 2162432u32, 4294946816u32, 2162436u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
