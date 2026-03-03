pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2171680u32;
pub const PC_MAX: u32 = 2194260u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 126usize] = [
        block_0x00212320,
        block_0x00212360,
        block_0x00212364,
        block_0x002123a8,
        block_0x002123ac,
        block_0x002123fc,
        block_0x00212400,
        block_0x0021243c,
        block_0x00212440,
        block_0x00212490,
        block_0x00212494,
        block_0x002124d0,
        block_0x002124d4,
        block_0x00212524,
        block_0x00212528,
        block_0x00212560,
        block_0x00212564,
        block_0x002125b4,
        block_0x002125b8,
        block_0x002125e0,
        block_0x002125e8,
        block_0x002125ec,
        block_0x00212620,
        block_0x00212624,
        block_0x00212648,
        block_0x0021264c,
        block_0x0021265c,
        block_0x0021266c,
        block_0x00212674,
        block_0x002126a0,
        block_0x002126b4,
        block_0x002126c0,
        block_0x002127a8,
        block_0x00212844,
        block_0x00212858,
        block_0x00212864,
        block_0x00212884,
        block_0x002128d0,
        block_0x002128e0,
        block_0x002128f8,
        block_0x00212908,
        block_0x00212914,
        block_0x002129c4,
        block_0x00212a34,
        block_0x00212a50,
        block_0x00212a6c,
        block_0x00212a70,
        block_0x00212a9c,
        block_0x00212ac8,
        block_0x00212af4,
        block_0x00212b20,
        block_0x00212b4c,
        block_0x00212b74,
        block_0x00212b90,
        block_0x00212b94,
        block_0x00212bbc,
        block_0x00212bc4,
        block_0x00212bec,
        block_0x00212bf4,
        block_0x00212c1c,
        block_0x00212c24,
        block_0x00212c4c,
        block_0x00212c54,
        block_0x00212c78,
        block_0x00212c80,
        block_0x00212ca4,
        block_0x00212d6c,
        block_0x0021304c,
        block_0x0021327c,
        block_0x00213408,
        block_0x00213468,
        block_0x0021346c,
        block_0x00213778,
        block_0x00217604,
        block_0x00217608,
        block_0x00217668,
        block_0x00217674,
        block_0x00217698,
        block_0x002176bc,
        block_0x002176e0,
        block_0x00217704,
        block_0x00217728,
        block_0x0021774c,
        block_0x00217770,
        block_0x00217794,
        block_0x002177b8,
        block_0x002177dc,
        block_0x00217800,
        block_0x00217824,
        block_0x00217848,
        block_0x0021786c,
        block_0x0021789c,
        block_0x002178c0,
        block_0x002178f0,
        block_0x00217914,
        block_0x00217938,
        block_0x0021795c,
        block_0x00217980,
        block_0x002179a4,
        block_0x002179d0,
        block_0x00217a20,
        block_0x00217a2c,
        block_0x00217a50,
        block_0x00217a9c,
        block_0x00217aa4,
        block_0x00217aac,
        block_0x00217ab4,
        block_0x00217abc,
        block_0x00217ac4,
        block_0x00217acc,
        block_0x00217ad4,
        block_0x00217adc,
        block_0x00217ae4,
        block_0x00217aec,
        block_0x00217af4,
        block_0x00217afc,
        block_0x00217b04,
        block_0x00217b0c,
        block_0x00217b14,
        block_0x00217b1c,
        block_0x00217b24,
        block_0x00217b2c,
        block_0x00217b34,
        block_0x00217b38,
        block_0x00217b50,
        block_0x00217b54,
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
            len: 2i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 34u32,
            len: 2i32 as u16,
            fn_offset: 3usize as u16,
        },
        Run {
            start_word: 55u32,
            len: 2i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 71u32,
            len: 2i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 92u32,
            len: 2i32 as u16,
            fn_offset: 9usize as u16,
        },
        Run {
            start_word: 108u32,
            len: 2i32 as u16,
            fn_offset: 11usize as u16,
        },
        Run {
            start_word: 129u32,
            len: 2i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 144u32,
            len: 2i32 as u16,
            fn_offset: 15usize as u16,
        },
        Run {
            start_word: 165u32,
            len: 2i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 176u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 178u32,
            len: 2i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 192u32,
            len: 2i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 202u32,
            len: 2i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 207u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 211u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 213u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 224u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 229u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 232u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 290u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 329u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 334u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 337u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 345u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 364u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 368u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 374u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 378u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 381u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 425u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 453u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 460u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 467u32,
            len: 2i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 479u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 490u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 501u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 512u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 523u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 533u32,
            len: 1i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 540u32,
            len: 2i32 as u16,
            fn_offset: 53usize as u16,
        },
        Run {
            start_word: 551u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 553u32,
            len: 1i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 563u32,
            len: 1i32 as u16,
            fn_offset: 57usize as u16,
        },
        Run {
            start_word: 565u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 575u32,
            len: 1i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 577u32,
            len: 1i32 as u16,
            fn_offset: 60usize as u16,
        },
        Run {
            start_word: 587u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 589u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 598u32,
            len: 1i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 600u32,
            len: 1i32 as u16,
            fn_offset: 64usize as u16,
        },
        Run {
            start_word: 609u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 659u32,
            len: 1i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 843u32,
            len: 1i32 as u16,
            fn_offset: 67usize as u16,
        },
        Run {
            start_word: 983u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 1082u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 1106u32,
            len: 2i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 1302u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 5305u32,
            len: 2i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 5330u32,
            len: 1i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 5333u32,
            len: 1i32 as u16,
            fn_offset: 76usize as u16,
        },
        Run {
            start_word: 5342u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 5351u32,
            len: 1i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 5360u32,
            len: 1i32 as u16,
            fn_offset: 79usize as u16,
        },
        Run {
            start_word: 5369u32,
            len: 1i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 5378u32,
            len: 1i32 as u16,
            fn_offset: 81usize as u16,
        },
        Run {
            start_word: 5387u32,
            len: 1i32 as u16,
            fn_offset: 82usize as u16,
        },
        Run {
            start_word: 5396u32,
            len: 1i32 as u16,
            fn_offset: 83usize as u16,
        },
        Run {
            start_word: 5405u32,
            len: 1i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 5414u32,
            len: 1i32 as u16,
            fn_offset: 85usize as u16,
        },
        Run {
            start_word: 5423u32,
            len: 1i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 5432u32,
            len: 1i32 as u16,
            fn_offset: 87usize as u16,
        },
        Run {
            start_word: 5441u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 5450u32,
            len: 1i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 5459u32,
            len: 1i32 as u16,
            fn_offset: 90usize as u16,
        },
        Run {
            start_word: 5471u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 5480u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 5492u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 5501u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 5510u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 5519u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 5528u32,
            len: 1i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 5537u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 5548u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 5568u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 5571u32,
            len: 1i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 5580u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 5599u32,
            len: 1i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 5601u32,
            len: 1i32 as u16,
            fn_offset: 104usize as u16,
        },
        Run {
            start_word: 5603u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 5605u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
        Run {
            start_word: 5607u32,
            len: 1i32 as u16,
            fn_offset: 107usize as u16,
        },
        Run {
            start_word: 5609u32,
            len: 1i32 as u16,
            fn_offset: 108usize as u16,
        },
        Run {
            start_word: 5611u32,
            len: 1i32 as u16,
            fn_offset: 109usize as u16,
        },
        Run {
            start_word: 5613u32,
            len: 1i32 as u16,
            fn_offset: 110usize as u16,
        },
        Run {
            start_word: 5615u32,
            len: 1i32 as u16,
            fn_offset: 111usize as u16,
        },
        Run {
            start_word: 5617u32,
            len: 1i32 as u16,
            fn_offset: 112usize as u16,
        },
        Run {
            start_word: 5619u32,
            len: 1i32 as u16,
            fn_offset: 113usize as u16,
        },
        Run {
            start_word: 5621u32,
            len: 1i32 as u16,
            fn_offset: 114usize as u16,
        },
        Run {
            start_word: 5623u32,
            len: 1i32 as u16,
            fn_offset: 115usize as u16,
        },
        Run {
            start_word: 5625u32,
            len: 1i32 as u16,
            fn_offset: 116usize as u16,
        },
        Run {
            start_word: 5627u32,
            len: 1i32 as u16,
            fn_offset: 117usize as u16,
        },
        Run {
            start_word: 5629u32,
            len: 1i32 as u16,
            fn_offset: 118usize as u16,
        },
        Run {
            start_word: 5631u32,
            len: 1i32 as u16,
            fn_offset: 119usize as u16,
        },
        Run {
            start_word: 5633u32,
            len: 1i32 as u16,
            fn_offset: 120usize as u16,
        },
        Run {
            start_word: 5635u32,
            len: 1i32 as u16,
            fn_offset: 121usize as u16,
        },
        Run {
            start_word: 5637u32,
            len: 2i32 as u16,
            fn_offset: 122usize as u16,
        },
        Run {
            start_word: 5644u32,
            len: 2i32 as u16,
            fn_offset: 124usize as u16,
        },
    ];
    if pc < 2171680u32 || pc > 2194260u32 {
        return None;
    }
    let word_offset = ((pc - 2171680u32) >> 2) as u32;
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
pub fn block_0x00212320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 10usize, 0u32, 2171684u32)?;
    emu.lw_no_count(17usize, 10usize, 4u32, 2171688u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2171692u32)?;
    emu.lw_no_count(11usize, 10usize, 16u32, 2171696u32)?;
    emu.lw_no_count(12usize, 10usize, 20u32, 2171700u32)?;
    emu.lw_no_count(7usize, 10usize, 24u32, 2171704u32)?;
    emu.lw_no_count(5usize, 10usize, 28u32, 2171708u32)?;
    emu.adr_no_count(13usize, 15usize, 13usize, 2171712u32);
    emu.sltru_no_count(14usize, 13usize, 15usize, 2171716u32);
    emu.adr_no_count(16usize, 17usize, 14usize, 2171720u32);
    emu.sltru_no_count(14usize, 16usize, 17usize, 2171724u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2171728u32);
    emu.sltru_no_count(16usize, 11usize, 16usize, 2171732u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2171736u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2171740u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2171748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212364));
    } else {
        emu.pc = 2171744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212360));
    }
}
#[inline(always)]
pub fn block_0x00212360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 12usize, 14usize, 2171748u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212364));
}
#[inline]
pub fn block_0x00212364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2171752u32)?;
    emu.sbr_no_count(6usize, 0usize, 15usize, 2171756u32);
    emu.sbr_no_count(28usize, 17usize, 15usize, 2171760u32);
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2171764u32);
    emu.sltru_no_count(6usize, 28usize, 6usize, 2171768u32);
    emu.mulhu_no_count(29usize, 15usize, 10usize, 2171772u32);
    emu.sbr_no_count(29usize, 29usize, 17usize, 2171776u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2171780u32);
    emu.sbr_no_count(29usize, 0usize, 17usize, 2171784u32);
    emu.adr_no_count(7usize, 15usize, 7usize, 2171788u32);
    emu.mulhu_no_count(30usize, 17usize, 10usize, 2171792u32);
    emu.adr_no_count(17usize, 28usize, 5usize, 2171796u32);
    emu.sltru_no_count(15usize, 7usize, 15usize, 2171800u32);
    emu.sltru_no_count(5usize, 6usize, 29usize, 2171804u32);
    emu.adr_no_count(17usize, 17usize, 15usize, 2171808u32);
    emu.adr_no_count(5usize, 30usize, 5usize, 2171812u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2171820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123ac));
    } else {
        emu.pc = 2171816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123a8));
    }
}
#[inline(always)]
pub fn block_0x002123a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 17usize, 28usize, 2171820u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002123ac));
}
#[inline]
pub fn block_0x002123ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 6usize, 15usize, 2171824u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2171828u32);
    emu.adr_no_count(15usize, 12usize, 14usize, 2171832u32);
    emu.sltru_no_count(6usize, 29usize, 6usize, 2171836u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2171840u32);
    emu.sltru_no_count(12usize, 15usize, 12usize, 2171844u32);
    emu.adr_no_count(6usize, 5usize, 6usize, 2171848u32);
    emu.adr_no_count(5usize, 17usize, 7usize, 2171852u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2171856u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2171860u32);
    emu.anr_no_count(28usize, 7usize, 17usize, 2171864u32);
    emu.sltru_no_count(7usize, 12usize, 13usize, 2171868u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2171872u32);
    emu.sltru_no_count(17usize, 12usize, 16usize, 2171876u32);
    emu.adr_no_count(16usize, 5usize, 7usize, 2171880u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2171884u32);
    emu.sltru_no_count(7usize, 28usize, 29usize, 2171888u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2171892u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2171896u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2171904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212400));
    } else {
        emu.pc = 2171900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123fc));
    }
}
#[inline(always)]
pub fn block_0x002123fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 16usize, 5usize, 2171904u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212400));
}
#[inline]
pub fn block_0x00212400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 0usize, 14usize, 2171908u32);
    emu.sbr_no_count(29usize, 13usize, 14usize, 2171912u32);
    emu.mulhu_no_count(7usize, 14usize, 10usize, 2171916u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2171920u32);
    emu.sbr_no_count(7usize, 7usize, 13usize, 2171924u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2171928u32);
    emu.sbr_no_count(30usize, 0usize, 13usize, 2171932u32);
    emu.mulhu_no_count(31usize, 13usize, 10usize, 2171936u32);
    emu.adr_no_count(7usize, 28usize, 14usize, 2171940u32);
    emu.sltru_no_count(14usize, 7usize, 28usize, 2171944u32);
    emu.adr_no_count(13usize, 6usize, 29usize, 2171948u32);
    emu.sltru_no_count(28usize, 5usize, 30usize, 2171952u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2171956u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2171960u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2171968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212440));
    } else {
        emu.pc = 2171964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021243c));
    }
}
#[inline(always)]
pub fn block_0x0021243c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 13usize, 6usize, 2171968u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212440));
}
#[inline]
pub fn block_0x00212440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 5usize, 14usize, 2171972u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2171976u32);
    emu.adr_no_count(14usize, 16usize, 11usize, 2171980u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2171984u32);
    emu.sltru_no_count(7usize, 17usize, 7usize, 2171988u32);
    emu.sltru_no_count(16usize, 14usize, 16usize, 2171992u32);
    emu.adr_no_count(29usize, 28usize, 5usize, 2171996u32);
    emu.adr_no_count(28usize, 13usize, 7usize, 2172000u32);
    emu.adr_no_count(16usize, 15usize, 16usize, 2172004u32);
    emu.sltru_no_count(13usize, 28usize, 13usize, 2172008u32);
    emu.anr_no_count(7usize, 7usize, 13usize, 2172012u32);
    emu.sltru_no_count(5usize, 16usize, 15usize, 2172016u32);
    emu.adr_no_count(13usize, 16usize, 17usize, 2172020u32);
    emu.sltru_no_count(17usize, 13usize, 16usize, 2172024u32);
    emu.adr_no_count(16usize, 5usize, 28usize, 2172028u32);
    emu.adr_no_count(28usize, 6usize, 7usize, 2172032u32);
    emu.sltru_no_count(6usize, 28usize, 6usize, 2172036u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2172040u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2172044u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2172052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212494));
    } else {
        emu.pc = 2172048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212490));
    }
}
#[inline(always)]
pub fn block_0x00212490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 16usize, 5usize, 2172052u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212494));
}
#[inline]
pub fn block_0x00212494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(29usize, 15usize, 11usize, 2172056u32);
    emu.sbr_no_count(5usize, 0usize, 11usize, 2172060u32);
    emu.mulhu_no_count(7usize, 11usize, 10usize, 2172064u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2172068u32);
    emu.sbr_no_count(7usize, 7usize, 15usize, 2172072u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2172076u32);
    emu.sbr_no_count(30usize, 0usize, 15usize, 2172080u32);
    emu.mulhu_no_count(31usize, 15usize, 10usize, 2172084u32);
    emu.adr_no_count(7usize, 28usize, 11usize, 2172088u32);
    emu.sltru_no_count(15usize, 7usize, 28usize, 2172092u32);
    emu.adr_no_count(11usize, 6usize, 29usize, 2172096u32);
    emu.sltru_no_count(28usize, 5usize, 30usize, 2172100u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2172104u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2172108u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2172116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124d4));
    } else {
        emu.pc = 2172112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124d0));
    }
}
#[inline(always)]
pub fn block_0x002124d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 6usize, 2172116u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002124d4));
}
#[inline]
pub fn block_0x002124d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 5usize, 15usize, 2172120u32);
    emu.adr_no_count(17usize, 7usize, 17usize, 2172124u32);
    emu.adr_no_count(15usize, 16usize, 12usize, 2172128u32);
    emu.sltru_no_count(5usize, 29usize, 5usize, 2172132u32);
    emu.sltru_no_count(6usize, 17usize, 7usize, 2172136u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2172140u32);
    emu.adr_no_count(28usize, 28usize, 5usize, 2172144u32);
    emu.adr_no_count(30usize, 11usize, 6usize, 2172148u32);
    emu.adr_no_count(5usize, 14usize, 16usize, 2172152u32);
    emu.sltru_no_count(11usize, 30usize, 11usize, 2172156u32);
    emu.sltru_no_count(7usize, 5usize, 14usize, 2172160u32);
    emu.adr_no_count(16usize, 5usize, 17usize, 2172164u32);
    emu.anr_no_count(6usize, 6usize, 11usize, 2172168u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2172172u32);
    emu.adr_no_count(11usize, 7usize, 5usize, 2172176u32);
    emu.adr_no_count(11usize, 11usize, 30usize, 2172180u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2172184u32);
    emu.sltru_no_count(17usize, 6usize, 29usize, 2172188u32);
    emu.adr_no_count(17usize, 28usize, 17usize, 2172192u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2172200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212528));
    } else {
        emu.pc = 2172196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212524));
    }
}
#[inline(always)]
pub fn block_0x00212524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 7usize, 2172200u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212528));
}
#[inline]
pub fn block_0x00212528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(28usize, 14usize, 12usize, 2172204u32);
    emu.sbr_no_count(7usize, 0usize, 12usize, 2172208u32);
    emu.mulhu_no_count(29usize, 12usize, 10usize, 2172212u32);
    emu.sltru_no_count(7usize, 28usize, 7usize, 2172216u32);
    emu.sbr_no_count(29usize, 29usize, 14usize, 2172220u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2172224u32);
    emu.sbr_no_count(30usize, 0usize, 14usize, 2172228u32);
    emu.adr_no_count(29usize, 6usize, 12usize, 2172232u32);
    emu.sltru_no_count(12usize, 29usize, 6usize, 2172236u32);
    emu.adr_no_count(28usize, 17usize, 28usize, 2172240u32);
    emu.adr_no_count(28usize, 28usize, 12usize, 2172244u32);
    emu.sltru_no_count(6usize, 7usize, 30usize, 2172248u32);
    emu.mulhu_no_count(10usize, 14usize, 10usize, 2172252u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2172260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212564));
    } else {
        emu.pc = 2172256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212560));
    }
}
#[inline(always)]
pub fn block_0x00212560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 28usize, 17usize, 2172260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212564));
}
#[inline]
pub fn block_0x00212564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 10usize, 6usize, 2172264u32);
    emu.adr_no_count(17usize, 7usize, 12usize, 2172268u32);
    emu.adr_no_count(12usize, 29usize, 5usize, 2172272u32);
    emu.adi_no_count(10usize, 13usize, 1u32, 2172276u32);
    emu.sltru_no_count(30usize, 17usize, 7usize, 2172280u32);
    emu.sltru_no_count(13usize, 12usize, 29usize, 2172284u32);
    emu.sltiu_no_count(5usize, 10usize, 1u32, 2172288u32);
    emu.adr_no_count(14usize, 28usize, 13usize, 2172292u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2172296u32);
    emu.sltru_no_count(5usize, 14usize, 28usize, 2172300u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2172304u32);
    emu.anr_no_count(13usize, 13usize, 5usize, 2172308u32);
    emu.sltiu_no_count(15usize, 15usize, 1u32, 2172312u32);
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2172316u32);
    emu.adr_no_count(5usize, 16usize, 15usize, 2172320u32);
    emu.sltru_no_count(16usize, 5usize, 16usize, 2172324u32);
    emu.adr_no_count(7usize, 11usize, 15usize, 2172328u32);
    emu.adr_no_count(7usize, 7usize, 16usize, 2172332u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2172336u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2172344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125b8));
    } else {
        emu.pc = 2172340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125b4));
    }
}
#[inline(always)]
pub fn block_0x002125b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 7usize, 11usize, 2172344u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172344u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002125b8));
}
#[inline]
pub fn block_0x002125b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 6usize, 30usize, 2172348u32);
    emu.adr_no_count(16usize, 15usize, 16usize, 2172352u32);
    emu.adi_no_count(28usize, 5usize, 1u32, 2172356u32);
    emu.sltru_no_count(6usize, 16usize, 15usize, 2172360u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2172364u32);
    emu.sltiu_no_count(15usize, 28usize, 1u32, 2172368u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2172372u32);
    emu.adi_no_count(29usize, 15usize, 4294967295u32, 2172376u32);
    emu.sltru_no_count(15usize, 13usize, 17usize, 2172380u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2172392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125e8));
    } else {
        emu.pc = 2172384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125e0));
    }
}
#[inline(always)]
pub fn block_0x002125e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 29usize, 7usize, 2172388u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2172392u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002125ec));
}
#[inline(always)]
pub fn block_0x002125e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 28usize, 5usize, 2172396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002125ec));
}
#[inline]
pub fn block_0x002125ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(5usize, 16usize, 1u32, 2172400u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2172404u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2172408u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2172412u32);
    emu.sltru_no_count(16usize, 17usize, 16usize, 2172416u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2172420u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2172424u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2172428u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2172432u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2172436u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2172440u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2172444u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2172452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212624));
    } else {
        emu.pc = 2172448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212620));
    }
}
#[inline(always)]
pub fn block_0x00212620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 14usize, 16usize, 2172452u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172452u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212624));
}
#[inline]
pub fn block_0x00212624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 16usize, 12usize, 2172456u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2172460u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2172464u32);
    emu.sai_no_count(15usize, 12usize, 1055u32, 2172468u32);
    emu.adr_no_count(16usize, 11usize, 15usize, 2172472u32);
    emu.adr_no_count(14usize, 13usize, 15usize, 2172476u32);
    emu.sltru_no_count(12usize, 14usize, 13usize, 2172480u32);
    emu.adr_no_count(13usize, 16usize, 12usize, 2172484u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2172492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021264c));
    } else {
        emu.pc = 2172488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212648));
    }
}
#[inline(always)]
pub fn block_0x00212648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 11usize, 2172492u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172492u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021264c));
}
#[inline(always)]
pub fn block_0x0021264c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 15usize, 12usize, 2172496u32);
    emu.sltru_no_count(11usize, 12usize, 15usize, 2172500u32);
    emu.adr_no_count(11usize, 15usize, 11usize, 2172504u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2172524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021266c));
    } else {
        emu.pc = 2172508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021265c));
    }
}
#[inline(always)]
pub fn block_0x0021265c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 14usize, 2172512u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2172516u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2172520u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2172524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212674));
}
#[inline(always)]
pub fn block_0x0021266c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2172528u32);
    emu.sltru_no_count(13usize, 13usize, 14usize, 2172532u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2172532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212674));
}
#[inline]
pub fn block_0x00212674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 12usize, 4294967295u32, 2172536u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2172540u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2172544u32);
    emu.sltru_no_count(13usize, 13usize, 14usize, 2172548u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2172552u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2172556u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2172560u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2172564u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2172568u32);
    emu.apc_no_count(6usize, 2172568u32, 24576u32, 2172572u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2172576u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002126a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2172580u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2172584u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2172588u32);
    emu.apc_no_count(1usize, 2172588u32, 4294938624u32, 2172592u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172596u32;
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
#[inline(always)]
pub fn block_0x002126b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2172600u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2172604u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172608u32;
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
pub fn block_0x002126c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 58u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2172612u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2172616u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2172620u32)?;
    emu.sw_no_count(18usize, 2usize, 4u32, 2172624u32)?;
    emu.sbr_no_count(13usize, 0usize, 13usize, 2172628u32);
    emu.lw_no_count(14usize, 11usize, 0u32, 2172632u32)?;
    emu.lw_no_count(15usize, 11usize, 4u32, 2172636u32)?;
    emu.lw_no_count(16usize, 11usize, 8u32, 2172640u32)?;
    emu.lw_no_count(17usize, 11usize, 12u32, 2172644u32)?;
    emu.lw_no_count(5usize, 12usize, 0u32, 2172648u32)?;
    emu.lw_no_count(6usize, 12usize, 4u32, 2172652u32)?;
    emu.lw_no_count(7usize, 12usize, 8u32, 2172656u32)?;
    emu.lw_no_count(28usize, 12usize, 12u32, 2172660u32)?;
    emu.lw_no_count(29usize, 11usize, 16u32, 2172664u32)?;
    emu.lw_no_count(30usize, 11usize, 20u32, 2172668u32)?;
    emu.lw_no_count(31usize, 11usize, 24u32, 2172672u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2172676u32)?;
    emu.lw_no_count(8usize, 12usize, 16u32, 2172680u32)?;
    emu.lw_no_count(9usize, 12usize, 20u32, 2172684u32)?;
    emu.lw_no_count(18usize, 12usize, 24u32, 2172688u32)?;
    emu.lw_no_count(12usize, 12usize, 28u32, 2172692u32)?;
    emu.xrr_no_count(5usize, 5usize, 14usize, 2172696u32);
    emu.xrr_no_count(6usize, 6usize, 15usize, 2172700u32);
    emu.xrr_no_count(7usize, 7usize, 16usize, 2172704u32);
    emu.xrr_no_count(28usize, 28usize, 17usize, 2172708u32);
    emu.xrr_no_count(8usize, 8usize, 29usize, 2172712u32);
    emu.xrr_no_count(9usize, 9usize, 30usize, 2172716u32);
    emu.xrr_no_count(18usize, 18usize, 31usize, 2172720u32);
    emu.xrr_no_count(12usize, 12usize, 11usize, 2172724u32);
    emu.anr_no_count(5usize, 5usize, 13usize, 2172728u32);
    emu.anr_no_count(6usize, 6usize, 13usize, 2172732u32);
    emu.anr_no_count(7usize, 7usize, 13usize, 2172736u32);
    emu.anr_no_count(28usize, 28usize, 13usize, 2172740u32);
    emu.anr_no_count(8usize, 8usize, 13usize, 2172744u32);
    emu.anr_no_count(9usize, 9usize, 13usize, 2172748u32);
    emu.anr_no_count(18usize, 18usize, 13usize, 2172752u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2172756u32);
    emu.xrr_no_count(13usize, 5usize, 14usize, 2172760u32);
    emu.xrr_no_count(14usize, 6usize, 15usize, 2172764u32);
    emu.xrr_no_count(15usize, 7usize, 16usize, 2172768u32);
    emu.xrr_no_count(16usize, 28usize, 17usize, 2172772u32);
    emu.xrr_no_count(17usize, 8usize, 29usize, 2172776u32);
    emu.xrr_no_count(5usize, 9usize, 30usize, 2172780u32);
    emu.xrr_no_count(6usize, 18usize, 31usize, 2172784u32);
    emu.xrr_no_count(11usize, 12usize, 11usize, 2172788u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2172792u32)?;
    emu.sw_no_count(14usize, 10usize, 4u32, 2172796u32)?;
    emu.sw_no_count(15usize, 10usize, 8u32, 2172800u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2172804u32)?;
    emu.sw_no_count(17usize, 10usize, 16u32, 2172808u32)?;
    emu.sw_no_count(5usize, 10usize, 20u32, 2172812u32)?;
    emu.sw_no_count(6usize, 10usize, 24u32, 2172816u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2172820u32)?;
    emu.lw_no_count(8usize, 2usize, 12u32, 2172824u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2172828u32)?;
    emu.lw_no_count(18usize, 2usize, 4u32, 2172832u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2172836u32);
    emu.add_memory_rw_events(58usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2172840u32;
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
pub fn block_0x002127a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2172844u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2172848u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2172852u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2172856u32)?;
    emu.lw_no_count(14usize, 10usize, 8u32, 2172860u32)?;
    emu.lw_no_count(15usize, 10usize, 12u32, 2172864u32)?;
    emu.lw_no_count(16usize, 11usize, 0u32, 2172868u32)?;
    emu.lw_no_count(17usize, 11usize, 4u32, 2172872u32)?;
    emu.lw_no_count(5usize, 11usize, 8u32, 2172876u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2172880u32)?;
    emu.lw_no_count(7usize, 10usize, 16u32, 2172884u32)?;
    emu.lw_no_count(28usize, 10usize, 20u32, 2172888u32)?;
    emu.lw_no_count(29usize, 10usize, 24u32, 2172892u32)?;
    emu.lw_no_count(10usize, 10usize, 28u32, 2172896u32)?;
    emu.lw_no_count(30usize, 11usize, 16u32, 2172900u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2172904u32)?;
    emu.lw_no_count(8usize, 11usize, 24u32, 2172908u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2172912u32)?;
    emu.xrr_no_count(13usize, 17usize, 13usize, 2172916u32);
    emu.xrr_no_count(12usize, 16usize, 12usize, 2172920u32);
    emu.xrr_no_count(14usize, 5usize, 14usize, 2172924u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2172928u32);
    emu.xrr_no_count(16usize, 30usize, 7usize, 2172932u32);
    emu.xrr_no_count(17usize, 31usize, 28usize, 2172936u32);
    emu.xrr_no_count(5usize, 8usize, 29usize, 2172940u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2172944u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2172948u32);
    emu.orr_no_count(13usize, 16usize, 17usize, 2172952u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2172956u32);
    emu.orr_no_count(13usize, 13usize, 5usize, 2172960u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2172964u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2172968u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2172972u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2172976u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2172980u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2172984u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2172988u32);
    emu.apc_no_count(6usize, 2172988u32, 24576u32, 2172992u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2172996u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2173000u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2173004u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2173008u32);
    emu.apc_no_count(1usize, 2173008u32, 4294938624u32, 2173012u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173016u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2173020u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2173024u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173028u32;
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
pub fn block_0x00212864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2173032u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2173036u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2173040u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2173044u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2173048u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2173052u32);
    emu.apc_no_count(1usize, 2173052u32, 4294950912u32, 2173056u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00212884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2173064u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2173068u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2173072u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2173076u32)?;
    emu.lw_no_count(14usize, 8usize, 16u32, 2173080u32)?;
    emu.lw_no_count(15usize, 8usize, 20u32, 2173084u32)?;
    emu.lw_no_count(16usize, 8usize, 24u32, 2173088u32)?;
    emu.lw_no_count(17usize, 8usize, 28u32, 2173092u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2173096u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2173100u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2173104u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2173108u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2173112u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2173116u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2173120u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2173124u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2173128u32);
    emu.apc_no_count(1usize, 2173128u32, 24576u32, 2173132u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002128d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2173140u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2173144u32);
    emu.apc_no_count(1usize, 2173144u32, 24576u32, 2173148u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173152u32;
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
pub fn block_0x002128e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 9usize, 32u32, 2173156u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2173160u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2173164u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2173168u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2173172u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173176u32;
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
pub fn block_0x002128f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2173180u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2173184u32)?;
    emu.apc_no_count(1usize, 2173184u32, 4294959104u32, 2173188u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173192u32;
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
#[inline(always)]
pub fn block_0x00212908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2173196u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2173200u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2173204u32;
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
pub fn block_0x00212914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 44u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2173208u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2173212u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2173216u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2173220u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2173224u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2173228u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2173232u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2173236u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2173240u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2173244u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2173248u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2173252u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2173256u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2173260u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2173264u32)?;
    emu.adi_no_count(15usize, 0usize, 0u32, 2173268u32);
    emu.adi_no_count(16usize, 2usize, 16u32, 2173272u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2173276u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2173280u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2173284u32)?;
    emu.sw_no_count(0usize, 2usize, 40u32, 2173288u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2173292u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2173296u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2173300u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2173304u32)?;
    emu.adi_no_count(17usize, 2usize, 24u32, 2173308u32);
    emu.sw_no_count(0usize, 2usize, 44u32, 2173312u32)?;
    emu.sw_no_count(0usize, 2usize, 48u32, 2173316u32)?;
    emu.sw_no_count(0usize, 2usize, 52u32, 2173320u32)?;
    emu.sw_no_count(0usize, 2usize, 56u32, 2173324u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2173328u32)?;
    emu.sw_no_count(0usize, 2usize, 64u32, 2173332u32)?;
    emu.sw_no_count(0usize, 2usize, 68u32, 2173336u32)?;
    emu.sw_no_count(0usize, 2usize, 72u32, 2173340u32)?;
    emu.adi_no_count(5usize, 2usize, 12u32, 2173344u32);
    emu.lw_no_count(10usize, 12usize, 0u32, 2173348u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2173352u32)?;
    emu.lw_no_count(7usize, 12usize, 4u32, 2173356u32)?;
    emu.lw_no_count(28usize, 12usize, 8u32, 2173360u32)?;
    emu.lw_no_count(29usize, 12usize, 12u32, 2173364u32)?;
    emu.lw_no_count(30usize, 12usize, 16u32, 2173368u32)?;
    emu.lw_no_count(31usize, 12usize, 20u32, 2173372u32)?;
    emu.lw_no_count(8usize, 12usize, 24u32, 2173376u32)?;
    emu.lw_no_count(12usize, 12usize, 28u32, 2173380u32)?;
    emu.add_memory_rw_events(44usize);
    emu.pc = 2173380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002129c4));
}
#[inline(never)]
pub fn block_0x002129c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 11usize, 0u32, 2173384u32)?;
    emu.lw_no_count(10usize, 17usize, 4294967284u32, 2173388u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2173392u32)?;
    emu.mulhu_no_count(14usize, 18usize, 13usize, 2173396u32);
    emu.mul_no_count(13usize, 18usize, 13usize, 2173400u32);
    emu.mulhu_no_count(19usize, 18usize, 7usize, 2173404u32);
    emu.mul_no_count(9usize, 18usize, 7usize, 2173408u32);
    emu.mulhu_no_count(24usize, 18usize, 12usize, 2173412u32);
    emu.mul_no_count(25usize, 18usize, 12usize, 2173416u32);
    emu.mulhu_no_count(26usize, 18usize, 8usize, 2173420u32);
    emu.mul_no_count(27usize, 18usize, 8usize, 2173424u32);
    emu.mulhu_no_count(1usize, 18usize, 31usize, 2173428u32);
    emu.mul_no_count(23usize, 18usize, 31usize, 2173432u32);
    emu.mulhu_no_count(22usize, 18usize, 30usize, 2173436u32);
    emu.mul_no_count(21usize, 18usize, 30usize, 2173440u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2173444u32);
    emu.sltru_no_count(20usize, 10usize, 13usize, 2173448u32);
    emu.sw_no_count(10usize, 17usize, 4294967284u32, 2173452u32)?;
    emu.mulhu_no_count(13usize, 18usize, 29usize, 2173456u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2173460u32);
    emu.mul_no_count(14usize, 18usize, 29usize, 2173464u32);
    emu.adr_no_count(9usize, 20usize, 9usize, 2173468u32);
    emu.sltru_no_count(10usize, 9usize, 20usize, 2173472u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2173476u32);
    emu.mulhu_no_count(20usize, 18usize, 28usize, 2173480u32);
    emu.mul_no_count(19usize, 18usize, 28usize, 2173484u32);
    emu.adi_no_count(6usize, 0usize, 7u32, 2173488u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2173520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a50));
    } else {
        emu.pc = 2173492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a34));
    }
}
#[inline(always)]
pub fn block_0x00212a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 16usize, 0u32, 2173496u32)?;
    emu.adi_no_count(18usize, 15usize, 2u32, 2173500u32);
    emu.adr_no_count(6usize, 9usize, 6usize, 2173504u32);
    emu.sltru_no_count(9usize, 6usize, 9usize, 2173508u32);
    emu.adr_no_count(9usize, 10usize, 9usize, 2173512u32);
    emu.sw_no_count(6usize, 16usize, 0u32, 2173516u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2173520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a70));
}
#[inline(always)]
pub fn block_0x00212a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 17usize, 4294967288u32, 2173524u32)?;
    emu.adr_no_count(6usize, 9usize, 6usize, 2173528u32);
    emu.sltru_no_count(9usize, 6usize, 9usize, 2173532u32);
    emu.adr_no_count(9usize, 10usize, 9usize, 2173536u32);
    emu.sw_no_count(6usize, 17usize, 4294967288u32, 2173540u32)?;
    emu.adi_no_count(10usize, 0usize, 6u32, 2173544u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2173844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b94));
    } else {
        emu.pc = 2173548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212a6c));
    }
}
#[inline(always)]
pub fn block_0x00212a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 8u32, 2173552u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2173552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a70));
}
#[inline]
pub fn block_0x00212a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(18usize, 18usize, 2u32, 2173556u32);
    emu.adr_no_count(18usize, 5usize, 18usize, 2173560u32);
    emu.lw_no_count(10usize, 18usize, 0u32, 2173564u32)?;
    emu.adr_no_count(9usize, 19usize, 9usize, 2173568u32);
    emu.sltru_no_count(6usize, 9usize, 19usize, 2173572u32);
    emu.adr_no_count(6usize, 20usize, 6usize, 2173576u32);
    emu.adr_no_count(10usize, 9usize, 10usize, 2173580u32);
    emu.sltru_no_count(9usize, 10usize, 9usize, 2173584u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2173588u32)?;
    emu.adr_no_count(9usize, 6usize, 9usize, 2173592u32);
    emu.adi_no_count(10usize, 15usize, 3u32, 2173596u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2173596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a9c));
}
#[inline]
pub fn block_0x00212a9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2173600u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2173604u32);
    emu.lw_no_count(6usize, 10usize, 0u32, 2173608u32)?;
    emu.adr_no_count(9usize, 14usize, 9usize, 2173612u32);
    emu.sltru_no_count(14usize, 9usize, 14usize, 2173616u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2173620u32);
    emu.adr_no_count(6usize, 9usize, 6usize, 2173624u32);
    emu.sltru_no_count(14usize, 6usize, 9usize, 2173628u32);
    emu.sw_no_count(6usize, 10usize, 0u32, 2173632u32)?;
    emu.adr_no_count(13usize, 13usize, 14usize, 2173636u32);
    emu.adi_no_count(10usize, 15usize, 4u32, 2173640u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2173640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212ac8));
}
#[inline]
pub fn block_0x00212ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2173644u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2173648u32);
    emu.lw_no_count(14usize, 10usize, 0u32, 2173652u32)?;
    emu.adr_no_count(13usize, 21usize, 13usize, 2173656u32);
    emu.sltru_no_count(6usize, 13usize, 21usize, 2173660u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2173664u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2173668u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2173672u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2173676u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2173680u32);
    emu.adi_no_count(10usize, 15usize, 5u32, 2173684u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2173684u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212af4));
}
#[inline]
pub fn block_0x00212af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2173688u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2173692u32);
    emu.lw_no_count(14usize, 10usize, 0u32, 2173696u32)?;
    emu.adr_no_count(13usize, 23usize, 13usize, 2173700u32);
    emu.sltru_no_count(6usize, 13usize, 23usize, 2173704u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2173708u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2173712u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2173716u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2173720u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2173724u32);
    emu.adi_no_count(10usize, 15usize, 6u32, 2173728u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2173728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b20));
}
#[inline]
pub fn block_0x00212b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2173732u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2173736u32);
    emu.lw_no_count(14usize, 10usize, 0u32, 2173740u32)?;
    emu.adr_no_count(13usize, 27usize, 13usize, 2173744u32);
    emu.sltru_no_count(6usize, 13usize, 27usize, 2173748u32);
    emu.adr_no_count(6usize, 26usize, 6usize, 2173752u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2173756u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2173760u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2173764u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2173768u32);
    emu.adi_no_count(10usize, 15usize, 7u32, 2173772u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2173772u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b4c));
}
#[inline]
pub fn block_0x00212b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2173776u32);
    emu.adr_no_count(14usize, 5usize, 10usize, 2173780u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2173784u32)?;
    emu.adr_no_count(13usize, 25usize, 13usize, 2173788u32);
    emu.sltru_no_count(6usize, 13usize, 25usize, 2173792u32);
    emu.adr_no_count(6usize, 24usize, 6usize, 2173796u32);
    emu.adr_no_count(9usize, 13usize, 10usize, 2173800u32);
    emu.sltru_no_count(10usize, 9usize, 13usize, 2173804u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2173808u32);
    emu.sw_no_count(9usize, 14usize, 0u32, 2173812u32)?;
    emu.add_memory_rw_events(10usize);
    emu.pc = 2173812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b74));
}
#[inline(always)]
pub fn block_0x00212b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 1u32, 2173816u32);
    emu.sw_no_count(10usize, 16usize, 28u32, 2173820u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2173824u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2173828u32);
    emu.adi_no_count(17usize, 17usize, 4u32, 2173832u32);
    emu.adi_no_count(10usize, 0usize, 8u32, 2173836u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2173380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002129c4));
    } else {
        emu.pc = 2173840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212b90));
    }
}
#[inline(always)]
pub fn block_0x00212b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2173844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2174116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212ca4));
}
#[inline]
pub fn block_0x00212b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 4294967292u32, 2173848u32)?;
    emu.adr_no_count(19usize, 9usize, 19usize, 2173852u32);
    emu.sltru_no_count(6usize, 19usize, 9usize, 2173856u32);
    emu.adr_no_count(6usize, 20usize, 6usize, 2173860u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2173864u32);
    emu.sltru_no_count(9usize, 10usize, 19usize, 2173868u32);
    emu.adr_no_count(9usize, 6usize, 9usize, 2173872u32);
    emu.sw_no_count(10usize, 17usize, 4294967292u32, 2173876u32)?;
    emu.adi_no_count(10usize, 0usize, 5u32, 2173880u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2173892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bc4));
    } else {
        emu.pc = 2173884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bbc));
    }
}
#[inline(always)]
pub fn block_0x00212bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2173888u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212a9c));
}
#[inline]
pub fn block_0x00212bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 0u32, 2173896u32)?;
    emu.adr_no_count(14usize, 9usize, 14usize, 2173900u32);
    emu.sltru_no_count(6usize, 14usize, 9usize, 2173904u32);
    emu.adr_no_count(13usize, 13usize, 6usize, 2173908u32);
    emu.adr_no_count(10usize, 14usize, 10usize, 2173912u32);
    emu.sltru_no_count(14usize, 10usize, 14usize, 2173916u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2173920u32);
    emu.sw_no_count(10usize, 17usize, 0u32, 2173924u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2173928u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2173940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bf4));
    } else {
        emu.pc = 2173932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212bec));
    }
}
#[inline(always)]
pub fn block_0x00212bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2173936u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173640u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212ac8));
}
#[inline]
pub fn block_0x00212bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 4u32, 2173944u32)?;
    emu.adr_no_count(21usize, 13usize, 21usize, 2173948u32);
    emu.sltru_no_count(13usize, 21usize, 13usize, 2173952u32);
    emu.adr_no_count(13usize, 22usize, 13usize, 2173956u32);
    emu.adr_no_count(10usize, 21usize, 10usize, 2173960u32);
    emu.sltru_no_count(14usize, 10usize, 21usize, 2173964u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2173968u32);
    emu.sw_no_count(10usize, 17usize, 4u32, 2173972u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2173976u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2173988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c24));
    } else {
        emu.pc = 2173980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c1c));
    }
}
#[inline(always)]
pub fn block_0x00212c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2173984u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2173988u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173684u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212af4));
}
#[inline]
pub fn block_0x00212c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 8u32, 2173992u32)?;
    emu.adr_no_count(23usize, 13usize, 23usize, 2173996u32);
    emu.sltru_no_count(13usize, 23usize, 13usize, 2174000u32);
    emu.adr_no_count(13usize, 1usize, 13usize, 2174004u32);
    emu.adr_no_count(10usize, 23usize, 10usize, 2174008u32);
    emu.sltru_no_count(14usize, 10usize, 23usize, 2174012u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2174016u32);
    emu.sw_no_count(10usize, 17usize, 8u32, 2174020u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2174024u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2174036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c54));
    } else {
        emu.pc = 2174028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c4c));
    }
}
#[inline(always)]
pub fn block_0x00212c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2174032u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2174036u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b20));
}
#[inline]
pub fn block_0x00212c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 12u32, 2174040u32)?;
    emu.adr_no_count(27usize, 13usize, 27usize, 2174044u32);
    emu.sltru_no_count(13usize, 27usize, 13usize, 2174048u32);
    emu.adr_no_count(13usize, 26usize, 13usize, 2174052u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2174056u32);
    emu.sltru_no_count(14usize, 10usize, 27usize, 2174060u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2174064u32);
    emu.sw_no_count(10usize, 17usize, 12u32, 2174068u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2174080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c80));
    } else {
        emu.pc = 2174072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212c78));
    }
}
#[inline(always)]
pub fn block_0x00212c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2174076u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2174080u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b4c));
}
#[inline]
pub fn block_0x00212c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 40u32, 2174084u32)?;
    emu.adr_no_count(25usize, 13usize, 25usize, 2174088u32);
    emu.sltru_no_count(13usize, 25usize, 13usize, 2174092u32);
    emu.adr_no_count(13usize, 24usize, 13usize, 2174096u32);
    emu.adr_no_count(14usize, 25usize, 10usize, 2174100u32);
    emu.sltru_no_count(10usize, 14usize, 25usize, 2174104u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2174108u32);
    emu.sw_no_count(14usize, 2usize, 40u32, 2174112u32)?;
    emu.add_memory_rw_events(9usize);
    let return_addr = 2174116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2173812u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212b74));
}
#[inline(never)]
pub fn block_0x00212ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 50u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 12u32, 2174120u32);
    emu.lw_no_count(10usize, 15usize, 16u32, 2174124u32)?;
    emu.lw_no_count(11usize, 15usize, 20u32, 2174128u32)?;
    emu.lw_no_count(12usize, 15usize, 24u32, 2174132u32)?;
    emu.lw_no_count(13usize, 15usize, 28u32, 2174136u32)?;
    emu.lw_no_count(14usize, 2usize, 4u32, 2174140u32)?;
    emu.sw_no_count(10usize, 14usize, 16u32, 2174144u32)?;
    emu.sw_no_count(11usize, 14usize, 20u32, 2174148u32)?;
    emu.sw_no_count(12usize, 14usize, 24u32, 2174152u32)?;
    emu.sw_no_count(13usize, 14usize, 28u32, 2174156u32)?;
    emu.lw_no_count(10usize, 15usize, 0u32, 2174160u32)?;
    emu.lw_no_count(11usize, 15usize, 4u32, 2174164u32)?;
    emu.lw_no_count(12usize, 15usize, 8u32, 2174168u32)?;
    emu.lw_no_count(13usize, 15usize, 12u32, 2174172u32)?;
    emu.sw_no_count(10usize, 14usize, 0u32, 2174176u32)?;
    emu.sw_no_count(11usize, 14usize, 4u32, 2174180u32)?;
    emu.sw_no_count(12usize, 14usize, 8u32, 2174184u32)?;
    emu.sw_no_count(13usize, 14usize, 12u32, 2174188u32)?;
    emu.adi_no_count(15usize, 2usize, 44u32, 2174192u32);
    emu.lw_no_count(10usize, 15usize, 0u32, 2174196u32)?;
    emu.lw_no_count(11usize, 15usize, 4u32, 2174200u32)?;
    emu.lw_no_count(12usize, 15usize, 8u32, 2174204u32)?;
    emu.lw_no_count(13usize, 15usize, 12u32, 2174208u32)?;
    emu.sw_no_count(10usize, 14usize, 32u32, 2174212u32)?;
    emu.sw_no_count(11usize, 14usize, 36u32, 2174216u32)?;
    emu.sw_no_count(12usize, 14usize, 40u32, 2174220u32)?;
    emu.sw_no_count(13usize, 14usize, 44u32, 2174224u32)?;
    emu.lw_no_count(10usize, 15usize, 16u32, 2174228u32)?;
    emu.lw_no_count(11usize, 15usize, 20u32, 2174232u32)?;
    emu.lw_no_count(12usize, 15usize, 24u32, 2174236u32)?;
    emu.lw_no_count(13usize, 15usize, 28u32, 2174240u32)?;
    emu.sw_no_count(10usize, 14usize, 48u32, 2174244u32)?;
    emu.sw_no_count(11usize, 14usize, 52u32, 2174248u32)?;
    emu.sw_no_count(12usize, 14usize, 56u32, 2174252u32)?;
    emu.sw_no_count(13usize, 14usize, 60u32, 2174256u32)?;
    emu.lw_no_count(1usize, 2usize, 124u32, 2174260u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2174264u32)?;
    emu.lw_no_count(9usize, 2usize, 116u32, 2174268u32)?;
    emu.lw_no_count(18usize, 2usize, 112u32, 2174272u32)?;
    emu.lw_no_count(19usize, 2usize, 108u32, 2174276u32)?;
    emu.lw_no_count(20usize, 2usize, 104u32, 2174280u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2174284u32)?;
    emu.lw_no_count(22usize, 2usize, 96u32, 2174288u32)?;
    emu.lw_no_count(23usize, 2usize, 92u32, 2174292u32)?;
    emu.lw_no_count(24usize, 2usize, 88u32, 2174296u32)?;
    emu.lw_no_count(25usize, 2usize, 84u32, 2174300u32)?;
    emu.lw_no_count(26usize, 2usize, 80u32, 2174304u32)?;
    emu.lw_no_count(27usize, 2usize, 76u32, 2174308u32)?;
    emu.adi_no_count(2usize, 2usize, 128u32, 2174312u32);
    emu.add_memory_rw_events(50usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2174316u32;
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
pub fn block_0x00212d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 184u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2174320u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2174324u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2174328u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2174332u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2174336u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2174340u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2174344u32)?;
    emu.sw_no_count(22usize, 2usize, 4u32, 2174348u32)?;
    emu.sw_no_count(23usize, 2usize, 0u32, 2174352u32)?;
    emu.lw_no_count(29usize, 11usize, 0u32, 2174356u32)?;
    emu.lw_no_count(9usize, 11usize, 4u32, 2174360u32)?;
    emu.lw_no_count(6usize, 11usize, 8u32, 2174364u32)?;
    emu.lw_no_count(30usize, 11usize, 12u32, 2174368u32)?;
    emu.lw_no_count(20usize, 12usize, 0u32, 2174372u32)?;
    emu.lw_no_count(18usize, 12usize, 4u32, 2174376u32)?;
    emu.lw_no_count(31usize, 12usize, 8u32, 2174380u32)?;
    emu.lw_no_count(8usize, 12usize, 12u32, 2174384u32)?;
    emu.lw_no_count(19usize, 11usize, 16u32, 2174388u32)?;
    emu.lw_no_count(28usize, 11usize, 20u32, 2174392u32)?;
    emu.lw_no_count(5usize, 11usize, 24u32, 2174396u32)?;
    emu.lw_no_count(7usize, 11usize, 28u32, 2174400u32)?;
    emu.lw_no_count(14usize, 13usize, 0u32, 2174404u32)?;
    emu.lw_no_count(15usize, 13usize, 4u32, 2174408u32)?;
    emu.lw_no_count(16usize, 13usize, 8u32, 2174412u32)?;
    emu.lw_no_count(17usize, 13usize, 12u32, 2174416u32)?;
    emu.adr_no_count(29usize, 20usize, 29usize, 2174420u32);
    emu.sltru_no_count(11usize, 29usize, 20usize, 2174424u32);
    emu.sltru_no_count(20usize, 29usize, 14usize, 2174428u32);
    emu.adr_no_count(9usize, 11usize, 9usize, 2174432u32);
    emu.sltru_no_count(11usize, 9usize, 11usize, 2174436u32);
    emu.adr_no_count(18usize, 9usize, 18usize, 2174440u32);
    emu.sltru_no_count(9usize, 18usize, 9usize, 2174444u32);
    emu.sbr_no_count(21usize, 18usize, 15usize, 2174448u32);
    emu.sltru_no_count(18usize, 18usize, 15usize, 2174452u32);
    emu.adr_no_count(22usize, 11usize, 9usize, 2174456u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2174460u32);
    emu.sbr_no_count(11usize, 21usize, 20usize, 2174464u32);
    emu.sltru_no_count(9usize, 11usize, 21usize, 2174468u32);
    emu.sbr_no_count(21usize, 9usize, 18usize, 2174472u32);
    emu.lw_no_count(23usize, 12usize, 16u32, 2174476u32)?;
    emu.lw_no_count(20usize, 12usize, 20u32, 2174480u32)?;
    emu.lw_no_count(18usize, 12usize, 24u32, 2174484u32)?;
    emu.lw_no_count(9usize, 12usize, 28u32, 2174488u32)?;
    emu.adr_no_count(6usize, 31usize, 6usize, 2174492u32);
    emu.adr_no_count(30usize, 8usize, 30usize, 2174496u32);
    emu.adr_no_count(19usize, 23usize, 19usize, 2174500u32);
    emu.sltru_no_count(12usize, 6usize, 31usize, 2174504u32);
    emu.sltru_no_count(31usize, 30usize, 8usize, 2174508u32);
    emu.sltru_no_count(8usize, 19usize, 23usize, 2174512u32);
    emu.adr_no_count(22usize, 6usize, 22usize, 2174516u32);
    emu.sltru_no_count(6usize, 22usize, 6usize, 2174520u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2174524u32);
    emu.sbr_no_count(6usize, 22usize, 16usize, 2174528u32);
    emu.sltru_no_count(22usize, 22usize, 16usize, 2174532u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2174536u32);
    emu.adr_no_count(23usize, 30usize, 12usize, 2174540u32);
    emu.sbr_no_count(22usize, 21usize, 22usize, 2174544u32);
    emu.adr_no_count(12usize, 6usize, 21usize, 2174548u32);
    emu.sltru_no_count(30usize, 23usize, 30usize, 2174552u32);
    emu.sltru_no_count(6usize, 12usize, 6usize, 2174556u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2174560u32);
    emu.sbr_no_count(31usize, 23usize, 17usize, 2174564u32);
    emu.sltru_no_count(21usize, 23usize, 17usize, 2174568u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2174572u32);
    emu.adr_no_count(22usize, 19usize, 30usize, 2174576u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2174580u32);
    emu.sltru_no_count(30usize, 22usize, 19usize, 2174584u32);
    emu.sbr_no_count(19usize, 6usize, 21usize, 2174588u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2174592u32);
    emu.adr_no_count(21usize, 8usize, 30usize, 2174596u32);
    emu.sltru_no_count(30usize, 6usize, 31usize, 2174600u32);
    emu.adr_no_count(23usize, 19usize, 30usize, 2174604u32);
    emu.lw_no_count(30usize, 13usize, 16u32, 2174608u32)?;
    emu.lw_no_count(31usize, 13usize, 20u32, 2174612u32)?;
    emu.lw_no_count(8usize, 13usize, 24u32, 2174616u32)?;
    emu.lw_no_count(19usize, 13usize, 28u32, 2174620u32)?;
    emu.adr_no_count(28usize, 20usize, 28usize, 2174624u32);
    emu.sltru_no_count(13usize, 28usize, 20usize, 2174628u32);
    emu.adr_no_count(21usize, 28usize, 21usize, 2174632u32);
    emu.sltru_no_count(28usize, 21usize, 28usize, 2174636u32);
    emu.adr_no_count(28usize, 13usize, 28usize, 2174640u32);
    emu.sbr_no_count(20usize, 22usize, 30usize, 2174644u32);
    emu.sltru_no_count(13usize, 22usize, 30usize, 2174648u32);
    emu.sai_no_count(22usize, 23usize, 1055u32, 2174652u32);
    emu.sbr_no_count(23usize, 22usize, 13usize, 2174656u32);
    emu.adr_no_count(13usize, 20usize, 22usize, 2174660u32);
    emu.sltru_no_count(20usize, 13usize, 20usize, 2174664u32);
    emu.adr_no_count(20usize, 23usize, 20usize, 2174668u32);
    emu.adr_no_count(5usize, 18usize, 5usize, 2174672u32);
    emu.sltru_no_count(18usize, 5usize, 18usize, 2174676u32);
    emu.adr_no_count(28usize, 5usize, 28usize, 2174680u32);
    emu.sltru_no_count(5usize, 28usize, 5usize, 2174684u32);
    emu.adr_no_count(18usize, 18usize, 5usize, 2174688u32);
    emu.adr_no_count(7usize, 9usize, 7usize, 2174692u32);
    emu.sltru_no_count(9usize, 7usize, 9usize, 2174696u32);
    emu.sbr_no_count(5usize, 29usize, 14usize, 2174700u32);
    emu.adr_no_count(18usize, 7usize, 18usize, 2174704u32);
    emu.sltru_no_count(7usize, 18usize, 7usize, 2174708u32);
    emu.adr_no_count(9usize, 9usize, 7usize, 2174712u32);
    emu.sbr_no_count(29usize, 21usize, 31usize, 2174716u32);
    emu.sltru_no_count(7usize, 21usize, 31usize, 2174720u32);
    emu.sai_no_count(20usize, 20usize, 1055u32, 2174724u32);
    emu.sbr_no_count(21usize, 20usize, 7usize, 2174728u32);
    emu.adr_no_count(7usize, 29usize, 20usize, 2174732u32);
    emu.sltru_no_count(29usize, 7usize, 29usize, 2174736u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2174740u32);
    emu.sbr_no_count(20usize, 28usize, 8usize, 2174744u32);
    emu.sltru_no_count(28usize, 28usize, 8usize, 2174748u32);
    emu.sai_no_count(29usize, 29usize, 1055u32, 2174752u32);
    emu.sbr_no_count(21usize, 29usize, 28usize, 2174756u32);
    emu.adr_no_count(28usize, 20usize, 29usize, 2174760u32);
    emu.sltru_no_count(29usize, 28usize, 20usize, 2174764u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2174768u32);
    emu.sbr_no_count(20usize, 18usize, 19usize, 2174772u32);
    emu.sltru_no_count(18usize, 18usize, 19usize, 2174776u32);
    emu.sai_no_count(29usize, 29usize, 1055u32, 2174780u32);
    emu.sbr_no_count(18usize, 29usize, 18usize, 2174784u32);
    emu.adr_no_count(29usize, 20usize, 29usize, 2174788u32);
    emu.sltru_no_count(20usize, 29usize, 20usize, 2174792u32);
    emu.adr_no_count(18usize, 18usize, 20usize, 2174796u32);
    emu.sai_no_count(18usize, 18usize, 1055u32, 2174800u32);
    emu.adr_no_count(9usize, 18usize, 9usize, 2174804u32);
    emu.sltru_no_count(9usize, 9usize, 18usize, 2174808u32);
    emu.adr_no_count(9usize, 18usize, 9usize, 2174812u32);
    emu.anr_no_count(14usize, 14usize, 9usize, 2174816u32);
    emu.anr_no_count(15usize, 15usize, 9usize, 2174820u32);
    emu.anr_no_count(16usize, 16usize, 9usize, 2174824u32);
    emu.anr_no_count(17usize, 17usize, 9usize, 2174828u32);
    emu.anr_no_count(30usize, 30usize, 9usize, 2174832u32);
    emu.anr_no_count(31usize, 31usize, 9usize, 2174836u32);
    emu.anr_no_count(8usize, 8usize, 9usize, 2174840u32);
    emu.anr_no_count(9usize, 19usize, 9usize, 2174844u32);
    emu.adr_no_count(14usize, 5usize, 14usize, 2174848u32);
    emu.adr_no_count(15usize, 11usize, 15usize, 2174852u32);
    emu.adr_no_count(16usize, 12usize, 16usize, 2174856u32);
    emu.adr_no_count(17usize, 6usize, 17usize, 2174860u32);
    emu.adr_no_count(30usize, 13usize, 30usize, 2174864u32);
    emu.adr_no_count(31usize, 7usize, 31usize, 2174868u32);
    emu.adr_no_count(8usize, 28usize, 8usize, 2174872u32);
    emu.sltru_no_count(5usize, 14usize, 5usize, 2174876u32);
    emu.sltru_no_count(11usize, 15usize, 11usize, 2174880u32);
    emu.sltru_no_count(12usize, 16usize, 12usize, 2174884u32);
    emu.sltru_no_count(6usize, 17usize, 6usize, 2174888u32);
    emu.sltru_no_count(13usize, 30usize, 13usize, 2174892u32);
    emu.sltru_no_count(7usize, 31usize, 7usize, 2174896u32);
    emu.sltru_no_count(28usize, 8usize, 28usize, 2174900u32);
    emu.adr_no_count(5usize, 15usize, 5usize, 2174904u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2174908u32);
    emu.sltru_no_count(15usize, 5usize, 15usize, 2174912u32);
    emu.adr_no_count(28usize, 9usize, 28usize, 2174916u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2174920u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2174924u32);
    emu.sltru_no_count(15usize, 11usize, 16usize, 2174928u32);
    emu.adr_no_count(12usize, 12usize, 15usize, 2174932u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2174936u32);
    emu.sltru_no_count(15usize, 12usize, 17usize, 2174940u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2174944u32)?;
    emu.sw_no_count(5usize, 10usize, 4u32, 2174948u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2174952u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2174956u32)?;
    emu.adr_no_count(15usize, 6usize, 15usize, 2174960u32);
    emu.adr_no_count(15usize, 30usize, 15usize, 2174964u32);
    emu.sltru_no_count(11usize, 15usize, 30usize, 2174968u32);
    emu.adr_no_count(11usize, 13usize, 11usize, 2174972u32);
    emu.adr_no_count(11usize, 31usize, 11usize, 2174976u32);
    emu.sltru_no_count(12usize, 11usize, 31usize, 2174980u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2174984u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2174988u32);
    emu.sltru_no_count(13usize, 12usize, 8usize, 2174992u32);
    emu.adr_no_count(13usize, 28usize, 13usize, 2174996u32);
    emu.sw_no_count(15usize, 10usize, 16u32, 2175000u32)?;
    emu.sw_no_count(11usize, 10usize, 20u32, 2175004u32)?;
    emu.sw_no_count(12usize, 10usize, 24u32, 2175008u32)?;
    emu.sw_no_count(13usize, 10usize, 28u32, 2175012u32)?;
    emu.lw_no_count(8usize, 2usize, 28u32, 2175016u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2175020u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2175024u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2175028u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2175032u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2175036u32)?;
    emu.lw_no_count(22usize, 2usize, 4u32, 2175040u32)?;
    emu.lw_no_count(23usize, 2usize, 0u32, 2175044u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2175048u32);
    emu.add_memory_rw_events(184usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175052u32;
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
pub fn block_0x0021304c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 140u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2175056u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2175060u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2175064u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2175068u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2175072u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2175076u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2175080u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2175084u32)?;
    emu.lw_no_count(14usize, 11usize, 4u32, 2175088u32)?;
    emu.lw_no_count(17usize, 11usize, 8u32, 2175092u32)?;
    emu.lw_no_count(5usize, 11usize, 12u32, 2175096u32)?;
    emu.lw_no_count(6usize, 12usize, 0u32, 2175100u32)?;
    emu.lw_no_count(16usize, 12usize, 4u32, 2175104u32)?;
    emu.lw_no_count(7usize, 12usize, 8u32, 2175108u32)?;
    emu.lw_no_count(28usize, 12usize, 12u32, 2175112u32)?;
    emu.lw_no_count(29usize, 11usize, 16u32, 2175116u32)?;
    emu.lw_no_count(30usize, 11usize, 20u32, 2175120u32)?;
    emu.lw_no_count(31usize, 11usize, 24u32, 2175124u32)?;
    emu.lw_no_count(8usize, 11usize, 28u32, 2175128u32)?;
    emu.lw_no_count(9usize, 12usize, 16u32, 2175132u32)?;
    emu.lw_no_count(18usize, 12usize, 20u32, 2175136u32)?;
    emu.lw_no_count(19usize, 12usize, 24u32, 2175140u32)?;
    emu.lw_no_count(12usize, 12usize, 28u32, 2175144u32)?;
    emu.sbr_no_count(11usize, 15usize, 6usize, 2175148u32);
    emu.sltru_no_count(15usize, 15usize, 6usize, 2175152u32);
    emu.sbr_no_count(6usize, 17usize, 7usize, 2175156u32);
    emu.sltru_no_count(17usize, 17usize, 7usize, 2175160u32);
    emu.sbr_no_count(7usize, 5usize, 28usize, 2175164u32);
    emu.sltru_no_count(5usize, 5usize, 28usize, 2175168u32);
    emu.sbr_no_count(28usize, 29usize, 9usize, 2175172u32);
    emu.sltru_no_count(29usize, 29usize, 9usize, 2175176u32);
    emu.sbr_no_count(9usize, 30usize, 18usize, 2175180u32);
    emu.sltru_no_count(30usize, 30usize, 18usize, 2175184u32);
    emu.sbr_no_count(18usize, 31usize, 19usize, 2175188u32);
    emu.sltru_no_count(31usize, 31usize, 19usize, 2175192u32);
    emu.sbr_no_count(19usize, 8usize, 12usize, 2175196u32);
    emu.sltru_no_count(8usize, 8usize, 12usize, 2175200u32);
    emu.sbr_no_count(12usize, 0usize, 15usize, 2175204u32);
    emu.sbr_no_count(14usize, 14usize, 15usize, 2175208u32);
    emu.sltru_no_count(20usize, 14usize, 12usize, 2175212u32);
    emu.sltru_no_count(21usize, 14usize, 16usize, 2175216u32);
    emu.sbr_no_count(12usize, 14usize, 16usize, 2175220u32);
    emu.sbr_no_count(14usize, 20usize, 15usize, 2175224u32);
    emu.sbr_no_count(14usize, 14usize, 21usize, 2175228u32);
    emu.sai_no_count(14usize, 14usize, 1055u32, 2175232u32);
    emu.sbr_no_count(15usize, 14usize, 17usize, 2175236u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2175240u32);
    emu.sltru_no_count(16usize, 14usize, 6usize, 2175244u32);
    emu.adr_no_count(15usize, 15usize, 16usize, 2175248u32);
    emu.sai_no_count(15usize, 15usize, 1055u32, 2175252u32);
    emu.sbr_no_count(16usize, 15usize, 5usize, 2175256u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2175260u32);
    emu.sltru_no_count(17usize, 15usize, 7usize, 2175264u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2175268u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2175272u32);
    emu.sbr_no_count(17usize, 16usize, 29usize, 2175276u32);
    emu.adr_no_count(16usize, 28usize, 16usize, 2175280u32);
    emu.sltru_no_count(5usize, 16usize, 28usize, 2175284u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2175288u32);
    emu.lw_no_count(7usize, 13usize, 0u32, 2175292u32)?;
    emu.lw_no_count(28usize, 13usize, 4u32, 2175296u32)?;
    emu.lw_no_count(29usize, 13usize, 8u32, 2175300u32)?;
    emu.lw_no_count(20usize, 13usize, 12u32, 2175304u32)?;
    emu.sai_no_count(17usize, 17usize, 1055u32, 2175308u32);
    emu.sbr_no_count(5usize, 17usize, 30usize, 2175312u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2175316u32);
    emu.sltru_no_count(6usize, 17usize, 9usize, 2175320u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2175324u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2175328u32);
    emu.sbr_no_count(6usize, 5usize, 31usize, 2175332u32);
    emu.adr_no_count(5usize, 18usize, 5usize, 2175336u32);
    emu.sltru_no_count(30usize, 5usize, 18usize, 2175340u32);
    emu.adr_no_count(6usize, 6usize, 30usize, 2175344u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2175348u32);
    emu.sbr_no_count(30usize, 6usize, 8usize, 2175352u32);
    emu.adr_no_count(6usize, 19usize, 6usize, 2175356u32);
    emu.sltru_no_count(31usize, 6usize, 19usize, 2175360u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2175364u32);
    emu.lw_no_count(31usize, 13usize, 16u32, 2175368u32)?;
    emu.lw_no_count(8usize, 13usize, 20u32, 2175372u32)?;
    emu.lw_no_count(9usize, 13usize, 24u32, 2175376u32)?;
    emu.lw_no_count(13usize, 13usize, 28u32, 2175380u32)?;
    emu.anr_no_count(7usize, 7usize, 30usize, 2175384u32);
    emu.anr_no_count(28usize, 28usize, 30usize, 2175388u32);
    emu.anr_no_count(29usize, 29usize, 30usize, 2175392u32);
    emu.anr_no_count(18usize, 20usize, 30usize, 2175396u32);
    emu.anr_no_count(31usize, 31usize, 30usize, 2175400u32);
    emu.anr_no_count(8usize, 8usize, 30usize, 2175404u32);
    emu.anr_no_count(9usize, 9usize, 30usize, 2175408u32);
    emu.anr_no_count(13usize, 13usize, 30usize, 2175412u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2175416u32);
    emu.adr_no_count(28usize, 12usize, 28usize, 2175420u32);
    emu.adr_no_count(29usize, 14usize, 29usize, 2175424u32);
    emu.adr_no_count(18usize, 15usize, 18usize, 2175428u32);
    emu.adr_no_count(31usize, 16usize, 31usize, 2175432u32);
    emu.adr_no_count(8usize, 17usize, 8usize, 2175436u32);
    emu.adr_no_count(9usize, 5usize, 9usize, 2175440u32);
    emu.sltru_no_count(11usize, 7usize, 11usize, 2175444u32);
    emu.sltru_no_count(12usize, 28usize, 12usize, 2175448u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2175452u32);
    emu.sltru_no_count(15usize, 18usize, 15usize, 2175456u32);
    emu.sltru_no_count(16usize, 31usize, 16usize, 2175460u32);
    emu.sltru_no_count(17usize, 8usize, 17usize, 2175464u32);
    emu.sltru_no_count(5usize, 9usize, 5usize, 2175468u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2175472u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2175476u32);
    emu.sltru_no_count(6usize, 11usize, 28usize, 2175480u32);
    emu.adr_no_count(13usize, 13usize, 5usize, 2175484u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2175488u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2175492u32);
    emu.sltru_no_count(5usize, 12usize, 29usize, 2175496u32);
    emu.adr_no_count(14usize, 14usize, 5usize, 2175500u32);
    emu.adr_no_count(14usize, 18usize, 14usize, 2175504u32);
    emu.sltru_no_count(5usize, 14usize, 18usize, 2175508u32);
    emu.sw_no_count(7usize, 10usize, 0u32, 2175512u32)?;
    emu.sw_no_count(11usize, 10usize, 4u32, 2175516u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2175520u32)?;
    emu.sw_no_count(14usize, 10usize, 12u32, 2175524u32)?;
    emu.adr_no_count(15usize, 15usize, 5usize, 2175528u32);
    emu.adr_no_count(15usize, 31usize, 15usize, 2175532u32);
    emu.sltru_no_count(11usize, 15usize, 31usize, 2175536u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2175540u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2175544u32);
    emu.sltru_no_count(12usize, 11usize, 8usize, 2175548u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2175552u32);
    emu.adr_no_count(12usize, 9usize, 12usize, 2175556u32);
    emu.sltru_no_count(14usize, 12usize, 9usize, 2175560u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2175564u32);
    emu.sw_no_count(15usize, 10usize, 16u32, 2175568u32)?;
    emu.sw_no_count(11usize, 10usize, 20u32, 2175572u32)?;
    emu.sw_no_count(12usize, 10usize, 24u32, 2175576u32)?;
    emu.sw_no_count(13usize, 10usize, 28u32, 2175580u32)?;
    emu.lw_no_count(8usize, 2usize, 28u32, 2175584u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2175588u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2175592u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2175596u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2175600u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2175604u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2175608u32);
    emu.add_memory_rw_events(140usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2175612u32;
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
pub fn block_0x0021327c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 99u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2175616u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2175620u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2175624u32)?;
    emu.sw_no_count(18usize, 2usize, 4u32, 2175628u32)?;
    emu.sw_no_count(19usize, 2usize, 0u32, 2175632u32)?;
    emu.lbu_no_count(12usize, 11usize, 0u32, 2175636u32);
    emu.lbu_no_count(13usize, 11usize, 1u32, 2175640u32);
    emu.lbu_no_count(14usize, 11usize, 2u32, 2175644u32);
    emu.lbu_no_count(15usize, 11usize, 3u32, 2175648u32);
    emu.lbu_no_count(16usize, 11usize, 4u32, 2175652u32);
    emu.lbu_no_count(17usize, 11usize, 5u32, 2175656u32);
    emu.lbu_no_count(5usize, 11usize, 6u32, 2175660u32);
    emu.lbu_no_count(6usize, 11usize, 7u32, 2175664u32);
    emu.lbu_no_count(7usize, 11usize, 8u32, 2175668u32);
    emu.lbu_no_count(28usize, 11usize, 9u32, 2175672u32);
    emu.lbu_no_count(29usize, 11usize, 10u32, 2175676u32);
    emu.lbu_no_count(30usize, 11usize, 11u32, 2175680u32);
    emu.lbu_no_count(31usize, 11usize, 12u32, 2175684u32);
    emu.lbu_no_count(8usize, 11usize, 13u32, 2175688u32);
    emu.lbu_no_count(9usize, 11usize, 14u32, 2175692u32);
    emu.lbu_no_count(18usize, 11usize, 15u32, 2175696u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2175700u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2175704u32);
    emu.sli_no_count(19usize, 12usize, 24u32, 2175708u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2175712u32);
    emu.orr_no_count(12usize, 14usize, 15usize, 2175716u32);
    emu.orr_no_count(13usize, 19usize, 13usize, 2175720u32);
    emu.orr_no_count(14usize, 5usize, 6usize, 2175724u32);
    emu.lbu_no_count(15usize, 11usize, 16u32, 2175728u32);
    emu.lbu_no_count(5usize, 11usize, 17u32, 2175732u32);
    emu.lbu_no_count(6usize, 11usize, 18u32, 2175736u32);
    emu.lbu_no_count(19usize, 11usize, 19u32, 2175740u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2175744u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2175748u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2175752u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2175756u32);
    emu.sli_no_count(7usize, 7usize, 24u32, 2175760u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2175764u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2175768u32);
    emu.orr_no_count(17usize, 29usize, 30usize, 2175772u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2175776u32);
    emu.orr_no_count(28usize, 9usize, 18usize, 2175780u32);
    emu.lbu_no_count(29usize, 11usize, 20u32, 2175784u32);
    emu.lbu_no_count(30usize, 11usize, 21u32, 2175788u32);
    emu.lbu_no_count(9usize, 11usize, 22u32, 2175792u32);
    emu.lbu_no_count(18usize, 11usize, 23u32, 2175796u32);
    emu.sli_no_count(8usize, 8usize, 16u32, 2175800u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2175804u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2175808u32);
    emu.sli_no_count(5usize, 5usize, 16u32, 2175812u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2175816u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2175820u32);
    emu.orr_no_count(31usize, 31usize, 8usize, 2175824u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2175828u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2175832u32);
    emu.orr_no_count(5usize, 9usize, 18usize, 2175836u32);
    emu.lbu_no_count(8usize, 11usize, 24u32, 2175840u32);
    emu.lbu_no_count(9usize, 11usize, 25u32, 2175844u32);
    emu.lbu_no_count(18usize, 11usize, 26u32, 2175848u32);
    emu.lbu_no_count(19usize, 11usize, 27u32, 2175852u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2175856u32);
    emu.sli_no_count(29usize, 29usize, 24u32, 2175860u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2175864u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2175868u32);
    emu.orr_no_count(30usize, 18usize, 19usize, 2175872u32);
    emu.lbu_no_count(18usize, 11usize, 30u32, 2175876u32);
    emu.lbu_no_count(19usize, 11usize, 31u32, 2175880u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2175884u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2175888u32);
    emu.orr_no_count(8usize, 8usize, 9usize, 2175892u32);
    emu.lbu_no_count(9usize, 11usize, 29u32, 2175896u32);
    emu.lbu_no_count(11usize, 11usize, 28u32, 2175900u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2175904u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2175908u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2175912u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2175916u32);
    emu.orr_no_count(11usize, 11usize, 9usize, 2175920u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2175924u32);
    emu.orr_no_count(13usize, 16usize, 14usize, 2175928u32);
    emu.orr_no_count(14usize, 7usize, 17usize, 2175932u32);
    emu.orr_no_count(16usize, 31usize, 28usize, 2175936u32);
    emu.orr_no_count(15usize, 15usize, 6usize, 2175940u32);
    emu.orr_no_count(17usize, 29usize, 5usize, 2175944u32);
    emu.orr_no_count(5usize, 8usize, 30usize, 2175948u32);
    emu.orr_no_count(11usize, 11usize, 18usize, 2175952u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2175956u32)?;
    emu.sw_no_count(5usize, 10usize, 4u32, 2175960u32)?;
    emu.sw_no_count(17usize, 10usize, 8u32, 2175964u32)?;
    emu.sw_no_count(15usize, 10usize, 12u32, 2175968u32)?;
    emu.sw_no_count(16usize, 10usize, 16u32, 2175972u32)?;
    emu.sw_no_count(14usize, 10usize, 20u32, 2175976u32)?;
    emu.sw_no_count(13usize, 10usize, 24u32, 2175980u32)?;
    emu.sw_no_count(12usize, 10usize, 28u32, 2175984u32)?;
    emu.lw_no_count(8usize, 2usize, 12u32, 2175988u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2175992u32)?;
    emu.lw_no_count(18usize, 2usize, 4u32, 2175996u32)?;
    emu.lw_no_count(19usize, 2usize, 0u32, 2176000u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2176004u32);
    emu.add_memory_rw_events(99usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2176008u32;
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
pub fn block_0x00213408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966720u32, 2176012u32);
    emu.sw_no_count(1usize, 2usize, 572u32, 2176016u32)?;
    emu.sw_no_count(8usize, 2usize, 568u32, 2176020u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2176024u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2176028u32)?;
    emu.sw_no_count(19usize, 2usize, 556u32, 2176032u32)?;
    emu.sw_no_count(20usize, 2usize, 552u32, 2176036u32)?;
    emu.sw_no_count(21usize, 2usize, 548u32, 2176040u32)?;
    emu.sw_no_count(22usize, 2usize, 544u32, 2176044u32)?;
    emu.sw_no_count(23usize, 2usize, 540u32, 2176048u32)?;
    emu.sw_no_count(24usize, 2usize, 536u32, 2176052u32)?;
    emu.sw_no_count(25usize, 2usize, 532u32, 2176056u32)?;
    emu.sw_no_count(26usize, 2usize, 528u32, 2176060u32)?;
    emu.sw_no_count(27usize, 2usize, 524u32, 2176064u32)?;
    emu.lw_no_count(23usize, 10usize, 0u32, 2176068u32)?;
    emu.lw_no_count(22usize, 10usize, 4u32, 2176072u32)?;
    emu.lw_no_count(21usize, 10usize, 8u32, 2176076u32)?;
    emu.lw_no_count(19usize, 10usize, 12u32, 2176080u32)?;
    emu.lw_no_count(30usize, 10usize, 16u32, 2176084u32)?;
    emu.lw_no_count(20usize, 10usize, 20u32, 2176088u32)?;
    emu.lw_no_count(26usize, 10usize, 24u32, 2176092u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2176096u32)?;
    emu.lw_no_count(9usize, 10usize, 28u32, 2176100u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2176108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021346c));
    } else {
        emu.pc = 2176104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00213468));
    }
}
#[inline(always)]
pub fn block_0x00213468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2176108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2192904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217608));
}
#[inline(never)]
pub fn block_0x0021346c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 195u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 6u32, 2176112u32);
    let a = 0u32.wrapping_add(607223808u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2176116u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1426882560u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2176120u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1925079040u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2176124u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2162077696u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2176128u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2614886400u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2176132u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3248222208u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2176136u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3835392000u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2176140u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4022222848u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2176144u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(264347648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2176148u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(604807168u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2176152u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(770256896u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2176156u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1249148928u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2176160u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1555083264u32);
    emu.write_reg_no_count(25usize, a);
    emu.pc = 2176164u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1996066816u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2176168u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2554220544u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2176172u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2821832704u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2176176u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2952994816u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176180u32;
    emu.update_insn_clock();
    emu.adr_no_count(12usize, 11usize, 12usize, 2176184u32);
    emu.sw_no_count(12usize, 2usize, 268u32, 2176188u32)?;
    let a = 0u32.wrapping_add(1116352512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176192u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967192u32, 2176196u32);
    emu.sw_no_count(12usize, 2usize, 264u32, 2176200u32)?;
    let a = 0u32.wrapping_add(1899446272u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176204u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1169u32, 2176208u32);
    emu.sw_no_count(12usize, 2usize, 260u32, 2176212u32)?;
    let a = 0u32.wrapping_add(3049324544u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176216u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966223u32, 2176220u32);
    emu.sw_no_count(12usize, 2usize, 256u32, 2176224u32)?;
    let a = 0u32.wrapping_add(3921010688u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176228u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966181u32, 2176232u32);
    emu.sw_no_count(12usize, 2usize, 252u32, 2176236u32)?;
    let a = 0u32.wrapping_add(961986560u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176240u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 603u32, 2176244u32);
    emu.sw_no_count(12usize, 2usize, 248u32, 2176248u32)?;
    let a = 0u32.wrapping_add(1508970496u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176252u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 497u32, 2176256u32);
    emu.sw_no_count(12usize, 2usize, 244u32, 2176260u32)?;
    let a = 0u32.wrapping_add(2453635072u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176264u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 676u32, 2176268u32);
    emu.sw_no_count(12usize, 2usize, 240u32, 2176272u32)?;
    let a = 0u32.wrapping_add(2870763520u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176276u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966997u32, 2176280u32);
    emu.sw_no_count(12usize, 2usize, 236u32, 2176284u32)?;
    let a = 0u32.wrapping_add(3624382464u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176288u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965912u32, 2176292u32);
    emu.sw_no_count(12usize, 2usize, 232u32, 2176296u32)?;
    let a = 0u32.wrapping_add(310599680u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2176300u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966017u32, 2176304u32);
    emu.sw_no_count(12usize, 2usize, 228u32, 2176308u32)?;
    emu.adi_no_count(12usize, 28usize, 1470u32, 2176312u32);
    emu.sw_no_count(12usize, 2usize, 224u32, 2176316u32)?;
    emu.adi_no_count(12usize, 29usize, 4294966723u32, 2176320u32);
    emu.sw_no_count(12usize, 2usize, 220u32, 2176324u32)?;
    emu.adi_no_count(12usize, 7usize, 4294966644u32, 2176328u32);
    emu.sw_no_count(12usize, 2usize, 216u32, 2176332u32)?;
    emu.adi_no_count(12usize, 31usize, 510u32, 2176336u32);
    emu.sw_no_count(12usize, 2usize, 212u32, 2176340u32)?;
    emu.adi_no_count(12usize, 8usize, 1703u32, 2176344u32);
    emu.sw_no_count(12usize, 2usize, 208u32, 2176348u32)?;
    emu.adi_no_count(12usize, 18usize, 372u32, 2176352u32);
    emu.sw_no_count(12usize, 2usize, 204u32, 2176356u32)?;
    emu.adi_no_count(12usize, 6usize, 4294965697u32, 2176360u32);
    emu.sw_no_count(12usize, 2usize, 200u32, 2176364u32)?;
    emu.adi_no_count(12usize, 5usize, 1926u32, 2176368u32);
    emu.sw_no_count(12usize, 2usize, 196u32, 2176372u32)?;
    emu.adi_no_count(12usize, 17usize, 4294966726u32, 2176376u32);
    emu.sw_no_count(12usize, 2usize, 192u32, 2176380u32)?;
    emu.adi_no_count(12usize, 16usize, 460u32, 2176384u32);
    emu.sw_no_count(12usize, 2usize, 188u32, 2176388u32)?;
    emu.adi_no_count(12usize, 15usize, 4294966383u32, 2176392u32);
    emu.sw_no_count(12usize, 2usize, 184u32, 2176396u32)?;
    emu.adi_no_count(12usize, 24usize, 1194u32, 2176400u32);
    emu.sw_no_count(12usize, 2usize, 180u32, 2176404u32)?;
    emu.adi_no_count(12usize, 25usize, 4294965724u32, 2176408u32);
    emu.sw_no_count(12usize, 2usize, 176u32, 2176412u32)?;
    emu.adi_no_count(12usize, 14usize, 4294965466u32, 2176416u32);
    emu.sw_no_count(12usize, 2usize, 172u32, 2176420u32)?;
    emu.adi_no_count(12usize, 27usize, 338u32, 2176424u32);
    emu.sw_no_count(12usize, 2usize, 168u32, 2176428u32)?;
    emu.adi_no_count(12usize, 13usize, 1645u32, 2176432u32);
    emu.sw_no_count(12usize, 2usize, 164u32, 2176436u32)?;
    emu.adi_no_count(10usize, 10usize, 1992u32, 2176440u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2176444u32)?;
    let a = 0u32.wrapping_add(3210313728u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176448u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967239u32, 2176452u32);
    emu.sw_no_count(10usize, 2usize, 156u32, 2176456u32)?;
    let a = 0u32.wrapping_add(3336572928u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176460u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966259u32, 2176464u32);
    emu.sw_no_count(10usize, 2usize, 152u32, 2176468u32)?;
    let a = 0u32.wrapping_add(3584528384u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176472u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 327u32, 2176476u32);
    emu.sw_no_count(10usize, 2usize, 148u32, 2176480u32)?;
    let a = 0u32.wrapping_add(113926144u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176484u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 849u32, 2176488u32);
    emu.sw_no_count(10usize, 2usize, 144u32, 2176492u32)?;
    let a = 0u32.wrapping_add(338243584u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176496u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965607u32, 2176500u32);
    emu.sw_no_count(10usize, 2usize, 140u32, 2176504u32)?;
    let a = 0u32.wrapping_add(666308608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176508u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965893u32, 2176512u32);
    emu.sw_no_count(10usize, 2usize, 136u32, 2176516u32)?;
    let a = 0u32.wrapping_add(773529600u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176520u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 312u32, 2176524u32);
    emu.sw_no_count(10usize, 2usize, 132u32, 2176528u32)?;
    let a = 0u32.wrapping_add(1294757888u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176532u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966780u32, 2176536u32);
    emu.sw_no_count(10usize, 2usize, 128u32, 2176540u32)?;
    let a = 0u32.wrapping_add(1396183040u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176544u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966547u32, 2176548u32);
    emu.sw_no_count(10usize, 2usize, 124u32, 2176552u32)?;
    let a = 0u32.wrapping_add(1695182848u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176556u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 852u32, 2176560u32);
    emu.sw_no_count(10usize, 2usize, 120u32, 2176564u32)?;
    let a = 0u32.wrapping_add(1986662400u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176568u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965947u32, 2176572u32);
    emu.sw_no_count(10usize, 2usize, 116u32, 2176576u32)?;
    let a = 0u32.wrapping_add(2177028096u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176580u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965550u32, 2176584u32);
    emu.sw_no_count(10usize, 2usize, 112u32, 2176588u32)?;
    let a = 0u32.wrapping_add(2456956928u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176592u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966405u32, 2176596u32);
    emu.sw_no_count(10usize, 2usize, 108u32, 2176600u32)?;
    let a = 0u32.wrapping_add(2730487808u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176604u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965409u32, 2176608u32);
    emu.sw_no_count(10usize, 2usize, 104u32, 2176612u32)?;
    let a = 0u32.wrapping_add(2820300800u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176616u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1611u32, 2176620u32);
    emu.sw_no_count(10usize, 2usize, 100u32, 2176624u32)?;
    let a = 0u32.wrapping_add(3259731968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176628u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966128u32, 2176632u32);
    emu.sw_no_count(10usize, 2usize, 96u32, 2176636u32)?;
    let a = 0u32.wrapping_add(3345764352u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176640u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 419u32, 2176644u32);
    emu.sw_no_count(10usize, 2usize, 92u32, 2176648u32)?;
    let a = 0u32.wrapping_add(3516067840u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176652u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965273u32, 2176656u32);
    emu.sw_no_count(10usize, 2usize, 88u32, 2176660u32)?;
    let a = 0u32.wrapping_add(3600351232u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176664u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1572u32, 2176668u32);
    emu.sw_no_count(10usize, 2usize, 84u32, 2176672u32)?;
    let a = 0u32.wrapping_add(4094570496u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176676u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1413u32, 2176680u32);
    emu.sw_no_count(10usize, 2usize, 80u32, 2176684u32)?;
    let a = 0u32.wrapping_add(275423232u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176688u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 112u32, 2176692u32);
    emu.sw_no_count(10usize, 2usize, 76u32, 2176696u32)?;
    let a = 0u32.wrapping_add(430227456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176700u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 278u32, 2176704u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2176708u32)?;
    let a = 0u32.wrapping_add(506949632u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176712u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966280u32, 2176716u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2176720u32)?;
    let a = 0u32.wrapping_add(659058688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176724u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1868u32, 2176728u32);
    emu.sw_no_count(10usize, 2usize, 64u32, 2176732u32)?;
    let a = 0u32.wrapping_add(883998720u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176736u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966453u32, 2176740u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2176744u32)?;
    let a = 0u32.wrapping_add(958140416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176748u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966451u32, 2176752u32);
    emu.sw_no_count(10usize, 2usize, 56u32, 2176756u32)?;
    let a = 0u32.wrapping_add(1322823680u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176760u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965834u32, 2176764u32);
    emu.sw_no_count(10usize, 2usize, 52u32, 2176768u32)?;
    let a = 0u32.wrapping_add(1537003520u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176772u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965839u32, 2176776u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2176780u32)?;
    let a = 0u32.wrapping_add(1747873792u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176784u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967283u32, 2176788u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2176792u32)?;
    let a = 0u32.wrapping_add(1955561472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176796u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 750u32, 2176800u32);
    emu.sw_no_count(10usize, 2usize, 40u32, 2176804u32)?;
    let a = 0u32.wrapping_add(2024103936u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176808u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 879u32, 2176812u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2176816u32)?;
    let a = 0u32.wrapping_add(2227732480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176820u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965268u32, 2176824u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2176828u32)?;
    let a = 0u32.wrapping_add(2361851904u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176832u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 520u32, 2176836u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2176840u32)?;
    let a = 0u32.wrapping_add(2428436480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176844u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967290u32, 2176848u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2176852u32)?;
    let a = 0u32.wrapping_add(2756734976u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176856u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966507u32, 2176860u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2176864u32)?;
    let a = 0u32.wrapping_add(3204030464u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176868u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1015u32, 2176872u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2176876u32)?;
    let a = 0u32.wrapping_add(3329327104u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2176880u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965490u32, 2176884u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2176888u32)?;
    emu.add_memory_rw_events(195usize);
    emu.pc = 2176888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213778));
}
#[inline(never)]
pub fn block_0x00213778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4003u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 2usize, 520u32, 2176892u32)?;
    emu.sw_no_count(21usize, 2usize, 512u32, 2176896u32)?;
    emu.sw_no_count(19usize, 2usize, 428u32, 2176900u32)?;
    emu.sw_no_count(20usize, 2usize, 516u32, 2176904u32)?;
    emu.sw_no_count(26usize, 2usize, 432u32, 2176908u32)?;
    emu.sw_no_count(9usize, 2usize, 372u32, 2176912u32)?;
    emu.lbu_no_count(16usize, 11usize, 0u32, 2176916u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2176920u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2176924u32);
    emu.lbu_no_count(29usize, 11usize, 3u32, 2176928u32);
    emu.lbu_no_count(14usize, 11usize, 4u32, 2176932u32);
    emu.lbu_no_count(15usize, 11usize, 5u32, 2176936u32);
    emu.lbu_no_count(10usize, 11usize, 6u32, 2176940u32);
    emu.sw_no_count(10usize, 2usize, 484u32, 2176944u32)?;
    emu.lbu_no_count(10usize, 11usize, 7u32, 2176948u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2176952u32)?;
    emu.lbu_no_count(17usize, 11usize, 8u32, 2176956u32);
    emu.lbu_no_count(6usize, 11usize, 9u32, 2176960u32);
    emu.lbu_no_count(10usize, 11usize, 10u32, 2176964u32);
    emu.sw_no_count(10usize, 2usize, 420u32, 2176968u32)?;
    emu.lbu_no_count(10usize, 11usize, 11u32, 2176972u32);
    emu.sw_no_count(10usize, 2usize, 416u32, 2176976u32)?;
    emu.lbu_no_count(5usize, 11usize, 12u32, 2176980u32);
    emu.lbu_no_count(28usize, 11usize, 13u32, 2176984u32);
    emu.lbu_no_count(10usize, 11usize, 14u32, 2176988u32);
    emu.sw_no_count(10usize, 2usize, 500u32, 2176992u32)?;
    emu.lbu_no_count(22usize, 11usize, 15u32, 2176996u32);
    emu.sri_no_count(7usize, 30usize, 6u32, 2177000u32);
    emu.adi_no_count(10usize, 30usize, 0u32, 2177004u32);
    emu.sli_no_count(30usize, 30usize, 26u32, 2177008u32);
    emu.sri_no_count(31usize, 10usize, 11u32, 2177012u32);
    emu.sli_no_count(8usize, 10usize, 21u32, 2177016u32);
    emu.sri_no_count(18usize, 10usize, 25u32, 2177020u32);
    emu.sli_no_count(19usize, 10usize, 7u32, 2177024u32);
    emu.sw_no_count(10usize, 2usize, 424u32, 2177028u32)?;
    emu.sri_no_count(21usize, 23usize, 2u32, 2177032u32);
    emu.adi_no_count(27usize, 23usize, 0u32, 2177036u32);
    emu.sli_no_count(23usize, 23usize, 30u32, 2177040u32);
    emu.sri_no_count(24usize, 27usize, 13u32, 2177044u32);
    emu.sli_no_count(25usize, 27usize, 19u32, 2177048u32);
    emu.orr_no_count(7usize, 30usize, 7usize, 2177052u32);
    emu.sri_no_count(30usize, 27usize, 22u32, 2177056u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2177060u32);
    emu.sli_no_count(8usize, 27usize, 10u32, 2177064u32);
    emu.sw_no_count(27usize, 2usize, 508u32, 2177068u32)?;
    emu.orr_no_count(18usize, 19usize, 18usize, 2177072u32);
    emu.orr_no_count(19usize, 23usize, 21usize, 2177076u32);
    emu.orr_no_count(21usize, 25usize, 24usize, 2177080u32);
    emu.orr_no_count(30usize, 8usize, 30usize, 2177084u32);
    emu.lbu_no_count(23usize, 11usize, 16u32, 2177088u32);
    emu.lbu_no_count(1usize, 11usize, 17u32, 2177092u32);
    emu.lbu_no_count(25usize, 11usize, 18u32, 2177096u32);
    emu.lbu_no_count(24usize, 11usize, 19u32, 2177100u32);
    emu.xrr_no_count(7usize, 7usize, 31usize, 2177104u32);
    emu.lw_no_count(31usize, 2usize, 512u32, 2177108u32)?;
    emu.lw_no_count(8usize, 2usize, 520u32, 2177112u32)?;
    emu.xrr_no_count(31usize, 31usize, 8usize, 2177116u32);
    emu.xrr_no_count(8usize, 19usize, 21usize, 2177120u32);
    emu.lw_no_count(19usize, 2usize, 512u32, 2177124u32)?;
    emu.lw_no_count(21usize, 2usize, 520u32, 2177128u32)?;
    emu.anr_no_count(19usize, 19usize, 21usize, 2177132u32);
    emu.anr_no_count(31usize, 31usize, 27usize, 2177136u32);
    emu.xrr_no_count(31usize, 31usize, 19usize, 2177140u32);
    emu.xrr_no_count(19usize, 26usize, 20usize, 2177144u32);
    emu.anr_no_count(19usize, 19usize, 10usize, 2177148u32);
    emu.xrr_no_count(19usize, 19usize, 26usize, 2177152u32);
    emu.xrr_no_count(7usize, 7usize, 18usize, 2177156u32);
    emu.adr_no_count(19usize, 9usize, 19usize, 2177160u32);
    emu.xrr_no_count(30usize, 8usize, 30usize, 2177164u32);
    emu.adr_no_count(7usize, 19usize, 7usize, 2177168u32);
    emu.sw_no_count(7usize, 2usize, 496u32, 2177172u32)?;
    emu.adr_no_count(30usize, 31usize, 30usize, 2177176u32);
    emu.sw_no_count(30usize, 2usize, 504u32, 2177180u32)?;
    emu.lbu_no_count(30usize, 11usize, 20u32, 2177184u32);
    emu.lbu_no_count(31usize, 11usize, 21u32, 2177188u32);
    emu.lbu_no_count(10usize, 11usize, 22u32, 2177192u32);
    emu.sw_no_count(10usize, 2usize, 488u32, 2177196u32)?;
    emu.lbu_no_count(9usize, 11usize, 23u32, 2177200u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2177204u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2177208u32);
    emu.sli_no_count(7usize, 16usize, 24u32, 2177212u32);
    emu.sli_no_count(15usize, 15usize, 16u32, 2177216u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2177220u32);
    emu.sli_no_count(8usize, 6usize, 16u32, 2177224u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2177228u32);
    emu.orr_no_count(16usize, 13usize, 29usize, 2177232u32);
    emu.orr_no_count(26usize, 7usize, 12usize, 2177236u32);
    emu.orr_no_count(6usize, 14usize, 15usize, 2177240u32);
    emu.orr_no_count(8usize, 17usize, 8usize, 2177244u32);
    emu.lbu_no_count(10usize, 11usize, 24u32, 2177248u32);
    emu.lbu_no_count(12usize, 11usize, 25u32, 2177252u32);
    emu.lbu_no_count(13usize, 11usize, 26u32, 2177256u32);
    emu.sw_no_count(13usize, 2usize, 472u32, 2177260u32)?;
    emu.lbu_no_count(7usize, 11usize, 27u32, 2177264u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2177268u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2177272u32);
    emu.sli_no_count(27usize, 1usize, 16u32, 2177276u32);
    emu.sli_no_count(23usize, 23usize, 24u32, 2177280u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2177284u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2177288u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2177292u32);
    emu.sli_no_count(15usize, 10usize, 24u32, 2177296u32);
    emu.orr_no_count(10usize, 5usize, 28usize, 2177300u32);
    emu.orr_no_count(18usize, 23usize, 27usize, 2177304u32);
    emu.orr_no_count(13usize, 30usize, 31usize, 2177308u32);
    emu.lbu_no_count(14usize, 11usize, 28u32, 2177312u32);
    emu.lbu_no_count(17usize, 11usize, 29u32, 2177316u32);
    emu.orr_no_count(15usize, 15usize, 12usize, 2177320u32);
    emu.lbu_no_count(12usize, 11usize, 30u32, 2177324u32);
    emu.sw_no_count(12usize, 2usize, 468u32, 2177328u32)?;
    emu.lbu_no_count(31usize, 11usize, 31u32, 2177332u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2177336u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2177340u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2177344u32);
    emu.sw_no_count(11usize, 2usize, 368u32, 2177348u32)?;
    emu.lbu_no_count(17usize, 11usize, 33u32, 2177352u32);
    emu.lbu_no_count(5usize, 11usize, 32u32, 2177356u32);
    emu.lbu_no_count(23usize, 11usize, 34u32, 2177360u32);
    emu.lbu_no_count(12usize, 11usize, 35u32, 2177364u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2177368u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2177372u32);
    emu.orr_no_count(1usize, 5usize, 17usize, 2177376u32);
    emu.lbu_no_count(17usize, 11usize, 37u32, 2177380u32);
    emu.lbu_no_count(5usize, 11usize, 36u32, 2177384u32);
    emu.lbu_no_count(28usize, 11usize, 38u32, 2177388u32);
    emu.sw_no_count(28usize, 2usize, 480u32, 2177392u32)?;
    emu.lbu_no_count(28usize, 11usize, 39u32, 2177396u32);
    emu.sw_no_count(28usize, 2usize, 440u32, 2177400u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2177404u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2177408u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2177412u32);
    emu.sw_no_count(17usize, 2usize, 444u32, 2177416u32)?;
    emu.lbu_no_count(17usize, 11usize, 41u32, 2177420u32);
    emu.lbu_no_count(5usize, 11usize, 40u32, 2177424u32);
    emu.lbu_no_count(28usize, 11usize, 42u32, 2177428u32);
    emu.sw_no_count(28usize, 2usize, 448u32, 2177432u32)?;
    emu.lbu_no_count(28usize, 11usize, 43u32, 2177436u32);
    emu.sw_no_count(28usize, 2usize, 412u32, 2177440u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2177444u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2177448u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2177452u32);
    emu.sw_no_count(17usize, 2usize, 436u32, 2177456u32)?;
    emu.lbu_no_count(17usize, 11usize, 45u32, 2177460u32);
    emu.lbu_no_count(5usize, 11usize, 44u32, 2177464u32);
    emu.lbu_no_count(28usize, 11usize, 46u32, 2177468u32);
    emu.sw_no_count(28usize, 2usize, 452u32, 2177472u32)?;
    emu.lbu_no_count(28usize, 11usize, 47u32, 2177476u32);
    emu.sw_no_count(28usize, 2usize, 404u32, 2177480u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2177484u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2177488u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2177492u32);
    emu.sw_no_count(17usize, 2usize, 408u32, 2177496u32)?;
    emu.lbu_no_count(17usize, 11usize, 49u32, 2177500u32);
    emu.lbu_no_count(5usize, 11usize, 48u32, 2177504u32);
    emu.lbu_no_count(28usize, 11usize, 50u32, 2177508u32);
    emu.sw_no_count(28usize, 2usize, 396u32, 2177512u32)?;
    emu.lbu_no_count(28usize, 11usize, 51u32, 2177516u32);
    emu.sw_no_count(28usize, 2usize, 392u32, 2177520u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2177524u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2177528u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2177532u32);
    emu.sw_no_count(17usize, 2usize, 400u32, 2177536u32)?;
    emu.lbu_no_count(17usize, 11usize, 53u32, 2177540u32);
    emu.lbu_no_count(5usize, 11usize, 52u32, 2177544u32);
    emu.lbu_no_count(29usize, 11usize, 54u32, 2177548u32);
    emu.sw_no_count(29usize, 2usize, 476u32, 2177552u32)?;
    emu.lbu_no_count(29usize, 11usize, 55u32, 2177556u32);
    emu.sw_no_count(29usize, 2usize, 456u32, 2177560u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2177564u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2177568u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2177572u32);
    emu.sw_no_count(17usize, 2usize, 460u32, 2177576u32)?;
    emu.lbu_no_count(17usize, 11usize, 57u32, 2177580u32);
    emu.lbu_no_count(30usize, 11usize, 56u32, 2177584u32);
    emu.lbu_no_count(5usize, 11usize, 58u32, 2177588u32);
    emu.lbu_no_count(20usize, 11usize, 59u32, 2177592u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2177596u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2177600u32);
    emu.orr_no_count(19usize, 30usize, 17usize, 2177604u32);
    emu.lbu_no_count(30usize, 11usize, 61u32, 2177608u32);
    emu.lbu_no_count(27usize, 11usize, 60u32, 2177612u32);
    emu.lbu_no_count(17usize, 11usize, 62u32, 2177616u32);
    emu.lbu_no_count(28usize, 11usize, 63u32, 2177620u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2177624u32);
    emu.sli_no_count(27usize, 27usize, 24u32, 2177628u32);
    emu.orr_no_count(21usize, 27usize, 30usize, 2177632u32);
    emu.orr_no_count(11usize, 26usize, 16usize, 2177636u32);
    emu.sw_no_count(11usize, 2usize, 492u32, 2177640u32)?;
    emu.sli_no_count(25usize, 25usize, 8u32, 2177644u32);
    emu.orr_no_count(11usize, 25usize, 24usize, 2177648u32);
    emu.sli_no_count(25usize, 24usize, 25u32, 2177652u32);
    emu.orr_no_count(24usize, 18usize, 11usize, 2177656u32);
    emu.sri_no_count(11usize, 24usize, 7u32, 2177660u32);
    emu.sw_no_count(24usize, 2usize, 384u32, 2177664u32)?;
    emu.orr_no_count(11usize, 25usize, 11usize, 2177668u32);
    emu.sw_no_count(11usize, 2usize, 376u32, 2177672u32)?;
    emu.lw_no_count(29usize, 2usize, 500u32, 2177676u32)?;
    emu.sli_no_count(29usize, 29usize, 8u32, 2177680u32);
    emu.orr_no_count(11usize, 29usize, 22usize, 2177684u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2177688u32);
    emu.orr_no_count(30usize, 10usize, 11usize, 2177692u32);
    emu.sri_no_count(11usize, 30usize, 7u32, 2177696u32);
    emu.sw_no_count(30usize, 2usize, 500u32, 2177700u32)?;
    emu.orr_no_count(11usize, 29usize, 11usize, 2177704u32);
    emu.sw_no_count(11usize, 2usize, 364u32, 2177708u32)?;
    emu.lw_no_count(11usize, 2usize, 420u32, 2177712u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2177716u32);
    emu.lw_no_count(29usize, 2usize, 416u32, 2177720u32)?;
    emu.orr_no_count(11usize, 11usize, 29usize, 2177724u32);
    emu.sli_no_count(29usize, 29usize, 25u32, 2177728u32);
    emu.orr_no_count(22usize, 8usize, 11usize, 2177732u32);
    emu.sri_no_count(11usize, 22usize, 7u32, 2177736u32);
    emu.sw_no_count(22usize, 2usize, 388u32, 2177740u32)?;
    emu.orr_no_count(11usize, 29usize, 11usize, 2177744u32);
    emu.sw_no_count(11usize, 2usize, 360u32, 2177748u32)?;
    emu.lw_no_count(11usize, 2usize, 484u32, 2177752u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2177756u32);
    emu.lw_no_count(16usize, 2usize, 464u32, 2177760u32)?;
    emu.orr_no_count(11usize, 11usize, 16usize, 2177764u32);
    emu.sli_no_count(16usize, 16usize, 25u32, 2177768u32);
    emu.orr_no_count(29usize, 6usize, 11usize, 2177772u32);
    emu.sri_no_count(11usize, 29usize, 7u32, 2177776u32);
    emu.sw_no_count(29usize, 2usize, 380u32, 2177780u32)?;
    emu.orr_no_count(11usize, 16usize, 11usize, 2177784u32);
    emu.sw_no_count(11usize, 2usize, 356u32, 2177788u32)?;
    emu.sri_no_count(11usize, 18usize, 18u32, 2177792u32);
    emu.sli_no_count(16usize, 24usize, 14u32, 2177796u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2177800u32);
    emu.sw_no_count(11usize, 2usize, 352u32, 2177804u32)?;
    emu.sri_no_count(10usize, 10usize, 18u32, 2177808u32);
    emu.sli_no_count(11usize, 30usize, 14u32, 2177812u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2177816u32);
    emu.sw_no_count(10usize, 2usize, 348u32, 2177820u32)?;
    emu.sri_no_count(8usize, 8usize, 18u32, 2177824u32);
    emu.sli_no_count(10usize, 22usize, 14u32, 2177828u32);
    emu.orr_no_count(10usize, 10usize, 8usize, 2177832u32);
    emu.sw_no_count(10usize, 2usize, 344u32, 2177836u32)?;
    emu.sri_no_count(10usize, 6usize, 18u32, 2177840u32);
    emu.sli_no_count(11usize, 29usize, 14u32, 2177844u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2177848u32);
    emu.sw_no_count(10usize, 2usize, 340u32, 2177852u32)?;
    emu.sli_no_count(5usize, 5usize, 8u32, 2177856u32);
    emu.orr_no_count(10usize, 5usize, 20usize, 2177860u32);
    emu.adi_no_count(8usize, 20usize, 0u32, 2177864u32);
    emu.orr_no_count(5usize, 19usize, 10usize, 2177868u32);
    emu.sri_no_count(10usize, 19usize, 17u32, 2177872u32);
    emu.sli_no_count(11usize, 5usize, 15u32, 2177876u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2177880u32);
    emu.sw_no_count(10usize, 2usize, 336u32, 2177884u32)?;
    emu.sri_no_count(10usize, 19usize, 19u32, 2177888u32);
    emu.adi_no_count(18usize, 19usize, 0u32, 2177892u32);
    emu.sli_no_count(11usize, 5usize, 13u32, 2177896u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2177900u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2177904u32)?;
    emu.sli_no_count(17usize, 17usize, 8u32, 2177908u32);
    emu.adi_no_count(25usize, 28usize, 0u32, 2177912u32);
    emu.orr_no_count(10usize, 17usize, 28usize, 2177916u32);
    emu.orr_no_count(16usize, 21usize, 10usize, 2177920u32);
    emu.sri_no_count(10usize, 21usize, 17u32, 2177924u32);
    emu.sli_no_count(11usize, 16usize, 15u32, 2177928u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2177932u32);
    emu.sw_no_count(10usize, 2usize, 328u32, 2177936u32)?;
    emu.sri_no_count(10usize, 21usize, 19u32, 2177940u32);
    emu.sli_no_count(11usize, 16usize, 13u32, 2177944u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2177948u32);
    emu.sw_no_count(10usize, 2usize, 324u32, 2177952u32)?;
    emu.sli_no_count(10usize, 23usize, 8u32, 2177956u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2177960u32);
    emu.sli_no_count(12usize, 12usize, 25u32, 2177964u32);
    emu.orr_no_count(17usize, 1usize, 10usize, 2177968u32);
    emu.sri_no_count(10usize, 17usize, 7u32, 2177972u32);
    emu.sw_no_count(17usize, 2usize, 484u32, 2177976u32)?;
    emu.orr_no_count(10usize, 12usize, 10usize, 2177980u32);
    emu.sw_no_count(10usize, 2usize, 320u32, 2177984u32)?;
    emu.lw_no_count(10usize, 2usize, 468u32, 2177988u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2177992u32);
    emu.orr_no_count(10usize, 10usize, 31usize, 2177996u32);
    emu.sli_no_count(31usize, 31usize, 25u32, 2178000u32);
    emu.orr_no_count(12usize, 14usize, 10usize, 2178004u32);
    emu.sri_no_count(10usize, 12usize, 7u32, 2178008u32);
    emu.sw_no_count(12usize, 2usize, 468u32, 2178012u32)?;
    emu.orr_no_count(10usize, 31usize, 10usize, 2178016u32);
    emu.sw_no_count(10usize, 2usize, 316u32, 2178020u32)?;
    emu.lw_no_count(10usize, 2usize, 472u32, 2178024u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2178028u32);
    emu.orr_no_count(10usize, 10usize, 7usize, 2178032u32);
    emu.sli_no_count(7usize, 7usize, 25u32, 2178036u32);
    emu.orr_no_count(6usize, 15usize, 10usize, 2178040u32);
    emu.sri_no_count(10usize, 6usize, 7u32, 2178044u32);
    emu.sw_no_count(6usize, 2usize, 464u32, 2178048u32)?;
    emu.orr_no_count(10usize, 7usize, 10usize, 2178052u32);
    emu.sw_no_count(10usize, 2usize, 312u32, 2178056u32)?;
    emu.lw_no_count(10usize, 2usize, 488u32, 2178060u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2178064u32);
    emu.orr_no_count(10usize, 10usize, 9usize, 2178068u32);
    emu.sli_no_count(11usize, 9usize, 25u32, 2178072u32);
    emu.orr_no_count(7usize, 13usize, 10usize, 2178076u32);
    emu.sri_no_count(10usize, 7usize, 7u32, 2178080u32);
    emu.sw_no_count(7usize, 2usize, 472u32, 2178084u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2178088u32);
    emu.sw_no_count(10usize, 2usize, 308u32, 2178092u32)?;
    emu.sri_no_count(10usize, 1usize, 18u32, 2178096u32);
    emu.sli_no_count(11usize, 17usize, 14u32, 2178100u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2178104u32);
    emu.sw_no_count(10usize, 2usize, 304u32, 2178108u32)?;
    emu.sri_no_count(14usize, 14usize, 18u32, 2178112u32);
    emu.sli_no_count(10usize, 12usize, 14u32, 2178116u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2178120u32);
    emu.sw_no_count(10usize, 2usize, 300u32, 2178124u32)?;
    emu.sri_no_count(15usize, 15usize, 18u32, 2178128u32);
    emu.sli_no_count(10usize, 6usize, 14u32, 2178132u32);
    emu.orr_no_count(20usize, 10usize, 15usize, 2178136u32);
    emu.sri_no_count(13usize, 13usize, 18u32, 2178140u32);
    emu.sli_no_count(10usize, 7usize, 14u32, 2178144u32);
    emu.orr_no_count(28usize, 10usize, 13usize, 2178148u32);
    emu.lw_no_count(10usize, 2usize, 396u32, 2178152u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2178156u32);
    emu.lw_no_count(23usize, 2usize, 392u32, 2178160u32)?;
    emu.orr_no_count(10usize, 10usize, 23usize, 2178164u32);
    emu.sli_no_count(23usize, 23usize, 25u32, 2178168u32);
    emu.lw_no_count(13usize, 2usize, 400u32, 2178172u32)?;
    emu.orr_no_count(7usize, 13usize, 10usize, 2178176u32);
    emu.sri_no_count(10usize, 7usize, 7u32, 2178180u32);
    emu.sw_no_count(7usize, 2usize, 488u32, 2178184u32)?;
    emu.orr_no_count(23usize, 23usize, 10usize, 2178188u32);
    emu.lw_no_count(10usize, 2usize, 452u32, 2178192u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2178196u32);
    emu.lw_no_count(11usize, 2usize, 404u32, 2178200u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2178204u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2178208u32);
    emu.lw_no_count(6usize, 2usize, 408u32, 2178212u32)?;
    emu.orr_no_count(12usize, 6usize, 10usize, 2178216u32);
    emu.sri_no_count(10usize, 12usize, 7u32, 2178220u32);
    emu.sw_no_count(12usize, 2usize, 452u32, 2178224u32)?;
    emu.orr_no_count(19usize, 11usize, 10usize, 2178228u32);
    emu.lw_no_count(10usize, 2usize, 448u32, 2178232u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2178236u32);
    emu.lw_no_count(11usize, 2usize, 412u32, 2178240u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2178244u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2178248u32);
    emu.lw_no_count(17usize, 2usize, 436u32, 2178252u32)?;
    emu.orr_no_count(30usize, 17usize, 10usize, 2178256u32);
    emu.sri_no_count(15usize, 30usize, 7u32, 2178260u32);
    emu.sw_no_count(30usize, 2usize, 448u32, 2178264u32)?;
    emu.orr_no_count(31usize, 11usize, 15usize, 2178268u32);
    emu.lw_no_count(10usize, 2usize, 480u32, 2178272u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2178276u32);
    emu.lw_no_count(11usize, 2usize, 440u32, 2178280u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2178284u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2178288u32);
    emu.lw_no_count(15usize, 2usize, 444u32, 2178292u32)?;
    emu.orr_no_count(26usize, 15usize, 10usize, 2178296u32);
    emu.sri_no_count(14usize, 26usize, 7u32, 2178300u32);
    emu.sw_no_count(26usize, 2usize, 480u32, 2178304u32)?;
    emu.orr_no_count(9usize, 11usize, 14usize, 2178308u32);
    emu.sri_no_count(10usize, 13usize, 18u32, 2178312u32);
    emu.sli_no_count(13usize, 7usize, 14u32, 2178316u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2178320u32);
    emu.sri_no_count(10usize, 6usize, 18u32, 2178324u32);
    emu.sli_no_count(12usize, 12usize, 14u32, 2178328u32);
    emu.orr_no_count(29usize, 12usize, 10usize, 2178332u32);
    emu.sri_no_count(10usize, 17usize, 18u32, 2178336u32);
    emu.sli_no_count(11usize, 30usize, 14u32, 2178340u32);
    emu.orr_no_count(24usize, 11usize, 10usize, 2178344u32);
    emu.sri_no_count(11usize, 15usize, 18u32, 2178348u32);
    emu.sli_no_count(10usize, 26usize, 14u32, 2178352u32);
    emu.orr_no_count(1usize, 10usize, 11usize, 2178356u32);
    emu.sli_no_count(11usize, 25usize, 25u32, 2178360u32);
    emu.sri_no_count(27usize, 16usize, 7u32, 2178364u32);
    emu.orr_no_count(27usize, 11usize, 27usize, 2178368u32);
    emu.sli_no_count(11usize, 8usize, 25u32, 2178372u32);
    emu.adi_no_count(6usize, 5usize, 0u32, 2178376u32);
    emu.sri_no_count(26usize, 5usize, 7u32, 2178380u32);
    emu.orr_no_count(26usize, 11usize, 26usize, 2178384u32);
    emu.lw_no_count(11usize, 2usize, 476u32, 2178388u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2178392u32);
    emu.lw_no_count(10usize, 2usize, 456u32, 2178396u32)?;
    emu.orr_no_count(11usize, 11usize, 10usize, 2178400u32);
    emu.sli_no_count(10usize, 10usize, 25u32, 2178404u32);
    emu.lw_no_count(13usize, 2usize, 460u32, 2178408u32)?;
    emu.orr_no_count(8usize, 13usize, 11usize, 2178412u32);
    emu.sri_no_count(11usize, 8usize, 7u32, 2178416u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2178420u32);
    emu.sri_no_count(11usize, 21usize, 18u32, 2178424u32);
    emu.sli_no_count(21usize, 16usize, 14u32, 2178428u32);
    emu.orr_no_count(21usize, 21usize, 11usize, 2178432u32);
    emu.sri_no_count(11usize, 18usize, 18u32, 2178436u32);
    emu.sli_no_count(25usize, 5usize, 14u32, 2178440u32);
    emu.orr_no_count(25usize, 25usize, 11usize, 2178444u32);
    emu.sri_no_count(11usize, 13usize, 18u32, 2178448u32);
    emu.sli_no_count(18usize, 8usize, 14u32, 2178452u32);
    emu.orr_no_count(12usize, 18usize, 11usize, 2178456u32);
    emu.lw_no_count(11usize, 2usize, 376u32, 2178460u32)?;
    emu.lw_no_count(13usize, 2usize, 352u32, 2178464u32)?;
    emu.xrr_no_count(30usize, 11usize, 13usize, 2178468u32);
    emu.lw_no_count(11usize, 2usize, 364u32, 2178472u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2178476u32)?;
    emu.xrr_no_count(11usize, 11usize, 13usize, 2178480u32);
    emu.lw_no_count(13usize, 2usize, 360u32, 2178484u32)?;
    emu.lw_no_count(14usize, 2usize, 344u32, 2178488u32)?;
    emu.xrr_no_count(13usize, 13usize, 14usize, 2178492u32);
    emu.lw_no_count(14usize, 2usize, 356u32, 2178496u32)?;
    emu.lw_no_count(15usize, 2usize, 340u32, 2178500u32)?;
    emu.xrr_no_count(14usize, 14usize, 15usize, 2178504u32);
    emu.lw_no_count(15usize, 2usize, 336u32, 2178508u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2178512u32)?;
    emu.xrr_no_count(15usize, 15usize, 17usize, 2178516u32);
    emu.lw_no_count(17usize, 2usize, 328u32, 2178520u32)?;
    emu.lw_no_count(5usize, 2usize, 324u32, 2178524u32)?;
    emu.xrr_no_count(17usize, 17usize, 5usize, 2178528u32);
    emu.lw_no_count(5usize, 2usize, 320u32, 2178532u32)?;
    emu.lw_no_count(7usize, 2usize, 304u32, 2178536u32)?;
    emu.xrr_no_count(5usize, 5usize, 7usize, 2178540u32);
    emu.lw_no_count(7usize, 2usize, 316u32, 2178544u32)?;
    emu.lw_no_count(18usize, 2usize, 300u32, 2178548u32)?;
    emu.xrr_no_count(7usize, 7usize, 18usize, 2178552u32);
    emu.lw_no_count(18usize, 2usize, 312u32, 2178556u32)?;
    emu.xrr_no_count(18usize, 18usize, 20usize, 2178560u32);
    emu.lw_no_count(20usize, 2usize, 308u32, 2178564u32)?;
    emu.xrr_no_count(20usize, 20usize, 28usize, 2178568u32);
    emu.xrr_no_count(22usize, 23usize, 22usize, 2178572u32);
    emu.xrr_no_count(23usize, 19usize, 29usize, 2178576u32);
    emu.xrr_no_count(28usize, 31usize, 24usize, 2178580u32);
    emu.sw_no_count(28usize, 2usize, 436u32, 2178584u32)?;
    emu.xrr_no_count(28usize, 9usize, 1usize, 2178588u32);
    emu.sw_no_count(28usize, 2usize, 456u32, 2178592u32)?;
    emu.xrr_no_count(28usize, 27usize, 21usize, 2178596u32);
    emu.sw_no_count(28usize, 2usize, 356u32, 2178600u32)?;
    emu.xrr_no_count(28usize, 26usize, 25usize, 2178604u32);
    emu.sw_no_count(28usize, 2usize, 364u32, 2178608u32)?;
    emu.xrr_no_count(10usize, 10usize, 12usize, 2178612u32);
    emu.sw_no_count(10usize, 2usize, 396u32, 2178616u32)?;
    emu.lw_no_count(10usize, 2usize, 264u32, 2178620u32)?;
    emu.lw_no_count(12usize, 2usize, 496u32, 2178624u32)?;
    emu.adr_no_count(10usize, 12usize, 10usize, 2178628u32);
    emu.lw_no_count(9usize, 2usize, 492u32, 2178632u32)?;
    emu.adr_no_count(10usize, 10usize, 9usize, 2178636u32);
    emu.lw_no_count(12usize, 2usize, 504u32, 2178640u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2178644u32);
    emu.lw_no_count(31usize, 2usize, 428u32, 2178648u32)?;
    emu.adr_no_count(28usize, 10usize, 31usize, 2178652u32);
    emu.sri_no_count(10usize, 28usize, 6u32, 2178656u32);
    emu.sli_no_count(29usize, 28usize, 26u32, 2178660u32);
    emu.orr_no_count(10usize, 29usize, 10usize, 2178664u32);
    emu.sw_no_count(10usize, 2usize, 496u32, 2178668u32)?;
    emu.sri_no_count(10usize, 28usize, 11u32, 2178672u32);
    emu.sli_no_count(29usize, 28usize, 21u32, 2178676u32);
    emu.orr_no_count(10usize, 29usize, 10usize, 2178680u32);
    emu.sw_no_count(10usize, 2usize, 504u32, 2178684u32)?;
    emu.sri_no_count(10usize, 28usize, 25u32, 2178688u32);
    emu.sli_no_count(29usize, 28usize, 7u32, 2178692u32);
    emu.sw_no_count(28usize, 2usize, 444u32, 2178696u32)?;
    emu.orr_no_count(10usize, 29usize, 10usize, 2178700u32);
    emu.sw_no_count(10usize, 2usize, 440u32, 2178704u32)?;
    emu.adi_no_count(31usize, 12usize, 0u32, 2178708u32);
    emu.sw_no_count(12usize, 2usize, 360u32, 2178712u32)?;
    emu.sri_no_count(29usize, 12usize, 2u32, 2178716u32);
    emu.sli_no_count(27usize, 12usize, 30u32, 2178720u32);
    emu.orr_no_count(1usize, 27usize, 29usize, 2178724u32);
    emu.sri_no_count(29usize, 12usize, 13u32, 2178728u32);
    emu.sli_no_count(27usize, 12usize, 19u32, 2178732u32);
    emu.orr_no_count(19usize, 27usize, 29usize, 2178736u32);
    emu.sri_no_count(29usize, 12usize, 22u32, 2178740u32);
    emu.sli_no_count(27usize, 12usize, 10u32, 2178744u32);
    emu.orr_no_count(10usize, 27usize, 29usize, 2178748u32);
    emu.sw_no_count(10usize, 2usize, 408u32, 2178752u32)?;
    emu.lw_no_count(12usize, 2usize, 520u32, 2178756u32)?;
    emu.lw_no_count(10usize, 2usize, 508u32, 2178760u32)?;
    emu.xrr_no_count(27usize, 12usize, 10usize, 2178764u32);
    emu.anr_no_count(27usize, 31usize, 27usize, 2178768u32);
    emu.lw_no_count(12usize, 2usize, 520u32, 2178772u32)?;
    emu.anr_no_count(24usize, 12usize, 10usize, 2178776u32);
    emu.xrr_no_count(10usize, 27usize, 24usize, 2178780u32);
    emu.sw_no_count(10usize, 2usize, 376u32, 2178784u32)?;
    emu.lw_no_count(31usize, 2usize, 384u32, 2178788u32)?;
    emu.sri_no_count(24usize, 31usize, 3u32, 2178792u32);
    emu.xrr_no_count(26usize, 30usize, 24usize, 2178796u32);
    emu.lw_no_count(29usize, 2usize, 500u32, 2178800u32)?;
    emu.sri_no_count(24usize, 29usize, 3u32, 2178804u32);
    emu.xrr_no_count(21usize, 11usize, 24usize, 2178808u32);
    emu.lw_no_count(30usize, 2usize, 388u32, 2178812u32)?;
    emu.sri_no_count(24usize, 30usize, 3u32, 2178816u32);
    emu.xrr_no_count(24usize, 13usize, 24usize, 2178820u32);
    emu.lw_no_count(10usize, 2usize, 380u32, 2178824u32)?;
    emu.sri_no_count(13usize, 10usize, 3u32, 2178828u32);
    emu.xrr_no_count(25usize, 14usize, 13usize, 2178832u32);
    emu.sri_no_count(14usize, 6usize, 10u32, 2178836u32);
    emu.xrr_no_count(14usize, 15usize, 14usize, 2178840u32);
    emu.sw_no_count(14usize, 2usize, 352u32, 2178844u32)?;
    emu.sri_no_count(15usize, 16usize, 10u32, 2178848u32);
    emu.xrr_no_count(27usize, 17usize, 15usize, 2178852u32);
    emu.lw_no_count(15usize, 2usize, 484u32, 2178856u32)?;
    emu.sri_no_count(15usize, 15usize, 3u32, 2178860u32);
    emu.xrr_no_count(11usize, 5usize, 15usize, 2178864u32);
    emu.sw_no_count(11usize, 2usize, 476u32, 2178868u32)?;
    emu.lw_no_count(11usize, 2usize, 468u32, 2178872u32)?;
    emu.sri_no_count(17usize, 11usize, 3u32, 2178876u32);
    emu.xrr_no_count(17usize, 7usize, 17usize, 2178880u32);
    emu.lw_no_count(14usize, 2usize, 464u32, 2178884u32)?;
    emu.sri_no_count(5usize, 14usize, 3u32, 2178888u32);
    emu.xrr_no_count(18usize, 18usize, 5usize, 2178892u32);
    emu.lw_no_count(13usize, 2usize, 472u32, 2178896u32)?;
    emu.sri_no_count(5usize, 13usize, 3u32, 2178900u32);
    emu.xrr_no_count(20usize, 20usize, 5usize, 2178904u32);
    emu.lw_no_count(12usize, 2usize, 488u32, 2178908u32)?;
    emu.sri_no_count(5usize, 12usize, 3u32, 2178912u32);
    emu.xrr_no_count(11usize, 22usize, 5usize, 2178916u32);
    emu.sw_no_count(11usize, 2usize, 460u32, 2178920u32)?;
    emu.lw_no_count(11usize, 2usize, 452u32, 2178924u32)?;
    emu.sri_no_count(5usize, 11usize, 3u32, 2178928u32);
    emu.xrr_no_count(15usize, 23usize, 5usize, 2178932u32);
    emu.sw_no_count(15usize, 2usize, 400u32, 2178936u32)?;
    emu.lw_no_count(22usize, 2usize, 448u32, 2178940u32)?;
    emu.sri_no_count(5usize, 22usize, 3u32, 2178944u32);
    emu.lw_no_count(15usize, 2usize, 436u32, 2178948u32)?;
    emu.xrr_no_count(15usize, 15usize, 5usize, 2178952u32);
    emu.sw_no_count(15usize, 2usize, 392u32, 2178956u32)?;
    emu.lw_no_count(23usize, 2usize, 480u32, 2178960u32)?;
    emu.sri_no_count(5usize, 23usize, 3u32, 2178964u32);
    emu.lw_no_count(15usize, 2usize, 456u32, 2178968u32)?;
    emu.xrr_no_count(15usize, 15usize, 5usize, 2178972u32);
    emu.sw_no_count(15usize, 2usize, 404u32, 2178976u32)?;
    emu.sw_no_count(16usize, 2usize, 420u32, 2178980u32)?;
    emu.sri_no_count(7usize, 16usize, 3u32, 2178984u32);
    emu.lw_no_count(15usize, 2usize, 356u32, 2178988u32)?;
    emu.xrr_no_count(5usize, 15usize, 7usize, 2178992u32);
    emu.sw_no_count(5usize, 2usize, 436u32, 2178996u32)?;
    emu.adi_no_count(5usize, 6usize, 0u32, 2179000u32);
    emu.sw_no_count(6usize, 2usize, 416u32, 2179004u32)?;
    emu.sri_no_count(7usize, 6usize, 3u32, 2179008u32);
    emu.lw_no_count(15usize, 2usize, 364u32, 2179012u32)?;
    emu.xrr_no_count(15usize, 15usize, 7usize, 2179016u32);
    emu.sw_no_count(15usize, 2usize, 456u32, 2179020u32)?;
    emu.sw_no_count(8usize, 2usize, 412u32, 2179024u32)?;
    emu.sri_no_count(6usize, 8usize, 3u32, 2179028u32);
    emu.lw_no_count(15usize, 2usize, 396u32, 2179032u32)?;
    emu.xrr_no_count(15usize, 15usize, 6usize, 2179036u32);
    emu.sw_no_count(15usize, 2usize, 396u32, 2179040u32)?;
    emu.lw_no_count(15usize, 2usize, 504u32, 2179044u32)?;
    emu.lw_no_count(6usize, 2usize, 496u32, 2179048u32)?;
    emu.xrr_no_count(6usize, 6usize, 15usize, 2179052u32);
    emu.lw_no_count(7usize, 2usize, 516u32, 2179056u32)?;
    emu.lw_no_count(15usize, 2usize, 424u32, 2179060u32)?;
    emu.xrr_no_count(7usize, 7usize, 15usize, 2179064u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2179068u32);
    emu.lw_no_count(28usize, 2usize, 516u32, 2179072u32)?;
    emu.xrr_no_count(7usize, 7usize, 28usize, 2179076u32);
    emu.lw_no_count(28usize, 2usize, 432u32, 2179080u32)?;
    emu.adr_no_count(28usize, 28usize, 10usize, 2179084u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2179088u32);
    emu.xrr_no_count(19usize, 1usize, 19usize, 2179092u32);
    emu.adr_no_count(28usize, 29usize, 12usize, 2179096u32);
    emu.adr_no_count(28usize, 26usize, 28usize, 2179100u32);
    emu.sw_no_count(28usize, 2usize, 504u32, 2179104u32)?;
    emu.adr_no_count(28usize, 30usize, 11usize, 2179108u32);
    emu.adi_no_count(11usize, 30usize, 0u32, 2179112u32);
    emu.adr_no_count(28usize, 21usize, 28usize, 2179116u32);
    emu.sw_no_count(28usize, 2usize, 496u32, 2179120u32)?;
    emu.adr_no_count(28usize, 10usize, 22usize, 2179124u32);
    emu.adr_no_count(28usize, 24usize, 28usize, 2179128u32);
    emu.adr_no_count(30usize, 9usize, 23usize, 2179132u32);
    emu.adr_no_count(30usize, 25usize, 30usize, 2179136u32);
    emu.adr_no_count(23usize, 14usize, 16usize, 2179140u32);
    emu.adr_no_count(23usize, 17usize, 23usize, 2179144u32);
    emu.adr_no_count(1usize, 13usize, 5usize, 2179148u32);
    emu.adr_no_count(1usize, 18usize, 1usize, 2179152u32);
    emu.adr_no_count(17usize, 31usize, 8usize, 2179156u32);
    emu.adr_no_count(12usize, 20usize, 17usize, 2179160u32);
    emu.lw_no_count(10usize, 2usize, 440u32, 2179164u32)?;
    emu.xrr_no_count(17usize, 6usize, 10usize, 2179168u32);
    emu.lw_no_count(10usize, 2usize, 408u32, 2179172u32)?;
    emu.xrr_no_count(6usize, 19usize, 10usize, 2179176u32);
    emu.lw_no_count(13usize, 2usize, 352u32, 2179180u32)?;
    emu.adr_no_count(22usize, 30usize, 13usize, 2179184u32);
    emu.adr_no_count(16usize, 28usize, 27usize, 2179188u32);
    emu.lw_no_count(10usize, 2usize, 260u32, 2179192u32)?;
    emu.adr_no_count(10usize, 7usize, 10usize, 2179196u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2179200u32);
    emu.lw_no_count(27usize, 2usize, 376u32, 2179204u32)?;
    emu.adr_no_count(27usize, 6usize, 27usize, 2179208u32);
    emu.sri_no_count(13usize, 22usize, 17u32, 2179212u32);
    emu.sli_no_count(14usize, 22usize, 15u32, 2179216u32);
    emu.orr_no_count(29usize, 14usize, 13usize, 2179220u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2179224u32);
    emu.sli_no_count(14usize, 22usize, 13u32, 2179228u32);
    emu.orr_no_count(30usize, 14usize, 13usize, 2179232u32);
    emu.sri_no_count(13usize, 16usize, 17u32, 2179236u32);
    emu.sli_no_count(14usize, 16usize, 15u32, 2179240u32);
    emu.orr_no_count(18usize, 14usize, 13usize, 2179244u32);
    emu.sri_no_count(13usize, 16usize, 19u32, 2179248u32);
    emu.sli_no_count(14usize, 16usize, 13u32, 2179252u32);
    emu.orr_no_count(17usize, 14usize, 13usize, 2179256u32);
    emu.sri_no_count(13usize, 22usize, 7u32, 2179260u32);
    emu.sli_no_count(14usize, 22usize, 25u32, 2179264u32);
    emu.orr_no_count(6usize, 14usize, 13usize, 2179268u32);
    emu.sri_no_count(13usize, 22usize, 18u32, 2179272u32);
    emu.sli_no_count(14usize, 22usize, 14u32, 2179276u32);
    emu.orr_no_count(7usize, 14usize, 13usize, 2179280u32);
    emu.adi_no_count(5usize, 16usize, 0u32, 2179284u32);
    emu.sri_no_count(13usize, 16usize, 7u32, 2179288u32);
    emu.sli_no_count(14usize, 16usize, 25u32, 2179292u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2179296u32);
    emu.sri_no_count(14usize, 16usize, 18u32, 2179300u32);
    emu.sli_no_count(16usize, 16usize, 14u32, 2179304u32);
    emu.adi_no_count(24usize, 5usize, 0u32, 2179308u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2179312u32);
    emu.xrr_no_count(16usize, 29usize, 30usize, 2179316u32);
    emu.xrr_no_count(17usize, 18usize, 17usize, 2179320u32);
    emu.xrr_no_count(6usize, 6usize, 7usize, 2179324u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2179328u32);
    emu.adr_no_count(27usize, 27usize, 10usize, 2179332u32);
    emu.lw_no_count(14usize, 2usize, 512u32, 2179336u32)?;
    emu.adr_no_count(10usize, 10usize, 14usize, 2179340u32);
    emu.sri_no_count(14usize, 10usize, 6u32, 2179344u32);
    emu.sli_no_count(7usize, 10usize, 26u32, 2179348u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2179352u32);
    emu.sri_no_count(7usize, 10usize, 11u32, 2179356u32);
    emu.sli_no_count(28usize, 10usize, 21u32, 2179360u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2179364u32);
    emu.sri_no_count(28usize, 10usize, 25u32, 2179368u32);
    emu.sli_no_count(29usize, 10usize, 7u32, 2179372u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2179376u32);
    emu.sri_no_count(29usize, 27usize, 2u32, 2179380u32);
    emu.sli_no_count(30usize, 27usize, 30u32, 2179384u32);
    emu.orr_no_count(30usize, 30usize, 29usize, 2179388u32);
    emu.sri_no_count(29usize, 27usize, 13u32, 2179392u32);
    emu.sli_no_count(18usize, 27usize, 19u32, 2179396u32);
    emu.orr_no_count(18usize, 18usize, 29usize, 2179400u32);
    emu.sri_no_count(29usize, 27usize, 22u32, 2179404u32);
    emu.sli_no_count(19usize, 27usize, 10u32, 2179408u32);
    emu.orr_no_count(19usize, 19usize, 29usize, 2179412u32);
    emu.lw_no_count(8usize, 2usize, 508u32, 2179416u32)?;
    emu.lw_no_count(26usize, 2usize, 360u32, 2179420u32)?;
    emu.xrr_no_count(29usize, 26usize, 8usize, 2179424u32);
    emu.anr_no_count(29usize, 27usize, 29usize, 2179428u32);
    emu.anr_no_count(20usize, 26usize, 8usize, 2179432u32);
    emu.xrr_no_count(20usize, 29usize, 20usize, 2179436u32);
    emu.adi_no_count(5usize, 22usize, 0u32, 2179440u32);
    emu.sri_no_count(29usize, 22usize, 10u32, 2179444u32);
    emu.xrr_no_count(16usize, 16usize, 29usize, 2179448u32);
    emu.sw_no_count(24usize, 2usize, 492u32, 2179452u32)?;
    emu.sri_no_count(29usize, 24usize, 10u32, 2179456u32);
    emu.xrr_no_count(17usize, 17usize, 29usize, 2179460u32);
    emu.sri_no_count(29usize, 22usize, 3u32, 2179464u32);
    emu.sw_no_count(22usize, 2usize, 408u32, 2179468u32)?;
    emu.xrr_no_count(6usize, 6usize, 29usize, 2179472u32);
    emu.sw_no_count(6usize, 2usize, 364u32, 2179476u32)?;
    emu.sri_no_count(6usize, 24usize, 3u32, 2179480u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2179484u32);
    emu.sw_no_count(13usize, 2usize, 352u32, 2179488u32)?;
    emu.xrr_no_count(13usize, 14usize, 7usize, 2179492u32);
    emu.lw_no_count(14usize, 2usize, 516u32, 2179496u32)?;
    emu.adr_no_count(14usize, 14usize, 11usize, 2179500u32);
    emu.adi_no_count(7usize, 15usize, 0u32, 2179504u32);
    emu.lw_no_count(15usize, 2usize, 444u32, 2179508u32)?;
    emu.xrr_no_count(6usize, 15usize, 7usize, 2179512u32);
    emu.anr_no_count(6usize, 10usize, 6usize, 2179516u32);
    emu.xrr_no_count(6usize, 6usize, 7usize, 2179520u32);
    emu.adi_no_count(21usize, 7usize, 0u32, 2179524u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2179528u32);
    emu.xrr_no_count(6usize, 30usize, 18usize, 2179532u32);
    emu.lw_no_count(29usize, 2usize, 496u32, 2179536u32)?;
    emu.adr_no_count(29usize, 29usize, 16usize, 2179540u32);
    emu.sw_no_count(29usize, 2usize, 496u32, 2179544u32)?;
    emu.lw_no_count(9usize, 2usize, 504u32, 2179548u32)?;
    emu.adr_no_count(18usize, 9usize, 17usize, 2179552u32);
    emu.xrr_no_count(11usize, 13usize, 28usize, 2179556u32);
    emu.xrr_no_count(13usize, 6usize, 19usize, 2179560u32);
    emu.lw_no_count(16usize, 2usize, 256u32, 2179564u32)?;
    emu.adr_no_count(14usize, 14usize, 16usize, 2179568u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2179572u32);
    emu.adr_no_count(13usize, 13usize, 20usize, 2179576u32);
    emu.sri_no_count(14usize, 29usize, 17u32, 2179580u32);
    emu.sli_no_count(16usize, 29usize, 15u32, 2179584u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2179588u32);
    emu.sri_no_count(16usize, 29usize, 19u32, 2179592u32);
    emu.sli_no_count(17usize, 29usize, 13u32, 2179596u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2179600u32);
    emu.sri_no_count(17usize, 18usize, 17u32, 2179604u32);
    emu.sli_no_count(6usize, 18usize, 15u32, 2179608u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2179612u32);
    emu.sri_no_count(6usize, 18usize, 19u32, 2179616u32);
    emu.sli_no_count(7usize, 18usize, 13u32, 2179620u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2179624u32);
    emu.sri_no_count(7usize, 18usize, 7u32, 2179628u32);
    emu.sli_no_count(28usize, 18usize, 25u32, 2179632u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2179636u32);
    emu.sri_no_count(28usize, 29usize, 7u32, 2179640u32);
    emu.sli_no_count(30usize, 29usize, 25u32, 2179644u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2179648u32);
    emu.sri_no_count(30usize, 18usize, 18u32, 2179652u32);
    emu.sli_no_count(9usize, 18usize, 14u32, 2179656u32);
    emu.adi_no_count(19usize, 18usize, 0u32, 2179660u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2179664u32);
    emu.sri_no_count(9usize, 29usize, 18u32, 2179668u32);
    emu.sli_no_count(18usize, 29usize, 14u32, 2179672u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2179676u32);
    emu.xrr_no_count(14usize, 14usize, 16usize, 2179680u32);
    emu.xrr_no_count(16usize, 17usize, 6usize, 2179684u32);
    emu.xrr_no_count(17usize, 7usize, 30usize, 2179688u32);
    emu.xrr_no_count(6usize, 28usize, 9usize, 2179692u32);
    emu.sri_no_count(7usize, 29usize, 10u32, 2179696u32);
    emu.xrr_no_count(7usize, 14usize, 7usize, 2179700u32);
    emu.sw_no_count(19usize, 2usize, 440u32, 2179704u32)?;
    emu.sri_no_count(14usize, 19usize, 10u32, 2179708u32);
    emu.xrr_no_count(16usize, 16usize, 14usize, 2179712u32);
    emu.sri_no_count(14usize, 19usize, 3u32, 2179716u32);
    emu.xrr_no_count(14usize, 17usize, 14usize, 2179720u32);
    emu.sw_no_count(14usize, 2usize, 388u32, 2179724u32)?;
    emu.sri_no_count(14usize, 29usize, 3u32, 2179728u32);
    emu.xrr_no_count(14usize, 6usize, 14usize, 2179732u32);
    emu.sw_no_count(14usize, 2usize, 348u32, 2179736u32)?;
    emu.adr_no_count(29usize, 13usize, 11usize, 2179740u32);
    emu.lw_no_count(22usize, 2usize, 520u32, 2179744u32)?;
    emu.adr_no_count(22usize, 11usize, 22usize, 2179748u32);
    emu.sri_no_count(11usize, 22usize, 6u32, 2179752u32);
    emu.sli_no_count(13usize, 22usize, 26u32, 2179756u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2179760u32);
    emu.sri_no_count(11usize, 22usize, 11u32, 2179764u32);
    emu.sli_no_count(17usize, 22usize, 21u32, 2179768u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2179772u32);
    emu.sri_no_count(11usize, 22usize, 25u32, 2179776u32);
    emu.sli_no_count(6usize, 22usize, 7u32, 2179780u32);
    emu.orr_no_count(6usize, 6usize, 11usize, 2179784u32);
    emu.adr_no_count(24usize, 15usize, 31usize, 2179788u32);
    emu.xrr_no_count(11usize, 10usize, 15usize, 2179792u32);
    emu.anr_no_count(11usize, 22usize, 11usize, 2179796u32);
    emu.xrr_no_count(28usize, 11usize, 15usize, 2179800u32);
    emu.sri_no_count(11usize, 29usize, 2u32, 2179804u32);
    emu.sli_no_count(30usize, 29usize, 30u32, 2179808u32);
    emu.orr_no_count(30usize, 30usize, 11usize, 2179812u32);
    emu.sri_no_count(11usize, 29usize, 13u32, 2179816u32);
    emu.sli_no_count(31usize, 29usize, 19u32, 2179820u32);
    emu.orr_no_count(31usize, 31usize, 11usize, 2179824u32);
    emu.sri_no_count(11usize, 29usize, 22u32, 2179828u32);
    emu.sli_no_count(9usize, 29usize, 10u32, 2179832u32);
    emu.orr_no_count(9usize, 9usize, 11usize, 2179836u32);
    emu.xrr_no_count(11usize, 27usize, 26usize, 2179840u32);
    emu.anr_no_count(11usize, 29usize, 11usize, 2179844u32);
    emu.anr_no_count(18usize, 27usize, 26usize, 2179848u32);
    emu.xrr_no_count(11usize, 11usize, 18usize, 2179852u32);
    emu.adr_no_count(18usize, 12usize, 7usize, 2179856u32);
    emu.adr_no_count(1usize, 1usize, 16usize, 2179860u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2179864u32);
    emu.lw_no_count(12usize, 2usize, 500u32, 2179868u32)?;
    emu.adr_no_count(12usize, 21usize, 12usize, 2179872u32);
    emu.adr_no_count(12usize, 12usize, 28usize, 2179876u32);
    emu.xrr_no_count(16usize, 30usize, 31usize, 2179880u32);
    emu.xrr_no_count(31usize, 13usize, 6usize, 2179884u32);
    emu.xrr_no_count(7usize, 16usize, 9usize, 2179888u32);
    emu.adi_no_count(6usize, 18usize, 0u32, 2179892u32);
    emu.sri_no_count(13usize, 18usize, 17u32, 2179896u32);
    emu.sli_no_count(16usize, 18usize, 15u32, 2179900u32);
    emu.orr_no_count(18usize, 16usize, 13usize, 2179904u32);
    emu.sri_no_count(13usize, 6usize, 19u32, 2179908u32);
    emu.sli_no_count(16usize, 6usize, 13u32, 2179912u32);
    emu.orr_no_count(20usize, 16usize, 13usize, 2179916u32);
    emu.sri_no_count(13usize, 1usize, 17u32, 2179920u32);
    emu.sli_no_count(16usize, 1usize, 15u32, 2179924u32);
    emu.orr_no_count(21usize, 16usize, 13usize, 2179928u32);
    emu.sri_no_count(13usize, 1usize, 19u32, 2179932u32);
    emu.sli_no_count(16usize, 1usize, 13u32, 2179936u32);
    emu.orr_no_count(25usize, 16usize, 13usize, 2179940u32);
    emu.sri_no_count(13usize, 6usize, 7u32, 2179944u32);
    emu.sli_no_count(16usize, 6usize, 25u32, 2179948u32);
    emu.orr_no_count(17usize, 16usize, 13usize, 2179952u32);
    emu.sri_no_count(13usize, 6usize, 18u32, 2179956u32);
    emu.sli_no_count(16usize, 6usize, 14u32, 2179960u32);
    emu.adi_no_count(30usize, 6usize, 0u32, 2179964u32);
    emu.orr_no_count(6usize, 16usize, 13usize, 2179968u32);
    emu.sri_no_count(13usize, 1usize, 7u32, 2179972u32);
    emu.sli_no_count(16usize, 1usize, 25u32, 2179976u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2179980u32);
    emu.sri_no_count(16usize, 1usize, 18u32, 2179984u32);
    emu.sli_no_count(28usize, 1usize, 14u32, 2179988u32);
    emu.sw_no_count(1usize, 2usize, 444u32, 2179992u32)?;
    emu.orr_no_count(16usize, 28usize, 16usize, 2179996u32);
    emu.lw_no_count(28usize, 2usize, 252u32, 2180000u32)?;
    emu.adr_no_count(12usize, 12usize, 28usize, 2180004u32);
    emu.adr_no_count(12usize, 12usize, 31usize, 2180008u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2180012u32);
    emu.xrr_no_count(7usize, 18usize, 20usize, 2180016u32);
    emu.xrr_no_count(28usize, 21usize, 25usize, 2180020u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2180024u32);
    emu.xrr_no_count(13usize, 13usize, 16usize, 2180028u32);
    emu.sw_no_count(30usize, 2usize, 504u32, 2180032u32)?;
    emu.sri_no_count(16usize, 30usize, 10u32, 2180036u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2180040u32);
    emu.sri_no_count(6usize, 1usize, 10u32, 2180044u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2180048u32);
    emu.sri_no_count(7usize, 30usize, 3u32, 2180052u32);
    emu.xrr_no_count(14usize, 17usize, 7usize, 2180056u32);
    emu.sw_no_count(14usize, 2usize, 332u32, 2180060u32)?;
    emu.sri_no_count(17usize, 1usize, 3u32, 2180064u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2180068u32);
    emu.sw_no_count(13usize, 2usize, 376u32, 2180072u32)?;
    emu.adr_no_count(23usize, 23usize, 16usize, 2180076u32);
    emu.lw_no_count(15usize, 2usize, 476u32, 2180080u32)?;
    emu.lw_no_count(13usize, 2usize, 468u32, 2180084u32)?;
    emu.adr_no_count(15usize, 15usize, 13usize, 2180088u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2180092u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2180096u32);
    emu.adr_no_count(1usize, 11usize, 12usize, 2180100u32);
    emu.adr_no_count(20usize, 12usize, 8usize, 2180104u32);
    emu.sri_no_count(11usize, 20usize, 6u32, 2180108u32);
    emu.sli_no_count(12usize, 20usize, 26u32, 2180112u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2180116u32);
    emu.sri_no_count(12usize, 20usize, 11u32, 2180120u32);
    emu.sli_no_count(13usize, 20usize, 21u32, 2180124u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2180128u32);
    emu.sri_no_count(13usize, 20usize, 25u32, 2180132u32);
    emu.sli_no_count(15usize, 20usize, 7u32, 2180136u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2180140u32);
    emu.lw_no_count(7usize, 2usize, 472u32, 2180144u32)?;
    emu.adr_no_count(7usize, 7usize, 10usize, 2180148u32);
    emu.xrr_no_count(15usize, 22usize, 10usize, 2180152u32);
    emu.anr_no_count(15usize, 20usize, 15usize, 2180156u32);
    emu.xrr_no_count(10usize, 15usize, 10usize, 2180160u32);
    emu.sri_no_count(15usize, 1usize, 2u32, 2180164u32);
    emu.sli_no_count(16usize, 1usize, 30u32, 2180168u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2180172u32);
    emu.sri_no_count(16usize, 1usize, 13u32, 2180176u32);
    emu.sli_no_count(17usize, 1usize, 19u32, 2180180u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2180184u32);
    emu.sri_no_count(17usize, 1usize, 22u32, 2180188u32);
    emu.sli_no_count(6usize, 1usize, 10u32, 2180192u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2180196u32);
    emu.xrr_no_count(6usize, 29usize, 27usize, 2180200u32);
    emu.anr_no_count(6usize, 1usize, 6usize, 2180204u32);
    emu.anr_no_count(28usize, 29usize, 27usize, 2180208u32);
    emu.xrr_no_count(6usize, 6usize, 28usize, 2180212u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2180216u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2180220u32);
    emu.xrr_no_count(12usize, 15usize, 16usize, 2180224u32);
    emu.adi_no_count(24usize, 23usize, 0u32, 2180228u32);
    emu.sri_no_count(15usize, 23usize, 17u32, 2180232u32);
    emu.sli_no_count(16usize, 23usize, 15u32, 2180236u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2180240u32);
    emu.sri_no_count(16usize, 23usize, 19u32, 2180244u32);
    emu.sli_no_count(28usize, 23usize, 13u32, 2180248u32);
    emu.orr_no_count(16usize, 28usize, 16usize, 2180252u32);
    emu.sri_no_count(28usize, 5usize, 17u32, 2180256u32);
    emu.sli_no_count(30usize, 5usize, 15u32, 2180260u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2180264u32);
    emu.sri_no_count(30usize, 5usize, 19u32, 2180268u32);
    emu.sli_no_count(31usize, 5usize, 13u32, 2180272u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2180276u32);
    emu.sri_no_count(31usize, 5usize, 7u32, 2180280u32);
    emu.sli_no_count(9usize, 5usize, 25u32, 2180284u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2180288u32);
    emu.sri_no_count(9usize, 23usize, 7u32, 2180292u32);
    emu.sli_no_count(18usize, 23usize, 25u32, 2180296u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2180300u32);
    emu.sri_no_count(18usize, 5usize, 18u32, 2180304u32);
    emu.sli_no_count(19usize, 5usize, 14u32, 2180308u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2180312u32);
    emu.sri_no_count(19usize, 23usize, 18u32, 2180316u32);
    emu.sli_no_count(23usize, 23usize, 14u32, 2180320u32);
    emu.orr_no_count(19usize, 23usize, 19usize, 2180324u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2180328u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2180332u32);
    emu.xrr_no_count(13usize, 15usize, 16usize, 2180336u32);
    emu.xrr_no_count(15usize, 28usize, 30usize, 2180340u32);
    emu.xrr_no_count(16usize, 31usize, 18usize, 2180344u32);
    emu.xrr_no_count(17usize, 9usize, 19usize, 2180348u32);
    emu.lw_no_count(28usize, 2usize, 248u32, 2180352u32)?;
    emu.adr_no_count(10usize, 10usize, 28usize, 2180356u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2180360u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2180364u32);
    emu.sw_no_count(24usize, 2usize, 500u32, 2180368u32)?;
    emu.sri_no_count(11usize, 24usize, 10u32, 2180372u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2180376u32);
    emu.sw_no_count(5usize, 2usize, 476u32, 2180380u32)?;
    emu.sri_no_count(13usize, 5usize, 10u32, 2180384u32);
    emu.xrr_no_count(13usize, 15usize, 13usize, 2180388u32);
    emu.sri_no_count(15usize, 5usize, 3u32, 2180392u32);
    emu.xrr_no_count(14usize, 16usize, 15usize, 2180396u32);
    emu.sw_no_count(14usize, 2usize, 384u32, 2180400u32)?;
    emu.sri_no_count(15usize, 24usize, 3u32, 2180404u32);
    emu.xrr_no_count(14usize, 17usize, 15usize, 2180408u32);
    emu.sw_no_count(14usize, 2usize, 324u32, 2180412u32)?;
    emu.adr_no_count(21usize, 12usize, 10usize, 2180416u32);
    emu.adr_no_count(19usize, 10usize, 26usize, 2180420u32);
    emu.lw_no_count(5usize, 2usize, 484u32, 2180424u32)?;
    emu.lw_no_count(10usize, 2usize, 404u32, 2180428u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2180432u32);
    emu.lw_no_count(10usize, 2usize, 492u32, 2180436u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2180440u32);
    emu.adr_no_count(24usize, 5usize, 11usize, 2180444u32);
    emu.lw_no_count(8usize, 2usize, 480u32, 2180448u32)?;
    emu.lw_no_count(10usize, 2usize, 392u32, 2180452u32)?;
    emu.adr_no_count(8usize, 10usize, 8usize, 2180456u32);
    emu.lw_no_count(10usize, 2usize, 496u32, 2180460u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2180464u32);
    emu.adr_no_count(26usize, 8usize, 13usize, 2180468u32);
    emu.sri_no_count(10usize, 19usize, 6u32, 2180472u32);
    emu.sli_no_count(13usize, 19usize, 26u32, 2180476u32);
    emu.orr_no_count(13usize, 13usize, 10usize, 2180480u32);
    emu.sri_no_count(10usize, 19usize, 11u32, 2180484u32);
    emu.sli_no_count(15usize, 19usize, 21u32, 2180488u32);
    emu.orr_no_count(15usize, 15usize, 10usize, 2180492u32);
    emu.sri_no_count(10usize, 19usize, 25u32, 2180496u32);
    emu.sli_no_count(11usize, 19usize, 7u32, 2180500u32);
    emu.orr_no_count(6usize, 11usize, 10usize, 2180504u32);
    emu.lw_no_count(10usize, 2usize, 464u32, 2180508u32)?;
    emu.adr_no_count(10usize, 10usize, 22usize, 2180512u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2180516u32)?;
    emu.xrr_no_count(10usize, 20usize, 22usize, 2180520u32);
    emu.anr_no_count(10usize, 19usize, 10usize, 2180524u32);
    emu.xrr_no_count(16usize, 10usize, 22usize, 2180528u32);
    emu.sri_no_count(10usize, 21usize, 2u32, 2180532u32);
    emu.sli_no_count(11usize, 21usize, 30u32, 2180536u32);
    emu.orr_no_count(31usize, 11usize, 10usize, 2180540u32);
    emu.sri_no_count(10usize, 21usize, 13u32, 2180544u32);
    emu.sli_no_count(11usize, 21usize, 19u32, 2180548u32);
    emu.orr_no_count(30usize, 11usize, 10usize, 2180552u32);
    emu.sri_no_count(10usize, 21usize, 22u32, 2180556u32);
    emu.sli_no_count(11usize, 21usize, 10u32, 2180560u32);
    emu.orr_no_count(5usize, 11usize, 10usize, 2180564u32);
    emu.xrr_no_count(10usize, 1usize, 29usize, 2180568u32);
    emu.anr_no_count(10usize, 21usize, 10usize, 2180572u32);
    emu.anr_no_count(11usize, 1usize, 29usize, 2180576u32);
    emu.xrr_no_count(8usize, 10usize, 11usize, 2180580u32);
    emu.sri_no_count(10usize, 24usize, 17u32, 2180584u32);
    emu.sli_no_count(11usize, 24usize, 15u32, 2180588u32);
    emu.orr_no_count(18usize, 11usize, 10usize, 2180592u32);
    emu.sri_no_count(10usize, 24usize, 19u32, 2180596u32);
    emu.sli_no_count(11usize, 24usize, 13u32, 2180600u32);
    emu.orr_no_count(22usize, 11usize, 10usize, 2180604u32);
    emu.sri_no_count(10usize, 26usize, 17u32, 2180608u32);
    emu.sli_no_count(11usize, 26usize, 15u32, 2180612u32);
    emu.orr_no_count(25usize, 11usize, 10usize, 2180616u32);
    emu.sri_no_count(10usize, 26usize, 19u32, 2180620u32);
    emu.sli_no_count(12usize, 26usize, 13u32, 2180624u32);
    emu.orr_no_count(12usize, 12usize, 10usize, 2180628u32);
    emu.sri_no_count(10usize, 24usize, 7u32, 2180632u32);
    emu.sli_no_count(11usize, 24usize, 25u32, 2180636u32);
    emu.orr_no_count(17usize, 11usize, 10usize, 2180640u32);
    emu.sri_no_count(10usize, 24usize, 18u32, 2180644u32);
    emu.sli_no_count(28usize, 24usize, 14u32, 2180648u32);
    emu.orr_no_count(11usize, 28usize, 10usize, 2180652u32);
    emu.adi_no_count(10usize, 26usize, 0u32, 2180656u32);
    emu.sri_no_count(28usize, 26usize, 7u32, 2180660u32);
    emu.sli_no_count(9usize, 26usize, 25u32, 2180664u32);
    emu.orr_no_count(28usize, 9usize, 28usize, 2180668u32);
    emu.sri_no_count(9usize, 26usize, 18u32, 2180672u32);
    emu.sli_no_count(26usize, 26usize, 14u32, 2180676u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2180680u32);
    emu.orr_no_count(9usize, 26usize, 9usize, 2180684u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2180688u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2180692u32);
    emu.xrr_no_count(15usize, 31usize, 30usize, 2180696u32);
    emu.xrr_no_count(7usize, 18usize, 22usize, 2180700u32);
    emu.xrr_no_count(12usize, 25usize, 12usize, 2180704u32);
    emu.xrr_no_count(10usize, 17usize, 11usize, 2180708u32);
    emu.xrr_no_count(11usize, 28usize, 9usize, 2180712u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2180716u32);
    emu.xrr_no_count(15usize, 15usize, 5usize, 2180720u32);
    emu.sri_no_count(5usize, 24usize, 10u32, 2180724u32);
    emu.xrr_no_count(5usize, 7usize, 5usize, 2180728u32);
    emu.sw_no_count(23usize, 2usize, 472u32, 2180732u32)?;
    emu.sri_no_count(6usize, 23usize, 10u32, 2180736u32);
    emu.xrr_no_count(12usize, 12usize, 6usize, 2180740u32);
    emu.sri_no_count(6usize, 24usize, 3u32, 2180744u32);
    emu.sw_no_count(24usize, 2usize, 404u32, 2180748u32)?;
    emu.xrr_no_count(10usize, 10usize, 6usize, 2180752u32);
    emu.sw_no_count(10usize, 2usize, 340u32, 2180756u32)?;
    emu.sri_no_count(10usize, 23usize, 3u32, 2180760u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2180764u32);
    emu.sw_no_count(10usize, 2usize, 360u32, 2180768u32)?;
    emu.lw_no_count(10usize, 2usize, 244u32, 2180772u32)?;
    emu.adr_no_count(16usize, 16usize, 10usize, 2180776u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2180780u32);
    emu.adr_no_count(10usize, 15usize, 8usize, 2180784u32);
    emu.lw_no_count(11usize, 2usize, 448u32, 2180788u32)?;
    emu.lw_no_count(14usize, 2usize, 400u32, 2180792u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2180796u32);
    emu.lw_no_count(14usize, 2usize, 440u32, 2180800u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2180804u32);
    emu.adr_no_count(28usize, 11usize, 5usize, 2180808u32);
    emu.lw_no_count(11usize, 2usize, 452u32, 2180812u32)?;
    emu.lw_no_count(14usize, 2usize, 460u32, 2180816u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2180820u32);
    emu.lw_no_count(15usize, 2usize, 504u32, 2180824u32)?;
    emu.adr_no_count(11usize, 11usize, 15usize, 2180828u32);
    emu.adr_no_count(14usize, 11usize, 12usize, 2180832u32);
    emu.adr_no_count(23usize, 10usize, 13usize, 2180836u32);
    emu.adr_no_count(27usize, 13usize, 27usize, 2180840u32);
    emu.sri_no_count(11usize, 28usize, 17u32, 2180844u32);
    emu.sli_no_count(12usize, 28usize, 15u32, 2180848u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2180852u32);
    emu.sri_no_count(12usize, 28usize, 19u32, 2180856u32);
    emu.sli_no_count(13usize, 28usize, 13u32, 2180860u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2180864u32);
    emu.sri_no_count(13usize, 14usize, 17u32, 2180868u32);
    emu.sli_no_count(15usize, 14usize, 15u32, 2180872u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2180876u32);
    emu.sri_no_count(15usize, 14usize, 19u32, 2180880u32);
    emu.sli_no_count(16usize, 14usize, 13u32, 2180884u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2180888u32);
    emu.sri_no_count(16usize, 14usize, 7u32, 2180892u32);
    emu.sli_no_count(17usize, 14usize, 25u32, 2180896u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2180900u32);
    emu.sri_no_count(17usize, 28usize, 7u32, 2180904u32);
    emu.sli_no_count(5usize, 28usize, 25u32, 2180908u32);
    emu.orr_no_count(5usize, 5usize, 17usize, 2180912u32);
    emu.sri_no_count(17usize, 14usize, 18u32, 2180916u32);
    emu.sli_no_count(6usize, 14usize, 14u32, 2180920u32);
    emu.orr_no_count(6usize, 6usize, 17usize, 2180924u32);
    emu.sri_no_count(17usize, 28usize, 18u32, 2180928u32);
    emu.sli_no_count(7usize, 28usize, 14u32, 2180932u32);
    emu.adi_no_count(10usize, 28usize, 0u32, 2180936u32);
    emu.orr_no_count(7usize, 7usize, 17usize, 2180940u32);
    emu.sri_no_count(17usize, 27usize, 6u32, 2180944u32);
    emu.sli_no_count(28usize, 27usize, 26u32, 2180948u32);
    emu.orr_no_count(28usize, 28usize, 17usize, 2180952u32);
    emu.sri_no_count(17usize, 27usize, 11u32, 2180956u32);
    emu.sli_no_count(30usize, 27usize, 21u32, 2180960u32);
    emu.orr_no_count(30usize, 30usize, 17usize, 2180964u32);
    emu.sri_no_count(17usize, 27usize, 25u32, 2180968u32);
    emu.sli_no_count(31usize, 27usize, 7u32, 2180972u32);
    emu.orr_no_count(31usize, 31usize, 17usize, 2180976u32);
    emu.lw_no_count(17usize, 2usize, 468u32, 2180980u32)?;
    emu.adr_no_count(17usize, 17usize, 20usize, 2180984u32);
    emu.xrr_no_count(9usize, 19usize, 20usize, 2180988u32);
    emu.anr_no_count(9usize, 27usize, 9usize, 2180992u32);
    emu.xrr_no_count(9usize, 9usize, 20usize, 2180996u32);
    emu.sri_no_count(18usize, 23usize, 2u32, 2181000u32);
    emu.sli_no_count(20usize, 23usize, 30u32, 2181004u32);
    emu.orr_no_count(18usize, 20usize, 18usize, 2181008u32);
    emu.sri_no_count(20usize, 23usize, 13u32, 2181012u32);
    emu.sli_no_count(22usize, 23usize, 19u32, 2181016u32);
    emu.orr_no_count(20usize, 22usize, 20usize, 2181020u32);
    emu.sri_no_count(22usize, 23usize, 22u32, 2181024u32);
    emu.sli_no_count(25usize, 23usize, 10u32, 2181028u32);
    emu.orr_no_count(22usize, 25usize, 22usize, 2181032u32);
    emu.xrr_no_count(25usize, 21usize, 1usize, 2181036u32);
    emu.anr_no_count(25usize, 23usize, 25usize, 2181040u32);
    emu.anr_no_count(26usize, 21usize, 1usize, 2181044u32);
    emu.xrr_no_count(25usize, 25usize, 26usize, 2181048u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2181052u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2181056u32);
    emu.xrr_no_count(12usize, 16usize, 6usize, 2181060u32);
    emu.xrr_no_count(15usize, 5usize, 7usize, 2181064u32);
    emu.xrr_no_count(16usize, 28usize, 30usize, 2181068u32);
    emu.lw_no_count(8usize, 2usize, 464u32, 2181072u32)?;
    emu.adr_no_count(8usize, 8usize, 9usize, 2181076u32);
    emu.xrr_no_count(5usize, 18usize, 20usize, 2181080u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2181084u32);
    emu.sri_no_count(6usize, 10usize, 10u32, 2181088u32);
    emu.xrr_no_count(11usize, 11usize, 6usize, 2181092u32);
    emu.sw_no_count(14usize, 2usize, 460u32, 2181096u32)?;
    emu.sri_no_count(6usize, 14usize, 10u32, 2181100u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2181104u32);
    emu.sri_no_count(6usize, 14usize, 3u32, 2181108u32);
    emu.xrr_no_count(10usize, 12usize, 6usize, 2181112u32);
    emu.sw_no_count(10usize, 2usize, 380u32, 2181116u32)?;
    emu.sri_no_count(12usize, 18usize, 3u32, 2181120u32);
    emu.sw_no_count(18usize, 2usize, 400u32, 2181124u32)?;
    emu.xrr_no_count(12usize, 15usize, 12usize, 2181128u32);
    emu.sw_no_count(12usize, 2usize, 320u32, 2181132u32)?;
    emu.xrr_no_count(12usize, 16usize, 31usize, 2181136u32);
    emu.xrr_no_count(15usize, 5usize, 22usize, 2181140u32);
    emu.lw_no_count(16usize, 2usize, 488u32, 2181144u32)?;
    emu.lw_no_count(10usize, 2usize, 396u32, 2181148u32)?;
    emu.adr_no_count(16usize, 10usize, 16usize, 2181152u32);
    emu.lw_no_count(10usize, 2usize, 444u32, 2181156u32)?;
    emu.adr_no_count(16usize, 16usize, 10usize, 2181160u32);
    emu.adr_no_count(22usize, 16usize, 11usize, 2181164u32);
    emu.lw_no_count(11usize, 2usize, 412u32, 2181168u32)?;
    emu.lw_no_count(14usize, 2usize, 456u32, 2181172u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2181176u32);
    emu.lw_no_count(14usize, 2usize, 500u32, 2181180u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2181184u32);
    emu.adr_no_count(28usize, 11usize, 13usize, 2181188u32);
    emu.lw_no_count(11usize, 2usize, 240u32, 2181192u32)?;
    emu.adr_no_count(8usize, 8usize, 11usize, 2181196u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2181200u32);
    emu.adr_no_count(11usize, 15usize, 25usize, 2181204u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2181208u32);
    emu.adr_no_count(6usize, 12usize, 29usize, 2181212u32);
    emu.sri_no_count(12usize, 22usize, 17u32, 2181216u32);
    emu.sli_no_count(13usize, 22usize, 15u32, 2181220u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2181224u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2181228u32);
    emu.sli_no_count(14usize, 22usize, 13u32, 2181232u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2181236u32);
    emu.sri_no_count(14usize, 28usize, 17u32, 2181240u32);
    emu.sli_no_count(15usize, 28usize, 15u32, 2181244u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2181248u32);
    emu.sri_no_count(15usize, 28usize, 19u32, 2181252u32);
    emu.sli_no_count(16usize, 28usize, 13u32, 2181256u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2181260u32);
    emu.sri_no_count(16usize, 22usize, 7u32, 2181264u32);
    emu.sli_no_count(5usize, 22usize, 25u32, 2181268u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2181272u32);
    emu.sri_no_count(5usize, 22usize, 18u32, 2181276u32);
    emu.sli_no_count(7usize, 22usize, 14u32, 2181280u32);
    emu.orr_no_count(5usize, 7usize, 5usize, 2181284u32);
    emu.adi_no_count(29usize, 28usize, 0u32, 2181288u32);
    emu.sri_no_count(7usize, 28usize, 7u32, 2181292u32);
    emu.sli_no_count(28usize, 28usize, 25u32, 2181296u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2181300u32);
    emu.sri_no_count(28usize, 29usize, 18u32, 2181304u32);
    emu.sli_no_count(30usize, 29usize, 14u32, 2181308u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2181312u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2181316u32);
    emu.xrr_no_count(15usize, 14usize, 15usize, 2181320u32);
    emu.xrr_no_count(13usize, 16usize, 5usize, 2181324u32);
    emu.xrr_no_count(16usize, 7usize, 28usize, 2181328u32);
    emu.sri_no_count(14usize, 6usize, 6u32, 2181332u32);
    emu.sli_no_count(5usize, 6usize, 26u32, 2181336u32);
    emu.orr_no_count(5usize, 5usize, 14usize, 2181340u32);
    emu.sri_no_count(14usize, 6usize, 11u32, 2181344u32);
    emu.sli_no_count(7usize, 6usize, 21u32, 2181348u32);
    emu.orr_no_count(7usize, 7usize, 14usize, 2181352u32);
    emu.sri_no_count(14usize, 6usize, 25u32, 2181356u32);
    emu.sli_no_count(28usize, 6usize, 7u32, 2181360u32);
    emu.orr_no_count(28usize, 28usize, 14usize, 2181364u32);
    emu.lw_no_count(14usize, 2usize, 484u32, 2181368u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2181372u32);
    emu.xrr_no_count(30usize, 27usize, 19usize, 2181376u32);
    emu.anr_no_count(30usize, 6usize, 30usize, 2181380u32);
    emu.xrr_no_count(30usize, 30usize, 19usize, 2181384u32);
    emu.sri_no_count(31usize, 11usize, 2u32, 2181388u32);
    emu.sli_no_count(8usize, 11usize, 30u32, 2181392u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2181396u32);
    emu.sri_no_count(8usize, 11usize, 13u32, 2181400u32);
    emu.sli_no_count(9usize, 11usize, 19u32, 2181404u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2181408u32);
    emu.sri_no_count(9usize, 11usize, 22u32, 2181412u32);
    emu.sli_no_count(19usize, 11usize, 10u32, 2181416u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2181420u32);
    emu.xrr_no_count(19usize, 23usize, 21usize, 2181424u32);
    emu.anr_no_count(19usize, 11usize, 19usize, 2181428u32);
    emu.anr_no_count(20usize, 23usize, 21usize, 2181432u32);
    emu.xrr_no_count(19usize, 19usize, 20usize, 2181436u32);
    emu.sri_no_count(20usize, 22usize, 10u32, 2181440u32);
    emu.xrr_no_count(12usize, 12usize, 20usize, 2181444u32);
    emu.sw_no_count(29usize, 2usize, 456u32, 2181448u32)?;
    emu.sri_no_count(20usize, 29usize, 10u32, 2181452u32);
    emu.xrr_no_count(15usize, 15usize, 20usize, 2181456u32);
    emu.sri_no_count(20usize, 22usize, 3u32, 2181460u32);
    emu.sw_no_count(22usize, 2usize, 396u32, 2181464u32)?;
    emu.xrr_no_count(10usize, 13usize, 20usize, 2181468u32);
    emu.sw_no_count(10usize, 2usize, 336u32, 2181472u32)?;
    emu.sri_no_count(13usize, 29usize, 3u32, 2181476u32);
    emu.xrr_no_count(10usize, 16usize, 13usize, 2181480u32);
    emu.sw_no_count(10usize, 2usize, 356u32, 2181484u32)?;
    emu.xrr_no_count(13usize, 5usize, 7usize, 2181488u32);
    emu.adr_no_count(17usize, 17usize, 30usize, 2181492u32);
    emu.xrr_no_count(16usize, 31usize, 8usize, 2181496u32);
    emu.lw_no_count(5usize, 2usize, 416u32, 2181500u32)?;
    emu.lw_no_count(10usize, 2usize, 436u32, 2181504u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2181508u32);
    emu.lw_no_count(10usize, 2usize, 476u32, 2181512u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2181516u32);
    emu.adr_no_count(26usize, 5usize, 12usize, 2181520u32);
    emu.lw_no_count(12usize, 2usize, 420u32, 2181524u32)?;
    emu.lw_no_count(29usize, 2usize, 364u32, 2181528u32)?;
    emu.adr_no_count(29usize, 29usize, 12usize, 2181532u32);
    emu.adr_no_count(29usize, 29usize, 24usize, 2181536u32);
    emu.adr_no_count(10usize, 29usize, 15usize, 2181540u32);
    emu.xrr_no_count(12usize, 13usize, 28usize, 2181544u32);
    emu.xrr_no_count(13usize, 16usize, 9usize, 2181548u32);
    emu.lw_no_count(15usize, 2usize, 236u32, 2181552u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2181556u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2181560u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2181564u32);
    emu.sri_no_count(15usize, 26usize, 17u32, 2181568u32);
    emu.sli_no_count(16usize, 26usize, 15u32, 2181572u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2181576u32);
    emu.sri_no_count(16usize, 26usize, 19u32, 2181580u32);
    emu.sli_no_count(17usize, 26usize, 13u32, 2181584u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2181588u32);
    emu.sri_no_count(17usize, 10usize, 17u32, 2181592u32);
    emu.sli_no_count(5usize, 10usize, 15u32, 2181596u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2181600u32);
    emu.sri_no_count(5usize, 10usize, 19u32, 2181604u32);
    emu.sli_no_count(7usize, 10usize, 13u32, 2181608u32);
    emu.orr_no_count(5usize, 7usize, 5usize, 2181612u32);
    emu.sri_no_count(7usize, 10usize, 7u32, 2181616u32);
    emu.sli_no_count(28usize, 10usize, 25u32, 2181620u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2181624u32);
    emu.sri_no_count(28usize, 26usize, 7u32, 2181628u32);
    emu.sli_no_count(29usize, 26usize, 25u32, 2181632u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2181636u32);
    emu.sri_no_count(29usize, 10usize, 18u32, 2181640u32);
    emu.sli_no_count(30usize, 10usize, 14u32, 2181644u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2181648u32);
    emu.sri_no_count(30usize, 26usize, 18u32, 2181652u32);
    emu.sli_no_count(31usize, 26usize, 14u32, 2181656u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2181660u32);
    emu.adr_no_count(25usize, 13usize, 12usize, 2181664u32);
    emu.adr_no_count(1usize, 12usize, 1usize, 2181668u32);
    emu.xrr_no_count(12usize, 15usize, 16usize, 2181672u32);
    emu.xrr_no_count(13usize, 17usize, 5usize, 2181676u32);
    emu.xrr_no_count(15usize, 7usize, 29usize, 2181680u32);
    emu.xrr_no_count(16usize, 28usize, 30usize, 2181684u32);
    emu.sri_no_count(17usize, 26usize, 10u32, 2181688u32);
    emu.xrr_no_count(17usize, 12usize, 17usize, 2181692u32);
    emu.sw_no_count(10usize, 2usize, 436u32, 2181696u32)?;
    emu.sri_no_count(12usize, 10usize, 10u32, 2181700u32);
    emu.xrr_no_count(5usize, 13usize, 12usize, 2181704u32);
    emu.sri_no_count(12usize, 10usize, 3u32, 2181708u32);
    emu.xrr_no_count(12usize, 15usize, 12usize, 2181712u32);
    emu.sw_no_count(12usize, 2usize, 364u32, 2181716u32)?;
    emu.sri_no_count(12usize, 26usize, 3u32, 2181720u32);
    emu.sw_no_count(26usize, 2usize, 392u32, 2181724u32)?;
    emu.xrr_no_count(10usize, 16usize, 12usize, 2181728u32);
    emu.sw_no_count(10usize, 2usize, 344u32, 2181732u32)?;
    emu.sri_no_count(12usize, 1usize, 6u32, 2181736u32);
    emu.sli_no_count(13usize, 1usize, 26u32, 2181740u32);
    emu.orr_no_count(15usize, 13usize, 12usize, 2181744u32);
    emu.sri_no_count(12usize, 1usize, 11u32, 2181748u32);
    emu.sli_no_count(13usize, 1usize, 21u32, 2181752u32);
    emu.orr_no_count(16usize, 13usize, 12usize, 2181756u32);
    emu.sri_no_count(12usize, 1usize, 25u32, 2181760u32);
    emu.sli_no_count(13usize, 1usize, 7u32, 2181764u32);
    emu.orr_no_count(7usize, 13usize, 12usize, 2181768u32);
    emu.lw_no_count(12usize, 2usize, 480u32, 2181772u32)?;
    emu.adr_no_count(12usize, 12usize, 27usize, 2181776u32);
    emu.xrr_no_count(13usize, 6usize, 27usize, 2181780u32);
    emu.anr_no_count(13usize, 1usize, 13usize, 2181784u32);
    emu.xrr_no_count(28usize, 13usize, 27usize, 2181788u32);
    emu.sri_no_count(13usize, 25usize, 2u32, 2181792u32);
    emu.sli_no_count(29usize, 25usize, 30u32, 2181796u32);
    emu.orr_no_count(29usize, 29usize, 13usize, 2181800u32);
    emu.sri_no_count(13usize, 25usize, 13u32, 2181804u32);
    emu.sli_no_count(30usize, 25usize, 19u32, 2181808u32);
    emu.orr_no_count(30usize, 30usize, 13usize, 2181812u32);
    emu.sri_no_count(13usize, 25usize, 22u32, 2181816u32);
    emu.sli_no_count(31usize, 25usize, 10u32, 2181820u32);
    emu.orr_no_count(31usize, 31usize, 13usize, 2181824u32);
    emu.xrr_no_count(13usize, 11usize, 23usize, 2181828u32);
    emu.anr_no_count(13usize, 25usize, 13usize, 2181832u32);
    emu.anr_no_count(8usize, 11usize, 23usize, 2181836u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2181840u32);
    emu.lw_no_count(8usize, 2usize, 408u32, 2181844u32)?;
    emu.lw_no_count(10usize, 2usize, 352u32, 2181848u32)?;
    emu.adr_no_count(8usize, 10usize, 8usize, 2181852u32);
    emu.lw_no_count(10usize, 2usize, 472u32, 2181856u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2181860u32);
    emu.adr_no_count(8usize, 8usize, 17usize, 2181864u32);
    emu.lw_no_count(17usize, 2usize, 492u32, 2181868u32)?;
    emu.lw_no_count(10usize, 2usize, 348u32, 2181872u32)?;
    emu.adr_no_count(17usize, 10usize, 17usize, 2181876u32);
    emu.adr_no_count(17usize, 17usize, 18usize, 2181880u32);
    emu.adr_no_count(10usize, 17usize, 5usize, 2181884u32);
    emu.xrr_no_count(15usize, 15usize, 16usize, 2181888u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2181892u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2181896u32);
    emu.xrr_no_count(16usize, 15usize, 7usize, 2181900u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2181904u32);
    emu.sri_no_count(15usize, 8usize, 17u32, 2181908u32);
    emu.sli_no_count(5usize, 8usize, 15u32, 2181912u32);
    emu.orr_no_count(5usize, 5usize, 15usize, 2181916u32);
    emu.sri_no_count(15usize, 8usize, 19u32, 2181920u32);
    emu.sli_no_count(7usize, 8usize, 13u32, 2181924u32);
    emu.orr_no_count(7usize, 7usize, 15usize, 2181928u32);
    emu.sri_no_count(15usize, 10usize, 17u32, 2181932u32);
    emu.sli_no_count(28usize, 10usize, 15u32, 2181936u32);
    emu.orr_no_count(28usize, 28usize, 15usize, 2181940u32);
    emu.sri_no_count(15usize, 10usize, 19u32, 2181944u32);
    emu.sli_no_count(29usize, 10usize, 13u32, 2181948u32);
    emu.orr_no_count(29usize, 29usize, 15usize, 2181952u32);
    emu.sri_no_count(15usize, 8usize, 7u32, 2181956u32);
    emu.sli_no_count(30usize, 8usize, 25u32, 2181960u32);
    emu.orr_no_count(31usize, 30usize, 15usize, 2181964u32);
    emu.sri_no_count(15usize, 8usize, 18u32, 2181968u32);
    emu.sli_no_count(30usize, 8usize, 14u32, 2181972u32);
    emu.adi_no_count(18usize, 8usize, 0u32, 2181976u32);
    emu.orr_no_count(15usize, 30usize, 15usize, 2181980u32);
    emu.sri_no_count(30usize, 10usize, 7u32, 2181984u32);
    emu.sli_no_count(8usize, 10usize, 25u32, 2181988u32);
    emu.orr_no_count(30usize, 8usize, 30usize, 2181992u32);
    emu.sri_no_count(8usize, 10usize, 18u32, 2181996u32);
    emu.sli_no_count(9usize, 10usize, 14u32, 2182000u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2182004u32);
    emu.lw_no_count(9usize, 2usize, 232u32, 2182008u32)?;
    emu.adr_no_count(14usize, 14usize, 9usize, 2182012u32);
    emu.adr_no_count(16usize, 14usize, 16usize, 2182016u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2182020u32);
    emu.xrr_no_count(5usize, 5usize, 7usize, 2182024u32);
    emu.xrr_no_count(7usize, 28usize, 29usize, 2182028u32);
    emu.xrr_no_count(15usize, 31usize, 15usize, 2182032u32);
    emu.xrr_no_count(28usize, 30usize, 8usize, 2182036u32);
    emu.adr_no_count(14usize, 13usize, 16usize, 2182040u32);
    emu.adr_no_count(17usize, 16usize, 21usize, 2182044u32);
    emu.sw_no_count(18usize, 2usize, 468u32, 2182048u32)?;
    emu.sri_no_count(13usize, 18usize, 10u32, 2182052u32);
    emu.xrr_no_count(13usize, 5usize, 13usize, 2182056u32);
    emu.sw_no_count(10usize, 2usize, 484u32, 2182060u32)?;
    emu.sri_no_count(16usize, 10usize, 10u32, 2182064u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2182068u32);
    emu.sri_no_count(5usize, 18usize, 3u32, 2182072u32);
    emu.xrr_no_count(15usize, 15usize, 5usize, 2182076u32);
    emu.sw_no_count(15usize, 2usize, 328u32, 2182080u32)?;
    emu.sri_no_count(15usize, 10usize, 3u32, 2182084u32);
    emu.xrr_no_count(10usize, 28usize, 15usize, 2182088u32);
    emu.sw_no_count(10usize, 2usize, 352u32, 2182092u32)?;
    emu.lw_no_count(15usize, 2usize, 496u32, 2182096u32)?;
    emu.lw_no_count(10usize, 2usize, 388u32, 2182100u32)?;
    emu.adr_no_count(15usize, 10usize, 15usize, 2182104u32);
    emu.lw_no_count(10usize, 2usize, 460u32, 2182108u32)?;
    emu.adr_no_count(15usize, 15usize, 10usize, 2182112u32);
    emu.adr_no_count(8usize, 15usize, 13usize, 2182116u32);
    emu.lw_no_count(10usize, 2usize, 332u32, 2182120u32)?;
    emu.lw_no_count(21usize, 2usize, 440u32, 2182124u32)?;
    emu.adr_no_count(21usize, 10usize, 21usize, 2182128u32);
    emu.adr_no_count(21usize, 21usize, 22usize, 2182132u32);
    emu.adr_no_count(18usize, 21usize, 16usize, 2182136u32);
    emu.sri_no_count(13usize, 17usize, 6u32, 2182140u32);
    emu.sli_no_count(15usize, 17usize, 26u32, 2182144u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2182148u32);
    emu.sri_no_count(15usize, 17usize, 11u32, 2182152u32);
    emu.sli_no_count(16usize, 17usize, 21u32, 2182156u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2182160u32);
    emu.sri_no_count(16usize, 17usize, 25u32, 2182164u32);
    emu.sli_no_count(5usize, 17usize, 7u32, 2182168u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2182172u32);
    emu.lw_no_count(5usize, 2usize, 448u32, 2182176u32)?;
    emu.adr_no_count(5usize, 5usize, 6usize, 2182180u32);
    emu.xrr_no_count(7usize, 1usize, 6usize, 2182184u32);
    emu.anr_no_count(7usize, 17usize, 7usize, 2182188u32);
    emu.xrr_no_count(6usize, 7usize, 6usize, 2182192u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2182196u32);
    emu.sli_no_count(28usize, 14usize, 30u32, 2182200u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2182204u32);
    emu.sri_no_count(28usize, 14usize, 13u32, 2182208u32);
    emu.sli_no_count(29usize, 14usize, 19u32, 2182212u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2182216u32);
    emu.sri_no_count(29usize, 14usize, 22u32, 2182220u32);
    emu.sli_no_count(30usize, 14usize, 10u32, 2182224u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2182228u32);
    emu.xrr_no_count(30usize, 25usize, 11usize, 2182232u32);
    emu.anr_no_count(30usize, 14usize, 30usize, 2182236u32);
    emu.anr_no_count(31usize, 25usize, 11usize, 2182240u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2182244u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2182248u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2182252u32);
    emu.xrr_no_count(15usize, 7usize, 28usize, 2182256u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2182260u32);
    emu.sri_no_count(6usize, 8usize, 17u32, 2182264u32);
    emu.sli_no_count(7usize, 8usize, 15u32, 2182268u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2182272u32);
    emu.sri_no_count(7usize, 8usize, 19u32, 2182276u32);
    emu.sli_no_count(28usize, 8usize, 13u32, 2182280u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2182284u32);
    emu.sri_no_count(28usize, 18usize, 17u32, 2182288u32);
    emu.sli_no_count(31usize, 18usize, 15u32, 2182292u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2182296u32);
    emu.sri_no_count(31usize, 18usize, 19u32, 2182300u32);
    emu.sli_no_count(8usize, 18usize, 13u32, 2182304u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2182308u32);
    emu.sri_no_count(8usize, 18usize, 7u32, 2182312u32);
    emu.sli_no_count(9usize, 18usize, 25u32, 2182316u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2182320u32);
    emu.sri_no_count(9usize, 10usize, 7u32, 2182324u32);
    emu.sli_no_count(19usize, 10usize, 25u32, 2182328u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2182332u32);
    emu.sri_no_count(19usize, 18usize, 18u32, 2182336u32);
    emu.sli_no_count(20usize, 18usize, 14u32, 2182340u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2182344u32);
    emu.sri_no_count(20usize, 10usize, 18u32, 2182348u32);
    emu.sli_no_count(21usize, 10usize, 14u32, 2182352u32);
    emu.orr_no_count(20usize, 21usize, 20usize, 2182356u32);
    emu.xrr_no_count(13usize, 13usize, 16usize, 2182360u32);
    emu.xrr_no_count(15usize, 15usize, 29usize, 2182364u32);
    emu.xrr_no_count(16usize, 6usize, 7usize, 2182368u32);
    emu.xrr_no_count(6usize, 28usize, 31usize, 2182372u32);
    emu.xrr_no_count(7usize, 8usize, 19usize, 2182376u32);
    emu.xrr_no_count(28usize, 9usize, 20usize, 2182380u32);
    emu.lw_no_count(29usize, 2usize, 228u32, 2182384u32)?;
    emu.adr_no_count(12usize, 12usize, 29usize, 2182388u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2182392u32);
    emu.adr_no_count(12usize, 15usize, 30usize, 2182396u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2182400u32)?;
    emu.sri_no_count(15usize, 10usize, 10u32, 2182404u32);
    emu.xrr_no_count(22usize, 16usize, 15usize, 2182408u32);
    emu.sw_no_count(18usize, 2usize, 480u32, 2182412u32)?;
    emu.sri_no_count(15usize, 18usize, 10u32, 2182416u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2182420u32);
    emu.sri_no_count(16usize, 18usize, 3u32, 2182424u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2182428u32);
    emu.sw_no_count(16usize, 2usize, 348u32, 2182432u32)?;
    emu.sri_no_count(16usize, 10usize, 3u32, 2182436u32);
    emu.xrr_no_count(10usize, 28usize, 16usize, 2182440u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2182444u32)?;
    emu.adr_no_count(18usize, 12usize, 13usize, 2182448u32);
    emu.adr_no_count(10usize, 13usize, 23usize, 2182452u32);
    emu.lw_no_count(13usize, 2usize, 504u32, 2182456u32)?;
    emu.lw_no_count(12usize, 2usize, 376u32, 2182460u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2182464u32);
    emu.lw_no_count(12usize, 2usize, 456u32, 2182468u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2182472u32);
    emu.adr_no_count(22usize, 13usize, 22usize, 2182476u32);
    emu.lw_no_count(12usize, 2usize, 324u32, 2182480u32)?;
    emu.lw_no_count(24usize, 2usize, 444u32, 2182484u32)?;
    emu.adr_no_count(24usize, 12usize, 24usize, 2182488u32);
    emu.adr_no_count(24usize, 24usize, 26usize, 2182492u32);
    emu.adr_no_count(12usize, 24usize, 15usize, 2182496u32);
    emu.sri_no_count(13usize, 10usize, 6u32, 2182500u32);
    emu.sli_no_count(15usize, 10usize, 26u32, 2182504u32);
    emu.orr_no_count(16usize, 15usize, 13usize, 2182508u32);
    emu.sri_no_count(13usize, 10usize, 11u32, 2182512u32);
    emu.sli_no_count(8usize, 10usize, 21u32, 2182516u32);
    emu.orr_no_count(8usize, 8usize, 13usize, 2182520u32);
    emu.sri_no_count(13usize, 10usize, 25u32, 2182524u32);
    emu.sli_no_count(15usize, 10usize, 7u32, 2182528u32);
    emu.orr_no_count(6usize, 15usize, 13usize, 2182532u32);
    emu.lw_no_count(15usize, 2usize, 452u32, 2182536u32)?;
    emu.adr_no_count(15usize, 15usize, 1usize, 2182540u32);
    emu.xrr_no_count(13usize, 17usize, 1usize, 2182544u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2182548u32);
    emu.xrr_no_count(28usize, 13usize, 1usize, 2182552u32);
    emu.sri_no_count(13usize, 18usize, 2u32, 2182556u32);
    emu.sli_no_count(7usize, 18usize, 30u32, 2182560u32);
    emu.orr_no_count(21usize, 7usize, 13usize, 2182564u32);
    emu.sri_no_count(13usize, 18usize, 13u32, 2182568u32);
    emu.sli_no_count(7usize, 18usize, 19u32, 2182572u32);
    emu.orr_no_count(23usize, 7usize, 13usize, 2182576u32);
    emu.sri_no_count(13usize, 18usize, 22u32, 2182580u32);
    emu.sli_no_count(7usize, 18usize, 10u32, 2182584u32);
    emu.orr_no_count(30usize, 7usize, 13usize, 2182588u32);
    emu.xrr_no_count(13usize, 14usize, 25usize, 2182592u32);
    emu.anr_no_count(13usize, 18usize, 13usize, 2182596u32);
    emu.anr_no_count(7usize, 14usize, 25usize, 2182600u32);
    emu.xrr_no_count(7usize, 13usize, 7usize, 2182604u32);
    emu.sri_no_count(13usize, 22usize, 17u32, 2182608u32);
    emu.sli_no_count(29usize, 22usize, 15u32, 2182612u32);
    emu.orr_no_count(24usize, 29usize, 13usize, 2182616u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2182620u32);
    emu.sli_no_count(29usize, 22usize, 13u32, 2182624u32);
    emu.orr_no_count(26usize, 29usize, 13usize, 2182628u32);
    emu.sri_no_count(13usize, 12usize, 17u32, 2182632u32);
    emu.sli_no_count(29usize, 12usize, 15u32, 2182636u32);
    emu.orr_no_count(27usize, 29usize, 13usize, 2182640u32);
    emu.sri_no_count(13usize, 12usize, 19u32, 2182644u32);
    emu.sli_no_count(29usize, 12usize, 13u32, 2182648u32);
    emu.orr_no_count(1usize, 29usize, 13usize, 2182652u32);
    emu.sri_no_count(13usize, 22usize, 7u32, 2182656u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2182660u32);
    emu.orr_no_count(13usize, 29usize, 13usize, 2182664u32);
    emu.sri_no_count(29usize, 22usize, 18u32, 2182668u32);
    emu.sli_no_count(31usize, 22usize, 14u32, 2182672u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2182676u32);
    emu.sri_no_count(31usize, 12usize, 7u32, 2182680u32);
    emu.sli_no_count(9usize, 12usize, 25u32, 2182684u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2182688u32);
    emu.sri_no_count(9usize, 12usize, 18u32, 2182692u32);
    emu.sli_no_count(19usize, 12usize, 14u32, 2182696u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2182700u32);
    emu.xrr_no_count(16usize, 16usize, 8usize, 2182704u32);
    emu.adr_no_count(5usize, 5usize, 28usize, 2182708u32);
    emu.xrr_no_count(28usize, 21usize, 23usize, 2182712u32);
    emu.xrr_no_count(8usize, 24usize, 26usize, 2182716u32);
    emu.xrr_no_count(19usize, 27usize, 1usize, 2182720u32);
    emu.xrr_no_count(13usize, 13usize, 29usize, 2182724u32);
    emu.xrr_no_count(29usize, 31usize, 9usize, 2182728u32);
    emu.xrr_no_count(16usize, 16usize, 6usize, 2182732u32);
    emu.xrr_no_count(6usize, 28usize, 30usize, 2182736u32);
    emu.sri_no_count(28usize, 22usize, 10u32, 2182740u32);
    emu.xrr_no_count(31usize, 8usize, 28usize, 2182744u32);
    emu.sw_no_count(12usize, 2usize, 448u32, 2182748u32)?;
    emu.sri_no_count(28usize, 12usize, 10u32, 2182752u32);
    emu.xrr_no_count(20usize, 19usize, 28usize, 2182756u32);
    emu.sri_no_count(28usize, 22usize, 3u32, 2182760u32);
    emu.sw_no_count(22usize, 2usize, 388u32, 2182764u32)?;
    emu.xrr_no_count(13usize, 13usize, 28usize, 2182768u32);
    emu.sw_no_count(13usize, 2usize, 312u32, 2182772u32)?;
    emu.sri_no_count(13usize, 12usize, 3u32, 2182776u32);
    emu.xrr_no_count(12usize, 29usize, 13usize, 2182780u32);
    emu.sw_no_count(12usize, 2usize, 324u32, 2182784u32)?;
    emu.lw_no_count(12usize, 2usize, 224u32, 2182788u32)?;
    emu.adr_no_count(5usize, 5usize, 12usize, 2182792u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2182796u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2182800u32);
    emu.lw_no_count(13usize, 2usize, 500u32, 2182804u32)?;
    emu.lw_no_count(12usize, 2usize, 384u32, 2182808u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2182812u32);
    emu.lw_no_count(12usize, 2usize, 436u32, 2182816u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2182820u32);
    emu.adr_no_count(31usize, 13usize, 31usize, 2182824u32);
    emu.lw_no_count(13usize, 2usize, 476u32, 2182828u32)?;
    emu.lw_no_count(12usize, 2usize, 340u32, 2182832u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2182836u32);
    emu.lw_no_count(12usize, 2usize, 468u32, 2182840u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2182844u32);
    emu.adr_no_count(20usize, 13usize, 20usize, 2182848u32);
    emu.adr_no_count(26usize, 6usize, 16usize, 2182852u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2182856u32);
    emu.sri_no_count(13usize, 31usize, 17u32, 2182860u32);
    emu.sli_no_count(16usize, 31usize, 15u32, 2182864u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2182868u32);
    emu.sri_no_count(16usize, 31usize, 19u32, 2182872u32);
    emu.sli_no_count(5usize, 31usize, 13u32, 2182876u32);
    emu.orr_no_count(5usize, 5usize, 16usize, 2182880u32);
    emu.sri_no_count(16usize, 20usize, 17u32, 2182884u32);
    emu.sli_no_count(6usize, 20usize, 15u32, 2182888u32);
    emu.orr_no_count(6usize, 6usize, 16usize, 2182892u32);
    emu.sri_no_count(16usize, 20usize, 19u32, 2182896u32);
    emu.sli_no_count(7usize, 20usize, 13u32, 2182900u32);
    emu.orr_no_count(7usize, 7usize, 16usize, 2182904u32);
    emu.sri_no_count(16usize, 20usize, 7u32, 2182908u32);
    emu.sli_no_count(28usize, 20usize, 25u32, 2182912u32);
    emu.orr_no_count(28usize, 28usize, 16usize, 2182916u32);
    emu.sri_no_count(16usize, 31usize, 7u32, 2182920u32);
    emu.sli_no_count(29usize, 31usize, 25u32, 2182924u32);
    emu.orr_no_count(29usize, 29usize, 16usize, 2182928u32);
    emu.sri_no_count(16usize, 20usize, 18u32, 2182932u32);
    emu.sli_no_count(30usize, 20usize, 14u32, 2182936u32);
    emu.orr_no_count(30usize, 30usize, 16usize, 2182940u32);
    emu.sri_no_count(16usize, 31usize, 18u32, 2182944u32);
    emu.sli_no_count(8usize, 31usize, 14u32, 2182948u32);
    emu.orr_no_count(8usize, 8usize, 16usize, 2182952u32);
    emu.sri_no_count(16usize, 11usize, 6u32, 2182956u32);
    emu.sli_no_count(9usize, 11usize, 26u32, 2182960u32);
    emu.orr_no_count(9usize, 9usize, 16usize, 2182964u32);
    emu.sri_no_count(16usize, 11usize, 11u32, 2182968u32);
    emu.sli_no_count(19usize, 11usize, 21u32, 2182972u32);
    emu.orr_no_count(19usize, 19usize, 16usize, 2182976u32);
    emu.sri_no_count(16usize, 11usize, 25u32, 2182980u32);
    emu.sli_no_count(21usize, 11usize, 7u32, 2182984u32);
    emu.orr_no_count(21usize, 21usize, 16usize, 2182988u32);
    emu.lw_no_count(16usize, 2usize, 488u32, 2182992u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2182996u32);
    emu.xrr_no_count(23usize, 10usize, 17usize, 2183000u32);
    emu.anr_no_count(23usize, 11usize, 23usize, 2183004u32);
    emu.xrr_no_count(17usize, 23usize, 17usize, 2183008u32);
    emu.sri_no_count(23usize, 26usize, 2u32, 2183012u32);
    emu.sli_no_count(24usize, 26usize, 30u32, 2183016u32);
    emu.orr_no_count(23usize, 24usize, 23usize, 2183020u32);
    emu.sri_no_count(24usize, 26usize, 13u32, 2183024u32);
    emu.sli_no_count(27usize, 26usize, 19u32, 2183028u32);
    emu.orr_no_count(24usize, 27usize, 24usize, 2183032u32);
    emu.sri_no_count(27usize, 26usize, 22u32, 2183036u32);
    emu.sli_no_count(1usize, 26usize, 10u32, 2183040u32);
    emu.orr_no_count(27usize, 1usize, 27usize, 2183044u32);
    emu.xrr_no_count(1usize, 18usize, 14usize, 2183048u32);
    emu.anr_no_count(1usize, 26usize, 1usize, 2183052u32);
    emu.anr_no_count(12usize, 18usize, 14usize, 2183056u32);
    emu.xrr_no_count(12usize, 1usize, 12usize, 2183060u32);
    emu.xrr_no_count(13usize, 13usize, 5usize, 2183064u32);
    emu.xrr_no_count(5usize, 6usize, 7usize, 2183068u32);
    emu.xrr_no_count(6usize, 28usize, 30usize, 2183072u32);
    emu.xrr_no_count(7usize, 29usize, 8usize, 2183076u32);
    emu.xrr_no_count(28usize, 9usize, 19usize, 2183080u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2183084u32);
    emu.xrr_no_count(17usize, 23usize, 24usize, 2183088u32);
    emu.sri_no_count(29usize, 31usize, 10u32, 2183092u32);
    emu.xrr_no_count(13usize, 13usize, 29usize, 2183096u32);
    emu.sri_no_count(29usize, 20usize, 10u32, 2183100u32);
    emu.xrr_no_count(19usize, 5usize, 29usize, 2183104u32);
    emu.sri_no_count(5usize, 20usize, 3u32, 2183108u32);
    emu.xrr_no_count(5usize, 6usize, 5usize, 2183112u32);
    emu.sw_no_count(5usize, 2usize, 316u32, 2183116u32)?;
    emu.sri_no_count(5usize, 31usize, 3u32, 2183120u32);
    emu.sw_no_count(31usize, 2usize, 452u32, 2183124u32)?;
    emu.xrr_no_count(5usize, 7usize, 5usize, 2183128u32);
    emu.sw_no_count(5usize, 2usize, 308u32, 2183132u32)?;
    emu.xrr_no_count(5usize, 28usize, 21usize, 2183136u32);
    emu.xrr_no_count(17usize, 17usize, 27usize, 2183140u32);
    emu.lw_no_count(6usize, 2usize, 404u32, 2183144u32)?;
    emu.lw_no_count(7usize, 2usize, 360u32, 2183148u32)?;
    emu.adr_no_count(6usize, 7usize, 6usize, 2183152u32);
    emu.lw_no_count(7usize, 2usize, 484u32, 2183156u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2183160u32);
    emu.adr_no_count(24usize, 6usize, 13usize, 2183164u32);
    emu.lw_no_count(13usize, 2usize, 472u32, 2183168u32)?;
    emu.lw_no_count(6usize, 2usize, 320u32, 2183172u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2183176u32);
    emu.lw_no_count(6usize, 2usize, 464u32, 2183180u32)?;
    emu.adr_no_count(13usize, 13usize, 6usize, 2183184u32);
    emu.adr_no_count(19usize, 13usize, 19usize, 2183188u32);
    emu.lw_no_count(13usize, 2usize, 220u32, 2183192u32)?;
    emu.adr_no_count(15usize, 15usize, 13usize, 2183196u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2183200u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2183204u32);
    emu.adr_no_count(1usize, 12usize, 15usize, 2183208u32);
    emu.adr_no_count(27usize, 15usize, 25usize, 2183212u32);
    emu.sri_no_count(12usize, 24usize, 17u32, 2183216u32);
    emu.sli_no_count(13usize, 24usize, 15u32, 2183220u32);
    emu.orr_no_count(13usize, 13usize, 12usize, 2183224u32);
    emu.sri_no_count(12usize, 24usize, 19u32, 2183228u32);
    emu.sli_no_count(15usize, 24usize, 13u32, 2183232u32);
    emu.orr_no_count(15usize, 15usize, 12usize, 2183236u32);
    emu.sri_no_count(12usize, 19usize, 17u32, 2183240u32);
    emu.sli_no_count(17usize, 19usize, 15u32, 2183244u32);
    emu.orr_no_count(17usize, 17usize, 12usize, 2183248u32);
    emu.sri_no_count(12usize, 19usize, 19u32, 2183252u32);
    emu.sli_no_count(5usize, 19usize, 13u32, 2183256u32);
    emu.orr_no_count(5usize, 5usize, 12usize, 2183260u32);
    emu.sri_no_count(12usize, 24usize, 7u32, 2183264u32);
    emu.sli_no_count(6usize, 24usize, 25u32, 2183268u32);
    emu.orr_no_count(6usize, 6usize, 12usize, 2183272u32);
    emu.sri_no_count(12usize, 24usize, 18u32, 2183276u32);
    emu.sli_no_count(7usize, 24usize, 14u32, 2183280u32);
    emu.orr_no_count(7usize, 7usize, 12usize, 2183284u32);
    emu.sri_no_count(12usize, 19usize, 7u32, 2183288u32);
    emu.sli_no_count(29usize, 19usize, 25u32, 2183292u32);
    emu.orr_no_count(12usize, 29usize, 12usize, 2183296u32);
    emu.sri_no_count(29usize, 19usize, 18u32, 2183300u32);
    emu.sli_no_count(30usize, 19usize, 14u32, 2183304u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2183308u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2183312u32);
    emu.xrr_no_count(15usize, 17usize, 5usize, 2183316u32);
    emu.xrr_no_count(17usize, 6usize, 7usize, 2183320u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2183324u32);
    emu.sri_no_count(5usize, 27usize, 6u32, 2183328u32);
    emu.sli_no_count(6usize, 27usize, 26u32, 2183332u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2183336u32);
    emu.sri_no_count(6usize, 27usize, 11u32, 2183340u32);
    emu.sli_no_count(7usize, 27usize, 21u32, 2183344u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2183348u32);
    emu.sri_no_count(7usize, 27usize, 25u32, 2183352u32);
    emu.sli_no_count(29usize, 27usize, 7u32, 2183356u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2183360u32);
    emu.lw_no_count(8usize, 2usize, 412u32, 2183364u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2183368u32);
    emu.xrr_no_count(29usize, 11usize, 10usize, 2183372u32);
    emu.anr_no_count(29usize, 27usize, 29usize, 2183376u32);
    emu.xrr_no_count(10usize, 29usize, 10usize, 2183380u32);
    emu.sri_no_count(29usize, 1usize, 2u32, 2183384u32);
    emu.sli_no_count(30usize, 1usize, 30u32, 2183388u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2183392u32);
    emu.sri_no_count(30usize, 1usize, 13u32, 2183396u32);
    emu.sli_no_count(9usize, 1usize, 19u32, 2183400u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2183404u32);
    emu.sri_no_count(9usize, 1usize, 22u32, 2183408u32);
    emu.sli_no_count(21usize, 1usize, 10u32, 2183412u32);
    emu.orr_no_count(9usize, 21usize, 9usize, 2183416u32);
    emu.xrr_no_count(21usize, 26usize, 18usize, 2183420u32);
    emu.anr_no_count(21usize, 1usize, 21usize, 2183424u32);
    emu.anr_no_count(23usize, 26usize, 18usize, 2183428u32);
    emu.xrr_no_count(21usize, 21usize, 23usize, 2183432u32);
    emu.sri_no_count(23usize, 24usize, 10u32, 2183436u32);
    emu.xrr_no_count(13usize, 13usize, 23usize, 2183440u32);
    emu.sw_no_count(19usize, 2usize, 488u32, 2183444u32)?;
    emu.sri_no_count(23usize, 19usize, 10u32, 2183448u32);
    emu.xrr_no_count(15usize, 15usize, 23usize, 2183452u32);
    emu.sri_no_count(23usize, 24usize, 3u32, 2183456u32);
    emu.sw_no_count(24usize, 2usize, 384u32, 2183460u32)?;
    emu.xrr_no_count(17usize, 17usize, 23usize, 2183464u32);
    emu.sw_no_count(17usize, 2usize, 304u32, 2183468u32)?;
    emu.sri_no_count(17usize, 19usize, 3u32, 2183472u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2183476u32);
    emu.sw_no_count(12usize, 2usize, 360u32, 2183480u32)?;
    emu.xrr_no_count(12usize, 5usize, 6usize, 2183484u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2183488u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2183492u32);
    emu.lw_no_count(16usize, 2usize, 400u32, 2183496u32)?;
    emu.lw_no_count(5usize, 2usize, 380u32, 2183500u32)?;
    emu.adr_no_count(16usize, 5usize, 16usize, 2183504u32);
    emu.lw_no_count(5usize, 2usize, 480u32, 2183508u32)?;
    emu.adr_no_count(16usize, 16usize, 5usize, 2183512u32);
    emu.adr_no_count(5usize, 16usize, 13usize, 2183516u32);
    emu.lw_no_count(13usize, 2usize, 460u32, 2183520u32)?;
    emu.lw_no_count(16usize, 2usize, 336u32, 2183524u32)?;
    emu.adr_no_count(13usize, 16usize, 13usize, 2183528u32);
    emu.adr_no_count(13usize, 13usize, 22usize, 2183532u32);
    emu.adr_no_count(16usize, 13usize, 15usize, 2183536u32);
    emu.xrr_no_count(12usize, 12usize, 7usize, 2183540u32);
    emu.xrr_no_count(13usize, 17usize, 9usize, 2183544u32);
    emu.lw_no_count(15usize, 2usize, 216u32, 2183548u32)?;
    emu.adr_no_count(10usize, 10usize, 15usize, 2183552u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2183556u32);
    emu.adr_no_count(10usize, 13usize, 21usize, 2183560u32);
    emu.sri_no_count(13usize, 5usize, 17u32, 2183564u32);
    emu.sli_no_count(15usize, 5usize, 15u32, 2183568u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2183572u32);
    emu.sri_no_count(15usize, 5usize, 19u32, 2183576u32);
    emu.sli_no_count(17usize, 5usize, 13u32, 2183580u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2183584u32);
    emu.sri_no_count(17usize, 16usize, 17u32, 2183588u32);
    emu.sli_no_count(6usize, 16usize, 15u32, 2183592u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2183596u32);
    emu.sri_no_count(6usize, 16usize, 19u32, 2183600u32);
    emu.sli_no_count(7usize, 16usize, 13u32, 2183604u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2183608u32);
    emu.sri_no_count(7usize, 16usize, 7u32, 2183612u32);
    emu.sli_no_count(29usize, 16usize, 25u32, 2183616u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2183620u32);
    emu.sri_no_count(29usize, 5usize, 7u32, 2183624u32);
    emu.sli_no_count(30usize, 5usize, 25u32, 2183628u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2183632u32);
    emu.sri_no_count(30usize, 16usize, 18u32, 2183636u32);
    emu.sli_no_count(9usize, 16usize, 14u32, 2183640u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2183644u32);
    emu.sri_no_count(9usize, 5usize, 18u32, 2183648u32);
    emu.sli_no_count(21usize, 5usize, 14u32, 2183652u32);
    emu.orr_no_count(9usize, 21usize, 9usize, 2183656u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2183660u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2183664u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2183668u32);
    emu.xrr_no_count(12usize, 17usize, 6usize, 2183672u32);
    emu.xrr_no_count(15usize, 7usize, 30usize, 2183676u32);
    emu.xrr_no_count(17usize, 29usize, 9usize, 2183680u32);
    emu.sri_no_count(6usize, 5usize, 10u32, 2183684u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2183688u32);
    emu.sw_no_count(16usize, 2usize, 380u32, 2183692u32)?;
    emu.sri_no_count(6usize, 16usize, 10u32, 2183696u32);
    emu.xrr_no_count(29usize, 12usize, 6usize, 2183700u32);
    emu.sri_no_count(12usize, 16usize, 3u32, 2183704u32);
    emu.xrr_no_count(12usize, 15usize, 12usize, 2183708u32);
    emu.sw_no_count(12usize, 2usize, 320u32, 2183712u32)?;
    emu.sri_no_count(12usize, 5usize, 3u32, 2183716u32);
    emu.sw_no_count(5usize, 2usize, 412u32, 2183720u32)?;
    emu.xrr_no_count(12usize, 17usize, 12usize, 2183724u32);
    emu.sw_no_count(12usize, 2usize, 300u32, 2183728u32)?;
    emu.sri_no_count(12usize, 14usize, 6u32, 2183732u32);
    emu.sli_no_count(15usize, 14usize, 26u32, 2183736u32);
    emu.orr_no_count(12usize, 15usize, 12usize, 2183740u32);
    emu.sri_no_count(15usize, 14usize, 11u32, 2183744u32);
    emu.sli_no_count(17usize, 14usize, 21u32, 2183748u32);
    emu.orr_no_count(17usize, 17usize, 15usize, 2183752u32);
    emu.sri_no_count(15usize, 14usize, 25u32, 2183756u32);
    emu.sli_no_count(6usize, 14usize, 7u32, 2183760u32);
    emu.orr_no_count(6usize, 6usize, 15usize, 2183764u32);
    emu.lw_no_count(15usize, 2usize, 416u32, 2183768u32)?;
    emu.adr_no_count(19usize, 15usize, 11usize, 2183772u32);
    emu.xrr_no_count(15usize, 27usize, 11usize, 2183776u32);
    emu.anr_no_count(15usize, 14usize, 15usize, 2183780u32);
    emu.xrr_no_count(11usize, 15usize, 11usize, 2183784u32);
    emu.sri_no_count(15usize, 10usize, 2u32, 2183788u32);
    emu.sli_no_count(7usize, 10usize, 30u32, 2183792u32);
    emu.orr_no_count(30usize, 7usize, 15usize, 2183796u32);
    emu.sri_no_count(15usize, 10usize, 13u32, 2183800u32);
    emu.sli_no_count(7usize, 10usize, 19u32, 2183804u32);
    emu.orr_no_count(9usize, 7usize, 15usize, 2183808u32);
    emu.sri_no_count(15usize, 10usize, 22u32, 2183812u32);
    emu.sli_no_count(7usize, 10usize, 10u32, 2183816u32);
    emu.orr_no_count(21usize, 7usize, 15usize, 2183820u32);
    emu.xrr_no_count(15usize, 1usize, 26usize, 2183824u32);
    emu.anr_no_count(15usize, 10usize, 15usize, 2183828u32);
    emu.anr_no_count(7usize, 1usize, 26usize, 2183832u32);
    emu.xrr_no_count(7usize, 15usize, 7usize, 2183836u32);
    emu.lw_no_count(15usize, 2usize, 396u32, 2183840u32)?;
    emu.lw_no_count(16usize, 2usize, 356u32, 2183844u32)?;
    emu.adr_no_count(15usize, 16usize, 15usize, 2183848u32);
    emu.lw_no_count(16usize, 2usize, 448u32, 2183852u32)?;
    emu.adr_no_count(15usize, 15usize, 16usize, 2183856u32);
    emu.adr_no_count(15usize, 15usize, 13usize, 2183860u32);
    emu.lw_no_count(13usize, 2usize, 456u32, 2183864u32)?;
    emu.lw_no_count(16usize, 2usize, 344u32, 2183868u32)?;
    emu.adr_no_count(13usize, 16usize, 13usize, 2183872u32);
    emu.adr_no_count(13usize, 13usize, 31usize, 2183876u32);
    emu.adr_no_count(16usize, 13usize, 29usize, 2183880u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2183884u32);
    emu.adr_no_count(8usize, 8usize, 11usize, 2183888u32);
    emu.xrr_no_count(11usize, 30usize, 9usize, 2183892u32);
    emu.xrr_no_count(12usize, 12usize, 6usize, 2183896u32);
    emu.xrr_no_count(21usize, 11usize, 21usize, 2183900u32);
    emu.sri_no_count(11usize, 15usize, 17u32, 2183904u32);
    emu.sli_no_count(13usize, 15usize, 15u32, 2183908u32);
    emu.orr_no_count(23usize, 13usize, 11usize, 2183912u32);
    emu.sri_no_count(11usize, 15usize, 19u32, 2183916u32);
    emu.sli_no_count(13usize, 15usize, 13u32, 2183920u32);
    emu.orr_no_count(31usize, 13usize, 11usize, 2183924u32);
    emu.sri_no_count(11usize, 16usize, 17u32, 2183928u32);
    emu.sli_no_count(13usize, 16usize, 15u32, 2183932u32);
    emu.orr_no_count(25usize, 13usize, 11usize, 2183936u32);
    emu.sri_no_count(11usize, 16usize, 19u32, 2183940u32);
    emu.sli_no_count(13usize, 16usize, 13u32, 2183944u32);
    emu.orr_no_count(6usize, 13usize, 11usize, 2183948u32);
    emu.sri_no_count(11usize, 15usize, 7u32, 2183952u32);
    emu.sli_no_count(13usize, 15usize, 25u32, 2183956u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2183960u32);
    emu.sri_no_count(11usize, 15usize, 18u32, 2183964u32);
    emu.sli_no_count(17usize, 15usize, 14u32, 2183968u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2183972u32);
    emu.sri_no_count(17usize, 16usize, 7u32, 2183976u32);
    emu.sli_no_count(30usize, 16usize, 25u32, 2183980u32);
    emu.orr_no_count(17usize, 30usize, 17usize, 2183984u32);
    emu.sri_no_count(30usize, 16usize, 18u32, 2183988u32);
    emu.sli_no_count(9usize, 16usize, 14u32, 2183992u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2183996u32);
    emu.lw_no_count(28usize, 2usize, 212u32, 2184000u32)?;
    emu.adr_no_count(8usize, 8usize, 28usize, 2184004u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2184008u32);
    emu.adr_no_count(7usize, 21usize, 7usize, 2184012u32);
    emu.xrr_no_count(8usize, 23usize, 31usize, 2184016u32);
    emu.xrr_no_count(9usize, 25usize, 6usize, 2184020u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2184024u32);
    emu.xrr_no_count(13usize, 17usize, 30usize, 2184028u32);
    emu.adr_no_count(21usize, 7usize, 12usize, 2184032u32);
    emu.adr_no_count(6usize, 12usize, 18usize, 2184036u32);
    emu.sri_no_count(12usize, 15usize, 10u32, 2184040u32);
    emu.xrr_no_count(12usize, 8usize, 12usize, 2184044u32);
    emu.sri_no_count(17usize, 16usize, 10u32, 2184048u32);
    emu.xrr_no_count(17usize, 9usize, 17usize, 2184052u32);
    emu.sri_no_count(7usize, 15usize, 3u32, 2184056u32);
    emu.sw_no_count(15usize, 2usize, 276u32, 2184060u32)?;
    emu.xrr_no_count(11usize, 11usize, 7usize, 2184064u32);
    emu.sw_no_count(11usize, 2usize, 288u32, 2184068u32)?;
    emu.sri_no_count(11usize, 16usize, 3u32, 2184072u32);
    emu.sw_no_count(16usize, 2usize, 272u32, 2184076u32)?;
    emu.xrr_no_count(11usize, 13usize, 11usize, 2184080u32);
    emu.sw_no_count(11usize, 2usize, 292u32, 2184084u32)?;
    emu.lw_no_count(11usize, 2usize, 392u32, 2184088u32)?;
    emu.lw_no_count(13usize, 2usize, 364u32, 2184092u32)?;
    emu.adr_no_count(11usize, 13usize, 11usize, 2184096u32);
    emu.sw_no_count(20usize, 2usize, 376u32, 2184100u32)?;
    emu.adr_no_count(11usize, 11usize, 20usize, 2184104u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2184108u32);
    emu.lw_no_count(11usize, 2usize, 328u32, 2184112u32)?;
    emu.lw_no_count(12usize, 2usize, 436u32, 2184116u32)?;
    emu.adr_no_count(11usize, 11usize, 12usize, 2184120u32);
    emu.adr_no_count(11usize, 11usize, 24usize, 2184124u32);
    emu.adr_no_count(22usize, 11usize, 17usize, 2184128u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2184132u32);
    emu.sli_no_count(17usize, 6usize, 26u32, 2184136u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2184140u32);
    emu.sri_no_count(11usize, 6usize, 11u32, 2184144u32);
    emu.sli_no_count(7usize, 6usize, 21u32, 2184148u32);
    emu.orr_no_count(7usize, 7usize, 11usize, 2184152u32);
    emu.sri_no_count(11usize, 6usize, 25u32, 2184156u32);
    emu.sli_no_count(30usize, 6usize, 7u32, 2184160u32);
    emu.orr_no_count(30usize, 30usize, 11usize, 2184164u32);
    emu.lw_no_count(11usize, 2usize, 420u32, 2184168u32)?;
    emu.adr_no_count(11usize, 11usize, 27usize, 2184172u32);
    emu.xrr_no_count(8usize, 14usize, 27usize, 2184176u32);
    emu.anr_no_count(8usize, 6usize, 8usize, 2184180u32);
    emu.xrr_no_count(8usize, 8usize, 27usize, 2184184u32);
    emu.sri_no_count(9usize, 21usize, 2u32, 2184188u32);
    emu.sli_no_count(18usize, 21usize, 30u32, 2184192u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2184196u32);
    emu.sri_no_count(18usize, 21usize, 13u32, 2184200u32);
    emu.sli_no_count(23usize, 21usize, 19u32, 2184204u32);
    emu.orr_no_count(18usize, 23usize, 18usize, 2184208u32);
    emu.sri_no_count(23usize, 21usize, 22u32, 2184212u32);
    emu.sli_no_count(24usize, 21usize, 10u32, 2184216u32);
    emu.orr_no_count(23usize, 24usize, 23usize, 2184220u32);
    emu.xrr_no_count(24usize, 10usize, 1usize, 2184224u32);
    emu.anr_no_count(24usize, 21usize, 24usize, 2184228u32);
    emu.anr_no_count(25usize, 10usize, 1usize, 2184232u32);
    emu.xrr_no_count(24usize, 24usize, 25usize, 2184236u32);
    emu.xrr_no_count(17usize, 17usize, 7usize, 2184240u32);
    emu.adr_no_count(8usize, 19usize, 8usize, 2184244u32);
    emu.xrr_no_count(7usize, 9usize, 18usize, 2184248u32);
    emu.sri_no_count(9usize, 20usize, 17u32, 2184252u32);
    emu.sli_no_count(18usize, 20usize, 15u32, 2184256u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2184260u32);
    emu.sri_no_count(18usize, 20usize, 19u32, 2184264u32);
    emu.sli_no_count(25usize, 20usize, 13u32, 2184268u32);
    emu.orr_no_count(18usize, 25usize, 18usize, 2184272u32);
    emu.sri_no_count(25usize, 22usize, 17u32, 2184276u32);
    emu.sli_no_count(27usize, 22usize, 15u32, 2184280u32);
    emu.orr_no_count(25usize, 27usize, 25usize, 2184284u32);
    emu.sri_no_count(27usize, 22usize, 19u32, 2184288u32);
    emu.sli_no_count(13usize, 22usize, 13u32, 2184292u32);
    emu.orr_no_count(13usize, 13usize, 27usize, 2184296u32);
    emu.sri_no_count(27usize, 22usize, 7u32, 2184300u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2184304u32);
    emu.orr_no_count(29usize, 29usize, 27usize, 2184308u32);
    emu.sri_no_count(27usize, 20usize, 7u32, 2184312u32);
    emu.sli_no_count(12usize, 20usize, 25u32, 2184316u32);
    emu.orr_no_count(12usize, 12usize, 27usize, 2184320u32);
    emu.sri_no_count(27usize, 22usize, 18u32, 2184324u32);
    emu.sli_no_count(19usize, 22usize, 14u32, 2184328u32);
    emu.orr_no_count(19usize, 19usize, 27usize, 2184332u32);
    emu.sri_no_count(27usize, 20usize, 18u32, 2184336u32);
    emu.sli_no_count(31usize, 20usize, 14u32, 2184340u32);
    emu.orr_no_count(31usize, 31usize, 27usize, 2184344u32);
    emu.xrr_no_count(17usize, 17usize, 30usize, 2184348u32);
    emu.xrr_no_count(7usize, 7usize, 23usize, 2184352u32);
    emu.xrr_no_count(30usize, 9usize, 18usize, 2184356u32);
    emu.xrr_no_count(13usize, 25usize, 13usize, 2184360u32);
    emu.xrr_no_count(29usize, 29usize, 19usize, 2184364u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2184368u32);
    emu.lw_no_count(28usize, 2usize, 208u32, 2184372u32)?;
    emu.adr_no_count(8usize, 8usize, 28usize, 2184376u32);
    emu.adr_no_count(17usize, 8usize, 17usize, 2184380u32);
    emu.adr_no_count(7usize, 7usize, 24usize, 2184384u32);
    emu.sri_no_count(31usize, 20usize, 10u32, 2184388u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2184392u32);
    emu.sw_no_count(22usize, 2usize, 416u32, 2184396u32)?;
    emu.sri_no_count(31usize, 22usize, 10u32, 2184400u32);
    emu.xrr_no_count(13usize, 13usize, 31usize, 2184404u32);
    emu.sri_no_count(31usize, 22usize, 3u32, 2184408u32);
    emu.xrr_no_count(28usize, 29usize, 31usize, 2184412u32);
    emu.sw_no_count(28usize, 2usize, 328u32, 2184416u32)?;
    emu.sri_no_count(29usize, 20usize, 3u32, 2184420u32);
    emu.adi_no_count(22usize, 20usize, 0u32, 2184424u32);
    emu.sw_no_count(20usize, 2usize, 284u32, 2184428u32)?;
    emu.xrr_no_count(12usize, 12usize, 29usize, 2184432u32);
    emu.sw_no_count(12usize, 2usize, 280u32, 2184436u32)?;
    emu.adr_no_count(18usize, 7usize, 17usize, 2184440u32);
    emu.adr_no_count(7usize, 17usize, 26usize, 2184444u32);
    emu.lw_no_count(12usize, 2usize, 468u32, 2184448u32)?;
    emu.lw_no_count(17usize, 2usize, 352u32, 2184452u32)?;
    emu.adr_no_count(12usize, 17usize, 12usize, 2184456u32);
    emu.lw_no_count(17usize, 2usize, 488u32, 2184460u32)?;
    emu.adr_no_count(12usize, 12usize, 17usize, 2184464u32);
    emu.adr_no_count(28usize, 12usize, 30usize, 2184468u32);
    emu.lw_no_count(12usize, 2usize, 484u32, 2184472u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2184476u32)?;
    emu.adr_no_count(12usize, 17usize, 12usize, 2184480u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2184484u32);
    emu.adr_no_count(20usize, 12usize, 13usize, 2184488u32);
    emu.sri_no_count(12usize, 7usize, 6u32, 2184492u32);
    emu.sli_no_count(13usize, 7usize, 26u32, 2184496u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2184500u32);
    emu.sri_no_count(13usize, 7usize, 11u32, 2184504u32);
    emu.sli_no_count(29usize, 7usize, 21u32, 2184508u32);
    emu.orr_no_count(13usize, 29usize, 13usize, 2184512u32);
    emu.sri_no_count(29usize, 7usize, 25u32, 2184516u32);
    emu.sli_no_count(30usize, 7usize, 7u32, 2184520u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2184524u32);
    emu.lw_no_count(27usize, 2usize, 408u32, 2184528u32)?;
    emu.adr_no_count(27usize, 27usize, 14usize, 2184532u32);
    emu.xrr_no_count(30usize, 6usize, 14usize, 2184536u32);
    emu.anr_no_count(30usize, 7usize, 30usize, 2184540u32);
    emu.xrr_no_count(14usize, 30usize, 14usize, 2184544u32);
    emu.sri_no_count(30usize, 18usize, 2u32, 2184548u32);
    emu.sli_no_count(31usize, 18usize, 30u32, 2184552u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2184556u32);
    emu.sri_no_count(31usize, 18usize, 13u32, 2184560u32);
    emu.sli_no_count(8usize, 18usize, 19u32, 2184564u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2184568u32);
    emu.sri_no_count(8usize, 18usize, 22u32, 2184572u32);
    emu.sli_no_count(9usize, 18usize, 10u32, 2184576u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2184580u32);
    emu.xrr_no_count(9usize, 21usize, 10usize, 2184584u32);
    emu.anr_no_count(9usize, 18usize, 9usize, 2184588u32);
    emu.anr_no_count(19usize, 21usize, 10usize, 2184592u32);
    emu.xrr_no_count(9usize, 9usize, 19usize, 2184596u32);
    emu.adi_no_count(5usize, 28usize, 0u32, 2184600u32);
    emu.sri_no_count(19usize, 28usize, 17u32, 2184604u32);
    emu.sli_no_count(24usize, 28usize, 15u32, 2184608u32);
    emu.orr_no_count(19usize, 24usize, 19usize, 2184612u32);
    emu.sri_no_count(24usize, 28usize, 19u32, 2184616u32);
    emu.sli_no_count(25usize, 28usize, 13u32, 2184620u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2184624u32);
    emu.sri_no_count(25usize, 20usize, 17u32, 2184628u32);
    emu.sli_no_count(26usize, 20usize, 15u32, 2184632u32);
    emu.orr_no_count(25usize, 26usize, 25usize, 2184636u32);
    emu.sri_no_count(26usize, 20usize, 19u32, 2184640u32);
    emu.sli_no_count(23usize, 20usize, 13u32, 2184644u32);
    emu.sw_no_count(20usize, 2usize, 420u32, 2184648u32)?;
    emu.orr_no_count(23usize, 23usize, 26usize, 2184652u32);
    emu.sri_no_count(26usize, 28usize, 7u32, 2184656u32);
    emu.sli_no_count(17usize, 28usize, 25u32, 2184660u32);
    emu.orr_no_count(17usize, 17usize, 26usize, 2184664u32);
    emu.sri_no_count(26usize, 28usize, 18u32, 2184668u32);
    emu.sli_no_count(28usize, 28usize, 14u32, 2184672u32);
    emu.orr_no_count(28usize, 28usize, 26usize, 2184676u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2184680u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2184684u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2184688u32);
    emu.xrr_no_count(14usize, 19usize, 24usize, 2184692u32);
    emu.xrr_no_count(30usize, 25usize, 23usize, 2184696u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2184700u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2184704u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2184708u32);
    emu.sri_no_count(28usize, 5usize, 10u32, 2184712u32);
    emu.xrr_no_count(25usize, 14usize, 28usize, 2184716u32);
    emu.sri_no_count(14usize, 20usize, 10u32, 2184720u32);
    emu.xrr_no_count(26usize, 30usize, 14usize, 2184724u32);
    emu.sri_no_count(14usize, 5usize, 3u32, 2184728u32);
    emu.sw_no_count(5usize, 2usize, 340u32, 2184732u32)?;
    emu.xrr_no_count(14usize, 17usize, 14usize, 2184736u32);
    emu.sw_no_count(14usize, 2usize, 296u32, 2184740u32)?;
    emu.lw_no_count(14usize, 2usize, 204u32, 2184744u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2184748u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2184752u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2184756u32);
    emu.lw_no_count(12usize, 2usize, 464u32, 2184760u32)?;
    emu.lw_no_count(14usize, 2usize, 348u32, 2184764u32)?;
    emu.adr_no_count(12usize, 14usize, 12usize, 2184768u32);
    emu.lw_no_count(20usize, 2usize, 380u32, 2184772u32)?;
    emu.adr_no_count(12usize, 12usize, 20usize, 2184776u32);
    emu.adr_no_count(25usize, 12usize, 25usize, 2184780u32);
    emu.lw_no_count(12usize, 2usize, 480u32, 2184784u32)?;
    emu.lw_no_count(14usize, 2usize, 312u32, 2184788u32)?;
    emu.adr_no_count(12usize, 14usize, 12usize, 2184792u32);
    emu.adr_no_count(12usize, 12usize, 15usize, 2184796u32);
    emu.adr_no_count(26usize, 12usize, 26usize, 2184800u32);
    emu.adr_no_count(8usize, 13usize, 11usize, 2184804u32);
    emu.adr_no_count(24usize, 11usize, 1usize, 2184808u32);
    emu.sri_no_count(11usize, 25usize, 17u32, 2184812u32);
    emu.sli_no_count(12usize, 25usize, 15u32, 2184816u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2184820u32);
    emu.sri_no_count(12usize, 25usize, 19u32, 2184824u32);
    emu.sli_no_count(13usize, 25usize, 13u32, 2184828u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2184832u32);
    emu.sri_no_count(13usize, 26usize, 17u32, 2184836u32);
    emu.sli_no_count(14usize, 26usize, 15u32, 2184840u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2184844u32);
    emu.sri_no_count(14usize, 26usize, 19u32, 2184848u32);
    emu.sli_no_count(17usize, 26usize, 13u32, 2184852u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2184856u32);
    emu.sri_no_count(17usize, 24usize, 6u32, 2184860u32);
    emu.sli_no_count(28usize, 24usize, 26u32, 2184864u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2184868u32);
    emu.sri_no_count(28usize, 24usize, 11u32, 2184872u32);
    emu.sli_no_count(29usize, 24usize, 21u32, 2184876u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2184880u32);
    emu.sri_no_count(29usize, 24usize, 25u32, 2184884u32);
    emu.sli_no_count(30usize, 24usize, 7u32, 2184888u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2184892u32);
    emu.lw_no_count(30usize, 2usize, 492u32, 2184896u32)?;
    emu.adr_no_count(30usize, 30usize, 6usize, 2184900u32);
    emu.xrr_no_count(31usize, 7usize, 6usize, 2184904u32);
    emu.anr_no_count(31usize, 24usize, 31usize, 2184908u32);
    emu.xrr_no_count(6usize, 31usize, 6usize, 2184912u32);
    emu.sri_no_count(31usize, 8usize, 2u32, 2184916u32);
    emu.sli_no_count(9usize, 8usize, 30u32, 2184920u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2184924u32);
    emu.sri_no_count(9usize, 8usize, 13u32, 2184928u32);
    emu.sli_no_count(19usize, 8usize, 19u32, 2184932u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2184936u32);
    emu.sri_no_count(19usize, 8usize, 22u32, 2184940u32);
    emu.sli_no_count(23usize, 8usize, 10u32, 2184944u32);
    emu.orr_no_count(19usize, 23usize, 19usize, 2184948u32);
    emu.xrr_no_count(23usize, 18usize, 21usize, 2184952u32);
    emu.anr_no_count(23usize, 8usize, 23usize, 2184956u32);
    emu.anr_no_count(1usize, 18usize, 21usize, 2184960u32);
    emu.xrr_no_count(23usize, 23usize, 1usize, 2184964u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2184968u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2184972u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2184976u32);
    emu.adr_no_count(6usize, 27usize, 6usize, 2184980u32);
    emu.xrr_no_count(14usize, 31usize, 9usize, 2184984u32);
    emu.sri_no_count(17usize, 25usize, 10u32, 2184988u32);
    emu.sw_no_count(25usize, 2usize, 344u32, 2184992u32)?;
    emu.xrr_no_count(27usize, 11usize, 17usize, 2184996u32);
    emu.sri_no_count(11usize, 26usize, 10u32, 2185000u32);
    emu.sw_no_count(26usize, 2usize, 348u32, 2185004u32)?;
    emu.xrr_no_count(11usize, 13usize, 11usize, 2185008u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2185012u32);
    emu.xrr_no_count(13usize, 14usize, 19usize, 2185016u32);
    emu.lw_no_count(14usize, 2usize, 388u32, 2185020u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2185024u32)?;
    emu.adr_no_count(14usize, 15usize, 14usize, 2185028u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2185032u32);
    emu.adr_no_count(27usize, 14usize, 27usize, 2185036u32);
    emu.lw_no_count(14usize, 2usize, 448u32, 2185040u32)?;
    emu.lw_no_count(15usize, 2usize, 308u32, 2185044u32)?;
    emu.adr_no_count(14usize, 15usize, 14usize, 2185048u32);
    emu.adr_no_count(14usize, 14usize, 22usize, 2185052u32);
    emu.adr_no_count(22usize, 14usize, 11usize, 2185056u32);
    emu.lw_no_count(11usize, 2usize, 200u32, 2185060u32)?;
    emu.adr_no_count(6usize, 6usize, 11usize, 2185064u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2185068u32);
    emu.adr_no_count(13usize, 13usize, 23usize, 2185072u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2185076u32);
    emu.adr_no_count(6usize, 12usize, 10usize, 2185080u32);
    emu.sri_no_count(10usize, 27usize, 17u32, 2185084u32);
    emu.sli_no_count(11usize, 27usize, 15u32, 2185088u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2185092u32);
    emu.sri_no_count(11usize, 27usize, 19u32, 2185096u32);
    emu.sli_no_count(12usize, 27usize, 13u32, 2185100u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2185104u32);
    emu.sri_no_count(12usize, 22usize, 17u32, 2185108u32);
    emu.sli_no_count(13usize, 22usize, 15u32, 2185112u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2185116u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2185120u32);
    emu.sli_no_count(17usize, 22usize, 13u32, 2185124u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2185128u32);
    emu.xrr_no_count(10usize, 10usize, 11usize, 2185132u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2185136u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2185140u32);
    emu.sli_no_count(13usize, 6usize, 26u32, 2185144u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2185148u32);
    emu.sri_no_count(11usize, 6usize, 11u32, 2185152u32);
    emu.sli_no_count(17usize, 6usize, 21u32, 2185156u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2185160u32);
    emu.sri_no_count(11usize, 6usize, 25u32, 2185164u32);
    emu.sli_no_count(28usize, 6usize, 7u32, 2185168u32);
    emu.orr_no_count(28usize, 28usize, 11usize, 2185172u32);
    emu.lw_no_count(11usize, 2usize, 496u32, 2185176u32)?;
    emu.adr_no_count(11usize, 11usize, 7usize, 2185180u32);
    emu.xrr_no_count(29usize, 24usize, 7usize, 2185184u32);
    emu.anr_no_count(29usize, 6usize, 29usize, 2185188u32);
    emu.xrr_no_count(7usize, 29usize, 7usize, 2185192u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2185196u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2185200u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2185204u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2185208u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2185212u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2185216u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2185220u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2185224u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2185228u32);
    emu.xrr_no_count(19usize, 8usize, 18usize, 2185232u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2185236u32);
    emu.anr_no_count(23usize, 8usize, 18usize, 2185240u32);
    emu.xrr_no_count(19usize, 19usize, 23usize, 2185244u32);
    emu.sri_no_count(23usize, 27usize, 10u32, 2185248u32);
    emu.sw_no_count(27usize, 2usize, 336u32, 2185252u32)?;
    emu.xrr_no_count(1usize, 10usize, 23usize, 2185256u32);
    emu.sri_no_count(10usize, 22usize, 10u32, 2185260u32);
    emu.sw_no_count(22usize, 2usize, 352u32, 2185264u32)?;
    emu.xrr_no_count(10usize, 12usize, 10usize, 2185268u32);
    emu.xrr_no_count(12usize, 13usize, 17usize, 2185272u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2185276u32);
    emu.xrr_no_count(13usize, 29usize, 31usize, 2185280u32);
    emu.lw_no_count(17usize, 2usize, 452u32, 2185284u32)?;
    emu.lw_no_count(15usize, 2usize, 316u32, 2185288u32)?;
    emu.adr_no_count(17usize, 15usize, 17usize, 2185292u32);
    emu.lw_no_count(15usize, 2usize, 416u32, 2185296u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2185300u32);
    emu.adr_no_count(1usize, 17usize, 1usize, 2185304u32);
    emu.lw_no_count(17usize, 2usize, 304u32, 2185308u32)?;
    emu.lw_no_count(15usize, 2usize, 376u32, 2185312u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2185316u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2185320u32);
    emu.adr_no_count(15usize, 17usize, 10usize, 2185324u32);
    emu.sw_no_count(15usize, 2usize, 496u32, 2185328u32)?;
    emu.xrr_no_count(10usize, 12usize, 28usize, 2185332u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2185336u32);
    emu.lw_no_count(12usize, 2usize, 196u32, 2185340u32)?;
    emu.adr_no_count(7usize, 7usize, 12usize, 2185344u32);
    emu.adr_no_count(7usize, 7usize, 10usize, 2185348u32);
    emu.adr_no_count(10usize, 13usize, 19usize, 2185352u32);
    emu.sri_no_count(12usize, 1usize, 17u32, 2185356u32);
    emu.sli_no_count(13usize, 1usize, 15u32, 2185360u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2185364u32);
    emu.sri_no_count(13usize, 1usize, 19u32, 2185368u32);
    emu.sli_no_count(17usize, 1usize, 13u32, 2185372u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2185376u32);
    emu.sri_no_count(17usize, 15usize, 17u32, 2185380u32);
    emu.sli_no_count(28usize, 15usize, 15u32, 2185384u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2185388u32);
    emu.sri_no_count(28usize, 15usize, 19u32, 2185392u32);
    emu.sli_no_count(29usize, 15usize, 13u32, 2185396u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2185400u32);
    emu.adr_no_count(16usize, 10usize, 7usize, 2185404u32);
    emu.adr_no_count(7usize, 7usize, 21usize, 2185408u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2185412u32);
    emu.xrr_no_count(13usize, 17usize, 28usize, 2185416u32);
    emu.sri_no_count(17usize, 1usize, 10u32, 2185420u32);
    emu.sw_no_count(1usize, 2usize, 356u32, 2185424u32)?;
    emu.xrr_no_count(12usize, 12usize, 17usize, 2185428u32);
    emu.sri_no_count(17usize, 15usize, 10u32, 2185432u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2185436u32);
    emu.sri_no_count(17usize, 7usize, 6u32, 2185440u32);
    emu.sli_no_count(28usize, 7usize, 26u32, 2185444u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2185448u32);
    emu.sri_no_count(28usize, 7usize, 11u32, 2185452u32);
    emu.sli_no_count(29usize, 7usize, 21u32, 2185456u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2185460u32);
    emu.sri_no_count(29usize, 7usize, 25u32, 2185464u32);
    emu.sli_no_count(30usize, 7usize, 7u32, 2185468u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2185472u32);
    emu.lw_no_count(30usize, 2usize, 440u32, 2185476u32)?;
    emu.adr_no_count(30usize, 30usize, 24usize, 2185480u32);
    emu.xrr_no_count(31usize, 6usize, 24usize, 2185484u32);
    emu.anr_no_count(31usize, 7usize, 31usize, 2185488u32);
    emu.xrr_no_count(31usize, 31usize, 24usize, 2185492u32);
    emu.sri_no_count(9usize, 16usize, 2u32, 2185496u32);
    emu.sli_no_count(19usize, 16usize, 30u32, 2185500u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2185504u32);
    emu.sri_no_count(19usize, 16usize, 13u32, 2185508u32);
    emu.sli_no_count(21usize, 16usize, 19u32, 2185512u32);
    emu.orr_no_count(19usize, 21usize, 19usize, 2185516u32);
    emu.sri_no_count(21usize, 16usize, 22u32, 2185520u32);
    emu.sli_no_count(23usize, 16usize, 10u32, 2185524u32);
    emu.orr_no_count(21usize, 23usize, 21usize, 2185528u32);
    emu.xrr_no_count(23usize, 14usize, 8usize, 2185532u32);
    emu.anr_no_count(23usize, 16usize, 23usize, 2185536u32);
    emu.anr_no_count(24usize, 14usize, 8usize, 2185540u32);
    emu.xrr_no_count(23usize, 23usize, 24usize, 2185544u32);
    emu.lw_no_count(24usize, 2usize, 384u32, 2185548u32)?;
    emu.lw_no_count(10usize, 2usize, 360u32, 2185552u32)?;
    emu.adr_no_count(24usize, 10usize, 24usize, 2185556u32);
    emu.lw_no_count(10usize, 2usize, 420u32, 2185560u32)?;
    emu.adr_no_count(24usize, 24usize, 10usize, 2185564u32);
    emu.adr_no_count(5usize, 24usize, 12usize, 2185568u32);
    emu.lw_no_count(12usize, 2usize, 300u32, 2185572u32)?;
    emu.lw_no_count(10usize, 2usize, 488u32, 2185576u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2185580u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2185584u32);
    emu.adr_no_count(15usize, 12usize, 13usize, 2185588u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2185592u32);
    emu.adr_no_count(11usize, 11usize, 31usize, 2185596u32);
    emu.xrr_no_count(13usize, 9usize, 19usize, 2185600u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2185604u32);
    emu.xrr_no_count(13usize, 13usize, 21usize, 2185608u32);
    emu.sri_no_count(17usize, 5usize, 17u32, 2185612u32);
    emu.sli_no_count(28usize, 5usize, 15u32, 2185616u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2185620u32);
    emu.sri_no_count(28usize, 5usize, 19u32, 2185624u32);
    emu.sli_no_count(29usize, 5usize, 13u32, 2185628u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2185632u32);
    emu.sri_no_count(29usize, 15usize, 17u32, 2185636u32);
    emu.sli_no_count(31usize, 15usize, 15u32, 2185640u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2185644u32);
    emu.sri_no_count(31usize, 15usize, 19u32, 2185648u32);
    emu.sli_no_count(9usize, 15usize, 13u32, 2185652u32);
    emu.sw_no_count(15usize, 2usize, 440u32, 2185656u32)?;
    emu.orr_no_count(31usize, 9usize, 31usize, 2185660u32);
    emu.lw_no_count(9usize, 2usize, 192u32, 2185664u32)?;
    emu.adr_no_count(11usize, 11usize, 9usize, 2185668u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2185672u32);
    emu.adr_no_count(13usize, 13usize, 23usize, 2185676u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2185680u32);
    emu.xrr_no_count(17usize, 29usize, 31usize, 2185684u32);
    emu.adr_no_count(24usize, 13usize, 11usize, 2185688u32);
    emu.adr_no_count(18usize, 11usize, 18usize, 2185692u32);
    emu.sri_no_count(11usize, 5usize, 10u32, 2185696u32);
    emu.sw_no_count(5usize, 2usize, 332u32, 2185700u32)?;
    emu.xrr_no_count(11usize, 12usize, 11usize, 2185704u32);
    emu.sri_no_count(12usize, 15usize, 10u32, 2185708u32);
    emu.xrr_no_count(12usize, 17usize, 12usize, 2185712u32);
    emu.lw_no_count(13usize, 2usize, 412u32, 2185716u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2185720u32)?;
    emu.adr_no_count(13usize, 10usize, 13usize, 2185724u32);
    emu.adr_no_count(13usize, 13usize, 26usize, 2185728u32);
    emu.adr_no_count(10usize, 13usize, 11usize, 2185732u32);
    emu.lw_no_count(11usize, 2usize, 288u32, 2185736u32)?;
    emu.adr_no_count(11usize, 11usize, 20usize, 2185740u32);
    emu.adr_no_count(11usize, 11usize, 27usize, 2185744u32);
    emu.adr_no_count(15usize, 11usize, 12usize, 2185748u32);
    emu.sri_no_count(11usize, 18usize, 6u32, 2185752u32);
    emu.sli_no_count(12usize, 18usize, 26u32, 2185756u32);
    emu.orr_no_count(12usize, 12usize, 11usize, 2185760u32);
    emu.sri_no_count(11usize, 18usize, 11u32, 2185764u32);
    emu.sli_no_count(13usize, 18usize, 21u32, 2185768u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2185772u32);
    emu.sri_no_count(11usize, 18usize, 25u32, 2185776u32);
    emu.sli_no_count(17usize, 18usize, 7u32, 2185780u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2185784u32);
    emu.lw_no_count(11usize, 2usize, 504u32, 2185788u32)?;
    emu.adr_no_count(11usize, 11usize, 6usize, 2185792u32);
    emu.xrr_no_count(28usize, 7usize, 6usize, 2185796u32);
    emu.anr_no_count(28usize, 18usize, 28usize, 2185800u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2185804u32);
    emu.sri_no_count(28usize, 24usize, 2u32, 2185808u32);
    emu.sli_no_count(29usize, 24usize, 30u32, 2185812u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2185816u32);
    emu.sri_no_count(29usize, 24usize, 13u32, 2185820u32);
    emu.sli_no_count(31usize, 24usize, 19u32, 2185824u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2185828u32);
    emu.sri_no_count(31usize, 24usize, 22u32, 2185832u32);
    emu.sli_no_count(9usize, 24usize, 10u32, 2185836u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2185840u32);
    emu.xrr_no_count(9usize, 16usize, 14usize, 2185844u32);
    emu.anr_no_count(9usize, 24usize, 9usize, 2185848u32);
    emu.anr_no_count(19usize, 16usize, 14usize, 2185852u32);
    emu.xrr_no_count(9usize, 9usize, 19usize, 2185856u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2185860u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2185864u32);
    emu.xrr_no_count(13usize, 28usize, 29usize, 2185868u32);
    emu.sri_no_count(28usize, 10usize, 17u32, 2185872u32);
    emu.sli_no_count(29usize, 10usize, 15u32, 2185876u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2185880u32);
    emu.sri_no_count(29usize, 10usize, 19u32, 2185884u32);
    emu.sli_no_count(30usize, 10usize, 13u32, 2185888u32);
    emu.sw_no_count(10usize, 2usize, 364u32, 2185892u32)?;
    emu.orr_no_count(29usize, 30usize, 29usize, 2185896u32);
    emu.sri_no_count(30usize, 15usize, 17u32, 2185900u32);
    emu.sli_no_count(19usize, 15usize, 15u32, 2185904u32);
    emu.orr_no_count(30usize, 19usize, 30usize, 2185908u32);
    emu.sri_no_count(19usize, 15usize, 19u32, 2185912u32);
    emu.sli_no_count(21usize, 15usize, 13u32, 2185916u32);
    emu.sw_no_count(15usize, 2usize, 492u32, 2185920u32)?;
    emu.orr_no_count(19usize, 21usize, 19usize, 2185924u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2185928u32);
    emu.xrr_no_count(13usize, 13usize, 31usize, 2185932u32);
    emu.xrr_no_count(17usize, 28usize, 29usize, 2185936u32);
    emu.xrr_no_count(28usize, 30usize, 19usize, 2185940u32);
    emu.lw_no_count(29usize, 2usize, 188u32, 2185944u32)?;
    emu.adr_no_count(6usize, 6usize, 29usize, 2185948u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2185952u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2185956u32);
    emu.sri_no_count(6usize, 10usize, 10u32, 2185960u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2185964u32);
    emu.sri_no_count(6usize, 15usize, 10u32, 2185968u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2185972u32);
    emu.adr_no_count(21usize, 13usize, 12usize, 2185976u32);
    emu.adr_no_count(8usize, 12usize, 8usize, 2185980u32);
    emu.lw_no_count(27usize, 2usize, 276u32, 2185984u32)?;
    emu.lw_no_count(12usize, 2usize, 292u32, 2185988u32)?;
    emu.adr_no_count(12usize, 12usize, 27usize, 2185992u32);
    emu.adr_no_count(12usize, 12usize, 22usize, 2185996u32);
    emu.adr_no_count(10usize, 12usize, 17usize, 2186000u32);
    emu.lw_no_count(22usize, 2usize, 272u32, 2186004u32)?;
    emu.lw_no_count(12usize, 2usize, 280u32, 2186008u32)?;
    emu.adr_no_count(12usize, 12usize, 22usize, 2186012u32);
    emu.adr_no_count(12usize, 12usize, 1usize, 2186016u32);
    emu.adr_no_count(15usize, 12usize, 6usize, 2186020u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2186024u32);
    emu.sli_no_count(13usize, 8usize, 26u32, 2186028u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2186032u32);
    emu.sri_no_count(13usize, 8usize, 11u32, 2186036u32);
    emu.sli_no_count(17usize, 8usize, 21u32, 2186040u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2186044u32);
    emu.sri_no_count(17usize, 8usize, 25u32, 2186048u32);
    emu.sli_no_count(6usize, 8usize, 7u32, 2186052u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2186056u32);
    emu.lw_no_count(6usize, 2usize, 444u32, 2186060u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2186064u32);
    emu.xrr_no_count(28usize, 18usize, 7usize, 2186068u32);
    emu.anr_no_count(28usize, 8usize, 28usize, 2186072u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2186076u32);
    emu.sri_no_count(28usize, 21usize, 2u32, 2186080u32);
    emu.sli_no_count(29usize, 21usize, 30u32, 2186084u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2186088u32);
    emu.sri_no_count(29usize, 21usize, 13u32, 2186092u32);
    emu.sli_no_count(30usize, 21usize, 19u32, 2186096u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2186100u32);
    emu.sri_no_count(30usize, 21usize, 22u32, 2186104u32);
    emu.sli_no_count(31usize, 21usize, 10u32, 2186108u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2186112u32);
    emu.xrr_no_count(31usize, 24usize, 16usize, 2186116u32);
    emu.anr_no_count(31usize, 21usize, 31usize, 2186120u32);
    emu.anr_no_count(9usize, 24usize, 16usize, 2186124u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2186128u32);
    emu.sri_no_count(9usize, 10usize, 17u32, 2186132u32);
    emu.sli_no_count(19usize, 10usize, 15u32, 2186136u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2186140u32);
    emu.sri_no_count(19usize, 10usize, 19u32, 2186144u32);
    emu.sli_no_count(23usize, 10usize, 13u32, 2186148u32);
    emu.sw_no_count(10usize, 2usize, 408u32, 2186152u32)?;
    emu.orr_no_count(19usize, 23usize, 19usize, 2186156u32);
    emu.sri_no_count(23usize, 15usize, 17u32, 2186160u32);
    emu.sli_no_count(26usize, 15usize, 15u32, 2186164u32);
    emu.orr_no_count(23usize, 26usize, 23usize, 2186168u32);
    emu.sri_no_count(26usize, 15usize, 19u32, 2186172u32);
    emu.sli_no_count(1usize, 15usize, 13u32, 2186176u32);
    emu.sw_no_count(15usize, 2usize, 504u32, 2186180u32)?;
    emu.orr_no_count(26usize, 1usize, 26usize, 2186184u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2186188u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2186192u32);
    emu.xrr_no_count(11usize, 28usize, 29usize, 2186196u32);
    emu.xrr_no_count(13usize, 9usize, 19usize, 2186200u32);
    emu.xrr_no_count(28usize, 23usize, 26usize, 2186204u32);
    emu.xrr_no_count(23usize, 12usize, 17usize, 2186208u32);
    emu.xrr_no_count(12usize, 11usize, 30usize, 2186212u32);
    emu.sri_no_count(11usize, 10usize, 10u32, 2186216u32);
    emu.xrr_no_count(30usize, 13usize, 11usize, 2186220u32);
    emu.sri_no_count(11usize, 15usize, 10u32, 2186224u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2186228u32);
    emu.lw_no_count(13usize, 2usize, 184u32, 2186232u32)?;
    emu.adr_no_count(7usize, 7usize, 13usize, 2186236u32);
    emu.adr_no_count(23usize, 7usize, 23usize, 2186240u32);
    emu.adr_no_count(7usize, 12usize, 31usize, 2186244u32);
    emu.lw_no_count(20usize, 2usize, 284u32, 2186248u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2186252u32)?;
    emu.adr_no_count(12usize, 20usize, 12usize, 2186256u32);
    emu.lw_no_count(10usize, 2usize, 496u32, 2186260u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2186264u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2186268u32);
    emu.sw_no_count(12usize, 2usize, 444u32, 2186272u32)?;
    emu.lw_no_count(25usize, 2usize, 416u32, 2186276u32)?;
    emu.lw_no_count(12usize, 2usize, 296u32, 2186280u32)?;
    emu.adr_no_count(12usize, 25usize, 12usize, 2186284u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2186288u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2186292u32);
    emu.sw_no_count(11usize, 2usize, 360u32, 2186296u32)?;
    emu.adr_no_count(7usize, 7usize, 23usize, 2186300u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2186304u32);
    emu.sri_no_count(11usize, 23usize, 6u32, 2186308u32);
    emu.sli_no_count(12usize, 23usize, 26u32, 2186312u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2186316u32);
    emu.sri_no_count(12usize, 23usize, 11u32, 2186320u32);
    emu.sli_no_count(13usize, 23usize, 21u32, 2186324u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2186328u32);
    emu.sri_no_count(13usize, 23usize, 25u32, 2186332u32);
    emu.sli_no_count(14usize, 23usize, 7u32, 2186336u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2186340u32);
    emu.lw_no_count(17usize, 2usize, 500u32, 2186344u32)?;
    emu.adr_no_count(17usize, 17usize, 18usize, 2186348u32);
    emu.xrr_no_count(14usize, 8usize, 18usize, 2186352u32);
    emu.anr_no_count(14usize, 23usize, 14usize, 2186356u32);
    emu.xrr_no_count(14usize, 14usize, 18usize, 2186360u32);
    emu.sri_no_count(28usize, 7usize, 2u32, 2186364u32);
    emu.sli_no_count(29usize, 7usize, 30u32, 2186368u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2186372u32);
    emu.sri_no_count(29usize, 7usize, 13u32, 2186376u32);
    emu.sli_no_count(30usize, 7usize, 19u32, 2186380u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2186384u32);
    emu.sri_no_count(30usize, 7usize, 22u32, 2186388u32);
    emu.sli_no_count(31usize, 7usize, 10u32, 2186392u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2186396u32);
    emu.xrr_no_count(31usize, 21usize, 24usize, 2186400u32);
    emu.anr_no_count(31usize, 7usize, 31usize, 2186404u32);
    emu.anr_no_count(9usize, 21usize, 24usize, 2186408u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2186412u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2186416u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2186420u32);
    emu.xrr_no_count(12usize, 28usize, 29usize, 2186424u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2186428u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2186432u32);
    emu.lw_no_count(13usize, 2usize, 180u32, 2186436u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2186440u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2186444u32);
    emu.adr_no_count(12usize, 12usize, 31usize, 2186448u32);
    emu.adr_no_count(14usize, 12usize, 11usize, 2186452u32);
    emu.adr_no_count(11usize, 11usize, 16usize, 2186456u32);
    emu.sri_no_count(10usize, 11usize, 6u32, 2186460u32);
    emu.sli_no_count(12usize, 11usize, 26u32, 2186464u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2186468u32);
    emu.sri_no_count(12usize, 11usize, 11u32, 2186472u32);
    emu.sli_no_count(13usize, 11usize, 21u32, 2186476u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2186480u32);
    emu.sri_no_count(13usize, 11usize, 25u32, 2186484u32);
    emu.sli_no_count(6usize, 11usize, 7u32, 2186488u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2186492u32);
    emu.lw_no_count(6usize, 2usize, 476u32, 2186496u32)?;
    emu.adr_no_count(6usize, 6usize, 8usize, 2186500u32);
    emu.xrr_no_count(28usize, 23usize, 8usize, 2186504u32);
    emu.anr_no_count(28usize, 11usize, 28usize, 2186508u32);
    emu.xrr_no_count(28usize, 28usize, 8usize, 2186512u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2186516u32);
    emu.sli_no_count(30usize, 14usize, 30u32, 2186520u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2186524u32);
    emu.sri_no_count(30usize, 14usize, 13u32, 2186528u32);
    emu.sli_no_count(31usize, 14usize, 19u32, 2186532u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2186536u32);
    emu.sri_no_count(31usize, 14usize, 22u32, 2186540u32);
    emu.sli_no_count(8usize, 14usize, 10u32, 2186544u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2186548u32);
    emu.xrr_no_count(8usize, 7usize, 21usize, 2186552u32);
    emu.anr_no_count(8usize, 14usize, 8usize, 2186556u32);
    emu.anr_no_count(9usize, 7usize, 21usize, 2186560u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2186564u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2186568u32);
    emu.adr_no_count(17usize, 17usize, 28usize, 2186572u32);
    emu.xrr_no_count(12usize, 29usize, 30usize, 2186576u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2186580u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2186584u32);
    emu.lw_no_count(13usize, 2usize, 176u32, 2186588u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2186592u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2186596u32);
    emu.adr_no_count(10usize, 12usize, 8usize, 2186600u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2186604u32);
    emu.adr_no_count(24usize, 17usize, 24usize, 2186608u32);
    emu.sri_no_count(12usize, 24usize, 6u32, 2186612u32);
    emu.sli_no_count(13usize, 24usize, 26u32, 2186616u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2186620u32);
    emu.sri_no_count(13usize, 24usize, 11u32, 2186624u32);
    emu.sli_no_count(17usize, 24usize, 21u32, 2186628u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2186632u32);
    emu.sri_no_count(17usize, 24usize, 25u32, 2186636u32);
    emu.sli_no_count(28usize, 24usize, 7u32, 2186640u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2186644u32);
    emu.lw_no_count(28usize, 2usize, 404u32, 2186648u32)?;
    emu.adr_no_count(28usize, 28usize, 23usize, 2186652u32);
    emu.xrr_no_count(29usize, 11usize, 23usize, 2186656u32);
    emu.anr_no_count(29usize, 24usize, 29usize, 2186660u32);
    emu.xrr_no_count(29usize, 29usize, 23usize, 2186664u32);
    emu.sri_no_count(30usize, 10usize, 2u32, 2186668u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2186672u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2186676u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2186680u32);
    emu.sli_no_count(8usize, 10usize, 19u32, 2186684u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2186688u32);
    emu.sri_no_count(8usize, 10usize, 22u32, 2186692u32);
    emu.sli_no_count(9usize, 10usize, 10u32, 2186696u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2186700u32);
    emu.xrr_no_count(9usize, 14usize, 7usize, 2186704u32);
    emu.anr_no_count(9usize, 10usize, 9usize, 2186708u32);
    emu.anr_no_count(18usize, 14usize, 7usize, 2186712u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2186716u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2186720u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2186724u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2186728u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2186732u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2186736u32);
    emu.lw_no_count(17usize, 2usize, 172u32, 2186740u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2186744u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2186748u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2186752u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2186756u32);
    emu.adr_no_count(21usize, 12usize, 21usize, 2186760u32);
    emu.sri_no_count(12usize, 21usize, 6u32, 2186764u32);
    emu.sli_no_count(13usize, 21usize, 26u32, 2186768u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2186772u32);
    emu.sri_no_count(13usize, 21usize, 11u32, 2186776u32);
    emu.sli_no_count(17usize, 21usize, 21u32, 2186780u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2186784u32);
    emu.sri_no_count(17usize, 21usize, 25u32, 2186788u32);
    emu.sli_no_count(29usize, 21usize, 7u32, 2186792u32);
    emu.orr_no_count(17usize, 29usize, 17usize, 2186796u32);
    emu.lw_no_count(29usize, 2usize, 472u32, 2186800u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2186804u32);
    emu.xrr_no_count(30usize, 24usize, 11usize, 2186808u32);
    emu.anr_no_count(30usize, 21usize, 30usize, 2186812u32);
    emu.xrr_no_count(11usize, 30usize, 11usize, 2186816u32);
    emu.sri_no_count(30usize, 6usize, 2u32, 2186820u32);
    emu.sli_no_count(31usize, 6usize, 30u32, 2186824u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2186828u32);
    emu.sri_no_count(31usize, 6usize, 13u32, 2186832u32);
    emu.sli_no_count(8usize, 6usize, 19u32, 2186836u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2186840u32);
    emu.sri_no_count(8usize, 6usize, 22u32, 2186844u32);
    emu.sli_no_count(9usize, 6usize, 10u32, 2186848u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2186852u32);
    emu.xrr_no_count(9usize, 10usize, 14usize, 2186856u32);
    emu.anr_no_count(9usize, 6usize, 9usize, 2186860u32);
    emu.anr_no_count(18usize, 10usize, 14usize, 2186864u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2186868u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2186872u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2186876u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2186880u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2186884u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2186888u32);
    emu.lw_no_count(17usize, 2usize, 168u32, 2186892u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2186896u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2186900u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2186904u32);
    emu.adr_no_count(8usize, 13usize, 11usize, 2186908u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2186912u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2186916u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2186920u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2186924u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2186928u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2186932u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2186936u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2186940u32);
    emu.sli_no_count(7usize, 11usize, 7u32, 2186944u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2186948u32);
    emu.lw_no_count(28usize, 2usize, 400u32, 2186952u32)?;
    emu.adr_no_count(28usize, 28usize, 24usize, 2186956u32);
    emu.xrr_no_count(7usize, 21usize, 24usize, 2186960u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2186964u32);
    emu.xrr_no_count(7usize, 7usize, 24usize, 2186968u32);
    emu.sri_no_count(30usize, 8usize, 2u32, 2186972u32);
    emu.sli_no_count(31usize, 8usize, 30u32, 2186976u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2186980u32);
    emu.sri_no_count(31usize, 8usize, 13u32, 2186984u32);
    emu.sli_no_count(9usize, 8usize, 19u32, 2186988u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2186992u32);
    emu.sri_no_count(9usize, 8usize, 22u32, 2186996u32);
    emu.sli_no_count(18usize, 8usize, 10u32, 2187000u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2187004u32);
    emu.xrr_no_count(18usize, 6usize, 10usize, 2187008u32);
    emu.anr_no_count(18usize, 8usize, 18usize, 2187012u32);
    emu.anr_no_count(19usize, 6usize, 10usize, 2187016u32);
    emu.xrr_no_count(18usize, 18usize, 19usize, 2187020u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2187024u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2187028u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2187032u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2187036u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2187040u32);
    emu.lw_no_count(17usize, 2usize, 164u32, 2187044u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2187048u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2187052u32);
    emu.adr_no_count(13usize, 13usize, 18usize, 2187056u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2187060u32);
    emu.adr_no_count(30usize, 12usize, 14usize, 2187064u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2187068u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2187072u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2187076u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2187080u32);
    emu.sli_no_count(14usize, 30usize, 21u32, 2187084u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2187088u32);
    emu.sri_no_count(14usize, 30usize, 25u32, 2187092u32);
    emu.sli_no_count(17usize, 30usize, 7u32, 2187096u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2187100u32);
    emu.lw_no_count(17usize, 2usize, 460u32, 2187104u32)?;
    emu.adr_no_count(17usize, 17usize, 21usize, 2187108u32);
    emu.xrr_no_count(29usize, 11usize, 21usize, 2187112u32);
    emu.anr_no_count(29usize, 30usize, 29usize, 2187116u32);
    emu.xrr_no_count(29usize, 29usize, 21usize, 2187120u32);
    emu.sri_no_count(31usize, 7usize, 2u32, 2187124u32);
    emu.sli_no_count(9usize, 7usize, 30u32, 2187128u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2187132u32);
    emu.sri_no_count(9usize, 7usize, 13u32, 2187136u32);
    emu.sli_no_count(18usize, 7usize, 19u32, 2187140u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2187144u32);
    emu.sri_no_count(18usize, 7usize, 22u32, 2187148u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2187152u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2187156u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2187160u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2187164u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2187168u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2187172u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2187176u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2187180u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2187184u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2187188u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2187192u32);
    emu.lw_no_count(14usize, 2usize, 160u32, 2187196u32)?;
    emu.adr_no_count(28usize, 28usize, 14usize, 2187200u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2187204u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2187208u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2187212u32);
    emu.adr_no_count(18usize, 12usize, 10usize, 2187216u32);
    emu.sri_no_count(10usize, 18usize, 6u32, 2187220u32);
    emu.sli_no_count(12usize, 18usize, 26u32, 2187224u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2187228u32);
    emu.sri_no_count(12usize, 18usize, 11u32, 2187232u32);
    emu.sli_no_count(13usize, 18usize, 21u32, 2187236u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2187240u32);
    emu.sri_no_count(13usize, 18usize, 25u32, 2187244u32);
    emu.sli_no_count(28usize, 18usize, 7u32, 2187248u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2187252u32);
    emu.lw_no_count(28usize, 2usize, 396u32, 2187256u32)?;
    emu.adr_no_count(28usize, 28usize, 11usize, 2187260u32);
    emu.xrr_no_count(29usize, 30usize, 11usize, 2187264u32);
    emu.anr_no_count(29usize, 18usize, 29usize, 2187268u32);
    emu.xrr_no_count(11usize, 29usize, 11usize, 2187272u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2187276u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2187280u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2187284u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2187288u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2187292u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2187296u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2187300u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2187304u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2187308u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2187312u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2187316u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2187320u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2187324u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2187328u32);
    emu.adr_no_count(11usize, 17usize, 11usize, 2187332u32);
    emu.xrr_no_count(12usize, 29usize, 31usize, 2187336u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2187340u32);
    emu.xrr_no_count(12usize, 12usize, 9usize, 2187344u32);
    emu.lw_no_count(13usize, 2usize, 156u32, 2187348u32)?;
    emu.adr_no_count(11usize, 11usize, 13usize, 2187352u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2187356u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2187360u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2187364u32);
    emu.adr_no_count(11usize, 11usize, 6usize, 2187368u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2187372u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2187376u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2187380u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2187384u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2187388u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2187392u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2187396u32);
    emu.sli_no_count(6usize, 11usize, 7u32, 2187400u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2187404u32);
    emu.lw_no_count(29usize, 2usize, 456u32, 2187408u32)?;
    emu.adr_no_count(29usize, 29usize, 30usize, 2187412u32);
    emu.xrr_no_count(6usize, 18usize, 30usize, 2187416u32);
    emu.anr_no_count(6usize, 11usize, 6usize, 2187420u32);
    emu.xrr_no_count(6usize, 6usize, 30usize, 2187424u32);
    emu.sri_no_count(30usize, 10usize, 2u32, 2187428u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2187432u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2187436u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2187440u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2187444u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2187448u32);
    emu.sri_no_count(9usize, 10usize, 22u32, 2187452u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2187456u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2187460u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2187464u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2187468u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2187472u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2187476u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2187480u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2187484u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2187488u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2187492u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2187496u32);
    emu.lw_no_count(17usize, 2usize, 152u32, 2187500u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2187504u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2187508u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2187512u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2187516u32);
    emu.adr_no_count(30usize, 12usize, 8usize, 2187520u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2187524u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2187528u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2187532u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2187536u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2187540u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2187544u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2187548u32);
    emu.sli_no_count(28usize, 30usize, 7u32, 2187552u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2187556u32);
    emu.lw_no_count(28usize, 2usize, 392u32, 2187560u32)?;
    emu.adr_no_count(28usize, 28usize, 18usize, 2187564u32);
    emu.xrr_no_count(31usize, 11usize, 18usize, 2187568u32);
    emu.anr_no_count(31usize, 30usize, 31usize, 2187572u32);
    emu.xrr_no_count(31usize, 31usize, 18usize, 2187576u32);
    emu.sri_no_count(8usize, 6usize, 2u32, 2187580u32);
    emu.sli_no_count(9usize, 6usize, 30u32, 2187584u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2187588u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2187592u32);
    emu.sli_no_count(18usize, 6usize, 19u32, 2187596u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2187600u32);
    emu.sri_no_count(18usize, 6usize, 22u32, 2187604u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2187608u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2187612u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2187616u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2187620u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2187624u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2187628u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2187632u32);
    emu.adr_no_count(29usize, 29usize, 31usize, 2187636u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2187640u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2187644u32);
    emu.xrr_no_count(13usize, 8usize, 18usize, 2187648u32);
    emu.lw_no_count(17usize, 2usize, 148u32, 2187652u32)?;
    emu.adr_no_count(29usize, 29usize, 17usize, 2187656u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2187660u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2187664u32);
    emu.adr_no_count(8usize, 13usize, 12usize, 2187668u32);
    emu.adr_no_count(18usize, 12usize, 7usize, 2187672u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2187676u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2187680u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2187684u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2187688u32);
    emu.sli_no_count(17usize, 18usize, 21u32, 2187692u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2187696u32);
    emu.sri_no_count(17usize, 18usize, 25u32, 2187700u32);
    emu.sli_no_count(7usize, 18usize, 7u32, 2187704u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2187708u32);
    emu.lw_no_count(29usize, 2usize, 436u32, 2187712u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2187716u32);
    emu.xrr_no_count(7usize, 30usize, 11usize, 2187720u32);
    emu.anr_no_count(7usize, 18usize, 7usize, 2187724u32);
    emu.xrr_no_count(11usize, 7usize, 11usize, 2187728u32);
    emu.sri_no_count(7usize, 8usize, 2u32, 2187732u32);
    emu.sli_no_count(31usize, 8usize, 30u32, 2187736u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2187740u32);
    emu.sri_no_count(31usize, 8usize, 13u32, 2187744u32);
    emu.sli_no_count(9usize, 8usize, 19u32, 2187748u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2187752u32);
    emu.sri_no_count(9usize, 8usize, 22u32, 2187756u32);
    emu.sli_no_count(19usize, 8usize, 10u32, 2187760u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2187764u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2187768u32);
    emu.anr_no_count(19usize, 8usize, 19usize, 2187772u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2187776u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2187780u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2187784u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2187788u32);
    emu.xrr_no_count(13usize, 7usize, 31usize, 2187792u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2187796u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2187800u32);
    emu.lw_no_count(17usize, 2usize, 144u32, 2187804u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2187808u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2187812u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2187816u32);
    emu.adr_no_count(7usize, 13usize, 11usize, 2187820u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2187824u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2187828u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2187832u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2187836u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2187840u32);
    emu.sli_no_count(14usize, 11usize, 21u32, 2187844u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2187848u32);
    emu.sri_no_count(14usize, 11usize, 25u32, 2187852u32);
    emu.sli_no_count(17usize, 11usize, 7u32, 2187856u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2187860u32);
    emu.lw_no_count(17usize, 2usize, 468u32, 2187864u32)?;
    emu.adr_no_count(17usize, 17usize, 30usize, 2187868u32);
    emu.xrr_no_count(28usize, 18usize, 30usize, 2187872u32);
    emu.anr_no_count(28usize, 11usize, 28usize, 2187876u32);
    emu.xrr_no_count(28usize, 28usize, 30usize, 2187880u32);
    emu.sri_no_count(30usize, 7usize, 2u32, 2187884u32);
    emu.sli_no_count(31usize, 7usize, 30u32, 2187888u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2187892u32);
    emu.sri_no_count(31usize, 7usize, 13u32, 2187896u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2187900u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2187904u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2187908u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2187912u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2187916u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2187920u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2187924u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2187928u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2187932u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2187936u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2187940u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2187944u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2187948u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2187952u32);
    emu.lw_no_count(14usize, 2usize, 140u32, 2187956u32)?;
    emu.adr_no_count(28usize, 28usize, 14usize, 2187960u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2187964u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2187968u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2187972u32);
    emu.adr_no_count(30usize, 12usize, 10usize, 2187976u32);
    emu.sri_no_count(10usize, 30usize, 6u32, 2187980u32);
    emu.sli_no_count(12usize, 30usize, 26u32, 2187984u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2187988u32);
    emu.sri_no_count(12usize, 30usize, 11u32, 2187992u32);
    emu.sli_no_count(13usize, 30usize, 21u32, 2187996u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2188000u32);
    emu.sri_no_count(13usize, 30usize, 25u32, 2188004u32);
    emu.sli_no_count(28usize, 30usize, 7u32, 2188008u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2188012u32);
    emu.lw_no_count(28usize, 2usize, 484u32, 2188016u32)?;
    emu.adr_no_count(28usize, 28usize, 18usize, 2188020u32);
    emu.xrr_no_count(29usize, 11usize, 18usize, 2188024u32);
    emu.anr_no_count(29usize, 30usize, 29usize, 2188028u32);
    emu.xrr_no_count(29usize, 29usize, 18usize, 2188032u32);
    emu.sri_no_count(31usize, 14usize, 2u32, 2188036u32);
    emu.sli_no_count(9usize, 14usize, 30u32, 2188040u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2188044u32);
    emu.sri_no_count(9usize, 14usize, 13u32, 2188048u32);
    emu.sli_no_count(18usize, 14usize, 19u32, 2188052u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2188056u32);
    emu.sri_no_count(18usize, 14usize, 22u32, 2188060u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2188064u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2188068u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2188072u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2188076u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2188080u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2188084u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2188088u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2188092u32);
    emu.xrr_no_count(12usize, 31usize, 9usize, 2188096u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2188100u32);
    emu.xrr_no_count(12usize, 12usize, 18usize, 2188104u32);
    emu.lw_no_count(13usize, 2usize, 136u32, 2188108u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2188112u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2188116u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2188120u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2188124u32);
    emu.adr_no_count(18usize, 17usize, 6usize, 2188128u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2188132u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2188136u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2188140u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2188144u32);
    emu.sli_no_count(17usize, 18usize, 21u32, 2188148u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2188152u32);
    emu.sri_no_count(17usize, 18usize, 25u32, 2188156u32);
    emu.sli_no_count(6usize, 18usize, 7u32, 2188160u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2188164u32);
    emu.lw_no_count(29usize, 2usize, 464u32, 2188168u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2188172u32);
    emu.xrr_no_count(6usize, 30usize, 11usize, 2188176u32);
    emu.anr_no_count(6usize, 18usize, 6usize, 2188180u32);
    emu.xrr_no_count(11usize, 6usize, 11usize, 2188184u32);
    emu.sri_no_count(6usize, 10usize, 2u32, 2188188u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2188192u32);
    emu.orr_no_count(6usize, 31usize, 6usize, 2188196u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2188200u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2188204u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2188208u32);
    emu.sri_no_count(9usize, 10usize, 22u32, 2188212u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2188216u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2188220u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2188224u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2188228u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2188232u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2188236u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2188240u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2188244u32);
    emu.xrr_no_count(13usize, 6usize, 31usize, 2188248u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2188252u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2188256u32);
    emu.lw_no_count(17usize, 2usize, 132u32, 2188260u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2188264u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2188268u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2188272u32);
    emu.adr_no_count(6usize, 13usize, 11usize, 2188276u32);
    emu.adr_no_count(11usize, 11usize, 8usize, 2188280u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2188284u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2188288u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2188292u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2188296u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2188300u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2188304u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2188308u32);
    emu.sli_no_count(28usize, 11usize, 7u32, 2188312u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2188316u32);
    emu.lw_no_count(28usize, 2usize, 480u32, 2188320u32)?;
    emu.adr_no_count(28usize, 28usize, 30usize, 2188324u32);
    emu.xrr_no_count(31usize, 18usize, 30usize, 2188328u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2188332u32);
    emu.xrr_no_count(30usize, 31usize, 30usize, 2188336u32);
    emu.sri_no_count(31usize, 6usize, 2u32, 2188340u32);
    emu.sli_no_count(8usize, 6usize, 30u32, 2188344u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2188348u32);
    emu.sri_no_count(8usize, 6usize, 13u32, 2188352u32);
    emu.sli_no_count(9usize, 6usize, 19u32, 2188356u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2188360u32);
    emu.sri_no_count(9usize, 6usize, 22u32, 2188364u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2188368u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2188372u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2188376u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2188380u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2188384u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2188388u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2188392u32);
    emu.adr_no_count(29usize, 29usize, 30usize, 2188396u32);
    emu.xrr_no_count(13usize, 31usize, 8usize, 2188400u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2188404u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2188408u32);
    emu.lw_no_count(17usize, 2usize, 128u32, 2188412u32)?;
    emu.adr_no_count(29usize, 29usize, 17usize, 2188416u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2188420u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2188424u32);
    emu.adr_no_count(8usize, 13usize, 12usize, 2188428u32);
    emu.adr_no_count(30usize, 12usize, 7usize, 2188432u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2188436u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2188440u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2188444u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2188448u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2188452u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2188456u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2188460u32);
    emu.sli_no_count(7usize, 30usize, 7u32, 2188464u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2188468u32);
    emu.lw_no_count(29usize, 2usize, 388u32, 2188472u32)?;
    emu.adr_no_count(29usize, 29usize, 18usize, 2188476u32);
    emu.xrr_no_count(7usize, 11usize, 18usize, 2188480u32);
    emu.anr_no_count(7usize, 30usize, 7usize, 2188484u32);
    emu.xrr_no_count(7usize, 7usize, 18usize, 2188488u32);
    emu.sri_no_count(31usize, 8usize, 2u32, 2188492u32);
    emu.sli_no_count(9usize, 8usize, 30u32, 2188496u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2188500u32);
    emu.sri_no_count(9usize, 8usize, 13u32, 2188504u32);
    emu.sli_no_count(18usize, 8usize, 19u32, 2188508u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2188512u32);
    emu.sri_no_count(18usize, 8usize, 22u32, 2188516u32);
    emu.sli_no_count(19usize, 8usize, 10u32, 2188520u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2188524u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2188528u32);
    emu.anr_no_count(19usize, 8usize, 19usize, 2188532u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2188536u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2188540u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2188544u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2188548u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2188552u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2188556u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2188560u32);
    emu.lw_no_count(17usize, 2usize, 124u32, 2188564u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2188568u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2188572u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2188576u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2188580u32);
    emu.adr_no_count(18usize, 12usize, 14usize, 2188584u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2188588u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2188592u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2188596u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2188600u32);
    emu.sli_no_count(14usize, 18usize, 21u32, 2188604u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2188608u32);
    emu.sri_no_count(14usize, 18usize, 25u32, 2188612u32);
    emu.sli_no_count(17usize, 18usize, 7u32, 2188616u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2188620u32);
    emu.lw_no_count(17usize, 2usize, 448u32, 2188624u32)?;
    emu.adr_no_count(17usize, 17usize, 11usize, 2188628u32);
    emu.xrr_no_count(28usize, 30usize, 11usize, 2188632u32);
    emu.anr_no_count(28usize, 18usize, 28usize, 2188636u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2188640u32);
    emu.sri_no_count(28usize, 7usize, 2u32, 2188644u32);
    emu.sli_no_count(31usize, 7usize, 30u32, 2188648u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2188652u32);
    emu.sri_no_count(31usize, 7usize, 13u32, 2188656u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2188660u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2188664u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2188668u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2188672u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2188676u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2188680u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2188684u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2188688u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2188692u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2188696u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2188700u32);
    emu.xrr_no_count(13usize, 28usize, 31usize, 2188704u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2188708u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2188712u32);
    emu.lw_no_count(14usize, 2usize, 120u32, 2188716u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2188720u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2188724u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2188728u32);
    emu.adr_no_count(14usize, 13usize, 11usize, 2188732u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2188736u32);
    emu.sri_no_count(10usize, 11usize, 6u32, 2188740u32);
    emu.sli_no_count(12usize, 11usize, 26u32, 2188744u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2188748u32);
    emu.sri_no_count(12usize, 11usize, 11u32, 2188752u32);
    emu.sli_no_count(13usize, 11usize, 21u32, 2188756u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2188760u32);
    emu.sri_no_count(13usize, 11usize, 25u32, 2188764u32);
    emu.sli_no_count(28usize, 11usize, 7u32, 2188768u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2188772u32);
    emu.lw_no_count(28usize, 2usize, 452u32, 2188776u32)?;
    emu.adr_no_count(28usize, 28usize, 30usize, 2188780u32);
    emu.xrr_no_count(29usize, 18usize, 30usize, 2188784u32);
    emu.anr_no_count(29usize, 11usize, 29usize, 2188788u32);
    emu.xrr_no_count(29usize, 29usize, 30usize, 2188792u32);
    emu.sri_no_count(30usize, 14usize, 2u32, 2188796u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2188800u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2188804u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2188808u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2188812u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2188816u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2188820u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2188824u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2188828u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2188832u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2188836u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2188840u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2188844u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2188848u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2188852u32);
    emu.xrr_no_count(12usize, 30usize, 31usize, 2188856u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2188860u32);
    emu.xrr_no_count(12usize, 12usize, 9usize, 2188864u32);
    emu.lw_no_count(13usize, 2usize, 116u32, 2188868u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2188872u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2188876u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2188880u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2188884u32);
    emu.adr_no_count(30usize, 17usize, 6usize, 2188888u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2188892u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2188896u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2188900u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2188904u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2188908u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2188912u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2188916u32);
    emu.sli_no_count(6usize, 30usize, 7u32, 2188920u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2188924u32);
    emu.lw_no_count(29usize, 2usize, 376u32, 2188928u32)?;
    emu.adr_no_count(29usize, 29usize, 18usize, 2188932u32);
    emu.xrr_no_count(6usize, 11usize, 18usize, 2188936u32);
    emu.anr_no_count(6usize, 30usize, 6usize, 2188940u32);
    emu.xrr_no_count(6usize, 6usize, 18usize, 2188944u32);
    emu.sri_no_count(31usize, 10usize, 2u32, 2188948u32);
    emu.sli_no_count(9usize, 10usize, 30u32, 2188952u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2188956u32);
    emu.sri_no_count(9usize, 10usize, 13u32, 2188960u32);
    emu.sli_no_count(18usize, 10usize, 19u32, 2188964u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2188968u32);
    emu.sri_no_count(18usize, 10usize, 22u32, 2188972u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2188976u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2188980u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2188984u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2188988u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2188992u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2188996u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2189000u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2189004u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2189008u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2189012u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2189016u32);
    emu.lw_no_count(17usize, 2usize, 112u32, 2189020u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2189024u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2189028u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2189032u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2189036u32);
    emu.adr_no_count(8usize, 12usize, 8usize, 2189040u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2189044u32);
    emu.sli_no_count(13usize, 8usize, 26u32, 2189048u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2189052u32);
    emu.sri_no_count(13usize, 8usize, 11u32, 2189056u32);
    emu.sli_no_count(17usize, 8usize, 21u32, 2189060u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2189064u32);
    emu.sri_no_count(17usize, 8usize, 25u32, 2189068u32);
    emu.sli_no_count(28usize, 8usize, 7u32, 2189072u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2189076u32);
    emu.lw_no_count(31usize, 2usize, 384u32, 2189080u32)?;
    emu.adr_no_count(31usize, 31usize, 11usize, 2189084u32);
    emu.xrr_no_count(28usize, 30usize, 11usize, 2189088u32);
    emu.anr_no_count(28usize, 8usize, 28usize, 2189092u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2189096u32);
    emu.sri_no_count(28usize, 6usize, 2u32, 2189100u32);
    emu.sli_no_count(9usize, 6usize, 30u32, 2189104u32);
    emu.orr_no_count(28usize, 9usize, 28usize, 2189108u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2189112u32);
    emu.sli_no_count(18usize, 6usize, 19u32, 2189116u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2189120u32);
    emu.sri_no_count(18usize, 6usize, 22u32, 2189124u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2189128u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2189132u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2189136u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2189140u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2189144u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2189148u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2189152u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2189156u32);
    emu.xrr_no_count(13usize, 28usize, 9usize, 2189160u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2189164u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2189168u32);
    emu.lw_no_count(17usize, 2usize, 108u32, 2189172u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2189176u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2189180u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2189184u32);
    emu.adr_no_count(28usize, 13usize, 11usize, 2189188u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2189192u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2189196u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2189200u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2189204u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2189208u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2189212u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2189216u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2189220u32);
    emu.sli_no_count(7usize, 11usize, 7u32, 2189224u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2189228u32);
    emu.lw_no_count(29usize, 2usize, 488u32, 2189232u32)?;
    emu.adr_no_count(29usize, 29usize, 30usize, 2189236u32);
    emu.xrr_no_count(7usize, 8usize, 30usize, 2189240u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2189244u32);
    emu.xrr_no_count(7usize, 7usize, 30usize, 2189248u32);
    emu.sri_no_count(30usize, 28usize, 2u32, 2189252u32);
    emu.sli_no_count(9usize, 28usize, 30u32, 2189256u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2189260u32);
    emu.sri_no_count(9usize, 28usize, 13u32, 2189264u32);
    emu.sli_no_count(18usize, 28usize, 19u32, 2189268u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2189272u32);
    emu.sri_no_count(18usize, 28usize, 22u32, 2189276u32);
    emu.sli_no_count(19usize, 28usize, 10u32, 2189280u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2189284u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2189288u32);
    emu.anr_no_count(19usize, 28usize, 19usize, 2189292u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2189296u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2189300u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2189304u32);
    emu.adr_no_count(7usize, 31usize, 7usize, 2189308u32);
    emu.xrr_no_count(13usize, 30usize, 9usize, 2189312u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2189316u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2189320u32);
    emu.lw_no_count(17usize, 2usize, 104u32, 2189324u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2189328u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2189332u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2189336u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2189340u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2189344u32);
    emu.sri_no_count(12usize, 14usize, 6u32, 2189348u32);
    emu.sli_no_count(13usize, 14usize, 26u32, 2189352u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2189356u32);
    emu.sri_no_count(13usize, 14usize, 11u32, 2189360u32);
    emu.sli_no_count(17usize, 14usize, 21u32, 2189364u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2189368u32);
    emu.sri_no_count(17usize, 14usize, 25u32, 2189372u32);
    emu.sli_no_count(30usize, 14usize, 7u32, 2189376u32);
    emu.orr_no_count(17usize, 30usize, 17usize, 2189380u32);
    emu.lw_no_count(30usize, 2usize, 412u32, 2189384u32)?;
    emu.adr_no_count(30usize, 30usize, 8usize, 2189388u32);
    emu.xrr_no_count(16usize, 11usize, 8usize, 2189392u32);
    emu.anr_no_count(16usize, 14usize, 16usize, 2189396u32);
    emu.xrr_no_count(16usize, 16usize, 8usize, 2189400u32);
    emu.sri_no_count(31usize, 7usize, 2u32, 2189404u32);
    emu.sli_no_count(8usize, 7usize, 30u32, 2189408u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2189412u32);
    emu.sri_no_count(8usize, 7usize, 13u32, 2189416u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2189420u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2189424u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2189428u32);
    emu.sli_no_count(18usize, 7usize, 10u32, 2189432u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2189436u32);
    emu.xrr_no_count(18usize, 28usize, 6usize, 2189440u32);
    emu.anr_no_count(18usize, 7usize, 18usize, 2189444u32);
    emu.anr_no_count(19usize, 28usize, 6usize, 2189448u32);
    emu.xrr_no_count(18usize, 18usize, 19usize, 2189452u32);
    emu.lw_no_count(19usize, 2usize, 428u32, 2189456u32)?;
    emu.xrr_no_count(12usize, 12usize, 13usize, 2189460u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2189464u32);
    emu.xrr_no_count(13usize, 31usize, 8usize, 2189468u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2189472u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2189476u32);
    emu.lw_no_count(17usize, 2usize, 100u32, 2189480u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2189484u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2189488u32);
    emu.adr_no_count(13usize, 13usize, 18usize, 2189492u32);
    emu.adr_no_count(16usize, 13usize, 12usize, 2189496u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2189500u32);
    emu.sri_no_count(12usize, 10usize, 6u32, 2189504u32);
    emu.sli_no_count(13usize, 10usize, 26u32, 2189508u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2189512u32);
    emu.sri_no_count(13usize, 10usize, 11u32, 2189516u32);
    emu.sli_no_count(17usize, 10usize, 21u32, 2189520u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2189524u32);
    emu.sri_no_count(17usize, 10usize, 25u32, 2189528u32);
    emu.sli_no_count(29usize, 10usize, 7u32, 2189532u32);
    emu.orr_no_count(17usize, 29usize, 17usize, 2189536u32);
    emu.lw_no_count(29usize, 2usize, 380u32, 2189540u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2189544u32);
    emu.xrr_no_count(5usize, 14usize, 11usize, 2189548u32);
    emu.anr_no_count(5usize, 10usize, 5usize, 2189552u32);
    emu.xrr_no_count(11usize, 5usize, 11usize, 2189556u32);
    emu.sri_no_count(5usize, 16usize, 2u32, 2189560u32);
    emu.sli_no_count(31usize, 16usize, 30u32, 2189564u32);
    emu.orr_no_count(5usize, 31usize, 5usize, 2189568u32);
    emu.sri_no_count(31usize, 16usize, 13u32, 2189572u32);
    emu.sli_no_count(8usize, 16usize, 19u32, 2189576u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2189580u32);
    emu.sri_no_count(8usize, 16usize, 22u32, 2189584u32);
    emu.sli_no_count(9usize, 16usize, 10u32, 2189588u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2189592u32);
    emu.xrr_no_count(9usize, 7usize, 28usize, 2189596u32);
    emu.anr_no_count(9usize, 16usize, 9usize, 2189600u32);
    emu.anr_no_count(18usize, 7usize, 28usize, 2189604u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2189608u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2189612u32);
    emu.adr_no_count(11usize, 30usize, 11usize, 2189616u32);
    emu.xrr_no_count(13usize, 5usize, 31usize, 2189620u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2189624u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2189628u32);
    emu.lw_no_count(17usize, 2usize, 96u32, 2189632u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2189636u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2189640u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2189644u32);
    emu.adr_no_count(5usize, 13usize, 11usize, 2189648u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2189652u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2189656u32);
    emu.sli_no_count(12usize, 6usize, 26u32, 2189660u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2189664u32);
    emu.sri_no_count(12usize, 6usize, 11u32, 2189668u32);
    emu.sli_no_count(13usize, 6usize, 21u32, 2189672u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2189676u32);
    emu.sri_no_count(13usize, 6usize, 25u32, 2189680u32);
    emu.sli_no_count(17usize, 6usize, 7u32, 2189684u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2189688u32);
    emu.adr_no_count(17usize, 27usize, 14usize, 2189692u32);
    emu.xrr_no_count(15usize, 10usize, 14usize, 2189696u32);
    emu.anr_no_count(15usize, 6usize, 15usize, 2189700u32);
    emu.xrr_no_count(14usize, 15usize, 14usize, 2189704u32);
    emu.sri_no_count(15usize, 5usize, 2u32, 2189708u32);
    emu.sli_no_count(30usize, 5usize, 30u32, 2189712u32);
    emu.orr_no_count(15usize, 30usize, 15usize, 2189716u32);
    emu.sri_no_count(30usize, 5usize, 13u32, 2189720u32);
    emu.sli_no_count(31usize, 5usize, 19u32, 2189724u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2189728u32);
    emu.sri_no_count(31usize, 5usize, 22u32, 2189732u32);
    emu.sli_no_count(8usize, 5usize, 10u32, 2189736u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2189740u32);
    emu.xrr_no_count(8usize, 16usize, 7usize, 2189744u32);
    emu.anr_no_count(8usize, 5usize, 8usize, 2189748u32);
    emu.anr_no_count(9usize, 16usize, 7usize, 2189752u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2189756u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2189760u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2189764u32);
    emu.xrr_no_count(12usize, 15usize, 30usize, 2189768u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2189772u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2189776u32);
    emu.lw_no_count(13usize, 2usize, 92u32, 2189780u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2189784u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2189788u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2189792u32);
    emu.adr_no_count(15usize, 12usize, 11usize, 2189796u32);
    emu.adr_no_count(28usize, 11usize, 28usize, 2189800u32);
    emu.sri_no_count(11usize, 28usize, 6u32, 2189804u32);
    emu.sli_no_count(12usize, 28usize, 26u32, 2189808u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2189812u32);
    emu.sri_no_count(12usize, 28usize, 11u32, 2189816u32);
    emu.sli_no_count(13usize, 28usize, 21u32, 2189820u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2189824u32);
    emu.sri_no_count(13usize, 28usize, 25u32, 2189828u32);
    emu.sli_no_count(14usize, 28usize, 7u32, 2189832u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2189836u32);
    emu.adr_no_count(29usize, 22usize, 10usize, 2189840u32);
    emu.xrr_no_count(14usize, 6usize, 10usize, 2189844u32);
    emu.anr_no_count(14usize, 28usize, 14usize, 2189848u32);
    emu.xrr_no_count(10usize, 14usize, 10usize, 2189852u32);
    emu.sri_no_count(14usize, 15usize, 2u32, 2189856u32);
    emu.sli_no_count(30usize, 15usize, 30u32, 2189860u32);
    emu.orr_no_count(14usize, 30usize, 14usize, 2189864u32);
    emu.sri_no_count(30usize, 15usize, 13u32, 2189868u32);
    emu.sli_no_count(31usize, 15usize, 19u32, 2189872u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2189876u32);
    emu.sri_no_count(31usize, 15usize, 22u32, 2189880u32);
    emu.sli_no_count(8usize, 15usize, 10u32, 2189884u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2189888u32);
    emu.xrr_no_count(8usize, 5usize, 16usize, 2189892u32);
    emu.anr_no_count(8usize, 15usize, 8usize, 2189896u32);
    emu.anr_no_count(9usize, 5usize, 16usize, 2189900u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2189904u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2189908u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2189912u32);
    emu.xrr_no_count(12usize, 14usize, 30usize, 2189916u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2189920u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2189924u32);
    emu.lw_no_count(13usize, 2usize, 88u32, 2189928u32)?;
    emu.adr_no_count(10usize, 10usize, 13usize, 2189932u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2189936u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2189940u32);
    emu.adr_no_count(14usize, 12usize, 10usize, 2189944u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2189948u32);
    emu.sri_no_count(10usize, 7usize, 6u32, 2189952u32);
    emu.sli_no_count(11usize, 7usize, 26u32, 2189956u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2189960u32);
    emu.sri_no_count(11usize, 7usize, 11u32, 2189964u32);
    emu.sli_no_count(12usize, 7usize, 21u32, 2189968u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2189972u32);
    emu.sri_no_count(12usize, 7usize, 25u32, 2189976u32);
    emu.sli_no_count(13usize, 7usize, 7u32, 2189980u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2189984u32);
    emu.adr_no_count(20usize, 20usize, 6usize, 2189988u32);
    emu.xrr_no_count(13usize, 28usize, 6usize, 2189992u32);
    emu.anr_no_count(13usize, 7usize, 13usize, 2189996u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2190000u32);
    emu.sri_no_count(17usize, 14usize, 2u32, 2190004u32);
    emu.sli_no_count(6usize, 14usize, 30u32, 2190008u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2190012u32);
    emu.sri_no_count(6usize, 14usize, 13u32, 2190016u32);
    emu.sli_no_count(30usize, 14usize, 19u32, 2190020u32);
    emu.orr_no_count(6usize, 30usize, 6usize, 2190024u32);
    emu.sri_no_count(30usize, 14usize, 22u32, 2190028u32);
    emu.sli_no_count(31usize, 14usize, 10u32, 2190032u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2190036u32);
    emu.xrr_no_count(31usize, 15usize, 5usize, 2190040u32);
    emu.anr_no_count(31usize, 14usize, 31usize, 2190044u32);
    emu.anr_no_count(8usize, 15usize, 5usize, 2190048u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2190052u32);
    emu.xrr_no_count(10usize, 10usize, 11usize, 2190056u32);
    emu.adr_no_count(13usize, 29usize, 13usize, 2190060u32);
    emu.xrr_no_count(11usize, 17usize, 6usize, 2190064u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2190068u32);
    emu.xrr_no_count(11usize, 11usize, 30usize, 2190072u32);
    emu.lw_no_count(12usize, 2usize, 84u32, 2190076u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2190080u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2190084u32);
    emu.adr_no_count(10usize, 11usize, 31usize, 2190088u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2190092u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2190096u32);
    emu.sri_no_count(11usize, 16usize, 6u32, 2190100u32);
    emu.sli_no_count(12usize, 16usize, 26u32, 2190104u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2190108u32);
    emu.sri_no_count(12usize, 16usize, 11u32, 2190112u32);
    emu.sli_no_count(13usize, 16usize, 21u32, 2190116u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2190120u32);
    emu.sri_no_count(13usize, 16usize, 25u32, 2190124u32);
    emu.sli_no_count(17usize, 16usize, 7u32, 2190128u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2190132u32);
    emu.adr_no_count(22usize, 25usize, 28usize, 2190136u32);
    emu.xrr_no_count(17usize, 7usize, 28usize, 2190140u32);
    emu.anr_no_count(17usize, 16usize, 17usize, 2190144u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2190148u32);
    emu.sri_no_count(6usize, 10usize, 2u32, 2190152u32);
    emu.sli_no_count(28usize, 10usize, 30u32, 2190156u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2190160u32);
    emu.sri_no_count(28usize, 10usize, 13u32, 2190164u32);
    emu.sli_no_count(29usize, 10usize, 19u32, 2190168u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2190172u32);
    emu.sri_no_count(29usize, 10usize, 22u32, 2190176u32);
    emu.sli_no_count(30usize, 10usize, 10u32, 2190180u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2190184u32);
    emu.xrr_no_count(30usize, 14usize, 15usize, 2190188u32);
    emu.anr_no_count(30usize, 10usize, 30usize, 2190192u32);
    emu.anr_no_count(31usize, 14usize, 15usize, 2190196u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2190200u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2190204u32);
    emu.adr_no_count(17usize, 20usize, 17usize, 2190208u32);
    emu.lw_no_count(21usize, 2usize, 512u32, 2190212u32)?;
    emu.xrr_no_count(12usize, 6usize, 28usize, 2190216u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2190220u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2190224u32);
    emu.lw_no_count(13usize, 2usize, 80u32, 2190228u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2190232u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2190236u32);
    emu.adr_no_count(11usize, 12usize, 30usize, 2190240u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2190244u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2190248u32);
    emu.sri_no_count(12usize, 5usize, 6u32, 2190252u32);
    emu.sli_no_count(13usize, 5usize, 26u32, 2190256u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2190260u32);
    emu.sri_no_count(13usize, 5usize, 11u32, 2190264u32);
    emu.sli_no_count(17usize, 5usize, 21u32, 2190268u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2190272u32);
    emu.sri_no_count(17usize, 5usize, 25u32, 2190276u32);
    emu.sli_no_count(6usize, 5usize, 7u32, 2190280u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2190284u32);
    emu.lw_no_count(6usize, 2usize, 340u32, 2190288u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2190292u32);
    emu.xrr_no_count(28usize, 16usize, 7usize, 2190296u32);
    emu.anr_no_count(28usize, 5usize, 28usize, 2190300u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2190304u32);
    emu.sri_no_count(28usize, 11usize, 2u32, 2190308u32);
    emu.sli_no_count(29usize, 11usize, 30u32, 2190312u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2190316u32);
    emu.sri_no_count(29usize, 11usize, 13u32, 2190320u32);
    emu.sli_no_count(30usize, 11usize, 19u32, 2190324u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2190328u32);
    emu.sri_no_count(30usize, 11usize, 22u32, 2190332u32);
    emu.sli_no_count(31usize, 11usize, 10u32, 2190336u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2190340u32);
    emu.xrr_no_count(31usize, 10usize, 14usize, 2190344u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2190348u32);
    emu.anr_no_count(8usize, 10usize, 14usize, 2190352u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2190356u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2190360u32);
    emu.adr_no_count(7usize, 22usize, 7usize, 2190364u32);
    emu.lw_no_count(22usize, 2usize, 520u32, 2190368u32)?;
    emu.xrr_no_count(13usize, 28usize, 29usize, 2190372u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2190376u32);
    emu.xrr_no_count(13usize, 13usize, 30usize, 2190380u32);
    emu.lw_no_count(17usize, 2usize, 76u32, 2190384u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2190388u32);
    emu.adr_no_count(7usize, 7usize, 12usize, 2190392u32);
    emu.adr_no_count(12usize, 13usize, 31usize, 2190396u32);
    emu.adr_no_count(12usize, 12usize, 7usize, 2190400u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2190404u32);
    emu.sri_no_count(13usize, 15usize, 6u32, 2190408u32);
    emu.sli_no_count(17usize, 15usize, 26u32, 2190412u32);
    emu.sri_no_count(7usize, 15usize, 11u32, 2190416u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2190420u32);
    emu.sli_no_count(17usize, 15usize, 21u32, 2190424u32);
    emu.orr_no_count(17usize, 17usize, 7usize, 2190428u32);
    emu.sri_no_count(7usize, 15usize, 25u32, 2190432u32);
    emu.sli_no_count(28usize, 15usize, 7u32, 2190436u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2190440u32);
    emu.lw_no_count(28usize, 2usize, 420u32, 2190444u32)?;
    emu.adr_no_count(28usize, 28usize, 16usize, 2190448u32);
    emu.xrr_no_count(29usize, 5usize, 16usize, 2190452u32);
    emu.anr_no_count(29usize, 15usize, 29usize, 2190456u32);
    emu.xrr_no_count(16usize, 29usize, 16usize, 2190460u32);
    emu.sri_no_count(29usize, 12usize, 2u32, 2190464u32);
    emu.sli_no_count(30usize, 12usize, 30u32, 2190468u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2190472u32);
    emu.sri_no_count(30usize, 12usize, 13u32, 2190476u32);
    emu.sli_no_count(31usize, 12usize, 19u32, 2190480u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2190484u32);
    emu.sri_no_count(31usize, 12usize, 22u32, 2190488u32);
    emu.sli_no_count(8usize, 12usize, 10u32, 2190492u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2190496u32);
    emu.xrr_no_count(8usize, 11usize, 10usize, 2190500u32);
    emu.anr_no_count(8usize, 12usize, 8usize, 2190504u32);
    emu.anr_no_count(9usize, 11usize, 10usize, 2190508u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2190512u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2190516u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2190520u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2190524u32);
    emu.xrr_no_count(13usize, 13usize, 7usize, 2190528u32);
    emu.lw_no_count(25usize, 2usize, 344u32, 2190532u32)?;
    emu.adr_no_count(25usize, 25usize, 5usize, 2190536u32);
    emu.lw_no_count(6usize, 2usize, 72u32, 2190540u32)?;
    emu.adr_no_count(16usize, 16usize, 6usize, 2190544u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2190548u32);
    emu.adr_no_count(16usize, 16usize, 13usize, 2190552u32);
    emu.adr_no_count(13usize, 17usize, 8usize, 2190556u32);
    emu.adr_no_count(13usize, 13usize, 16usize, 2190560u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2190564u32);
    emu.sri_no_count(14usize, 16usize, 6u32, 2190568u32);
    emu.sli_no_count(17usize, 16usize, 26u32, 2190572u32);
    emu.sri_no_count(6usize, 16usize, 11u32, 2190576u32);
    emu.sli_no_count(7usize, 16usize, 21u32, 2190580u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2190584u32);
    emu.sri_no_count(17usize, 16usize, 25u32, 2190588u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2190592u32);
    emu.sli_no_count(7usize, 16usize, 7u32, 2190596u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2190600u32);
    emu.xrr_no_count(7usize, 15usize, 5usize, 2190604u32);
    emu.anr_no_count(7usize, 16usize, 7usize, 2190608u32);
    emu.xrr_no_count(5usize, 7usize, 5usize, 2190612u32);
    emu.sri_no_count(7usize, 13usize, 2u32, 2190616u32);
    emu.sli_no_count(29usize, 13usize, 30u32, 2190620u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2190624u32);
    emu.sri_no_count(29usize, 13usize, 13u32, 2190628u32);
    emu.sli_no_count(30usize, 13usize, 19u32, 2190632u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2190636u32);
    emu.sri_no_count(30usize, 13usize, 22u32, 2190640u32);
    emu.sli_no_count(31usize, 13usize, 10u32, 2190644u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2190648u32);
    emu.xrr_no_count(31usize, 12usize, 11usize, 2190652u32);
    emu.anr_no_count(31usize, 13usize, 31usize, 2190656u32);
    emu.anr_no_count(8usize, 12usize, 11usize, 2190660u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2190664u32);
    emu.xrr_no_count(14usize, 14usize, 6usize, 2190668u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2190672u32);
    emu.xrr_no_count(6usize, 7usize, 29usize, 2190676u32);
    emu.lw_no_count(26usize, 2usize, 348u32, 2190680u32)?;
    emu.adr_no_count(26usize, 26usize, 15usize, 2190684u32);
    emu.xrr_no_count(14usize, 14usize, 17usize, 2190688u32);
    emu.xrr_no_count(7usize, 16usize, 15usize, 2190692u32);
    emu.lw_no_count(17usize, 2usize, 68u32, 2190696u32)?;
    emu.adr_no_count(5usize, 5usize, 17usize, 2190700u32);
    emu.xrr_no_count(17usize, 6usize, 30usize, 2190704u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2190708u32);
    emu.adr_no_count(14usize, 17usize, 31usize, 2190712u32);
    emu.adr_no_count(14usize, 14usize, 5usize, 2190716u32);
    emu.adr_no_count(17usize, 5usize, 10usize, 2190720u32);
    emu.sri_no_count(10usize, 17usize, 6u32, 2190724u32);
    emu.sli_no_count(5usize, 17usize, 26u32, 2190728u32);
    emu.sri_no_count(6usize, 17usize, 11u32, 2190732u32);
    emu.sli_no_count(28usize, 17usize, 21u32, 2190736u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2190740u32);
    emu.sri_no_count(5usize, 17usize, 25u32, 2190744u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2190748u32);
    emu.sli_no_count(28usize, 17usize, 7u32, 2190752u32);
    emu.anr_no_count(7usize, 17usize, 7usize, 2190756u32);
    emu.orr_no_count(5usize, 28usize, 5usize, 2190760u32);
    emu.sri_no_count(28usize, 14usize, 2u32, 2190764u32);
    emu.xrr_no_count(15usize, 7usize, 15usize, 2190768u32);
    emu.sli_no_count(7usize, 14usize, 30u32, 2190772u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2190776u32);
    emu.sri_no_count(28usize, 14usize, 13u32, 2190780u32);
    emu.sli_no_count(29usize, 14usize, 19u32, 2190784u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2190788u32);
    emu.sri_no_count(29usize, 14usize, 22u32, 2190792u32);
    emu.sli_no_count(30usize, 14usize, 10u32, 2190796u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2190800u32);
    emu.xrr_no_count(30usize, 13usize, 12usize, 2190804u32);
    emu.anr_no_count(30usize, 14usize, 30usize, 2190808u32);
    emu.anr_no_count(31usize, 13usize, 12usize, 2190812u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2190816u32);
    emu.xrr_no_count(10usize, 10usize, 6usize, 2190820u32);
    emu.adr_no_count(15usize, 25usize, 15usize, 2190824u32);
    emu.lw_no_count(27usize, 2usize, 336u32, 2190828u32)?;
    emu.adr_no_count(27usize, 27usize, 16usize, 2190832u32);
    emu.xrr_no_count(6usize, 7usize, 28usize, 2190836u32);
    emu.xrr_no_count(7usize, 17usize, 16usize, 2190840u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2190844u32);
    emu.lw_no_count(5usize, 2usize, 64u32, 2190848u32)?;
    emu.adr_no_count(15usize, 15usize, 5usize, 2190852u32);
    emu.xrr_no_count(5usize, 6usize, 29usize, 2190856u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2190860u32);
    emu.adr_no_count(10usize, 5usize, 30usize, 2190864u32);
    emu.adr_no_count(10usize, 10usize, 15usize, 2190868u32);
    emu.adr_no_count(15usize, 15usize, 11usize, 2190872u32);
    emu.sri_no_count(11usize, 15usize, 6u32, 2190876u32);
    emu.sli_no_count(5usize, 15usize, 26u32, 2190880u32);
    emu.sri_no_count(6usize, 15usize, 11u32, 2190884u32);
    emu.sli_no_count(28usize, 15usize, 21u32, 2190888u32);
    emu.sri_no_count(29usize, 15usize, 25u32, 2190892u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2190896u32);
    emu.sli_no_count(5usize, 15usize, 7u32, 2190900u32);
    emu.anr_no_count(7usize, 15usize, 7usize, 2190904u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2190908u32);
    emu.sri_no_count(28usize, 10usize, 2u32, 2190912u32);
    emu.orr_no_count(29usize, 5usize, 29usize, 2190916u32);
    emu.sli_no_count(5usize, 10usize, 30u32, 2190920u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2190924u32);
    emu.sri_no_count(7usize, 10usize, 13u32, 2190928u32);
    emu.orr_no_count(28usize, 5usize, 28usize, 2190932u32);
    emu.sli_no_count(5usize, 10usize, 19u32, 2190936u32);
    emu.orr_no_count(7usize, 5usize, 7usize, 2190940u32);
    emu.sri_no_count(5usize, 10usize, 22u32, 2190944u32);
    emu.sli_no_count(30usize, 10usize, 10u32, 2190948u32);
    emu.orr_no_count(30usize, 30usize, 5usize, 2190952u32);
    emu.xrr_no_count(5usize, 14usize, 13usize, 2190956u32);
    emu.anr_no_count(5usize, 10usize, 5usize, 2190960u32);
    emu.anr_no_count(31usize, 14usize, 13usize, 2190964u32);
    emu.xrr_no_count(31usize, 5usize, 31usize, 2190968u32);
    emu.xrr_no_count(11usize, 11usize, 6usize, 2190972u32);
    emu.lw_no_count(5usize, 2usize, 352u32, 2190976u32)?;
    emu.adr_no_count(5usize, 5usize, 17usize, 2190980u32);
    emu.adr_no_count(16usize, 26usize, 16usize, 2190984u32);
    emu.lw_no_count(26usize, 2usize, 432u32, 2190988u32)?;
    emu.xrr_no_count(6usize, 15usize, 17usize, 2190992u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2190996u32);
    emu.xrr_no_count(11usize, 11usize, 29usize, 2191000u32);
    emu.lw_no_count(28usize, 2usize, 60u32, 2191004u32)?;
    emu.adr_no_count(16usize, 16usize, 28usize, 2191008u32);
    emu.xrr_no_count(7usize, 7usize, 30usize, 2191012u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2191016u32);
    emu.adr_no_count(11usize, 7usize, 31usize, 2191020u32);
    emu.adr_no_count(11usize, 11usize, 16usize, 2191024u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2191028u32);
    emu.sri_no_count(12usize, 16usize, 6u32, 2191032u32);
    emu.sli_no_count(7usize, 16usize, 26u32, 2191036u32);
    emu.sri_no_count(28usize, 16usize, 11u32, 2191040u32);
    emu.sli_no_count(29usize, 16usize, 21u32, 2191044u32);
    emu.sri_no_count(30usize, 16usize, 25u32, 2191048u32);
    emu.sli_no_count(31usize, 16usize, 7u32, 2191052u32);
    emu.anr_no_count(6usize, 16usize, 6usize, 2191056u32);
    emu.orr_no_count(12usize, 7usize, 12usize, 2191060u32);
    emu.sri_no_count(7usize, 11usize, 2u32, 2191064u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2191068u32);
    emu.sli_no_count(29usize, 11usize, 30u32, 2191072u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2191076u32);
    emu.sri_no_count(31usize, 11usize, 13u32, 2191080u32);
    emu.xrr_no_count(17usize, 6usize, 17usize, 2191084u32);
    emu.sli_no_count(6usize, 11usize, 19u32, 2191088u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2191092u32);
    emu.sri_no_count(29usize, 11usize, 22u32, 2191096u32);
    emu.orr_no_count(6usize, 6usize, 31usize, 2191100u32);
    emu.sli_no_count(31usize, 11usize, 10u32, 2191104u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2191108u32);
    emu.xrr_no_count(31usize, 10usize, 14usize, 2191112u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2191116u32);
    emu.anr_no_count(8usize, 10usize, 14usize, 2191120u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2191124u32);
    emu.lw_no_count(1usize, 2usize, 356u32, 2191128u32)?;
    emu.adr_no_count(1usize, 1usize, 15usize, 2191132u32);
    emu.xrr_no_count(12usize, 12usize, 28usize, 2191136u32);
    emu.anr_no_count(28usize, 11usize, 10usize, 2191140u32);
    emu.adr_no_count(17usize, 27usize, 17usize, 2191144u32);
    emu.xrr_no_count(8usize, 16usize, 15usize, 2191148u32);
    emu.xrr_no_count(6usize, 7usize, 6usize, 2191152u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2191156u32);
    emu.lw_no_count(7usize, 2usize, 56u32, 2191160u32)?;
    emu.adr_no_count(17usize, 17usize, 7usize, 2191164u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2191168u32);
    emu.adr_no_count(17usize, 17usize, 12usize, 2191172u32);
    emu.adr_no_count(12usize, 6usize, 31usize, 2191176u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2191180u32);
    emu.adr_no_count(17usize, 17usize, 13usize, 2191184u32);
    emu.sri_no_count(13usize, 17usize, 6u32, 2191188u32);
    emu.sli_no_count(6usize, 17usize, 26u32, 2191192u32);
    emu.sri_no_count(7usize, 17usize, 11u32, 2191196u32);
    emu.sli_no_count(29usize, 17usize, 21u32, 2191200u32);
    emu.sri_no_count(30usize, 17usize, 25u32, 2191204u32);
    emu.sli_no_count(31usize, 17usize, 7u32, 2191208u32);
    emu.anr_no_count(8usize, 17usize, 8usize, 2191212u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2191216u32);
    emu.sri_no_count(6usize, 12usize, 2u32, 2191220u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2191224u32);
    emu.sli_no_count(29usize, 12usize, 30u32, 2191228u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2191232u32);
    emu.sri_no_count(31usize, 12usize, 13u32, 2191236u32);
    emu.xrr_no_count(8usize, 8usize, 15usize, 2191240u32);
    emu.sli_no_count(15usize, 12usize, 19u32, 2191244u32);
    emu.orr_no_count(6usize, 29usize, 6usize, 2191248u32);
    emu.sri_no_count(29usize, 12usize, 22u32, 2191252u32);
    emu.orr_no_count(31usize, 15usize, 31usize, 2191256u32);
    emu.sli_no_count(15usize, 12usize, 10u32, 2191260u32);
    emu.orr_no_count(29usize, 15usize, 29usize, 2191264u32);
    emu.xrr_no_count(15usize, 11usize, 10usize, 2191268u32);
    emu.anr_no_count(15usize, 12usize, 15usize, 2191272u32);
    emu.xrr_no_count(28usize, 15usize, 28usize, 2191276u32);
    emu.lw_no_count(15usize, 2usize, 496u32, 2191280u32)?;
    emu.adr_no_count(15usize, 15usize, 16usize, 2191284u32);
    emu.xrr_no_count(13usize, 13usize, 7usize, 2191288u32);
    emu.xrr_no_count(7usize, 17usize, 16usize, 2191292u32);
    emu.adr_no_count(5usize, 5usize, 8usize, 2191296u32);
    emu.xrr_no_count(8usize, 12usize, 11usize, 2191300u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2191304u32);
    emu.xrr_no_count(13usize, 13usize, 30usize, 2191308u32);
    emu.lw_no_count(30usize, 2usize, 52u32, 2191312u32)?;
    emu.adr_no_count(5usize, 5usize, 30usize, 2191316u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2191320u32);
    emu.adr_no_count(5usize, 5usize, 13usize, 2191324u32);
    emu.adr_no_count(13usize, 6usize, 28usize, 2191328u32);
    emu.adr_no_count(13usize, 13usize, 5usize, 2191332u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2191336u32);
    emu.sri_no_count(14usize, 5usize, 6u32, 2191340u32);
    emu.sli_no_count(6usize, 5usize, 26u32, 2191344u32);
    emu.sri_no_count(28usize, 5usize, 11u32, 2191348u32);
    emu.sli_no_count(29usize, 5usize, 21u32, 2191352u32);
    emu.sri_no_count(30usize, 5usize, 25u32, 2191356u32);
    emu.sli_no_count(31usize, 5usize, 7u32, 2191360u32);
    emu.anr_no_count(7usize, 5usize, 7usize, 2191364u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2191368u32);
    emu.sri_no_count(6usize, 13usize, 2u32, 2191372u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2191376u32);
    emu.sli_no_count(29usize, 13usize, 30u32, 2191380u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2191384u32);
    emu.sri_no_count(31usize, 13usize, 13u32, 2191388u32);
    emu.xrr_no_count(7usize, 7usize, 16usize, 2191392u32);
    emu.sli_no_count(16usize, 13usize, 19u32, 2191396u32);
    emu.orr_no_count(6usize, 29usize, 6usize, 2191400u32);
    emu.sri_no_count(29usize, 13usize, 22u32, 2191404u32);
    emu.orr_no_count(31usize, 16usize, 31usize, 2191408u32);
    emu.sli_no_count(16usize, 13usize, 10u32, 2191412u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2191416u32);
    emu.anr_no_count(16usize, 12usize, 11usize, 2191420u32);
    emu.anr_no_count(8usize, 13usize, 8usize, 2191424u32);
    emu.xrr_no_count(8usize, 8usize, 16usize, 2191428u32);
    emu.lw_no_count(16usize, 2usize, 332u32, 2191432u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2191436u32);
    emu.xrr_no_count(14usize, 14usize, 28usize, 2191440u32);
    emu.anr_no_count(28usize, 13usize, 12usize, 2191444u32);
    emu.adr_no_count(7usize, 1usize, 7usize, 2191448u32);
    emu.lw_no_count(20usize, 2usize, 516u32, 2191452u32)?;
    emu.xrr_no_count(9usize, 5usize, 17usize, 2191456u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2191460u32);
    emu.xrr_no_count(14usize, 14usize, 30usize, 2191464u32);
    emu.lw_no_count(30usize, 2usize, 48u32, 2191468u32)?;
    emu.adr_no_count(7usize, 7usize, 30usize, 2191472u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2191476u32);
    emu.adr_no_count(7usize, 7usize, 14usize, 2191480u32);
    emu.adr_no_count(14usize, 6usize, 8usize, 2191484u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2191488u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2191492u32);
    emu.sri_no_count(6usize, 10usize, 6u32, 2191496u32);
    emu.sli_no_count(7usize, 10usize, 26u32, 2191500u32);
    emu.sri_no_count(29usize, 10usize, 11u32, 2191504u32);
    emu.sli_no_count(30usize, 10usize, 21u32, 2191508u32);
    emu.sri_no_count(31usize, 10usize, 25u32, 2191512u32);
    emu.sli_no_count(8usize, 10usize, 7u32, 2191516u32);
    emu.anr_no_count(9usize, 10usize, 9usize, 2191520u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2191524u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2191528u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2191532u32);
    emu.sli_no_count(30usize, 14usize, 30u32, 2191536u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2191540u32);
    emu.sri_no_count(8usize, 14usize, 13u32, 2191544u32);
    emu.xrr_no_count(17usize, 9usize, 17usize, 2191548u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2191552u32);
    emu.orr_no_count(30usize, 30usize, 7usize, 2191556u32);
    emu.sri_no_count(7usize, 14usize, 22u32, 2191560u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2191564u32);
    emu.sli_no_count(9usize, 14usize, 10u32, 2191568u32);
    emu.orr_no_count(9usize, 9usize, 7usize, 2191572u32);
    emu.xrr_no_count(7usize, 13usize, 12usize, 2191576u32);
    emu.anr_no_count(7usize, 14usize, 7usize, 2191580u32);
    emu.xrr_no_count(28usize, 7usize, 28usize, 2191584u32);
    emu.lw_no_count(7usize, 2usize, 440u32, 2191588u32)?;
    emu.adr_no_count(7usize, 7usize, 5usize, 2191592u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2191596u32);
    emu.xrr_no_count(29usize, 10usize, 5usize, 2191600u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2191604u32);
    emu.xrr_no_count(18usize, 14usize, 13usize, 2191608u32);
    emu.xrr_no_count(17usize, 30usize, 8usize, 2191612u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2191616u32);
    emu.lw_no_count(30usize, 2usize, 44u32, 2191620u32)?;
    emu.adr_no_count(15usize, 15usize, 30usize, 2191624u32);
    emu.xrr_no_count(17usize, 17usize, 9usize, 2191628u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2191632u32);
    emu.adr_no_count(15usize, 17usize, 28usize, 2191636u32);
    emu.adr_no_count(15usize, 15usize, 6usize, 2191640u32);
    emu.adr_no_count(17usize, 6usize, 11usize, 2191644u32);
    emu.sri_no_count(11usize, 17usize, 6u32, 2191648u32);
    emu.sli_no_count(6usize, 17usize, 26u32, 2191652u32);
    emu.sri_no_count(28usize, 17usize, 11u32, 2191656u32);
    emu.sli_no_count(30usize, 17usize, 21u32, 2191660u32);
    emu.sri_no_count(31usize, 17usize, 25u32, 2191664u32);
    emu.sli_no_count(8usize, 17usize, 7u32, 2191668u32);
    emu.anr_no_count(29usize, 17usize, 29usize, 2191672u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2191676u32);
    emu.sri_no_count(6usize, 15usize, 2u32, 2191680u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2191684u32);
    emu.sli_no_count(30usize, 15usize, 30u32, 2191688u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2191692u32);
    emu.sri_no_count(8usize, 15usize, 13u32, 2191696u32);
    emu.xrr_no_count(29usize, 29usize, 5usize, 2191700u32);
    emu.sli_no_count(5usize, 15usize, 19u32, 2191704u32);
    emu.orr_no_count(6usize, 30usize, 6usize, 2191708u32);
    emu.sri_no_count(30usize, 15usize, 22u32, 2191712u32);
    emu.orr_no_count(8usize, 5usize, 8usize, 2191716u32);
    emu.sli_no_count(5usize, 15usize, 10u32, 2191720u32);
    emu.orr_no_count(30usize, 5usize, 30usize, 2191724u32);
    emu.anr_no_count(5usize, 14usize, 13usize, 2191728u32);
    emu.anr_no_count(9usize, 15usize, 18usize, 2191732u32);
    emu.xrr_no_count(9usize, 9usize, 5usize, 2191736u32);
    emu.lw_no_count(5usize, 2usize, 364u32, 2191740u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2191744u32);
    emu.xrr_no_count(11usize, 11usize, 28usize, 2191748u32);
    emu.anr_no_count(28usize, 15usize, 14usize, 2191752u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2191756u32);
    emu.xrr_no_count(29usize, 17usize, 10usize, 2191760u32);
    emu.xrr_no_count(6usize, 6usize, 8usize, 2191764u32);
    emu.xrr_no_count(11usize, 11usize, 31usize, 2191768u32);
    emu.lw_no_count(31usize, 2usize, 40u32, 2191772u32)?;
    emu.adr_no_count(16usize, 16usize, 31usize, 2191776u32);
    emu.xrr_no_count(6usize, 6usize, 30usize, 2191780u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2191784u32);
    emu.adr_no_count(16usize, 6usize, 9usize, 2191788u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2191792u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2191796u32);
    emu.sri_no_count(11usize, 12usize, 6u32, 2191800u32);
    emu.sli_no_count(6usize, 12usize, 26u32, 2191804u32);
    emu.sri_no_count(30usize, 12usize, 11u32, 2191808u32);
    emu.sli_no_count(31usize, 12usize, 21u32, 2191812u32);
    emu.sri_no_count(8usize, 12usize, 25u32, 2191816u32);
    emu.sli_no_count(9usize, 12usize, 7u32, 2191820u32);
    emu.anr_no_count(29usize, 12usize, 29usize, 2191824u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2191828u32);
    emu.sri_no_count(6usize, 16usize, 2u32, 2191832u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2191836u32);
    emu.sli_no_count(31usize, 16usize, 30u32, 2191840u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2191844u32);
    emu.sri_no_count(9usize, 16usize, 13u32, 2191848u32);
    emu.xrr_no_count(10usize, 29usize, 10usize, 2191852u32);
    emu.sli_no_count(29usize, 16usize, 19u32, 2191856u32);
    emu.orr_no_count(31usize, 31usize, 6usize, 2191860u32);
    emu.sri_no_count(6usize, 16usize, 22u32, 2191864u32);
    emu.orr_no_count(29usize, 29usize, 9usize, 2191868u32);
    emu.sli_no_count(9usize, 16usize, 10u32, 2191872u32);
    emu.orr_no_count(9usize, 9usize, 6usize, 2191876u32);
    emu.xrr_no_count(6usize, 15usize, 14usize, 2191880u32);
    emu.anr_no_count(6usize, 16usize, 6usize, 2191884u32);
    emu.xrr_no_count(28usize, 6usize, 28usize, 2191888u32);
    emu.lw_no_count(6usize, 2usize, 492u32, 2191892u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2191896u32);
    emu.xrr_no_count(11usize, 11usize, 30usize, 2191900u32);
    emu.xrr_no_count(30usize, 12usize, 17usize, 2191904u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2191908u32);
    emu.xrr_no_count(7usize, 16usize, 15usize, 2191912u32);
    emu.xrr_no_count(29usize, 31usize, 29usize, 2191916u32);
    emu.xrr_no_count(11usize, 11usize, 8usize, 2191920u32);
    emu.lw_no_count(31usize, 2usize, 36u32, 2191924u32)?;
    emu.adr_no_count(10usize, 10usize, 31usize, 2191928u32);
    emu.xrr_no_count(29usize, 29usize, 9usize, 2191932u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2191936u32);
    emu.adr_no_count(11usize, 29usize, 28usize, 2191940u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2191944u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2191948u32);
    emu.sri_no_count(10usize, 13usize, 6u32, 2191952u32);
    emu.sli_no_count(28usize, 13usize, 26u32, 2191956u32);
    emu.sri_no_count(29usize, 13usize, 11u32, 2191960u32);
    emu.sli_no_count(31usize, 13usize, 21u32, 2191964u32);
    emu.sri_no_count(8usize, 13usize, 25u32, 2191968u32);
    emu.sli_no_count(9usize, 13usize, 7u32, 2191972u32);
    emu.anr_no_count(30usize, 13usize, 30usize, 2191976u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2191980u32);
    emu.sri_no_count(28usize, 11usize, 2u32, 2191984u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2191988u32);
    emu.sli_no_count(31usize, 11usize, 30u32, 2191992u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2191996u32);
    emu.sri_no_count(9usize, 11usize, 13u32, 2192000u32);
    emu.xrr_no_count(17usize, 30usize, 17usize, 2192004u32);
    emu.sli_no_count(30usize, 11usize, 19u32, 2192008u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2192012u32);
    emu.sri_no_count(31usize, 11usize, 22u32, 2192016u32);
    emu.orr_no_count(30usize, 30usize, 9usize, 2192020u32);
    emu.sli_no_count(9usize, 11usize, 10u32, 2192024u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2192028u32);
    emu.anr_no_count(9usize, 16usize, 15usize, 2192032u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2192036u32);
    emu.xrr_no_count(9usize, 7usize, 9usize, 2192040u32);
    emu.lw_no_count(7usize, 2usize, 408u32, 2192044u32)?;
    emu.adr_no_count(7usize, 7usize, 12usize, 2192048u32);
    emu.xrr_no_count(10usize, 10usize, 29usize, 2192052u32);
    emu.anr_no_count(29usize, 11usize, 16usize, 2192056u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2192060u32);
    emu.xrr_no_count(18usize, 13usize, 12usize, 2192064u32);
    emu.xrr_no_count(5usize, 28usize, 30usize, 2192068u32);
    emu.xrr_no_count(10usize, 10usize, 8usize, 2192072u32);
    emu.lw_no_count(28usize, 2usize, 32u32, 2192076u32)?;
    emu.adr_no_count(17usize, 17usize, 28usize, 2192080u32);
    emu.xrr_no_count(5usize, 5usize, 31usize, 2192084u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2192088u32);
    emu.adr_no_count(10usize, 5usize, 9usize, 2192092u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2192096u32);
    emu.adr_no_count(5usize, 17usize, 14usize, 2192100u32);
    emu.sri_no_count(14usize, 5usize, 6u32, 2192104u32);
    emu.sli_no_count(17usize, 5usize, 26u32, 2192108u32);
    emu.sri_no_count(28usize, 5usize, 11u32, 2192112u32);
    emu.sli_no_count(30usize, 5usize, 21u32, 2192116u32);
    emu.sri_no_count(31usize, 5usize, 25u32, 2192120u32);
    emu.sli_no_count(8usize, 5usize, 7u32, 2192124u32);
    emu.anr_no_count(9usize, 5usize, 18usize, 2192128u32);
    emu.orr_no_count(17usize, 17usize, 14usize, 2192132u32);
    emu.sri_no_count(14usize, 10usize, 2u32, 2192136u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2192140u32);
    emu.sli_no_count(30usize, 10usize, 30u32, 2192144u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2192148u32);
    emu.sri_no_count(8usize, 10usize, 13u32, 2192152u32);
    emu.xrr_no_count(12usize, 9usize, 12usize, 2192156u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2192160u32);
    emu.orr_no_count(30usize, 30usize, 14usize, 2192164u32);
    emu.sri_no_count(14usize, 10usize, 22u32, 2192168u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2192172u32);
    emu.sli_no_count(9usize, 10usize, 10u32, 2192176u32);
    emu.orr_no_count(9usize, 9usize, 14usize, 2192180u32);
    emu.xrr_no_count(14usize, 11usize, 16usize, 2192184u32);
    emu.anr_no_count(14usize, 10usize, 14usize, 2192188u32);
    emu.xrr_no_count(29usize, 14usize, 29usize, 2192192u32);
    emu.lw_no_count(14usize, 2usize, 504u32, 2192196u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2192200u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2192204u32);
    emu.xrr_no_count(28usize, 5usize, 13usize, 2192208u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2192212u32);
    emu.xrr_no_count(6usize, 10usize, 11usize, 2192216u32);
    emu.xrr_no_count(30usize, 30usize, 8usize, 2192220u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2192224u32);
    emu.lw_no_count(31usize, 2usize, 28u32, 2192228u32)?;
    emu.adr_no_count(12usize, 12usize, 31usize, 2192232u32);
    emu.xrr_no_count(30usize, 30usize, 9usize, 2192236u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2192240u32);
    emu.adr_no_count(17usize, 30usize, 29usize, 2192244u32);
    emu.adr_no_count(17usize, 17usize, 12usize, 2192248u32);
    emu.adr_no_count(15usize, 12usize, 15usize, 2192252u32);
    emu.sri_no_count(12usize, 15usize, 6u32, 2192256u32);
    emu.sli_no_count(29usize, 15usize, 26u32, 2192260u32);
    emu.sri_no_count(30usize, 15usize, 11u32, 2192264u32);
    emu.sli_no_count(31usize, 15usize, 21u32, 2192268u32);
    emu.sri_no_count(8usize, 15usize, 25u32, 2192272u32);
    emu.sli_no_count(9usize, 15usize, 7u32, 2192276u32);
    emu.anr_no_count(28usize, 15usize, 28usize, 2192280u32);
    emu.orr_no_count(12usize, 29usize, 12usize, 2192284u32);
    emu.sri_no_count(29usize, 17usize, 2u32, 2192288u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2192292u32);
    emu.sli_no_count(31usize, 17usize, 30u32, 2192296u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2192300u32);
    emu.sri_no_count(9usize, 17usize, 13u32, 2192304u32);
    emu.xrr_no_count(28usize, 28usize, 13usize, 2192308u32);
    emu.sli_no_count(13usize, 17usize, 19u32, 2192312u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2192316u32);
    emu.sri_no_count(31usize, 17usize, 22u32, 2192320u32);
    emu.orr_no_count(9usize, 13usize, 9usize, 2192324u32);
    emu.sli_no_count(13usize, 17usize, 10u32, 2192328u32);
    emu.orr_no_count(31usize, 13usize, 31usize, 2192332u32);
    emu.anr_no_count(13usize, 10usize, 11usize, 2192336u32);
    emu.anr_no_count(6usize, 17usize, 6usize, 2192340u32);
    emu.xrr_no_count(6usize, 6usize, 13usize, 2192344u32);
    emu.lw_no_count(13usize, 2usize, 444u32, 2192348u32)?;
    emu.adr_no_count(13usize, 13usize, 5usize, 2192352u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2192356u32);
    emu.anr_no_count(30usize, 17usize, 10usize, 2192360u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2192364u32);
    emu.xrr_no_count(28usize, 15usize, 5usize, 2192368u32);
    emu.xrr_no_count(29usize, 29usize, 9usize, 2192372u32);
    emu.xrr_no_count(12usize, 12usize, 8usize, 2192376u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2192380u32)?;
    emu.adr_no_count(7usize, 7usize, 8usize, 2192384u32);
    emu.xrr_no_count(29usize, 29usize, 31usize, 2192388u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2192392u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2192396u32);
    emu.adr_no_count(6usize, 6usize, 12usize, 2192400u32);
    emu.adr_no_count(16usize, 12usize, 16usize, 2192404u32);
    emu.sri_no_count(12usize, 16usize, 6u32, 2192408u32);
    emu.sli_no_count(7usize, 16usize, 26u32, 2192412u32);
    emu.sri_no_count(29usize, 16usize, 11u32, 2192416u32);
    emu.sli_no_count(31usize, 16usize, 21u32, 2192420u32);
    emu.sri_no_count(8usize, 16usize, 25u32, 2192424u32);
    emu.sli_no_count(9usize, 16usize, 7u32, 2192428u32);
    emu.anr_no_count(28usize, 16usize, 28usize, 2192432u32);
    emu.orr_no_count(7usize, 7usize, 12usize, 2192436u32);
    emu.sri_no_count(12usize, 6usize, 2u32, 2192440u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2192444u32);
    emu.sli_no_count(31usize, 6usize, 30u32, 2192448u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2192452u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2192456u32);
    emu.xrr_no_count(28usize, 28usize, 5usize, 2192460u32);
    emu.sli_no_count(5usize, 6usize, 19u32, 2192464u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2192468u32);
    emu.sri_no_count(12usize, 6usize, 22u32, 2192472u32);
    emu.orr_no_count(9usize, 5usize, 9usize, 2192476u32);
    emu.sli_no_count(5usize, 6usize, 10u32, 2192480u32);
    emu.orr_no_count(18usize, 5usize, 12usize, 2192484u32);
    emu.xrr_no_count(12usize, 17usize, 10usize, 2192488u32);
    emu.anr_no_count(12usize, 6usize, 12usize, 2192492u32);
    emu.xrr_no_count(30usize, 12usize, 30usize, 2192496u32);
    emu.lw_no_count(12usize, 2usize, 360u32, 2192500u32)?;
    emu.adr_no_count(12usize, 12usize, 15usize, 2192504u32);
    emu.lw_no_count(5usize, 2usize, 424u32, 2192508u32)?;
    emu.adr_no_count(5usize, 17usize, 5usize, 2192512u32);
    emu.xrr_no_count(7usize, 7usize, 29usize, 2192516u32);
    emu.xrr_no_count(29usize, 16usize, 15usize, 2192520u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2192524u32);
    emu.xrr_no_count(28usize, 6usize, 17usize, 2192528u32);
    emu.anr_no_count(17usize, 6usize, 17usize, 2192532u32);
    emu.adr_no_count(19usize, 6usize, 19usize, 2192536u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2192540u32);
    emu.xrr_no_count(7usize, 7usize, 8usize, 2192544u32);
    emu.lw_no_count(8usize, 2usize, 20u32, 2192548u32)?;
    emu.adr_no_count(14usize, 14usize, 8usize, 2192552u32);
    emu.xrr_no_count(31usize, 31usize, 18usize, 2192556u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2192560u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2192564u32);
    emu.adr_no_count(30usize, 30usize, 14usize, 2192568u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2192572u32);
    emu.sri_no_count(14usize, 11usize, 6u32, 2192576u32);
    emu.sli_no_count(7usize, 11usize, 26u32, 2192580u32);
    emu.sri_no_count(31usize, 11usize, 11u32, 2192584u32);
    emu.sli_no_count(8usize, 11usize, 21u32, 2192588u32);
    emu.sri_no_count(9usize, 11usize, 25u32, 2192592u32);
    emu.sli_no_count(18usize, 11usize, 7u32, 2192596u32);
    emu.anr_no_count(29usize, 11usize, 29usize, 2192600u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2192604u32);
    emu.sri_no_count(7usize, 30usize, 2u32, 2192608u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2192612u32);
    emu.sli_no_count(8usize, 30usize, 30u32, 2192616u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2192620u32);
    emu.sri_no_count(18usize, 30usize, 13u32, 2192624u32);
    emu.xrr_no_count(15usize, 29usize, 15usize, 2192628u32);
    emu.sli_no_count(29usize, 30usize, 19u32, 2192632u32);
    emu.orr_no_count(7usize, 8usize, 7usize, 2192636u32);
    emu.sri_no_count(8usize, 30usize, 22u32, 2192640u32);
    emu.orr_no_count(29usize, 29usize, 18usize, 2192644u32);
    emu.anr_no_count(18usize, 30usize, 6usize, 2192648u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2192652u32);
    emu.xrr_no_count(6usize, 30usize, 6usize, 2192656u32);
    emu.adr_no_count(21usize, 30usize, 21usize, 2192660u32);
    emu.sli_no_count(30usize, 30usize, 10u32, 2192664u32);
    emu.orr_no_count(30usize, 30usize, 8usize, 2192668u32);
    emu.xrr_no_count(17usize, 28usize, 17usize, 2192672u32);
    emu.xrr_no_count(14usize, 14usize, 31usize, 2192676u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2192680u32);
    emu.xrr_no_count(15usize, 7usize, 29usize, 2192684u32);
    emu.adr_no_count(26usize, 11usize, 26usize, 2192688u32);
    emu.xrr_no_count(11usize, 11usize, 16usize, 2192692u32);
    emu.xrr_no_count(14usize, 14usize, 9usize, 2192696u32);
    emu.lw_no_count(9usize, 2usize, 372u32, 2192700u32)?;
    emu.lw_no_count(7usize, 2usize, 16u32, 2192704u32)?;
    emu.adr_no_count(13usize, 13usize, 7usize, 2192708u32);
    emu.xrr_no_count(15usize, 15usize, 30usize, 2192712u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2192716u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2192720u32);
    emu.adr_no_count(15usize, 15usize, 13usize, 2192724u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2192728u32);
    emu.sri_no_count(13usize, 10usize, 6u32, 2192732u32);
    emu.sli_no_count(14usize, 10usize, 26u32, 2192736u32);
    emu.sri_no_count(17usize, 10usize, 11u32, 2192740u32);
    emu.sli_no_count(7usize, 10usize, 21u32, 2192744u32);
    emu.sri_no_count(28usize, 10usize, 25u32, 2192748u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2192752u32);
    emu.adr_no_count(20usize, 10usize, 20usize, 2192756u32);
    emu.sli_no_count(10usize, 10usize, 7u32, 2192760u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2192764u32);
    emu.sri_no_count(14usize, 15usize, 2u32, 2192768u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2192772u32);
    emu.sli_no_count(7usize, 15usize, 30u32, 2192776u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2192780u32);
    emu.sri_no_count(28usize, 15usize, 13u32, 2192784u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2192788u32);
    emu.sli_no_count(7usize, 15usize, 19u32, 2192792u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2192796u32);
    emu.sri_no_count(28usize, 15usize, 22u32, 2192800u32);
    emu.anr_no_count(6usize, 15usize, 6usize, 2192804u32);
    emu.adr_no_count(22usize, 15usize, 22usize, 2192808u32);
    emu.sli_no_count(15usize, 15usize, 10u32, 2192812u32);
    emu.orr_no_count(15usize, 15usize, 28usize, 2192816u32);
    emu.xrr_no_count(6usize, 6usize, 18usize, 2192820u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2192824u32);
    emu.xrr_no_count(11usize, 11usize, 16usize, 2192828u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2192832u32);
    emu.xrr_no_count(12usize, 14usize, 7usize, 2192836u32);
    emu.lw_no_count(14usize, 2usize, 508u32, 2192840u32)?;
    emu.adr_no_count(6usize, 6usize, 14usize, 2192844u32);
    emu.xrr_no_count(10usize, 13usize, 10usize, 2192848u32);
    emu.xrr_no_count(12usize, 12usize, 15usize, 2192852u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2192856u32)?;
    emu.adr_no_count(11usize, 11usize, 13usize, 2192860u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2192864u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2192868u32);
    emu.adr_no_count(23usize, 12usize, 10usize, 2192872u32);
    emu.adr_no_count(30usize, 5usize, 10usize, 2192876u32);
    emu.lw_no_count(10usize, 2usize, 368u32, 2192880u32)?;
    emu.adi_no_count(12usize, 10usize, 64u32, 2192884u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2192888u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2192892u32);
    emu.lw_no_count(10usize, 2usize, 268u32, 2192896u32)?;
    emu.add_memory_rw_events(4002usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2192904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217608));
    } else {
        emu.pc = 2192900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217604));
    }
}
#[inline(always)]
pub fn block_0x00217604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2192904u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2176888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00213778));
}
#[inline]
pub fn block_0x00217608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2192908u32)?;
    emu.sw_no_count(23usize, 10usize, 0u32, 2192912u32)?;
    emu.sw_no_count(22usize, 10usize, 4u32, 2192916u32)?;
    emu.sw_no_count(21usize, 10usize, 8u32, 2192920u32)?;
    emu.sw_no_count(19usize, 10usize, 12u32, 2192924u32)?;
    emu.sw_no_count(30usize, 10usize, 16u32, 2192928u32)?;
    emu.sw_no_count(20usize, 10usize, 20u32, 2192932u32)?;
    emu.sw_no_count(26usize, 10usize, 24u32, 2192936u32)?;
    emu.sw_no_count(9usize, 10usize, 28u32, 2192940u32)?;
    emu.lw_no_count(1usize, 2usize, 572u32, 2192944u32)?;
    emu.lw_no_count(8usize, 2usize, 568u32, 2192948u32)?;
    emu.lw_no_count(9usize, 2usize, 564u32, 2192952u32)?;
    emu.lw_no_count(18usize, 2usize, 560u32, 2192956u32)?;
    emu.lw_no_count(19usize, 2usize, 556u32, 2192960u32)?;
    emu.lw_no_count(20usize, 2usize, 552u32, 2192964u32)?;
    emu.lw_no_count(21usize, 2usize, 548u32, 2192968u32)?;
    emu.lw_no_count(22usize, 2usize, 544u32, 2192972u32)?;
    emu.lw_no_count(23usize, 2usize, 540u32, 2192976u32)?;
    emu.lw_no_count(24usize, 2usize, 536u32, 2192980u32)?;
    emu.lw_no_count(25usize, 2usize, 532u32, 2192984u32)?;
    emu.lw_no_count(26usize, 2usize, 528u32, 2192988u32)?;
    emu.lw_no_count(27usize, 2usize, 524u32, 2192992u32)?;
    emu.adi_no_count(2usize, 2usize, 576u32, 2192996u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193000u32;
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
pub fn block_0x00217668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2193004u32)?;
    emu.apc_no_count(6usize, 2193004u32, 0u32, 2193008u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193012u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2193016u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2193020u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2193024u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2193028u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193032u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 120u32, 2193036u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2193040u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2193044u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2193048u32;
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
pub fn block_0x00217698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193052u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 305u32, 2193056u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2193060u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193064u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193068u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193072u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193076u32);
    emu.apc_no_count(6usize, 2193076u32, 45056u32, 2193080u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193084u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002176bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193088u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 445u32, 2193092u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2193096u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193100u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193104u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193108u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193112u32);
    emu.apc_no_count(6usize, 2193112u32, 45056u32, 2193116u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193120u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002176e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193124u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 422u32, 2193128u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2193132u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193136u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193140u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193144u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193148u32);
    emu.apc_no_count(6usize, 2193148u32, 45056u32, 2193152u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193156u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193160u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 381u32, 2193164u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2193168u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193172u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193176u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193180u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193184u32);
    emu.apc_no_count(6usize, 2193184u32, 45056u32, 2193188u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193192u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193196u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 568u32, 2193200u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2193204u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193208u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193212u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193216u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193220u32);
    emu.apc_no_count(6usize, 2193220u32, 45056u32, 2193224u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193228u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021774c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193232u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 394u32, 2193236u32);
    emu.adi_no_count(12usize, 0usize, 15u32, 2193240u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193244u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193248u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193252u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193256u32);
    emu.apc_no_count(6usize, 2193256u32, 45056u32, 2193260u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193264u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193268u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966676u32, 2193272u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2193276u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193280u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193284u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193288u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193292u32);
    emu.apc_no_count(6usize, 2193292u32, 45056u32, 2193296u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193300u32;
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
pub fn block_0x00217794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193304u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 319u32, 2193308u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2193312u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193316u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193320u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193324u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193328u32);
    emu.apc_no_count(6usize, 2193328u32, 45056u32, 2193332u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193336u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002177b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193340u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 436u32, 2193344u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2193348u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193352u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193356u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193360u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193364u32);
    emu.apc_no_count(6usize, 2193364u32, 45056u32, 2193368u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193372u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002177dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193376u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 329u32, 2193380u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2193384u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193388u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193392u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193396u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193400u32);
    emu.apc_no_count(6usize, 2193400u32, 45056u32, 2193404u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193408u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00217800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193412u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 358u32, 2193416u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2193420u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193424u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193428u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193432u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193436u32);
    emu.apc_no_count(6usize, 2193436u32, 45056u32, 2193440u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193444u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193448u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 312u32, 2193452u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2193456u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193460u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193464u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193468u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193472u32);
    emu.apc_no_count(6usize, 2193472u32, 45056u32, 2193476u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193480u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193484u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 452u32, 2193488u32);
    emu.adi_no_count(12usize, 0usize, 15u32, 2193492u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193496u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193500u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193504u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193508u32);
    emu.apc_no_count(6usize, 2193508u32, 45056u32, 2193512u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193516u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021786c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2193520u32);
    emu.lbu_no_count(10usize, 10usize, 2u32, 2193524u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193528u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 512u32, 2193532u32);
    emu.adi_no_count(14usize, 2usize, 3u32, 2193536u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2193540u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966260u32, 2193544u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2193548u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966228u32, 2193552u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2193556u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 544u32, 2193560u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2193564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2193872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002179d0));
}
#[inline]
pub fn block_0x0021789c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193568u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 341u32, 2193572u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2193576u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193580u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193584u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193588u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193592u32);
    emu.apc_no_count(6usize, 2193592u32, 45056u32, 2193596u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193600u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002178c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2193604u32);
    emu.lbu_no_count(10usize, 10usize, 2u32, 2193608u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193612u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 512u32, 2193616u32);
    emu.adi_no_count(14usize, 2usize, 3u32, 2193620u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2193624u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966260u32, 2193628u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2193632u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966228u32, 2193636u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2193640u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 588u32, 2193644u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2193648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2193872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002179d0));
}
#[inline]
pub fn block_0x002178f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193652u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 409u32, 2193656u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2193660u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193664u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193668u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193672u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193676u32);
    emu.apc_no_count(6usize, 2193676u32, 45056u32, 2193680u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193684u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00217914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193688u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 628u32, 2193692u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2193696u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193700u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193704u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193708u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193712u32);
    emu.apc_no_count(6usize, 2193712u32, 45056u32, 2193716u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193720u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193724u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 467u32, 2193728u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2193732u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193736u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193740u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193744u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193748u32);
    emu.apc_no_count(6usize, 2193748u32, 45056u32, 2193752u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193756u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021795c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193760u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 378u32, 2193764u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2193768u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193772u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193776u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193780u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193784u32);
    emu.apc_no_count(6usize, 2193784u32, 45056u32, 2193788u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193792u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193796u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 368u32, 2193800u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2193804u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193808u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193812u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193816u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193820u32);
    emu.apc_no_count(6usize, 2193820u32, 45056u32, 2193824u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2193828u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002179a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2193832u32);
    emu.lbu_no_count(10usize, 10usize, 2u32, 2193836u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193840u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 512u32, 2193844u32);
    emu.adi_no_count(14usize, 2usize, 3u32, 2193848u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2193852u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966260u32, 2193856u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2193860u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966228u32, 2193864u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2193868u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 624u32, 2193872u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2193872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002179d0));
}
#[inline]
pub fn block_0x002179d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 3u32, 2193876u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2193880u32)?;
    emu.sb_no_count(12usize, 2usize, 3u32, 2193884u32);
    emu.adi_no_count(12usize, 2usize, 28u32, 2193888u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2193892u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2193896u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2193900u32);
    emu.sw_no_count(14usize, 2usize, 28u32, 2193904u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2193908u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2193912u32)?;
    emu.sw_no_count(16usize, 2usize, 40u32, 2193916u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2193920u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2193924u32)?;
    emu.sw_no_count(17usize, 2usize, 4u32, 2193928u32)?;
    emu.sw_no_count(5usize, 2usize, 8u32, 2193932u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2193936u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2193940u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2193944u32);
    emu.apc_no_count(1usize, 2193944u32, 40960u32, 2193948u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217a20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2193956u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193960u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2193964u32;
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
pub fn block_0x00217a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2193968u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 480u32, 2193972u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2193976u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2193980u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2193984u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2193988u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2193992u32);
    emu.apc_no_count(6usize, 2193992u32, 45056u32, 2193996u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194000u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2194004u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2194008u32)?;
    emu.lbu_no_count(12usize, 10usize, 1u32, 2194012u32);
    emu.lbu_no_count(13usize, 10usize, 2u32, 2194016u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2194020u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2194024u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2194028u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2194032u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2194036u32);
    emu.ani_no_count(13usize, 12usize, 255u32, 2194040u32);
    emu.sli_no_count(13usize, 13usize, 2u32, 2194044u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2194048u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 212u32, 2194052u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2194056u32);
    emu.lw_no_count(14usize, 13usize, 0u32, 2194060u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2194064u32)?;
    emu.adi_no_count(13usize, 0usize, 64u32, 2194068u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2194072u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2194076u32;
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
pub fn block_0x00217a9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2194080u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 23u32, 2194088u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194092u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 21u32, 2194096u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194100u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 18u32, 2194104u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 5u32, 2194112u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 19u32, 2194120u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 48u32, 2194128u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2194136u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217adc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 22u32, 2194144u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2194152u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 10u32, 2194160u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 24u32, 2194168u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 6u32, 2194176u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 128u32, 2194184u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194188u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b38));
}
#[inline(always)]
pub fn block_0x00217b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2194192u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 9u32, 2194200u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 26u32, 2194208u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 49u32, 2194216u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2194224u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
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
    emu.adi_no_count(13usize, 0usize, 192u32, 2194232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b38));
}
#[inline(always)]
pub fn block_0x00217b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 12usize, 8u32, 2194236u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2194240u32);
    emu.ani_no_count(12usize, 12usize, 32u32, 2194244u32);
    emu.orr_no_count(10usize, 10usize, 13usize, 2194248u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2194252u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2194256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(always)]
pub fn block_0x00217b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 30u32, 2194260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217b54));
}
#[inline(never)]
pub fn block_0x00217b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 2usize, 43u32, 2194264u32);
    emu.adi_no_count(10usize, 2usize, 43u32, 2194268u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2194272u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966492u32, 2194276u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2194280u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2194284u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1640u32, 2194288u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2194292u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 656u32, 2194296u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2194300u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2194304u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 680u32, 2194308u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2194312u32);
    emu.adi_no_count(6usize, 2usize, 44u32, 2194316u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2194320u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2194324u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2194328u32)?;
    emu.sw_no_count(14usize, 2usize, 56u32, 2194332u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2194336u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2194340u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2194344u32)?;
    emu.sw_no_count(5usize, 2usize, 36u32, 2194348u32)?;
    emu.sw_no_count(15usize, 2usize, 16u32, 2194352u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2194356u32)?;
    emu.sw_no_count(6usize, 2usize, 24u32, 2194360u32)?;
    emu.sw_no_count(5usize, 2usize, 28u32, 2194364u32)?;
    emu.adi_no_count(12usize, 2usize, 16u32, 2194368u32);
    emu.apc_no_count(1usize, 2194368u32, 40960u32, 2194372u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
