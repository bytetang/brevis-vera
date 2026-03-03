pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2141984u32;
pub const PC_MAX: u32 = 2147144u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 164usize] = [
        block_0x0020af20,
        block_0x0020af28,
        block_0x0020af38,
        block_0x0020af3c,
        block_0x0020af64,
        block_0x0020af88,
        block_0x0020af98,
        block_0x0020afa0,
        block_0x0020afb0,
        block_0x0020afb4,
        block_0x0020afbc,
        block_0x0020afc0,
        block_0x0020aff4,
        block_0x0020affc,
        block_0x0020b00c,
        block_0x0020b014,
        block_0x0020b01c,
        block_0x0020b024,
        block_0x0020b02c,
        block_0x0020b05c,
        block_0x0020b090,
        block_0x0020b094,
        block_0x0020b0b8,
        block_0x0020b0bc,
        block_0x0020b0f4,
        block_0x0020b0f8,
        block_0x0020b11c,
        block_0x0020b120,
        block_0x0020b13c,
        block_0x0020b140,
        block_0x0020b154,
        block_0x0020b158,
        block_0x0020b184,
        block_0x0020b188,
        block_0x0020b1b0,
        block_0x0020b1b8,
        block_0x0020b1bc,
        block_0x0020b1ec,
        block_0x0020b1f0,
        block_0x0020b218,
        block_0x0020b21c,
        block_0x0020b23c,
        block_0x0020b244,
        block_0x0020b248,
        block_0x0020b278,
        block_0x0020b27c,
        block_0x0020b29c,
        block_0x0020b2a0,
        block_0x0020b2e8,
        block_0x0020b2ec,
        block_0x0020b330,
        block_0x0020b35c,
        block_0x0020b360,
        block_0x0020b38c,
        block_0x0020b390,
        block_0x0020b3cc,
        block_0x0020b3d0,
        block_0x0020b3ec,
        block_0x0020b3f0,
        block_0x0020b424,
        block_0x0020b428,
        block_0x0020b454,
        block_0x0020b458,
        block_0x0020b4a0,
        block_0x0020b4a4,
        block_0x0020b4d4,
        block_0x0020b4d8,
        block_0x0020b520,
        block_0x0020b524,
        block_0x0020b568,
        block_0x0020b678,
        block_0x0020b67c,
        block_0x0020b6f8,
        block_0x0020b6fc,
        block_0x0020b758,
        block_0x0020b75c,
        block_0x0020b7cc,
        block_0x0020b7d0,
        block_0x0020b850,
        block_0x0020b854,
        block_0x0020b880,
        block_0x0020b884,
        block_0x0020b8fc,
        block_0x0020b900,
        block_0x0020b924,
        block_0x0020b928,
        block_0x0020b97c,
        block_0x0020b980,
        block_0x0020b9a4,
        block_0x0020b9a8,
        block_0x0020ba20,
        block_0x0020ba24,
        block_0x0020baa4,
        block_0x0020baa8,
        block_0x0020bad4,
        block_0x0020bad8,
        block_0x0020bb50,
        block_0x0020bb54,
        block_0x0020bb7c,
        block_0x0020bb80,
        block_0x0020bbd8,
        block_0x0020bbdc,
        block_0x0020bc04,
        block_0x0020bc08,
        block_0x0020bc78,
        block_0x0020bc7c,
        block_0x0020bcf8,
        block_0x0020bcfc,
        block_0x0020bd28,
        block_0x0020bd2c,
        block_0x0020bda4,
        block_0x0020bda8,
        block_0x0020bdd4,
        block_0x0020bdd8,
        block_0x0020be18,
        block_0x0020be1c,
        block_0x0020be34,
        block_0x0020be38,
        block_0x0020be64,
        block_0x0020be68,
        block_0x0020becc,
        block_0x0020bed0,
        block_0x0020bef0,
        block_0x0020bef4,
        block_0x0020bf1c,
        block_0x0020bf20,
        block_0x0020bf9c,
        block_0x0020bfa0,
        block_0x0020bfe0,
        block_0x0020bfe4,
        block_0x0020c018,
        block_0x0020c01c,
        block_0x0020c06c,
        block_0x0020c070,
        block_0x0020c0bc,
        block_0x0020c0c0,
        block_0x0020c0e0,
        block_0x0020c0e4,
        block_0x0020c114,
        block_0x0020c118,
        block_0x0020c15c,
        block_0x0020c160,
        block_0x0020c1a0,
        block_0x0020c1a4,
        block_0x0020c1b8,
        block_0x0020c1bc,
        block_0x0020c1e8,
        block_0x0020c1ec,
        block_0x0020c20c,
        block_0x0020c214,
        block_0x0020c218,
        block_0x0020c248,
        block_0x0020c24c,
        block_0x0020c274,
        block_0x0020c278,
        block_0x0020c298,
        block_0x0020c2a0,
        block_0x0020c2a4,
        block_0x0020c2d4,
        block_0x0020c2d8,
        block_0x0020c2f8,
        block_0x0020c2fc,
        block_0x0020c344,
        block_0x0020c348,
    ];
    const IDX: [u16; 1291usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        6u16, 0u16, 0u16, 0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 0u16, 9u16, 10u16, 0u16,
        11u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 17u16,
        0u16, 18u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16,
        24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 25u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 28u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 30u16, 0u16, 0u16, 0u16, 0u16, 31u16,
        32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 34u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 36u16, 37u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 41u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 43u16, 44u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16,
        53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 55u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 71u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 78u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 79u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 84u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 90u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 102u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 106u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 107u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 112u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 115u16, 116u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 117u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 119u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 121u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        123u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 125u16,
        126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 127u16, 128u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 129u16, 130u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 131u16, 132u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 133u16, 134u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 135u16,
        136u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 137u16, 138u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 139u16, 140u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 141u16, 142u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 143u16, 144u16, 0u16, 0u16, 0u16, 0u16, 145u16,
        146u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 147u16,
        148u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 149u16, 0u16, 150u16, 151u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 152u16, 153u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 154u16, 155u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 156u16, 0u16, 157u16, 158u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 159u16, 160u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 161u16, 162u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 163u16, 164u16,
    ];
    if pc < 2141984u32 || pc > 2147144u32 {
        return None;
    }
    let word_offset = ((pc - 2141984u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020af20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2141984u32, 4294963200u32, 2141988u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020af28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2141996u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2142000u32);
    emu.apc_no_count(1usize, 2142000u32, 4294930432u32, 2142004u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142008u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(72u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020af38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2142088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af88));
    } else {
        emu.pc = 2142012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af3c));
    }
}
#[inline]
pub fn block_0x0020af3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2142016u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2142020u32)?;
    emu.sw_no_count(19usize, 10usize, 4u32, 2142024u32)?;
    emu.sw_no_count(8usize, 10usize, 8u32, 2142028u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2142032u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 44u32, 2142036u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2142040u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2142044u32);
    emu.apc_no_count(1usize, 2142044u32, 65536u32, 2142048u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020af64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2142056u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2142060u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2142064u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2142068u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2142072u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2142076u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2142080u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2142084u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142088u32;
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
pub fn block_0x0020af88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2142092u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2142096u32);
    emu.apc_no_count(1usize, 2142096u32, 69632u32, 2142100u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020af98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2142108u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2142128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afb0));
    } else {
        emu.pc = 2142112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afa0));
    }
}
#[inline(always)]
pub fn block_0x0020afa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2142116u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2142120u32);
    emu.apc_no_count(6usize, 2142120u32, 4294930432u32, 2142124u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2142128u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020afb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142132u32;
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
pub fn block_0x0020afb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2142136u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142140u32;
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
pub fn block_0x0020afbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142144u32;
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
pub fn block_0x0020afc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(129052672u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2142148u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(257171456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2142152u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4247375872u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2142156u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(953024512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2142160u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2142164u32);
    emu.adi_no_count(12usize, 12usize, 1173u32, 2142168u32);
    emu.adi_no_count(13usize, 13usize, 1807u32, 2142172u32);
    emu.adi_no_count(14usize, 14usize, 937u32, 2142176u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2142180u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2142184u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2142188u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2142192u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142196u32;
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
pub fn block_0x0020aff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2142200u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142204u32;
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
pub fn block_0x0020affc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2142208u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2142212u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2142216u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2142236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b01c));
    } else {
        emu.pc = 2142220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b00c));
    }
}
#[inline(always)]
pub fn block_0x0020b00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2142224u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2142244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b024));
    } else {
        emu.pc = 2142228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b014));
    }
}
#[inline(always)]
pub fn block_0x0020b014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2142228u32, 73728u32, 2142232u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2142236u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b01c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2142236u32, 73728u32, 2142240u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2142244u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020b024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2142244u32, 73728u32, 2142248u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2142252u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2142256u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2142260u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2142264u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2142268u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2142272u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2142276u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 104u32, 2142280u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2142284u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2142288u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2142292u32);
    emu.apc_no_count(1usize, 2142292u32, 77824u32, 2142296u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2142300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020b05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 12usize, 0u32, 2142304u32)?;
    emu.lw_no_count(15usize, 12usize, 4u32, 2142308u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2142312u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2142316u32)?;
    emu.lw_no_count(17usize, 11usize, 8u32, 2142320u32)?;
    emu.lw_no_count(30usize, 11usize, 12u32, 2142324u32)?;
    emu.lw_no_count(29usize, 12usize, 8u32, 2142328u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2142332u32)?;
    emu.adr_no_count(5usize, 15usize, 5usize, 2142336u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2142340u32);
    emu.sltru_no_count(28usize, 13usize, 14usize, 2142344u32);
    emu.adr_no_count(14usize, 5usize, 28usize, 2142348u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2142356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b094));
    } else {
        emu.pc = 2142352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b090));
    }
}
#[inline(always)]
pub fn block_0x0020b090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 15usize, 2142356u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142356u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b094));
}
#[inline]
pub fn block_0x0020b094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 11usize, 16u32, 2142360u32)?;
    emu.lw_no_count(7usize, 11usize, 20u32, 2142364u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2142368u32)?;
    emu.lw_no_count(15usize, 12usize, 20u32, 2142372u32)?;
    emu.adr_no_count(30usize, 16usize, 30usize, 2142376u32);
    emu.adr_no_count(31usize, 29usize, 17usize, 2142380u32);
    emu.sltru_no_count(17usize, 31usize, 29usize, 2142384u32);
    emu.adr_no_count(30usize, 30usize, 17usize, 2142388u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2142396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0bc));
    } else {
        emu.pc = 2142392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0b8));
    }
}
#[inline(always)]
pub fn block_0x0020b0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 30usize, 16usize, 2142396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b0bc));
}
#[inline]
pub fn block_0x0020b0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2142400u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2142404u32)?;
    emu.adr_no_count(28usize, 31usize, 28usize, 2142408u32);
    emu.adr_no_count(7usize, 15usize, 7usize, 2142412u32);
    emu.adr_no_count(16usize, 6usize, 5usize, 2142416u32);
    emu.sltru_no_count(31usize, 28usize, 31usize, 2142420u32);
    emu.sltru_no_count(5usize, 16usize, 6usize, 2142424u32);
    emu.adr_no_count(29usize, 30usize, 31usize, 2142428u32);
    emu.sltru_no_count(6usize, 29usize, 30usize, 2142432u32);
    emu.anr_no_count(31usize, 31usize, 6usize, 2142436u32);
    emu.adr_no_count(31usize, 17usize, 31usize, 2142440u32);
    emu.adr_no_count(6usize, 7usize, 5usize, 2142444u32);
    emu.sltru_no_count(8usize, 31usize, 17usize, 2142448u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2142456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0f8));
    } else {
        emu.pc = 2142452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0f4));
    }
}
#[inline(always)]
pub fn block_0x0020b0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 15usize, 2142456u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b0f8));
}
#[inline]
pub fn block_0x0020b0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 11usize, 24u32, 2142460u32)?;
    emu.lw_no_count(30usize, 11usize, 28u32, 2142464u32)?;
    emu.lw_no_count(7usize, 12usize, 24u32, 2142468u32)?;
    emu.lw_no_count(11usize, 12usize, 28u32, 2142472u32)?;
    emu.adr_no_count(15usize, 16usize, 31usize, 2142476u32);
    emu.sltru_no_count(12usize, 15usize, 16usize, 2142480u32);
    emu.adr_no_count(16usize, 6usize, 8usize, 2142484u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2142488u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2142496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b120));
    } else {
        emu.pc = 2142492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b11c));
    }
}
#[inline(always)]
pub fn block_0x0020b11c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 6usize, 2142496u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b120));
}
#[inline(always)]
pub fn block_0x0020b120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 5usize, 12usize, 2142500u32);
    emu.adr_no_count(12usize, 11usize, 30usize, 2142504u32);
    emu.adr_no_count(30usize, 7usize, 17usize, 2142508u32);
    emu.sltru_no_count(17usize, 30usize, 7usize, 2142512u32);
    emu.adr_no_count(12usize, 12usize, 17usize, 2142516u32);
    emu.sltru_no_count(5usize, 6usize, 5usize, 2142520u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2142528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b140));
    } else {
        emu.pc = 2142524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b13c));
    }
}
#[inline(always)]
pub fn block_0x0020b13c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 12usize, 11usize, 2142528u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142528u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b140));
}
#[inline(always)]
pub fn block_0x0020b140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 30usize, 6usize, 2142532u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2142536u32);
    emu.adr_no_count(5usize, 12usize, 5usize, 2142540u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2142544u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2142552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b158));
    } else {
        emu.pc = 2142548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b154));
    }
}
#[inline(always)]
pub fn block_0x0020b154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 12usize, 2142552u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b158));
}
#[inline]
pub fn block_0x0020b158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 1u32, 2142556u32);
    emu.sltiu_no_count(11usize, 12usize, 1u32, 2142560u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2142564u32);
    emu.orr_no_count(13usize, 12usize, 11usize, 2142568u32);
    emu.sltru_no_count(30usize, 0usize, 13usize, 2142572u32);
    emu.sbr_no_count(13usize, 0usize, 30usize, 2142576u32);
    emu.sbr_no_count(28usize, 28usize, 30usize, 2142580u32);
    emu.sltru_no_count(14usize, 28usize, 13usize, 2142584u32);
    emu.sbr_no_count(31usize, 29usize, 30usize, 2142588u32);
    emu.adr_no_count(31usize, 31usize, 14usize, 2142592u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2142600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b188));
    } else {
        emu.pc = 2142596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b184));
    }
}
#[inline(always)]
pub fn block_0x0020b184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 13usize, 2142600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b188));
}
#[inline]
pub fn block_0x0020b188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(29usize, 14usize, 30usize, 2142604u32);
    emu.adi_no_count(14usize, 28usize, 1u32, 2142608u32);
    emu.sltru_no_count(8usize, 29usize, 13usize, 2142612u32);
    emu.sltiu_no_count(13usize, 14usize, 1u32, 2142616u32);
    emu.adr_no_count(13usize, 31usize, 13usize, 2142620u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2142624u32);
    emu.sbr_no_count(30usize, 8usize, 30usize, 2142628u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2142632u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2142636u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2142648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1b8));
    } else {
        emu.pc = 2142640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1b0));
    }
}
#[inline(always)]
pub fn block_0x0020b1b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 31usize, 2142644u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2142648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2142652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b1bc));
}
#[inline(always)]
pub fn block_0x0020b1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 28usize, 2142652u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142652u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b1bc));
}
#[inline]
pub fn block_0x0020b1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 29usize, 4294967295u32, 2142656u32);
    emu.sltiu_no_count(29usize, 29usize, 1u32, 2142660u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2142664u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2142668u32);
    emu.sltru_no_count(28usize, 28usize, 31usize, 2142672u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2142676u32);
    emu.sai_no_count(28usize, 28usize, 1055u32, 2142680u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2142684u32);
    emu.sltru_no_count(29usize, 15usize, 28usize, 2142688u32);
    emu.adr_no_count(16usize, 28usize, 16usize, 2142692u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2142696u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2142704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1f0));
    } else {
        emu.pc = 2142700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1ec));
    }
}
#[inline(always)]
pub fn block_0x0020b1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 16usize, 28usize, 2142704u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b1f0));
}
#[inline]
pub fn block_0x0020b1f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 28usize, 29usize, 2142708u32);
    emu.sltru_no_count(29usize, 29usize, 28usize, 2142712u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2142716u32);
    emu.sai_no_count(30usize, 28usize, 1055u32, 2142720u32);
    emu.adr_no_count(31usize, 5usize, 30usize, 2142724u32);
    emu.adr_no_count(28usize, 6usize, 30usize, 2142728u32);
    emu.sltru_no_count(29usize, 28usize, 6usize, 2142732u32);
    emu.adr_no_count(31usize, 31usize, 29usize, 2142736u32);
    emu.adr_no_count(7usize, 17usize, 7usize, 2142740u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2142748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b21c));
    } else {
        emu.pc = 2142744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b218));
    }
}
#[inline(always)]
pub fn block_0x0020b218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 31usize, 5usize, 2142748u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b21c));
}
#[inline(always)]
pub fn block_0x0020b21c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 7usize, 17usize, 2142752u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2142756u32);
    emu.sltru_no_count(5usize, 0usize, 28usize, 2142760u32);
    emu.sltru_no_count(17usize, 29usize, 30usize, 2142764u32);
    emu.adr_no_count(5usize, 31usize, 5usize, 2142768u32);
    emu.adr_no_count(30usize, 30usize, 17usize, 2142772u32);
    emu.adi_no_count(17usize, 28usize, 4294967295u32, 2142776u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2142788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b244));
    } else {
        emu.pc = 2142780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b23c));
    }
}
#[inline(always)]
pub fn block_0x0020b23c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 5usize, 31usize, 2142784u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2142788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2142792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b248));
}
#[inline(always)]
pub fn block_0x0020b244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 17usize, 28usize, 2142792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b248));
}
#[inline]
pub fn block_0x0020b248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 29usize, 4294967295u32, 2142796u32);
    emu.sltiu_no_count(29usize, 29usize, 1u32, 2142800u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2142804u32);
    emu.sbr_no_count(29usize, 30usize, 29usize, 2142808u32);
    emu.sltru_no_count(28usize, 28usize, 31usize, 2142812u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2142816u32);
    emu.sai_no_count(28usize, 28usize, 1055u32, 2142820u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2142824u32);
    emu.sltru_no_count(7usize, 7usize, 28usize, 2142828u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2142832u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2142836u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2142844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b27c));
    } else {
        emu.pc = 2142840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b278));
    }
}
#[inline(always)]
pub fn block_0x0020b278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 6usize, 28usize, 2142844u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142844u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b27c));
}
#[inline(always)]
pub fn block_0x0020b27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 28usize, 7usize, 2142848u32);
    emu.sltru_no_count(7usize, 6usize, 28usize, 2142852u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2142856u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2142860u32);
    emu.sltru_no_count(28usize, 12usize, 6usize, 2142864u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2142868u32);
    emu.adr_no_count(11usize, 11usize, 28usize, 2142872u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2142880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2a0));
    } else {
        emu.pc = 2142876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b29c));
    }
}
#[inline(always)]
pub fn block_0x0020b29c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 11usize, 7usize, 2142880u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b2a0));
}
#[inline]
pub fn block_0x0020b2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 6usize, 14usize, 2142884u32);
    emu.sltru_no_count(30usize, 29usize, 6usize, 2142888u32);
    emu.adr_no_count(14usize, 29usize, 28usize, 2142892u32);
    emu.adr_no_count(28usize, 13usize, 30usize, 2142896u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2142900u32);
    emu.sltru_no_count(13usize, 0usize, 28usize, 2142904u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2142908u32);
    emu.anr_no_count(30usize, 13usize, 30usize, 2142912u32);
    emu.adr_no_count(13usize, 28usize, 29usize, 2142916u32);
    emu.sltru_no_count(28usize, 13usize, 28usize, 2142920u32);
    emu.anr_no_count(28usize, 29usize, 28usize, 2142924u32);
    emu.adr_no_count(28usize, 30usize, 28usize, 2142928u32);
    emu.sltru_no_count(29usize, 28usize, 30usize, 2142932u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2142936u32);
    emu.sltru_no_count(28usize, 15usize, 28usize, 2142940u32);
    emu.adr_no_count(16usize, 29usize, 16usize, 2142944u32);
    emu.adr_no_count(16usize, 16usize, 28usize, 2142948u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2142956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2ec));
    } else {
        emu.pc = 2142952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2e8));
    }
}
#[inline(always)]
pub fn block_0x0020b2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 16usize, 29usize, 2142956u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b2ec));
}
#[inline]
pub fn block_0x0020b2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(6usize, 6usize, 1u32, 2142960u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2142964u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2142968u32)?;
    emu.sw_no_count(11usize, 10usize, 4u32, 2142972u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2142976u32)?;
    emu.sw_no_count(13usize, 10usize, 12u32, 2142980u32)?;
    emu.adr_no_count(17usize, 6usize, 17usize, 2142984u32);
    emu.sltru_no_count(11usize, 17usize, 6usize, 2142988u32);
    emu.adr_no_count(28usize, 17usize, 28usize, 2142992u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2142996u32);
    emu.sltru_no_count(12usize, 28usize, 17usize, 2143000u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2143004u32);
    emu.sw_no_count(15usize, 10usize, 16u32, 2143008u32)?;
    emu.sw_no_count(16usize, 10usize, 20u32, 2143012u32)?;
    emu.sw_no_count(28usize, 10usize, 24u32, 2143016u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2143020u32)?;
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143024u32;
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
pub fn block_0x0020b330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 11usize, 0u32, 2143028u32)?;
    emu.lw_no_count(5usize, 11usize, 4u32, 2143032u32)?;
    emu.lw_no_count(13usize, 11usize, 8u32, 2143036u32)?;
    emu.lw_no_count(7usize, 11usize, 12u32, 2143040u32)?;
    emu.lw_no_count(16usize, 12usize, 0u32, 2143044u32)?;
    emu.lw_no_count(6usize, 12usize, 4u32, 2143048u32)?;
    emu.lw_no_count(30usize, 12usize, 8u32, 2143052u32)?;
    emu.lw_no_count(29usize, 12usize, 12u32, 2143056u32)?;
    emu.sltru_no_count(17usize, 14usize, 16usize, 2143060u32);
    emu.adi_no_count(15usize, 17usize, 0u32, 2143064u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2143072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b360));
    } else {
        emu.pc = 2143068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b35c));
    }
}
#[inline(always)]
pub fn block_0x0020b35c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 5usize, 6usize, 2143072u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b360));
}
#[inline]
pub fn block_0x0020b360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2143076u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2143080u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2143084u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2143088u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2143092u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2143096u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2143100u32)?;
    emu.sltru_no_count(8usize, 13usize, 30usize, 2143104u32);
    emu.sbr_no_count(28usize, 7usize, 29usize, 2143108u32);
    emu.sbr_no_count(28usize, 28usize, 8usize, 2143112u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2143120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b390));
    } else {
        emu.pc = 2143116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b38c));
    }
}
#[inline(always)]
pub fn block_0x0020b38c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 7usize, 29usize, 2143120u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143120u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b390));
}
#[inline]
pub fn block_0x0020b390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 11usize, 16u32, 2143124u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2143128u32)?;
    emu.lw_no_count(29usize, 12usize, 16u32, 2143132u32)?;
    emu.lw_no_count(9usize, 12usize, 20u32, 2143136u32)?;
    emu.sbr_no_count(18usize, 0usize, 8usize, 2143140u32);
    emu.sbr_no_count(13usize, 13usize, 30usize, 2143144u32);
    emu.adr_no_count(30usize, 15usize, 8usize, 2143148u32);
    emu.sbr_no_count(20usize, 28usize, 15usize, 2143152u32);
    emu.sbr_no_count(8usize, 0usize, 30usize, 2143156u32);
    emu.sbr_no_count(15usize, 13usize, 15usize, 2143160u32);
    emu.sltru_no_count(18usize, 8usize, 18usize, 2143164u32);
    emu.sltru_no_count(19usize, 15usize, 13usize, 2143168u32);
    emu.adr_no_count(13usize, 20usize, 19usize, 2143172u32);
    emu.sbr_no_count(18usize, 18usize, 30usize, 2143176u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2143184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3d0));
    } else {
        emu.pc = 2143180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3cc));
    }
}
#[inline(always)]
pub fn block_0x0020b3cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(19usize, 13usize, 28usize, 2143184u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b3d0));
}
#[inline(always)]
pub fn block_0x0020b3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(28usize, 19usize, 30usize, 2143188u32);
    emu.sltru_no_count(28usize, 28usize, 8usize, 2143192u32);
    emu.adr_no_count(28usize, 18usize, 28usize, 2143196u32);
    emu.sltru_no_count(21usize, 7usize, 29usize, 2143200u32);
    emu.sai_no_count(19usize, 28usize, 1055u32, 2143204u32);
    emu.adi_no_count(20usize, 21usize, 0u32, 2143208u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2143216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3f0));
    } else {
        emu.pc = 2143212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3ec));
    }
}
#[inline(always)]
pub fn block_0x0020b3ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 31usize, 9usize, 2143216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b3f0));
}
#[inline]
pub fn block_0x0020b3f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(28usize, 11usize, 24u32, 2143220u32)?;
    emu.lw_no_count(8usize, 11usize, 28u32, 2143224u32)?;
    emu.lw_no_count(30usize, 12usize, 24u32, 2143228u32)?;
    emu.lw_no_count(18usize, 12usize, 28u32, 2143232u32)?;
    emu.sbr_no_count(11usize, 31usize, 9usize, 2143236u32);
    emu.sbr_no_count(12usize, 7usize, 29usize, 2143240u32);
    emu.sbr_no_count(31usize, 11usize, 21usize, 2143244u32);
    emu.adr_no_count(11usize, 12usize, 19usize, 2143248u32);
    emu.adr_no_count(7usize, 31usize, 19usize, 2143252u32);
    emu.sltru_no_count(29usize, 11usize, 12usize, 2143256u32);
    emu.adr_no_count(12usize, 7usize, 29usize, 2143260u32);
    emu.sbr_no_count(7usize, 0usize, 20usize, 2143264u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2143272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b428));
    } else {
        emu.pc = 2143268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b424));
    }
}
#[inline(always)]
pub fn block_0x0020b424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 12usize, 31usize, 2143272u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b428));
}
#[inline]
pub fn block_0x0020b428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 5usize, 6usize, 2143276u32);
    emu.sbr_no_count(6usize, 19usize, 20usize, 2143280u32);
    emu.adr_no_count(29usize, 6usize, 29usize, 2143284u32);
    emu.sltru_no_count(7usize, 6usize, 7usize, 2143288u32);
    emu.sltru_no_count(29usize, 29usize, 6usize, 2143292u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2143296u32);
    emu.adr_no_count(6usize, 6usize, 29usize, 2143300u32);
    emu.sltru_no_count(29usize, 28usize, 30usize, 2143304u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2143308u32);
    emu.adi_no_count(7usize, 29usize, 0u32, 2143312u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2143320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b458));
    } else {
        emu.pc = 2143316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b454));
    }
}
#[inline(always)]
pub fn block_0x0020b454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 8usize, 18usize, 2143320u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b458));
}
#[inline]
pub fn block_0x0020b458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 5usize, 17usize, 2143324u32);
    emu.sbr_no_count(17usize, 14usize, 16usize, 2143328u32);
    emu.sbr_no_count(14usize, 8usize, 18usize, 2143332u32);
    emu.sbr_no_count(16usize, 28usize, 30usize, 2143336u32);
    emu.sbr_no_count(29usize, 14usize, 29usize, 2143340u32);
    emu.adr_no_count(14usize, 16usize, 6usize, 2143344u32);
    emu.adr_no_count(28usize, 29usize, 6usize, 2143348u32);
    emu.sltru_no_count(30usize, 14usize, 16usize, 2143352u32);
    emu.adr_no_count(16usize, 28usize, 30usize, 2143356u32);
    emu.sbr_no_count(28usize, 0usize, 7usize, 2143360u32);
    emu.lw_no_count(8usize, 2usize, 28u32, 2143364u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2143368u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2143372u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2143376u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2143380u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2143384u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2143388u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2143396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4a4));
    } else {
        emu.pc = 2143392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4a0));
    }
}
#[inline(always)]
pub fn block_0x0020b4a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 16usize, 29usize, 2143396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4a4));
}
#[inline]
pub fn block_0x0020b4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(6usize, 6usize, 7usize, 2143400u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2143404u32);
    emu.sltru_no_count(7usize, 6usize, 28usize, 2143408u32);
    emu.sltru_no_count(28usize, 30usize, 6usize, 2143412u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2143416u32);
    emu.adr_no_count(6usize, 6usize, 28usize, 2143420u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2143424u32);
    emu.adr_no_count(17usize, 6usize, 17usize, 2143428u32);
    emu.sltru_no_count(7usize, 17usize, 6usize, 2143432u32);
    emu.adr_no_count(5usize, 6usize, 5usize, 2143436u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2143440u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2143448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4d8));
    } else {
        emu.pc = 2143444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4d4));
    }
}
#[inline(always)]
pub fn block_0x0020b4d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 6usize, 2143448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4d8));
}
#[inline]
pub fn block_0x0020b4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 6usize, 15usize, 2143452u32);
    emu.sltru_no_count(29usize, 28usize, 6usize, 2143456u32);
    emu.adr_no_count(15usize, 28usize, 7usize, 2143460u32);
    emu.adr_no_count(7usize, 13usize, 29usize, 2143464u32);
    emu.sltru_no_count(28usize, 15usize, 28usize, 2143468u32);
    emu.sltru_no_count(13usize, 0usize, 7usize, 2143472u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2143476u32);
    emu.anr_no_count(29usize, 13usize, 29usize, 2143480u32);
    emu.adr_no_count(13usize, 7usize, 28usize, 2143484u32);
    emu.sltru_no_count(7usize, 13usize, 7usize, 2143488u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2143492u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2143496u32);
    emu.sltru_no_count(28usize, 7usize, 29usize, 2143500u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2143504u32);
    emu.sltru_no_count(7usize, 11usize, 7usize, 2143508u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2143512u32);
    emu.adr_no_count(12usize, 12usize, 7usize, 2143516u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2143524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b524));
    } else {
        emu.pc = 2143520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b520));
    }
}
#[inline(always)]
pub fn block_0x0020b520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 28usize, 2143524u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143524u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b524));
}
#[inline]
pub fn block_0x0020b524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(28usize, 6usize, 1u32, 2143528u32);
    emu.sbr_no_count(14usize, 14usize, 6usize, 2143532u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2143536u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2143540u32)?;
    emu.sw_no_count(5usize, 10usize, 4u32, 2143544u32)?;
    emu.sw_no_count(15usize, 10usize, 8u32, 2143548u32)?;
    emu.sw_no_count(13usize, 10usize, 12u32, 2143552u32)?;
    emu.sltru_no_count(13usize, 14usize, 28usize, 2143556u32);
    emu.adr_no_count(7usize, 14usize, 7usize, 2143560u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2143564u32);
    emu.sltru_no_count(14usize, 7usize, 14usize, 2143568u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2143572u32);
    emu.sw_no_count(11usize, 10usize, 16u32, 2143576u32)?;
    emu.sw_no_count(12usize, 10usize, 20u32, 2143580u32)?;
    emu.sw_no_count(7usize, 10usize, 24u32, 2143584u32)?;
    emu.sw_no_count(13usize, 10usize, 28u32, 2143588u32)?;
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143592u32;
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
pub fn block_0x0020b568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 68u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2143596u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2143600u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2143604u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2143608u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2143612u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2143616u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2143620u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2143624u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2143628u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2143632u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2143636u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2143640u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2143644u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2143648u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2143652u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2143656u32)?;
    emu.lw_no_count(9usize, 11usize, 4u32, 2143660u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2143664u32)?;
    emu.lw_no_count(29usize, 12usize, 4u32, 2143668u32)?;
    emu.lw_no_count(7usize, 12usize, 8u32, 2143672u32)?;
    emu.lw_no_count(28usize, 12usize, 12u32, 2143676u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2143680u32)?;
    emu.lw_no_count(5usize, 12usize, 20u32, 2143684u32)?;
    emu.mulhu_no_count(10usize, 15usize, 14usize, 2143688u32);
    emu.mul_no_count(13usize, 29usize, 14usize, 2143692u32);
    emu.mulhu_no_count(16usize, 29usize, 14usize, 2143696u32);
    emu.mul_no_count(6usize, 15usize, 9usize, 2143700u32);
    emu.mulhu_no_count(30usize, 15usize, 9usize, 2143704u32);
    emu.mul_no_count(31usize, 29usize, 9usize, 2143708u32);
    emu.mulhu_no_count(8usize, 7usize, 14usize, 2143712u32);
    emu.mul_no_count(18usize, 28usize, 14usize, 2143716u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2143720u32);
    emu.sltru_no_count(13usize, 10usize, 13usize, 2143724u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2143728u32);
    emu.mulhu_no_count(16usize, 28usize, 14usize, 2143732u32);
    emu.adr_no_count(8usize, 18usize, 8usize, 2143736u32);
    emu.sltru_no_count(18usize, 8usize, 18usize, 2143740u32);
    emu.adr_no_count(16usize, 16usize, 18usize, 2143744u32);
    emu.mul_no_count(18usize, 7usize, 9usize, 2143748u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2143752u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2143756u32)?;
    emu.sltru_no_count(10usize, 10usize, 6usize, 2143760u32);
    emu.adr_no_count(30usize, 30usize, 10usize, 2143764u32);
    emu.mulhu_no_count(6usize, 7usize, 9usize, 2143768u32);
    emu.adr_no_count(10usize, 18usize, 8usize, 2143772u32);
    emu.sltru_no_count(8usize, 10usize, 18usize, 2143776u32);
    emu.adr_no_count(6usize, 6usize, 8usize, 2143780u32);
    emu.mulhu_no_count(8usize, 29usize, 9usize, 2143784u32);
    emu.adr_no_count(30usize, 13usize, 30usize, 2143788u32);
    emu.sltru_no_count(13usize, 30usize, 13usize, 2143792u32);
    emu.adr_no_count(13usize, 8usize, 13usize, 2143796u32);
    emu.mulhu_no_count(8usize, 28usize, 9usize, 2143800u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2143804u32);
    emu.sltru_no_count(16usize, 6usize, 16usize, 2143808u32);
    emu.adr_no_count(8usize, 8usize, 16usize, 2143812u32);
    emu.mul_no_count(16usize, 28usize, 9usize, 2143816u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2143820u32);
    emu.sltru_no_count(31usize, 30usize, 31usize, 2143824u32);
    emu.adr_no_count(13usize, 13usize, 31usize, 2143828u32);
    emu.mul_no_count(31usize, 7usize, 14usize, 2143832u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2143836u32);
    emu.sltru_no_count(18usize, 6usize, 16usize, 2143840u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2143844u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2143848u32);
    emu.sltru_no_count(16usize, 30usize, 31usize, 2143852u32);
    emu.adr_no_count(21usize, 13usize, 16usize, 2143856u32);
    emu.adr_no_count(31usize, 8usize, 18usize, 2143860u32);
    emu.add_memory_rw_events(67usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2143868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b67c));
    } else {
        emu.pc = 2143864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b678));
    }
}
#[inline(always)]
pub fn block_0x0020b678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 21usize, 10usize, 2143868u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b67c));
}
#[inline(never)]
pub fn block_0x0020b67c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 12usize, 24u32, 2143872u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2143876u32)?;
    emu.lw_no_count(13usize, 12usize, 28u32, 2143880u32)?;
    emu.adr_no_count(16usize, 6usize, 16usize, 2143884u32);
    emu.mulhu_no_count(10usize, 17usize, 14usize, 2143888u32);
    emu.mul_no_count(8usize, 5usize, 14usize, 2143892u32);
    emu.mulhu_no_count(18usize, 5usize, 14usize, 2143896u32);
    emu.mul_no_count(19usize, 17usize, 9usize, 2143900u32);
    emu.sltru_no_count(6usize, 16usize, 6usize, 2143904u32);
    emu.adr_no_count(31usize, 31usize, 6usize, 2143908u32);
    emu.mulhu_no_count(20usize, 17usize, 9usize, 2143912u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2143916u32);
    emu.sltru_no_count(6usize, 10usize, 8usize, 2143920u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2143924u32);
    emu.mul_no_count(8usize, 5usize, 9usize, 2143928u32);
    emu.adr_no_count(6usize, 19usize, 10usize, 2143932u32);
    emu.sltru_no_count(10usize, 6usize, 19usize, 2143936u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2143940u32);
    emu.mulhu_no_count(19usize, 5usize, 9usize, 2143944u32);
    emu.adr_no_count(10usize, 18usize, 10usize, 2143948u32);
    emu.sltru_no_count(18usize, 10usize, 18usize, 2143952u32);
    emu.adr_no_count(18usize, 19usize, 18usize, 2143956u32);
    emu.mul_no_count(19usize, 17usize, 14usize, 2143960u32);
    emu.adr_no_count(22usize, 19usize, 16usize, 2143964u32);
    emu.sltru_no_count(16usize, 22usize, 19usize, 2143968u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2143972u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2143976u32);
    emu.sltru_no_count(8usize, 10usize, 8usize, 2143980u32);
    emu.adr_no_count(20usize, 31usize, 16usize, 2143984u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2143988u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2143996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6fc));
    } else {
        emu.pc = 2143992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6f8));
    }
}
#[inline(always)]
pub fn block_0x0020b6f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 6usize, 2143996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6fc));
}
#[inline]
pub fn block_0x0020b6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 11usize, 8u32, 2144000u32)?;
    emu.lw_no_count(31usize, 11usize, 12u32, 2144004u32)?;
    emu.adr_no_count(16usize, 10usize, 16usize, 2144008u32);
    emu.lw_no_count(12usize, 2usize, 36u32, 2144012u32)?;
    emu.mulhu_no_count(6usize, 12usize, 14usize, 2144016u32);
    emu.mul_no_count(19usize, 13usize, 14usize, 2144020u32);
    emu.mulhu_no_count(25usize, 13usize, 14usize, 2144024u32);
    emu.mul_no_count(24usize, 12usize, 9usize, 2144028u32);
    emu.sltru_no_count(10usize, 16usize, 10usize, 2144032u32);
    emu.adr_no_count(18usize, 18usize, 10usize, 2144036u32);
    emu.mulhu_no_count(26usize, 12usize, 9usize, 2144040u32);
    emu.adr_no_count(10usize, 19usize, 6usize, 2144044u32);
    emu.sltru_no_count(6usize, 10usize, 19usize, 2144048u32);
    emu.adr_no_count(25usize, 25usize, 6usize, 2144052u32);
    emu.mul_no_count(6usize, 12usize, 14usize, 2144056u32);
    emu.adr_no_count(19usize, 6usize, 16usize, 2144060u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2144064u32);
    emu.sltru_no_count(23usize, 19usize, 6usize, 2144068u32);
    emu.sltru_no_count(16usize, 10usize, 24usize, 2144072u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2144076u32);
    emu.adr_no_count(18usize, 18usize, 23usize, 2144080u32);
    emu.adr_no_count(26usize, 26usize, 16usize, 2144084u32);
    emu.add_memory_rw_events(22usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2144092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b75c));
    } else {
        emu.pc = 2144088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b758));
    }
}
#[inline(always)]
pub fn block_0x0020b758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(23usize, 18usize, 10usize, 2144092u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b75c));
}
#[inline(never)]
pub fn block_0x0020b75c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 25usize, 26usize, 2144096u32);
    emu.mul_no_count(24usize, 13usize, 9usize, 2144100u32);
    emu.mulhu_no_count(10usize, 15usize, 8usize, 2144104u32);
    emu.mul_no_count(16usize, 29usize, 8usize, 2144108u32);
    emu.mulhu_no_count(6usize, 29usize, 8usize, 2144112u32);
    emu.mul_no_count(27usize, 15usize, 31usize, 2144116u32);
    emu.mulhu_no_count(1usize, 15usize, 31usize, 2144120u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2144124u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2144128u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2144132u32);
    emu.mul_no_count(6usize, 29usize, 31usize, 2144136u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2144140u32);
    emu.sltru_no_count(27usize, 10usize, 27usize, 2144144u32);
    emu.adr_no_count(27usize, 1usize, 27usize, 2144148u32);
    emu.mulhu_no_count(1usize, 29usize, 31usize, 2144152u32);
    emu.adr_no_count(27usize, 16usize, 27usize, 2144156u32);
    emu.sltru_no_count(16usize, 27usize, 16usize, 2144160u32);
    emu.adr_no_count(1usize, 1usize, 16usize, 2144164u32);
    emu.mul_no_count(16usize, 15usize, 8usize, 2144168u32);
    emu.adr_no_count(16usize, 30usize, 16usize, 2144172u32);
    emu.sw_no_count(16usize, 2usize, 32u32, 2144176u32)?;
    emu.sltru_no_count(16usize, 16usize, 30usize, 2144180u32);
    emu.adr_no_count(30usize, 10usize, 16usize, 2144184u32);
    emu.adr_no_count(10usize, 6usize, 27usize, 2144188u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2144192u32);
    emu.adr_no_count(30usize, 21usize, 30usize, 2144196u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2144200u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2144208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7d0));
    } else {
        emu.pc = 2144204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7cc));
    }
}
#[inline(always)]
pub fn block_0x0020b7cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 30usize, 21usize, 2144208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b7d0));
}
#[inline(never)]
pub fn block_0x0020b7d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(30usize, 2usize, 20u32, 2144212u32)?;
    emu.adr_no_count(21usize, 24usize, 26usize, 2144216u32);
    emu.sltru_no_count(26usize, 26usize, 25usize, 2144220u32);
    emu.mulhu_no_count(9usize, 13usize, 9usize, 2144224u32);
    emu.adr_no_count(25usize, 10usize, 16usize, 2144228u32);
    emu.mulhu_no_count(30usize, 7usize, 8usize, 2144232u32);
    emu.mul_no_count(27usize, 28usize, 8usize, 2144236u32);
    emu.mulhu_no_count(1usize, 28usize, 8usize, 2144240u32);
    emu.mul_no_count(12usize, 7usize, 31usize, 2144244u32);
    emu.sltru_no_count(16usize, 25usize, 10usize, 2144248u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2144252u32);
    emu.mulhu_no_count(10usize, 7usize, 31usize, 2144256u32);
    emu.adr_no_count(30usize, 27usize, 30usize, 2144260u32);
    emu.sltru_no_count(6usize, 30usize, 27usize, 2144264u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2144268u32);
    emu.mul_no_count(27usize, 28usize, 31usize, 2144272u32);
    emu.adr_no_count(30usize, 12usize, 30usize, 2144276u32);
    emu.sltru_no_count(12usize, 30usize, 12usize, 2144280u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2144284u32);
    emu.mulhu_no_count(12usize, 28usize, 31usize, 2144288u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2144292u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2144296u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2144300u32);
    emu.mul_no_count(1usize, 7usize, 8usize, 2144304u32);
    emu.adr_no_count(1usize, 22usize, 1usize, 2144308u32);
    emu.sltru_no_count(22usize, 1usize, 22usize, 2144312u32);
    emu.adr_no_count(6usize, 30usize, 22usize, 2144316u32);
    emu.adr_no_count(30usize, 27usize, 10usize, 2144320u32);
    emu.sltru_no_count(10usize, 30usize, 27usize, 2144324u32);
    emu.adr_no_count(27usize, 20usize, 6usize, 2144328u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2144332u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2144340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b854));
    } else {
        emu.pc = 2144336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b850));
    }
}
#[inline(always)]
pub fn block_0x0020b850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 27usize, 20usize, 2144340u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b854));
}
#[inline]
pub fn block_0x0020b854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 21usize, 24usize, 2144344u32);
    emu.adr_no_count(24usize, 9usize, 26usize, 2144348u32);
    emu.adr_no_count(9usize, 21usize, 23usize, 2144352u32);
    emu.adr_no_count(26usize, 30usize, 22usize, 2144356u32);
    emu.adr_no_count(20usize, 27usize, 16usize, 2144360u32);
    emu.adr_no_count(22usize, 1usize, 25usize, 2144364u32);
    emu.sltru_no_count(12usize, 26usize, 30usize, 2144368u32);
    emu.sltru_no_count(16usize, 22usize, 1usize, 2144372u32);
    emu.adr_no_count(20usize, 20usize, 16usize, 2144376u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2144380u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2144388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b884));
    } else {
        emu.pc = 2144384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b880));
    }
}
#[inline(always)]
pub fn block_0x0020b880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 27usize, 2144388u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b884));
}
#[inline(never)]
pub fn block_0x0020b884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 24usize, 6usize, 2144392u32);
    emu.sltru_no_count(21usize, 9usize, 21usize, 2144396u32);
    emu.adr_no_count(24usize, 26usize, 16usize, 2144400u32);
    emu.mulhu_no_count(12usize, 17usize, 8usize, 2144404u32);
    emu.mul_no_count(16usize, 5usize, 8usize, 2144408u32);
    emu.mulhu_no_count(6usize, 5usize, 8usize, 2144412u32);
    emu.mul_no_count(30usize, 17usize, 31usize, 2144416u32);
    emu.sltru_no_count(25usize, 24usize, 26usize, 2144420u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2144424u32);
    emu.mulhu_no_count(25usize, 17usize, 31usize, 2144428u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2144432u32);
    emu.sltru_no_count(16usize, 12usize, 16usize, 2144436u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2144440u32);
    emu.mul_no_count(6usize, 5usize, 31usize, 2144444u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2144448u32);
    emu.sltru_no_count(30usize, 12usize, 30usize, 2144452u32);
    emu.adr_no_count(30usize, 25usize, 30usize, 2144456u32);
    emu.mulhu_no_count(25usize, 5usize, 31usize, 2144460u32);
    emu.adr_no_count(26usize, 16usize, 30usize, 2144464u32);
    emu.sltru_no_count(16usize, 26usize, 16usize, 2144468u32);
    emu.adr_no_count(27usize, 25usize, 16usize, 2144472u32);
    emu.mul_no_count(25usize, 17usize, 8usize, 2144476u32);
    emu.adr_no_count(25usize, 19usize, 25usize, 2144480u32);
    emu.sltru_no_count(30usize, 25usize, 19usize, 2144484u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2144488u32);
    emu.adr_no_count(16usize, 6usize, 26usize, 2144492u32);
    emu.sltru_no_count(19usize, 16usize, 6usize, 2144496u32);
    emu.adr_no_count(6usize, 18usize, 12usize, 2144500u32);
    emu.adr_no_count(19usize, 27usize, 19usize, 2144504u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2144512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b900));
    } else {
        emu.pc = 2144508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8fc));
    }
}
#[inline(always)]
pub fn block_0x0020b8fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 6usize, 18usize, 2144512u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b900));
}
#[inline]
pub fn block_0x0020b900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 23usize, 21usize, 2144516u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2144520u32);
    emu.adr_no_count(23usize, 6usize, 10usize, 2144524u32);
    emu.adr_no_count(24usize, 25usize, 24usize, 2144528u32);
    emu.sltru_no_count(12usize, 30usize, 16usize, 2144532u32);
    emu.sltru_no_count(10usize, 24usize, 25usize, 2144536u32);
    emu.adr_no_count(23usize, 23usize, 10usize, 2144540u32);
    emu.adr_no_count(19usize, 19usize, 12usize, 2144544u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2144552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b928));
    } else {
        emu.pc = 2144548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b924));
    }
}
#[inline(always)]
pub fn block_0x0020b924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 23usize, 6usize, 2144552u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b928));
}
#[inline]
pub fn block_0x0020b928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2144556u32);
    emu.lw_no_count(26usize, 2usize, 36u32, 2144560u32)?;
    emu.mulhu_no_count(12usize, 26usize, 8usize, 2144564u32);
    emu.mul_no_count(16usize, 13usize, 8usize, 2144568u32);
    emu.mulhu_no_count(6usize, 13usize, 8usize, 2144572u32);
    emu.sltru_no_count(30usize, 10usize, 30usize, 2144576u32);
    emu.adr_no_count(19usize, 19usize, 30usize, 2144580u32);
    emu.mul_no_count(18usize, 26usize, 31usize, 2144584u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2144588u32);
    emu.sltru_no_count(30usize, 12usize, 16usize, 2144592u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2144596u32);
    emu.mulhu_no_count(25usize, 26usize, 31usize, 2144600u32);
    emu.mul_no_count(16usize, 26usize, 8usize, 2144604u32);
    emu.adr_no_count(16usize, 9usize, 16usize, 2144608u32);
    emu.adr_no_count(12usize, 18usize, 12usize, 2144612u32);
    emu.sltru_no_count(27usize, 16usize, 9usize, 2144616u32);
    emu.sltru_no_count(8usize, 12usize, 18usize, 2144620u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2144624u32);
    emu.adr_no_count(6usize, 12usize, 27usize, 2144628u32);
    emu.adr_no_count(8usize, 25usize, 8usize, 2144632u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2144640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b980));
    } else {
        emu.pc = 2144636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b97c));
    }
}
#[inline(always)]
pub fn block_0x0020b97c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(27usize, 6usize, 21usize, 2144640u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b980));
}
#[inline]
pub fn block_0x0020b980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 30usize, 8usize, 2144644u32);
    emu.lw_no_count(9usize, 11usize, 16u32, 2144648u32)?;
    emu.lw_no_count(18usize, 11usize, 20u32, 2144652u32)?;
    emu.adr_no_count(21usize, 16usize, 10usize, 2144656u32);
    emu.sltru_no_count(25usize, 21usize, 16usize, 2144660u32);
    emu.adr_no_count(19usize, 6usize, 19usize, 2144664u32);
    emu.adr_no_count(19usize, 19usize, 25usize, 2144668u32);
    emu.mul_no_count(1usize, 13usize, 31usize, 2144672u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2144680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a8));
    } else {
        emu.pc = 2144676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a4));
    }
}
#[inline(always)]
pub fn block_0x0020b9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 6usize, 2144680u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b9a8));
}
#[inline(never)]
pub fn block_0x0020b9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 1usize, 8usize, 2144684u32);
    emu.sltru_no_count(30usize, 8usize, 30usize, 2144688u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2144692u32)?;
    emu.mulhu_no_count(10usize, 13usize, 31usize, 2144696u32);
    emu.mulhu_no_count(12usize, 15usize, 9usize, 2144700u32);
    emu.mul_no_count(16usize, 29usize, 9usize, 2144704u32);
    emu.mulhu_no_count(6usize, 29usize, 9usize, 2144708u32);
    emu.mul_no_count(31usize, 15usize, 18usize, 2144712u32);
    emu.mulhu_no_count(8usize, 15usize, 18usize, 2144716u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2144720u32);
    emu.sltru_no_count(16usize, 12usize, 16usize, 2144724u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2144728u32);
    emu.mul_no_count(13usize, 29usize, 18usize, 2144732u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2144736u32);
    emu.sltru_no_count(6usize, 12usize, 31usize, 2144740u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2144744u32);
    emu.mulhu_no_count(31usize, 29usize, 18usize, 2144748u32);
    emu.adr_no_count(8usize, 16usize, 6usize, 2144752u32);
    emu.sltru_no_count(16usize, 8usize, 16usize, 2144756u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2144760u32);
    emu.mul_no_count(6usize, 15usize, 9usize, 2144764u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2144768u32);
    emu.sw_no_count(6usize, 2usize, 12u32, 2144772u32)?;
    emu.sltru_no_count(6usize, 6usize, 22usize, 2144776u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2144780u32);
    emu.adr_no_count(8usize, 13usize, 8usize, 2144784u32);
    emu.sltru_no_count(22usize, 8usize, 13usize, 2144788u32);
    emu.adr_no_count(12usize, 20usize, 12usize, 2144792u32);
    emu.adr_no_count(22usize, 16usize, 22usize, 2144796u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2144804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba24));
    } else {
        emu.pc = 2144800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba20));
    }
}
#[inline(always)]
pub fn block_0x0020ba20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 12usize, 20usize, 2144804u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144804u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ba24));
}
#[inline(never)]
pub fn block_0x0020ba24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 16u32, 2144808u32)?;
    emu.sltru_no_count(20usize, 26usize, 1usize, 2144812u32);
    emu.adr_no_count(1usize, 10usize, 30usize, 2144816u32);
    emu.adr_no_count(27usize, 26usize, 27usize, 2144820u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2144824u32);
    emu.mulhu_no_count(12usize, 7usize, 9usize, 2144828u32);
    emu.mul_no_count(13usize, 28usize, 9usize, 2144832u32);
    emu.mulhu_no_count(16usize, 28usize, 9usize, 2144836u32);
    emu.mul_no_count(30usize, 7usize, 18usize, 2144840u32);
    emu.sltru_no_count(10usize, 6usize, 8usize, 2144844u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2144848u32);
    emu.mulhu_no_count(8usize, 7usize, 18usize, 2144852u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2144856u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2144860u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2144864u32);
    emu.mul_no_count(31usize, 28usize, 18usize, 2144868u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2144872u32);
    emu.sltru_no_count(16usize, 12usize, 30usize, 2144876u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2144880u32);
    emu.mulhu_no_count(30usize, 28usize, 18usize, 2144884u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2144888u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2144892u32);
    emu.adr_no_count(13usize, 30usize, 13usize, 2144896u32);
    emu.mul_no_count(22usize, 7usize, 9usize, 2144900u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2144904u32);
    emu.sltru_no_count(30usize, 22usize, 24usize, 2144908u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2144912u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2144916u32);
    emu.sltru_no_count(24usize, 16usize, 31usize, 2144920u32);
    emu.adr_no_count(8usize, 23usize, 12usize, 2144924u32);
    emu.adr_no_count(24usize, 13usize, 24usize, 2144928u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2144936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020baa8));
    } else {
        emu.pc = 2144932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020baa4));
    }
}
#[inline(always)]
pub fn block_0x0020baa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 8usize, 23usize, 2144936u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020baa8));
}
#[inline]
pub fn block_0x0020baa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 1usize, 20usize, 2144940u32);
    emu.sltru_no_count(26usize, 27usize, 26usize, 2144944u32);
    emu.adr_no_count(25usize, 27usize, 25usize, 2144948u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2144952u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2144956u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2144960u32);
    emu.sltru_no_count(12usize, 30usize, 16usize, 2144964u32);
    emu.sltru_no_count(16usize, 6usize, 22usize, 2144968u32);
    emu.adr_no_count(20usize, 10usize, 16usize, 2144972u32);
    emu.adr_no_count(10usize, 24usize, 12usize, 2144976u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2144984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bad8));
    } else {
        emu.pc = 2144980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bad4));
    }
}
#[inline(always)]
pub fn block_0x0020bad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 8usize, 2144984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bad8));
}
#[inline(never)]
pub fn block_0x0020bad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 26usize, 2144988u32);
    emu.sltru_no_count(26usize, 25usize, 27usize, 2144992u32);
    emu.adr_no_count(24usize, 30usize, 16usize, 2144996u32);
    emu.mulhu_no_count(12usize, 17usize, 9usize, 2145000u32);
    emu.mul_no_count(13usize, 5usize, 9usize, 2145004u32);
    emu.mulhu_no_count(16usize, 5usize, 9usize, 2145008u32);
    emu.mul_no_count(31usize, 17usize, 18usize, 2145012u32);
    emu.sltru_no_count(30usize, 24usize, 30usize, 2145016u32);
    emu.adr_no_count(10usize, 10usize, 30usize, 2145020u32);
    emu.mulhu_no_count(30usize, 17usize, 18usize, 2145024u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2145028u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2145032u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2145036u32);
    emu.mul_no_count(8usize, 5usize, 18usize, 2145040u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2145044u32);
    emu.sltru_no_count(16usize, 12usize, 31usize, 2145048u32);
    emu.adr_no_count(16usize, 30usize, 16usize, 2145052u32);
    emu.mulhu_no_count(30usize, 5usize, 18usize, 2145056u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2145060u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2145064u32);
    emu.adr_no_count(13usize, 30usize, 13usize, 2145068u32);
    emu.mul_no_count(22usize, 17usize, 9usize, 2145072u32);
    emu.adr_no_count(22usize, 21usize, 22usize, 2145076u32);
    emu.sltru_no_count(30usize, 22usize, 21usize, 2145080u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2145084u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2145088u32);
    emu.sltru_no_count(27usize, 16usize, 8usize, 2145092u32);
    emu.adr_no_count(8usize, 19usize, 12usize, 2145096u32);
    emu.adr_no_count(27usize, 13usize, 27usize, 2145100u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2145108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb54));
    } else {
        emu.pc = 2145104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb50));
    }
}
#[inline(always)]
pub fn block_0x0020bb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 8usize, 19usize, 2145108u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb54));
}
#[inline]
pub fn block_0x0020bb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 26usize, 2145112u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2145116u32);
    emu.adr_no_count(21usize, 8usize, 10usize, 2145120u32);
    emu.adr_no_count(24usize, 22usize, 24usize, 2145124u32);
    emu.sltru_no_count(19usize, 30usize, 16usize, 2145128u32);
    emu.sltru_no_count(10usize, 24usize, 22usize, 2145132u32);
    emu.adr_no_count(21usize, 21usize, 10usize, 2145136u32);
    emu.adr_no_count(19usize, 27usize, 19usize, 2145140u32);
    emu.lw_no_count(13usize, 2usize, 40u32, 2145144u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2145152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb80));
    } else {
        emu.pc = 2145148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb7c));
    }
}
#[inline(always)]
pub fn block_0x0020bb7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 8usize, 2145152u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb80));
}
#[inline]
pub fn block_0x0020bb80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2145156u32);
    emu.lw_no_count(8usize, 2usize, 36u32, 2145160u32)?;
    emu.mulhu_no_count(12usize, 8usize, 9usize, 2145164u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2145168u32);
    emu.mul_no_count(13usize, 13usize, 9usize, 2145172u32);
    emu.mulhu_no_count(16usize, 16usize, 9usize, 2145176u32);
    emu.sltru_no_count(30usize, 10usize, 30usize, 2145180u32);
    emu.adr_no_count(19usize, 19usize, 30usize, 2145184u32);
    emu.mul_no_count(31usize, 8usize, 18usize, 2145188u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2145192u32);
    emu.sltru_no_count(30usize, 12usize, 13usize, 2145196u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2145200u32);
    emu.mulhu_no_count(22usize, 8usize, 18usize, 2145204u32);
    emu.mul_no_count(16usize, 8usize, 9usize, 2145208u32);
    emu.adr_no_count(16usize, 25usize, 16usize, 2145212u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2145216u32);
    emu.sltru_no_count(27usize, 16usize, 25usize, 2145220u32);
    emu.sltru_no_count(13usize, 12usize, 31usize, 2145224u32);
    emu.adr_no_count(12usize, 23usize, 12usize, 2145228u32);
    emu.adr_no_count(8usize, 12usize, 27usize, 2145232u32);
    emu.adr_no_count(22usize, 22usize, 13usize, 2145236u32);
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2145244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbdc));
    } else {
        emu.pc = 2145240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbd8));
    }
}
#[inline(always)]
pub fn block_0x0020bbd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(27usize, 8usize, 23usize, 2145244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bbdc));
}
#[inline]
pub fn block_0x0020bbdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 30usize, 22usize, 2145248u32);
    emu.lw_no_count(9usize, 11usize, 24u32, 2145252u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2145256u32)?;
    emu.adr_no_count(23usize, 16usize, 10usize, 2145260u32);
    emu.sltru_no_count(25usize, 23usize, 16usize, 2145264u32);
    emu.adr_no_count(19usize, 8usize, 19usize, 2145268u32);
    emu.adr_no_count(19usize, 19usize, 25usize, 2145272u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2145276u32)?;
    emu.mul_no_count(1usize, 10usize, 18usize, 2145280u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2145288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc08));
    } else {
        emu.pc = 2145284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc04));
    }
}
#[inline(always)]
pub fn block_0x0020bc04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 8usize, 2145288u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc08));
}
#[inline(never)]
pub fn block_0x0020bc08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 1usize, 22usize, 2145292u32);
    emu.sltru_no_count(30usize, 22usize, 30usize, 2145296u32);
    emu.mulhu_no_count(10usize, 10usize, 18usize, 2145300u32);
    emu.mulhu_no_count(12usize, 15usize, 9usize, 2145304u32);
    emu.mul_no_count(13usize, 29usize, 9usize, 2145308u32);
    emu.mulhu_no_count(16usize, 29usize, 9usize, 2145312u32);
    emu.mul_no_count(31usize, 15usize, 11usize, 2145316u32);
    emu.mulhu_no_count(8usize, 15usize, 11usize, 2145320u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2145324u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2145328u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2145332u32);
    emu.mul_no_count(16usize, 29usize, 11usize, 2145336u32);
    emu.mulhu_no_count(29usize, 29usize, 11usize, 2145340u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2145344u32);
    emu.sltru_no_count(31usize, 12usize, 31usize, 2145348u32);
    emu.adr_no_count(31usize, 8usize, 31usize, 2145352u32);
    emu.mul_no_count(18usize, 15usize, 9usize, 2145356u32);
    emu.adr_no_count(18usize, 6usize, 18usize, 2145360u32);
    emu.sltru_no_count(8usize, 18usize, 6usize, 2145364u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2145368u32);
    emu.adr_no_count(31usize, 13usize, 31usize, 2145372u32);
    emu.sltru_no_count(13usize, 31usize, 13usize, 2145376u32);
    emu.adr_no_count(22usize, 16usize, 31usize, 2145380u32);
    emu.sltru_no_count(6usize, 22usize, 16usize, 2145384u32);
    emu.adr_no_count(13usize, 29usize, 13usize, 2145388u32);
    emu.adr_no_count(29usize, 20usize, 12usize, 2145392u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2145396u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2145404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc7c));
    } else {
        emu.pc = 2145400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc78));
    }
}
#[inline(always)]
pub fn block_0x0020bc78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 29usize, 20usize, 2145404u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc7c));
}
#[inline(never)]
pub fn block_0x0020bc7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 26usize, 1usize, 2145408u32);
    emu.adr_no_count(1usize, 10usize, 30usize, 2145412u32);
    emu.adr_no_count(27usize, 26usize, 27usize, 2145416u32);
    emu.adr_no_count(8usize, 22usize, 8usize, 2145420u32);
    emu.mulhu_no_count(12usize, 7usize, 9usize, 2145424u32);
    emu.mul_no_count(13usize, 28usize, 9usize, 2145428u32);
    emu.mulhu_no_count(16usize, 28usize, 9usize, 2145432u32);
    emu.mul_no_count(31usize, 7usize, 11usize, 2145436u32);
    emu.sltru_no_count(10usize, 8usize, 22usize, 2145440u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2145444u32);
    emu.mulhu_no_count(6usize, 7usize, 11usize, 2145448u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2145452u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2145456u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2145460u32);
    emu.mul_no_count(16usize, 28usize, 11usize, 2145464u32);
    emu.mulhu_no_count(22usize, 28usize, 11usize, 2145468u32);
    emu.mul_no_count(28usize, 7usize, 9usize, 2145472u32);
    emu.adr_no_count(28usize, 24usize, 28usize, 2145476u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2145480u32);
    emu.sltru_no_count(30usize, 28usize, 24usize, 2145484u32);
    emu.sltru_no_count(7usize, 12usize, 31usize, 2145488u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2145492u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2145496u32);
    emu.adr_no_count(7usize, 13usize, 6usize, 2145500u32);
    emu.adr_no_count(6usize, 16usize, 7usize, 2145504u32);
    emu.sltru_no_count(13usize, 7usize, 13usize, 2145508u32);
    emu.sltru_no_count(16usize, 6usize, 16usize, 2145512u32);
    emu.adr_no_count(13usize, 22usize, 13usize, 2145516u32);
    emu.adr_no_count(24usize, 21usize, 12usize, 2145520u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2145524u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2145532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcfc));
    } else {
        emu.pc = 2145528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcf8));
    }
}
#[inline(always)]
pub fn block_0x0020bcf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 24usize, 21usize, 2145532u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bcfc));
}
#[inline]
pub fn block_0x0020bcfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 1usize, 20usize, 2145536u32);
    emu.sltru_no_count(26usize, 27usize, 26usize, 2145540u32);
    emu.adr_no_count(7usize, 27usize, 25usize, 2145544u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2145548u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2145552u32);
    emu.adr_no_count(22usize, 28usize, 8usize, 2145556u32);
    emu.sltru_no_count(12usize, 30usize, 6usize, 2145560u32);
    emu.sltru_no_count(6usize, 22usize, 28usize, 2145564u32);
    emu.adr_no_count(21usize, 10usize, 6usize, 2145568u32);
    emu.adr_no_count(10usize, 16usize, 12usize, 2145572u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2145580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd2c));
    } else {
        emu.pc = 2145576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd28));
    }
}
#[inline(always)]
pub fn block_0x0020bd28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 21usize, 24usize, 2145580u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bd2c));
}
#[inline(never)]
pub fn block_0x0020bd2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 20usize, 26usize, 2145584u32);
    emu.sltru_no_count(24usize, 7usize, 27usize, 2145588u32);
    emu.adr_no_count(20usize, 30usize, 6usize, 2145592u32);
    emu.mulhu_no_count(12usize, 17usize, 9usize, 2145596u32);
    emu.mul_no_count(13usize, 5usize, 9usize, 2145600u32);
    emu.mulhu_no_count(16usize, 5usize, 9usize, 2145604u32);
    emu.mul_no_count(6usize, 17usize, 11usize, 2145608u32);
    emu.sltru_no_count(30usize, 20usize, 30usize, 2145612u32);
    emu.adr_no_count(10usize, 10usize, 30usize, 2145616u32);
    emu.mulhu_no_count(31usize, 17usize, 11usize, 2145620u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2145624u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2145628u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2145632u32);
    emu.mul_no_count(8usize, 5usize, 11usize, 2145636u32);
    emu.mulhu_no_count(25usize, 5usize, 11usize, 2145640u32);
    emu.mul_no_count(5usize, 17usize, 9usize, 2145644u32);
    emu.adr_no_count(5usize, 23usize, 5usize, 2145648u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2145652u32);
    emu.sltru_no_count(30usize, 5usize, 23usize, 2145656u32);
    emu.sltru_no_count(16usize, 12usize, 6usize, 2145660u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2145664u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2145668u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2145672u32);
    emu.adr_no_count(16usize, 8usize, 17usize, 2145676u32);
    emu.sltru_no_count(13usize, 17usize, 13usize, 2145680u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2145684u32);
    emu.adr_no_count(13usize, 25usize, 13usize, 2145688u32);
    emu.adr_no_count(6usize, 19usize, 12usize, 2145692u32);
    emu.adr_no_count(8usize, 13usize, 8usize, 2145696u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2145704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bda8));
    } else {
        emu.pc = 2145700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bda4));
    }
}
#[inline(always)]
pub fn block_0x0020bda4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 6usize, 19usize, 2145704u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bda8));
}
#[inline]
pub fn block_0x0020bda8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 28usize, 24usize, 2145708u32);
    emu.adr_no_count(28usize, 16usize, 30usize, 2145712u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2145716u32);
    emu.adr_no_count(20usize, 5usize, 20usize, 2145720u32);
    emu.sltru_no_count(12usize, 28usize, 16usize, 2145724u32);
    emu.sltru_no_count(5usize, 20usize, 5usize, 2145728u32);
    emu.adr_no_count(19usize, 10usize, 5usize, 2145732u32);
    emu.adr_no_count(10usize, 8usize, 12usize, 2145736u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2145740u32)?;
    emu.lw_no_count(31usize, 2usize, 24u32, 2145744u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2145752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdd8));
    } else {
        emu.pc = 2145748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdd4));
    }
}
#[inline(always)]
pub fn block_0x0020bdd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 19usize, 6usize, 2145752u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145752u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bdd8));
}
#[inline]
pub fn block_0x0020bdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 28usize, 5usize, 2145756u32);
    emu.lw_no_count(13usize, 2usize, 36u32, 2145760u32)?;
    emu.mulhu_no_count(25usize, 13usize, 9usize, 2145764u32);
    emu.mul_no_count(23usize, 12usize, 9usize, 2145768u32);
    emu.mul_no_count(24usize, 13usize, 11usize, 2145772u32);
    emu.mul_no_count(6usize, 13usize, 9usize, 2145776u32);
    emu.sltru_no_count(12usize, 5usize, 28usize, 2145780u32);
    emu.adr_no_count(25usize, 23usize, 25usize, 2145784u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2145788u32);
    emu.adr_no_count(26usize, 24usize, 25usize, 2145792u32);
    emu.sltru_no_count(28usize, 6usize, 7usize, 2145796u32);
    emu.adr_no_count(16usize, 17usize, 26usize, 2145800u32);
    emu.adr_no_count(16usize, 16usize, 28usize, 2145804u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2145808u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2145812u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2145820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be1c));
    } else {
        emu.pc = 2145816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be18));
    }
}
#[inline(always)]
pub fn block_0x0020be18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 16usize, 17usize, 2145820u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be1c));
}
#[inline(always)]
pub fn block_0x0020be1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 6usize, 5usize, 2145824u32);
    emu.sltru_no_count(17usize, 5usize, 6usize, 2145828u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2145832u32);
    emu.adr_no_count(7usize, 10usize, 17usize, 2145836u32);
    emu.mul_no_count(10usize, 15usize, 14usize, 2145840u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2145848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be38));
    } else {
        emu.pc = 2145844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be34));
    }
}
#[inline(always)]
pub fn block_0x0020be34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 7usize, 16usize, 2145848u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be38));
}
#[inline]
pub fn block_0x0020be38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 12usize, 10usize, 2145852u32);
    emu.sltru_no_count(12usize, 27usize, 12usize, 2145856u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2145860u32);
    emu.sltru_no_count(13usize, 12usize, 31usize, 2145864u32);
    emu.lw_no_count(14usize, 2usize, 12u32, 2145868u32)?;
    emu.adr_no_count(30usize, 14usize, 12usize, 2145872u32);
    emu.sltru_no_count(8usize, 30usize, 14usize, 2145876u32);
    emu.adr_no_count(13usize, 13usize, 8usize, 2145880u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2145884u32)?;
    emu.adr_no_count(15usize, 12usize, 13usize, 2145888u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2145896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be68));
    } else {
        emu.pc = 2145892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be64));
    }
}
#[inline(always)]
pub fn block_0x0020be64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 15usize, 12usize, 2145896u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be68));
}
#[inline(never)]
pub fn block_0x0020be68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(6usize, 31usize, 10usize, 2145900u32);
    emu.sbr_no_count(12usize, 0usize, 10usize, 2145904u32);
    emu.adi_no_count(14usize, 0usize, 4294967295u32, 2145908u32);
    emu.sltru_no_count(12usize, 6usize, 12usize, 2145912u32);
    emu.mulhu_no_count(13usize, 10usize, 14usize, 2145916u32);
    emu.sbr_no_count(13usize, 13usize, 31usize, 2145920u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2145924u32);
    emu.sbr_no_count(13usize, 0usize, 31usize, 2145928u32);
    emu.adr_no_count(16usize, 8usize, 10usize, 2145932u32);
    emu.mulhu_no_count(31usize, 31usize, 14usize, 2145936u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2145940u32);
    emu.adr_no_count(10usize, 16usize, 18usize, 2145944u32);
    emu.adr_no_count(6usize, 6usize, 8usize, 2145948u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2145952u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2145956u32);
    emu.adr_no_count(13usize, 31usize, 13usize, 2145960u32);
    emu.sltru_no_count(31usize, 0usize, 6usize, 2145964u32);
    emu.adi_no_count(31usize, 31usize, 4294967295u32, 2145968u32);
    emu.anr_no_count(31usize, 31usize, 8usize, 2145972u32);
    emu.adr_no_count(1usize, 6usize, 16usize, 2145976u32);
    emu.adr_no_count(31usize, 12usize, 31usize, 2145980u32);
    emu.sltru_no_count(12usize, 31usize, 12usize, 2145984u32);
    emu.adr_no_count(1usize, 1usize, 29usize, 2145988u32);
    emu.adr_no_count(29usize, 13usize, 12usize, 2145992u32);
    emu.add_memory_rw_events(24usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a == b {
        emu.pc = 2146000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bed0));
    } else {
        emu.pc = 2145996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020becc));
    }
}
#[inline(always)]
pub fn block_0x0020becc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 1usize, 6usize, 2146000u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bed0));
}
#[inline(always)]
pub fn block_0x0020bed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 31usize, 16usize, 2146004u32);
    emu.sltru_no_count(12usize, 16usize, 31usize, 2146008u32);
    emu.adr_no_count(18usize, 22usize, 16usize, 2146012u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2146016u32);
    emu.sltru_no_count(16usize, 18usize, 22usize, 2146020u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2146024u32);
    emu.adr_no_count(8usize, 21usize, 12usize, 2146028u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2146036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bef4));
    } else {
        emu.pc = 2146032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bef0));
    }
}
#[inline(always)]
pub fn block_0x0020bef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 8usize, 21usize, 2146036u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146036u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bef4));
}
#[inline]
pub fn block_0x0020bef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(29usize, 2usize, 32u32, 2146040u32)?;
    emu.adr_no_count(29usize, 15usize, 29usize, 2146044u32);
    emu.sltru_no_count(12usize, 29usize, 15usize, 2146048u32);
    emu.adr_no_count(12usize, 27usize, 12usize, 2146052u32);
    emu.sltru_no_count(13usize, 12usize, 27usize, 2146056u32);
    emu.adr_no_count(15usize, 10usize, 12usize, 2146060u32);
    emu.sltru_no_count(22usize, 15usize, 10usize, 2146064u32);
    emu.adr_no_count(13usize, 13usize, 22usize, 2146068u32);
    emu.adr_no_count(31usize, 1usize, 13usize, 2146072u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2146080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf20));
    } else {
        emu.pc = 2146076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf1c));
    }
}
#[inline(always)]
pub fn block_0x0020bf1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 31usize, 1usize, 2146080u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf20));
}
#[inline(never)]
pub fn block_0x0020bf20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 25usize, 23usize, 2146084u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2146088u32)?;
    emu.mulhu_no_count(10usize, 10usize, 9usize, 2146092u32);
    emu.sltru_no_count(9usize, 26usize, 24usize, 2146096u32);
    emu.lw_no_count(12usize, 2usize, 36u32, 2146100u32)?;
    emu.mulhu_no_count(13usize, 12usize, 11usize, 2146104u32);
    emu.lw_no_count(26usize, 2usize, 32u32, 2146108u32)?;
    emu.sbr_no_count(24usize, 27usize, 26usize, 2146112u32);
    emu.sbr_no_count(12usize, 0usize, 26usize, 2146116u32);
    emu.mulhu_no_count(6usize, 26usize, 14usize, 2146120u32);
    emu.sltru_no_count(12usize, 24usize, 12usize, 2146124u32);
    emu.sbr_no_count(6usize, 6usize, 27usize, 2146128u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2146132u32);
    emu.sbr_no_count(23usize, 0usize, 27usize, 2146136u32);
    emu.mulhu_no_count(25usize, 27usize, 14usize, 2146140u32);
    emu.adr_no_count(26usize, 22usize, 26usize, 2146144u32);
    emu.sltru_no_count(27usize, 26usize, 22usize, 2146148u32);
    emu.adr_no_count(6usize, 26usize, 18usize, 2146152u32);
    emu.adr_no_count(24usize, 24usize, 27usize, 2146156u32);
    emu.sltru_no_count(22usize, 6usize, 26usize, 2146160u32);
    emu.sltru_no_count(18usize, 12usize, 23usize, 2146164u32);
    emu.adr_no_count(25usize, 25usize, 18usize, 2146168u32);
    emu.sltru_no_count(18usize, 0usize, 24usize, 2146172u32);
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2146176u32);
    emu.anr_no_count(23usize, 18usize, 27usize, 2146180u32);
    emu.adr_no_count(18usize, 24usize, 22usize, 2146184u32);
    emu.adr_no_count(23usize, 12usize, 23usize, 2146188u32);
    emu.sltru_no_count(12usize, 23usize, 12usize, 2146192u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2146196u32);
    emu.adr_no_count(8usize, 25usize, 12usize, 2146200u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2146208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfa0));
    } else {
        emu.pc = 2146204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf9c));
    }
}
#[inline(always)]
pub fn block_0x0020bf9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 18usize, 24usize, 2146208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfa0));
}
#[inline]
pub fn block_0x0020bfa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 10usize, 21usize, 2146212u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2146216u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2146220u32);
    emu.adr_no_count(10usize, 16usize, 20usize, 2146224u32);
    emu.sltru_no_count(12usize, 22usize, 23usize, 2146228u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2146232u32);
    emu.adr_no_count(20usize, 10usize, 22usize, 2146236u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2146240u32);
    emu.adr_no_count(19usize, 19usize, 16usize, 2146244u32);
    emu.sltru_no_count(8usize, 20usize, 10usize, 2146248u32);
    emu.sltru_no_count(10usize, 0usize, 19usize, 2146252u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2146256u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2146260u32);
    emu.adr_no_count(9usize, 12usize, 8usize, 2146264u32);
    emu.anr_no_count(22usize, 10usize, 16usize, 2146268u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2146276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfe4));
    } else {
        emu.pc = 2146272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfe0));
    }
}
#[inline(always)]
pub fn block_0x0020bfe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 9usize, 19usize, 2146276u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfe4));
}
#[inline]
pub fn block_0x0020bfe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 21usize, 13usize, 2146280u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2146284u32)?;
    emu.mul_no_count(19usize, 10usize, 11usize, 2146288u32);
    emu.adr_no_count(16usize, 31usize, 30usize, 2146292u32);
    emu.sltru_no_count(10usize, 16usize, 31usize, 2146296u32);
    emu.adr_no_count(10usize, 29usize, 10usize, 2146300u32);
    emu.sltru_no_count(12usize, 10usize, 29usize, 2146304u32);
    emu.adr_no_count(13usize, 6usize, 10usize, 2146308u32);
    emu.adr_no_count(12usize, 18usize, 12usize, 2146312u32);
    emu.sltru_no_count(10usize, 13usize, 6usize, 2146316u32);
    emu.adr_no_count(6usize, 12usize, 10usize, 2146320u32);
    emu.adr_no_count(8usize, 22usize, 8usize, 2146324u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2146332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c01c));
    } else {
        emu.pc = 2146328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c018));
    }
}
#[inline(always)]
pub fn block_0x0020c018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 18usize, 2146332u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146332u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c01c));
}
#[inline]
pub fn block_0x0020c01c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 19usize, 23usize, 2146336u32);
    emu.sltru_no_count(21usize, 23usize, 21usize, 2146340u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2146344u32)?;
    emu.mulhu_no_count(12usize, 12usize, 11usize, 2146348u32);
    emu.sltru_no_count(11usize, 8usize, 22usize, 2146352u32);
    emu.sbr_no_count(22usize, 29usize, 30usize, 2146356u32);
    emu.sbr_no_count(18usize, 0usize, 30usize, 2146360u32);
    emu.mulhu_no_count(23usize, 30usize, 14usize, 2146364u32);
    emu.sltru_no_count(18usize, 22usize, 18usize, 2146368u32);
    emu.sbr_no_count(23usize, 23usize, 29usize, 2146372u32);
    emu.adr_no_count(18usize, 23usize, 18usize, 2146376u32);
    emu.sbr_no_count(23usize, 0usize, 29usize, 2146380u32);
    emu.mulhu_no_count(24usize, 29usize, 14usize, 2146384u32);
    emu.adr_no_count(30usize, 20usize, 30usize, 2146388u32);
    emu.sltru_no_count(29usize, 30usize, 20usize, 2146392u32);
    emu.adr_no_count(20usize, 22usize, 29usize, 2146396u32);
    emu.sltru_no_count(22usize, 18usize, 23usize, 2146400u32);
    emu.adr_no_count(20usize, 9usize, 20usize, 2146404u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2146408u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2146416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c070));
    } else {
        emu.pc = 2146412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c06c));
    }
}
#[inline(always)]
pub fn block_0x0020c06c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 20usize, 9usize, 2146416u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c070));
}
#[inline]
pub fn block_0x0020c070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(9usize, 31usize, 19usize, 2146420u32);
    emu.adr_no_count(19usize, 12usize, 21usize, 2146424u32);
    emu.adr_no_count(12usize, 31usize, 28usize, 2146428u32);
    emu.adr_no_count(21usize, 18usize, 29usize, 2146432u32);
    emu.adr_no_count(28usize, 30usize, 10usize, 2146436u32);
    emu.adr_no_count(5usize, 8usize, 5usize, 2146440u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2146444u32);
    emu.sltru_no_count(10usize, 21usize, 18usize, 2146448u32);
    emu.sltru_no_count(18usize, 28usize, 30usize, 2146452u32);
    emu.sltru_no_count(29usize, 5usize, 8usize, 2146456u32);
    emu.adr_no_count(22usize, 22usize, 10usize, 2146460u32);
    emu.adr_no_count(30usize, 20usize, 18usize, 2146464u32);
    emu.sltru_no_count(10usize, 30usize, 20usize, 2146468u32);
    emu.anr_no_count(10usize, 18usize, 10usize, 2146472u32);
    emu.adr_no_count(10usize, 21usize, 10usize, 2146476u32);
    emu.sltru_no_count(18usize, 10usize, 21usize, 2146480u32);
    emu.adr_no_count(8usize, 7usize, 29usize, 2146484u32);
    emu.adr_no_count(7usize, 22usize, 18usize, 2146488u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2146496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0c0));
    } else {
        emu.pc = 2146492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0bc));
    }
}
#[inline(always)]
pub fn block_0x0020c0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 8usize, 11usize, 2146496u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c0c0));
}
#[inline(always)]
pub fn block_0x0020c0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 9usize, 2146500u32);
    emu.sltru_no_count(11usize, 12usize, 31usize, 2146504u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2146508u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2146512u32);
    emu.sltru_no_count(9usize, 10usize, 5usize, 2146516u32);
    emu.adr_no_count(5usize, 7usize, 9usize, 2146520u32);
    emu.adr_no_count(7usize, 12usize, 17usize, 2146524u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2146532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0e4));
    } else {
        emu.pc = 2146528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0e0));
    }
}
#[inline(always)]
pub fn block_0x0020c0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(9usize, 5usize, 8usize, 2146532u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c0e4));
}
#[inline]
pub fn block_0x0020c0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 19usize, 11usize, 2146536u32);
    emu.sltru_no_count(18usize, 7usize, 12usize, 2146540u32);
    emu.adr_no_count(11usize, 6usize, 15usize, 2146544u32);
    emu.sltru_no_count(12usize, 11usize, 6usize, 2146548u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2146552u32);
    emu.sltru_no_count(17usize, 12usize, 16usize, 2146556u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2146560u32);
    emu.sltru_no_count(31usize, 12usize, 28usize, 2146564u32);
    emu.adr_no_count(17usize, 17usize, 31usize, 2146568u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2146572u32);
    emu.adr_no_count(6usize, 29usize, 9usize, 2146576u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2146584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c118));
    } else {
        emu.pc = 2146580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c114));
    }
}
#[inline(always)]
pub fn block_0x0020c114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 17usize, 30usize, 2146584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c118));
}
#[inline]
pub fn block_0x0020c118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 8usize, 18usize, 2146588u32);
    emu.sltru_no_count(28usize, 6usize, 29usize, 2146592u32);
    emu.sbr_no_count(8usize, 16usize, 15usize, 2146596u32);
    emu.sbr_no_count(29usize, 0usize, 15usize, 2146600u32);
    emu.mulhu_no_count(9usize, 15usize, 14usize, 2146604u32);
    emu.mulhu_no_count(18usize, 16usize, 14usize, 2146608u32);
    emu.sbr_no_count(14usize, 9usize, 16usize, 2146612u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2146616u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2146620u32);
    emu.sltru_no_count(9usize, 8usize, 29usize, 2146624u32);
    emu.sltru_no_count(29usize, 15usize, 10usize, 2146628u32);
    emu.adr_no_count(14usize, 14usize, 9usize, 2146632u32);
    emu.adr_no_count(10usize, 8usize, 29usize, 2146636u32);
    emu.sltru_no_count(16usize, 14usize, 16usize, 2146640u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2146644u32);
    emu.adr_no_count(16usize, 18usize, 16usize, 2146648u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2146656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c160));
    } else {
        emu.pc = 2146652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c15c));
    }
}
#[inline(always)]
pub fn block_0x0020c15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 5usize, 2146656u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c160));
}
#[inline]
pub fn block_0x0020c160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 14usize, 29usize, 2146660u32);
    emu.adr_no_count(5usize, 15usize, 31usize, 2146664u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2146668u32);
    emu.adr_no_count(30usize, 28usize, 30usize, 2146672u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2146676u32);
    emu.sltru_no_count(31usize, 5usize, 15usize, 2146680u32);
    emu.sltru_no_count(15usize, 7usize, 6usize, 2146684u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2146688u32);
    emu.adr_no_count(6usize, 10usize, 31usize, 2146692u32);
    emu.sltru_no_count(10usize, 6usize, 10usize, 2146696u32);
    emu.anr_no_count(14usize, 31usize, 10usize, 2146700u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2146704u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2146708u32);
    emu.adr_no_count(10usize, 30usize, 15usize, 2146712u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2146716u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2146724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1a4));
    } else {
        emu.pc = 2146720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1a0));
    }
}
#[inline(always)]
pub fn block_0x0020c1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 10usize, 28usize, 2146724u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c1a4));
}
#[inline(always)]
pub fn block_0x0020c1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 10usize, 16usize, 2146728u32);
    emu.adr_no_count(28usize, 7usize, 14usize, 2146732u32);
    emu.sltru_no_count(29usize, 28usize, 7usize, 2146736u32);
    emu.adr_no_count(7usize, 16usize, 29usize, 2146740u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2146748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1bc));
    } else {
        emu.pc = 2146744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1b8));
    }
}
#[inline(always)]
pub fn block_0x0020c1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 10usize, 2146748u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c1bc));
}
#[inline]
pub fn block_0x0020c1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2146752u32);
    emu.sltiu_no_count(10usize, 13usize, 1u32, 2146756u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2146760u32);
    emu.orr_no_count(10usize, 13usize, 11usize, 2146764u32);
    emu.sltiu_no_count(30usize, 10usize, 1u32, 2146768u32);
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2146772u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2146776u32);
    emu.adr_no_count(16usize, 30usize, 12usize, 2146780u32);
    emu.sltru_no_count(10usize, 16usize, 30usize, 2146784u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2146788u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2146796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1ec));
    } else {
        emu.pc = 2146792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1e8));
    }
}
#[inline(always)]
pub fn block_0x0020c1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 17usize, 30usize, 2146796u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c1ec));
}
#[inline(always)]
pub fn block_0x0020c1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2146800u32);
    emu.adi_no_count(14usize, 16usize, 1u32, 2146804u32);
    emu.sltru_no_count(31usize, 10usize, 30usize, 2146808u32);
    emu.sltiu_no_count(12usize, 14usize, 1u32, 2146812u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2146816u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2146820u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2146824u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2146836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c214));
    } else {
        emu.pc = 2146828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c20c));
    }
}
#[inline(always)]
pub fn block_0x0020c20c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 12usize, 17usize, 2146832u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146836u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146840u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c218));
}
#[inline(always)]
pub fn block_0x0020c214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 14usize, 16usize, 2146840u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c218));
}
#[inline]
pub fn block_0x0020c218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 10usize, 4294967295u32, 2146844u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2146848u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2146852u32);
    emu.sbr_no_count(10usize, 30usize, 10usize, 2146856u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2146860u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2146864u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2146868u32);
    emu.adr_no_count(17usize, 6usize, 10usize, 2146872u32);
    emu.adr_no_count(16usize, 5usize, 10usize, 2146876u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2146880u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2146884u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2146892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c24c));
    } else {
        emu.pc = 2146888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c248));
    }
}
#[inline(always)]
pub fn block_0x0020c248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2146892u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c24c));
}
#[inline]
pub fn block_0x0020c24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 10usize, 5usize, 2146896u32);
    emu.sltru_no_count(5usize, 5usize, 10usize, 2146900u32);
    emu.adr_no_count(10usize, 10usize, 5usize, 2146904u32);
    emu.sai_no_count(30usize, 10usize, 1055u32, 2146908u32);
    emu.adr_no_count(10usize, 7usize, 30usize, 2146912u32);
    emu.adr_no_count(6usize, 28usize, 30usize, 2146916u32);
    emu.sltru_no_count(5usize, 6usize, 28usize, 2146920u32);
    emu.adr_no_count(28usize, 10usize, 5usize, 2146924u32);
    emu.adr_no_count(29usize, 15usize, 29usize, 2146928u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2146936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c278));
    } else {
        emu.pc = 2146932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c274));
    }
}
#[inline(always)]
pub fn block_0x0020c274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2146936u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c278));
}
#[inline(always)]
pub fn block_0x0020c278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 29usize, 15usize, 2146940u32);
    emu.adr_no_count(7usize, 30usize, 5usize, 2146944u32);
    emu.sltru_no_count(5usize, 0usize, 6usize, 2146948u32);
    emu.sltru_no_count(15usize, 7usize, 30usize, 2146952u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2146956u32);
    emu.adr_no_count(30usize, 30usize, 15usize, 2146960u32);
    emu.adi_no_count(15usize, 6usize, 4294967295u32, 2146964u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2146976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2a0));
    } else {
        emu.pc = 2146968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c298));
    }
}
#[inline(always)]
pub fn block_0x0020c298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 5usize, 28usize, 2146972u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c2a4));
}
#[inline(always)]
pub fn block_0x0020c2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 15usize, 6usize, 2146980u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c2a4));
}
#[inline]
pub fn block_0x0020c2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 7usize, 4294967295u32, 2146984u32);
    emu.sltiu_no_count(7usize, 7usize, 1u32, 2146988u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2146992u32);
    emu.sbr_no_count(7usize, 30usize, 7usize, 2146996u32);
    emu.sltru_no_count(6usize, 6usize, 28usize, 2147000u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2147004u32);
    emu.sai_no_count(7usize, 6usize, 1055u32, 2147008u32);
    emu.adr_no_count(29usize, 7usize, 29usize, 2147012u32);
    emu.sltru_no_count(6usize, 29usize, 7usize, 2147016u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2147020u32);
    emu.adr_no_count(10usize, 10usize, 6usize, 2147024u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2d8));
    } else {
        emu.pc = 2147028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2d4));
    }
}
#[inline(always)]
pub fn block_0x0020c2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 10usize, 7usize, 2147032u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c2d8));
}
#[inline(always)]
pub fn block_0x0020c2d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 7usize, 6usize, 2147036u32);
    emu.sltru_no_count(10usize, 6usize, 7usize, 2147040u32);
    emu.adr_no_count(13usize, 6usize, 13usize, 2147044u32);
    emu.adr_no_count(7usize, 7usize, 10usize, 2147048u32);
    emu.sltru_no_count(28usize, 13usize, 6usize, 2147052u32);
    emu.adr_no_count(10usize, 7usize, 11usize, 2147056u32);
    emu.adr_no_count(10usize, 10usize, 28usize, 2147060u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2fc));
    } else {
        emu.pc = 2147064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2f8));
    }
}
#[inline(always)]
pub fn block_0x0020c2f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 10usize, 7usize, 2147068u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147068u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c2fc));
}
#[inline]
pub fn block_0x0020c2fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 6usize, 14usize, 2147072u32);
    emu.sltru_no_count(29usize, 14usize, 6usize, 2147076u32);
    emu.adr_no_count(11usize, 14usize, 28usize, 2147080u32);
    emu.adr_no_count(28usize, 12usize, 29usize, 2147084u32);
    emu.sltru_no_count(14usize, 11usize, 14usize, 2147088u32);
    emu.sltru_no_count(12usize, 0usize, 28usize, 2147092u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2147096u32);
    emu.anr_no_count(29usize, 12usize, 29usize, 2147100u32);
    emu.adr_no_count(12usize, 28usize, 14usize, 2147104u32);
    emu.sltru_no_count(28usize, 12usize, 28usize, 2147108u32);
    emu.anr_no_count(14usize, 14usize, 28usize, 2147112u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2147116u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2147120u32);
    emu.adr_no_count(16usize, 14usize, 16usize, 2147124u32);
    emu.sltru_no_count(28usize, 16usize, 14usize, 2147128u32);
    emu.adr_no_count(14usize, 29usize, 17usize, 2147132u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2147136u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2147144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c348));
    } else {
        emu.pc = 2147140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c344));
    }
}
#[inline(always)]
pub fn block_0x0020c344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 29usize, 2147144u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147144u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c348));
}
#[inline(never)]
pub fn block_0x0020c348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(17usize, 6usize, 1u32, 2147148u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2147152u32);
    emu.lw_no_count(6usize, 2usize, 28u32, 2147156u32)?;
    emu.sw_no_count(13usize, 6usize, 0u32, 2147160u32)?;
    emu.sw_no_count(10usize, 6usize, 4u32, 2147164u32)?;
    emu.sw_no_count(11usize, 6usize, 8u32, 2147168u32)?;
    emu.sw_no_count(12usize, 6usize, 12u32, 2147172u32)?;
    emu.adr_no_count(15usize, 17usize, 15usize, 2147176u32);
    emu.sltru_no_count(10usize, 15usize, 17usize, 2147180u32);
    emu.adr_no_count(28usize, 15usize, 28usize, 2147184u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2147188u32);
    emu.sltru_no_count(11usize, 28usize, 15usize, 2147192u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2147196u32);
    emu.sw_no_count(16usize, 6usize, 16u32, 2147200u32)?;
    emu.sw_no_count(14usize, 6usize, 20u32, 2147204u32)?;
    emu.sw_no_count(28usize, 6usize, 24u32, 2147208u32)?;
    emu.sw_no_count(10usize, 6usize, 28u32, 2147212u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2147216u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2147220u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2147224u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2147228u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2147232u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2147236u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2147240u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2147244u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2147248u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2147252u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2147256u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2147260u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2147264u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2147268u32);
    emu.add_memory_rw_events(32usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147272u32;
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
