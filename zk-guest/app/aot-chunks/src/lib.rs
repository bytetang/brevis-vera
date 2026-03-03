pub use pico_aot_runtime::AotEmulatorCore;
use pico_aot_runtime::{set_lookup_block_fn, BlockFn, NextStep};
pub fn run_aot(emu: &mut AotEmulatorCore) -> Result<(), String> {
    set_lookup_block_fn(lookup_block);
    let mut next = if emu.pc == 0 {
        NextStep::Halt
    } else if let Some(func) = lookup_block(emu.pc) {
        NextStep::Direct(func)
    } else {
        NextStep::Dynamic(emu.pc)
    };
    loop {
        if emu.should_yield() {
            break;
        }
        match next {
            NextStep::Direct(func) => {
                next = func(emu)?;
            }
            NextStep::Dynamic(pc) => {
                emu.pc = pc;
                if emu.pc == 0 {
                    next = NextStep::Halt;
                } else if let Some(func) = lookup_block(pc) {
                    next = NextStep::Direct(func);
                } else {
                    next = emu.interpret_from_current_pc()?;
                }
            }
            NextStep::Halt => break,
        }
    }
    Ok(())
}
type ChunkLookupFn = fn(u32) -> Option<BlockFn>;
#[repr(C)]
struct ChunkDesc {
    pc_min: u32,
    pc_max: u32,
    lookup: ChunkLookupFn,
}
const CHUNKS: &[ChunkDesc] = &[
    ChunkDesc {
        pc_min: 2099200u32,
        pc_max: 2101528u32,
        lookup: pico_aot_chunk_000::lookup,
    },
    ChunkDesc {
        pc_min: 2101548u32,
        pc_max: 2103840u32,
        lookup: pico_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2103868u32,
        pc_max: 2107244u32,
        lookup: pico_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2107260u32,
        pc_max: 2110740u32,
        lookup: pico_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2110744u32,
        pc_max: 2114620u32,
        lookup: pico_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2114668u32,
        pc_max: 2116640u32,
        lookup: pico_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2116660u32,
        pc_max: 2124256u32,
        lookup: pico_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2124284u32,
        pc_max: 2127068u32,
        lookup: pico_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2127108u32,
        pc_max: 2129716u32,
        lookup: pico_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2129740u32,
        pc_max: 2134052u32,
        lookup: pico_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2134144u32,
        pc_max: 2136388u32,
        lookup: pico_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2136396u32,
        pc_max: 2139352u32,
        lookup: pico_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2139356u32,
        pc_max: 2141964u32,
        lookup: pico_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2141984u32,
        pc_max: 2147144u32,
        lookup: pico_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2147272u32,
        pc_max: 2153232u32,
        lookup: pico_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2153356u32,
        pc_max: 2162356u32,
        lookup: pico_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2162440u32,
        pc_max: 2171368u32,
        lookup: pico_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2171680u32,
        pc_max: 2194260u32,
        lookup: pico_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2194376u32,
        pc_max: 2196992u32,
        lookup: pico_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2197004u32,
        pc_max: 2199296u32,
        lookup: pico_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2199336u32,
        pc_max: 2201172u32,
        lookup: pico_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2201188u32,
        pc_max: 2203872u32,
        lookup: pico_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2203928u32,
        pc_max: 2206756u32,
        lookup: pico_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2206768u32,
        pc_max: 2209080u32,
        lookup: pico_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2209148u32,
        pc_max: 2211216u32,
        lookup: pico_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2211260u32,
        pc_max: 2214948u32,
        lookup: pico_aot_chunk_025::lookup,
    },
    ChunkDesc {
        pc_min: 2214952u32,
        pc_max: 2218636u32,
        lookup: pico_aot_chunk_026::lookup,
    },
    ChunkDesc {
        pc_min: 2218684u32,
        pc_max: 2221316u32,
        lookup: pico_aot_chunk_027::lookup,
    },
    ChunkDesc {
        pc_min: 2221376u32,
        pc_max: 2223696u32,
        lookup: pico_aot_chunk_028::lookup,
    },
    ChunkDesc {
        pc_min: 2223728u32,
        pc_max: 2225840u32,
        lookup: pico_aot_chunk_029::lookup,
    },
    ChunkDesc {
        pc_min: 2225868u32,
        pc_max: 2228476u32,
        lookup: pico_aot_chunk_030::lookup,
    },
    ChunkDesc {
        pc_min: 2228484u32,
        pc_max: 2229936u32,
        lookup: pico_aot_chunk_031::lookup,
    },
    ChunkDesc {
        pc_min: 2230008u32,
        pc_max: 2232488u32,
        lookup: pico_aot_chunk_032::lookup,
    },
    ChunkDesc {
        pc_min: 2232512u32,
        pc_max: 2235164u32,
        lookup: pico_aot_chunk_033::lookup,
    },
    ChunkDesc {
        pc_min: 2235168u32,
        pc_max: 2237392u32,
        lookup: pico_aot_chunk_034::lookup,
    },
    ChunkDesc {
        pc_min: 2237420u32,
        pc_max: 2239624u32,
        lookup: pico_aot_chunk_035::lookup,
    },
    ChunkDesc {
        pc_min: 2239652u32,
        pc_max: 2241496u32,
        lookup: pico_aot_chunk_036::lookup,
    },
    ChunkDesc {
        pc_min: 2241512u32,
        pc_max: 2243892u32,
        lookup: pico_aot_chunk_037::lookup,
    },
    ChunkDesc {
        pc_min: 2243900u32,
        pc_max: 2245440u32,
        lookup: pico_aot_chunk_038::lookup,
    },
    ChunkDesc {
        pc_min: 2245460u32,
        pc_max: 2247432u32,
        lookup: pico_aot_chunk_039::lookup,
    },
    ChunkDesc {
        pc_min: 2247440u32,
        pc_max: 2249940u32,
        lookup: pico_aot_chunk_040::lookup,
    },
    ChunkDesc {
        pc_min: 2249964u32,
        pc_max: 2252328u32,
        lookup: pico_aot_chunk_041::lookup,
    },
    ChunkDesc {
        pc_min: 2252336u32,
        pc_max: 2252336u32,
        lookup: pico_aot_chunk_042::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 38usize] = [
    0u16, 1u16, 3u16, 4u16, 5u16, 6u16, 6u16, 8u16, 9u16, 10u16, 12u16, 13u16, 14u16,
    14u16, 15u16, 15u16, 16u16, 16u16, 17u16, 17u16, 17u16, 17u16, 17u16, 17u16, 19u16,
    21u16, 22u16, 24u16, 25u16, 26u16, 28u16, 30u16, 32u16, 33u16, 35u16, 37u16, 39u16,
    41u16,
];
fn lookup_block(pc: u32) -> Option<BlockFn> {
    if CHUNKS.is_empty() {
        return None;
    }
    if pc < GLOBAL_PC_MIN {
        return None;
    }
    let off = pc - GLOBAL_PC_MIN;
    let page = (off >> PAGE_SHIFT) as usize;
    let mut idx = if page < PAGE_HINT.len() {
        PAGE_HINT[page] as usize
    } else {
        CHUNKS.len().saturating_sub(1)
    };
    while idx < CHUNKS.len() && pc > CHUNKS[idx].pc_max {
        idx += 1;
    }
    if idx == CHUNKS.len() {
        return None;
    }
    let c = &CHUNKS[idx];
    if pc < c.pc_min {
        return None;
    }
    (c.lookup)(pc)
}
