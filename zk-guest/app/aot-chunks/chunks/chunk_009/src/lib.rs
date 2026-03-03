pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2121932u32;
pub const PC_MAX: u32 = 2141280u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x002060cc,
        block_0x002060d4,
        block_0x002060e4,
        block_0x002060e8,
        block_0x00206110,
        block_0x00206134,
        block_0x00206144,
        block_0x0020614c,
        block_0x0020615c,
        block_0x00206160,
        block_0x00206168,
        block_0x0020616c,
        block_0x002061a0,
        block_0x002061a8,
        block_0x00206208,
        block_0x0020620c,
        block_0x00206518,
        block_0x0020a3a4,
        block_0x0020a3a8,
        block_0x0020a408,
        block_0x0020a428,
        block_0x0020a43c,
        block_0x0020a460,
        block_0x0020a4b8,
        block_0x0020a4c4,
        block_0x0020a4e8,
        block_0x0020a50c,
        block_0x0020a530,
        block_0x0020a588,
        block_0x0020a594,
        block_0x0020a5b8,
        block_0x0020a5dc,
        block_0x0020a5f4,
        block_0x0020a618,
        block_0x0020a658,
        block_0x0020a67c,
        block_0x0020a690,
        block_0x0020a6b8,
        block_0x0020a6dc,
        block_0x0020a6e8,
        block_0x0020a708,
        block_0x0020a764,
        block_0x0020a770,
        block_0x0020a794,
        block_0x0020a7b8,
        block_0x0020a7dc,
        block_0x0020a800,
        block_0x0020a818,
        block_0x0020a824,
        block_0x0020a844,
        block_0x0020a88c,
        block_0x0020a89c,
        block_0x0020a8f4,
        block_0x0020a8f8,
        block_0x0020a90c,
        block_0x0020a914,
        block_0x0020a92c,
        block_0x0020a930,
        block_0x0020a944,
        block_0x0020a964,
        block_0x0020a968,
        block_0x0020a974,
        block_0x0020a984,
        block_0x0020a994,
        block_0x0020a998,
        block_0x0020a9ac,
        block_0x0020a9c0,
        block_0x0020a9c4,
        block_0x0020a9f0,
        block_0x0020aa14,
        block_0x0020aa30,
        block_0x0020aa40,
        block_0x0020aa48,
        block_0x0020aa5c,
        block_0x0020aa60,
        block_0x0020aa70,
        block_0x0020aa74,
        block_0x0020aa7c,
        block_0x0020aa8c,
        block_0x0020aa90,
        block_0x0020aa9c,
        block_0x0020aaa0,
        block_0x0020aaa4,
        block_0x0020aac0,
        block_0x0020aac8,
        block_0x0020aacc,
        block_0x0020aaf8,
        block_0x0020aafc,
        block_0x0020ab08,
        block_0x0020ab10,
        block_0x0020ab14,
        block_0x0020ab1c,
        block_0x0020ab28,
        block_0x0020ab3c,
        block_0x0020ab54,
        block_0x0020ab5c,
        block_0x0020ab80,
        block_0x0020ab88,
        block_0x0020ab94,
        block_0x0020aba8,
        block_0x0020abb4,
        block_0x0020abbc,
        block_0x0020abdc,
        block_0x0020abe4,
        block_0x0020ac14,
        block_0x0020ac50,
        block_0x0020ac60,
    ];
    #[repr(C)]
    struct Run {
        start_word: u32,
        len: u16,
        fn_offset: u16,
    }
    const RUNS: [Run; 89usize] = [
        Run {
            start_word: 0u32,
            len: 1i32 as u16,
            fn_offset: 0usize as u16,
        },
        Run {
            start_word: 2u32,
            len: 1i32 as u16,
            fn_offset: 1usize as u16,
        },
        Run {
            start_word: 6u32,
            len: 2i32 as u16,
            fn_offset: 2usize as u16,
        },
        Run {
            start_word: 17u32,
            len: 1i32 as u16,
            fn_offset: 4usize as u16,
        },
        Run {
            start_word: 26u32,
            len: 1i32 as u16,
            fn_offset: 5usize as u16,
        },
        Run {
            start_word: 30u32,
            len: 1i32 as u16,
            fn_offset: 6usize as u16,
        },
        Run {
            start_word: 32u32,
            len: 1i32 as u16,
            fn_offset: 7usize as u16,
        },
        Run {
            start_word: 36u32,
            len: 2i32 as u16,
            fn_offset: 8usize as u16,
        },
        Run {
            start_word: 39u32,
            len: 2i32 as u16,
            fn_offset: 10usize as u16,
        },
        Run {
            start_word: 53u32,
            len: 1i32 as u16,
            fn_offset: 12usize as u16,
        },
        Run {
            start_word: 55u32,
            len: 1i32 as u16,
            fn_offset: 13usize as u16,
        },
        Run {
            start_word: 79u32,
            len: 2i32 as u16,
            fn_offset: 14usize as u16,
        },
        Run {
            start_word: 275u32,
            len: 1i32 as u16,
            fn_offset: 16usize as u16,
        },
        Run {
            start_word: 4278u32,
            len: 2i32 as u16,
            fn_offset: 17usize as u16,
        },
        Run {
            start_word: 4303u32,
            len: 1i32 as u16,
            fn_offset: 19usize as u16,
        },
        Run {
            start_word: 4311u32,
            len: 1i32 as u16,
            fn_offset: 20usize as u16,
        },
        Run {
            start_word: 4316u32,
            len: 1i32 as u16,
            fn_offset: 21usize as u16,
        },
        Run {
            start_word: 4325u32,
            len: 1i32 as u16,
            fn_offset: 22usize as u16,
        },
        Run {
            start_word: 4347u32,
            len: 1i32 as u16,
            fn_offset: 23usize as u16,
        },
        Run {
            start_word: 4350u32,
            len: 1i32 as u16,
            fn_offset: 24usize as u16,
        },
        Run {
            start_word: 4359u32,
            len: 1i32 as u16,
            fn_offset: 25usize as u16,
        },
        Run {
            start_word: 4368u32,
            len: 1i32 as u16,
            fn_offset: 26usize as u16,
        },
        Run {
            start_word: 4377u32,
            len: 1i32 as u16,
            fn_offset: 27usize as u16,
        },
        Run {
            start_word: 4399u32,
            len: 1i32 as u16,
            fn_offset: 28usize as u16,
        },
        Run {
            start_word: 4402u32,
            len: 1i32 as u16,
            fn_offset: 29usize as u16,
        },
        Run {
            start_word: 4411u32,
            len: 1i32 as u16,
            fn_offset: 30usize as u16,
        },
        Run {
            start_word: 4420u32,
            len: 1i32 as u16,
            fn_offset: 31usize as u16,
        },
        Run {
            start_word: 4426u32,
            len: 1i32 as u16,
            fn_offset: 32usize as u16,
        },
        Run {
            start_word: 4435u32,
            len: 1i32 as u16,
            fn_offset: 33usize as u16,
        },
        Run {
            start_word: 4451u32,
            len: 1i32 as u16,
            fn_offset: 34usize as u16,
        },
        Run {
            start_word: 4460u32,
            len: 1i32 as u16,
            fn_offset: 35usize as u16,
        },
        Run {
            start_word: 4465u32,
            len: 1i32 as u16,
            fn_offset: 36usize as u16,
        },
        Run {
            start_word: 4475u32,
            len: 1i32 as u16,
            fn_offset: 37usize as u16,
        },
        Run {
            start_word: 4484u32,
            len: 1i32 as u16,
            fn_offset: 38usize as u16,
        },
        Run {
            start_word: 4487u32,
            len: 1i32 as u16,
            fn_offset: 39usize as u16,
        },
        Run {
            start_word: 4495u32,
            len: 1i32 as u16,
            fn_offset: 40usize as u16,
        },
        Run {
            start_word: 4518u32,
            len: 1i32 as u16,
            fn_offset: 41usize as u16,
        },
        Run {
            start_word: 4521u32,
            len: 1i32 as u16,
            fn_offset: 42usize as u16,
        },
        Run {
            start_word: 4530u32,
            len: 1i32 as u16,
            fn_offset: 43usize as u16,
        },
        Run {
            start_word: 4539u32,
            len: 1i32 as u16,
            fn_offset: 44usize as u16,
        },
        Run {
            start_word: 4548u32,
            len: 1i32 as u16,
            fn_offset: 45usize as u16,
        },
        Run {
            start_word: 4557u32,
            len: 1i32 as u16,
            fn_offset: 46usize as u16,
        },
        Run {
            start_word: 4563u32,
            len: 1i32 as u16,
            fn_offset: 47usize as u16,
        },
        Run {
            start_word: 4566u32,
            len: 1i32 as u16,
            fn_offset: 48usize as u16,
        },
        Run {
            start_word: 4574u32,
            len: 1i32 as u16,
            fn_offset: 49usize as u16,
        },
        Run {
            start_word: 4592u32,
            len: 1i32 as u16,
            fn_offset: 50usize as u16,
        },
        Run {
            start_word: 4596u32,
            len: 1i32 as u16,
            fn_offset: 51usize as u16,
        },
        Run {
            start_word: 4618u32,
            len: 2i32 as u16,
            fn_offset: 52usize as u16,
        },
        Run {
            start_word: 4624u32,
            len: 1i32 as u16,
            fn_offset: 54usize as u16,
        },
        Run {
            start_word: 4626u32,
            len: 1i32 as u16,
            fn_offset: 55usize as u16,
        },
        Run {
            start_word: 4632u32,
            len: 2i32 as u16,
            fn_offset: 56usize as u16,
        },
        Run {
            start_word: 4638u32,
            len: 1i32 as u16,
            fn_offset: 58usize as u16,
        },
        Run {
            start_word: 4646u32,
            len: 2i32 as u16,
            fn_offset: 59usize as u16,
        },
        Run {
            start_word: 4650u32,
            len: 1i32 as u16,
            fn_offset: 61usize as u16,
        },
        Run {
            start_word: 4654u32,
            len: 1i32 as u16,
            fn_offset: 62usize as u16,
        },
        Run {
            start_word: 4658u32,
            len: 2i32 as u16,
            fn_offset: 63usize as u16,
        },
        Run {
            start_word: 4664u32,
            len: 1i32 as u16,
            fn_offset: 65usize as u16,
        },
        Run {
            start_word: 4669u32,
            len: 2i32 as u16,
            fn_offset: 66usize as u16,
        },
        Run {
            start_word: 4681u32,
            len: 1i32 as u16,
            fn_offset: 68usize as u16,
        },
        Run {
            start_word: 4690u32,
            len: 1i32 as u16,
            fn_offset: 69usize as u16,
        },
        Run {
            start_word: 4697u32,
            len: 1i32 as u16,
            fn_offset: 70usize as u16,
        },
        Run {
            start_word: 4701u32,
            len: 1i32 as u16,
            fn_offset: 71usize as u16,
        },
        Run {
            start_word: 4703u32,
            len: 1i32 as u16,
            fn_offset: 72usize as u16,
        },
        Run {
            start_word: 4708u32,
            len: 2i32 as u16,
            fn_offset: 73usize as u16,
        },
        Run {
            start_word: 4713u32,
            len: 2i32 as u16,
            fn_offset: 75usize as u16,
        },
        Run {
            start_word: 4716u32,
            len: 1i32 as u16,
            fn_offset: 77usize as u16,
        },
        Run {
            start_word: 4720u32,
            len: 2i32 as u16,
            fn_offset: 78usize as u16,
        },
        Run {
            start_word: 4724u32,
            len: 3i32 as u16,
            fn_offset: 80usize as u16,
        },
        Run {
            start_word: 4733u32,
            len: 1i32 as u16,
            fn_offset: 83usize as u16,
        },
        Run {
            start_word: 4735u32,
            len: 2i32 as u16,
            fn_offset: 84usize as u16,
        },
        Run {
            start_word: 4747u32,
            len: 2i32 as u16,
            fn_offset: 86usize as u16,
        },
        Run {
            start_word: 4751u32,
            len: 1i32 as u16,
            fn_offset: 88usize as u16,
        },
        Run {
            start_word: 4753u32,
            len: 2i32 as u16,
            fn_offset: 89usize as u16,
        },
        Run {
            start_word: 4756u32,
            len: 1i32 as u16,
            fn_offset: 91usize as u16,
        },
        Run {
            start_word: 4759u32,
            len: 1i32 as u16,
            fn_offset: 92usize as u16,
        },
        Run {
            start_word: 4764u32,
            len: 1i32 as u16,
            fn_offset: 93usize as u16,
        },
        Run {
            start_word: 4770u32,
            len: 1i32 as u16,
            fn_offset: 94usize as u16,
        },
        Run {
            start_word: 4772u32,
            len: 1i32 as u16,
            fn_offset: 95usize as u16,
        },
        Run {
            start_word: 4781u32,
            len: 1i32 as u16,
            fn_offset: 96usize as u16,
        },
        Run {
            start_word: 4783u32,
            len: 1i32 as u16,
            fn_offset: 97usize as u16,
        },
        Run {
            start_word: 4786u32,
            len: 1i32 as u16,
            fn_offset: 98usize as u16,
        },
        Run {
            start_word: 4791u32,
            len: 1i32 as u16,
            fn_offset: 99usize as u16,
        },
        Run {
            start_word: 4794u32,
            len: 1i32 as u16,
            fn_offset: 100usize as u16,
        },
        Run {
            start_word: 4796u32,
            len: 1i32 as u16,
            fn_offset: 101usize as u16,
        },
        Run {
            start_word: 4804u32,
            len: 1i32 as u16,
            fn_offset: 102usize as u16,
        },
        Run {
            start_word: 4806u32,
            len: 1i32 as u16,
            fn_offset: 103usize as u16,
        },
        Run {
            start_word: 4818u32,
            len: 1i32 as u16,
            fn_offset: 104usize as u16,
        },
        Run {
            start_word: 4833u32,
            len: 1i32 as u16,
            fn_offset: 105usize as u16,
        },
        Run {
            start_word: 4837u32,
            len: 1i32 as u16,
            fn_offset: 106usize as u16,
        },
    ];
    if pc < 2121932u32 || pc > 2141280u32 {
        return None;
    }
    let word_offset = ((pc - 2121932u32) >> 2) as u32;
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
#[inline(always)]
pub fn block_0x002060cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2121932u32, 4294963200u32, 2121936u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002060d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2121944u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2121948u32);
    emu.apc_no_count(1usize, 2121948u32, 4294950912u32, 2121952u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002060e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2122036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206134));
    } else {
        emu.pc = 2121960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002060e8));
    }
}
#[inline]
pub fn block_0x002060e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2121964u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2121968u32)?;
    emu.sw_no_count(19usize, 10usize, 4u32, 2121972u32)?;
    emu.sw_no_count(8usize, 10usize, 8u32, 2121976u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121980u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 764u32, 2121984u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2121988u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2121992u32);
    emu.apc_no_count(1usize, 2121992u32, 28672u32, 2121996u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2122004u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2122008u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2122012u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2122016u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2122020u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2122024u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2122028u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122032u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122036u32;
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
pub fn block_0x00206134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2122040u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2122044u32);
    emu.apc_no_count(1usize, 2122044u32, 32768u32, 2122048u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122052u32;
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
pub fn block_0x00206144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2122056u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2122076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020615c));
    } else {
        emu.pc = 2122060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020614c));
    }
}
#[inline(always)]
pub fn block_0x0020614c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2122064u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2122068u32);
    emu.apc_no_count(6usize, 2122068u32, 4294950912u32, 2122072u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122076u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020615c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122080u32;
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
pub fn block_0x00206160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2122084u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122088u32;
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
pub fn block_0x00206168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122092u32;
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
pub fn block_0x0020616c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(129052672u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122096u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(257171456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122100u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4247375872u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122104u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(953024512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2122108u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2122112u32);
    emu.adi_no_count(12usize, 12usize, 1173u32, 2122116u32);
    emu.adi_no_count(13usize, 13usize, 1807u32, 2122120u32);
    emu.adi_no_count(14usize, 14usize, 937u32, 2122124u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2122128u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2122132u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2122136u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2122140u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122144u32;
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
pub fn block_0x002061a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2122148u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122152u32;
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
pub fn block_0x002061a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966720u32, 2122156u32);
    emu.sw_no_count(1usize, 2usize, 572u32, 2122160u32)?;
    emu.sw_no_count(8usize, 2usize, 568u32, 2122164u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2122168u32)?;
    emu.sw_no_count(18usize, 2usize, 560u32, 2122172u32)?;
    emu.sw_no_count(19usize, 2usize, 556u32, 2122176u32)?;
    emu.sw_no_count(20usize, 2usize, 552u32, 2122180u32)?;
    emu.sw_no_count(21usize, 2usize, 548u32, 2122184u32)?;
    emu.sw_no_count(22usize, 2usize, 544u32, 2122188u32)?;
    emu.sw_no_count(23usize, 2usize, 540u32, 2122192u32)?;
    emu.sw_no_count(24usize, 2usize, 536u32, 2122196u32)?;
    emu.sw_no_count(25usize, 2usize, 532u32, 2122200u32)?;
    emu.sw_no_count(26usize, 2usize, 528u32, 2122204u32)?;
    emu.sw_no_count(27usize, 2usize, 524u32, 2122208u32)?;
    emu.lw_no_count(23usize, 10usize, 0u32, 2122212u32)?;
    emu.lw_no_count(22usize, 10usize, 4u32, 2122216u32)?;
    emu.lw_no_count(21usize, 10usize, 8u32, 2122220u32)?;
    emu.lw_no_count(19usize, 10usize, 12u32, 2122224u32)?;
    emu.lw_no_count(30usize, 10usize, 16u32, 2122228u32)?;
    emu.lw_no_count(20usize, 10usize, 20u32, 2122232u32)?;
    emu.lw_no_count(26usize, 10usize, 24u32, 2122236u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2122240u32)?;
    emu.lw_no_count(9usize, 10usize, 28u32, 2122244u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2122252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020620c));
    } else {
        emu.pc = 2122248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206208));
    }
}
#[inline(always)]
pub fn block_0x00206208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2122252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139048u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3a8));
}
#[inline(never)]
pub fn block_0x0020620c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 195u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 6u32, 2122256u32);
    let a = 0u32.wrapping_add(607223808u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2122260u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1426882560u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2122264u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1925079040u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2122268u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2162077696u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2122272u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2614886400u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2122276u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3248222208u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2122280u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3835392000u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2122284u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4022222848u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2122288u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(264347648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2122292u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(604807168u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2122296u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(770256896u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2122300u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1249148928u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2122304u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1555083264u32);
    emu.write_reg_no_count(25usize, a);
    emu.pc = 2122308u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1996066816u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2122312u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2554220544u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2122316u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2821832704u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122320u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2952994816u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122324u32;
    emu.update_insn_clock();
    emu.adr_no_count(12usize, 11usize, 12usize, 2122328u32);
    emu.sw_no_count(12usize, 2usize, 268u32, 2122332u32)?;
    let a = 0u32.wrapping_add(1116352512u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122336u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967192u32, 2122340u32);
    emu.sw_no_count(12usize, 2usize, 264u32, 2122344u32)?;
    let a = 0u32.wrapping_add(1899446272u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122348u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1169u32, 2122352u32);
    emu.sw_no_count(12usize, 2usize, 260u32, 2122356u32)?;
    let a = 0u32.wrapping_add(3049324544u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122360u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966223u32, 2122364u32);
    emu.sw_no_count(12usize, 2usize, 256u32, 2122368u32)?;
    let a = 0u32.wrapping_add(3921010688u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122372u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966181u32, 2122376u32);
    emu.sw_no_count(12usize, 2usize, 252u32, 2122380u32)?;
    let a = 0u32.wrapping_add(961986560u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122384u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 603u32, 2122388u32);
    emu.sw_no_count(12usize, 2usize, 248u32, 2122392u32)?;
    let a = 0u32.wrapping_add(1508970496u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122396u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 497u32, 2122400u32);
    emu.sw_no_count(12usize, 2usize, 244u32, 2122404u32)?;
    let a = 0u32.wrapping_add(2453635072u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122408u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 676u32, 2122412u32);
    emu.sw_no_count(12usize, 2usize, 240u32, 2122416u32)?;
    let a = 0u32.wrapping_add(2870763520u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122420u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966997u32, 2122424u32);
    emu.sw_no_count(12usize, 2usize, 236u32, 2122428u32)?;
    let a = 0u32.wrapping_add(3624382464u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122432u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965912u32, 2122436u32);
    emu.sw_no_count(12usize, 2usize, 232u32, 2122440u32)?;
    let a = 0u32.wrapping_add(310599680u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2122444u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966017u32, 2122448u32);
    emu.sw_no_count(12usize, 2usize, 228u32, 2122452u32)?;
    emu.adi_no_count(12usize, 28usize, 1470u32, 2122456u32);
    emu.sw_no_count(12usize, 2usize, 224u32, 2122460u32)?;
    emu.adi_no_count(12usize, 29usize, 4294966723u32, 2122464u32);
    emu.sw_no_count(12usize, 2usize, 220u32, 2122468u32)?;
    emu.adi_no_count(12usize, 7usize, 4294966644u32, 2122472u32);
    emu.sw_no_count(12usize, 2usize, 216u32, 2122476u32)?;
    emu.adi_no_count(12usize, 31usize, 510u32, 2122480u32);
    emu.sw_no_count(12usize, 2usize, 212u32, 2122484u32)?;
    emu.adi_no_count(12usize, 8usize, 1703u32, 2122488u32);
    emu.sw_no_count(12usize, 2usize, 208u32, 2122492u32)?;
    emu.adi_no_count(12usize, 18usize, 372u32, 2122496u32);
    emu.sw_no_count(12usize, 2usize, 204u32, 2122500u32)?;
    emu.adi_no_count(12usize, 6usize, 4294965697u32, 2122504u32);
    emu.sw_no_count(12usize, 2usize, 200u32, 2122508u32)?;
    emu.adi_no_count(12usize, 5usize, 1926u32, 2122512u32);
    emu.sw_no_count(12usize, 2usize, 196u32, 2122516u32)?;
    emu.adi_no_count(12usize, 17usize, 4294966726u32, 2122520u32);
    emu.sw_no_count(12usize, 2usize, 192u32, 2122524u32)?;
    emu.adi_no_count(12usize, 16usize, 460u32, 2122528u32);
    emu.sw_no_count(12usize, 2usize, 188u32, 2122532u32)?;
    emu.adi_no_count(12usize, 15usize, 4294966383u32, 2122536u32);
    emu.sw_no_count(12usize, 2usize, 184u32, 2122540u32)?;
    emu.adi_no_count(12usize, 24usize, 1194u32, 2122544u32);
    emu.sw_no_count(12usize, 2usize, 180u32, 2122548u32)?;
    emu.adi_no_count(12usize, 25usize, 4294965724u32, 2122552u32);
    emu.sw_no_count(12usize, 2usize, 176u32, 2122556u32)?;
    emu.adi_no_count(12usize, 14usize, 4294965466u32, 2122560u32);
    emu.sw_no_count(12usize, 2usize, 172u32, 2122564u32)?;
    emu.adi_no_count(12usize, 27usize, 338u32, 2122568u32);
    emu.sw_no_count(12usize, 2usize, 168u32, 2122572u32)?;
    emu.adi_no_count(12usize, 13usize, 1645u32, 2122576u32);
    emu.sw_no_count(12usize, 2usize, 164u32, 2122580u32)?;
    emu.adi_no_count(10usize, 10usize, 1992u32, 2122584u32);
    emu.sw_no_count(10usize, 2usize, 160u32, 2122588u32)?;
    let a = 0u32.wrapping_add(3210313728u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122592u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967239u32, 2122596u32);
    emu.sw_no_count(10usize, 2usize, 156u32, 2122600u32)?;
    let a = 0u32.wrapping_add(3336572928u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122604u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966259u32, 2122608u32);
    emu.sw_no_count(10usize, 2usize, 152u32, 2122612u32)?;
    let a = 0u32.wrapping_add(3584528384u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122616u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 327u32, 2122620u32);
    emu.sw_no_count(10usize, 2usize, 148u32, 2122624u32)?;
    let a = 0u32.wrapping_add(113926144u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122628u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 849u32, 2122632u32);
    emu.sw_no_count(10usize, 2usize, 144u32, 2122636u32)?;
    let a = 0u32.wrapping_add(338243584u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122640u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965607u32, 2122644u32);
    emu.sw_no_count(10usize, 2usize, 140u32, 2122648u32)?;
    let a = 0u32.wrapping_add(666308608u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122652u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965893u32, 2122656u32);
    emu.sw_no_count(10usize, 2usize, 136u32, 2122660u32)?;
    let a = 0u32.wrapping_add(773529600u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122664u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 312u32, 2122668u32);
    emu.sw_no_count(10usize, 2usize, 132u32, 2122672u32)?;
    let a = 0u32.wrapping_add(1294757888u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122676u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966780u32, 2122680u32);
    emu.sw_no_count(10usize, 2usize, 128u32, 2122684u32)?;
    let a = 0u32.wrapping_add(1396183040u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122688u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966547u32, 2122692u32);
    emu.sw_no_count(10usize, 2usize, 124u32, 2122696u32)?;
    let a = 0u32.wrapping_add(1695182848u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122700u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 852u32, 2122704u32);
    emu.sw_no_count(10usize, 2usize, 120u32, 2122708u32)?;
    let a = 0u32.wrapping_add(1986662400u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122712u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965947u32, 2122716u32);
    emu.sw_no_count(10usize, 2usize, 116u32, 2122720u32)?;
    let a = 0u32.wrapping_add(2177028096u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122724u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965550u32, 2122728u32);
    emu.sw_no_count(10usize, 2usize, 112u32, 2122732u32)?;
    let a = 0u32.wrapping_add(2456956928u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122736u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966405u32, 2122740u32);
    emu.sw_no_count(10usize, 2usize, 108u32, 2122744u32)?;
    let a = 0u32.wrapping_add(2730487808u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122748u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965409u32, 2122752u32);
    emu.sw_no_count(10usize, 2usize, 104u32, 2122756u32)?;
    let a = 0u32.wrapping_add(2820300800u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122760u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1611u32, 2122764u32);
    emu.sw_no_count(10usize, 2usize, 100u32, 2122768u32)?;
    let a = 0u32.wrapping_add(3259731968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122772u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966128u32, 2122776u32);
    emu.sw_no_count(10usize, 2usize, 96u32, 2122780u32)?;
    let a = 0u32.wrapping_add(3345764352u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122784u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 419u32, 2122788u32);
    emu.sw_no_count(10usize, 2usize, 92u32, 2122792u32)?;
    let a = 0u32.wrapping_add(3516067840u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122796u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965273u32, 2122800u32);
    emu.sw_no_count(10usize, 2usize, 88u32, 2122804u32)?;
    let a = 0u32.wrapping_add(3600351232u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122808u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1572u32, 2122812u32);
    emu.sw_no_count(10usize, 2usize, 84u32, 2122816u32)?;
    let a = 0u32.wrapping_add(4094570496u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122820u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1413u32, 2122824u32);
    emu.sw_no_count(10usize, 2usize, 80u32, 2122828u32)?;
    let a = 0u32.wrapping_add(275423232u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122832u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 112u32, 2122836u32);
    emu.sw_no_count(10usize, 2usize, 76u32, 2122840u32)?;
    let a = 0u32.wrapping_add(430227456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122844u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 278u32, 2122848u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2122852u32)?;
    let a = 0u32.wrapping_add(506949632u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122856u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966280u32, 2122860u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2122864u32)?;
    let a = 0u32.wrapping_add(659058688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122868u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1868u32, 2122872u32);
    emu.sw_no_count(10usize, 2usize, 64u32, 2122876u32)?;
    let a = 0u32.wrapping_add(883998720u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122880u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966453u32, 2122884u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2122888u32)?;
    let a = 0u32.wrapping_add(958140416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122892u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966451u32, 2122896u32);
    emu.sw_no_count(10usize, 2usize, 56u32, 2122900u32)?;
    let a = 0u32.wrapping_add(1322823680u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122904u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965834u32, 2122908u32);
    emu.sw_no_count(10usize, 2usize, 52u32, 2122912u32)?;
    let a = 0u32.wrapping_add(1537003520u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122916u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965839u32, 2122920u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2122924u32)?;
    let a = 0u32.wrapping_add(1747873792u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122928u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967283u32, 2122932u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2122936u32)?;
    let a = 0u32.wrapping_add(1955561472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122940u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 750u32, 2122944u32);
    emu.sw_no_count(10usize, 2usize, 40u32, 2122948u32)?;
    let a = 0u32.wrapping_add(2024103936u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122952u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 879u32, 2122956u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2122960u32)?;
    let a = 0u32.wrapping_add(2227732480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122964u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965268u32, 2122968u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2122972u32)?;
    let a = 0u32.wrapping_add(2361851904u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122976u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 520u32, 2122980u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2122984u32)?;
    let a = 0u32.wrapping_add(2428436480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2122988u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967290u32, 2122992u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2122996u32)?;
    let a = 0u32.wrapping_add(2756734976u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2123000u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966507u32, 2123004u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2123008u32)?;
    let a = 0u32.wrapping_add(3204030464u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2123012u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1015u32, 2123016u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2123020u32)?;
    let a = 0u32.wrapping_add(3329327104u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2123024u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965490u32, 2123028u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2123032u32)?;
    emu.add_memory_rw_events(195usize);
    emu.pc = 2123032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206518));
}
#[inline(never)]
pub fn block_0x00206518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4003u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 2usize, 520u32, 2123036u32)?;
    emu.sw_no_count(21usize, 2usize, 512u32, 2123040u32)?;
    emu.sw_no_count(19usize, 2usize, 428u32, 2123044u32)?;
    emu.sw_no_count(20usize, 2usize, 516u32, 2123048u32)?;
    emu.sw_no_count(26usize, 2usize, 432u32, 2123052u32)?;
    emu.sw_no_count(9usize, 2usize, 372u32, 2123056u32)?;
    emu.lbu_no_count(16usize, 11usize, 0u32, 2123060u32);
    emu.lbu_no_count(12usize, 11usize, 1u32, 2123064u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2123068u32);
    emu.lbu_no_count(29usize, 11usize, 3u32, 2123072u32);
    emu.lbu_no_count(14usize, 11usize, 4u32, 2123076u32);
    emu.lbu_no_count(15usize, 11usize, 5u32, 2123080u32);
    emu.lbu_no_count(10usize, 11usize, 6u32, 2123084u32);
    emu.sw_no_count(10usize, 2usize, 484u32, 2123088u32)?;
    emu.lbu_no_count(10usize, 11usize, 7u32, 2123092u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2123096u32)?;
    emu.lbu_no_count(17usize, 11usize, 8u32, 2123100u32);
    emu.lbu_no_count(6usize, 11usize, 9u32, 2123104u32);
    emu.lbu_no_count(10usize, 11usize, 10u32, 2123108u32);
    emu.sw_no_count(10usize, 2usize, 420u32, 2123112u32)?;
    emu.lbu_no_count(10usize, 11usize, 11u32, 2123116u32);
    emu.sw_no_count(10usize, 2usize, 416u32, 2123120u32)?;
    emu.lbu_no_count(5usize, 11usize, 12u32, 2123124u32);
    emu.lbu_no_count(28usize, 11usize, 13u32, 2123128u32);
    emu.lbu_no_count(10usize, 11usize, 14u32, 2123132u32);
    emu.sw_no_count(10usize, 2usize, 500u32, 2123136u32)?;
    emu.lbu_no_count(22usize, 11usize, 15u32, 2123140u32);
    emu.sri_no_count(7usize, 30usize, 6u32, 2123144u32);
    emu.adi_no_count(10usize, 30usize, 0u32, 2123148u32);
    emu.sli_no_count(30usize, 30usize, 26u32, 2123152u32);
    emu.sri_no_count(31usize, 10usize, 11u32, 2123156u32);
    emu.sli_no_count(8usize, 10usize, 21u32, 2123160u32);
    emu.sri_no_count(18usize, 10usize, 25u32, 2123164u32);
    emu.sli_no_count(19usize, 10usize, 7u32, 2123168u32);
    emu.sw_no_count(10usize, 2usize, 424u32, 2123172u32)?;
    emu.sri_no_count(21usize, 23usize, 2u32, 2123176u32);
    emu.adi_no_count(27usize, 23usize, 0u32, 2123180u32);
    emu.sli_no_count(23usize, 23usize, 30u32, 2123184u32);
    emu.sri_no_count(24usize, 27usize, 13u32, 2123188u32);
    emu.sli_no_count(25usize, 27usize, 19u32, 2123192u32);
    emu.orr_no_count(7usize, 30usize, 7usize, 2123196u32);
    emu.sri_no_count(30usize, 27usize, 22u32, 2123200u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2123204u32);
    emu.sli_no_count(8usize, 27usize, 10u32, 2123208u32);
    emu.sw_no_count(27usize, 2usize, 508u32, 2123212u32)?;
    emu.orr_no_count(18usize, 19usize, 18usize, 2123216u32);
    emu.orr_no_count(19usize, 23usize, 21usize, 2123220u32);
    emu.orr_no_count(21usize, 25usize, 24usize, 2123224u32);
    emu.orr_no_count(30usize, 8usize, 30usize, 2123228u32);
    emu.lbu_no_count(23usize, 11usize, 16u32, 2123232u32);
    emu.lbu_no_count(1usize, 11usize, 17u32, 2123236u32);
    emu.lbu_no_count(25usize, 11usize, 18u32, 2123240u32);
    emu.lbu_no_count(24usize, 11usize, 19u32, 2123244u32);
    emu.xrr_no_count(7usize, 7usize, 31usize, 2123248u32);
    emu.lw_no_count(31usize, 2usize, 512u32, 2123252u32)?;
    emu.lw_no_count(8usize, 2usize, 520u32, 2123256u32)?;
    emu.xrr_no_count(31usize, 31usize, 8usize, 2123260u32);
    emu.xrr_no_count(8usize, 19usize, 21usize, 2123264u32);
    emu.lw_no_count(19usize, 2usize, 512u32, 2123268u32)?;
    emu.lw_no_count(21usize, 2usize, 520u32, 2123272u32)?;
    emu.anr_no_count(19usize, 19usize, 21usize, 2123276u32);
    emu.anr_no_count(31usize, 31usize, 27usize, 2123280u32);
    emu.xrr_no_count(31usize, 31usize, 19usize, 2123284u32);
    emu.xrr_no_count(19usize, 26usize, 20usize, 2123288u32);
    emu.anr_no_count(19usize, 19usize, 10usize, 2123292u32);
    emu.xrr_no_count(19usize, 19usize, 26usize, 2123296u32);
    emu.xrr_no_count(7usize, 7usize, 18usize, 2123300u32);
    emu.adr_no_count(19usize, 9usize, 19usize, 2123304u32);
    emu.xrr_no_count(30usize, 8usize, 30usize, 2123308u32);
    emu.adr_no_count(7usize, 19usize, 7usize, 2123312u32);
    emu.sw_no_count(7usize, 2usize, 496u32, 2123316u32)?;
    emu.adr_no_count(30usize, 31usize, 30usize, 2123320u32);
    emu.sw_no_count(30usize, 2usize, 504u32, 2123324u32)?;
    emu.lbu_no_count(30usize, 11usize, 20u32, 2123328u32);
    emu.lbu_no_count(31usize, 11usize, 21u32, 2123332u32);
    emu.lbu_no_count(10usize, 11usize, 22u32, 2123336u32);
    emu.sw_no_count(10usize, 2usize, 488u32, 2123340u32)?;
    emu.lbu_no_count(9usize, 11usize, 23u32, 2123344u32);
    emu.sli_no_count(13usize, 13usize, 8u32, 2123348u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2123352u32);
    emu.sli_no_count(7usize, 16usize, 24u32, 2123356u32);
    emu.sli_no_count(15usize, 15usize, 16u32, 2123360u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2123364u32);
    emu.sli_no_count(8usize, 6usize, 16u32, 2123368u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2123372u32);
    emu.orr_no_count(16usize, 13usize, 29usize, 2123376u32);
    emu.orr_no_count(26usize, 7usize, 12usize, 2123380u32);
    emu.orr_no_count(6usize, 14usize, 15usize, 2123384u32);
    emu.orr_no_count(8usize, 17usize, 8usize, 2123388u32);
    emu.lbu_no_count(10usize, 11usize, 24u32, 2123392u32);
    emu.lbu_no_count(12usize, 11usize, 25u32, 2123396u32);
    emu.lbu_no_count(13usize, 11usize, 26u32, 2123400u32);
    emu.sw_no_count(13usize, 2usize, 472u32, 2123404u32)?;
    emu.lbu_no_count(7usize, 11usize, 27u32, 2123408u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2123412u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123416u32);
    emu.sli_no_count(27usize, 1usize, 16u32, 2123420u32);
    emu.sli_no_count(23usize, 23usize, 24u32, 2123424u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2123428u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2123432u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2123436u32);
    emu.sli_no_count(15usize, 10usize, 24u32, 2123440u32);
    emu.orr_no_count(10usize, 5usize, 28usize, 2123444u32);
    emu.orr_no_count(18usize, 23usize, 27usize, 2123448u32);
    emu.orr_no_count(13usize, 30usize, 31usize, 2123452u32);
    emu.lbu_no_count(14usize, 11usize, 28u32, 2123456u32);
    emu.lbu_no_count(17usize, 11usize, 29u32, 2123460u32);
    emu.orr_no_count(15usize, 15usize, 12usize, 2123464u32);
    emu.lbu_no_count(12usize, 11usize, 30u32, 2123468u32);
    emu.sw_no_count(12usize, 2usize, 468u32, 2123472u32)?;
    emu.lbu_no_count(31usize, 11usize, 31u32, 2123476u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2123480u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2123484u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2123488u32);
    emu.sw_no_count(11usize, 2usize, 368u32, 2123492u32)?;
    emu.lbu_no_count(17usize, 11usize, 33u32, 2123496u32);
    emu.lbu_no_count(5usize, 11usize, 32u32, 2123500u32);
    emu.lbu_no_count(23usize, 11usize, 34u32, 2123504u32);
    emu.lbu_no_count(12usize, 11usize, 35u32, 2123508u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2123512u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123516u32);
    emu.orr_no_count(1usize, 5usize, 17usize, 2123520u32);
    emu.lbu_no_count(17usize, 11usize, 37u32, 2123524u32);
    emu.lbu_no_count(5usize, 11usize, 36u32, 2123528u32);
    emu.lbu_no_count(28usize, 11usize, 38u32, 2123532u32);
    emu.sw_no_count(28usize, 2usize, 480u32, 2123536u32)?;
    emu.lbu_no_count(28usize, 11usize, 39u32, 2123540u32);
    emu.sw_no_count(28usize, 2usize, 440u32, 2123544u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2123548u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123552u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2123556u32);
    emu.sw_no_count(17usize, 2usize, 444u32, 2123560u32)?;
    emu.lbu_no_count(17usize, 11usize, 41u32, 2123564u32);
    emu.lbu_no_count(5usize, 11usize, 40u32, 2123568u32);
    emu.lbu_no_count(28usize, 11usize, 42u32, 2123572u32);
    emu.sw_no_count(28usize, 2usize, 448u32, 2123576u32)?;
    emu.lbu_no_count(28usize, 11usize, 43u32, 2123580u32);
    emu.sw_no_count(28usize, 2usize, 412u32, 2123584u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2123588u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123592u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2123596u32);
    emu.sw_no_count(17usize, 2usize, 436u32, 2123600u32)?;
    emu.lbu_no_count(17usize, 11usize, 45u32, 2123604u32);
    emu.lbu_no_count(5usize, 11usize, 44u32, 2123608u32);
    emu.lbu_no_count(28usize, 11usize, 46u32, 2123612u32);
    emu.sw_no_count(28usize, 2usize, 452u32, 2123616u32)?;
    emu.lbu_no_count(28usize, 11usize, 47u32, 2123620u32);
    emu.sw_no_count(28usize, 2usize, 404u32, 2123624u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2123628u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123632u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2123636u32);
    emu.sw_no_count(17usize, 2usize, 408u32, 2123640u32)?;
    emu.lbu_no_count(17usize, 11usize, 49u32, 2123644u32);
    emu.lbu_no_count(5usize, 11usize, 48u32, 2123648u32);
    emu.lbu_no_count(28usize, 11usize, 50u32, 2123652u32);
    emu.sw_no_count(28usize, 2usize, 396u32, 2123656u32)?;
    emu.lbu_no_count(28usize, 11usize, 51u32, 2123660u32);
    emu.sw_no_count(28usize, 2usize, 392u32, 2123664u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2123668u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123672u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2123676u32);
    emu.sw_no_count(17usize, 2usize, 400u32, 2123680u32)?;
    emu.lbu_no_count(17usize, 11usize, 53u32, 2123684u32);
    emu.lbu_no_count(5usize, 11usize, 52u32, 2123688u32);
    emu.lbu_no_count(29usize, 11usize, 54u32, 2123692u32);
    emu.sw_no_count(29usize, 2usize, 476u32, 2123696u32)?;
    emu.lbu_no_count(29usize, 11usize, 55u32, 2123700u32);
    emu.sw_no_count(29usize, 2usize, 456u32, 2123704u32)?;
    emu.sli_no_count(17usize, 17usize, 16u32, 2123708u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123712u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2123716u32);
    emu.sw_no_count(17usize, 2usize, 460u32, 2123720u32)?;
    emu.lbu_no_count(17usize, 11usize, 57u32, 2123724u32);
    emu.lbu_no_count(30usize, 11usize, 56u32, 2123728u32);
    emu.lbu_no_count(5usize, 11usize, 58u32, 2123732u32);
    emu.lbu_no_count(20usize, 11usize, 59u32, 2123736u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2123740u32);
    emu.sli_no_count(30usize, 30usize, 24u32, 2123744u32);
    emu.orr_no_count(19usize, 30usize, 17usize, 2123748u32);
    emu.lbu_no_count(30usize, 11usize, 61u32, 2123752u32);
    emu.lbu_no_count(27usize, 11usize, 60u32, 2123756u32);
    emu.lbu_no_count(17usize, 11usize, 62u32, 2123760u32);
    emu.lbu_no_count(28usize, 11usize, 63u32, 2123764u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2123768u32);
    emu.sli_no_count(27usize, 27usize, 24u32, 2123772u32);
    emu.orr_no_count(21usize, 27usize, 30usize, 2123776u32);
    emu.orr_no_count(11usize, 26usize, 16usize, 2123780u32);
    emu.sw_no_count(11usize, 2usize, 492u32, 2123784u32)?;
    emu.sli_no_count(25usize, 25usize, 8u32, 2123788u32);
    emu.orr_no_count(11usize, 25usize, 24usize, 2123792u32);
    emu.sli_no_count(25usize, 24usize, 25u32, 2123796u32);
    emu.orr_no_count(24usize, 18usize, 11usize, 2123800u32);
    emu.sri_no_count(11usize, 24usize, 7u32, 2123804u32);
    emu.sw_no_count(24usize, 2usize, 384u32, 2123808u32)?;
    emu.orr_no_count(11usize, 25usize, 11usize, 2123812u32);
    emu.sw_no_count(11usize, 2usize, 376u32, 2123816u32)?;
    emu.lw_no_count(29usize, 2usize, 500u32, 2123820u32)?;
    emu.sli_no_count(29usize, 29usize, 8u32, 2123824u32);
    emu.orr_no_count(11usize, 29usize, 22usize, 2123828u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2123832u32);
    emu.orr_no_count(30usize, 10usize, 11usize, 2123836u32);
    emu.sri_no_count(11usize, 30usize, 7u32, 2123840u32);
    emu.sw_no_count(30usize, 2usize, 500u32, 2123844u32)?;
    emu.orr_no_count(11usize, 29usize, 11usize, 2123848u32);
    emu.sw_no_count(11usize, 2usize, 364u32, 2123852u32)?;
    emu.lw_no_count(11usize, 2usize, 420u32, 2123856u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2123860u32);
    emu.lw_no_count(29usize, 2usize, 416u32, 2123864u32)?;
    emu.orr_no_count(11usize, 11usize, 29usize, 2123868u32);
    emu.sli_no_count(29usize, 29usize, 25u32, 2123872u32);
    emu.orr_no_count(22usize, 8usize, 11usize, 2123876u32);
    emu.sri_no_count(11usize, 22usize, 7u32, 2123880u32);
    emu.sw_no_count(22usize, 2usize, 388u32, 2123884u32)?;
    emu.orr_no_count(11usize, 29usize, 11usize, 2123888u32);
    emu.sw_no_count(11usize, 2usize, 360u32, 2123892u32)?;
    emu.lw_no_count(11usize, 2usize, 484u32, 2123896u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2123900u32);
    emu.lw_no_count(16usize, 2usize, 464u32, 2123904u32)?;
    emu.orr_no_count(11usize, 11usize, 16usize, 2123908u32);
    emu.sli_no_count(16usize, 16usize, 25u32, 2123912u32);
    emu.orr_no_count(29usize, 6usize, 11usize, 2123916u32);
    emu.sri_no_count(11usize, 29usize, 7u32, 2123920u32);
    emu.sw_no_count(29usize, 2usize, 380u32, 2123924u32)?;
    emu.orr_no_count(11usize, 16usize, 11usize, 2123928u32);
    emu.sw_no_count(11usize, 2usize, 356u32, 2123932u32)?;
    emu.sri_no_count(11usize, 18usize, 18u32, 2123936u32);
    emu.sli_no_count(16usize, 24usize, 14u32, 2123940u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2123944u32);
    emu.sw_no_count(11usize, 2usize, 352u32, 2123948u32)?;
    emu.sri_no_count(10usize, 10usize, 18u32, 2123952u32);
    emu.sli_no_count(11usize, 30usize, 14u32, 2123956u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2123960u32);
    emu.sw_no_count(10usize, 2usize, 348u32, 2123964u32)?;
    emu.sri_no_count(8usize, 8usize, 18u32, 2123968u32);
    emu.sli_no_count(10usize, 22usize, 14u32, 2123972u32);
    emu.orr_no_count(10usize, 10usize, 8usize, 2123976u32);
    emu.sw_no_count(10usize, 2usize, 344u32, 2123980u32)?;
    emu.sri_no_count(10usize, 6usize, 18u32, 2123984u32);
    emu.sli_no_count(11usize, 29usize, 14u32, 2123988u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2123992u32);
    emu.sw_no_count(10usize, 2usize, 340u32, 2123996u32)?;
    emu.sli_no_count(5usize, 5usize, 8u32, 2124000u32);
    emu.orr_no_count(10usize, 5usize, 20usize, 2124004u32);
    emu.adi_no_count(8usize, 20usize, 0u32, 2124008u32);
    emu.orr_no_count(5usize, 19usize, 10usize, 2124012u32);
    emu.sri_no_count(10usize, 19usize, 17u32, 2124016u32);
    emu.sli_no_count(11usize, 5usize, 15u32, 2124020u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2124024u32);
    emu.sw_no_count(10usize, 2usize, 336u32, 2124028u32)?;
    emu.sri_no_count(10usize, 19usize, 19u32, 2124032u32);
    emu.adi_no_count(18usize, 19usize, 0u32, 2124036u32);
    emu.sli_no_count(11usize, 5usize, 13u32, 2124040u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2124044u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2124048u32)?;
    emu.sli_no_count(17usize, 17usize, 8u32, 2124052u32);
    emu.adi_no_count(25usize, 28usize, 0u32, 2124056u32);
    emu.orr_no_count(10usize, 17usize, 28usize, 2124060u32);
    emu.orr_no_count(16usize, 21usize, 10usize, 2124064u32);
    emu.sri_no_count(10usize, 21usize, 17u32, 2124068u32);
    emu.sli_no_count(11usize, 16usize, 15u32, 2124072u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2124076u32);
    emu.sw_no_count(10usize, 2usize, 328u32, 2124080u32)?;
    emu.sri_no_count(10usize, 21usize, 19u32, 2124084u32);
    emu.sli_no_count(11usize, 16usize, 13u32, 2124088u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2124092u32);
    emu.sw_no_count(10usize, 2usize, 324u32, 2124096u32)?;
    emu.sli_no_count(10usize, 23usize, 8u32, 2124100u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2124104u32);
    emu.sli_no_count(12usize, 12usize, 25u32, 2124108u32);
    emu.orr_no_count(17usize, 1usize, 10usize, 2124112u32);
    emu.sri_no_count(10usize, 17usize, 7u32, 2124116u32);
    emu.sw_no_count(17usize, 2usize, 484u32, 2124120u32)?;
    emu.orr_no_count(10usize, 12usize, 10usize, 2124124u32);
    emu.sw_no_count(10usize, 2usize, 320u32, 2124128u32)?;
    emu.lw_no_count(10usize, 2usize, 468u32, 2124132u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2124136u32);
    emu.orr_no_count(10usize, 10usize, 31usize, 2124140u32);
    emu.sli_no_count(31usize, 31usize, 25u32, 2124144u32);
    emu.orr_no_count(12usize, 14usize, 10usize, 2124148u32);
    emu.sri_no_count(10usize, 12usize, 7u32, 2124152u32);
    emu.sw_no_count(12usize, 2usize, 468u32, 2124156u32)?;
    emu.orr_no_count(10usize, 31usize, 10usize, 2124160u32);
    emu.sw_no_count(10usize, 2usize, 316u32, 2124164u32)?;
    emu.lw_no_count(10usize, 2usize, 472u32, 2124168u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2124172u32);
    emu.orr_no_count(10usize, 10usize, 7usize, 2124176u32);
    emu.sli_no_count(7usize, 7usize, 25u32, 2124180u32);
    emu.orr_no_count(6usize, 15usize, 10usize, 2124184u32);
    emu.sri_no_count(10usize, 6usize, 7u32, 2124188u32);
    emu.sw_no_count(6usize, 2usize, 464u32, 2124192u32)?;
    emu.orr_no_count(10usize, 7usize, 10usize, 2124196u32);
    emu.sw_no_count(10usize, 2usize, 312u32, 2124200u32)?;
    emu.lw_no_count(10usize, 2usize, 488u32, 2124204u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2124208u32);
    emu.orr_no_count(10usize, 10usize, 9usize, 2124212u32);
    emu.sli_no_count(11usize, 9usize, 25u32, 2124216u32);
    emu.orr_no_count(7usize, 13usize, 10usize, 2124220u32);
    emu.sri_no_count(10usize, 7usize, 7u32, 2124224u32);
    emu.sw_no_count(7usize, 2usize, 472u32, 2124228u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2124232u32);
    emu.sw_no_count(10usize, 2usize, 308u32, 2124236u32)?;
    emu.sri_no_count(10usize, 1usize, 18u32, 2124240u32);
    emu.sli_no_count(11usize, 17usize, 14u32, 2124244u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2124248u32);
    emu.sw_no_count(10usize, 2usize, 304u32, 2124252u32)?;
    emu.sri_no_count(14usize, 14usize, 18u32, 2124256u32);
    emu.sli_no_count(10usize, 12usize, 14u32, 2124260u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2124264u32);
    emu.sw_no_count(10usize, 2usize, 300u32, 2124268u32)?;
    emu.sri_no_count(15usize, 15usize, 18u32, 2124272u32);
    emu.sli_no_count(10usize, 6usize, 14u32, 2124276u32);
    emu.orr_no_count(20usize, 10usize, 15usize, 2124280u32);
    emu.sri_no_count(13usize, 13usize, 18u32, 2124284u32);
    emu.sli_no_count(10usize, 7usize, 14u32, 2124288u32);
    emu.orr_no_count(28usize, 10usize, 13usize, 2124292u32);
    emu.lw_no_count(10usize, 2usize, 396u32, 2124296u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2124300u32);
    emu.lw_no_count(23usize, 2usize, 392u32, 2124304u32)?;
    emu.orr_no_count(10usize, 10usize, 23usize, 2124308u32);
    emu.sli_no_count(23usize, 23usize, 25u32, 2124312u32);
    emu.lw_no_count(13usize, 2usize, 400u32, 2124316u32)?;
    emu.orr_no_count(7usize, 13usize, 10usize, 2124320u32);
    emu.sri_no_count(10usize, 7usize, 7u32, 2124324u32);
    emu.sw_no_count(7usize, 2usize, 488u32, 2124328u32)?;
    emu.orr_no_count(23usize, 23usize, 10usize, 2124332u32);
    emu.lw_no_count(10usize, 2usize, 452u32, 2124336u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2124340u32);
    emu.lw_no_count(11usize, 2usize, 404u32, 2124344u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2124348u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2124352u32);
    emu.lw_no_count(6usize, 2usize, 408u32, 2124356u32)?;
    emu.orr_no_count(12usize, 6usize, 10usize, 2124360u32);
    emu.sri_no_count(10usize, 12usize, 7u32, 2124364u32);
    emu.sw_no_count(12usize, 2usize, 452u32, 2124368u32)?;
    emu.orr_no_count(19usize, 11usize, 10usize, 2124372u32);
    emu.lw_no_count(10usize, 2usize, 448u32, 2124376u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2124380u32);
    emu.lw_no_count(11usize, 2usize, 412u32, 2124384u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2124388u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2124392u32);
    emu.lw_no_count(17usize, 2usize, 436u32, 2124396u32)?;
    emu.orr_no_count(30usize, 17usize, 10usize, 2124400u32);
    emu.sri_no_count(15usize, 30usize, 7u32, 2124404u32);
    emu.sw_no_count(30usize, 2usize, 448u32, 2124408u32)?;
    emu.orr_no_count(31usize, 11usize, 15usize, 2124412u32);
    emu.lw_no_count(10usize, 2usize, 480u32, 2124416u32)?;
    emu.sli_no_count(10usize, 10usize, 8u32, 2124420u32);
    emu.lw_no_count(11usize, 2usize, 440u32, 2124424u32)?;
    emu.orr_no_count(10usize, 10usize, 11usize, 2124428u32);
    emu.sli_no_count(11usize, 11usize, 25u32, 2124432u32);
    emu.lw_no_count(15usize, 2usize, 444u32, 2124436u32)?;
    emu.orr_no_count(26usize, 15usize, 10usize, 2124440u32);
    emu.sri_no_count(14usize, 26usize, 7u32, 2124444u32);
    emu.sw_no_count(26usize, 2usize, 480u32, 2124448u32)?;
    emu.orr_no_count(9usize, 11usize, 14usize, 2124452u32);
    emu.sri_no_count(10usize, 13usize, 18u32, 2124456u32);
    emu.sli_no_count(13usize, 7usize, 14u32, 2124460u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2124464u32);
    emu.sri_no_count(10usize, 6usize, 18u32, 2124468u32);
    emu.sli_no_count(12usize, 12usize, 14u32, 2124472u32);
    emu.orr_no_count(29usize, 12usize, 10usize, 2124476u32);
    emu.sri_no_count(10usize, 17usize, 18u32, 2124480u32);
    emu.sli_no_count(11usize, 30usize, 14u32, 2124484u32);
    emu.orr_no_count(24usize, 11usize, 10usize, 2124488u32);
    emu.sri_no_count(11usize, 15usize, 18u32, 2124492u32);
    emu.sli_no_count(10usize, 26usize, 14u32, 2124496u32);
    emu.orr_no_count(1usize, 10usize, 11usize, 2124500u32);
    emu.sli_no_count(11usize, 25usize, 25u32, 2124504u32);
    emu.sri_no_count(27usize, 16usize, 7u32, 2124508u32);
    emu.orr_no_count(27usize, 11usize, 27usize, 2124512u32);
    emu.sli_no_count(11usize, 8usize, 25u32, 2124516u32);
    emu.adi_no_count(6usize, 5usize, 0u32, 2124520u32);
    emu.sri_no_count(26usize, 5usize, 7u32, 2124524u32);
    emu.orr_no_count(26usize, 11usize, 26usize, 2124528u32);
    emu.lw_no_count(11usize, 2usize, 476u32, 2124532u32)?;
    emu.sli_no_count(11usize, 11usize, 8u32, 2124536u32);
    emu.lw_no_count(10usize, 2usize, 456u32, 2124540u32)?;
    emu.orr_no_count(11usize, 11usize, 10usize, 2124544u32);
    emu.sli_no_count(10usize, 10usize, 25u32, 2124548u32);
    emu.lw_no_count(13usize, 2usize, 460u32, 2124552u32)?;
    emu.orr_no_count(8usize, 13usize, 11usize, 2124556u32);
    emu.sri_no_count(11usize, 8usize, 7u32, 2124560u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2124564u32);
    emu.sri_no_count(11usize, 21usize, 18u32, 2124568u32);
    emu.sli_no_count(21usize, 16usize, 14u32, 2124572u32);
    emu.orr_no_count(21usize, 21usize, 11usize, 2124576u32);
    emu.sri_no_count(11usize, 18usize, 18u32, 2124580u32);
    emu.sli_no_count(25usize, 5usize, 14u32, 2124584u32);
    emu.orr_no_count(25usize, 25usize, 11usize, 2124588u32);
    emu.sri_no_count(11usize, 13usize, 18u32, 2124592u32);
    emu.sli_no_count(18usize, 8usize, 14u32, 2124596u32);
    emu.orr_no_count(12usize, 18usize, 11usize, 2124600u32);
    emu.lw_no_count(11usize, 2usize, 376u32, 2124604u32)?;
    emu.lw_no_count(13usize, 2usize, 352u32, 2124608u32)?;
    emu.xrr_no_count(30usize, 11usize, 13usize, 2124612u32);
    emu.lw_no_count(11usize, 2usize, 364u32, 2124616u32)?;
    emu.lw_no_count(13usize, 2usize, 348u32, 2124620u32)?;
    emu.xrr_no_count(11usize, 11usize, 13usize, 2124624u32);
    emu.lw_no_count(13usize, 2usize, 360u32, 2124628u32)?;
    emu.lw_no_count(14usize, 2usize, 344u32, 2124632u32)?;
    emu.xrr_no_count(13usize, 13usize, 14usize, 2124636u32);
    emu.lw_no_count(14usize, 2usize, 356u32, 2124640u32)?;
    emu.lw_no_count(15usize, 2usize, 340u32, 2124644u32)?;
    emu.xrr_no_count(14usize, 14usize, 15usize, 2124648u32);
    emu.lw_no_count(15usize, 2usize, 336u32, 2124652u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2124656u32)?;
    emu.xrr_no_count(15usize, 15usize, 17usize, 2124660u32);
    emu.lw_no_count(17usize, 2usize, 328u32, 2124664u32)?;
    emu.lw_no_count(5usize, 2usize, 324u32, 2124668u32)?;
    emu.xrr_no_count(17usize, 17usize, 5usize, 2124672u32);
    emu.lw_no_count(5usize, 2usize, 320u32, 2124676u32)?;
    emu.lw_no_count(7usize, 2usize, 304u32, 2124680u32)?;
    emu.xrr_no_count(5usize, 5usize, 7usize, 2124684u32);
    emu.lw_no_count(7usize, 2usize, 316u32, 2124688u32)?;
    emu.lw_no_count(18usize, 2usize, 300u32, 2124692u32)?;
    emu.xrr_no_count(7usize, 7usize, 18usize, 2124696u32);
    emu.lw_no_count(18usize, 2usize, 312u32, 2124700u32)?;
    emu.xrr_no_count(18usize, 18usize, 20usize, 2124704u32);
    emu.lw_no_count(20usize, 2usize, 308u32, 2124708u32)?;
    emu.xrr_no_count(20usize, 20usize, 28usize, 2124712u32);
    emu.xrr_no_count(22usize, 23usize, 22usize, 2124716u32);
    emu.xrr_no_count(23usize, 19usize, 29usize, 2124720u32);
    emu.xrr_no_count(28usize, 31usize, 24usize, 2124724u32);
    emu.sw_no_count(28usize, 2usize, 436u32, 2124728u32)?;
    emu.xrr_no_count(28usize, 9usize, 1usize, 2124732u32);
    emu.sw_no_count(28usize, 2usize, 456u32, 2124736u32)?;
    emu.xrr_no_count(28usize, 27usize, 21usize, 2124740u32);
    emu.sw_no_count(28usize, 2usize, 356u32, 2124744u32)?;
    emu.xrr_no_count(28usize, 26usize, 25usize, 2124748u32);
    emu.sw_no_count(28usize, 2usize, 364u32, 2124752u32)?;
    emu.xrr_no_count(10usize, 10usize, 12usize, 2124756u32);
    emu.sw_no_count(10usize, 2usize, 396u32, 2124760u32)?;
    emu.lw_no_count(10usize, 2usize, 264u32, 2124764u32)?;
    emu.lw_no_count(12usize, 2usize, 496u32, 2124768u32)?;
    emu.adr_no_count(10usize, 12usize, 10usize, 2124772u32);
    emu.lw_no_count(9usize, 2usize, 492u32, 2124776u32)?;
    emu.adr_no_count(10usize, 10usize, 9usize, 2124780u32);
    emu.lw_no_count(12usize, 2usize, 504u32, 2124784u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2124788u32);
    emu.lw_no_count(31usize, 2usize, 428u32, 2124792u32)?;
    emu.adr_no_count(28usize, 10usize, 31usize, 2124796u32);
    emu.sri_no_count(10usize, 28usize, 6u32, 2124800u32);
    emu.sli_no_count(29usize, 28usize, 26u32, 2124804u32);
    emu.orr_no_count(10usize, 29usize, 10usize, 2124808u32);
    emu.sw_no_count(10usize, 2usize, 496u32, 2124812u32)?;
    emu.sri_no_count(10usize, 28usize, 11u32, 2124816u32);
    emu.sli_no_count(29usize, 28usize, 21u32, 2124820u32);
    emu.orr_no_count(10usize, 29usize, 10usize, 2124824u32);
    emu.sw_no_count(10usize, 2usize, 504u32, 2124828u32)?;
    emu.sri_no_count(10usize, 28usize, 25u32, 2124832u32);
    emu.sli_no_count(29usize, 28usize, 7u32, 2124836u32);
    emu.sw_no_count(28usize, 2usize, 444u32, 2124840u32)?;
    emu.orr_no_count(10usize, 29usize, 10usize, 2124844u32);
    emu.sw_no_count(10usize, 2usize, 440u32, 2124848u32)?;
    emu.adi_no_count(31usize, 12usize, 0u32, 2124852u32);
    emu.sw_no_count(12usize, 2usize, 360u32, 2124856u32)?;
    emu.sri_no_count(29usize, 12usize, 2u32, 2124860u32);
    emu.sli_no_count(27usize, 12usize, 30u32, 2124864u32);
    emu.orr_no_count(1usize, 27usize, 29usize, 2124868u32);
    emu.sri_no_count(29usize, 12usize, 13u32, 2124872u32);
    emu.sli_no_count(27usize, 12usize, 19u32, 2124876u32);
    emu.orr_no_count(19usize, 27usize, 29usize, 2124880u32);
    emu.sri_no_count(29usize, 12usize, 22u32, 2124884u32);
    emu.sli_no_count(27usize, 12usize, 10u32, 2124888u32);
    emu.orr_no_count(10usize, 27usize, 29usize, 2124892u32);
    emu.sw_no_count(10usize, 2usize, 408u32, 2124896u32)?;
    emu.lw_no_count(12usize, 2usize, 520u32, 2124900u32)?;
    emu.lw_no_count(10usize, 2usize, 508u32, 2124904u32)?;
    emu.xrr_no_count(27usize, 12usize, 10usize, 2124908u32);
    emu.anr_no_count(27usize, 31usize, 27usize, 2124912u32);
    emu.lw_no_count(12usize, 2usize, 520u32, 2124916u32)?;
    emu.anr_no_count(24usize, 12usize, 10usize, 2124920u32);
    emu.xrr_no_count(10usize, 27usize, 24usize, 2124924u32);
    emu.sw_no_count(10usize, 2usize, 376u32, 2124928u32)?;
    emu.lw_no_count(31usize, 2usize, 384u32, 2124932u32)?;
    emu.sri_no_count(24usize, 31usize, 3u32, 2124936u32);
    emu.xrr_no_count(26usize, 30usize, 24usize, 2124940u32);
    emu.lw_no_count(29usize, 2usize, 500u32, 2124944u32)?;
    emu.sri_no_count(24usize, 29usize, 3u32, 2124948u32);
    emu.xrr_no_count(21usize, 11usize, 24usize, 2124952u32);
    emu.lw_no_count(30usize, 2usize, 388u32, 2124956u32)?;
    emu.sri_no_count(24usize, 30usize, 3u32, 2124960u32);
    emu.xrr_no_count(24usize, 13usize, 24usize, 2124964u32);
    emu.lw_no_count(10usize, 2usize, 380u32, 2124968u32)?;
    emu.sri_no_count(13usize, 10usize, 3u32, 2124972u32);
    emu.xrr_no_count(25usize, 14usize, 13usize, 2124976u32);
    emu.sri_no_count(14usize, 6usize, 10u32, 2124980u32);
    emu.xrr_no_count(14usize, 15usize, 14usize, 2124984u32);
    emu.sw_no_count(14usize, 2usize, 352u32, 2124988u32)?;
    emu.sri_no_count(15usize, 16usize, 10u32, 2124992u32);
    emu.xrr_no_count(27usize, 17usize, 15usize, 2124996u32);
    emu.lw_no_count(15usize, 2usize, 484u32, 2125000u32)?;
    emu.sri_no_count(15usize, 15usize, 3u32, 2125004u32);
    emu.xrr_no_count(11usize, 5usize, 15usize, 2125008u32);
    emu.sw_no_count(11usize, 2usize, 476u32, 2125012u32)?;
    emu.lw_no_count(11usize, 2usize, 468u32, 2125016u32)?;
    emu.sri_no_count(17usize, 11usize, 3u32, 2125020u32);
    emu.xrr_no_count(17usize, 7usize, 17usize, 2125024u32);
    emu.lw_no_count(14usize, 2usize, 464u32, 2125028u32)?;
    emu.sri_no_count(5usize, 14usize, 3u32, 2125032u32);
    emu.xrr_no_count(18usize, 18usize, 5usize, 2125036u32);
    emu.lw_no_count(13usize, 2usize, 472u32, 2125040u32)?;
    emu.sri_no_count(5usize, 13usize, 3u32, 2125044u32);
    emu.xrr_no_count(20usize, 20usize, 5usize, 2125048u32);
    emu.lw_no_count(12usize, 2usize, 488u32, 2125052u32)?;
    emu.sri_no_count(5usize, 12usize, 3u32, 2125056u32);
    emu.xrr_no_count(11usize, 22usize, 5usize, 2125060u32);
    emu.sw_no_count(11usize, 2usize, 460u32, 2125064u32)?;
    emu.lw_no_count(11usize, 2usize, 452u32, 2125068u32)?;
    emu.sri_no_count(5usize, 11usize, 3u32, 2125072u32);
    emu.xrr_no_count(15usize, 23usize, 5usize, 2125076u32);
    emu.sw_no_count(15usize, 2usize, 400u32, 2125080u32)?;
    emu.lw_no_count(22usize, 2usize, 448u32, 2125084u32)?;
    emu.sri_no_count(5usize, 22usize, 3u32, 2125088u32);
    emu.lw_no_count(15usize, 2usize, 436u32, 2125092u32)?;
    emu.xrr_no_count(15usize, 15usize, 5usize, 2125096u32);
    emu.sw_no_count(15usize, 2usize, 392u32, 2125100u32)?;
    emu.lw_no_count(23usize, 2usize, 480u32, 2125104u32)?;
    emu.sri_no_count(5usize, 23usize, 3u32, 2125108u32);
    emu.lw_no_count(15usize, 2usize, 456u32, 2125112u32)?;
    emu.xrr_no_count(15usize, 15usize, 5usize, 2125116u32);
    emu.sw_no_count(15usize, 2usize, 404u32, 2125120u32)?;
    emu.sw_no_count(16usize, 2usize, 420u32, 2125124u32)?;
    emu.sri_no_count(7usize, 16usize, 3u32, 2125128u32);
    emu.lw_no_count(15usize, 2usize, 356u32, 2125132u32)?;
    emu.xrr_no_count(5usize, 15usize, 7usize, 2125136u32);
    emu.sw_no_count(5usize, 2usize, 436u32, 2125140u32)?;
    emu.adi_no_count(5usize, 6usize, 0u32, 2125144u32);
    emu.sw_no_count(6usize, 2usize, 416u32, 2125148u32)?;
    emu.sri_no_count(7usize, 6usize, 3u32, 2125152u32);
    emu.lw_no_count(15usize, 2usize, 364u32, 2125156u32)?;
    emu.xrr_no_count(15usize, 15usize, 7usize, 2125160u32);
    emu.sw_no_count(15usize, 2usize, 456u32, 2125164u32)?;
    emu.sw_no_count(8usize, 2usize, 412u32, 2125168u32)?;
    emu.sri_no_count(6usize, 8usize, 3u32, 2125172u32);
    emu.lw_no_count(15usize, 2usize, 396u32, 2125176u32)?;
    emu.xrr_no_count(15usize, 15usize, 6usize, 2125180u32);
    emu.sw_no_count(15usize, 2usize, 396u32, 2125184u32)?;
    emu.lw_no_count(15usize, 2usize, 504u32, 2125188u32)?;
    emu.lw_no_count(6usize, 2usize, 496u32, 2125192u32)?;
    emu.xrr_no_count(6usize, 6usize, 15usize, 2125196u32);
    emu.lw_no_count(7usize, 2usize, 516u32, 2125200u32)?;
    emu.lw_no_count(15usize, 2usize, 424u32, 2125204u32)?;
    emu.xrr_no_count(7usize, 7usize, 15usize, 2125208u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2125212u32);
    emu.lw_no_count(28usize, 2usize, 516u32, 2125216u32)?;
    emu.xrr_no_count(7usize, 7usize, 28usize, 2125220u32);
    emu.lw_no_count(28usize, 2usize, 432u32, 2125224u32)?;
    emu.adr_no_count(28usize, 28usize, 10usize, 2125228u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2125232u32);
    emu.xrr_no_count(19usize, 1usize, 19usize, 2125236u32);
    emu.adr_no_count(28usize, 29usize, 12usize, 2125240u32);
    emu.adr_no_count(28usize, 26usize, 28usize, 2125244u32);
    emu.sw_no_count(28usize, 2usize, 504u32, 2125248u32)?;
    emu.adr_no_count(28usize, 30usize, 11usize, 2125252u32);
    emu.adi_no_count(11usize, 30usize, 0u32, 2125256u32);
    emu.adr_no_count(28usize, 21usize, 28usize, 2125260u32);
    emu.sw_no_count(28usize, 2usize, 496u32, 2125264u32)?;
    emu.adr_no_count(28usize, 10usize, 22usize, 2125268u32);
    emu.adr_no_count(28usize, 24usize, 28usize, 2125272u32);
    emu.adr_no_count(30usize, 9usize, 23usize, 2125276u32);
    emu.adr_no_count(30usize, 25usize, 30usize, 2125280u32);
    emu.adr_no_count(23usize, 14usize, 16usize, 2125284u32);
    emu.adr_no_count(23usize, 17usize, 23usize, 2125288u32);
    emu.adr_no_count(1usize, 13usize, 5usize, 2125292u32);
    emu.adr_no_count(1usize, 18usize, 1usize, 2125296u32);
    emu.adr_no_count(17usize, 31usize, 8usize, 2125300u32);
    emu.adr_no_count(12usize, 20usize, 17usize, 2125304u32);
    emu.lw_no_count(10usize, 2usize, 440u32, 2125308u32)?;
    emu.xrr_no_count(17usize, 6usize, 10usize, 2125312u32);
    emu.lw_no_count(10usize, 2usize, 408u32, 2125316u32)?;
    emu.xrr_no_count(6usize, 19usize, 10usize, 2125320u32);
    emu.lw_no_count(13usize, 2usize, 352u32, 2125324u32)?;
    emu.adr_no_count(22usize, 30usize, 13usize, 2125328u32);
    emu.adr_no_count(16usize, 28usize, 27usize, 2125332u32);
    emu.lw_no_count(10usize, 2usize, 260u32, 2125336u32)?;
    emu.adr_no_count(10usize, 7usize, 10usize, 2125340u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2125344u32);
    emu.lw_no_count(27usize, 2usize, 376u32, 2125348u32)?;
    emu.adr_no_count(27usize, 6usize, 27usize, 2125352u32);
    emu.sri_no_count(13usize, 22usize, 17u32, 2125356u32);
    emu.sli_no_count(14usize, 22usize, 15u32, 2125360u32);
    emu.orr_no_count(29usize, 14usize, 13usize, 2125364u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2125368u32);
    emu.sli_no_count(14usize, 22usize, 13u32, 2125372u32);
    emu.orr_no_count(30usize, 14usize, 13usize, 2125376u32);
    emu.sri_no_count(13usize, 16usize, 17u32, 2125380u32);
    emu.sli_no_count(14usize, 16usize, 15u32, 2125384u32);
    emu.orr_no_count(18usize, 14usize, 13usize, 2125388u32);
    emu.sri_no_count(13usize, 16usize, 19u32, 2125392u32);
    emu.sli_no_count(14usize, 16usize, 13u32, 2125396u32);
    emu.orr_no_count(17usize, 14usize, 13usize, 2125400u32);
    emu.sri_no_count(13usize, 22usize, 7u32, 2125404u32);
    emu.sli_no_count(14usize, 22usize, 25u32, 2125408u32);
    emu.orr_no_count(6usize, 14usize, 13usize, 2125412u32);
    emu.sri_no_count(13usize, 22usize, 18u32, 2125416u32);
    emu.sli_no_count(14usize, 22usize, 14u32, 2125420u32);
    emu.orr_no_count(7usize, 14usize, 13usize, 2125424u32);
    emu.adi_no_count(5usize, 16usize, 0u32, 2125428u32);
    emu.sri_no_count(13usize, 16usize, 7u32, 2125432u32);
    emu.sli_no_count(14usize, 16usize, 25u32, 2125436u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2125440u32);
    emu.sri_no_count(14usize, 16usize, 18u32, 2125444u32);
    emu.sli_no_count(16usize, 16usize, 14u32, 2125448u32);
    emu.adi_no_count(24usize, 5usize, 0u32, 2125452u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2125456u32);
    emu.xrr_no_count(16usize, 29usize, 30usize, 2125460u32);
    emu.xrr_no_count(17usize, 18usize, 17usize, 2125464u32);
    emu.xrr_no_count(6usize, 6usize, 7usize, 2125468u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2125472u32);
    emu.adr_no_count(27usize, 27usize, 10usize, 2125476u32);
    emu.lw_no_count(14usize, 2usize, 512u32, 2125480u32)?;
    emu.adr_no_count(10usize, 10usize, 14usize, 2125484u32);
    emu.sri_no_count(14usize, 10usize, 6u32, 2125488u32);
    emu.sli_no_count(7usize, 10usize, 26u32, 2125492u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2125496u32);
    emu.sri_no_count(7usize, 10usize, 11u32, 2125500u32);
    emu.sli_no_count(28usize, 10usize, 21u32, 2125504u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2125508u32);
    emu.sri_no_count(28usize, 10usize, 25u32, 2125512u32);
    emu.sli_no_count(29usize, 10usize, 7u32, 2125516u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2125520u32);
    emu.sri_no_count(29usize, 27usize, 2u32, 2125524u32);
    emu.sli_no_count(30usize, 27usize, 30u32, 2125528u32);
    emu.orr_no_count(30usize, 30usize, 29usize, 2125532u32);
    emu.sri_no_count(29usize, 27usize, 13u32, 2125536u32);
    emu.sli_no_count(18usize, 27usize, 19u32, 2125540u32);
    emu.orr_no_count(18usize, 18usize, 29usize, 2125544u32);
    emu.sri_no_count(29usize, 27usize, 22u32, 2125548u32);
    emu.sli_no_count(19usize, 27usize, 10u32, 2125552u32);
    emu.orr_no_count(19usize, 19usize, 29usize, 2125556u32);
    emu.lw_no_count(8usize, 2usize, 508u32, 2125560u32)?;
    emu.lw_no_count(26usize, 2usize, 360u32, 2125564u32)?;
    emu.xrr_no_count(29usize, 26usize, 8usize, 2125568u32);
    emu.anr_no_count(29usize, 27usize, 29usize, 2125572u32);
    emu.anr_no_count(20usize, 26usize, 8usize, 2125576u32);
    emu.xrr_no_count(20usize, 29usize, 20usize, 2125580u32);
    emu.adi_no_count(5usize, 22usize, 0u32, 2125584u32);
    emu.sri_no_count(29usize, 22usize, 10u32, 2125588u32);
    emu.xrr_no_count(16usize, 16usize, 29usize, 2125592u32);
    emu.sw_no_count(24usize, 2usize, 492u32, 2125596u32)?;
    emu.sri_no_count(29usize, 24usize, 10u32, 2125600u32);
    emu.xrr_no_count(17usize, 17usize, 29usize, 2125604u32);
    emu.sri_no_count(29usize, 22usize, 3u32, 2125608u32);
    emu.sw_no_count(22usize, 2usize, 408u32, 2125612u32)?;
    emu.xrr_no_count(6usize, 6usize, 29usize, 2125616u32);
    emu.sw_no_count(6usize, 2usize, 364u32, 2125620u32)?;
    emu.sri_no_count(6usize, 24usize, 3u32, 2125624u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2125628u32);
    emu.sw_no_count(13usize, 2usize, 352u32, 2125632u32)?;
    emu.xrr_no_count(13usize, 14usize, 7usize, 2125636u32);
    emu.lw_no_count(14usize, 2usize, 516u32, 2125640u32)?;
    emu.adr_no_count(14usize, 14usize, 11usize, 2125644u32);
    emu.adi_no_count(7usize, 15usize, 0u32, 2125648u32);
    emu.lw_no_count(15usize, 2usize, 444u32, 2125652u32)?;
    emu.xrr_no_count(6usize, 15usize, 7usize, 2125656u32);
    emu.anr_no_count(6usize, 10usize, 6usize, 2125660u32);
    emu.xrr_no_count(6usize, 6usize, 7usize, 2125664u32);
    emu.adi_no_count(21usize, 7usize, 0u32, 2125668u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2125672u32);
    emu.xrr_no_count(6usize, 30usize, 18usize, 2125676u32);
    emu.lw_no_count(29usize, 2usize, 496u32, 2125680u32)?;
    emu.adr_no_count(29usize, 29usize, 16usize, 2125684u32);
    emu.sw_no_count(29usize, 2usize, 496u32, 2125688u32)?;
    emu.lw_no_count(9usize, 2usize, 504u32, 2125692u32)?;
    emu.adr_no_count(18usize, 9usize, 17usize, 2125696u32);
    emu.xrr_no_count(11usize, 13usize, 28usize, 2125700u32);
    emu.xrr_no_count(13usize, 6usize, 19usize, 2125704u32);
    emu.lw_no_count(16usize, 2usize, 256u32, 2125708u32)?;
    emu.adr_no_count(14usize, 14usize, 16usize, 2125712u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2125716u32);
    emu.adr_no_count(13usize, 13usize, 20usize, 2125720u32);
    emu.sri_no_count(14usize, 29usize, 17u32, 2125724u32);
    emu.sli_no_count(16usize, 29usize, 15u32, 2125728u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2125732u32);
    emu.sri_no_count(16usize, 29usize, 19u32, 2125736u32);
    emu.sli_no_count(17usize, 29usize, 13u32, 2125740u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2125744u32);
    emu.sri_no_count(17usize, 18usize, 17u32, 2125748u32);
    emu.sli_no_count(6usize, 18usize, 15u32, 2125752u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2125756u32);
    emu.sri_no_count(6usize, 18usize, 19u32, 2125760u32);
    emu.sli_no_count(7usize, 18usize, 13u32, 2125764u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2125768u32);
    emu.sri_no_count(7usize, 18usize, 7u32, 2125772u32);
    emu.sli_no_count(28usize, 18usize, 25u32, 2125776u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2125780u32);
    emu.sri_no_count(28usize, 29usize, 7u32, 2125784u32);
    emu.sli_no_count(30usize, 29usize, 25u32, 2125788u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2125792u32);
    emu.sri_no_count(30usize, 18usize, 18u32, 2125796u32);
    emu.sli_no_count(9usize, 18usize, 14u32, 2125800u32);
    emu.adi_no_count(19usize, 18usize, 0u32, 2125804u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2125808u32);
    emu.sri_no_count(9usize, 29usize, 18u32, 2125812u32);
    emu.sli_no_count(18usize, 29usize, 14u32, 2125816u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2125820u32);
    emu.xrr_no_count(14usize, 14usize, 16usize, 2125824u32);
    emu.xrr_no_count(16usize, 17usize, 6usize, 2125828u32);
    emu.xrr_no_count(17usize, 7usize, 30usize, 2125832u32);
    emu.xrr_no_count(6usize, 28usize, 9usize, 2125836u32);
    emu.sri_no_count(7usize, 29usize, 10u32, 2125840u32);
    emu.xrr_no_count(7usize, 14usize, 7usize, 2125844u32);
    emu.sw_no_count(19usize, 2usize, 440u32, 2125848u32)?;
    emu.sri_no_count(14usize, 19usize, 10u32, 2125852u32);
    emu.xrr_no_count(16usize, 16usize, 14usize, 2125856u32);
    emu.sri_no_count(14usize, 19usize, 3u32, 2125860u32);
    emu.xrr_no_count(14usize, 17usize, 14usize, 2125864u32);
    emu.sw_no_count(14usize, 2usize, 388u32, 2125868u32)?;
    emu.sri_no_count(14usize, 29usize, 3u32, 2125872u32);
    emu.xrr_no_count(14usize, 6usize, 14usize, 2125876u32);
    emu.sw_no_count(14usize, 2usize, 348u32, 2125880u32)?;
    emu.adr_no_count(29usize, 13usize, 11usize, 2125884u32);
    emu.lw_no_count(22usize, 2usize, 520u32, 2125888u32)?;
    emu.adr_no_count(22usize, 11usize, 22usize, 2125892u32);
    emu.sri_no_count(11usize, 22usize, 6u32, 2125896u32);
    emu.sli_no_count(13usize, 22usize, 26u32, 2125900u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2125904u32);
    emu.sri_no_count(11usize, 22usize, 11u32, 2125908u32);
    emu.sli_no_count(17usize, 22usize, 21u32, 2125912u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2125916u32);
    emu.sri_no_count(11usize, 22usize, 25u32, 2125920u32);
    emu.sli_no_count(6usize, 22usize, 7u32, 2125924u32);
    emu.orr_no_count(6usize, 6usize, 11usize, 2125928u32);
    emu.adr_no_count(24usize, 15usize, 31usize, 2125932u32);
    emu.xrr_no_count(11usize, 10usize, 15usize, 2125936u32);
    emu.anr_no_count(11usize, 22usize, 11usize, 2125940u32);
    emu.xrr_no_count(28usize, 11usize, 15usize, 2125944u32);
    emu.sri_no_count(11usize, 29usize, 2u32, 2125948u32);
    emu.sli_no_count(30usize, 29usize, 30u32, 2125952u32);
    emu.orr_no_count(30usize, 30usize, 11usize, 2125956u32);
    emu.sri_no_count(11usize, 29usize, 13u32, 2125960u32);
    emu.sli_no_count(31usize, 29usize, 19u32, 2125964u32);
    emu.orr_no_count(31usize, 31usize, 11usize, 2125968u32);
    emu.sri_no_count(11usize, 29usize, 22u32, 2125972u32);
    emu.sli_no_count(9usize, 29usize, 10u32, 2125976u32);
    emu.orr_no_count(9usize, 9usize, 11usize, 2125980u32);
    emu.xrr_no_count(11usize, 27usize, 26usize, 2125984u32);
    emu.anr_no_count(11usize, 29usize, 11usize, 2125988u32);
    emu.anr_no_count(18usize, 27usize, 26usize, 2125992u32);
    emu.xrr_no_count(11usize, 11usize, 18usize, 2125996u32);
    emu.adr_no_count(18usize, 12usize, 7usize, 2126000u32);
    emu.adr_no_count(1usize, 1usize, 16usize, 2126004u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2126008u32);
    emu.lw_no_count(12usize, 2usize, 500u32, 2126012u32)?;
    emu.adr_no_count(12usize, 21usize, 12usize, 2126016u32);
    emu.adr_no_count(12usize, 12usize, 28usize, 2126020u32);
    emu.xrr_no_count(16usize, 30usize, 31usize, 2126024u32);
    emu.xrr_no_count(31usize, 13usize, 6usize, 2126028u32);
    emu.xrr_no_count(7usize, 16usize, 9usize, 2126032u32);
    emu.adi_no_count(6usize, 18usize, 0u32, 2126036u32);
    emu.sri_no_count(13usize, 18usize, 17u32, 2126040u32);
    emu.sli_no_count(16usize, 18usize, 15u32, 2126044u32);
    emu.orr_no_count(18usize, 16usize, 13usize, 2126048u32);
    emu.sri_no_count(13usize, 6usize, 19u32, 2126052u32);
    emu.sli_no_count(16usize, 6usize, 13u32, 2126056u32);
    emu.orr_no_count(20usize, 16usize, 13usize, 2126060u32);
    emu.sri_no_count(13usize, 1usize, 17u32, 2126064u32);
    emu.sli_no_count(16usize, 1usize, 15u32, 2126068u32);
    emu.orr_no_count(21usize, 16usize, 13usize, 2126072u32);
    emu.sri_no_count(13usize, 1usize, 19u32, 2126076u32);
    emu.sli_no_count(16usize, 1usize, 13u32, 2126080u32);
    emu.orr_no_count(25usize, 16usize, 13usize, 2126084u32);
    emu.sri_no_count(13usize, 6usize, 7u32, 2126088u32);
    emu.sli_no_count(16usize, 6usize, 25u32, 2126092u32);
    emu.orr_no_count(17usize, 16usize, 13usize, 2126096u32);
    emu.sri_no_count(13usize, 6usize, 18u32, 2126100u32);
    emu.sli_no_count(16usize, 6usize, 14u32, 2126104u32);
    emu.adi_no_count(30usize, 6usize, 0u32, 2126108u32);
    emu.orr_no_count(6usize, 16usize, 13usize, 2126112u32);
    emu.sri_no_count(13usize, 1usize, 7u32, 2126116u32);
    emu.sli_no_count(16usize, 1usize, 25u32, 2126120u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2126124u32);
    emu.sri_no_count(16usize, 1usize, 18u32, 2126128u32);
    emu.sli_no_count(28usize, 1usize, 14u32, 2126132u32);
    emu.sw_no_count(1usize, 2usize, 444u32, 2126136u32)?;
    emu.orr_no_count(16usize, 28usize, 16usize, 2126140u32);
    emu.lw_no_count(28usize, 2usize, 252u32, 2126144u32)?;
    emu.adr_no_count(12usize, 12usize, 28usize, 2126148u32);
    emu.adr_no_count(12usize, 12usize, 31usize, 2126152u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2126156u32);
    emu.xrr_no_count(7usize, 18usize, 20usize, 2126160u32);
    emu.xrr_no_count(28usize, 21usize, 25usize, 2126164u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2126168u32);
    emu.xrr_no_count(13usize, 13usize, 16usize, 2126172u32);
    emu.sw_no_count(30usize, 2usize, 504u32, 2126176u32)?;
    emu.sri_no_count(16usize, 30usize, 10u32, 2126180u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2126184u32);
    emu.sri_no_count(6usize, 1usize, 10u32, 2126188u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2126192u32);
    emu.sri_no_count(7usize, 30usize, 3u32, 2126196u32);
    emu.xrr_no_count(14usize, 17usize, 7usize, 2126200u32);
    emu.sw_no_count(14usize, 2usize, 332u32, 2126204u32)?;
    emu.sri_no_count(17usize, 1usize, 3u32, 2126208u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2126212u32);
    emu.sw_no_count(13usize, 2usize, 376u32, 2126216u32)?;
    emu.adr_no_count(23usize, 23usize, 16usize, 2126220u32);
    emu.lw_no_count(15usize, 2usize, 476u32, 2126224u32)?;
    emu.lw_no_count(13usize, 2usize, 468u32, 2126228u32)?;
    emu.adr_no_count(15usize, 15usize, 13usize, 2126232u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2126236u32);
    emu.adr_no_count(5usize, 15usize, 6usize, 2126240u32);
    emu.adr_no_count(1usize, 11usize, 12usize, 2126244u32);
    emu.adr_no_count(20usize, 12usize, 8usize, 2126248u32);
    emu.sri_no_count(11usize, 20usize, 6u32, 2126252u32);
    emu.sli_no_count(12usize, 20usize, 26u32, 2126256u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2126260u32);
    emu.sri_no_count(12usize, 20usize, 11u32, 2126264u32);
    emu.sli_no_count(13usize, 20usize, 21u32, 2126268u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2126272u32);
    emu.sri_no_count(13usize, 20usize, 25u32, 2126276u32);
    emu.sli_no_count(15usize, 20usize, 7u32, 2126280u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2126284u32);
    emu.lw_no_count(7usize, 2usize, 472u32, 2126288u32)?;
    emu.adr_no_count(7usize, 7usize, 10usize, 2126292u32);
    emu.xrr_no_count(15usize, 22usize, 10usize, 2126296u32);
    emu.anr_no_count(15usize, 20usize, 15usize, 2126300u32);
    emu.xrr_no_count(10usize, 15usize, 10usize, 2126304u32);
    emu.sri_no_count(15usize, 1usize, 2u32, 2126308u32);
    emu.sli_no_count(16usize, 1usize, 30u32, 2126312u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2126316u32);
    emu.sri_no_count(16usize, 1usize, 13u32, 2126320u32);
    emu.sli_no_count(17usize, 1usize, 19u32, 2126324u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2126328u32);
    emu.sri_no_count(17usize, 1usize, 22u32, 2126332u32);
    emu.sli_no_count(6usize, 1usize, 10u32, 2126336u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2126340u32);
    emu.xrr_no_count(6usize, 29usize, 27usize, 2126344u32);
    emu.anr_no_count(6usize, 1usize, 6usize, 2126348u32);
    emu.anr_no_count(28usize, 29usize, 27usize, 2126352u32);
    emu.xrr_no_count(6usize, 6usize, 28usize, 2126356u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2126360u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2126364u32);
    emu.xrr_no_count(12usize, 15usize, 16usize, 2126368u32);
    emu.adi_no_count(24usize, 23usize, 0u32, 2126372u32);
    emu.sri_no_count(15usize, 23usize, 17u32, 2126376u32);
    emu.sli_no_count(16usize, 23usize, 15u32, 2126380u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2126384u32);
    emu.sri_no_count(16usize, 23usize, 19u32, 2126388u32);
    emu.sli_no_count(28usize, 23usize, 13u32, 2126392u32);
    emu.orr_no_count(16usize, 28usize, 16usize, 2126396u32);
    emu.sri_no_count(28usize, 5usize, 17u32, 2126400u32);
    emu.sli_no_count(30usize, 5usize, 15u32, 2126404u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2126408u32);
    emu.sri_no_count(30usize, 5usize, 19u32, 2126412u32);
    emu.sli_no_count(31usize, 5usize, 13u32, 2126416u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2126420u32);
    emu.sri_no_count(31usize, 5usize, 7u32, 2126424u32);
    emu.sli_no_count(9usize, 5usize, 25u32, 2126428u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2126432u32);
    emu.sri_no_count(9usize, 23usize, 7u32, 2126436u32);
    emu.sli_no_count(18usize, 23usize, 25u32, 2126440u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2126444u32);
    emu.sri_no_count(18usize, 5usize, 18u32, 2126448u32);
    emu.sli_no_count(19usize, 5usize, 14u32, 2126452u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2126456u32);
    emu.sri_no_count(19usize, 23usize, 18u32, 2126460u32);
    emu.sli_no_count(23usize, 23usize, 14u32, 2126464u32);
    emu.orr_no_count(19usize, 23usize, 19usize, 2126468u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2126472u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2126476u32);
    emu.xrr_no_count(13usize, 15usize, 16usize, 2126480u32);
    emu.xrr_no_count(15usize, 28usize, 30usize, 2126484u32);
    emu.xrr_no_count(16usize, 31usize, 18usize, 2126488u32);
    emu.xrr_no_count(17usize, 9usize, 19usize, 2126492u32);
    emu.lw_no_count(28usize, 2usize, 248u32, 2126496u32)?;
    emu.adr_no_count(10usize, 10usize, 28usize, 2126500u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2126504u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2126508u32);
    emu.sw_no_count(24usize, 2usize, 500u32, 2126512u32)?;
    emu.sri_no_count(11usize, 24usize, 10u32, 2126516u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2126520u32);
    emu.sw_no_count(5usize, 2usize, 476u32, 2126524u32)?;
    emu.sri_no_count(13usize, 5usize, 10u32, 2126528u32);
    emu.xrr_no_count(13usize, 15usize, 13usize, 2126532u32);
    emu.sri_no_count(15usize, 5usize, 3u32, 2126536u32);
    emu.xrr_no_count(14usize, 16usize, 15usize, 2126540u32);
    emu.sw_no_count(14usize, 2usize, 384u32, 2126544u32)?;
    emu.sri_no_count(15usize, 24usize, 3u32, 2126548u32);
    emu.xrr_no_count(14usize, 17usize, 15usize, 2126552u32);
    emu.sw_no_count(14usize, 2usize, 324u32, 2126556u32)?;
    emu.adr_no_count(21usize, 12usize, 10usize, 2126560u32);
    emu.adr_no_count(19usize, 10usize, 26usize, 2126564u32);
    emu.lw_no_count(5usize, 2usize, 484u32, 2126568u32)?;
    emu.lw_no_count(10usize, 2usize, 404u32, 2126572u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2126576u32);
    emu.lw_no_count(10usize, 2usize, 492u32, 2126580u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2126584u32);
    emu.adr_no_count(24usize, 5usize, 11usize, 2126588u32);
    emu.lw_no_count(8usize, 2usize, 480u32, 2126592u32)?;
    emu.lw_no_count(10usize, 2usize, 392u32, 2126596u32)?;
    emu.adr_no_count(8usize, 10usize, 8usize, 2126600u32);
    emu.lw_no_count(10usize, 2usize, 496u32, 2126604u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2126608u32);
    emu.adr_no_count(26usize, 8usize, 13usize, 2126612u32);
    emu.sri_no_count(10usize, 19usize, 6u32, 2126616u32);
    emu.sli_no_count(13usize, 19usize, 26u32, 2126620u32);
    emu.orr_no_count(13usize, 13usize, 10usize, 2126624u32);
    emu.sri_no_count(10usize, 19usize, 11u32, 2126628u32);
    emu.sli_no_count(15usize, 19usize, 21u32, 2126632u32);
    emu.orr_no_count(15usize, 15usize, 10usize, 2126636u32);
    emu.sri_no_count(10usize, 19usize, 25u32, 2126640u32);
    emu.sli_no_count(11usize, 19usize, 7u32, 2126644u32);
    emu.orr_no_count(6usize, 11usize, 10usize, 2126648u32);
    emu.lw_no_count(10usize, 2usize, 464u32, 2126652u32)?;
    emu.adr_no_count(10usize, 10usize, 22usize, 2126656u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2126660u32)?;
    emu.xrr_no_count(10usize, 20usize, 22usize, 2126664u32);
    emu.anr_no_count(10usize, 19usize, 10usize, 2126668u32);
    emu.xrr_no_count(16usize, 10usize, 22usize, 2126672u32);
    emu.sri_no_count(10usize, 21usize, 2u32, 2126676u32);
    emu.sli_no_count(11usize, 21usize, 30u32, 2126680u32);
    emu.orr_no_count(31usize, 11usize, 10usize, 2126684u32);
    emu.sri_no_count(10usize, 21usize, 13u32, 2126688u32);
    emu.sli_no_count(11usize, 21usize, 19u32, 2126692u32);
    emu.orr_no_count(30usize, 11usize, 10usize, 2126696u32);
    emu.sri_no_count(10usize, 21usize, 22u32, 2126700u32);
    emu.sli_no_count(11usize, 21usize, 10u32, 2126704u32);
    emu.orr_no_count(5usize, 11usize, 10usize, 2126708u32);
    emu.xrr_no_count(10usize, 1usize, 29usize, 2126712u32);
    emu.anr_no_count(10usize, 21usize, 10usize, 2126716u32);
    emu.anr_no_count(11usize, 1usize, 29usize, 2126720u32);
    emu.xrr_no_count(8usize, 10usize, 11usize, 2126724u32);
    emu.sri_no_count(10usize, 24usize, 17u32, 2126728u32);
    emu.sli_no_count(11usize, 24usize, 15u32, 2126732u32);
    emu.orr_no_count(18usize, 11usize, 10usize, 2126736u32);
    emu.sri_no_count(10usize, 24usize, 19u32, 2126740u32);
    emu.sli_no_count(11usize, 24usize, 13u32, 2126744u32);
    emu.orr_no_count(22usize, 11usize, 10usize, 2126748u32);
    emu.sri_no_count(10usize, 26usize, 17u32, 2126752u32);
    emu.sli_no_count(11usize, 26usize, 15u32, 2126756u32);
    emu.orr_no_count(25usize, 11usize, 10usize, 2126760u32);
    emu.sri_no_count(10usize, 26usize, 19u32, 2126764u32);
    emu.sli_no_count(12usize, 26usize, 13u32, 2126768u32);
    emu.orr_no_count(12usize, 12usize, 10usize, 2126772u32);
    emu.sri_no_count(10usize, 24usize, 7u32, 2126776u32);
    emu.sli_no_count(11usize, 24usize, 25u32, 2126780u32);
    emu.orr_no_count(17usize, 11usize, 10usize, 2126784u32);
    emu.sri_no_count(10usize, 24usize, 18u32, 2126788u32);
    emu.sli_no_count(28usize, 24usize, 14u32, 2126792u32);
    emu.orr_no_count(11usize, 28usize, 10usize, 2126796u32);
    emu.adi_no_count(10usize, 26usize, 0u32, 2126800u32);
    emu.sri_no_count(28usize, 26usize, 7u32, 2126804u32);
    emu.sli_no_count(9usize, 26usize, 25u32, 2126808u32);
    emu.orr_no_count(28usize, 9usize, 28usize, 2126812u32);
    emu.sri_no_count(9usize, 26usize, 18u32, 2126816u32);
    emu.sli_no_count(26usize, 26usize, 14u32, 2126820u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2126824u32);
    emu.orr_no_count(9usize, 26usize, 9usize, 2126828u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2126832u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2126836u32);
    emu.xrr_no_count(15usize, 31usize, 30usize, 2126840u32);
    emu.xrr_no_count(7usize, 18usize, 22usize, 2126844u32);
    emu.xrr_no_count(12usize, 25usize, 12usize, 2126848u32);
    emu.xrr_no_count(10usize, 17usize, 11usize, 2126852u32);
    emu.xrr_no_count(11usize, 28usize, 9usize, 2126856u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2126860u32);
    emu.xrr_no_count(15usize, 15usize, 5usize, 2126864u32);
    emu.sri_no_count(5usize, 24usize, 10u32, 2126868u32);
    emu.xrr_no_count(5usize, 7usize, 5usize, 2126872u32);
    emu.sw_no_count(23usize, 2usize, 472u32, 2126876u32)?;
    emu.sri_no_count(6usize, 23usize, 10u32, 2126880u32);
    emu.xrr_no_count(12usize, 12usize, 6usize, 2126884u32);
    emu.sri_no_count(6usize, 24usize, 3u32, 2126888u32);
    emu.sw_no_count(24usize, 2usize, 404u32, 2126892u32)?;
    emu.xrr_no_count(10usize, 10usize, 6usize, 2126896u32);
    emu.sw_no_count(10usize, 2usize, 340u32, 2126900u32)?;
    emu.sri_no_count(10usize, 23usize, 3u32, 2126904u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2126908u32);
    emu.sw_no_count(10usize, 2usize, 360u32, 2126912u32)?;
    emu.lw_no_count(10usize, 2usize, 244u32, 2126916u32)?;
    emu.adr_no_count(16usize, 16usize, 10usize, 2126920u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2126924u32);
    emu.adr_no_count(10usize, 15usize, 8usize, 2126928u32);
    emu.lw_no_count(11usize, 2usize, 448u32, 2126932u32)?;
    emu.lw_no_count(14usize, 2usize, 400u32, 2126936u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2126940u32);
    emu.lw_no_count(14usize, 2usize, 440u32, 2126944u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2126948u32);
    emu.adr_no_count(28usize, 11usize, 5usize, 2126952u32);
    emu.lw_no_count(11usize, 2usize, 452u32, 2126956u32)?;
    emu.lw_no_count(14usize, 2usize, 460u32, 2126960u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2126964u32);
    emu.lw_no_count(15usize, 2usize, 504u32, 2126968u32)?;
    emu.adr_no_count(11usize, 11usize, 15usize, 2126972u32);
    emu.adr_no_count(14usize, 11usize, 12usize, 2126976u32);
    emu.adr_no_count(23usize, 10usize, 13usize, 2126980u32);
    emu.adr_no_count(27usize, 13usize, 27usize, 2126984u32);
    emu.sri_no_count(11usize, 28usize, 17u32, 2126988u32);
    emu.sli_no_count(12usize, 28usize, 15u32, 2126992u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2126996u32);
    emu.sri_no_count(12usize, 28usize, 19u32, 2127000u32);
    emu.sli_no_count(13usize, 28usize, 13u32, 2127004u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2127008u32);
    emu.sri_no_count(13usize, 14usize, 17u32, 2127012u32);
    emu.sli_no_count(15usize, 14usize, 15u32, 2127016u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2127020u32);
    emu.sri_no_count(15usize, 14usize, 19u32, 2127024u32);
    emu.sli_no_count(16usize, 14usize, 13u32, 2127028u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2127032u32);
    emu.sri_no_count(16usize, 14usize, 7u32, 2127036u32);
    emu.sli_no_count(17usize, 14usize, 25u32, 2127040u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2127044u32);
    emu.sri_no_count(17usize, 28usize, 7u32, 2127048u32);
    emu.sli_no_count(5usize, 28usize, 25u32, 2127052u32);
    emu.orr_no_count(5usize, 5usize, 17usize, 2127056u32);
    emu.sri_no_count(17usize, 14usize, 18u32, 2127060u32);
    emu.sli_no_count(6usize, 14usize, 14u32, 2127064u32);
    emu.orr_no_count(6usize, 6usize, 17usize, 2127068u32);
    emu.sri_no_count(17usize, 28usize, 18u32, 2127072u32);
    emu.sli_no_count(7usize, 28usize, 14u32, 2127076u32);
    emu.adi_no_count(10usize, 28usize, 0u32, 2127080u32);
    emu.orr_no_count(7usize, 7usize, 17usize, 2127084u32);
    emu.sri_no_count(17usize, 27usize, 6u32, 2127088u32);
    emu.sli_no_count(28usize, 27usize, 26u32, 2127092u32);
    emu.orr_no_count(28usize, 28usize, 17usize, 2127096u32);
    emu.sri_no_count(17usize, 27usize, 11u32, 2127100u32);
    emu.sli_no_count(30usize, 27usize, 21u32, 2127104u32);
    emu.orr_no_count(30usize, 30usize, 17usize, 2127108u32);
    emu.sri_no_count(17usize, 27usize, 25u32, 2127112u32);
    emu.sli_no_count(31usize, 27usize, 7u32, 2127116u32);
    emu.orr_no_count(31usize, 31usize, 17usize, 2127120u32);
    emu.lw_no_count(17usize, 2usize, 468u32, 2127124u32)?;
    emu.adr_no_count(17usize, 17usize, 20usize, 2127128u32);
    emu.xrr_no_count(9usize, 19usize, 20usize, 2127132u32);
    emu.anr_no_count(9usize, 27usize, 9usize, 2127136u32);
    emu.xrr_no_count(9usize, 9usize, 20usize, 2127140u32);
    emu.sri_no_count(18usize, 23usize, 2u32, 2127144u32);
    emu.sli_no_count(20usize, 23usize, 30u32, 2127148u32);
    emu.orr_no_count(18usize, 20usize, 18usize, 2127152u32);
    emu.sri_no_count(20usize, 23usize, 13u32, 2127156u32);
    emu.sli_no_count(22usize, 23usize, 19u32, 2127160u32);
    emu.orr_no_count(20usize, 22usize, 20usize, 2127164u32);
    emu.sri_no_count(22usize, 23usize, 22u32, 2127168u32);
    emu.sli_no_count(25usize, 23usize, 10u32, 2127172u32);
    emu.orr_no_count(22usize, 25usize, 22usize, 2127176u32);
    emu.xrr_no_count(25usize, 21usize, 1usize, 2127180u32);
    emu.anr_no_count(25usize, 23usize, 25usize, 2127184u32);
    emu.anr_no_count(26usize, 21usize, 1usize, 2127188u32);
    emu.xrr_no_count(25usize, 25usize, 26usize, 2127192u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2127196u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2127200u32);
    emu.xrr_no_count(12usize, 16usize, 6usize, 2127204u32);
    emu.xrr_no_count(15usize, 5usize, 7usize, 2127208u32);
    emu.xrr_no_count(16usize, 28usize, 30usize, 2127212u32);
    emu.lw_no_count(8usize, 2usize, 464u32, 2127216u32)?;
    emu.adr_no_count(8usize, 8usize, 9usize, 2127220u32);
    emu.xrr_no_count(5usize, 18usize, 20usize, 2127224u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2127228u32);
    emu.sri_no_count(6usize, 10usize, 10u32, 2127232u32);
    emu.xrr_no_count(11usize, 11usize, 6usize, 2127236u32);
    emu.sw_no_count(14usize, 2usize, 460u32, 2127240u32)?;
    emu.sri_no_count(6usize, 14usize, 10u32, 2127244u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2127248u32);
    emu.sri_no_count(6usize, 14usize, 3u32, 2127252u32);
    emu.xrr_no_count(10usize, 12usize, 6usize, 2127256u32);
    emu.sw_no_count(10usize, 2usize, 380u32, 2127260u32)?;
    emu.sri_no_count(12usize, 18usize, 3u32, 2127264u32);
    emu.sw_no_count(18usize, 2usize, 400u32, 2127268u32)?;
    emu.xrr_no_count(12usize, 15usize, 12usize, 2127272u32);
    emu.sw_no_count(12usize, 2usize, 320u32, 2127276u32)?;
    emu.xrr_no_count(12usize, 16usize, 31usize, 2127280u32);
    emu.xrr_no_count(15usize, 5usize, 22usize, 2127284u32);
    emu.lw_no_count(16usize, 2usize, 488u32, 2127288u32)?;
    emu.lw_no_count(10usize, 2usize, 396u32, 2127292u32)?;
    emu.adr_no_count(16usize, 10usize, 16usize, 2127296u32);
    emu.lw_no_count(10usize, 2usize, 444u32, 2127300u32)?;
    emu.adr_no_count(16usize, 16usize, 10usize, 2127304u32);
    emu.adr_no_count(22usize, 16usize, 11usize, 2127308u32);
    emu.lw_no_count(11usize, 2usize, 412u32, 2127312u32)?;
    emu.lw_no_count(14usize, 2usize, 456u32, 2127316u32)?;
    emu.adr_no_count(11usize, 14usize, 11usize, 2127320u32);
    emu.lw_no_count(14usize, 2usize, 500u32, 2127324u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2127328u32);
    emu.adr_no_count(28usize, 11usize, 13usize, 2127332u32);
    emu.lw_no_count(11usize, 2usize, 240u32, 2127336u32)?;
    emu.adr_no_count(8usize, 8usize, 11usize, 2127340u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2127344u32);
    emu.adr_no_count(11usize, 15usize, 25usize, 2127348u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2127352u32);
    emu.adr_no_count(6usize, 12usize, 29usize, 2127356u32);
    emu.sri_no_count(12usize, 22usize, 17u32, 2127360u32);
    emu.sli_no_count(13usize, 22usize, 15u32, 2127364u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2127368u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2127372u32);
    emu.sli_no_count(14usize, 22usize, 13u32, 2127376u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2127380u32);
    emu.sri_no_count(14usize, 28usize, 17u32, 2127384u32);
    emu.sli_no_count(15usize, 28usize, 15u32, 2127388u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2127392u32);
    emu.sri_no_count(15usize, 28usize, 19u32, 2127396u32);
    emu.sli_no_count(16usize, 28usize, 13u32, 2127400u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2127404u32);
    emu.sri_no_count(16usize, 22usize, 7u32, 2127408u32);
    emu.sli_no_count(5usize, 22usize, 25u32, 2127412u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2127416u32);
    emu.sri_no_count(5usize, 22usize, 18u32, 2127420u32);
    emu.sli_no_count(7usize, 22usize, 14u32, 2127424u32);
    emu.orr_no_count(5usize, 7usize, 5usize, 2127428u32);
    emu.adi_no_count(29usize, 28usize, 0u32, 2127432u32);
    emu.sri_no_count(7usize, 28usize, 7u32, 2127436u32);
    emu.sli_no_count(28usize, 28usize, 25u32, 2127440u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2127444u32);
    emu.sri_no_count(28usize, 29usize, 18u32, 2127448u32);
    emu.sli_no_count(30usize, 29usize, 14u32, 2127452u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2127456u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2127460u32);
    emu.xrr_no_count(15usize, 14usize, 15usize, 2127464u32);
    emu.xrr_no_count(13usize, 16usize, 5usize, 2127468u32);
    emu.xrr_no_count(16usize, 7usize, 28usize, 2127472u32);
    emu.sri_no_count(14usize, 6usize, 6u32, 2127476u32);
    emu.sli_no_count(5usize, 6usize, 26u32, 2127480u32);
    emu.orr_no_count(5usize, 5usize, 14usize, 2127484u32);
    emu.sri_no_count(14usize, 6usize, 11u32, 2127488u32);
    emu.sli_no_count(7usize, 6usize, 21u32, 2127492u32);
    emu.orr_no_count(7usize, 7usize, 14usize, 2127496u32);
    emu.sri_no_count(14usize, 6usize, 25u32, 2127500u32);
    emu.sli_no_count(28usize, 6usize, 7u32, 2127504u32);
    emu.orr_no_count(28usize, 28usize, 14usize, 2127508u32);
    emu.lw_no_count(14usize, 2usize, 484u32, 2127512u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2127516u32);
    emu.xrr_no_count(30usize, 27usize, 19usize, 2127520u32);
    emu.anr_no_count(30usize, 6usize, 30usize, 2127524u32);
    emu.xrr_no_count(30usize, 30usize, 19usize, 2127528u32);
    emu.sri_no_count(31usize, 11usize, 2u32, 2127532u32);
    emu.sli_no_count(8usize, 11usize, 30u32, 2127536u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2127540u32);
    emu.sri_no_count(8usize, 11usize, 13u32, 2127544u32);
    emu.sli_no_count(9usize, 11usize, 19u32, 2127548u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2127552u32);
    emu.sri_no_count(9usize, 11usize, 22u32, 2127556u32);
    emu.sli_no_count(19usize, 11usize, 10u32, 2127560u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2127564u32);
    emu.xrr_no_count(19usize, 23usize, 21usize, 2127568u32);
    emu.anr_no_count(19usize, 11usize, 19usize, 2127572u32);
    emu.anr_no_count(20usize, 23usize, 21usize, 2127576u32);
    emu.xrr_no_count(19usize, 19usize, 20usize, 2127580u32);
    emu.sri_no_count(20usize, 22usize, 10u32, 2127584u32);
    emu.xrr_no_count(12usize, 12usize, 20usize, 2127588u32);
    emu.sw_no_count(29usize, 2usize, 456u32, 2127592u32)?;
    emu.sri_no_count(20usize, 29usize, 10u32, 2127596u32);
    emu.xrr_no_count(15usize, 15usize, 20usize, 2127600u32);
    emu.sri_no_count(20usize, 22usize, 3u32, 2127604u32);
    emu.sw_no_count(22usize, 2usize, 396u32, 2127608u32)?;
    emu.xrr_no_count(10usize, 13usize, 20usize, 2127612u32);
    emu.sw_no_count(10usize, 2usize, 336u32, 2127616u32)?;
    emu.sri_no_count(13usize, 29usize, 3u32, 2127620u32);
    emu.xrr_no_count(10usize, 16usize, 13usize, 2127624u32);
    emu.sw_no_count(10usize, 2usize, 356u32, 2127628u32)?;
    emu.xrr_no_count(13usize, 5usize, 7usize, 2127632u32);
    emu.adr_no_count(17usize, 17usize, 30usize, 2127636u32);
    emu.xrr_no_count(16usize, 31usize, 8usize, 2127640u32);
    emu.lw_no_count(5usize, 2usize, 416u32, 2127644u32)?;
    emu.lw_no_count(10usize, 2usize, 436u32, 2127648u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2127652u32);
    emu.lw_no_count(10usize, 2usize, 476u32, 2127656u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2127660u32);
    emu.adr_no_count(26usize, 5usize, 12usize, 2127664u32);
    emu.lw_no_count(12usize, 2usize, 420u32, 2127668u32)?;
    emu.lw_no_count(29usize, 2usize, 364u32, 2127672u32)?;
    emu.adr_no_count(29usize, 29usize, 12usize, 2127676u32);
    emu.adr_no_count(29usize, 29usize, 24usize, 2127680u32);
    emu.adr_no_count(10usize, 29usize, 15usize, 2127684u32);
    emu.xrr_no_count(12usize, 13usize, 28usize, 2127688u32);
    emu.xrr_no_count(13usize, 16usize, 9usize, 2127692u32);
    emu.lw_no_count(15usize, 2usize, 236u32, 2127696u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2127700u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2127704u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2127708u32);
    emu.sri_no_count(15usize, 26usize, 17u32, 2127712u32);
    emu.sli_no_count(16usize, 26usize, 15u32, 2127716u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2127720u32);
    emu.sri_no_count(16usize, 26usize, 19u32, 2127724u32);
    emu.sli_no_count(17usize, 26usize, 13u32, 2127728u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2127732u32);
    emu.sri_no_count(17usize, 10usize, 17u32, 2127736u32);
    emu.sli_no_count(5usize, 10usize, 15u32, 2127740u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2127744u32);
    emu.sri_no_count(5usize, 10usize, 19u32, 2127748u32);
    emu.sli_no_count(7usize, 10usize, 13u32, 2127752u32);
    emu.orr_no_count(5usize, 7usize, 5usize, 2127756u32);
    emu.sri_no_count(7usize, 10usize, 7u32, 2127760u32);
    emu.sli_no_count(28usize, 10usize, 25u32, 2127764u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2127768u32);
    emu.sri_no_count(28usize, 26usize, 7u32, 2127772u32);
    emu.sli_no_count(29usize, 26usize, 25u32, 2127776u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2127780u32);
    emu.sri_no_count(29usize, 10usize, 18u32, 2127784u32);
    emu.sli_no_count(30usize, 10usize, 14u32, 2127788u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2127792u32);
    emu.sri_no_count(30usize, 26usize, 18u32, 2127796u32);
    emu.sli_no_count(31usize, 26usize, 14u32, 2127800u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2127804u32);
    emu.adr_no_count(25usize, 13usize, 12usize, 2127808u32);
    emu.adr_no_count(1usize, 12usize, 1usize, 2127812u32);
    emu.xrr_no_count(12usize, 15usize, 16usize, 2127816u32);
    emu.xrr_no_count(13usize, 17usize, 5usize, 2127820u32);
    emu.xrr_no_count(15usize, 7usize, 29usize, 2127824u32);
    emu.xrr_no_count(16usize, 28usize, 30usize, 2127828u32);
    emu.sri_no_count(17usize, 26usize, 10u32, 2127832u32);
    emu.xrr_no_count(17usize, 12usize, 17usize, 2127836u32);
    emu.sw_no_count(10usize, 2usize, 436u32, 2127840u32)?;
    emu.sri_no_count(12usize, 10usize, 10u32, 2127844u32);
    emu.xrr_no_count(5usize, 13usize, 12usize, 2127848u32);
    emu.sri_no_count(12usize, 10usize, 3u32, 2127852u32);
    emu.xrr_no_count(12usize, 15usize, 12usize, 2127856u32);
    emu.sw_no_count(12usize, 2usize, 364u32, 2127860u32)?;
    emu.sri_no_count(12usize, 26usize, 3u32, 2127864u32);
    emu.sw_no_count(26usize, 2usize, 392u32, 2127868u32)?;
    emu.xrr_no_count(10usize, 16usize, 12usize, 2127872u32);
    emu.sw_no_count(10usize, 2usize, 344u32, 2127876u32)?;
    emu.sri_no_count(12usize, 1usize, 6u32, 2127880u32);
    emu.sli_no_count(13usize, 1usize, 26u32, 2127884u32);
    emu.orr_no_count(15usize, 13usize, 12usize, 2127888u32);
    emu.sri_no_count(12usize, 1usize, 11u32, 2127892u32);
    emu.sli_no_count(13usize, 1usize, 21u32, 2127896u32);
    emu.orr_no_count(16usize, 13usize, 12usize, 2127900u32);
    emu.sri_no_count(12usize, 1usize, 25u32, 2127904u32);
    emu.sli_no_count(13usize, 1usize, 7u32, 2127908u32);
    emu.orr_no_count(7usize, 13usize, 12usize, 2127912u32);
    emu.lw_no_count(12usize, 2usize, 480u32, 2127916u32)?;
    emu.adr_no_count(12usize, 12usize, 27usize, 2127920u32);
    emu.xrr_no_count(13usize, 6usize, 27usize, 2127924u32);
    emu.anr_no_count(13usize, 1usize, 13usize, 2127928u32);
    emu.xrr_no_count(28usize, 13usize, 27usize, 2127932u32);
    emu.sri_no_count(13usize, 25usize, 2u32, 2127936u32);
    emu.sli_no_count(29usize, 25usize, 30u32, 2127940u32);
    emu.orr_no_count(29usize, 29usize, 13usize, 2127944u32);
    emu.sri_no_count(13usize, 25usize, 13u32, 2127948u32);
    emu.sli_no_count(30usize, 25usize, 19u32, 2127952u32);
    emu.orr_no_count(30usize, 30usize, 13usize, 2127956u32);
    emu.sri_no_count(13usize, 25usize, 22u32, 2127960u32);
    emu.sli_no_count(31usize, 25usize, 10u32, 2127964u32);
    emu.orr_no_count(31usize, 31usize, 13usize, 2127968u32);
    emu.xrr_no_count(13usize, 11usize, 23usize, 2127972u32);
    emu.anr_no_count(13usize, 25usize, 13usize, 2127976u32);
    emu.anr_no_count(8usize, 11usize, 23usize, 2127980u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2127984u32);
    emu.lw_no_count(8usize, 2usize, 408u32, 2127988u32)?;
    emu.lw_no_count(10usize, 2usize, 352u32, 2127992u32)?;
    emu.adr_no_count(8usize, 10usize, 8usize, 2127996u32);
    emu.lw_no_count(10usize, 2usize, 472u32, 2128000u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2128004u32);
    emu.adr_no_count(8usize, 8usize, 17usize, 2128008u32);
    emu.lw_no_count(17usize, 2usize, 492u32, 2128012u32)?;
    emu.lw_no_count(10usize, 2usize, 348u32, 2128016u32)?;
    emu.adr_no_count(17usize, 10usize, 17usize, 2128020u32);
    emu.adr_no_count(17usize, 17usize, 18usize, 2128024u32);
    emu.adr_no_count(10usize, 17usize, 5usize, 2128028u32);
    emu.xrr_no_count(15usize, 15usize, 16usize, 2128032u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2128036u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2128040u32);
    emu.xrr_no_count(16usize, 15usize, 7usize, 2128044u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2128048u32);
    emu.sri_no_count(15usize, 8usize, 17u32, 2128052u32);
    emu.sli_no_count(5usize, 8usize, 15u32, 2128056u32);
    emu.orr_no_count(5usize, 5usize, 15usize, 2128060u32);
    emu.sri_no_count(15usize, 8usize, 19u32, 2128064u32);
    emu.sli_no_count(7usize, 8usize, 13u32, 2128068u32);
    emu.orr_no_count(7usize, 7usize, 15usize, 2128072u32);
    emu.sri_no_count(15usize, 10usize, 17u32, 2128076u32);
    emu.sli_no_count(28usize, 10usize, 15u32, 2128080u32);
    emu.orr_no_count(28usize, 28usize, 15usize, 2128084u32);
    emu.sri_no_count(15usize, 10usize, 19u32, 2128088u32);
    emu.sli_no_count(29usize, 10usize, 13u32, 2128092u32);
    emu.orr_no_count(29usize, 29usize, 15usize, 2128096u32);
    emu.sri_no_count(15usize, 8usize, 7u32, 2128100u32);
    emu.sli_no_count(30usize, 8usize, 25u32, 2128104u32);
    emu.orr_no_count(31usize, 30usize, 15usize, 2128108u32);
    emu.sri_no_count(15usize, 8usize, 18u32, 2128112u32);
    emu.sli_no_count(30usize, 8usize, 14u32, 2128116u32);
    emu.adi_no_count(18usize, 8usize, 0u32, 2128120u32);
    emu.orr_no_count(15usize, 30usize, 15usize, 2128124u32);
    emu.sri_no_count(30usize, 10usize, 7u32, 2128128u32);
    emu.sli_no_count(8usize, 10usize, 25u32, 2128132u32);
    emu.orr_no_count(30usize, 8usize, 30usize, 2128136u32);
    emu.sri_no_count(8usize, 10usize, 18u32, 2128140u32);
    emu.sli_no_count(9usize, 10usize, 14u32, 2128144u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2128148u32);
    emu.lw_no_count(9usize, 2usize, 232u32, 2128152u32)?;
    emu.adr_no_count(14usize, 14usize, 9usize, 2128156u32);
    emu.adr_no_count(16usize, 14usize, 16usize, 2128160u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2128164u32);
    emu.xrr_no_count(5usize, 5usize, 7usize, 2128168u32);
    emu.xrr_no_count(7usize, 28usize, 29usize, 2128172u32);
    emu.xrr_no_count(15usize, 31usize, 15usize, 2128176u32);
    emu.xrr_no_count(28usize, 30usize, 8usize, 2128180u32);
    emu.adr_no_count(14usize, 13usize, 16usize, 2128184u32);
    emu.adr_no_count(17usize, 16usize, 21usize, 2128188u32);
    emu.sw_no_count(18usize, 2usize, 468u32, 2128192u32)?;
    emu.sri_no_count(13usize, 18usize, 10u32, 2128196u32);
    emu.xrr_no_count(13usize, 5usize, 13usize, 2128200u32);
    emu.sw_no_count(10usize, 2usize, 484u32, 2128204u32)?;
    emu.sri_no_count(16usize, 10usize, 10u32, 2128208u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2128212u32);
    emu.sri_no_count(5usize, 18usize, 3u32, 2128216u32);
    emu.xrr_no_count(15usize, 15usize, 5usize, 2128220u32);
    emu.sw_no_count(15usize, 2usize, 328u32, 2128224u32)?;
    emu.sri_no_count(15usize, 10usize, 3u32, 2128228u32);
    emu.xrr_no_count(10usize, 28usize, 15usize, 2128232u32);
    emu.sw_no_count(10usize, 2usize, 352u32, 2128236u32)?;
    emu.lw_no_count(15usize, 2usize, 496u32, 2128240u32)?;
    emu.lw_no_count(10usize, 2usize, 388u32, 2128244u32)?;
    emu.adr_no_count(15usize, 10usize, 15usize, 2128248u32);
    emu.lw_no_count(10usize, 2usize, 460u32, 2128252u32)?;
    emu.adr_no_count(15usize, 15usize, 10usize, 2128256u32);
    emu.adr_no_count(8usize, 15usize, 13usize, 2128260u32);
    emu.lw_no_count(10usize, 2usize, 332u32, 2128264u32)?;
    emu.lw_no_count(21usize, 2usize, 440u32, 2128268u32)?;
    emu.adr_no_count(21usize, 10usize, 21usize, 2128272u32);
    emu.adr_no_count(21usize, 21usize, 22usize, 2128276u32);
    emu.adr_no_count(18usize, 21usize, 16usize, 2128280u32);
    emu.sri_no_count(13usize, 17usize, 6u32, 2128284u32);
    emu.sli_no_count(15usize, 17usize, 26u32, 2128288u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2128292u32);
    emu.sri_no_count(15usize, 17usize, 11u32, 2128296u32);
    emu.sli_no_count(16usize, 17usize, 21u32, 2128300u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2128304u32);
    emu.sri_no_count(16usize, 17usize, 25u32, 2128308u32);
    emu.sli_no_count(5usize, 17usize, 7u32, 2128312u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2128316u32);
    emu.lw_no_count(5usize, 2usize, 448u32, 2128320u32)?;
    emu.adr_no_count(5usize, 5usize, 6usize, 2128324u32);
    emu.xrr_no_count(7usize, 1usize, 6usize, 2128328u32);
    emu.anr_no_count(7usize, 17usize, 7usize, 2128332u32);
    emu.xrr_no_count(6usize, 7usize, 6usize, 2128336u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2128340u32);
    emu.sli_no_count(28usize, 14usize, 30u32, 2128344u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2128348u32);
    emu.sri_no_count(28usize, 14usize, 13u32, 2128352u32);
    emu.sli_no_count(29usize, 14usize, 19u32, 2128356u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2128360u32);
    emu.sri_no_count(29usize, 14usize, 22u32, 2128364u32);
    emu.sli_no_count(30usize, 14usize, 10u32, 2128368u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2128372u32);
    emu.xrr_no_count(30usize, 25usize, 11usize, 2128376u32);
    emu.anr_no_count(30usize, 14usize, 30usize, 2128380u32);
    emu.anr_no_count(31usize, 25usize, 11usize, 2128384u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2128388u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2128392u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2128396u32);
    emu.xrr_no_count(15usize, 7usize, 28usize, 2128400u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2128404u32);
    emu.sri_no_count(6usize, 8usize, 17u32, 2128408u32);
    emu.sli_no_count(7usize, 8usize, 15u32, 2128412u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2128416u32);
    emu.sri_no_count(7usize, 8usize, 19u32, 2128420u32);
    emu.sli_no_count(28usize, 8usize, 13u32, 2128424u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2128428u32);
    emu.sri_no_count(28usize, 18usize, 17u32, 2128432u32);
    emu.sli_no_count(31usize, 18usize, 15u32, 2128436u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2128440u32);
    emu.sri_no_count(31usize, 18usize, 19u32, 2128444u32);
    emu.sli_no_count(8usize, 18usize, 13u32, 2128448u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2128452u32);
    emu.sri_no_count(8usize, 18usize, 7u32, 2128456u32);
    emu.sli_no_count(9usize, 18usize, 25u32, 2128460u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2128464u32);
    emu.sri_no_count(9usize, 10usize, 7u32, 2128468u32);
    emu.sli_no_count(19usize, 10usize, 25u32, 2128472u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2128476u32);
    emu.sri_no_count(19usize, 18usize, 18u32, 2128480u32);
    emu.sli_no_count(20usize, 18usize, 14u32, 2128484u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2128488u32);
    emu.sri_no_count(20usize, 10usize, 18u32, 2128492u32);
    emu.sli_no_count(21usize, 10usize, 14u32, 2128496u32);
    emu.orr_no_count(20usize, 21usize, 20usize, 2128500u32);
    emu.xrr_no_count(13usize, 13usize, 16usize, 2128504u32);
    emu.xrr_no_count(15usize, 15usize, 29usize, 2128508u32);
    emu.xrr_no_count(16usize, 6usize, 7usize, 2128512u32);
    emu.xrr_no_count(6usize, 28usize, 31usize, 2128516u32);
    emu.xrr_no_count(7usize, 8usize, 19usize, 2128520u32);
    emu.xrr_no_count(28usize, 9usize, 20usize, 2128524u32);
    emu.lw_no_count(29usize, 2usize, 228u32, 2128528u32)?;
    emu.adr_no_count(12usize, 12usize, 29usize, 2128532u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2128536u32);
    emu.adr_no_count(12usize, 15usize, 30usize, 2128540u32);
    emu.sw_no_count(10usize, 2usize, 464u32, 2128544u32)?;
    emu.sri_no_count(15usize, 10usize, 10u32, 2128548u32);
    emu.xrr_no_count(22usize, 16usize, 15usize, 2128552u32);
    emu.sw_no_count(18usize, 2usize, 480u32, 2128556u32)?;
    emu.sri_no_count(15usize, 18usize, 10u32, 2128560u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2128564u32);
    emu.sri_no_count(16usize, 18usize, 3u32, 2128568u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2128572u32);
    emu.sw_no_count(16usize, 2usize, 348u32, 2128576u32)?;
    emu.sri_no_count(16usize, 10usize, 3u32, 2128580u32);
    emu.xrr_no_count(10usize, 28usize, 16usize, 2128584u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2128588u32)?;
    emu.adr_no_count(18usize, 12usize, 13usize, 2128592u32);
    emu.adr_no_count(10usize, 13usize, 23usize, 2128596u32);
    emu.lw_no_count(13usize, 2usize, 504u32, 2128600u32)?;
    emu.lw_no_count(12usize, 2usize, 376u32, 2128604u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2128608u32);
    emu.lw_no_count(12usize, 2usize, 456u32, 2128612u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2128616u32);
    emu.adr_no_count(22usize, 13usize, 22usize, 2128620u32);
    emu.lw_no_count(12usize, 2usize, 324u32, 2128624u32)?;
    emu.lw_no_count(24usize, 2usize, 444u32, 2128628u32)?;
    emu.adr_no_count(24usize, 12usize, 24usize, 2128632u32);
    emu.adr_no_count(24usize, 24usize, 26usize, 2128636u32);
    emu.adr_no_count(12usize, 24usize, 15usize, 2128640u32);
    emu.sri_no_count(13usize, 10usize, 6u32, 2128644u32);
    emu.sli_no_count(15usize, 10usize, 26u32, 2128648u32);
    emu.orr_no_count(16usize, 15usize, 13usize, 2128652u32);
    emu.sri_no_count(13usize, 10usize, 11u32, 2128656u32);
    emu.sli_no_count(8usize, 10usize, 21u32, 2128660u32);
    emu.orr_no_count(8usize, 8usize, 13usize, 2128664u32);
    emu.sri_no_count(13usize, 10usize, 25u32, 2128668u32);
    emu.sli_no_count(15usize, 10usize, 7u32, 2128672u32);
    emu.orr_no_count(6usize, 15usize, 13usize, 2128676u32);
    emu.lw_no_count(15usize, 2usize, 452u32, 2128680u32)?;
    emu.adr_no_count(15usize, 15usize, 1usize, 2128684u32);
    emu.xrr_no_count(13usize, 17usize, 1usize, 2128688u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2128692u32);
    emu.xrr_no_count(28usize, 13usize, 1usize, 2128696u32);
    emu.sri_no_count(13usize, 18usize, 2u32, 2128700u32);
    emu.sli_no_count(7usize, 18usize, 30u32, 2128704u32);
    emu.orr_no_count(21usize, 7usize, 13usize, 2128708u32);
    emu.sri_no_count(13usize, 18usize, 13u32, 2128712u32);
    emu.sli_no_count(7usize, 18usize, 19u32, 2128716u32);
    emu.orr_no_count(23usize, 7usize, 13usize, 2128720u32);
    emu.sri_no_count(13usize, 18usize, 22u32, 2128724u32);
    emu.sli_no_count(7usize, 18usize, 10u32, 2128728u32);
    emu.orr_no_count(30usize, 7usize, 13usize, 2128732u32);
    emu.xrr_no_count(13usize, 14usize, 25usize, 2128736u32);
    emu.anr_no_count(13usize, 18usize, 13usize, 2128740u32);
    emu.anr_no_count(7usize, 14usize, 25usize, 2128744u32);
    emu.xrr_no_count(7usize, 13usize, 7usize, 2128748u32);
    emu.sri_no_count(13usize, 22usize, 17u32, 2128752u32);
    emu.sli_no_count(29usize, 22usize, 15u32, 2128756u32);
    emu.orr_no_count(24usize, 29usize, 13usize, 2128760u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2128764u32);
    emu.sli_no_count(29usize, 22usize, 13u32, 2128768u32);
    emu.orr_no_count(26usize, 29usize, 13usize, 2128772u32);
    emu.sri_no_count(13usize, 12usize, 17u32, 2128776u32);
    emu.sli_no_count(29usize, 12usize, 15u32, 2128780u32);
    emu.orr_no_count(27usize, 29usize, 13usize, 2128784u32);
    emu.sri_no_count(13usize, 12usize, 19u32, 2128788u32);
    emu.sli_no_count(29usize, 12usize, 13u32, 2128792u32);
    emu.orr_no_count(1usize, 29usize, 13usize, 2128796u32);
    emu.sri_no_count(13usize, 22usize, 7u32, 2128800u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2128804u32);
    emu.orr_no_count(13usize, 29usize, 13usize, 2128808u32);
    emu.sri_no_count(29usize, 22usize, 18u32, 2128812u32);
    emu.sli_no_count(31usize, 22usize, 14u32, 2128816u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2128820u32);
    emu.sri_no_count(31usize, 12usize, 7u32, 2128824u32);
    emu.sli_no_count(9usize, 12usize, 25u32, 2128828u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2128832u32);
    emu.sri_no_count(9usize, 12usize, 18u32, 2128836u32);
    emu.sli_no_count(19usize, 12usize, 14u32, 2128840u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2128844u32);
    emu.xrr_no_count(16usize, 16usize, 8usize, 2128848u32);
    emu.adr_no_count(5usize, 5usize, 28usize, 2128852u32);
    emu.xrr_no_count(28usize, 21usize, 23usize, 2128856u32);
    emu.xrr_no_count(8usize, 24usize, 26usize, 2128860u32);
    emu.xrr_no_count(19usize, 27usize, 1usize, 2128864u32);
    emu.xrr_no_count(13usize, 13usize, 29usize, 2128868u32);
    emu.xrr_no_count(29usize, 31usize, 9usize, 2128872u32);
    emu.xrr_no_count(16usize, 16usize, 6usize, 2128876u32);
    emu.xrr_no_count(6usize, 28usize, 30usize, 2128880u32);
    emu.sri_no_count(28usize, 22usize, 10u32, 2128884u32);
    emu.xrr_no_count(31usize, 8usize, 28usize, 2128888u32);
    emu.sw_no_count(12usize, 2usize, 448u32, 2128892u32)?;
    emu.sri_no_count(28usize, 12usize, 10u32, 2128896u32);
    emu.xrr_no_count(20usize, 19usize, 28usize, 2128900u32);
    emu.sri_no_count(28usize, 22usize, 3u32, 2128904u32);
    emu.sw_no_count(22usize, 2usize, 388u32, 2128908u32)?;
    emu.xrr_no_count(13usize, 13usize, 28usize, 2128912u32);
    emu.sw_no_count(13usize, 2usize, 312u32, 2128916u32)?;
    emu.sri_no_count(13usize, 12usize, 3u32, 2128920u32);
    emu.xrr_no_count(12usize, 29usize, 13usize, 2128924u32);
    emu.sw_no_count(12usize, 2usize, 324u32, 2128928u32)?;
    emu.lw_no_count(12usize, 2usize, 224u32, 2128932u32)?;
    emu.adr_no_count(5usize, 5usize, 12usize, 2128936u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2128940u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2128944u32);
    emu.lw_no_count(13usize, 2usize, 500u32, 2128948u32)?;
    emu.lw_no_count(12usize, 2usize, 384u32, 2128952u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2128956u32);
    emu.lw_no_count(12usize, 2usize, 436u32, 2128960u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2128964u32);
    emu.adr_no_count(31usize, 13usize, 31usize, 2128968u32);
    emu.lw_no_count(13usize, 2usize, 476u32, 2128972u32)?;
    emu.lw_no_count(12usize, 2usize, 340u32, 2128976u32)?;
    emu.adr_no_count(13usize, 12usize, 13usize, 2128980u32);
    emu.lw_no_count(12usize, 2usize, 468u32, 2128984u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2128988u32);
    emu.adr_no_count(20usize, 13usize, 20usize, 2128992u32);
    emu.adr_no_count(26usize, 6usize, 16usize, 2128996u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2129000u32);
    emu.sri_no_count(13usize, 31usize, 17u32, 2129004u32);
    emu.sli_no_count(16usize, 31usize, 15u32, 2129008u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2129012u32);
    emu.sri_no_count(16usize, 31usize, 19u32, 2129016u32);
    emu.sli_no_count(5usize, 31usize, 13u32, 2129020u32);
    emu.orr_no_count(5usize, 5usize, 16usize, 2129024u32);
    emu.sri_no_count(16usize, 20usize, 17u32, 2129028u32);
    emu.sli_no_count(6usize, 20usize, 15u32, 2129032u32);
    emu.orr_no_count(6usize, 6usize, 16usize, 2129036u32);
    emu.sri_no_count(16usize, 20usize, 19u32, 2129040u32);
    emu.sli_no_count(7usize, 20usize, 13u32, 2129044u32);
    emu.orr_no_count(7usize, 7usize, 16usize, 2129048u32);
    emu.sri_no_count(16usize, 20usize, 7u32, 2129052u32);
    emu.sli_no_count(28usize, 20usize, 25u32, 2129056u32);
    emu.orr_no_count(28usize, 28usize, 16usize, 2129060u32);
    emu.sri_no_count(16usize, 31usize, 7u32, 2129064u32);
    emu.sli_no_count(29usize, 31usize, 25u32, 2129068u32);
    emu.orr_no_count(29usize, 29usize, 16usize, 2129072u32);
    emu.sri_no_count(16usize, 20usize, 18u32, 2129076u32);
    emu.sli_no_count(30usize, 20usize, 14u32, 2129080u32);
    emu.orr_no_count(30usize, 30usize, 16usize, 2129084u32);
    emu.sri_no_count(16usize, 31usize, 18u32, 2129088u32);
    emu.sli_no_count(8usize, 31usize, 14u32, 2129092u32);
    emu.orr_no_count(8usize, 8usize, 16usize, 2129096u32);
    emu.sri_no_count(16usize, 11usize, 6u32, 2129100u32);
    emu.sli_no_count(9usize, 11usize, 26u32, 2129104u32);
    emu.orr_no_count(9usize, 9usize, 16usize, 2129108u32);
    emu.sri_no_count(16usize, 11usize, 11u32, 2129112u32);
    emu.sli_no_count(19usize, 11usize, 21u32, 2129116u32);
    emu.orr_no_count(19usize, 19usize, 16usize, 2129120u32);
    emu.sri_no_count(16usize, 11usize, 25u32, 2129124u32);
    emu.sli_no_count(21usize, 11usize, 7u32, 2129128u32);
    emu.orr_no_count(21usize, 21usize, 16usize, 2129132u32);
    emu.lw_no_count(16usize, 2usize, 488u32, 2129136u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2129140u32);
    emu.xrr_no_count(23usize, 10usize, 17usize, 2129144u32);
    emu.anr_no_count(23usize, 11usize, 23usize, 2129148u32);
    emu.xrr_no_count(17usize, 23usize, 17usize, 2129152u32);
    emu.sri_no_count(23usize, 26usize, 2u32, 2129156u32);
    emu.sli_no_count(24usize, 26usize, 30u32, 2129160u32);
    emu.orr_no_count(23usize, 24usize, 23usize, 2129164u32);
    emu.sri_no_count(24usize, 26usize, 13u32, 2129168u32);
    emu.sli_no_count(27usize, 26usize, 19u32, 2129172u32);
    emu.orr_no_count(24usize, 27usize, 24usize, 2129176u32);
    emu.sri_no_count(27usize, 26usize, 22u32, 2129180u32);
    emu.sli_no_count(1usize, 26usize, 10u32, 2129184u32);
    emu.orr_no_count(27usize, 1usize, 27usize, 2129188u32);
    emu.xrr_no_count(1usize, 18usize, 14usize, 2129192u32);
    emu.anr_no_count(1usize, 26usize, 1usize, 2129196u32);
    emu.anr_no_count(12usize, 18usize, 14usize, 2129200u32);
    emu.xrr_no_count(12usize, 1usize, 12usize, 2129204u32);
    emu.xrr_no_count(13usize, 13usize, 5usize, 2129208u32);
    emu.xrr_no_count(5usize, 6usize, 7usize, 2129212u32);
    emu.xrr_no_count(6usize, 28usize, 30usize, 2129216u32);
    emu.xrr_no_count(7usize, 29usize, 8usize, 2129220u32);
    emu.xrr_no_count(28usize, 9usize, 19usize, 2129224u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2129228u32);
    emu.xrr_no_count(17usize, 23usize, 24usize, 2129232u32);
    emu.sri_no_count(29usize, 31usize, 10u32, 2129236u32);
    emu.xrr_no_count(13usize, 13usize, 29usize, 2129240u32);
    emu.sri_no_count(29usize, 20usize, 10u32, 2129244u32);
    emu.xrr_no_count(19usize, 5usize, 29usize, 2129248u32);
    emu.sri_no_count(5usize, 20usize, 3u32, 2129252u32);
    emu.xrr_no_count(5usize, 6usize, 5usize, 2129256u32);
    emu.sw_no_count(5usize, 2usize, 316u32, 2129260u32)?;
    emu.sri_no_count(5usize, 31usize, 3u32, 2129264u32);
    emu.sw_no_count(31usize, 2usize, 452u32, 2129268u32)?;
    emu.xrr_no_count(5usize, 7usize, 5usize, 2129272u32);
    emu.sw_no_count(5usize, 2usize, 308u32, 2129276u32)?;
    emu.xrr_no_count(5usize, 28usize, 21usize, 2129280u32);
    emu.xrr_no_count(17usize, 17usize, 27usize, 2129284u32);
    emu.lw_no_count(6usize, 2usize, 404u32, 2129288u32)?;
    emu.lw_no_count(7usize, 2usize, 360u32, 2129292u32)?;
    emu.adr_no_count(6usize, 7usize, 6usize, 2129296u32);
    emu.lw_no_count(7usize, 2usize, 484u32, 2129300u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2129304u32);
    emu.adr_no_count(24usize, 6usize, 13usize, 2129308u32);
    emu.lw_no_count(13usize, 2usize, 472u32, 2129312u32)?;
    emu.lw_no_count(6usize, 2usize, 320u32, 2129316u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2129320u32);
    emu.lw_no_count(6usize, 2usize, 464u32, 2129324u32)?;
    emu.adr_no_count(13usize, 13usize, 6usize, 2129328u32);
    emu.adr_no_count(19usize, 13usize, 19usize, 2129332u32);
    emu.lw_no_count(13usize, 2usize, 220u32, 2129336u32)?;
    emu.adr_no_count(15usize, 15usize, 13usize, 2129340u32);
    emu.adr_no_count(15usize, 15usize, 5usize, 2129344u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2129348u32);
    emu.adr_no_count(1usize, 12usize, 15usize, 2129352u32);
    emu.adr_no_count(27usize, 15usize, 25usize, 2129356u32);
    emu.sri_no_count(12usize, 24usize, 17u32, 2129360u32);
    emu.sli_no_count(13usize, 24usize, 15u32, 2129364u32);
    emu.orr_no_count(13usize, 13usize, 12usize, 2129368u32);
    emu.sri_no_count(12usize, 24usize, 19u32, 2129372u32);
    emu.sli_no_count(15usize, 24usize, 13u32, 2129376u32);
    emu.orr_no_count(15usize, 15usize, 12usize, 2129380u32);
    emu.sri_no_count(12usize, 19usize, 17u32, 2129384u32);
    emu.sli_no_count(17usize, 19usize, 15u32, 2129388u32);
    emu.orr_no_count(17usize, 17usize, 12usize, 2129392u32);
    emu.sri_no_count(12usize, 19usize, 19u32, 2129396u32);
    emu.sli_no_count(5usize, 19usize, 13u32, 2129400u32);
    emu.orr_no_count(5usize, 5usize, 12usize, 2129404u32);
    emu.sri_no_count(12usize, 24usize, 7u32, 2129408u32);
    emu.sli_no_count(6usize, 24usize, 25u32, 2129412u32);
    emu.orr_no_count(6usize, 6usize, 12usize, 2129416u32);
    emu.sri_no_count(12usize, 24usize, 18u32, 2129420u32);
    emu.sli_no_count(7usize, 24usize, 14u32, 2129424u32);
    emu.orr_no_count(7usize, 7usize, 12usize, 2129428u32);
    emu.sri_no_count(12usize, 19usize, 7u32, 2129432u32);
    emu.sli_no_count(29usize, 19usize, 25u32, 2129436u32);
    emu.orr_no_count(12usize, 29usize, 12usize, 2129440u32);
    emu.sri_no_count(29usize, 19usize, 18u32, 2129444u32);
    emu.sli_no_count(30usize, 19usize, 14u32, 2129448u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2129452u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2129456u32);
    emu.xrr_no_count(15usize, 17usize, 5usize, 2129460u32);
    emu.xrr_no_count(17usize, 6usize, 7usize, 2129464u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2129468u32);
    emu.sri_no_count(5usize, 27usize, 6u32, 2129472u32);
    emu.sli_no_count(6usize, 27usize, 26u32, 2129476u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2129480u32);
    emu.sri_no_count(6usize, 27usize, 11u32, 2129484u32);
    emu.sli_no_count(7usize, 27usize, 21u32, 2129488u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2129492u32);
    emu.sri_no_count(7usize, 27usize, 25u32, 2129496u32);
    emu.sli_no_count(29usize, 27usize, 7u32, 2129500u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2129504u32);
    emu.lw_no_count(8usize, 2usize, 412u32, 2129508u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2129512u32);
    emu.xrr_no_count(29usize, 11usize, 10usize, 2129516u32);
    emu.anr_no_count(29usize, 27usize, 29usize, 2129520u32);
    emu.xrr_no_count(10usize, 29usize, 10usize, 2129524u32);
    emu.sri_no_count(29usize, 1usize, 2u32, 2129528u32);
    emu.sli_no_count(30usize, 1usize, 30u32, 2129532u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2129536u32);
    emu.sri_no_count(30usize, 1usize, 13u32, 2129540u32);
    emu.sli_no_count(9usize, 1usize, 19u32, 2129544u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2129548u32);
    emu.sri_no_count(9usize, 1usize, 22u32, 2129552u32);
    emu.sli_no_count(21usize, 1usize, 10u32, 2129556u32);
    emu.orr_no_count(9usize, 21usize, 9usize, 2129560u32);
    emu.xrr_no_count(21usize, 26usize, 18usize, 2129564u32);
    emu.anr_no_count(21usize, 1usize, 21usize, 2129568u32);
    emu.anr_no_count(23usize, 26usize, 18usize, 2129572u32);
    emu.xrr_no_count(21usize, 21usize, 23usize, 2129576u32);
    emu.sri_no_count(23usize, 24usize, 10u32, 2129580u32);
    emu.xrr_no_count(13usize, 13usize, 23usize, 2129584u32);
    emu.sw_no_count(19usize, 2usize, 488u32, 2129588u32)?;
    emu.sri_no_count(23usize, 19usize, 10u32, 2129592u32);
    emu.xrr_no_count(15usize, 15usize, 23usize, 2129596u32);
    emu.sri_no_count(23usize, 24usize, 3u32, 2129600u32);
    emu.sw_no_count(24usize, 2usize, 384u32, 2129604u32)?;
    emu.xrr_no_count(17usize, 17usize, 23usize, 2129608u32);
    emu.sw_no_count(17usize, 2usize, 304u32, 2129612u32)?;
    emu.sri_no_count(17usize, 19usize, 3u32, 2129616u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2129620u32);
    emu.sw_no_count(12usize, 2usize, 360u32, 2129624u32)?;
    emu.xrr_no_count(12usize, 5usize, 6usize, 2129628u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2129632u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2129636u32);
    emu.lw_no_count(16usize, 2usize, 400u32, 2129640u32)?;
    emu.lw_no_count(5usize, 2usize, 380u32, 2129644u32)?;
    emu.adr_no_count(16usize, 5usize, 16usize, 2129648u32);
    emu.lw_no_count(5usize, 2usize, 480u32, 2129652u32)?;
    emu.adr_no_count(16usize, 16usize, 5usize, 2129656u32);
    emu.adr_no_count(5usize, 16usize, 13usize, 2129660u32);
    emu.lw_no_count(13usize, 2usize, 460u32, 2129664u32)?;
    emu.lw_no_count(16usize, 2usize, 336u32, 2129668u32)?;
    emu.adr_no_count(13usize, 16usize, 13usize, 2129672u32);
    emu.adr_no_count(13usize, 13usize, 22usize, 2129676u32);
    emu.adr_no_count(16usize, 13usize, 15usize, 2129680u32);
    emu.xrr_no_count(12usize, 12usize, 7usize, 2129684u32);
    emu.xrr_no_count(13usize, 17usize, 9usize, 2129688u32);
    emu.lw_no_count(15usize, 2usize, 216u32, 2129692u32)?;
    emu.adr_no_count(10usize, 10usize, 15usize, 2129696u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2129700u32);
    emu.adr_no_count(10usize, 13usize, 21usize, 2129704u32);
    emu.sri_no_count(13usize, 5usize, 17u32, 2129708u32);
    emu.sli_no_count(15usize, 5usize, 15u32, 2129712u32);
    emu.orr_no_count(13usize, 15usize, 13usize, 2129716u32);
    emu.sri_no_count(15usize, 5usize, 19u32, 2129720u32);
    emu.sli_no_count(17usize, 5usize, 13u32, 2129724u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2129728u32);
    emu.sri_no_count(17usize, 16usize, 17u32, 2129732u32);
    emu.sli_no_count(6usize, 16usize, 15u32, 2129736u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2129740u32);
    emu.sri_no_count(6usize, 16usize, 19u32, 2129744u32);
    emu.sli_no_count(7usize, 16usize, 13u32, 2129748u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2129752u32);
    emu.sri_no_count(7usize, 16usize, 7u32, 2129756u32);
    emu.sli_no_count(29usize, 16usize, 25u32, 2129760u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2129764u32);
    emu.sri_no_count(29usize, 5usize, 7u32, 2129768u32);
    emu.sli_no_count(30usize, 5usize, 25u32, 2129772u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2129776u32);
    emu.sri_no_count(30usize, 16usize, 18u32, 2129780u32);
    emu.sli_no_count(9usize, 16usize, 14u32, 2129784u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2129788u32);
    emu.sri_no_count(9usize, 5usize, 18u32, 2129792u32);
    emu.sli_no_count(21usize, 5usize, 14u32, 2129796u32);
    emu.orr_no_count(9usize, 21usize, 9usize, 2129800u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2129804u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2129808u32);
    emu.xrr_no_count(13usize, 13usize, 15usize, 2129812u32);
    emu.xrr_no_count(12usize, 17usize, 6usize, 2129816u32);
    emu.xrr_no_count(15usize, 7usize, 30usize, 2129820u32);
    emu.xrr_no_count(17usize, 29usize, 9usize, 2129824u32);
    emu.sri_no_count(6usize, 5usize, 10u32, 2129828u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2129832u32);
    emu.sw_no_count(16usize, 2usize, 380u32, 2129836u32)?;
    emu.sri_no_count(6usize, 16usize, 10u32, 2129840u32);
    emu.xrr_no_count(29usize, 12usize, 6usize, 2129844u32);
    emu.sri_no_count(12usize, 16usize, 3u32, 2129848u32);
    emu.xrr_no_count(12usize, 15usize, 12usize, 2129852u32);
    emu.sw_no_count(12usize, 2usize, 320u32, 2129856u32)?;
    emu.sri_no_count(12usize, 5usize, 3u32, 2129860u32);
    emu.sw_no_count(5usize, 2usize, 412u32, 2129864u32)?;
    emu.xrr_no_count(12usize, 17usize, 12usize, 2129868u32);
    emu.sw_no_count(12usize, 2usize, 300u32, 2129872u32)?;
    emu.sri_no_count(12usize, 14usize, 6u32, 2129876u32);
    emu.sli_no_count(15usize, 14usize, 26u32, 2129880u32);
    emu.orr_no_count(12usize, 15usize, 12usize, 2129884u32);
    emu.sri_no_count(15usize, 14usize, 11u32, 2129888u32);
    emu.sli_no_count(17usize, 14usize, 21u32, 2129892u32);
    emu.orr_no_count(17usize, 17usize, 15usize, 2129896u32);
    emu.sri_no_count(15usize, 14usize, 25u32, 2129900u32);
    emu.sli_no_count(6usize, 14usize, 7u32, 2129904u32);
    emu.orr_no_count(6usize, 6usize, 15usize, 2129908u32);
    emu.lw_no_count(15usize, 2usize, 416u32, 2129912u32)?;
    emu.adr_no_count(19usize, 15usize, 11usize, 2129916u32);
    emu.xrr_no_count(15usize, 27usize, 11usize, 2129920u32);
    emu.anr_no_count(15usize, 14usize, 15usize, 2129924u32);
    emu.xrr_no_count(11usize, 15usize, 11usize, 2129928u32);
    emu.sri_no_count(15usize, 10usize, 2u32, 2129932u32);
    emu.sli_no_count(7usize, 10usize, 30u32, 2129936u32);
    emu.orr_no_count(30usize, 7usize, 15usize, 2129940u32);
    emu.sri_no_count(15usize, 10usize, 13u32, 2129944u32);
    emu.sli_no_count(7usize, 10usize, 19u32, 2129948u32);
    emu.orr_no_count(9usize, 7usize, 15usize, 2129952u32);
    emu.sri_no_count(15usize, 10usize, 22u32, 2129956u32);
    emu.sli_no_count(7usize, 10usize, 10u32, 2129960u32);
    emu.orr_no_count(21usize, 7usize, 15usize, 2129964u32);
    emu.xrr_no_count(15usize, 1usize, 26usize, 2129968u32);
    emu.anr_no_count(15usize, 10usize, 15usize, 2129972u32);
    emu.anr_no_count(7usize, 1usize, 26usize, 2129976u32);
    emu.xrr_no_count(7usize, 15usize, 7usize, 2129980u32);
    emu.lw_no_count(15usize, 2usize, 396u32, 2129984u32)?;
    emu.lw_no_count(16usize, 2usize, 356u32, 2129988u32)?;
    emu.adr_no_count(15usize, 16usize, 15usize, 2129992u32);
    emu.lw_no_count(16usize, 2usize, 448u32, 2129996u32)?;
    emu.adr_no_count(15usize, 15usize, 16usize, 2130000u32);
    emu.adr_no_count(15usize, 15usize, 13usize, 2130004u32);
    emu.lw_no_count(13usize, 2usize, 456u32, 2130008u32)?;
    emu.lw_no_count(16usize, 2usize, 344u32, 2130012u32)?;
    emu.adr_no_count(13usize, 16usize, 13usize, 2130016u32);
    emu.adr_no_count(13usize, 13usize, 31usize, 2130020u32);
    emu.adr_no_count(16usize, 13usize, 29usize, 2130024u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2130028u32);
    emu.adr_no_count(8usize, 8usize, 11usize, 2130032u32);
    emu.xrr_no_count(11usize, 30usize, 9usize, 2130036u32);
    emu.xrr_no_count(12usize, 12usize, 6usize, 2130040u32);
    emu.xrr_no_count(21usize, 11usize, 21usize, 2130044u32);
    emu.sri_no_count(11usize, 15usize, 17u32, 2130048u32);
    emu.sli_no_count(13usize, 15usize, 15u32, 2130052u32);
    emu.orr_no_count(23usize, 13usize, 11usize, 2130056u32);
    emu.sri_no_count(11usize, 15usize, 19u32, 2130060u32);
    emu.sli_no_count(13usize, 15usize, 13u32, 2130064u32);
    emu.orr_no_count(31usize, 13usize, 11usize, 2130068u32);
    emu.sri_no_count(11usize, 16usize, 17u32, 2130072u32);
    emu.sli_no_count(13usize, 16usize, 15u32, 2130076u32);
    emu.orr_no_count(25usize, 13usize, 11usize, 2130080u32);
    emu.sri_no_count(11usize, 16usize, 19u32, 2130084u32);
    emu.sli_no_count(13usize, 16usize, 13u32, 2130088u32);
    emu.orr_no_count(6usize, 13usize, 11usize, 2130092u32);
    emu.sri_no_count(11usize, 15usize, 7u32, 2130096u32);
    emu.sli_no_count(13usize, 15usize, 25u32, 2130100u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2130104u32);
    emu.sri_no_count(11usize, 15usize, 18u32, 2130108u32);
    emu.sli_no_count(17usize, 15usize, 14u32, 2130112u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2130116u32);
    emu.sri_no_count(17usize, 16usize, 7u32, 2130120u32);
    emu.sli_no_count(30usize, 16usize, 25u32, 2130124u32);
    emu.orr_no_count(17usize, 30usize, 17usize, 2130128u32);
    emu.sri_no_count(30usize, 16usize, 18u32, 2130132u32);
    emu.sli_no_count(9usize, 16usize, 14u32, 2130136u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2130140u32);
    emu.lw_no_count(28usize, 2usize, 212u32, 2130144u32)?;
    emu.adr_no_count(8usize, 8usize, 28usize, 2130148u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2130152u32);
    emu.adr_no_count(7usize, 21usize, 7usize, 2130156u32);
    emu.xrr_no_count(8usize, 23usize, 31usize, 2130160u32);
    emu.xrr_no_count(9usize, 25usize, 6usize, 2130164u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2130168u32);
    emu.xrr_no_count(13usize, 17usize, 30usize, 2130172u32);
    emu.adr_no_count(21usize, 7usize, 12usize, 2130176u32);
    emu.adr_no_count(6usize, 12usize, 18usize, 2130180u32);
    emu.sri_no_count(12usize, 15usize, 10u32, 2130184u32);
    emu.xrr_no_count(12usize, 8usize, 12usize, 2130188u32);
    emu.sri_no_count(17usize, 16usize, 10u32, 2130192u32);
    emu.xrr_no_count(17usize, 9usize, 17usize, 2130196u32);
    emu.sri_no_count(7usize, 15usize, 3u32, 2130200u32);
    emu.sw_no_count(15usize, 2usize, 276u32, 2130204u32)?;
    emu.xrr_no_count(11usize, 11usize, 7usize, 2130208u32);
    emu.sw_no_count(11usize, 2usize, 288u32, 2130212u32)?;
    emu.sri_no_count(11usize, 16usize, 3u32, 2130216u32);
    emu.sw_no_count(16usize, 2usize, 272u32, 2130220u32)?;
    emu.xrr_no_count(11usize, 13usize, 11usize, 2130224u32);
    emu.sw_no_count(11usize, 2usize, 292u32, 2130228u32)?;
    emu.lw_no_count(11usize, 2usize, 392u32, 2130232u32)?;
    emu.lw_no_count(13usize, 2usize, 364u32, 2130236u32)?;
    emu.adr_no_count(11usize, 13usize, 11usize, 2130240u32);
    emu.sw_no_count(20usize, 2usize, 376u32, 2130244u32)?;
    emu.adr_no_count(11usize, 11usize, 20usize, 2130248u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2130252u32);
    emu.lw_no_count(11usize, 2usize, 328u32, 2130256u32)?;
    emu.lw_no_count(12usize, 2usize, 436u32, 2130260u32)?;
    emu.adr_no_count(11usize, 11usize, 12usize, 2130264u32);
    emu.adr_no_count(11usize, 11usize, 24usize, 2130268u32);
    emu.adr_no_count(22usize, 11usize, 17usize, 2130272u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2130276u32);
    emu.sli_no_count(17usize, 6usize, 26u32, 2130280u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2130284u32);
    emu.sri_no_count(11usize, 6usize, 11u32, 2130288u32);
    emu.sli_no_count(7usize, 6usize, 21u32, 2130292u32);
    emu.orr_no_count(7usize, 7usize, 11usize, 2130296u32);
    emu.sri_no_count(11usize, 6usize, 25u32, 2130300u32);
    emu.sli_no_count(30usize, 6usize, 7u32, 2130304u32);
    emu.orr_no_count(30usize, 30usize, 11usize, 2130308u32);
    emu.lw_no_count(11usize, 2usize, 420u32, 2130312u32)?;
    emu.adr_no_count(11usize, 11usize, 27usize, 2130316u32);
    emu.xrr_no_count(8usize, 14usize, 27usize, 2130320u32);
    emu.anr_no_count(8usize, 6usize, 8usize, 2130324u32);
    emu.xrr_no_count(8usize, 8usize, 27usize, 2130328u32);
    emu.sri_no_count(9usize, 21usize, 2u32, 2130332u32);
    emu.sli_no_count(18usize, 21usize, 30u32, 2130336u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2130340u32);
    emu.sri_no_count(18usize, 21usize, 13u32, 2130344u32);
    emu.sli_no_count(23usize, 21usize, 19u32, 2130348u32);
    emu.orr_no_count(18usize, 23usize, 18usize, 2130352u32);
    emu.sri_no_count(23usize, 21usize, 22u32, 2130356u32);
    emu.sli_no_count(24usize, 21usize, 10u32, 2130360u32);
    emu.orr_no_count(23usize, 24usize, 23usize, 2130364u32);
    emu.xrr_no_count(24usize, 10usize, 1usize, 2130368u32);
    emu.anr_no_count(24usize, 21usize, 24usize, 2130372u32);
    emu.anr_no_count(25usize, 10usize, 1usize, 2130376u32);
    emu.xrr_no_count(24usize, 24usize, 25usize, 2130380u32);
    emu.xrr_no_count(17usize, 17usize, 7usize, 2130384u32);
    emu.adr_no_count(8usize, 19usize, 8usize, 2130388u32);
    emu.xrr_no_count(7usize, 9usize, 18usize, 2130392u32);
    emu.sri_no_count(9usize, 20usize, 17u32, 2130396u32);
    emu.sli_no_count(18usize, 20usize, 15u32, 2130400u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2130404u32);
    emu.sri_no_count(18usize, 20usize, 19u32, 2130408u32);
    emu.sli_no_count(25usize, 20usize, 13u32, 2130412u32);
    emu.orr_no_count(18usize, 25usize, 18usize, 2130416u32);
    emu.sri_no_count(25usize, 22usize, 17u32, 2130420u32);
    emu.sli_no_count(27usize, 22usize, 15u32, 2130424u32);
    emu.orr_no_count(25usize, 27usize, 25usize, 2130428u32);
    emu.sri_no_count(27usize, 22usize, 19u32, 2130432u32);
    emu.sli_no_count(13usize, 22usize, 13u32, 2130436u32);
    emu.orr_no_count(13usize, 13usize, 27usize, 2130440u32);
    emu.sri_no_count(27usize, 22usize, 7u32, 2130444u32);
    emu.sli_no_count(29usize, 22usize, 25u32, 2130448u32);
    emu.orr_no_count(29usize, 29usize, 27usize, 2130452u32);
    emu.sri_no_count(27usize, 20usize, 7u32, 2130456u32);
    emu.sli_no_count(12usize, 20usize, 25u32, 2130460u32);
    emu.orr_no_count(12usize, 12usize, 27usize, 2130464u32);
    emu.sri_no_count(27usize, 22usize, 18u32, 2130468u32);
    emu.sli_no_count(19usize, 22usize, 14u32, 2130472u32);
    emu.orr_no_count(19usize, 19usize, 27usize, 2130476u32);
    emu.sri_no_count(27usize, 20usize, 18u32, 2130480u32);
    emu.sli_no_count(31usize, 20usize, 14u32, 2130484u32);
    emu.orr_no_count(31usize, 31usize, 27usize, 2130488u32);
    emu.xrr_no_count(17usize, 17usize, 30usize, 2130492u32);
    emu.xrr_no_count(7usize, 7usize, 23usize, 2130496u32);
    emu.xrr_no_count(30usize, 9usize, 18usize, 2130500u32);
    emu.xrr_no_count(13usize, 25usize, 13usize, 2130504u32);
    emu.xrr_no_count(29usize, 29usize, 19usize, 2130508u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2130512u32);
    emu.lw_no_count(28usize, 2usize, 208u32, 2130516u32)?;
    emu.adr_no_count(8usize, 8usize, 28usize, 2130520u32);
    emu.adr_no_count(17usize, 8usize, 17usize, 2130524u32);
    emu.adr_no_count(7usize, 7usize, 24usize, 2130528u32);
    emu.sri_no_count(31usize, 20usize, 10u32, 2130532u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2130536u32);
    emu.sw_no_count(22usize, 2usize, 416u32, 2130540u32)?;
    emu.sri_no_count(31usize, 22usize, 10u32, 2130544u32);
    emu.xrr_no_count(13usize, 13usize, 31usize, 2130548u32);
    emu.sri_no_count(31usize, 22usize, 3u32, 2130552u32);
    emu.xrr_no_count(28usize, 29usize, 31usize, 2130556u32);
    emu.sw_no_count(28usize, 2usize, 328u32, 2130560u32)?;
    emu.sri_no_count(29usize, 20usize, 3u32, 2130564u32);
    emu.adi_no_count(22usize, 20usize, 0u32, 2130568u32);
    emu.sw_no_count(20usize, 2usize, 284u32, 2130572u32)?;
    emu.xrr_no_count(12usize, 12usize, 29usize, 2130576u32);
    emu.sw_no_count(12usize, 2usize, 280u32, 2130580u32)?;
    emu.adr_no_count(18usize, 7usize, 17usize, 2130584u32);
    emu.adr_no_count(7usize, 17usize, 26usize, 2130588u32);
    emu.lw_no_count(12usize, 2usize, 468u32, 2130592u32)?;
    emu.lw_no_count(17usize, 2usize, 352u32, 2130596u32)?;
    emu.adr_no_count(12usize, 17usize, 12usize, 2130600u32);
    emu.lw_no_count(17usize, 2usize, 488u32, 2130604u32)?;
    emu.adr_no_count(12usize, 12usize, 17usize, 2130608u32);
    emu.adr_no_count(28usize, 12usize, 30usize, 2130612u32);
    emu.lw_no_count(12usize, 2usize, 484u32, 2130616u32)?;
    emu.lw_no_count(17usize, 2usize, 332u32, 2130620u32)?;
    emu.adr_no_count(12usize, 17usize, 12usize, 2130624u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2130628u32);
    emu.adr_no_count(20usize, 12usize, 13usize, 2130632u32);
    emu.sri_no_count(12usize, 7usize, 6u32, 2130636u32);
    emu.sli_no_count(13usize, 7usize, 26u32, 2130640u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2130644u32);
    emu.sri_no_count(13usize, 7usize, 11u32, 2130648u32);
    emu.sli_no_count(29usize, 7usize, 21u32, 2130652u32);
    emu.orr_no_count(13usize, 29usize, 13usize, 2130656u32);
    emu.sri_no_count(29usize, 7usize, 25u32, 2130660u32);
    emu.sli_no_count(30usize, 7usize, 7u32, 2130664u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2130668u32);
    emu.lw_no_count(27usize, 2usize, 408u32, 2130672u32)?;
    emu.adr_no_count(27usize, 27usize, 14usize, 2130676u32);
    emu.xrr_no_count(30usize, 6usize, 14usize, 2130680u32);
    emu.anr_no_count(30usize, 7usize, 30usize, 2130684u32);
    emu.xrr_no_count(14usize, 30usize, 14usize, 2130688u32);
    emu.sri_no_count(30usize, 18usize, 2u32, 2130692u32);
    emu.sli_no_count(31usize, 18usize, 30u32, 2130696u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2130700u32);
    emu.sri_no_count(31usize, 18usize, 13u32, 2130704u32);
    emu.sli_no_count(8usize, 18usize, 19u32, 2130708u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2130712u32);
    emu.sri_no_count(8usize, 18usize, 22u32, 2130716u32);
    emu.sli_no_count(9usize, 18usize, 10u32, 2130720u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2130724u32);
    emu.xrr_no_count(9usize, 21usize, 10usize, 2130728u32);
    emu.anr_no_count(9usize, 18usize, 9usize, 2130732u32);
    emu.anr_no_count(19usize, 21usize, 10usize, 2130736u32);
    emu.xrr_no_count(9usize, 9usize, 19usize, 2130740u32);
    emu.adi_no_count(5usize, 28usize, 0u32, 2130744u32);
    emu.sri_no_count(19usize, 28usize, 17u32, 2130748u32);
    emu.sli_no_count(24usize, 28usize, 15u32, 2130752u32);
    emu.orr_no_count(19usize, 24usize, 19usize, 2130756u32);
    emu.sri_no_count(24usize, 28usize, 19u32, 2130760u32);
    emu.sli_no_count(25usize, 28usize, 13u32, 2130764u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2130768u32);
    emu.sri_no_count(25usize, 20usize, 17u32, 2130772u32);
    emu.sli_no_count(26usize, 20usize, 15u32, 2130776u32);
    emu.orr_no_count(25usize, 26usize, 25usize, 2130780u32);
    emu.sri_no_count(26usize, 20usize, 19u32, 2130784u32);
    emu.sli_no_count(23usize, 20usize, 13u32, 2130788u32);
    emu.sw_no_count(20usize, 2usize, 420u32, 2130792u32)?;
    emu.orr_no_count(23usize, 23usize, 26usize, 2130796u32);
    emu.sri_no_count(26usize, 28usize, 7u32, 2130800u32);
    emu.sli_no_count(17usize, 28usize, 25u32, 2130804u32);
    emu.orr_no_count(17usize, 17usize, 26usize, 2130808u32);
    emu.sri_no_count(26usize, 28usize, 18u32, 2130812u32);
    emu.sli_no_count(28usize, 28usize, 14u32, 2130816u32);
    emu.orr_no_count(28usize, 28usize, 26usize, 2130820u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2130824u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2130828u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2130832u32);
    emu.xrr_no_count(14usize, 19usize, 24usize, 2130836u32);
    emu.xrr_no_count(30usize, 25usize, 23usize, 2130840u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2130844u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2130848u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2130852u32);
    emu.sri_no_count(28usize, 5usize, 10u32, 2130856u32);
    emu.xrr_no_count(25usize, 14usize, 28usize, 2130860u32);
    emu.sri_no_count(14usize, 20usize, 10u32, 2130864u32);
    emu.xrr_no_count(26usize, 30usize, 14usize, 2130868u32);
    emu.sri_no_count(14usize, 5usize, 3u32, 2130872u32);
    emu.sw_no_count(5usize, 2usize, 340u32, 2130876u32)?;
    emu.xrr_no_count(14usize, 17usize, 14usize, 2130880u32);
    emu.sw_no_count(14usize, 2usize, 296u32, 2130884u32)?;
    emu.lw_no_count(14usize, 2usize, 204u32, 2130888u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2130892u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2130896u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2130900u32);
    emu.lw_no_count(12usize, 2usize, 464u32, 2130904u32)?;
    emu.lw_no_count(14usize, 2usize, 348u32, 2130908u32)?;
    emu.adr_no_count(12usize, 14usize, 12usize, 2130912u32);
    emu.lw_no_count(20usize, 2usize, 380u32, 2130916u32)?;
    emu.adr_no_count(12usize, 12usize, 20usize, 2130920u32);
    emu.adr_no_count(25usize, 12usize, 25usize, 2130924u32);
    emu.lw_no_count(12usize, 2usize, 480u32, 2130928u32)?;
    emu.lw_no_count(14usize, 2usize, 312u32, 2130932u32)?;
    emu.adr_no_count(12usize, 14usize, 12usize, 2130936u32);
    emu.adr_no_count(12usize, 12usize, 15usize, 2130940u32);
    emu.adr_no_count(26usize, 12usize, 26usize, 2130944u32);
    emu.adr_no_count(8usize, 13usize, 11usize, 2130948u32);
    emu.adr_no_count(24usize, 11usize, 1usize, 2130952u32);
    emu.sri_no_count(11usize, 25usize, 17u32, 2130956u32);
    emu.sli_no_count(12usize, 25usize, 15u32, 2130960u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2130964u32);
    emu.sri_no_count(12usize, 25usize, 19u32, 2130968u32);
    emu.sli_no_count(13usize, 25usize, 13u32, 2130972u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2130976u32);
    emu.sri_no_count(13usize, 26usize, 17u32, 2130980u32);
    emu.sli_no_count(14usize, 26usize, 15u32, 2130984u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2130988u32);
    emu.sri_no_count(14usize, 26usize, 19u32, 2130992u32);
    emu.sli_no_count(17usize, 26usize, 13u32, 2130996u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2131000u32);
    emu.sri_no_count(17usize, 24usize, 6u32, 2131004u32);
    emu.sli_no_count(28usize, 24usize, 26u32, 2131008u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2131012u32);
    emu.sri_no_count(28usize, 24usize, 11u32, 2131016u32);
    emu.sli_no_count(29usize, 24usize, 21u32, 2131020u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2131024u32);
    emu.sri_no_count(29usize, 24usize, 25u32, 2131028u32);
    emu.sli_no_count(30usize, 24usize, 7u32, 2131032u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2131036u32);
    emu.lw_no_count(30usize, 2usize, 492u32, 2131040u32)?;
    emu.adr_no_count(30usize, 30usize, 6usize, 2131044u32);
    emu.xrr_no_count(31usize, 7usize, 6usize, 2131048u32);
    emu.anr_no_count(31usize, 24usize, 31usize, 2131052u32);
    emu.xrr_no_count(6usize, 31usize, 6usize, 2131056u32);
    emu.sri_no_count(31usize, 8usize, 2u32, 2131060u32);
    emu.sli_no_count(9usize, 8usize, 30u32, 2131064u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2131068u32);
    emu.sri_no_count(9usize, 8usize, 13u32, 2131072u32);
    emu.sli_no_count(19usize, 8usize, 19u32, 2131076u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2131080u32);
    emu.sri_no_count(19usize, 8usize, 22u32, 2131084u32);
    emu.sli_no_count(23usize, 8usize, 10u32, 2131088u32);
    emu.orr_no_count(19usize, 23usize, 19usize, 2131092u32);
    emu.xrr_no_count(23usize, 18usize, 21usize, 2131096u32);
    emu.anr_no_count(23usize, 8usize, 23usize, 2131100u32);
    emu.anr_no_count(1usize, 18usize, 21usize, 2131104u32);
    emu.xrr_no_count(23usize, 23usize, 1usize, 2131108u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2131112u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2131116u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2131120u32);
    emu.adr_no_count(6usize, 27usize, 6usize, 2131124u32);
    emu.xrr_no_count(14usize, 31usize, 9usize, 2131128u32);
    emu.sri_no_count(17usize, 25usize, 10u32, 2131132u32);
    emu.sw_no_count(25usize, 2usize, 344u32, 2131136u32)?;
    emu.xrr_no_count(27usize, 11usize, 17usize, 2131140u32);
    emu.sri_no_count(11usize, 26usize, 10u32, 2131144u32);
    emu.sw_no_count(26usize, 2usize, 348u32, 2131148u32)?;
    emu.xrr_no_count(11usize, 13usize, 11usize, 2131152u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2131156u32);
    emu.xrr_no_count(13usize, 14usize, 19usize, 2131160u32);
    emu.lw_no_count(14usize, 2usize, 388u32, 2131164u32)?;
    emu.lw_no_count(15usize, 2usize, 324u32, 2131168u32)?;
    emu.adr_no_count(14usize, 15usize, 14usize, 2131172u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2131176u32);
    emu.adr_no_count(27usize, 14usize, 27usize, 2131180u32);
    emu.lw_no_count(14usize, 2usize, 448u32, 2131184u32)?;
    emu.lw_no_count(15usize, 2usize, 308u32, 2131188u32)?;
    emu.adr_no_count(14usize, 15usize, 14usize, 2131192u32);
    emu.adr_no_count(14usize, 14usize, 22usize, 2131196u32);
    emu.adr_no_count(22usize, 14usize, 11usize, 2131200u32);
    emu.lw_no_count(11usize, 2usize, 200u32, 2131204u32)?;
    emu.adr_no_count(6usize, 6usize, 11usize, 2131208u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2131212u32);
    emu.adr_no_count(13usize, 13usize, 23usize, 2131216u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2131220u32);
    emu.adr_no_count(6usize, 12usize, 10usize, 2131224u32);
    emu.sri_no_count(10usize, 27usize, 17u32, 2131228u32);
    emu.sli_no_count(11usize, 27usize, 15u32, 2131232u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2131236u32);
    emu.sri_no_count(11usize, 27usize, 19u32, 2131240u32);
    emu.sli_no_count(12usize, 27usize, 13u32, 2131244u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2131248u32);
    emu.sri_no_count(12usize, 22usize, 17u32, 2131252u32);
    emu.sli_no_count(13usize, 22usize, 15u32, 2131256u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2131260u32);
    emu.sri_no_count(13usize, 22usize, 19u32, 2131264u32);
    emu.sli_no_count(17usize, 22usize, 13u32, 2131268u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2131272u32);
    emu.xrr_no_count(10usize, 10usize, 11usize, 2131276u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2131280u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2131284u32);
    emu.sli_no_count(13usize, 6usize, 26u32, 2131288u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2131292u32);
    emu.sri_no_count(11usize, 6usize, 11u32, 2131296u32);
    emu.sli_no_count(17usize, 6usize, 21u32, 2131300u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2131304u32);
    emu.sri_no_count(11usize, 6usize, 25u32, 2131308u32);
    emu.sli_no_count(28usize, 6usize, 7u32, 2131312u32);
    emu.orr_no_count(28usize, 28usize, 11usize, 2131316u32);
    emu.lw_no_count(11usize, 2usize, 496u32, 2131320u32)?;
    emu.adr_no_count(11usize, 11usize, 7usize, 2131324u32);
    emu.xrr_no_count(29usize, 24usize, 7usize, 2131328u32);
    emu.anr_no_count(29usize, 6usize, 29usize, 2131332u32);
    emu.xrr_no_count(7usize, 29usize, 7usize, 2131336u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2131340u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2131344u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2131348u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2131352u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2131356u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2131360u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2131364u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2131368u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2131372u32);
    emu.xrr_no_count(19usize, 8usize, 18usize, 2131376u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2131380u32);
    emu.anr_no_count(23usize, 8usize, 18usize, 2131384u32);
    emu.xrr_no_count(19usize, 19usize, 23usize, 2131388u32);
    emu.sri_no_count(23usize, 27usize, 10u32, 2131392u32);
    emu.sw_no_count(27usize, 2usize, 336u32, 2131396u32)?;
    emu.xrr_no_count(1usize, 10usize, 23usize, 2131400u32);
    emu.sri_no_count(10usize, 22usize, 10u32, 2131404u32);
    emu.sw_no_count(22usize, 2usize, 352u32, 2131408u32)?;
    emu.xrr_no_count(10usize, 12usize, 10usize, 2131412u32);
    emu.xrr_no_count(12usize, 13usize, 17usize, 2131416u32);
    emu.adr_no_count(7usize, 30usize, 7usize, 2131420u32);
    emu.xrr_no_count(13usize, 29usize, 31usize, 2131424u32);
    emu.lw_no_count(17usize, 2usize, 452u32, 2131428u32)?;
    emu.lw_no_count(15usize, 2usize, 316u32, 2131432u32)?;
    emu.adr_no_count(17usize, 15usize, 17usize, 2131436u32);
    emu.lw_no_count(15usize, 2usize, 416u32, 2131440u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2131444u32);
    emu.adr_no_count(1usize, 17usize, 1usize, 2131448u32);
    emu.lw_no_count(17usize, 2usize, 304u32, 2131452u32)?;
    emu.lw_no_count(15usize, 2usize, 376u32, 2131456u32)?;
    emu.adr_no_count(17usize, 17usize, 15usize, 2131460u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2131464u32);
    emu.adr_no_count(15usize, 17usize, 10usize, 2131468u32);
    emu.sw_no_count(15usize, 2usize, 496u32, 2131472u32)?;
    emu.xrr_no_count(10usize, 12usize, 28usize, 2131476u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2131480u32);
    emu.lw_no_count(12usize, 2usize, 196u32, 2131484u32)?;
    emu.adr_no_count(7usize, 7usize, 12usize, 2131488u32);
    emu.adr_no_count(7usize, 7usize, 10usize, 2131492u32);
    emu.adr_no_count(10usize, 13usize, 19usize, 2131496u32);
    emu.sri_no_count(12usize, 1usize, 17u32, 2131500u32);
    emu.sli_no_count(13usize, 1usize, 15u32, 2131504u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2131508u32);
    emu.sri_no_count(13usize, 1usize, 19u32, 2131512u32);
    emu.sli_no_count(17usize, 1usize, 13u32, 2131516u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2131520u32);
    emu.sri_no_count(17usize, 15usize, 17u32, 2131524u32);
    emu.sli_no_count(28usize, 15usize, 15u32, 2131528u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2131532u32);
    emu.sri_no_count(28usize, 15usize, 19u32, 2131536u32);
    emu.sli_no_count(29usize, 15usize, 13u32, 2131540u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2131544u32);
    emu.adr_no_count(16usize, 10usize, 7usize, 2131548u32);
    emu.adr_no_count(7usize, 7usize, 21usize, 2131552u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2131556u32);
    emu.xrr_no_count(13usize, 17usize, 28usize, 2131560u32);
    emu.sri_no_count(17usize, 1usize, 10u32, 2131564u32);
    emu.sw_no_count(1usize, 2usize, 356u32, 2131568u32)?;
    emu.xrr_no_count(12usize, 12usize, 17usize, 2131572u32);
    emu.sri_no_count(17usize, 15usize, 10u32, 2131576u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2131580u32);
    emu.sri_no_count(17usize, 7usize, 6u32, 2131584u32);
    emu.sli_no_count(28usize, 7usize, 26u32, 2131588u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2131592u32);
    emu.sri_no_count(28usize, 7usize, 11u32, 2131596u32);
    emu.sli_no_count(29usize, 7usize, 21u32, 2131600u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2131604u32);
    emu.sri_no_count(29usize, 7usize, 25u32, 2131608u32);
    emu.sli_no_count(30usize, 7usize, 7u32, 2131612u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2131616u32);
    emu.lw_no_count(30usize, 2usize, 440u32, 2131620u32)?;
    emu.adr_no_count(30usize, 30usize, 24usize, 2131624u32);
    emu.xrr_no_count(31usize, 6usize, 24usize, 2131628u32);
    emu.anr_no_count(31usize, 7usize, 31usize, 2131632u32);
    emu.xrr_no_count(31usize, 31usize, 24usize, 2131636u32);
    emu.sri_no_count(9usize, 16usize, 2u32, 2131640u32);
    emu.sli_no_count(19usize, 16usize, 30u32, 2131644u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2131648u32);
    emu.sri_no_count(19usize, 16usize, 13u32, 2131652u32);
    emu.sli_no_count(21usize, 16usize, 19u32, 2131656u32);
    emu.orr_no_count(19usize, 21usize, 19usize, 2131660u32);
    emu.sri_no_count(21usize, 16usize, 22u32, 2131664u32);
    emu.sli_no_count(23usize, 16usize, 10u32, 2131668u32);
    emu.orr_no_count(21usize, 23usize, 21usize, 2131672u32);
    emu.xrr_no_count(23usize, 14usize, 8usize, 2131676u32);
    emu.anr_no_count(23usize, 16usize, 23usize, 2131680u32);
    emu.anr_no_count(24usize, 14usize, 8usize, 2131684u32);
    emu.xrr_no_count(23usize, 23usize, 24usize, 2131688u32);
    emu.lw_no_count(24usize, 2usize, 384u32, 2131692u32)?;
    emu.lw_no_count(10usize, 2usize, 360u32, 2131696u32)?;
    emu.adr_no_count(24usize, 10usize, 24usize, 2131700u32);
    emu.lw_no_count(10usize, 2usize, 420u32, 2131704u32)?;
    emu.adr_no_count(24usize, 24usize, 10usize, 2131708u32);
    emu.adr_no_count(5usize, 24usize, 12usize, 2131712u32);
    emu.lw_no_count(12usize, 2usize, 300u32, 2131716u32)?;
    emu.lw_no_count(10usize, 2usize, 488u32, 2131720u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2131724u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2131728u32);
    emu.adr_no_count(15usize, 12usize, 13usize, 2131732u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2131736u32);
    emu.adr_no_count(11usize, 11usize, 31usize, 2131740u32);
    emu.xrr_no_count(13usize, 9usize, 19usize, 2131744u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2131748u32);
    emu.xrr_no_count(13usize, 13usize, 21usize, 2131752u32);
    emu.sri_no_count(17usize, 5usize, 17u32, 2131756u32);
    emu.sli_no_count(28usize, 5usize, 15u32, 2131760u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2131764u32);
    emu.sri_no_count(28usize, 5usize, 19u32, 2131768u32);
    emu.sli_no_count(29usize, 5usize, 13u32, 2131772u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2131776u32);
    emu.sri_no_count(29usize, 15usize, 17u32, 2131780u32);
    emu.sli_no_count(31usize, 15usize, 15u32, 2131784u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2131788u32);
    emu.sri_no_count(31usize, 15usize, 19u32, 2131792u32);
    emu.sli_no_count(9usize, 15usize, 13u32, 2131796u32);
    emu.sw_no_count(15usize, 2usize, 440u32, 2131800u32)?;
    emu.orr_no_count(31usize, 9usize, 31usize, 2131804u32);
    emu.lw_no_count(9usize, 2usize, 192u32, 2131808u32)?;
    emu.adr_no_count(11usize, 11usize, 9usize, 2131812u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2131816u32);
    emu.adr_no_count(13usize, 13usize, 23usize, 2131820u32);
    emu.xrr_no_count(12usize, 17usize, 28usize, 2131824u32);
    emu.xrr_no_count(17usize, 29usize, 31usize, 2131828u32);
    emu.adr_no_count(24usize, 13usize, 11usize, 2131832u32);
    emu.adr_no_count(18usize, 11usize, 18usize, 2131836u32);
    emu.sri_no_count(11usize, 5usize, 10u32, 2131840u32);
    emu.sw_no_count(5usize, 2usize, 332u32, 2131844u32)?;
    emu.xrr_no_count(11usize, 12usize, 11usize, 2131848u32);
    emu.sri_no_count(12usize, 15usize, 10u32, 2131852u32);
    emu.xrr_no_count(12usize, 17usize, 12usize, 2131856u32);
    emu.lw_no_count(13usize, 2usize, 412u32, 2131860u32)?;
    emu.lw_no_count(10usize, 2usize, 320u32, 2131864u32)?;
    emu.adr_no_count(13usize, 10usize, 13usize, 2131868u32);
    emu.adr_no_count(13usize, 13usize, 26usize, 2131872u32);
    emu.adr_no_count(10usize, 13usize, 11usize, 2131876u32);
    emu.lw_no_count(11usize, 2usize, 288u32, 2131880u32)?;
    emu.adr_no_count(11usize, 11usize, 20usize, 2131884u32);
    emu.adr_no_count(11usize, 11usize, 27usize, 2131888u32);
    emu.adr_no_count(15usize, 11usize, 12usize, 2131892u32);
    emu.sri_no_count(11usize, 18usize, 6u32, 2131896u32);
    emu.sli_no_count(12usize, 18usize, 26u32, 2131900u32);
    emu.orr_no_count(12usize, 12usize, 11usize, 2131904u32);
    emu.sri_no_count(11usize, 18usize, 11u32, 2131908u32);
    emu.sli_no_count(13usize, 18usize, 21u32, 2131912u32);
    emu.orr_no_count(13usize, 13usize, 11usize, 2131916u32);
    emu.sri_no_count(11usize, 18usize, 25u32, 2131920u32);
    emu.sli_no_count(17usize, 18usize, 7u32, 2131924u32);
    emu.orr_no_count(17usize, 17usize, 11usize, 2131928u32);
    emu.lw_no_count(11usize, 2usize, 504u32, 2131932u32)?;
    emu.adr_no_count(11usize, 11usize, 6usize, 2131936u32);
    emu.xrr_no_count(28usize, 7usize, 6usize, 2131940u32);
    emu.anr_no_count(28usize, 18usize, 28usize, 2131944u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2131948u32);
    emu.sri_no_count(28usize, 24usize, 2u32, 2131952u32);
    emu.sli_no_count(29usize, 24usize, 30u32, 2131956u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2131960u32);
    emu.sri_no_count(29usize, 24usize, 13u32, 2131964u32);
    emu.sli_no_count(31usize, 24usize, 19u32, 2131968u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2131972u32);
    emu.sri_no_count(31usize, 24usize, 22u32, 2131976u32);
    emu.sli_no_count(9usize, 24usize, 10u32, 2131980u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2131984u32);
    emu.xrr_no_count(9usize, 16usize, 14usize, 2131988u32);
    emu.anr_no_count(9usize, 24usize, 9usize, 2131992u32);
    emu.anr_no_count(19usize, 16usize, 14usize, 2131996u32);
    emu.xrr_no_count(9usize, 9usize, 19usize, 2132000u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2132004u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2132008u32);
    emu.xrr_no_count(13usize, 28usize, 29usize, 2132012u32);
    emu.sri_no_count(28usize, 10usize, 17u32, 2132016u32);
    emu.sli_no_count(29usize, 10usize, 15u32, 2132020u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2132024u32);
    emu.sri_no_count(29usize, 10usize, 19u32, 2132028u32);
    emu.sli_no_count(30usize, 10usize, 13u32, 2132032u32);
    emu.sw_no_count(10usize, 2usize, 364u32, 2132036u32)?;
    emu.orr_no_count(29usize, 30usize, 29usize, 2132040u32);
    emu.sri_no_count(30usize, 15usize, 17u32, 2132044u32);
    emu.sli_no_count(19usize, 15usize, 15u32, 2132048u32);
    emu.orr_no_count(30usize, 19usize, 30usize, 2132052u32);
    emu.sri_no_count(19usize, 15usize, 19u32, 2132056u32);
    emu.sli_no_count(21usize, 15usize, 13u32, 2132060u32);
    emu.sw_no_count(15usize, 2usize, 492u32, 2132064u32)?;
    emu.orr_no_count(19usize, 21usize, 19usize, 2132068u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2132072u32);
    emu.xrr_no_count(13usize, 13usize, 31usize, 2132076u32);
    emu.xrr_no_count(17usize, 28usize, 29usize, 2132080u32);
    emu.xrr_no_count(28usize, 30usize, 19usize, 2132084u32);
    emu.lw_no_count(29usize, 2usize, 188u32, 2132088u32)?;
    emu.adr_no_count(6usize, 6usize, 29usize, 2132092u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2132096u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2132100u32);
    emu.sri_no_count(6usize, 10usize, 10u32, 2132104u32);
    emu.xrr_no_count(17usize, 17usize, 6usize, 2132108u32);
    emu.sri_no_count(6usize, 15usize, 10u32, 2132112u32);
    emu.xrr_no_count(6usize, 28usize, 6usize, 2132116u32);
    emu.adr_no_count(21usize, 13usize, 12usize, 2132120u32);
    emu.adr_no_count(8usize, 12usize, 8usize, 2132124u32);
    emu.lw_no_count(27usize, 2usize, 276u32, 2132128u32)?;
    emu.lw_no_count(12usize, 2usize, 292u32, 2132132u32)?;
    emu.adr_no_count(12usize, 12usize, 27usize, 2132136u32);
    emu.adr_no_count(12usize, 12usize, 22usize, 2132140u32);
    emu.adr_no_count(10usize, 12usize, 17usize, 2132144u32);
    emu.lw_no_count(22usize, 2usize, 272u32, 2132148u32)?;
    emu.lw_no_count(12usize, 2usize, 280u32, 2132152u32)?;
    emu.adr_no_count(12usize, 12usize, 22usize, 2132156u32);
    emu.adr_no_count(12usize, 12usize, 1usize, 2132160u32);
    emu.adr_no_count(15usize, 12usize, 6usize, 2132164u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2132168u32);
    emu.sli_no_count(13usize, 8usize, 26u32, 2132172u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2132176u32);
    emu.sri_no_count(13usize, 8usize, 11u32, 2132180u32);
    emu.sli_no_count(17usize, 8usize, 21u32, 2132184u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2132188u32);
    emu.sri_no_count(17usize, 8usize, 25u32, 2132192u32);
    emu.sli_no_count(6usize, 8usize, 7u32, 2132196u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2132200u32);
    emu.lw_no_count(6usize, 2usize, 444u32, 2132204u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2132208u32);
    emu.xrr_no_count(28usize, 18usize, 7usize, 2132212u32);
    emu.anr_no_count(28usize, 8usize, 28usize, 2132216u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2132220u32);
    emu.sri_no_count(28usize, 21usize, 2u32, 2132224u32);
    emu.sli_no_count(29usize, 21usize, 30u32, 2132228u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2132232u32);
    emu.sri_no_count(29usize, 21usize, 13u32, 2132236u32);
    emu.sli_no_count(30usize, 21usize, 19u32, 2132240u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2132244u32);
    emu.sri_no_count(30usize, 21usize, 22u32, 2132248u32);
    emu.sli_no_count(31usize, 21usize, 10u32, 2132252u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2132256u32);
    emu.xrr_no_count(31usize, 24usize, 16usize, 2132260u32);
    emu.anr_no_count(31usize, 21usize, 31usize, 2132264u32);
    emu.anr_no_count(9usize, 24usize, 16usize, 2132268u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2132272u32);
    emu.sri_no_count(9usize, 10usize, 17u32, 2132276u32);
    emu.sli_no_count(19usize, 10usize, 15u32, 2132280u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2132284u32);
    emu.sri_no_count(19usize, 10usize, 19u32, 2132288u32);
    emu.sli_no_count(23usize, 10usize, 13u32, 2132292u32);
    emu.sw_no_count(10usize, 2usize, 408u32, 2132296u32)?;
    emu.orr_no_count(19usize, 23usize, 19usize, 2132300u32);
    emu.sri_no_count(23usize, 15usize, 17u32, 2132304u32);
    emu.sli_no_count(26usize, 15usize, 15u32, 2132308u32);
    emu.orr_no_count(23usize, 26usize, 23usize, 2132312u32);
    emu.sri_no_count(26usize, 15usize, 19u32, 2132316u32);
    emu.sli_no_count(1usize, 15usize, 13u32, 2132320u32);
    emu.sw_no_count(15usize, 2usize, 504u32, 2132324u32)?;
    emu.orr_no_count(26usize, 1usize, 26usize, 2132328u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2132332u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2132336u32);
    emu.xrr_no_count(11usize, 28usize, 29usize, 2132340u32);
    emu.xrr_no_count(13usize, 9usize, 19usize, 2132344u32);
    emu.xrr_no_count(28usize, 23usize, 26usize, 2132348u32);
    emu.xrr_no_count(23usize, 12usize, 17usize, 2132352u32);
    emu.xrr_no_count(12usize, 11usize, 30usize, 2132356u32);
    emu.sri_no_count(11usize, 10usize, 10u32, 2132360u32);
    emu.xrr_no_count(30usize, 13usize, 11usize, 2132364u32);
    emu.sri_no_count(11usize, 15usize, 10u32, 2132368u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2132372u32);
    emu.lw_no_count(13usize, 2usize, 184u32, 2132376u32)?;
    emu.adr_no_count(7usize, 7usize, 13usize, 2132380u32);
    emu.adr_no_count(23usize, 7usize, 23usize, 2132384u32);
    emu.adr_no_count(7usize, 12usize, 31usize, 2132388u32);
    emu.lw_no_count(20usize, 2usize, 284u32, 2132392u32)?;
    emu.lw_no_count(12usize, 2usize, 328u32, 2132396u32)?;
    emu.adr_no_count(12usize, 20usize, 12usize, 2132400u32);
    emu.lw_no_count(10usize, 2usize, 496u32, 2132404u32)?;
    emu.adr_no_count(12usize, 12usize, 10usize, 2132408u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2132412u32);
    emu.sw_no_count(12usize, 2usize, 444u32, 2132416u32)?;
    emu.lw_no_count(25usize, 2usize, 416u32, 2132420u32)?;
    emu.lw_no_count(12usize, 2usize, 296u32, 2132424u32)?;
    emu.adr_no_count(12usize, 25usize, 12usize, 2132428u32);
    emu.adr_no_count(12usize, 12usize, 5usize, 2132432u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2132436u32);
    emu.sw_no_count(11usize, 2usize, 360u32, 2132440u32)?;
    emu.adr_no_count(7usize, 7usize, 23usize, 2132444u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2132448u32);
    emu.sri_no_count(11usize, 23usize, 6u32, 2132452u32);
    emu.sli_no_count(12usize, 23usize, 26u32, 2132456u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2132460u32);
    emu.sri_no_count(12usize, 23usize, 11u32, 2132464u32);
    emu.sli_no_count(13usize, 23usize, 21u32, 2132468u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2132472u32);
    emu.sri_no_count(13usize, 23usize, 25u32, 2132476u32);
    emu.sli_no_count(14usize, 23usize, 7u32, 2132480u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2132484u32);
    emu.lw_no_count(17usize, 2usize, 500u32, 2132488u32)?;
    emu.adr_no_count(17usize, 17usize, 18usize, 2132492u32);
    emu.xrr_no_count(14usize, 8usize, 18usize, 2132496u32);
    emu.anr_no_count(14usize, 23usize, 14usize, 2132500u32);
    emu.xrr_no_count(14usize, 14usize, 18usize, 2132504u32);
    emu.sri_no_count(28usize, 7usize, 2u32, 2132508u32);
    emu.sli_no_count(29usize, 7usize, 30u32, 2132512u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2132516u32);
    emu.sri_no_count(29usize, 7usize, 13u32, 2132520u32);
    emu.sli_no_count(30usize, 7usize, 19u32, 2132524u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2132528u32);
    emu.sri_no_count(30usize, 7usize, 22u32, 2132532u32);
    emu.sli_no_count(31usize, 7usize, 10u32, 2132536u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2132540u32);
    emu.xrr_no_count(31usize, 21usize, 24usize, 2132544u32);
    emu.anr_no_count(31usize, 7usize, 31usize, 2132548u32);
    emu.anr_no_count(9usize, 21usize, 24usize, 2132552u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2132556u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2132560u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2132564u32);
    emu.xrr_no_count(12usize, 28usize, 29usize, 2132568u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2132572u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2132576u32);
    emu.lw_no_count(13usize, 2usize, 180u32, 2132580u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2132584u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2132588u32);
    emu.adr_no_count(12usize, 12usize, 31usize, 2132592u32);
    emu.adr_no_count(14usize, 12usize, 11usize, 2132596u32);
    emu.adr_no_count(11usize, 11usize, 16usize, 2132600u32);
    emu.sri_no_count(10usize, 11usize, 6u32, 2132604u32);
    emu.sli_no_count(12usize, 11usize, 26u32, 2132608u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2132612u32);
    emu.sri_no_count(12usize, 11usize, 11u32, 2132616u32);
    emu.sli_no_count(13usize, 11usize, 21u32, 2132620u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2132624u32);
    emu.sri_no_count(13usize, 11usize, 25u32, 2132628u32);
    emu.sli_no_count(6usize, 11usize, 7u32, 2132632u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2132636u32);
    emu.lw_no_count(6usize, 2usize, 476u32, 2132640u32)?;
    emu.adr_no_count(6usize, 6usize, 8usize, 2132644u32);
    emu.xrr_no_count(28usize, 23usize, 8usize, 2132648u32);
    emu.anr_no_count(28usize, 11usize, 28usize, 2132652u32);
    emu.xrr_no_count(28usize, 28usize, 8usize, 2132656u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2132660u32);
    emu.sli_no_count(30usize, 14usize, 30u32, 2132664u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2132668u32);
    emu.sri_no_count(30usize, 14usize, 13u32, 2132672u32);
    emu.sli_no_count(31usize, 14usize, 19u32, 2132676u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2132680u32);
    emu.sri_no_count(31usize, 14usize, 22u32, 2132684u32);
    emu.sli_no_count(8usize, 14usize, 10u32, 2132688u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2132692u32);
    emu.xrr_no_count(8usize, 7usize, 21usize, 2132696u32);
    emu.anr_no_count(8usize, 14usize, 8usize, 2132700u32);
    emu.anr_no_count(9usize, 7usize, 21usize, 2132704u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2132708u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2132712u32);
    emu.adr_no_count(17usize, 17usize, 28usize, 2132716u32);
    emu.xrr_no_count(12usize, 29usize, 30usize, 2132720u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2132724u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2132728u32);
    emu.lw_no_count(13usize, 2usize, 176u32, 2132732u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2132736u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2132740u32);
    emu.adr_no_count(10usize, 12usize, 8usize, 2132744u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2132748u32);
    emu.adr_no_count(24usize, 17usize, 24usize, 2132752u32);
    emu.sri_no_count(12usize, 24usize, 6u32, 2132756u32);
    emu.sli_no_count(13usize, 24usize, 26u32, 2132760u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2132764u32);
    emu.sri_no_count(13usize, 24usize, 11u32, 2132768u32);
    emu.sli_no_count(17usize, 24usize, 21u32, 2132772u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2132776u32);
    emu.sri_no_count(17usize, 24usize, 25u32, 2132780u32);
    emu.sli_no_count(28usize, 24usize, 7u32, 2132784u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2132788u32);
    emu.lw_no_count(28usize, 2usize, 404u32, 2132792u32)?;
    emu.adr_no_count(28usize, 28usize, 23usize, 2132796u32);
    emu.xrr_no_count(29usize, 11usize, 23usize, 2132800u32);
    emu.anr_no_count(29usize, 24usize, 29usize, 2132804u32);
    emu.xrr_no_count(29usize, 29usize, 23usize, 2132808u32);
    emu.sri_no_count(30usize, 10usize, 2u32, 2132812u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2132816u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2132820u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2132824u32);
    emu.sli_no_count(8usize, 10usize, 19u32, 2132828u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2132832u32);
    emu.sri_no_count(8usize, 10usize, 22u32, 2132836u32);
    emu.sli_no_count(9usize, 10usize, 10u32, 2132840u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2132844u32);
    emu.xrr_no_count(9usize, 14usize, 7usize, 2132848u32);
    emu.anr_no_count(9usize, 10usize, 9usize, 2132852u32);
    emu.anr_no_count(18usize, 14usize, 7usize, 2132856u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2132860u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2132864u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2132868u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2132872u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2132876u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2132880u32);
    emu.lw_no_count(17usize, 2usize, 172u32, 2132884u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2132888u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2132892u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2132896u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2132900u32);
    emu.adr_no_count(21usize, 12usize, 21usize, 2132904u32);
    emu.sri_no_count(12usize, 21usize, 6u32, 2132908u32);
    emu.sli_no_count(13usize, 21usize, 26u32, 2132912u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2132916u32);
    emu.sri_no_count(13usize, 21usize, 11u32, 2132920u32);
    emu.sli_no_count(17usize, 21usize, 21u32, 2132924u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2132928u32);
    emu.sri_no_count(17usize, 21usize, 25u32, 2132932u32);
    emu.sli_no_count(29usize, 21usize, 7u32, 2132936u32);
    emu.orr_no_count(17usize, 29usize, 17usize, 2132940u32);
    emu.lw_no_count(29usize, 2usize, 472u32, 2132944u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2132948u32);
    emu.xrr_no_count(30usize, 24usize, 11usize, 2132952u32);
    emu.anr_no_count(30usize, 21usize, 30usize, 2132956u32);
    emu.xrr_no_count(11usize, 30usize, 11usize, 2132960u32);
    emu.sri_no_count(30usize, 6usize, 2u32, 2132964u32);
    emu.sli_no_count(31usize, 6usize, 30u32, 2132968u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2132972u32);
    emu.sri_no_count(31usize, 6usize, 13u32, 2132976u32);
    emu.sli_no_count(8usize, 6usize, 19u32, 2132980u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2132984u32);
    emu.sri_no_count(8usize, 6usize, 22u32, 2132988u32);
    emu.sli_no_count(9usize, 6usize, 10u32, 2132992u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2132996u32);
    emu.xrr_no_count(9usize, 10usize, 14usize, 2133000u32);
    emu.anr_no_count(9usize, 6usize, 9usize, 2133004u32);
    emu.anr_no_count(18usize, 10usize, 14usize, 2133008u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2133012u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2133016u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2133020u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2133024u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2133028u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2133032u32);
    emu.lw_no_count(17usize, 2usize, 168u32, 2133036u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2133040u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2133044u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2133048u32);
    emu.adr_no_count(8usize, 13usize, 11usize, 2133052u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2133056u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2133060u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2133064u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2133068u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2133072u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2133076u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2133080u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2133084u32);
    emu.sli_no_count(7usize, 11usize, 7u32, 2133088u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2133092u32);
    emu.lw_no_count(28usize, 2usize, 400u32, 2133096u32)?;
    emu.adr_no_count(28usize, 28usize, 24usize, 2133100u32);
    emu.xrr_no_count(7usize, 21usize, 24usize, 2133104u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2133108u32);
    emu.xrr_no_count(7usize, 7usize, 24usize, 2133112u32);
    emu.sri_no_count(30usize, 8usize, 2u32, 2133116u32);
    emu.sli_no_count(31usize, 8usize, 30u32, 2133120u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2133124u32);
    emu.sri_no_count(31usize, 8usize, 13u32, 2133128u32);
    emu.sli_no_count(9usize, 8usize, 19u32, 2133132u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2133136u32);
    emu.sri_no_count(9usize, 8usize, 22u32, 2133140u32);
    emu.sli_no_count(18usize, 8usize, 10u32, 2133144u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2133148u32);
    emu.xrr_no_count(18usize, 6usize, 10usize, 2133152u32);
    emu.anr_no_count(18usize, 8usize, 18usize, 2133156u32);
    emu.anr_no_count(19usize, 6usize, 10usize, 2133160u32);
    emu.xrr_no_count(18usize, 18usize, 19usize, 2133164u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2133168u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2133172u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2133176u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2133180u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2133184u32);
    emu.lw_no_count(17usize, 2usize, 164u32, 2133188u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2133192u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2133196u32);
    emu.adr_no_count(13usize, 13usize, 18usize, 2133200u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2133204u32);
    emu.adr_no_count(30usize, 12usize, 14usize, 2133208u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2133212u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2133216u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2133220u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2133224u32);
    emu.sli_no_count(14usize, 30usize, 21u32, 2133228u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2133232u32);
    emu.sri_no_count(14usize, 30usize, 25u32, 2133236u32);
    emu.sli_no_count(17usize, 30usize, 7u32, 2133240u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2133244u32);
    emu.lw_no_count(17usize, 2usize, 460u32, 2133248u32)?;
    emu.adr_no_count(17usize, 17usize, 21usize, 2133252u32);
    emu.xrr_no_count(29usize, 11usize, 21usize, 2133256u32);
    emu.anr_no_count(29usize, 30usize, 29usize, 2133260u32);
    emu.xrr_no_count(29usize, 29usize, 21usize, 2133264u32);
    emu.sri_no_count(31usize, 7usize, 2u32, 2133268u32);
    emu.sli_no_count(9usize, 7usize, 30u32, 2133272u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2133276u32);
    emu.sri_no_count(9usize, 7usize, 13u32, 2133280u32);
    emu.sli_no_count(18usize, 7usize, 19u32, 2133284u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2133288u32);
    emu.sri_no_count(18usize, 7usize, 22u32, 2133292u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2133296u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2133300u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2133304u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2133308u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2133312u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2133316u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2133320u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2133324u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2133328u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2133332u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2133336u32);
    emu.lw_no_count(14usize, 2usize, 160u32, 2133340u32)?;
    emu.adr_no_count(28usize, 28usize, 14usize, 2133344u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2133348u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2133352u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2133356u32);
    emu.adr_no_count(18usize, 12usize, 10usize, 2133360u32);
    emu.sri_no_count(10usize, 18usize, 6u32, 2133364u32);
    emu.sli_no_count(12usize, 18usize, 26u32, 2133368u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2133372u32);
    emu.sri_no_count(12usize, 18usize, 11u32, 2133376u32);
    emu.sli_no_count(13usize, 18usize, 21u32, 2133380u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2133384u32);
    emu.sri_no_count(13usize, 18usize, 25u32, 2133388u32);
    emu.sli_no_count(28usize, 18usize, 7u32, 2133392u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2133396u32);
    emu.lw_no_count(28usize, 2usize, 396u32, 2133400u32)?;
    emu.adr_no_count(28usize, 28usize, 11usize, 2133404u32);
    emu.xrr_no_count(29usize, 30usize, 11usize, 2133408u32);
    emu.anr_no_count(29usize, 18usize, 29usize, 2133412u32);
    emu.xrr_no_count(11usize, 29usize, 11usize, 2133416u32);
    emu.sri_no_count(29usize, 14usize, 2u32, 2133420u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2133424u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2133428u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2133432u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2133436u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2133440u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2133444u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2133448u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2133452u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2133456u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2133460u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2133464u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2133468u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2133472u32);
    emu.adr_no_count(11usize, 17usize, 11usize, 2133476u32);
    emu.xrr_no_count(12usize, 29usize, 31usize, 2133480u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2133484u32);
    emu.xrr_no_count(12usize, 12usize, 9usize, 2133488u32);
    emu.lw_no_count(13usize, 2usize, 156u32, 2133492u32)?;
    emu.adr_no_count(11usize, 11usize, 13usize, 2133496u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2133500u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2133504u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2133508u32);
    emu.adr_no_count(11usize, 11usize, 6usize, 2133512u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2133516u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2133520u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2133524u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2133528u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2133532u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2133536u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2133540u32);
    emu.sli_no_count(6usize, 11usize, 7u32, 2133544u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2133548u32);
    emu.lw_no_count(29usize, 2usize, 456u32, 2133552u32)?;
    emu.adr_no_count(29usize, 29usize, 30usize, 2133556u32);
    emu.xrr_no_count(6usize, 18usize, 30usize, 2133560u32);
    emu.anr_no_count(6usize, 11usize, 6usize, 2133564u32);
    emu.xrr_no_count(6usize, 6usize, 30usize, 2133568u32);
    emu.sri_no_count(30usize, 10usize, 2u32, 2133572u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2133576u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2133580u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2133584u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2133588u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2133592u32);
    emu.sri_no_count(9usize, 10usize, 22u32, 2133596u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2133600u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2133604u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2133608u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2133612u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2133616u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2133620u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2133624u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2133628u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2133632u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2133636u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2133640u32);
    emu.lw_no_count(17usize, 2usize, 152u32, 2133644u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2133648u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2133652u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2133656u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2133660u32);
    emu.adr_no_count(30usize, 12usize, 8usize, 2133664u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2133668u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2133672u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2133676u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2133680u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2133684u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2133688u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2133692u32);
    emu.sli_no_count(28usize, 30usize, 7u32, 2133696u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2133700u32);
    emu.lw_no_count(28usize, 2usize, 392u32, 2133704u32)?;
    emu.adr_no_count(28usize, 28usize, 18usize, 2133708u32);
    emu.xrr_no_count(31usize, 11usize, 18usize, 2133712u32);
    emu.anr_no_count(31usize, 30usize, 31usize, 2133716u32);
    emu.xrr_no_count(31usize, 31usize, 18usize, 2133720u32);
    emu.sri_no_count(8usize, 6usize, 2u32, 2133724u32);
    emu.sli_no_count(9usize, 6usize, 30u32, 2133728u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2133732u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2133736u32);
    emu.sli_no_count(18usize, 6usize, 19u32, 2133740u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2133744u32);
    emu.sri_no_count(18usize, 6usize, 22u32, 2133748u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2133752u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2133756u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2133760u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2133764u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2133768u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2133772u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2133776u32);
    emu.adr_no_count(29usize, 29usize, 31usize, 2133780u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2133784u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2133788u32);
    emu.xrr_no_count(13usize, 8usize, 18usize, 2133792u32);
    emu.lw_no_count(17usize, 2usize, 148u32, 2133796u32)?;
    emu.adr_no_count(29usize, 29usize, 17usize, 2133800u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2133804u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2133808u32);
    emu.adr_no_count(8usize, 13usize, 12usize, 2133812u32);
    emu.adr_no_count(18usize, 12usize, 7usize, 2133816u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2133820u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2133824u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2133828u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2133832u32);
    emu.sli_no_count(17usize, 18usize, 21u32, 2133836u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2133840u32);
    emu.sri_no_count(17usize, 18usize, 25u32, 2133844u32);
    emu.sli_no_count(7usize, 18usize, 7u32, 2133848u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2133852u32);
    emu.lw_no_count(29usize, 2usize, 436u32, 2133856u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2133860u32);
    emu.xrr_no_count(7usize, 30usize, 11usize, 2133864u32);
    emu.anr_no_count(7usize, 18usize, 7usize, 2133868u32);
    emu.xrr_no_count(11usize, 7usize, 11usize, 2133872u32);
    emu.sri_no_count(7usize, 8usize, 2u32, 2133876u32);
    emu.sli_no_count(31usize, 8usize, 30u32, 2133880u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2133884u32);
    emu.sri_no_count(31usize, 8usize, 13u32, 2133888u32);
    emu.sli_no_count(9usize, 8usize, 19u32, 2133892u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2133896u32);
    emu.sri_no_count(9usize, 8usize, 22u32, 2133900u32);
    emu.sli_no_count(19usize, 8usize, 10u32, 2133904u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2133908u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2133912u32);
    emu.anr_no_count(19usize, 8usize, 19usize, 2133916u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2133920u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2133924u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2133928u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2133932u32);
    emu.xrr_no_count(13usize, 7usize, 31usize, 2133936u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2133940u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2133944u32);
    emu.lw_no_count(17usize, 2usize, 144u32, 2133948u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2133952u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2133956u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2133960u32);
    emu.adr_no_count(7usize, 13usize, 11usize, 2133964u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2133968u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2133972u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2133976u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2133980u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2133984u32);
    emu.sli_no_count(14usize, 11usize, 21u32, 2133988u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2133992u32);
    emu.sri_no_count(14usize, 11usize, 25u32, 2133996u32);
    emu.sli_no_count(17usize, 11usize, 7u32, 2134000u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2134004u32);
    emu.lw_no_count(17usize, 2usize, 468u32, 2134008u32)?;
    emu.adr_no_count(17usize, 17usize, 30usize, 2134012u32);
    emu.xrr_no_count(28usize, 18usize, 30usize, 2134016u32);
    emu.anr_no_count(28usize, 11usize, 28usize, 2134020u32);
    emu.xrr_no_count(28usize, 28usize, 30usize, 2134024u32);
    emu.sri_no_count(30usize, 7usize, 2u32, 2134028u32);
    emu.sli_no_count(31usize, 7usize, 30u32, 2134032u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2134036u32);
    emu.sri_no_count(31usize, 7usize, 13u32, 2134040u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2134044u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2134048u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2134052u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2134056u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2134060u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2134064u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2134068u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2134072u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2134076u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2134080u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2134084u32);
    emu.xrr_no_count(13usize, 30usize, 31usize, 2134088u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2134092u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2134096u32);
    emu.lw_no_count(14usize, 2usize, 140u32, 2134100u32)?;
    emu.adr_no_count(28usize, 28usize, 14usize, 2134104u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2134108u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2134112u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2134116u32);
    emu.adr_no_count(30usize, 12usize, 10usize, 2134120u32);
    emu.sri_no_count(10usize, 30usize, 6u32, 2134124u32);
    emu.sli_no_count(12usize, 30usize, 26u32, 2134128u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2134132u32);
    emu.sri_no_count(12usize, 30usize, 11u32, 2134136u32);
    emu.sli_no_count(13usize, 30usize, 21u32, 2134140u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2134144u32);
    emu.sri_no_count(13usize, 30usize, 25u32, 2134148u32);
    emu.sli_no_count(28usize, 30usize, 7u32, 2134152u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2134156u32);
    emu.lw_no_count(28usize, 2usize, 484u32, 2134160u32)?;
    emu.adr_no_count(28usize, 28usize, 18usize, 2134164u32);
    emu.xrr_no_count(29usize, 11usize, 18usize, 2134168u32);
    emu.anr_no_count(29usize, 30usize, 29usize, 2134172u32);
    emu.xrr_no_count(29usize, 29usize, 18usize, 2134176u32);
    emu.sri_no_count(31usize, 14usize, 2u32, 2134180u32);
    emu.sli_no_count(9usize, 14usize, 30u32, 2134184u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2134188u32);
    emu.sri_no_count(9usize, 14usize, 13u32, 2134192u32);
    emu.sli_no_count(18usize, 14usize, 19u32, 2134196u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2134200u32);
    emu.sri_no_count(18usize, 14usize, 22u32, 2134204u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2134208u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2134212u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2134216u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2134220u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2134224u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2134228u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2134232u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2134236u32);
    emu.xrr_no_count(12usize, 31usize, 9usize, 2134240u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2134244u32);
    emu.xrr_no_count(12usize, 12usize, 18usize, 2134248u32);
    emu.lw_no_count(13usize, 2usize, 136u32, 2134252u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2134256u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2134260u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2134264u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2134268u32);
    emu.adr_no_count(18usize, 17usize, 6usize, 2134272u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2134276u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2134280u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2134284u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2134288u32);
    emu.sli_no_count(17usize, 18usize, 21u32, 2134292u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2134296u32);
    emu.sri_no_count(17usize, 18usize, 25u32, 2134300u32);
    emu.sli_no_count(6usize, 18usize, 7u32, 2134304u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2134308u32);
    emu.lw_no_count(29usize, 2usize, 464u32, 2134312u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2134316u32);
    emu.xrr_no_count(6usize, 30usize, 11usize, 2134320u32);
    emu.anr_no_count(6usize, 18usize, 6usize, 2134324u32);
    emu.xrr_no_count(11usize, 6usize, 11usize, 2134328u32);
    emu.sri_no_count(6usize, 10usize, 2u32, 2134332u32);
    emu.sli_no_count(31usize, 10usize, 30u32, 2134336u32);
    emu.orr_no_count(6usize, 31usize, 6usize, 2134340u32);
    emu.sri_no_count(31usize, 10usize, 13u32, 2134344u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2134348u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2134352u32);
    emu.sri_no_count(9usize, 10usize, 22u32, 2134356u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2134360u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2134364u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2134368u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2134372u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2134376u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2134380u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2134384u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2134388u32);
    emu.xrr_no_count(13usize, 6usize, 31usize, 2134392u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2134396u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2134400u32);
    emu.lw_no_count(17usize, 2usize, 132u32, 2134404u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2134408u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2134412u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2134416u32);
    emu.adr_no_count(6usize, 13usize, 11usize, 2134420u32);
    emu.adr_no_count(11usize, 11usize, 8usize, 2134424u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2134428u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2134432u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2134436u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2134440u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2134444u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2134448u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2134452u32);
    emu.sli_no_count(28usize, 11usize, 7u32, 2134456u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2134460u32);
    emu.lw_no_count(28usize, 2usize, 480u32, 2134464u32)?;
    emu.adr_no_count(28usize, 28usize, 30usize, 2134468u32);
    emu.xrr_no_count(31usize, 18usize, 30usize, 2134472u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2134476u32);
    emu.xrr_no_count(30usize, 31usize, 30usize, 2134480u32);
    emu.sri_no_count(31usize, 6usize, 2u32, 2134484u32);
    emu.sli_no_count(8usize, 6usize, 30u32, 2134488u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2134492u32);
    emu.sri_no_count(8usize, 6usize, 13u32, 2134496u32);
    emu.sli_no_count(9usize, 6usize, 19u32, 2134500u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2134504u32);
    emu.sri_no_count(9usize, 6usize, 22u32, 2134508u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2134512u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2134516u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2134520u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2134524u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2134528u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2134532u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2134536u32);
    emu.adr_no_count(29usize, 29usize, 30usize, 2134540u32);
    emu.xrr_no_count(13usize, 31usize, 8usize, 2134544u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2134548u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2134552u32);
    emu.lw_no_count(17usize, 2usize, 128u32, 2134556u32)?;
    emu.adr_no_count(29usize, 29usize, 17usize, 2134560u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2134564u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2134568u32);
    emu.adr_no_count(8usize, 13usize, 12usize, 2134572u32);
    emu.adr_no_count(30usize, 12usize, 7usize, 2134576u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2134580u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2134584u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2134588u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2134592u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2134596u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2134600u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2134604u32);
    emu.sli_no_count(7usize, 30usize, 7u32, 2134608u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2134612u32);
    emu.lw_no_count(29usize, 2usize, 388u32, 2134616u32)?;
    emu.adr_no_count(29usize, 29usize, 18usize, 2134620u32);
    emu.xrr_no_count(7usize, 11usize, 18usize, 2134624u32);
    emu.anr_no_count(7usize, 30usize, 7usize, 2134628u32);
    emu.xrr_no_count(7usize, 7usize, 18usize, 2134632u32);
    emu.sri_no_count(31usize, 8usize, 2u32, 2134636u32);
    emu.sli_no_count(9usize, 8usize, 30u32, 2134640u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2134644u32);
    emu.sri_no_count(9usize, 8usize, 13u32, 2134648u32);
    emu.sli_no_count(18usize, 8usize, 19u32, 2134652u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2134656u32);
    emu.sri_no_count(18usize, 8usize, 22u32, 2134660u32);
    emu.sli_no_count(19usize, 8usize, 10u32, 2134664u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2134668u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2134672u32);
    emu.anr_no_count(19usize, 8usize, 19usize, 2134676u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2134680u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2134684u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2134688u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2134692u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2134696u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2134700u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2134704u32);
    emu.lw_no_count(17usize, 2usize, 124u32, 2134708u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2134712u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2134716u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2134720u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2134724u32);
    emu.adr_no_count(18usize, 12usize, 14usize, 2134728u32);
    emu.sri_no_count(12usize, 18usize, 6u32, 2134732u32);
    emu.sli_no_count(13usize, 18usize, 26u32, 2134736u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2134740u32);
    emu.sri_no_count(13usize, 18usize, 11u32, 2134744u32);
    emu.sli_no_count(14usize, 18usize, 21u32, 2134748u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2134752u32);
    emu.sri_no_count(14usize, 18usize, 25u32, 2134756u32);
    emu.sli_no_count(17usize, 18usize, 7u32, 2134760u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2134764u32);
    emu.lw_no_count(17usize, 2usize, 448u32, 2134768u32)?;
    emu.adr_no_count(17usize, 17usize, 11usize, 2134772u32);
    emu.xrr_no_count(28usize, 30usize, 11usize, 2134776u32);
    emu.anr_no_count(28usize, 18usize, 28usize, 2134780u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2134784u32);
    emu.sri_no_count(28usize, 7usize, 2u32, 2134788u32);
    emu.sli_no_count(31usize, 7usize, 30u32, 2134792u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2134796u32);
    emu.sri_no_count(31usize, 7usize, 13u32, 2134800u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2134804u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2134808u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2134812u32);
    emu.sli_no_count(19usize, 7usize, 10u32, 2134816u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2134820u32);
    emu.xrr_no_count(19usize, 8usize, 6usize, 2134824u32);
    emu.anr_no_count(19usize, 7usize, 19usize, 2134828u32);
    emu.anr_no_count(21usize, 8usize, 6usize, 2134832u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2134836u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2134840u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2134844u32);
    emu.xrr_no_count(13usize, 28usize, 31usize, 2134848u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2134852u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2134856u32);
    emu.lw_no_count(14usize, 2usize, 120u32, 2134860u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2134864u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2134868u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2134872u32);
    emu.adr_no_count(14usize, 13usize, 11usize, 2134876u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2134880u32);
    emu.sri_no_count(10usize, 11usize, 6u32, 2134884u32);
    emu.sli_no_count(12usize, 11usize, 26u32, 2134888u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2134892u32);
    emu.sri_no_count(12usize, 11usize, 11u32, 2134896u32);
    emu.sli_no_count(13usize, 11usize, 21u32, 2134900u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2134904u32);
    emu.sri_no_count(13usize, 11usize, 25u32, 2134908u32);
    emu.sli_no_count(28usize, 11usize, 7u32, 2134912u32);
    emu.orr_no_count(13usize, 28usize, 13usize, 2134916u32);
    emu.lw_no_count(28usize, 2usize, 452u32, 2134920u32)?;
    emu.adr_no_count(28usize, 28usize, 30usize, 2134924u32);
    emu.xrr_no_count(29usize, 18usize, 30usize, 2134928u32);
    emu.anr_no_count(29usize, 11usize, 29usize, 2134932u32);
    emu.xrr_no_count(29usize, 29usize, 30usize, 2134936u32);
    emu.sri_no_count(30usize, 14usize, 2u32, 2134940u32);
    emu.sli_no_count(31usize, 14usize, 30u32, 2134944u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2134948u32);
    emu.sri_no_count(31usize, 14usize, 13u32, 2134952u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2134956u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2134960u32);
    emu.sri_no_count(9usize, 14usize, 22u32, 2134964u32);
    emu.sli_no_count(19usize, 14usize, 10u32, 2134968u32);
    emu.orr_no_count(9usize, 19usize, 9usize, 2134972u32);
    emu.xrr_no_count(19usize, 7usize, 8usize, 2134976u32);
    emu.anr_no_count(19usize, 14usize, 19usize, 2134980u32);
    emu.anr_no_count(21usize, 7usize, 8usize, 2134984u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2134988u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2134992u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2134996u32);
    emu.xrr_no_count(12usize, 30usize, 31usize, 2135000u32);
    emu.xrr_no_count(10usize, 10usize, 13usize, 2135004u32);
    emu.xrr_no_count(12usize, 12usize, 9usize, 2135008u32);
    emu.lw_no_count(13usize, 2usize, 116u32, 2135012u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2135016u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2135020u32);
    emu.adr_no_count(10usize, 12usize, 19usize, 2135024u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2135028u32);
    emu.adr_no_count(30usize, 17usize, 6usize, 2135032u32);
    emu.sri_no_count(12usize, 30usize, 6u32, 2135036u32);
    emu.sli_no_count(13usize, 30usize, 26u32, 2135040u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2135044u32);
    emu.sri_no_count(13usize, 30usize, 11u32, 2135048u32);
    emu.sli_no_count(17usize, 30usize, 21u32, 2135052u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2135056u32);
    emu.sri_no_count(17usize, 30usize, 25u32, 2135060u32);
    emu.sli_no_count(6usize, 30usize, 7u32, 2135064u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2135068u32);
    emu.lw_no_count(29usize, 2usize, 376u32, 2135072u32)?;
    emu.adr_no_count(29usize, 29usize, 18usize, 2135076u32);
    emu.xrr_no_count(6usize, 11usize, 18usize, 2135080u32);
    emu.anr_no_count(6usize, 30usize, 6usize, 2135084u32);
    emu.xrr_no_count(6usize, 6usize, 18usize, 2135088u32);
    emu.sri_no_count(31usize, 10usize, 2u32, 2135092u32);
    emu.sli_no_count(9usize, 10usize, 30u32, 2135096u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2135100u32);
    emu.sri_no_count(9usize, 10usize, 13u32, 2135104u32);
    emu.sli_no_count(18usize, 10usize, 19u32, 2135108u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2135112u32);
    emu.sri_no_count(18usize, 10usize, 22u32, 2135116u32);
    emu.sli_no_count(19usize, 10usize, 10u32, 2135120u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2135124u32);
    emu.xrr_no_count(19usize, 14usize, 7usize, 2135128u32);
    emu.anr_no_count(19usize, 10usize, 19usize, 2135132u32);
    emu.anr_no_count(21usize, 14usize, 7usize, 2135136u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2135140u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2135144u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2135148u32);
    emu.xrr_no_count(13usize, 31usize, 9usize, 2135152u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2135156u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2135160u32);
    emu.lw_no_count(17usize, 2usize, 112u32, 2135164u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2135168u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2135172u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2135176u32);
    emu.adr_no_count(6usize, 13usize, 12usize, 2135180u32);
    emu.adr_no_count(8usize, 12usize, 8usize, 2135184u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2135188u32);
    emu.sli_no_count(13usize, 8usize, 26u32, 2135192u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2135196u32);
    emu.sri_no_count(13usize, 8usize, 11u32, 2135200u32);
    emu.sli_no_count(17usize, 8usize, 21u32, 2135204u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2135208u32);
    emu.sri_no_count(17usize, 8usize, 25u32, 2135212u32);
    emu.sli_no_count(28usize, 8usize, 7u32, 2135216u32);
    emu.orr_no_count(17usize, 28usize, 17usize, 2135220u32);
    emu.lw_no_count(31usize, 2usize, 384u32, 2135224u32)?;
    emu.adr_no_count(31usize, 31usize, 11usize, 2135228u32);
    emu.xrr_no_count(28usize, 30usize, 11usize, 2135232u32);
    emu.anr_no_count(28usize, 8usize, 28usize, 2135236u32);
    emu.xrr_no_count(11usize, 28usize, 11usize, 2135240u32);
    emu.sri_no_count(28usize, 6usize, 2u32, 2135244u32);
    emu.sli_no_count(9usize, 6usize, 30u32, 2135248u32);
    emu.orr_no_count(28usize, 9usize, 28usize, 2135252u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2135256u32);
    emu.sli_no_count(18usize, 6usize, 19u32, 2135260u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2135264u32);
    emu.sri_no_count(18usize, 6usize, 22u32, 2135268u32);
    emu.sli_no_count(19usize, 6usize, 10u32, 2135272u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2135276u32);
    emu.xrr_no_count(19usize, 10usize, 14usize, 2135280u32);
    emu.anr_no_count(19usize, 6usize, 19usize, 2135284u32);
    emu.anr_no_count(21usize, 10usize, 14usize, 2135288u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2135292u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2135296u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2135300u32);
    emu.xrr_no_count(13usize, 28usize, 9usize, 2135304u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2135308u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2135312u32);
    emu.lw_no_count(17usize, 2usize, 108u32, 2135316u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2135320u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2135324u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2135328u32);
    emu.adr_no_count(28usize, 13usize, 11usize, 2135332u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2135336u32);
    emu.sri_no_count(12usize, 11usize, 6u32, 2135340u32);
    emu.sli_no_count(13usize, 11usize, 26u32, 2135344u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2135348u32);
    emu.sri_no_count(13usize, 11usize, 11u32, 2135352u32);
    emu.sli_no_count(17usize, 11usize, 21u32, 2135356u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2135360u32);
    emu.sri_no_count(17usize, 11usize, 25u32, 2135364u32);
    emu.sli_no_count(7usize, 11usize, 7u32, 2135368u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2135372u32);
    emu.lw_no_count(29usize, 2usize, 488u32, 2135376u32)?;
    emu.adr_no_count(29usize, 29usize, 30usize, 2135380u32);
    emu.xrr_no_count(7usize, 8usize, 30usize, 2135384u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2135388u32);
    emu.xrr_no_count(7usize, 7usize, 30usize, 2135392u32);
    emu.sri_no_count(30usize, 28usize, 2u32, 2135396u32);
    emu.sli_no_count(9usize, 28usize, 30u32, 2135400u32);
    emu.orr_no_count(30usize, 9usize, 30usize, 2135404u32);
    emu.sri_no_count(9usize, 28usize, 13u32, 2135408u32);
    emu.sli_no_count(18usize, 28usize, 19u32, 2135412u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2135416u32);
    emu.sri_no_count(18usize, 28usize, 22u32, 2135420u32);
    emu.sli_no_count(19usize, 28usize, 10u32, 2135424u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2135428u32);
    emu.xrr_no_count(19usize, 6usize, 10usize, 2135432u32);
    emu.anr_no_count(19usize, 28usize, 19usize, 2135436u32);
    emu.anr_no_count(21usize, 6usize, 10usize, 2135440u32);
    emu.xrr_no_count(19usize, 19usize, 21usize, 2135444u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2135448u32);
    emu.adr_no_count(7usize, 31usize, 7usize, 2135452u32);
    emu.xrr_no_count(13usize, 30usize, 9usize, 2135456u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2135460u32);
    emu.xrr_no_count(13usize, 13usize, 18usize, 2135464u32);
    emu.lw_no_count(17usize, 2usize, 104u32, 2135468u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2135472u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2135476u32);
    emu.adr_no_count(13usize, 13usize, 19usize, 2135480u32);
    emu.adr_no_count(7usize, 13usize, 12usize, 2135484u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2135488u32);
    emu.sri_no_count(12usize, 14usize, 6u32, 2135492u32);
    emu.sli_no_count(13usize, 14usize, 26u32, 2135496u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2135500u32);
    emu.sri_no_count(13usize, 14usize, 11u32, 2135504u32);
    emu.sli_no_count(17usize, 14usize, 21u32, 2135508u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2135512u32);
    emu.sri_no_count(17usize, 14usize, 25u32, 2135516u32);
    emu.sli_no_count(30usize, 14usize, 7u32, 2135520u32);
    emu.orr_no_count(17usize, 30usize, 17usize, 2135524u32);
    emu.lw_no_count(30usize, 2usize, 412u32, 2135528u32)?;
    emu.adr_no_count(30usize, 30usize, 8usize, 2135532u32);
    emu.xrr_no_count(16usize, 11usize, 8usize, 2135536u32);
    emu.anr_no_count(16usize, 14usize, 16usize, 2135540u32);
    emu.xrr_no_count(16usize, 16usize, 8usize, 2135544u32);
    emu.sri_no_count(31usize, 7usize, 2u32, 2135548u32);
    emu.sli_no_count(8usize, 7usize, 30u32, 2135552u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2135556u32);
    emu.sri_no_count(8usize, 7usize, 13u32, 2135560u32);
    emu.sli_no_count(9usize, 7usize, 19u32, 2135564u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2135568u32);
    emu.sri_no_count(9usize, 7usize, 22u32, 2135572u32);
    emu.sli_no_count(18usize, 7usize, 10u32, 2135576u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2135580u32);
    emu.xrr_no_count(18usize, 28usize, 6usize, 2135584u32);
    emu.anr_no_count(18usize, 7usize, 18usize, 2135588u32);
    emu.anr_no_count(19usize, 28usize, 6usize, 2135592u32);
    emu.xrr_no_count(18usize, 18usize, 19usize, 2135596u32);
    emu.lw_no_count(19usize, 2usize, 428u32, 2135600u32)?;
    emu.xrr_no_count(12usize, 12usize, 13usize, 2135604u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2135608u32);
    emu.xrr_no_count(13usize, 31usize, 8usize, 2135612u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2135616u32);
    emu.xrr_no_count(13usize, 13usize, 9usize, 2135620u32);
    emu.lw_no_count(17usize, 2usize, 100u32, 2135624u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2135628u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2135632u32);
    emu.adr_no_count(13usize, 13usize, 18usize, 2135636u32);
    emu.adr_no_count(16usize, 13usize, 12usize, 2135640u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2135644u32);
    emu.sri_no_count(12usize, 10usize, 6u32, 2135648u32);
    emu.sli_no_count(13usize, 10usize, 26u32, 2135652u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2135656u32);
    emu.sri_no_count(13usize, 10usize, 11u32, 2135660u32);
    emu.sli_no_count(17usize, 10usize, 21u32, 2135664u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2135668u32);
    emu.sri_no_count(17usize, 10usize, 25u32, 2135672u32);
    emu.sli_no_count(29usize, 10usize, 7u32, 2135676u32);
    emu.orr_no_count(17usize, 29usize, 17usize, 2135680u32);
    emu.lw_no_count(29usize, 2usize, 380u32, 2135684u32)?;
    emu.adr_no_count(29usize, 29usize, 11usize, 2135688u32);
    emu.xrr_no_count(5usize, 14usize, 11usize, 2135692u32);
    emu.anr_no_count(5usize, 10usize, 5usize, 2135696u32);
    emu.xrr_no_count(11usize, 5usize, 11usize, 2135700u32);
    emu.sri_no_count(5usize, 16usize, 2u32, 2135704u32);
    emu.sli_no_count(31usize, 16usize, 30u32, 2135708u32);
    emu.orr_no_count(5usize, 31usize, 5usize, 2135712u32);
    emu.sri_no_count(31usize, 16usize, 13u32, 2135716u32);
    emu.sli_no_count(8usize, 16usize, 19u32, 2135720u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2135724u32);
    emu.sri_no_count(8usize, 16usize, 22u32, 2135728u32);
    emu.sli_no_count(9usize, 16usize, 10u32, 2135732u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2135736u32);
    emu.xrr_no_count(9usize, 7usize, 28usize, 2135740u32);
    emu.anr_no_count(9usize, 16usize, 9usize, 2135744u32);
    emu.anr_no_count(18usize, 7usize, 28usize, 2135748u32);
    emu.xrr_no_count(9usize, 9usize, 18usize, 2135752u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2135756u32);
    emu.adr_no_count(11usize, 30usize, 11usize, 2135760u32);
    emu.xrr_no_count(13usize, 5usize, 31usize, 2135764u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2135768u32);
    emu.xrr_no_count(13usize, 13usize, 8usize, 2135772u32);
    emu.lw_no_count(17usize, 2usize, 96u32, 2135776u32)?;
    emu.adr_no_count(11usize, 11usize, 17usize, 2135780u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2135784u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2135788u32);
    emu.adr_no_count(5usize, 13usize, 11usize, 2135792u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2135796u32);
    emu.sri_no_count(11usize, 6usize, 6u32, 2135800u32);
    emu.sli_no_count(12usize, 6usize, 26u32, 2135804u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2135808u32);
    emu.sri_no_count(12usize, 6usize, 11u32, 2135812u32);
    emu.sli_no_count(13usize, 6usize, 21u32, 2135816u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2135820u32);
    emu.sri_no_count(13usize, 6usize, 25u32, 2135824u32);
    emu.sli_no_count(17usize, 6usize, 7u32, 2135828u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2135832u32);
    emu.adr_no_count(17usize, 27usize, 14usize, 2135836u32);
    emu.xrr_no_count(15usize, 10usize, 14usize, 2135840u32);
    emu.anr_no_count(15usize, 6usize, 15usize, 2135844u32);
    emu.xrr_no_count(14usize, 15usize, 14usize, 2135848u32);
    emu.sri_no_count(15usize, 5usize, 2u32, 2135852u32);
    emu.sli_no_count(30usize, 5usize, 30u32, 2135856u32);
    emu.orr_no_count(15usize, 30usize, 15usize, 2135860u32);
    emu.sri_no_count(30usize, 5usize, 13u32, 2135864u32);
    emu.sli_no_count(31usize, 5usize, 19u32, 2135868u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2135872u32);
    emu.sri_no_count(31usize, 5usize, 22u32, 2135876u32);
    emu.sli_no_count(8usize, 5usize, 10u32, 2135880u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2135884u32);
    emu.xrr_no_count(8usize, 16usize, 7usize, 2135888u32);
    emu.anr_no_count(8usize, 5usize, 8usize, 2135892u32);
    emu.anr_no_count(9usize, 16usize, 7usize, 2135896u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2135900u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2135904u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2135908u32);
    emu.xrr_no_count(12usize, 15usize, 30usize, 2135912u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2135916u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2135920u32);
    emu.lw_no_count(13usize, 2usize, 92u32, 2135924u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2135928u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2135932u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2135936u32);
    emu.adr_no_count(15usize, 12usize, 11usize, 2135940u32);
    emu.adr_no_count(28usize, 11usize, 28usize, 2135944u32);
    emu.sri_no_count(11usize, 28usize, 6u32, 2135948u32);
    emu.sli_no_count(12usize, 28usize, 26u32, 2135952u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2135956u32);
    emu.sri_no_count(12usize, 28usize, 11u32, 2135960u32);
    emu.sli_no_count(13usize, 28usize, 21u32, 2135964u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2135968u32);
    emu.sri_no_count(13usize, 28usize, 25u32, 2135972u32);
    emu.sli_no_count(14usize, 28usize, 7u32, 2135976u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2135980u32);
    emu.adr_no_count(29usize, 22usize, 10usize, 2135984u32);
    emu.xrr_no_count(14usize, 6usize, 10usize, 2135988u32);
    emu.anr_no_count(14usize, 28usize, 14usize, 2135992u32);
    emu.xrr_no_count(10usize, 14usize, 10usize, 2135996u32);
    emu.sri_no_count(14usize, 15usize, 2u32, 2136000u32);
    emu.sli_no_count(30usize, 15usize, 30u32, 2136004u32);
    emu.orr_no_count(14usize, 30usize, 14usize, 2136008u32);
    emu.sri_no_count(30usize, 15usize, 13u32, 2136012u32);
    emu.sli_no_count(31usize, 15usize, 19u32, 2136016u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2136020u32);
    emu.sri_no_count(31usize, 15usize, 22u32, 2136024u32);
    emu.sli_no_count(8usize, 15usize, 10u32, 2136028u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2136032u32);
    emu.xrr_no_count(8usize, 5usize, 16usize, 2136036u32);
    emu.anr_no_count(8usize, 15usize, 8usize, 2136040u32);
    emu.anr_no_count(9usize, 5usize, 16usize, 2136044u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2136048u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2136052u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2136056u32);
    emu.xrr_no_count(12usize, 14usize, 30usize, 2136060u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2136064u32);
    emu.xrr_no_count(12usize, 12usize, 31usize, 2136068u32);
    emu.lw_no_count(13usize, 2usize, 88u32, 2136072u32)?;
    emu.adr_no_count(10usize, 10usize, 13usize, 2136076u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2136080u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2136084u32);
    emu.adr_no_count(14usize, 12usize, 10usize, 2136088u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2136092u32);
    emu.sri_no_count(10usize, 7usize, 6u32, 2136096u32);
    emu.sli_no_count(11usize, 7usize, 26u32, 2136100u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2136104u32);
    emu.sri_no_count(11usize, 7usize, 11u32, 2136108u32);
    emu.sli_no_count(12usize, 7usize, 21u32, 2136112u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2136116u32);
    emu.sri_no_count(12usize, 7usize, 25u32, 2136120u32);
    emu.sli_no_count(13usize, 7usize, 7u32, 2136124u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2136128u32);
    emu.adr_no_count(20usize, 20usize, 6usize, 2136132u32);
    emu.xrr_no_count(13usize, 28usize, 6usize, 2136136u32);
    emu.anr_no_count(13usize, 7usize, 13usize, 2136140u32);
    emu.xrr_no_count(13usize, 13usize, 6usize, 2136144u32);
    emu.sri_no_count(17usize, 14usize, 2u32, 2136148u32);
    emu.sli_no_count(6usize, 14usize, 30u32, 2136152u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2136156u32);
    emu.sri_no_count(6usize, 14usize, 13u32, 2136160u32);
    emu.sli_no_count(30usize, 14usize, 19u32, 2136164u32);
    emu.orr_no_count(6usize, 30usize, 6usize, 2136168u32);
    emu.sri_no_count(30usize, 14usize, 22u32, 2136172u32);
    emu.sli_no_count(31usize, 14usize, 10u32, 2136176u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2136180u32);
    emu.xrr_no_count(31usize, 15usize, 5usize, 2136184u32);
    emu.anr_no_count(31usize, 14usize, 31usize, 2136188u32);
    emu.anr_no_count(8usize, 15usize, 5usize, 2136192u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2136196u32);
    emu.xrr_no_count(10usize, 10usize, 11usize, 2136200u32);
    emu.adr_no_count(13usize, 29usize, 13usize, 2136204u32);
    emu.xrr_no_count(11usize, 17usize, 6usize, 2136208u32);
    emu.xrr_no_count(10usize, 10usize, 12usize, 2136212u32);
    emu.xrr_no_count(11usize, 11usize, 30usize, 2136216u32);
    emu.lw_no_count(12usize, 2usize, 84u32, 2136220u32)?;
    emu.adr_no_count(13usize, 13usize, 12usize, 2136224u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2136228u32);
    emu.adr_no_count(10usize, 11usize, 31usize, 2136232u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2136236u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2136240u32);
    emu.sri_no_count(11usize, 16usize, 6u32, 2136244u32);
    emu.sli_no_count(12usize, 16usize, 26u32, 2136248u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2136252u32);
    emu.sri_no_count(12usize, 16usize, 11u32, 2136256u32);
    emu.sli_no_count(13usize, 16usize, 21u32, 2136260u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2136264u32);
    emu.sri_no_count(13usize, 16usize, 25u32, 2136268u32);
    emu.sli_no_count(17usize, 16usize, 7u32, 2136272u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2136276u32);
    emu.adr_no_count(22usize, 25usize, 28usize, 2136280u32);
    emu.xrr_no_count(17usize, 7usize, 28usize, 2136284u32);
    emu.anr_no_count(17usize, 16usize, 17usize, 2136288u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2136292u32);
    emu.sri_no_count(6usize, 10usize, 2u32, 2136296u32);
    emu.sli_no_count(28usize, 10usize, 30u32, 2136300u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2136304u32);
    emu.sri_no_count(28usize, 10usize, 13u32, 2136308u32);
    emu.sli_no_count(29usize, 10usize, 19u32, 2136312u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2136316u32);
    emu.sri_no_count(29usize, 10usize, 22u32, 2136320u32);
    emu.sli_no_count(30usize, 10usize, 10u32, 2136324u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2136328u32);
    emu.xrr_no_count(30usize, 14usize, 15usize, 2136332u32);
    emu.anr_no_count(30usize, 10usize, 30usize, 2136336u32);
    emu.anr_no_count(31usize, 14usize, 15usize, 2136340u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2136344u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2136348u32);
    emu.adr_no_count(17usize, 20usize, 17usize, 2136352u32);
    emu.lw_no_count(21usize, 2usize, 512u32, 2136356u32)?;
    emu.xrr_no_count(12usize, 6usize, 28usize, 2136360u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2136364u32);
    emu.xrr_no_count(12usize, 12usize, 29usize, 2136368u32);
    emu.lw_no_count(13usize, 2usize, 80u32, 2136372u32)?;
    emu.adr_no_count(17usize, 17usize, 13usize, 2136376u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2136380u32);
    emu.adr_no_count(11usize, 12usize, 30usize, 2136384u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2136388u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2136392u32);
    emu.sri_no_count(12usize, 5usize, 6u32, 2136396u32);
    emu.sli_no_count(13usize, 5usize, 26u32, 2136400u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2136404u32);
    emu.sri_no_count(13usize, 5usize, 11u32, 2136408u32);
    emu.sli_no_count(17usize, 5usize, 21u32, 2136412u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2136416u32);
    emu.sri_no_count(17usize, 5usize, 25u32, 2136420u32);
    emu.sli_no_count(6usize, 5usize, 7u32, 2136424u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2136428u32);
    emu.lw_no_count(6usize, 2usize, 340u32, 2136432u32)?;
    emu.adr_no_count(6usize, 6usize, 7usize, 2136436u32);
    emu.xrr_no_count(28usize, 16usize, 7usize, 2136440u32);
    emu.anr_no_count(28usize, 5usize, 28usize, 2136444u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2136448u32);
    emu.sri_no_count(28usize, 11usize, 2u32, 2136452u32);
    emu.sli_no_count(29usize, 11usize, 30u32, 2136456u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2136460u32);
    emu.sri_no_count(29usize, 11usize, 13u32, 2136464u32);
    emu.sli_no_count(30usize, 11usize, 19u32, 2136468u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2136472u32);
    emu.sri_no_count(30usize, 11usize, 22u32, 2136476u32);
    emu.sli_no_count(31usize, 11usize, 10u32, 2136480u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2136484u32);
    emu.xrr_no_count(31usize, 10usize, 14usize, 2136488u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2136492u32);
    emu.anr_no_count(8usize, 10usize, 14usize, 2136496u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2136500u32);
    emu.xrr_no_count(12usize, 12usize, 13usize, 2136504u32);
    emu.adr_no_count(7usize, 22usize, 7usize, 2136508u32);
    emu.lw_no_count(22usize, 2usize, 520u32, 2136512u32)?;
    emu.xrr_no_count(13usize, 28usize, 29usize, 2136516u32);
    emu.xrr_no_count(12usize, 12usize, 17usize, 2136520u32);
    emu.xrr_no_count(13usize, 13usize, 30usize, 2136524u32);
    emu.lw_no_count(17usize, 2usize, 76u32, 2136528u32)?;
    emu.adr_no_count(7usize, 7usize, 17usize, 2136532u32);
    emu.adr_no_count(7usize, 7usize, 12usize, 2136536u32);
    emu.adr_no_count(12usize, 13usize, 31usize, 2136540u32);
    emu.adr_no_count(12usize, 12usize, 7usize, 2136544u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2136548u32);
    emu.sri_no_count(13usize, 15usize, 6u32, 2136552u32);
    emu.sli_no_count(17usize, 15usize, 26u32, 2136556u32);
    emu.sri_no_count(7usize, 15usize, 11u32, 2136560u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2136564u32);
    emu.sli_no_count(17usize, 15usize, 21u32, 2136568u32);
    emu.orr_no_count(17usize, 17usize, 7usize, 2136572u32);
    emu.sri_no_count(7usize, 15usize, 25u32, 2136576u32);
    emu.sli_no_count(28usize, 15usize, 7u32, 2136580u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2136584u32);
    emu.lw_no_count(28usize, 2usize, 420u32, 2136588u32)?;
    emu.adr_no_count(28usize, 28usize, 16usize, 2136592u32);
    emu.xrr_no_count(29usize, 5usize, 16usize, 2136596u32);
    emu.anr_no_count(29usize, 15usize, 29usize, 2136600u32);
    emu.xrr_no_count(16usize, 29usize, 16usize, 2136604u32);
    emu.sri_no_count(29usize, 12usize, 2u32, 2136608u32);
    emu.sli_no_count(30usize, 12usize, 30u32, 2136612u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2136616u32);
    emu.sri_no_count(30usize, 12usize, 13u32, 2136620u32);
    emu.sli_no_count(31usize, 12usize, 19u32, 2136624u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2136628u32);
    emu.sri_no_count(31usize, 12usize, 22u32, 2136632u32);
    emu.sli_no_count(8usize, 12usize, 10u32, 2136636u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2136640u32);
    emu.xrr_no_count(8usize, 11usize, 10usize, 2136644u32);
    emu.anr_no_count(8usize, 12usize, 8usize, 2136648u32);
    emu.anr_no_count(9usize, 11usize, 10usize, 2136652u32);
    emu.xrr_no_count(8usize, 8usize, 9usize, 2136656u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2136660u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2136664u32);
    emu.xrr_no_count(17usize, 29usize, 30usize, 2136668u32);
    emu.xrr_no_count(13usize, 13usize, 7usize, 2136672u32);
    emu.lw_no_count(25usize, 2usize, 344u32, 2136676u32)?;
    emu.adr_no_count(25usize, 25usize, 5usize, 2136680u32);
    emu.lw_no_count(6usize, 2usize, 72u32, 2136684u32)?;
    emu.adr_no_count(16usize, 16usize, 6usize, 2136688u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2136692u32);
    emu.adr_no_count(16usize, 16usize, 13usize, 2136696u32);
    emu.adr_no_count(13usize, 17usize, 8usize, 2136700u32);
    emu.adr_no_count(13usize, 13usize, 16usize, 2136704u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2136708u32);
    emu.sri_no_count(14usize, 16usize, 6u32, 2136712u32);
    emu.sli_no_count(17usize, 16usize, 26u32, 2136716u32);
    emu.sri_no_count(6usize, 16usize, 11u32, 2136720u32);
    emu.sli_no_count(7usize, 16usize, 21u32, 2136724u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2136728u32);
    emu.sri_no_count(17usize, 16usize, 25u32, 2136732u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2136736u32);
    emu.sli_no_count(7usize, 16usize, 7u32, 2136740u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2136744u32);
    emu.xrr_no_count(7usize, 15usize, 5usize, 2136748u32);
    emu.anr_no_count(7usize, 16usize, 7usize, 2136752u32);
    emu.xrr_no_count(5usize, 7usize, 5usize, 2136756u32);
    emu.sri_no_count(7usize, 13usize, 2u32, 2136760u32);
    emu.sli_no_count(29usize, 13usize, 30u32, 2136764u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2136768u32);
    emu.sri_no_count(29usize, 13usize, 13u32, 2136772u32);
    emu.sli_no_count(30usize, 13usize, 19u32, 2136776u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2136780u32);
    emu.sri_no_count(30usize, 13usize, 22u32, 2136784u32);
    emu.sli_no_count(31usize, 13usize, 10u32, 2136788u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2136792u32);
    emu.xrr_no_count(31usize, 12usize, 11usize, 2136796u32);
    emu.anr_no_count(31usize, 13usize, 31usize, 2136800u32);
    emu.anr_no_count(8usize, 12usize, 11usize, 2136804u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2136808u32);
    emu.xrr_no_count(14usize, 14usize, 6usize, 2136812u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2136816u32);
    emu.xrr_no_count(6usize, 7usize, 29usize, 2136820u32);
    emu.lw_no_count(26usize, 2usize, 348u32, 2136824u32)?;
    emu.adr_no_count(26usize, 26usize, 15usize, 2136828u32);
    emu.xrr_no_count(14usize, 14usize, 17usize, 2136832u32);
    emu.xrr_no_count(7usize, 16usize, 15usize, 2136836u32);
    emu.lw_no_count(17usize, 2usize, 68u32, 2136840u32)?;
    emu.adr_no_count(5usize, 5usize, 17usize, 2136844u32);
    emu.xrr_no_count(17usize, 6usize, 30usize, 2136848u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2136852u32);
    emu.adr_no_count(14usize, 17usize, 31usize, 2136856u32);
    emu.adr_no_count(14usize, 14usize, 5usize, 2136860u32);
    emu.adr_no_count(17usize, 5usize, 10usize, 2136864u32);
    emu.sri_no_count(10usize, 17usize, 6u32, 2136868u32);
    emu.sli_no_count(5usize, 17usize, 26u32, 2136872u32);
    emu.sri_no_count(6usize, 17usize, 11u32, 2136876u32);
    emu.sli_no_count(28usize, 17usize, 21u32, 2136880u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2136884u32);
    emu.sri_no_count(5usize, 17usize, 25u32, 2136888u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2136892u32);
    emu.sli_no_count(28usize, 17usize, 7u32, 2136896u32);
    emu.anr_no_count(7usize, 17usize, 7usize, 2136900u32);
    emu.orr_no_count(5usize, 28usize, 5usize, 2136904u32);
    emu.sri_no_count(28usize, 14usize, 2u32, 2136908u32);
    emu.xrr_no_count(15usize, 7usize, 15usize, 2136912u32);
    emu.sli_no_count(7usize, 14usize, 30u32, 2136916u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2136920u32);
    emu.sri_no_count(28usize, 14usize, 13u32, 2136924u32);
    emu.sli_no_count(29usize, 14usize, 19u32, 2136928u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2136932u32);
    emu.sri_no_count(29usize, 14usize, 22u32, 2136936u32);
    emu.sli_no_count(30usize, 14usize, 10u32, 2136940u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2136944u32);
    emu.xrr_no_count(30usize, 13usize, 12usize, 2136948u32);
    emu.anr_no_count(30usize, 14usize, 30usize, 2136952u32);
    emu.anr_no_count(31usize, 13usize, 12usize, 2136956u32);
    emu.xrr_no_count(30usize, 30usize, 31usize, 2136960u32);
    emu.xrr_no_count(10usize, 10usize, 6usize, 2136964u32);
    emu.adr_no_count(15usize, 25usize, 15usize, 2136968u32);
    emu.lw_no_count(27usize, 2usize, 336u32, 2136972u32)?;
    emu.adr_no_count(27usize, 27usize, 16usize, 2136976u32);
    emu.xrr_no_count(6usize, 7usize, 28usize, 2136980u32);
    emu.xrr_no_count(7usize, 17usize, 16usize, 2136984u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2136988u32);
    emu.lw_no_count(5usize, 2usize, 64u32, 2136992u32)?;
    emu.adr_no_count(15usize, 15usize, 5usize, 2136996u32);
    emu.xrr_no_count(5usize, 6usize, 29usize, 2137000u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2137004u32);
    emu.adr_no_count(10usize, 5usize, 30usize, 2137008u32);
    emu.adr_no_count(10usize, 10usize, 15usize, 2137012u32);
    emu.adr_no_count(15usize, 15usize, 11usize, 2137016u32);
    emu.sri_no_count(11usize, 15usize, 6u32, 2137020u32);
    emu.sli_no_count(5usize, 15usize, 26u32, 2137024u32);
    emu.sri_no_count(6usize, 15usize, 11u32, 2137028u32);
    emu.sli_no_count(28usize, 15usize, 21u32, 2137032u32);
    emu.sri_no_count(29usize, 15usize, 25u32, 2137036u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2137040u32);
    emu.sli_no_count(5usize, 15usize, 7u32, 2137044u32);
    emu.anr_no_count(7usize, 15usize, 7usize, 2137048u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2137052u32);
    emu.sri_no_count(28usize, 10usize, 2u32, 2137056u32);
    emu.orr_no_count(29usize, 5usize, 29usize, 2137060u32);
    emu.sli_no_count(5usize, 10usize, 30u32, 2137064u32);
    emu.xrr_no_count(16usize, 7usize, 16usize, 2137068u32);
    emu.sri_no_count(7usize, 10usize, 13u32, 2137072u32);
    emu.orr_no_count(28usize, 5usize, 28usize, 2137076u32);
    emu.sli_no_count(5usize, 10usize, 19u32, 2137080u32);
    emu.orr_no_count(7usize, 5usize, 7usize, 2137084u32);
    emu.sri_no_count(5usize, 10usize, 22u32, 2137088u32);
    emu.sli_no_count(30usize, 10usize, 10u32, 2137092u32);
    emu.orr_no_count(30usize, 30usize, 5usize, 2137096u32);
    emu.xrr_no_count(5usize, 14usize, 13usize, 2137100u32);
    emu.anr_no_count(5usize, 10usize, 5usize, 2137104u32);
    emu.anr_no_count(31usize, 14usize, 13usize, 2137108u32);
    emu.xrr_no_count(31usize, 5usize, 31usize, 2137112u32);
    emu.xrr_no_count(11usize, 11usize, 6usize, 2137116u32);
    emu.lw_no_count(5usize, 2usize, 352u32, 2137120u32)?;
    emu.adr_no_count(5usize, 5usize, 17usize, 2137124u32);
    emu.adr_no_count(16usize, 26usize, 16usize, 2137128u32);
    emu.lw_no_count(26usize, 2usize, 432u32, 2137132u32)?;
    emu.xrr_no_count(6usize, 15usize, 17usize, 2137136u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2137140u32);
    emu.xrr_no_count(11usize, 11usize, 29usize, 2137144u32);
    emu.lw_no_count(28usize, 2usize, 60u32, 2137148u32)?;
    emu.adr_no_count(16usize, 16usize, 28usize, 2137152u32);
    emu.xrr_no_count(7usize, 7usize, 30usize, 2137156u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2137160u32);
    emu.adr_no_count(11usize, 7usize, 31usize, 2137164u32);
    emu.adr_no_count(11usize, 11usize, 16usize, 2137168u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2137172u32);
    emu.sri_no_count(12usize, 16usize, 6u32, 2137176u32);
    emu.sli_no_count(7usize, 16usize, 26u32, 2137180u32);
    emu.sri_no_count(28usize, 16usize, 11u32, 2137184u32);
    emu.sli_no_count(29usize, 16usize, 21u32, 2137188u32);
    emu.sri_no_count(30usize, 16usize, 25u32, 2137192u32);
    emu.sli_no_count(31usize, 16usize, 7u32, 2137196u32);
    emu.anr_no_count(6usize, 16usize, 6usize, 2137200u32);
    emu.orr_no_count(12usize, 7usize, 12usize, 2137204u32);
    emu.sri_no_count(7usize, 11usize, 2u32, 2137208u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2137212u32);
    emu.sli_no_count(29usize, 11usize, 30u32, 2137216u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2137220u32);
    emu.sri_no_count(31usize, 11usize, 13u32, 2137224u32);
    emu.xrr_no_count(17usize, 6usize, 17usize, 2137228u32);
    emu.sli_no_count(6usize, 11usize, 19u32, 2137232u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2137236u32);
    emu.sri_no_count(29usize, 11usize, 22u32, 2137240u32);
    emu.orr_no_count(6usize, 6usize, 31usize, 2137244u32);
    emu.sli_no_count(31usize, 11usize, 10u32, 2137248u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2137252u32);
    emu.xrr_no_count(31usize, 10usize, 14usize, 2137256u32);
    emu.anr_no_count(31usize, 11usize, 31usize, 2137260u32);
    emu.anr_no_count(8usize, 10usize, 14usize, 2137264u32);
    emu.xrr_no_count(31usize, 31usize, 8usize, 2137268u32);
    emu.lw_no_count(1usize, 2usize, 356u32, 2137272u32)?;
    emu.adr_no_count(1usize, 1usize, 15usize, 2137276u32);
    emu.xrr_no_count(12usize, 12usize, 28usize, 2137280u32);
    emu.anr_no_count(28usize, 11usize, 10usize, 2137284u32);
    emu.adr_no_count(17usize, 27usize, 17usize, 2137288u32);
    emu.xrr_no_count(8usize, 16usize, 15usize, 2137292u32);
    emu.xrr_no_count(6usize, 7usize, 6usize, 2137296u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2137300u32);
    emu.lw_no_count(7usize, 2usize, 56u32, 2137304u32)?;
    emu.adr_no_count(17usize, 17usize, 7usize, 2137308u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2137312u32);
    emu.adr_no_count(17usize, 17usize, 12usize, 2137316u32);
    emu.adr_no_count(12usize, 6usize, 31usize, 2137320u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2137324u32);
    emu.adr_no_count(17usize, 17usize, 13usize, 2137328u32);
    emu.sri_no_count(13usize, 17usize, 6u32, 2137332u32);
    emu.sli_no_count(6usize, 17usize, 26u32, 2137336u32);
    emu.sri_no_count(7usize, 17usize, 11u32, 2137340u32);
    emu.sli_no_count(29usize, 17usize, 21u32, 2137344u32);
    emu.sri_no_count(30usize, 17usize, 25u32, 2137348u32);
    emu.sli_no_count(31usize, 17usize, 7u32, 2137352u32);
    emu.anr_no_count(8usize, 17usize, 8usize, 2137356u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2137360u32);
    emu.sri_no_count(6usize, 12usize, 2u32, 2137364u32);
    emu.orr_no_count(7usize, 29usize, 7usize, 2137368u32);
    emu.sli_no_count(29usize, 12usize, 30u32, 2137372u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2137376u32);
    emu.sri_no_count(31usize, 12usize, 13u32, 2137380u32);
    emu.xrr_no_count(8usize, 8usize, 15usize, 2137384u32);
    emu.sli_no_count(15usize, 12usize, 19u32, 2137388u32);
    emu.orr_no_count(6usize, 29usize, 6usize, 2137392u32);
    emu.sri_no_count(29usize, 12usize, 22u32, 2137396u32);
    emu.orr_no_count(31usize, 15usize, 31usize, 2137400u32);
    emu.sli_no_count(15usize, 12usize, 10u32, 2137404u32);
    emu.orr_no_count(29usize, 15usize, 29usize, 2137408u32);
    emu.xrr_no_count(15usize, 11usize, 10usize, 2137412u32);
    emu.anr_no_count(15usize, 12usize, 15usize, 2137416u32);
    emu.xrr_no_count(28usize, 15usize, 28usize, 2137420u32);
    emu.lw_no_count(15usize, 2usize, 496u32, 2137424u32)?;
    emu.adr_no_count(15usize, 15usize, 16usize, 2137428u32);
    emu.xrr_no_count(13usize, 13usize, 7usize, 2137432u32);
    emu.xrr_no_count(7usize, 17usize, 16usize, 2137436u32);
    emu.adr_no_count(5usize, 5usize, 8usize, 2137440u32);
    emu.xrr_no_count(8usize, 12usize, 11usize, 2137444u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2137448u32);
    emu.xrr_no_count(13usize, 13usize, 30usize, 2137452u32);
    emu.lw_no_count(30usize, 2usize, 52u32, 2137456u32)?;
    emu.adr_no_count(5usize, 5usize, 30usize, 2137460u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2137464u32);
    emu.adr_no_count(5usize, 5usize, 13usize, 2137468u32);
    emu.adr_no_count(13usize, 6usize, 28usize, 2137472u32);
    emu.adr_no_count(13usize, 13usize, 5usize, 2137476u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2137480u32);
    emu.sri_no_count(14usize, 5usize, 6u32, 2137484u32);
    emu.sli_no_count(6usize, 5usize, 26u32, 2137488u32);
    emu.sri_no_count(28usize, 5usize, 11u32, 2137492u32);
    emu.sli_no_count(29usize, 5usize, 21u32, 2137496u32);
    emu.sri_no_count(30usize, 5usize, 25u32, 2137500u32);
    emu.sli_no_count(31usize, 5usize, 7u32, 2137504u32);
    emu.anr_no_count(7usize, 5usize, 7usize, 2137508u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2137512u32);
    emu.sri_no_count(6usize, 13usize, 2u32, 2137516u32);
    emu.orr_no_count(28usize, 29usize, 28usize, 2137520u32);
    emu.sli_no_count(29usize, 13usize, 30u32, 2137524u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2137528u32);
    emu.sri_no_count(31usize, 13usize, 13u32, 2137532u32);
    emu.xrr_no_count(7usize, 7usize, 16usize, 2137536u32);
    emu.sli_no_count(16usize, 13usize, 19u32, 2137540u32);
    emu.orr_no_count(6usize, 29usize, 6usize, 2137544u32);
    emu.sri_no_count(29usize, 13usize, 22u32, 2137548u32);
    emu.orr_no_count(31usize, 16usize, 31usize, 2137552u32);
    emu.sli_no_count(16usize, 13usize, 10u32, 2137556u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2137560u32);
    emu.anr_no_count(16usize, 12usize, 11usize, 2137564u32);
    emu.anr_no_count(8usize, 13usize, 8usize, 2137568u32);
    emu.xrr_no_count(8usize, 8usize, 16usize, 2137572u32);
    emu.lw_no_count(16usize, 2usize, 332u32, 2137576u32)?;
    emu.adr_no_count(16usize, 16usize, 17usize, 2137580u32);
    emu.xrr_no_count(14usize, 14usize, 28usize, 2137584u32);
    emu.anr_no_count(28usize, 13usize, 12usize, 2137588u32);
    emu.adr_no_count(7usize, 1usize, 7usize, 2137592u32);
    emu.lw_no_count(20usize, 2usize, 516u32, 2137596u32)?;
    emu.xrr_no_count(9usize, 5usize, 17usize, 2137600u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2137604u32);
    emu.xrr_no_count(14usize, 14usize, 30usize, 2137608u32);
    emu.lw_no_count(30usize, 2usize, 48u32, 2137612u32)?;
    emu.adr_no_count(7usize, 7usize, 30usize, 2137616u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2137620u32);
    emu.adr_no_count(7usize, 7usize, 14usize, 2137624u32);
    emu.adr_no_count(14usize, 6usize, 8usize, 2137628u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2137632u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2137636u32);
    emu.sri_no_count(6usize, 10usize, 6u32, 2137640u32);
    emu.sli_no_count(7usize, 10usize, 26u32, 2137644u32);
    emu.sri_no_count(29usize, 10usize, 11u32, 2137648u32);
    emu.sli_no_count(30usize, 10usize, 21u32, 2137652u32);
    emu.sri_no_count(31usize, 10usize, 25u32, 2137656u32);
    emu.sli_no_count(8usize, 10usize, 7u32, 2137660u32);
    emu.anr_no_count(9usize, 10usize, 9usize, 2137664u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2137668u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2137672u32);
    emu.orr_no_count(29usize, 30usize, 29usize, 2137676u32);
    emu.sli_no_count(30usize, 14usize, 30u32, 2137680u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2137684u32);
    emu.sri_no_count(8usize, 14usize, 13u32, 2137688u32);
    emu.xrr_no_count(17usize, 9usize, 17usize, 2137692u32);
    emu.sli_no_count(9usize, 14usize, 19u32, 2137696u32);
    emu.orr_no_count(30usize, 30usize, 7usize, 2137700u32);
    emu.sri_no_count(7usize, 14usize, 22u32, 2137704u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2137708u32);
    emu.sli_no_count(9usize, 14usize, 10u32, 2137712u32);
    emu.orr_no_count(9usize, 9usize, 7usize, 2137716u32);
    emu.xrr_no_count(7usize, 13usize, 12usize, 2137720u32);
    emu.anr_no_count(7usize, 14usize, 7usize, 2137724u32);
    emu.xrr_no_count(28usize, 7usize, 28usize, 2137728u32);
    emu.lw_no_count(7usize, 2usize, 440u32, 2137732u32)?;
    emu.adr_no_count(7usize, 7usize, 5usize, 2137736u32);
    emu.xrr_no_count(6usize, 6usize, 29usize, 2137740u32);
    emu.xrr_no_count(29usize, 10usize, 5usize, 2137744u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2137748u32);
    emu.xrr_no_count(18usize, 14usize, 13usize, 2137752u32);
    emu.xrr_no_count(17usize, 30usize, 8usize, 2137756u32);
    emu.xrr_no_count(6usize, 6usize, 31usize, 2137760u32);
    emu.lw_no_count(30usize, 2usize, 44u32, 2137764u32)?;
    emu.adr_no_count(15usize, 15usize, 30usize, 2137768u32);
    emu.xrr_no_count(17usize, 17usize, 9usize, 2137772u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2137776u32);
    emu.adr_no_count(15usize, 17usize, 28usize, 2137780u32);
    emu.adr_no_count(15usize, 15usize, 6usize, 2137784u32);
    emu.adr_no_count(17usize, 6usize, 11usize, 2137788u32);
    emu.sri_no_count(11usize, 17usize, 6u32, 2137792u32);
    emu.sli_no_count(6usize, 17usize, 26u32, 2137796u32);
    emu.sri_no_count(28usize, 17usize, 11u32, 2137800u32);
    emu.sli_no_count(30usize, 17usize, 21u32, 2137804u32);
    emu.sri_no_count(31usize, 17usize, 25u32, 2137808u32);
    emu.sli_no_count(8usize, 17usize, 7u32, 2137812u32);
    emu.anr_no_count(29usize, 17usize, 29usize, 2137816u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2137820u32);
    emu.sri_no_count(6usize, 15usize, 2u32, 2137824u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2137828u32);
    emu.sli_no_count(30usize, 15usize, 30u32, 2137832u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2137836u32);
    emu.sri_no_count(8usize, 15usize, 13u32, 2137840u32);
    emu.xrr_no_count(29usize, 29usize, 5usize, 2137844u32);
    emu.sli_no_count(5usize, 15usize, 19u32, 2137848u32);
    emu.orr_no_count(6usize, 30usize, 6usize, 2137852u32);
    emu.sri_no_count(30usize, 15usize, 22u32, 2137856u32);
    emu.orr_no_count(8usize, 5usize, 8usize, 2137860u32);
    emu.sli_no_count(5usize, 15usize, 10u32, 2137864u32);
    emu.orr_no_count(30usize, 5usize, 30usize, 2137868u32);
    emu.anr_no_count(5usize, 14usize, 13usize, 2137872u32);
    emu.anr_no_count(9usize, 15usize, 18usize, 2137876u32);
    emu.xrr_no_count(9usize, 9usize, 5usize, 2137880u32);
    emu.lw_no_count(5usize, 2usize, 364u32, 2137884u32)?;
    emu.adr_no_count(5usize, 5usize, 10usize, 2137888u32);
    emu.xrr_no_count(11usize, 11usize, 28usize, 2137892u32);
    emu.anr_no_count(28usize, 15usize, 14usize, 2137896u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2137900u32);
    emu.xrr_no_count(29usize, 17usize, 10usize, 2137904u32);
    emu.xrr_no_count(6usize, 6usize, 8usize, 2137908u32);
    emu.xrr_no_count(11usize, 11usize, 31usize, 2137912u32);
    emu.lw_no_count(31usize, 2usize, 40u32, 2137916u32)?;
    emu.adr_no_count(16usize, 16usize, 31usize, 2137920u32);
    emu.xrr_no_count(6usize, 6usize, 30usize, 2137924u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2137928u32);
    emu.adr_no_count(16usize, 6usize, 9usize, 2137932u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2137936u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2137940u32);
    emu.sri_no_count(11usize, 12usize, 6u32, 2137944u32);
    emu.sli_no_count(6usize, 12usize, 26u32, 2137948u32);
    emu.sri_no_count(30usize, 12usize, 11u32, 2137952u32);
    emu.sli_no_count(31usize, 12usize, 21u32, 2137956u32);
    emu.sri_no_count(8usize, 12usize, 25u32, 2137960u32);
    emu.sli_no_count(9usize, 12usize, 7u32, 2137964u32);
    emu.anr_no_count(29usize, 12usize, 29usize, 2137968u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2137972u32);
    emu.sri_no_count(6usize, 16usize, 2u32, 2137976u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2137980u32);
    emu.sli_no_count(31usize, 16usize, 30u32, 2137984u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2137988u32);
    emu.sri_no_count(9usize, 16usize, 13u32, 2137992u32);
    emu.xrr_no_count(10usize, 29usize, 10usize, 2137996u32);
    emu.sli_no_count(29usize, 16usize, 19u32, 2138000u32);
    emu.orr_no_count(31usize, 31usize, 6usize, 2138004u32);
    emu.sri_no_count(6usize, 16usize, 22u32, 2138008u32);
    emu.orr_no_count(29usize, 29usize, 9usize, 2138012u32);
    emu.sli_no_count(9usize, 16usize, 10u32, 2138016u32);
    emu.orr_no_count(9usize, 9usize, 6usize, 2138020u32);
    emu.xrr_no_count(6usize, 15usize, 14usize, 2138024u32);
    emu.anr_no_count(6usize, 16usize, 6usize, 2138028u32);
    emu.xrr_no_count(28usize, 6usize, 28usize, 2138032u32);
    emu.lw_no_count(6usize, 2usize, 492u32, 2138036u32)?;
    emu.adr_no_count(6usize, 6usize, 17usize, 2138040u32);
    emu.xrr_no_count(11usize, 11usize, 30usize, 2138044u32);
    emu.xrr_no_count(30usize, 12usize, 17usize, 2138048u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2138052u32);
    emu.xrr_no_count(7usize, 16usize, 15usize, 2138056u32);
    emu.xrr_no_count(29usize, 31usize, 29usize, 2138060u32);
    emu.xrr_no_count(11usize, 11usize, 8usize, 2138064u32);
    emu.lw_no_count(31usize, 2usize, 36u32, 2138068u32)?;
    emu.adr_no_count(10usize, 10usize, 31usize, 2138072u32);
    emu.xrr_no_count(29usize, 29usize, 9usize, 2138076u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2138080u32);
    emu.adr_no_count(11usize, 29usize, 28usize, 2138084u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2138088u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2138092u32);
    emu.sri_no_count(10usize, 13usize, 6u32, 2138096u32);
    emu.sli_no_count(28usize, 13usize, 26u32, 2138100u32);
    emu.sri_no_count(29usize, 13usize, 11u32, 2138104u32);
    emu.sli_no_count(31usize, 13usize, 21u32, 2138108u32);
    emu.sri_no_count(8usize, 13usize, 25u32, 2138112u32);
    emu.sli_no_count(9usize, 13usize, 7u32, 2138116u32);
    emu.anr_no_count(30usize, 13usize, 30usize, 2138120u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2138124u32);
    emu.sri_no_count(28usize, 11usize, 2u32, 2138128u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2138132u32);
    emu.sli_no_count(31usize, 11usize, 30u32, 2138136u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2138140u32);
    emu.sri_no_count(9usize, 11usize, 13u32, 2138144u32);
    emu.xrr_no_count(17usize, 30usize, 17usize, 2138148u32);
    emu.sli_no_count(30usize, 11usize, 19u32, 2138152u32);
    emu.orr_no_count(28usize, 31usize, 28usize, 2138156u32);
    emu.sri_no_count(31usize, 11usize, 22u32, 2138160u32);
    emu.orr_no_count(30usize, 30usize, 9usize, 2138164u32);
    emu.sli_no_count(9usize, 11usize, 10u32, 2138168u32);
    emu.orr_no_count(31usize, 9usize, 31usize, 2138172u32);
    emu.anr_no_count(9usize, 16usize, 15usize, 2138176u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2138180u32);
    emu.xrr_no_count(9usize, 7usize, 9usize, 2138184u32);
    emu.lw_no_count(7usize, 2usize, 408u32, 2138188u32)?;
    emu.adr_no_count(7usize, 7usize, 12usize, 2138192u32);
    emu.xrr_no_count(10usize, 10usize, 29usize, 2138196u32);
    emu.anr_no_count(29usize, 11usize, 16usize, 2138200u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2138204u32);
    emu.xrr_no_count(18usize, 13usize, 12usize, 2138208u32);
    emu.xrr_no_count(5usize, 28usize, 30usize, 2138212u32);
    emu.xrr_no_count(10usize, 10usize, 8usize, 2138216u32);
    emu.lw_no_count(28usize, 2usize, 32u32, 2138220u32)?;
    emu.adr_no_count(17usize, 17usize, 28usize, 2138224u32);
    emu.xrr_no_count(5usize, 5usize, 31usize, 2138228u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2138232u32);
    emu.adr_no_count(10usize, 5usize, 9usize, 2138236u32);
    emu.adr_no_count(10usize, 10usize, 17usize, 2138240u32);
    emu.adr_no_count(5usize, 17usize, 14usize, 2138244u32);
    emu.sri_no_count(14usize, 5usize, 6u32, 2138248u32);
    emu.sli_no_count(17usize, 5usize, 26u32, 2138252u32);
    emu.sri_no_count(28usize, 5usize, 11u32, 2138256u32);
    emu.sli_no_count(30usize, 5usize, 21u32, 2138260u32);
    emu.sri_no_count(31usize, 5usize, 25u32, 2138264u32);
    emu.sli_no_count(8usize, 5usize, 7u32, 2138268u32);
    emu.anr_no_count(9usize, 5usize, 18usize, 2138272u32);
    emu.orr_no_count(17usize, 17usize, 14usize, 2138276u32);
    emu.sri_no_count(14usize, 10usize, 2u32, 2138280u32);
    emu.orr_no_count(28usize, 30usize, 28usize, 2138284u32);
    emu.sli_no_count(30usize, 10usize, 30u32, 2138288u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2138292u32);
    emu.sri_no_count(8usize, 10usize, 13u32, 2138296u32);
    emu.xrr_no_count(12usize, 9usize, 12usize, 2138300u32);
    emu.sli_no_count(9usize, 10usize, 19u32, 2138304u32);
    emu.orr_no_count(30usize, 30usize, 14usize, 2138308u32);
    emu.sri_no_count(14usize, 10usize, 22u32, 2138312u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2138316u32);
    emu.sli_no_count(9usize, 10usize, 10u32, 2138320u32);
    emu.orr_no_count(9usize, 9usize, 14usize, 2138324u32);
    emu.xrr_no_count(14usize, 11usize, 16usize, 2138328u32);
    emu.anr_no_count(14usize, 10usize, 14usize, 2138332u32);
    emu.xrr_no_count(29usize, 14usize, 29usize, 2138336u32);
    emu.lw_no_count(14usize, 2usize, 504u32, 2138340u32)?;
    emu.adr_no_count(14usize, 14usize, 13usize, 2138344u32);
    emu.xrr_no_count(17usize, 17usize, 28usize, 2138348u32);
    emu.xrr_no_count(28usize, 5usize, 13usize, 2138352u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2138356u32);
    emu.xrr_no_count(6usize, 10usize, 11usize, 2138360u32);
    emu.xrr_no_count(30usize, 30usize, 8usize, 2138364u32);
    emu.xrr_no_count(17usize, 17usize, 31usize, 2138368u32);
    emu.lw_no_count(31usize, 2usize, 28u32, 2138372u32)?;
    emu.adr_no_count(12usize, 12usize, 31usize, 2138376u32);
    emu.xrr_no_count(30usize, 30usize, 9usize, 2138380u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2138384u32);
    emu.adr_no_count(17usize, 30usize, 29usize, 2138388u32);
    emu.adr_no_count(17usize, 17usize, 12usize, 2138392u32);
    emu.adr_no_count(15usize, 12usize, 15usize, 2138396u32);
    emu.sri_no_count(12usize, 15usize, 6u32, 2138400u32);
    emu.sli_no_count(29usize, 15usize, 26u32, 2138404u32);
    emu.sri_no_count(30usize, 15usize, 11u32, 2138408u32);
    emu.sli_no_count(31usize, 15usize, 21u32, 2138412u32);
    emu.sri_no_count(8usize, 15usize, 25u32, 2138416u32);
    emu.sli_no_count(9usize, 15usize, 7u32, 2138420u32);
    emu.anr_no_count(28usize, 15usize, 28usize, 2138424u32);
    emu.orr_no_count(12usize, 29usize, 12usize, 2138428u32);
    emu.sri_no_count(29usize, 17usize, 2u32, 2138432u32);
    emu.orr_no_count(30usize, 31usize, 30usize, 2138436u32);
    emu.sli_no_count(31usize, 17usize, 30u32, 2138440u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2138444u32);
    emu.sri_no_count(9usize, 17usize, 13u32, 2138448u32);
    emu.xrr_no_count(28usize, 28usize, 13usize, 2138452u32);
    emu.sli_no_count(13usize, 17usize, 19u32, 2138456u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2138460u32);
    emu.sri_no_count(31usize, 17usize, 22u32, 2138464u32);
    emu.orr_no_count(9usize, 13usize, 9usize, 2138468u32);
    emu.sli_no_count(13usize, 17usize, 10u32, 2138472u32);
    emu.orr_no_count(31usize, 13usize, 31usize, 2138476u32);
    emu.anr_no_count(13usize, 10usize, 11usize, 2138480u32);
    emu.anr_no_count(6usize, 17usize, 6usize, 2138484u32);
    emu.xrr_no_count(6usize, 6usize, 13usize, 2138488u32);
    emu.lw_no_count(13usize, 2usize, 444u32, 2138492u32)?;
    emu.adr_no_count(13usize, 13usize, 5usize, 2138496u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2138500u32);
    emu.anr_no_count(30usize, 17usize, 10usize, 2138504u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2138508u32);
    emu.xrr_no_count(28usize, 15usize, 5usize, 2138512u32);
    emu.xrr_no_count(29usize, 29usize, 9usize, 2138516u32);
    emu.xrr_no_count(12usize, 12usize, 8usize, 2138520u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2138524u32)?;
    emu.adr_no_count(7usize, 7usize, 8usize, 2138528u32);
    emu.xrr_no_count(29usize, 29usize, 31usize, 2138532u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2138536u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2138540u32);
    emu.adr_no_count(6usize, 6usize, 12usize, 2138544u32);
    emu.adr_no_count(16usize, 12usize, 16usize, 2138548u32);
    emu.sri_no_count(12usize, 16usize, 6u32, 2138552u32);
    emu.sli_no_count(7usize, 16usize, 26u32, 2138556u32);
    emu.sri_no_count(29usize, 16usize, 11u32, 2138560u32);
    emu.sli_no_count(31usize, 16usize, 21u32, 2138564u32);
    emu.sri_no_count(8usize, 16usize, 25u32, 2138568u32);
    emu.sli_no_count(9usize, 16usize, 7u32, 2138572u32);
    emu.anr_no_count(28usize, 16usize, 28usize, 2138576u32);
    emu.orr_no_count(7usize, 7usize, 12usize, 2138580u32);
    emu.sri_no_count(12usize, 6usize, 2u32, 2138584u32);
    emu.orr_no_count(29usize, 31usize, 29usize, 2138588u32);
    emu.sli_no_count(31usize, 6usize, 30u32, 2138592u32);
    emu.orr_no_count(8usize, 9usize, 8usize, 2138596u32);
    emu.sri_no_count(9usize, 6usize, 13u32, 2138600u32);
    emu.xrr_no_count(28usize, 28usize, 5usize, 2138604u32);
    emu.sli_no_count(5usize, 6usize, 19u32, 2138608u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2138612u32);
    emu.sri_no_count(12usize, 6usize, 22u32, 2138616u32);
    emu.orr_no_count(9usize, 5usize, 9usize, 2138620u32);
    emu.sli_no_count(5usize, 6usize, 10u32, 2138624u32);
    emu.orr_no_count(18usize, 5usize, 12usize, 2138628u32);
    emu.xrr_no_count(12usize, 17usize, 10usize, 2138632u32);
    emu.anr_no_count(12usize, 6usize, 12usize, 2138636u32);
    emu.xrr_no_count(30usize, 12usize, 30usize, 2138640u32);
    emu.lw_no_count(12usize, 2usize, 360u32, 2138644u32)?;
    emu.adr_no_count(12usize, 12usize, 15usize, 2138648u32);
    emu.lw_no_count(5usize, 2usize, 424u32, 2138652u32)?;
    emu.adr_no_count(5usize, 17usize, 5usize, 2138656u32);
    emu.xrr_no_count(7usize, 7usize, 29usize, 2138660u32);
    emu.xrr_no_count(29usize, 16usize, 15usize, 2138664u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2138668u32);
    emu.xrr_no_count(28usize, 6usize, 17usize, 2138672u32);
    emu.anr_no_count(17usize, 6usize, 17usize, 2138676u32);
    emu.adr_no_count(19usize, 6usize, 19usize, 2138680u32);
    emu.xrr_no_count(31usize, 31usize, 9usize, 2138684u32);
    emu.xrr_no_count(7usize, 7usize, 8usize, 2138688u32);
    emu.lw_no_count(8usize, 2usize, 20u32, 2138692u32)?;
    emu.adr_no_count(14usize, 14usize, 8usize, 2138696u32);
    emu.xrr_no_count(31usize, 31usize, 18usize, 2138700u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2138704u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2138708u32);
    emu.adr_no_count(30usize, 30usize, 14usize, 2138712u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2138716u32);
    emu.sri_no_count(14usize, 11usize, 6u32, 2138720u32);
    emu.sli_no_count(7usize, 11usize, 26u32, 2138724u32);
    emu.sri_no_count(31usize, 11usize, 11u32, 2138728u32);
    emu.sli_no_count(8usize, 11usize, 21u32, 2138732u32);
    emu.sri_no_count(9usize, 11usize, 25u32, 2138736u32);
    emu.sli_no_count(18usize, 11usize, 7u32, 2138740u32);
    emu.anr_no_count(29usize, 11usize, 29usize, 2138744u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2138748u32);
    emu.sri_no_count(7usize, 30usize, 2u32, 2138752u32);
    emu.orr_no_count(31usize, 8usize, 31usize, 2138756u32);
    emu.sli_no_count(8usize, 30usize, 30u32, 2138760u32);
    emu.orr_no_count(9usize, 18usize, 9usize, 2138764u32);
    emu.sri_no_count(18usize, 30usize, 13u32, 2138768u32);
    emu.xrr_no_count(15usize, 29usize, 15usize, 2138772u32);
    emu.sli_no_count(29usize, 30usize, 19u32, 2138776u32);
    emu.orr_no_count(7usize, 8usize, 7usize, 2138780u32);
    emu.sri_no_count(8usize, 30usize, 22u32, 2138784u32);
    emu.orr_no_count(29usize, 29usize, 18usize, 2138788u32);
    emu.anr_no_count(18usize, 30usize, 6usize, 2138792u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2138796u32);
    emu.xrr_no_count(6usize, 30usize, 6usize, 2138800u32);
    emu.adr_no_count(21usize, 30usize, 21usize, 2138804u32);
    emu.sli_no_count(30usize, 30usize, 10u32, 2138808u32);
    emu.orr_no_count(30usize, 30usize, 8usize, 2138812u32);
    emu.xrr_no_count(17usize, 28usize, 17usize, 2138816u32);
    emu.xrr_no_count(14usize, 14usize, 31usize, 2138820u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2138824u32);
    emu.xrr_no_count(15usize, 7usize, 29usize, 2138828u32);
    emu.adr_no_count(26usize, 11usize, 26usize, 2138832u32);
    emu.xrr_no_count(11usize, 11usize, 16usize, 2138836u32);
    emu.xrr_no_count(14usize, 14usize, 9usize, 2138840u32);
    emu.lw_no_count(9usize, 2usize, 372u32, 2138844u32)?;
    emu.lw_no_count(7usize, 2usize, 16u32, 2138848u32)?;
    emu.adr_no_count(13usize, 13usize, 7usize, 2138852u32);
    emu.xrr_no_count(15usize, 15usize, 30usize, 2138856u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2138860u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2138864u32);
    emu.adr_no_count(15usize, 15usize, 13usize, 2138868u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2138872u32);
    emu.sri_no_count(13usize, 10usize, 6u32, 2138876u32);
    emu.sli_no_count(14usize, 10usize, 26u32, 2138880u32);
    emu.sri_no_count(17usize, 10usize, 11u32, 2138884u32);
    emu.sli_no_count(7usize, 10usize, 21u32, 2138888u32);
    emu.sri_no_count(28usize, 10usize, 25u32, 2138892u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2138896u32);
    emu.adr_no_count(20usize, 10usize, 20usize, 2138900u32);
    emu.sli_no_count(10usize, 10usize, 7u32, 2138904u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2138908u32);
    emu.sri_no_count(14usize, 15usize, 2u32, 2138912u32);
    emu.orr_no_count(17usize, 7usize, 17usize, 2138916u32);
    emu.sli_no_count(7usize, 15usize, 30u32, 2138920u32);
    emu.orr_no_count(10usize, 10usize, 28usize, 2138924u32);
    emu.sri_no_count(28usize, 15usize, 13u32, 2138928u32);
    emu.orr_no_count(14usize, 7usize, 14usize, 2138932u32);
    emu.sli_no_count(7usize, 15usize, 19u32, 2138936u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2138940u32);
    emu.sri_no_count(28usize, 15usize, 22u32, 2138944u32);
    emu.anr_no_count(6usize, 15usize, 6usize, 2138948u32);
    emu.adr_no_count(22usize, 15usize, 22usize, 2138952u32);
    emu.sli_no_count(15usize, 15usize, 10u32, 2138956u32);
    emu.orr_no_count(15usize, 15usize, 28usize, 2138960u32);
    emu.xrr_no_count(6usize, 6usize, 18usize, 2138964u32);
    emu.xrr_no_count(13usize, 13usize, 17usize, 2138968u32);
    emu.xrr_no_count(11usize, 11usize, 16usize, 2138972u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2138976u32);
    emu.xrr_no_count(12usize, 14usize, 7usize, 2138980u32);
    emu.lw_no_count(14usize, 2usize, 508u32, 2138984u32)?;
    emu.adr_no_count(6usize, 6usize, 14usize, 2138988u32);
    emu.xrr_no_count(10usize, 13usize, 10usize, 2138992u32);
    emu.xrr_no_count(12usize, 12usize, 15usize, 2138996u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2139000u32)?;
    emu.adr_no_count(11usize, 11usize, 13usize, 2139004u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2139008u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2139012u32);
    emu.adr_no_count(23usize, 12usize, 10usize, 2139016u32);
    emu.adr_no_count(30usize, 5usize, 10usize, 2139020u32);
    emu.lw_no_count(10usize, 2usize, 368u32, 2139024u32)?;
    emu.adi_no_count(12usize, 10usize, 64u32, 2139028u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2139032u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2139036u32);
    emu.lw_no_count(10usize, 2usize, 268u32, 2139040u32)?;
    emu.add_memory_rw_events(4002usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2139048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3a8));
    } else {
        emu.pc = 2139044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3a4));
    }
}
#[inline(always)]
pub fn block_0x0020a3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2139048u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2123032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206518));
}
#[inline]
pub fn block_0x0020a3a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2139052u32)?;
    emu.sw_no_count(23usize, 10usize, 0u32, 2139056u32)?;
    emu.sw_no_count(22usize, 10usize, 4u32, 2139060u32)?;
    emu.sw_no_count(21usize, 10usize, 8u32, 2139064u32)?;
    emu.sw_no_count(19usize, 10usize, 12u32, 2139068u32)?;
    emu.sw_no_count(30usize, 10usize, 16u32, 2139072u32)?;
    emu.sw_no_count(20usize, 10usize, 20u32, 2139076u32)?;
    emu.sw_no_count(26usize, 10usize, 24u32, 2139080u32)?;
    emu.sw_no_count(9usize, 10usize, 28u32, 2139084u32)?;
    emu.lw_no_count(1usize, 2usize, 572u32, 2139088u32)?;
    emu.lw_no_count(8usize, 2usize, 568u32, 2139092u32)?;
    emu.lw_no_count(9usize, 2usize, 564u32, 2139096u32)?;
    emu.lw_no_count(18usize, 2usize, 560u32, 2139100u32)?;
    emu.lw_no_count(19usize, 2usize, 556u32, 2139104u32)?;
    emu.lw_no_count(20usize, 2usize, 552u32, 2139108u32)?;
    emu.lw_no_count(21usize, 2usize, 548u32, 2139112u32)?;
    emu.lw_no_count(22usize, 2usize, 544u32, 2139116u32)?;
    emu.lw_no_count(23usize, 2usize, 540u32, 2139120u32)?;
    emu.lw_no_count(24usize, 2usize, 536u32, 2139124u32)?;
    emu.lw_no_count(25usize, 2usize, 532u32, 2139128u32)?;
    emu.lw_no_count(26usize, 2usize, 528u32, 2139132u32)?;
    emu.lw_no_count(27usize, 2usize, 524u32, 2139136u32)?;
    emu.adi_no_count(2usize, 2usize, 576u32, 2139140u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139144u32;
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
pub fn block_0x0020a408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2139148u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2139152u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2139156u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2139160u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139164u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2139168u32);
    emu.apc_no_count(6usize, 2139168u32, 40960u32, 2139172u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139176u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2139180u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139184u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 896u32, 2139188u32);
    emu.apc_no_count(6usize, 2139188u32, 36864u32, 2139192u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139196u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2139200u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2139204u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2139208u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2139212u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139216u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 824u32, 2139220u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2139224u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2139228u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2139232u32;
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
pub fn block_0x0020a460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2139236u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2139240u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139244u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965512u32, 2139248u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139252u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 940u32, 2139256u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2139260u32);
    emu.adi_no_count(16usize, 2usize, 32u32, 2139264u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2139268u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2139272u32)?;
    emu.sw_no_count(13usize, 2usize, 36u32, 2139276u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2139280u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2139284u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2139288u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2139292u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2139296u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2139300u32)?;
    emu.sb_no_count(12usize, 2usize, 0u32, 2139304u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2139308u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2139312u32);
    emu.apc_no_count(1usize, 2139312u32, 36864u32, 2139316u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139320u32;
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
#[inline(always)]
pub fn block_0x0020a4b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2139324u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139328u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139332u32;
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
pub fn block_0x0020a4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139336u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1123u32, 2139340u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2139344u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2139348u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139352u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139356u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139360u32);
    emu.apc_no_count(6usize, 2139360u32, 40960u32, 2139364u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139368u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139372u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1095u32, 2139376u32);
    emu.adi_no_count(12usize, 0usize, 15u32, 2139380u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2139384u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139388u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139392u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139396u32);
    emu.apc_no_count(6usize, 2139396u32, 40960u32, 2139400u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139404u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(76u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a50c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139408u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1080u32, 2139412u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2139416u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2139420u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139424u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139428u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139432u32);
    emu.apc_no_count(6usize, 2139432u32, 40960u32, 2139436u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139440u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(40u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2139444u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2139448u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139452u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966768u32, 2139456u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139460u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1012u32, 2139464u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2139468u32);
    emu.adi_no_count(16usize, 2usize, 32u32, 2139472u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2139476u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2139480u32)?;
    emu.sw_no_count(13usize, 2usize, 36u32, 2139484u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2139488u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2139492u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2139496u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2139500u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2139504u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2139508u32)?;
    emu.sw_no_count(12usize, 2usize, 0u32, 2139512u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2139516u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2139520u32);
    emu.apc_no_count(1usize, 2139520u32, 36864u32, 2139524u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2139532u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139536u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139540u32;
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
pub fn block_0x0020a594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139544u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966836u32, 2139548u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2139552u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2139556u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139560u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139564u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139568u32);
    emu.apc_no_count(6usize, 2139568u32, 40960u32, 2139572u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139576u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a5b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139580u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1066u32, 2139584u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2139588u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2139592u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139596u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139600u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139604u32);
    emu.apc_no_count(6usize, 2139604u32, 40960u32, 2139608u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139612u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2139616u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2139620u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2139624u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139628u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1652u32, 2139632u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2139636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a690));
}
#[inline]
pub fn block_0x0020a5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139640u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1110u32, 2139644u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2139648u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2139652u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139656u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139660u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139664u32);
    emu.apc_no_count(6usize, 2139664u32, 40960u32, 2139668u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139672u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2139676u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2139680u32)?;
    emu.adi_no_count(13usize, 2usize, 32u32, 2139684u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139688u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965284u32, 2139692u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2139696u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 984u32, 2139700u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2139704u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2139708u32)?;
    emu.adi_no_count(17usize, 2usize, 0u32, 2139712u32);
    emu.sw_no_count(13usize, 2usize, 0u32, 2139716u32)?;
    emu.sw_no_count(14usize, 2usize, 4u32, 2139720u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2139724u32);
    emu.sw_no_count(12usize, 2usize, 32u32, 2139728u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2139732u32)?;
    emu.add_memory_rw_events(16usize);
    let return_addr = 2139736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6b8));
}
#[inline]
pub fn block_0x0020a658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139740u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1044u32, 2139744u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2139748u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2139752u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139756u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139760u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139764u32);
    emu.apc_no_count(6usize, 2139764u32, 40960u32, 2139768u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139772u32;
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
#[inline(always)]
pub fn block_0x0020a67c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2139776u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2139780u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2139784u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139788u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1532u32, 2139792u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2139792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a690));
}
#[inline]
pub fn block_0x0020a690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2139796u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 968u32, 2139800u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2139804u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2139808u32)?;
    emu.adi_no_count(17usize, 2usize, 32u32, 2139812u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2139816u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2139820u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2139824u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2139828u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2139832u32)?;
    emu.add_memory_rw_events(10usize);
    emu.pc = 2139832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6b8));
}
#[inline]
pub fn block_0x0020a6b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2139836u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2139840u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2139844u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2139848u32)?;
    emu.sw_no_count(17usize, 2usize, 16u32, 2139852u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2139856u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2139860u32);
    emu.apc_no_count(1usize, 2139860u32, 36864u32, 2139864u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a6dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2139872u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139876u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139880u32;
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
pub fn block_0x0020a6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2139884u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2139888u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2139892u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139896u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2139900u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2139904u32);
    emu.apc_no_count(6usize, 2139904u32, 40960u32, 2139908u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139912u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2139916u32)?;
    emu.lw_no_count(10usize, 10usize, 8u32, 2139920u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2139924u32);
    let a = 0u32.wrapping_add(2138112u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139928u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1032u32, 2139932u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2139936u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1036u32, 2139940u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2139944u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2139948u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2139952u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2139956u32)?;
    emu.adi_no_count(13usize, 2usize, 32u32, 2139960u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2139964u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2139968u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2139972u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2139976u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2139980u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2139984u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2139988u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2139992u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2139996u32);
    emu.apc_no_count(1usize, 2139996u32, 36864u32, 2140000u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140004u32;
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
pub fn block_0x0020a764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2140008u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140012u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140016u32;
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
pub fn block_0x0020a770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140020u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1083u32, 2140024u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2140028u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2140032u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140036u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2140040u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140044u32);
    emu.apc_no_count(6usize, 2140044u32, 40960u32, 2140048u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140052u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140056u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 920u32, 2140060u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2140064u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2140068u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140072u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2140076u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140080u32);
    emu.apc_no_count(6usize, 2140080u32, 40960u32, 2140084u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140088u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a7b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140092u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966004u32, 2140096u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2140100u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2140104u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140108u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2140112u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140116u32);
    emu.apc_no_count(6usize, 2140116u32, 40960u32, 2140120u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140124u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a7dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140128u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1054u32, 2140132u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2140136u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2140140u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140144u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2140148u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140152u32);
    emu.apc_no_count(6usize, 2140152u32, 40960u32, 2140156u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140160u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2140164u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2140168u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2140172u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140176u32);
    emu.apc_no_count(6usize, 2140176u32, 40960u32, 2140180u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140184u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 12u32, 2140188u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2140192u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140196u32;
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
pub fn block_0x0020a824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2140200u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2140204u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2140208u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2140212u32)?;
    emu.sli_no_count(12usize, 12usize, 1u32, 2140216u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2140220u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140224u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2140316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a89c));
    } else {
        emu.pc = 2140228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a844));
    }
}
#[inline]
pub fn block_0x0020a844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140232u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966504u32, 2140236u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140240u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966028u32, 2140244u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2140248u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2140252u32)?;
    emu.adi_no_count(15usize, 2usize, 32u32, 2140256u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2140260u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2140264u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2140268u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2140272u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2140276u32)?;
    emu.sw_no_count(14usize, 2usize, 12u32, 2140280u32)?;
    emu.sw_no_count(15usize, 2usize, 16u32, 2140284u32)?;
    emu.sw_no_count(14usize, 2usize, 20u32, 2140288u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2140292u32);
    emu.apc_no_count(1usize, 2140292u32, 36864u32, 2140296u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a88c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2140304u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2140308u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140312u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140316u32;
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
pub fn block_0x0020a89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2140320u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2140324u32)?;
    emu.sb_no_count(0usize, 2usize, 4u32, 2140328u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140332u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966504u32, 2140336u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140340u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966028u32, 2140344u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2140348u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2140352u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2140356u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2140360u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2140364u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2140368u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2140372u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2140376u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2140380u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140384u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 896u32, 2140388u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2140392u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2140396u32);
    emu.apc_no_count(1usize, 2140396u32, 36864u32, 2140400u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a90c));
    } else {
        emu.pc = 2140408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8f8));
    }
}
#[inline(always)]
pub fn block_0x0020a8f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2140412u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2140416u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2140420u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140424u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140428u32;
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
pub fn block_0x0020a90c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 4u32, 2140432u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2140464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a930));
    } else {
        emu.pc = 2140436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a914));
    }
}
#[inline(always)]
pub fn block_0x0020a914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140440u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1137u32, 2140444u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2140448u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2140452u32);
    emu.apc_no_count(1usize, 2140452u32, 40960u32, 2140456u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8f8));
    } else {
        emu.pc = 2140464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a930));
    }
}
#[inline(always)]
pub fn block_0x0020a930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140468u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2140472u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2140476u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2140480u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140484u32;
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
pub fn block_0x0020a944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140488u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140492u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140496u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2140500u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2140504u32)?;
    emu.adi_no_count(13usize, 0usize, 7u32, 2140508u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2140512u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2140568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a998));
    } else {
        emu.pc = 2140516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a964));
    }
}
#[inline(always)]
pub fn block_0x0020a964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9c0));
    } else {
        emu.pc = 2140520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a968));
    }
}
#[inline(always)]
pub fn block_0x0020a968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4294967295u32, 2140524u32);
    emu.adi_no_count(10usize, 0usize, 46u32, 2140528u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2140532u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2140532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a974));
}
#[inline(always)]
pub fn block_0x0020a974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 14usize, 0u32, 2140536u32);
    emu.adi_no_count(13usize, 17usize, 4294967250u32, 2140540u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2140544u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2140612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9c4));
    } else {
        emu.pc = 2140548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a984));
    }
}
#[inline(always)]
pub fn block_0x0020a984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 15usize, 0u32, 2140552u32);
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2140556u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2140560u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2140532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a974));
    } else {
        emu.pc = 2140564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a994));
    }
}
#[inline(always)]
pub fn block_0x0020a994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2140568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9c4));
}
#[inline(always)]
pub fn block_0x0020a998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 46u32, 2140572u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2140576u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2140580u32);
    emu.apc_no_count(1usize, 2140580u32, 28672u32, 2140584u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2140592u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2140596u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2140600u32);
    emu.sltiu_no_count(13usize, 10usize, 1u32, 2140604u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2140608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9c4));
}
#[inline(always)]
pub fn block_0x0020a9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2140612u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9c4));
}
#[inline]
pub fn block_0x0020a9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 8usize, 4u32, 2140616u32);
    emu.lw_no_count(10usize, 8usize, 0u32, 2140620u32)?;
    emu.orr_no_count(13usize, 13usize, 14usize, 2140624u32);
    emu.sb_no_count(13usize, 8usize, 4u32, 2140628u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2140632u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140636u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2140640u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2140644u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140648u32);
    emu.apc_no_count(6usize, 2140648u32, 40960u32, 2140652u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140656u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020a9f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 4u32, 2140660u32);
    emu.lw_no_count(12usize, 10usize, 0u32, 2140664u32)?;
    emu.adi_no_count(14usize, 11usize, 4294967250u32, 2140668u32);
    emu.sltiu_no_count(14usize, 14usize, 1u32, 2140672u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2140676u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2140680u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2140684u32);
    emu.apc_no_count(6usize, 2140684u32, 40960u32, 2140688u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140692u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aa14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140696u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140700u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140704u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2140708u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2140712u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140716u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2140828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa9c));
    } else {
        emu.pc = 2140720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa30));
    }
}
#[inline(always)]
pub fn block_0x0020aa30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2140724u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2140728u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2140732u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2140784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa70));
    } else {
        emu.pc = 2140736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa40));
    }
}
#[inline(always)]
pub fn block_0x0020aa40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2140740u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2140784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa70));
    } else {
        emu.pc = 2140744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa48));
    }
}
#[inline(always)]
pub fn block_0x0020aa48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2140748u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2140752u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2140756u32);
    emu.apc_no_count(1usize, 2140756u32, 4294930432u32, 2140760u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140764u32;
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
#[inline(always)]
pub fn block_0x0020aa5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa90));
    } else {
        emu.pc = 2140768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa60));
    }
}
#[inline(always)]
pub fn block_0x0020aa60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2140772u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2140776u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2140780u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2140784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aaa4));
}
#[inline(always)]
pub fn block_0x0020aa70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aac0));
    } else {
        emu.pc = 2140788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa74));
    }
}
#[inline(always)]
pub fn block_0x0020aa74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2140788u32, 4294942720u32, 2140792u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aa7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2140800u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2140804u32);
    emu.apc_no_count(1usize, 2140804u32, 4294930432u32, 2140808u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140812u32;
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
pub fn block_0x0020aa8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa60));
    } else {
        emu.pc = 2140816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa90));
    }
}
#[inline(always)]
pub fn block_0x0020aa90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2140820u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2140824u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2140828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aaa0));
}
#[inline(always)]
pub fn block_0x0020aa9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2140832u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aaa0));
}
#[inline(always)]
pub fn block_0x0020aaa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2140836u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aaa4));
}
#[inline(always)]
pub fn block_0x0020aaa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2140840u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2140844u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140848u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2140852u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2140856u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140860u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140864u32;
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
pub fn block_0x0020aac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2140868u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2140768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa60));
    } else {
        emu.pc = 2140872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aac8));
    }
}
#[inline(always)]
pub fn block_0x0020aac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2140876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140816u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa90));
}
#[inline]
pub fn block_0x0020aacc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2140880u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2140884u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2140888u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2140892u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2140896u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2140900u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2140904u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2140908u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2140912u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2140916u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2140924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aafc));
    } else {
        emu.pc = 2140920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aaf8));
    }
}
#[inline(always)]
pub fn block_0x0020aaf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2140924u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aafc));
}
#[inline(always)]
pub fn block_0x0020aafc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 13usize, 26u32, 2140928u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2140932u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab1c));
    } else {
        emu.pc = 2140936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab08));
    }
}
#[inline(always)]
pub fn block_0x0020ab08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 18usize, 5u32, 2140940u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2141064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab88));
    } else {
        emu.pc = 2140944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab10));
    }
}
#[inline(always)]
pub fn block_0x0020ab10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2140968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab28));
    } else {
        emu.pc = 2140948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab14));
    }
}
#[inline(always)]
pub fn block_0x0020ab14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140952u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2140956u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab3c));
}
#[inline(always)]
pub fn block_0x0020ab1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2140960u32);
    emu.apc_no_count(1usize, 2140960u32, 12288u32, 2140964u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ab28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2140972u32)?;
    emu.sli_no_count(13usize, 13usize, 5u32, 2140976u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2140980u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2140984u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2140988u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2140988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab3c));
}
#[inline(always)]
pub fn block_0x0020ab3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2140992u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2140996u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2141000u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2141004u32);
    emu.apc_no_count(1usize, 2141004u32, 0u32, 2141008u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141012u32;
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
#[inline(always)]
pub fn block_0x0020ab54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2141016u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2141056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab80));
    } else {
        emu.pc = 2141020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab5c));
    }
}
#[inline]
pub fn block_0x0020ab5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2141024u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2141028u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2141032u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2141036u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2141040u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2141044u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2141048u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2141052u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141056u32;
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
pub fn block_0x0020ab80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2141060u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2141064u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2141064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab88));
}
#[inline(always)]
pub fn block_0x0020ab88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2141068u32);
    emu.apc_no_count(1usize, 2141068u32, 12288u32, 2141072u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141076u32;
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
#[inline(always)]
pub fn block_0x0020ab94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141080u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2141084u32)?;
    emu.adi_no_count(10usize, 0usize, 128u32, 2141088u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2141092u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2141108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abb4));
    } else {
        emu.pc = 2141096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aba8));
    }
}
#[inline(always)]
pub fn block_0x0020aba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 8u32, 2141100u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2141104u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2141108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac50));
}
#[inline(always)]
pub fn block_0x0020abb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2141112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2141148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abdc));
    } else {
        emu.pc = 2141116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abbc));
    }
}
#[inline(always)]
pub fn block_0x0020abbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2141120u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2141124u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2141128u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2141132u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2141136u32);
    emu.sb_no_count(11usize, 2usize, 9u32, 2141140u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2141144u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2141148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac50));
}
#[inline(always)]
pub fn block_0x0020abdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2141152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2141204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac14));
    } else {
        emu.pc = 2141156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abe4));
    }
}
#[inline]
pub fn block_0x0020abe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2141160u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2141164u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2141168u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2141172u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2141176u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2141180u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2141184u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2141188u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2141192u32);
    emu.sb_no_count(11usize, 2usize, 10u32, 2141196u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2141200u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2141204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac50));
}
#[inline]
pub fn block_0x0020ac14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2141208u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2141212u32);
    emu.sli_no_count(13usize, 11usize, 20u32, 2141216u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2141220u32);
    emu.ori_no_count(10usize, 10usize, 240u32, 2141224u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2141228u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2141232u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2141236u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2141240u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2141244u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2141248u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2141252u32);
    emu.sb_no_count(13usize, 2usize, 10u32, 2141256u32);
    emu.sb_no_count(11usize, 2usize, 11u32, 2141260u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2141264u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2141264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac50));
}
#[inline(always)]
pub fn block_0x0020ac50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2141268u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2141272u32);
    emu.apc_no_count(1usize, 2141272u32, 4294946816u32, 2141276u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ac60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2141284u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2141288u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141292u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141296u32;
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
