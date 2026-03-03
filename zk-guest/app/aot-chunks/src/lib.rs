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
        pc_max: 2100920u32,
        lookup: pico_aot_chunk_000::lookup,
    },
    ChunkDesc {
        pc_min: 2100928u32,
        pc_max: 2103328u32,
        lookup: pico_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2103356u32,
        pc_max: 2106732u32,
        lookup: pico_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2106748u32,
        pc_max: 2108924u32,
        lookup: pico_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2108984u32,
        pc_max: 2111840u32,
        lookup: pico_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2111864u32,
        pc_max: 2114292u32,
        lookup: pico_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2114328u32,
        pc_max: 2116340u32,
        lookup: pico_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2116348u32,
        pc_max: 2119320u32,
        lookup: pico_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2119324u32,
        pc_max: 2121912u32,
        lookup: pico_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2121932u32,
        pc_max: 2141280u32,
        lookup: pico_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2141296u32,
        pc_max: 2143524u32,
        lookup: pico_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2143528u32,
        pc_max: 2145620u32,
        lookup: pico_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2145672u32,
        pc_max: 2147944u32,
        lookup: pico_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2147980u32,
        pc_max: 2150684u32,
        lookup: pico_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2150708u32,
        pc_max: 2153012u32,
        lookup: pico_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2153036u32,
        pc_max: 2155120u32,
        lookup: pico_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2155164u32,
        pc_max: 2158060u32,
        lookup: pico_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2158064u32,
        pc_max: 2162004u32,
        lookup: pico_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2162020u32,
        pc_max: 2164328u32,
        lookup: pico_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2164388u32,
        pc_max: 2166708u32,
        lookup: pico_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2166740u32,
        pc_max: 2168852u32,
        lookup: pico_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2168880u32,
        pc_max: 2171488u32,
        lookup: pico_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2171496u32,
        pc_max: 2172924u32,
        lookup: pico_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2172948u32,
        pc_max: 2175500u32,
        lookup: pico_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2175524u32,
        pc_max: 2178176u32,
        lookup: pico_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2178180u32,
        pc_max: 2180404u32,
        lookup: pico_aot_chunk_025::lookup,
    },
    ChunkDesc {
        pc_min: 2180432u32,
        pc_max: 2182400u32,
        lookup: pico_aot_chunk_026::lookup,
    },
    ChunkDesc {
        pc_min: 2182428u32,
        pc_max: 2184272u32,
        lookup: pico_aot_chunk_027::lookup,
    },
    ChunkDesc {
        pc_min: 2184288u32,
        pc_max: 2186668u32,
        lookup: pico_aot_chunk_028::lookup,
    },
    ChunkDesc {
        pc_min: 2186676u32,
        pc_max: 2188216u32,
        lookup: pico_aot_chunk_029::lookup,
    },
    ChunkDesc {
        pc_min: 2188236u32,
        pc_max: 2190208u32,
        lookup: pico_aot_chunk_030::lookup,
    },
    ChunkDesc {
        pc_min: 2190216u32,
        pc_max: 2192660u32,
        lookup: pico_aot_chunk_031::lookup,
    },
    ChunkDesc {
        pc_min: 2192688u32,
        pc_max: 2194984u32,
        lookup: pico_aot_chunk_032::lookup,
    },
    ChunkDesc {
        pc_min: 2194992u32,
        pc_max: 2194992u32,
        lookup: pico_aot_chunk_033::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 24usize] = [
    0u16, 1u16, 3u16, 4u16, 6u16, 8u16, 9u16, 9u16, 9u16, 9u16, 9u16, 11u16, 13u16,
    14u16, 16u16, 17u16, 19u16, 20u16, 23u16, 24u16, 26u16, 28u16, 30u16, 32u16,
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
