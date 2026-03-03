pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2147272u32;
pub const PC_MAX: u32 = 2153232u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 162usize] = [
        block_0x0020c3c8,
        block_0x0020c3e4,
        block_0x0020c3f0,
        block_0x0020c500,
        block_0x0020c504,
        block_0x0020c580,
        block_0x0020c584,
        block_0x0020c5a4,
        block_0x0020c5a8,
        block_0x0020c5c4,
        block_0x0020c5c8,
        block_0x0020c638,
        block_0x0020c63c,
        block_0x0020c6ac,
        block_0x0020c6b0,
        block_0x0020c6d0,
        block_0x0020c6d4,
        block_0x0020c748,
        block_0x0020c74c,
        block_0x0020c76c,
        block_0x0020c770,
        block_0x0020c790,
        block_0x0020c794,
        block_0x0020c7b0,
        block_0x0020c7b4,
        block_0x0020c7e0,
        block_0x0020c7e4,
        block_0x0020c854,
        block_0x0020c858,
        block_0x0020c8c8,
        block_0x0020c8cc,
        block_0x0020c8f4,
        block_0x0020c8f8,
        block_0x0020c974,
        block_0x0020c978,
        block_0x0020c99c,
        block_0x0020c9a0,
        block_0x0020c9c8,
        block_0x0020c9cc,
        block_0x0020c9ec,
        block_0x0020c9f0,
        block_0x0020ca10,
        block_0x0020ca14,
        block_0x0020ca28,
        block_0x0020ca2c,
        block_0x0020ca9c,
        block_0x0020caa0,
        block_0x0020cb10,
        block_0x0020cb14,
        block_0x0020cb34,
        block_0x0020cb38,
        block_0x0020cbb0,
        block_0x0020cbb4,
        block_0x0020cbd4,
        block_0x0020cbd8,
        block_0x0020cbfc,
        block_0x0020cc00,
        block_0x0020cc20,
        block_0x0020cc24,
        block_0x0020cc44,
        block_0x0020cc48,
        block_0x0020cc5c,
        block_0x0020cc60,
        block_0x0020ccd0,
        block_0x0020ccd4,
        block_0x0020cd44,
        block_0x0020cd48,
        block_0x0020cd68,
        block_0x0020cd6c,
        block_0x0020cde4,
        block_0x0020cde8,
        block_0x0020ce08,
        block_0x0020ce0c,
        block_0x0020ce30,
        block_0x0020ce34,
        block_0x0020ce54,
        block_0x0020ce58,
        block_0x0020ce78,
        block_0x0020ce7c,
        block_0x0020ce90,
        block_0x0020ce94,
        block_0x0020cf00,
        block_0x0020cf48,
        block_0x0020cfdc,
        block_0x0020d0bc,
        block_0x0020d0c0,
        block_0x0020d128,
        block_0x0020d12c,
        block_0x0020d1ac,
        block_0x0020d1b0,
        block_0x0020d228,
        block_0x0020d22c,
        block_0x0020d250,
        block_0x0020d254,
        block_0x0020d2a4,
        block_0x0020d2a8,
        block_0x0020d2d0,
        block_0x0020d2d4,
        block_0x0020d358,
        block_0x0020d35c,
        block_0x0020d3d0,
        block_0x0020d3d4,
        block_0x0020d408,
        block_0x0020d40c,
        block_0x0020d484,
        block_0x0020d488,
        block_0x0020d504,
        block_0x0020d510,
        block_0x0020d568,
        block_0x0020d56c,
        block_0x0020d5d4,
        block_0x0020d5d8,
        block_0x0020d618,
        block_0x0020d61c,
        block_0x0020d668,
        block_0x0020d66c,
        block_0x0020d6a0,
        block_0x0020d6a4,
        block_0x0020d6f0,
        block_0x0020d6f4,
        block_0x0020d72c,
        block_0x0020d730,
        block_0x0020d758,
        block_0x0020d75c,
        block_0x0020d784,
        block_0x0020d788,
        block_0x0020d7c0,
        block_0x0020d7c4,
        block_0x0020d81c,
        block_0x0020d820,
        block_0x0020d864,
        block_0x0020d868,
        block_0x0020d894,
        block_0x0020d898,
        block_0x0020d8b4,
        block_0x0020d8b8,
        block_0x0020d8e8,
        block_0x0020d8ec,
        block_0x0020d900,
        block_0x0020d904,
        block_0x0020d924,
        block_0x0020d928,
        block_0x0020d948,
        block_0x0020d94c,
        block_0x0020d968,
        block_0x0020d96c,
        block_0x0020d994,
        block_0x0020d998,
        block_0x0020d9d0,
        block_0x0020d9d4,
        block_0x0020da28,
        block_0x0020da2c,
        block_0x0020da70,
        block_0x0020da74,
        block_0x0020daa0,
        block_0x0020daa4,
        block_0x0020dac0,
        block_0x0020dac4,
        block_0x0020daf4,
        block_0x0020daf8,
        block_0x0020db0c,
        block_0x0020db10,
    ];
    const IDX: [u16; 1491usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        4u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 8u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16,
        17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16,
        21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 26u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 37u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 40u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16,
        43u16, 0u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 49u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16,
        53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 55u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16,
        0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16,
        77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 86u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        97u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16,
        102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16,
        0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 110u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 112u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 113u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 115u16, 116u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16, 118u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 119u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 121u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 123u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 125u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 127u16, 128u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 129u16, 130u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 131u16, 132u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 133u16, 134u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        135u16, 136u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        137u16, 138u16, 0u16, 0u16, 0u16, 0u16, 139u16, 140u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 141u16, 142u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        143u16, 144u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 145u16, 146u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 147u16, 148u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 149u16, 150u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 151u16, 152u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 153u16, 154u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 155u16, 156u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 157u16, 158u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 159u16, 160u16, 0u16, 0u16, 0u16, 0u16, 161u16, 162u16,
    ];
    if pc < 2147272u32 || pc > 2153232u32 {
        return None;
    }
    let word_offset = ((pc - 2147272u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020c3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2147276u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2147280u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2147284u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147288u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967292u32, 2147292u32);
    emu.apc_no_count(1usize, 2147292u32, 4294963200u32, 2147296u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c3e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2147304u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2147308u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147312u32;
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
pub fn block_0x0020c3f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 68u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2147316u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2147320u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2147324u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2147328u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2147332u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2147336u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2147340u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2147344u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2147348u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2147352u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2147356u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2147360u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2147364u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2147368u32)?;
    emu.lw_no_count(29usize, 11usize, 0u32, 2147372u32)?;
    emu.lw_no_count(6usize, 11usize, 4u32, 2147376u32)?;
    let a = 0u32.wrapping_add(4007632896u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147380u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(19922944u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2147384u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3743051776u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2147388u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2147392u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 4294966270u32, 2147396u32);
    emu.adi_no_count(16usize, 13usize, 4294966661u32, 2147400u32);
    emu.adi_no_count(12usize, 14usize, 4294966305u32, 2147404u32);
    emu.adi_no_count(13usize, 15usize, 1362u32, 2147408u32);
    emu.mulhu_no_count(14usize, 29usize, 17usize, 2147412u32);
    emu.mul_no_count(15usize, 6usize, 17usize, 2147416u32);
    emu.mulhu_no_count(5usize, 6usize, 17usize, 2147420u32);
    emu.mul_no_count(7usize, 29usize, 16usize, 2147424u32);
    emu.mulhu_no_count(28usize, 29usize, 16usize, 2147428u32);
    emu.mul_no_count(30usize, 6usize, 16usize, 2147432u32);
    emu.mulhu_no_count(31usize, 29usize, 12usize, 2147436u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2147440u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2147444u32);
    emu.adr_no_count(15usize, 5usize, 15usize, 2147448u32);
    emu.mul_no_count(5usize, 6usize, 12usize, 2147452u32);
    emu.adr_no_count(14usize, 7usize, 14usize, 2147456u32);
    emu.sltru_no_count(14usize, 14usize, 7usize, 2147460u32);
    emu.mulhu_no_count(7usize, 6usize, 12usize, 2147464u32);
    emu.adr_no_count(31usize, 5usize, 31usize, 2147468u32);
    emu.sltru_no_count(5usize, 31usize, 5usize, 2147472u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2147476u32);
    emu.mul_no_count(7usize, 29usize, 13usize, 2147480u32);
    emu.adr_no_count(14usize, 28usize, 14usize, 2147484u32);
    emu.mulhu_no_count(28usize, 29usize, 13usize, 2147488u32);
    emu.adr_no_count(31usize, 7usize, 31usize, 2147492u32);
    emu.sltru_no_count(7usize, 31usize, 7usize, 2147496u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2147500u32);
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2147504u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2147508u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2147512u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2147516u32);
    emu.mulhu_no_count(28usize, 6usize, 13usize, 2147520u32);
    emu.adr_no_count(7usize, 5usize, 7usize, 2147524u32);
    emu.sltru_no_count(5usize, 7usize, 5usize, 2147528u32);
    emu.adr_no_count(28usize, 28usize, 5usize, 2147532u32);
    emu.mul_no_count(8usize, 6usize, 13usize, 2147536u32);
    emu.adr_no_count(9usize, 30usize, 14usize, 2147540u32);
    emu.sltru_no_count(14usize, 9usize, 30usize, 2147544u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2147548u32);
    emu.mul_no_count(19usize, 29usize, 12usize, 2147552u32);
    emu.adr_no_count(5usize, 8usize, 7usize, 2147556u32);
    emu.sltru_no_count(7usize, 5usize, 8usize, 2147560u32);
    emu.adr_no_count(19usize, 9usize, 19usize, 2147564u32);
    emu.sltru_no_count(15usize, 19usize, 9usize, 2147568u32);
    emu.adr_no_count(31usize, 14usize, 31usize, 2147572u32);
    emu.adr_no_count(8usize, 31usize, 15usize, 2147576u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2147580u32);
    emu.add_memory_rw_events(67usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2147588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c504));
    } else {
        emu.pc = 2147584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c500));
    }
}
#[inline(always)]
pub fn block_0x0020c500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 8usize, 14usize, 2147588u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c504));
}
#[inline(never)]
pub fn block_0x0020c504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 5usize, 15usize, 2147592u32);
    emu.sbr_no_count(30usize, 0usize, 6usize, 2147596u32);
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2147600u32);
    emu.adr_no_count(9usize, 29usize, 29usize, 2147604u32);
    emu.adi_no_count(14usize, 0usize, 4294967294u32, 2147608u32);
    emu.adr_no_count(18usize, 6usize, 6usize, 2147612u32);
    emu.sltru_no_count(28usize, 31usize, 5usize, 2147616u32);
    emu.mulhu_no_count(5usize, 29usize, 15usize, 2147620u32);
    emu.mulhu_no_count(20usize, 6usize, 15usize, 2147624u32);
    emu.sbr_no_count(21usize, 0usize, 9usize, 2147628u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2147632u32);
    emu.mulhu_no_count(22usize, 29usize, 14usize, 2147636u32);
    emu.sbr_no_count(7usize, 5usize, 6usize, 2147640u32);
    emu.sltru_no_count(5usize, 7usize, 30usize, 2147644u32);
    emu.adr_no_count(5usize, 20usize, 5usize, 2147648u32);
    emu.sbr_no_count(20usize, 0usize, 18usize, 2147652u32);
    emu.sbr_no_count(9usize, 7usize, 9usize, 2147656u32);
    emu.sltru_no_count(30usize, 9usize, 21usize, 2147660u32);
    emu.adr_no_count(30usize, 22usize, 30usize, 2147664u32);
    emu.mulhu_no_count(21usize, 6usize, 14usize, 2147668u32);
    emu.adr_no_count(22usize, 5usize, 30usize, 2147672u32);
    emu.sbr_no_count(30usize, 22usize, 18usize, 2147676u32);
    emu.sltru_no_count(18usize, 22usize, 5usize, 2147680u32);
    emu.adr_no_count(21usize, 21usize, 18usize, 2147684u32);
    emu.sbr_no_count(18usize, 31usize, 29usize, 2147688u32);
    emu.sltru_no_count(31usize, 18usize, 31usize, 2147692u32);
    emu.adr_no_count(9usize, 28usize, 9usize, 2147696u32);
    emu.sltru_no_count(20usize, 30usize, 20usize, 2147700u32);
    emu.adr_no_count(9usize, 9usize, 31usize, 2147704u32);
    emu.adr_no_count(20usize, 21usize, 20usize, 2147708u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2147716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c584));
    } else {
        emu.pc = 2147712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c580));
    }
}
#[inline(always)]
pub fn block_0x0020c580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 9usize, 28usize, 2147716u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c584));
}
#[inline(always)]
pub fn block_0x0020c584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 30usize, 31usize, 2147720u32);
    emu.sltru_no_count(30usize, 28usize, 30usize, 2147724u32);
    emu.adr_no_count(20usize, 20usize, 30usize, 2147728u32);
    emu.sbr_no_count(31usize, 28usize, 29usize, 2147732u32);
    emu.sltru_no_count(28usize, 31usize, 28usize, 2147736u32);
    emu.adr_no_count(7usize, 20usize, 7usize, 2147740u32);
    emu.adr_no_count(30usize, 7usize, 28usize, 2147744u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2147752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5a8));
    } else {
        emu.pc = 2147748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5a4));
    }
}
#[inline(always)]
pub fn block_0x0020c5a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 30usize, 20usize, 2147752u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147752u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c5a8));
}
#[inline(always)]
pub fn block_0x0020c5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 5usize, 28usize, 2147756u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2147760u32);
    emu.adr_no_count(29usize, 28usize, 29usize, 2147764u32);
    emu.sltru_no_count(5usize, 29usize, 28usize, 2147768u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2147772u32);
    emu.adr_no_count(28usize, 6usize, 5usize, 2147776u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2147784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5c8));
    } else {
        emu.pc = 2147780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5c4));
    }
}
#[inline(always)]
pub fn block_0x0020c5c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2147784u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c5c8));
}
#[inline(never)]
pub fn block_0x0020c5c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 11usize, 8u32, 2147788u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2147792u32)?;
    emu.ani_no_count(19usize, 19usize, 4294967294u32, 2147796u32);
    emu.mulhu_no_count(21usize, 7usize, 17usize, 2147800u32);
    emu.mul_no_count(22usize, 6usize, 17usize, 2147804u32);
    emu.mulhu_no_count(23usize, 6usize, 17usize, 2147808u32);
    emu.mul_no_count(24usize, 7usize, 16usize, 2147812u32);
    emu.mul_no_count(20usize, 7usize, 17usize, 2147816u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2147820u32);
    emu.sltru_no_count(20usize, 20usize, 19usize, 2147824u32);
    emu.mulhu_no_count(19usize, 7usize, 16usize, 2147828u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2147832u32);
    emu.sltru_no_count(22usize, 21usize, 22usize, 2147836u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2147840u32);
    emu.mul_no_count(23usize, 6usize, 16usize, 2147844u32);
    emu.adr_no_count(21usize, 24usize, 21usize, 2147848u32);
    emu.sltru_no_count(24usize, 21usize, 24usize, 2147852u32);
    emu.adr_no_count(19usize, 19usize, 24usize, 2147856u32);
    emu.mulhu_no_count(24usize, 6usize, 16usize, 2147860u32);
    emu.adr_no_count(21usize, 8usize, 21usize, 2147864u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2147868u32);
    emu.sltru_no_count(22usize, 19usize, 22usize, 2147872u32);
    emu.adr_no_count(19usize, 23usize, 19usize, 2147876u32);
    emu.sltru_no_count(23usize, 19usize, 23usize, 2147880u32);
    emu.adr_no_count(24usize, 24usize, 22usize, 2147884u32);
    emu.adr_no_count(22usize, 21usize, 20usize, 2147888u32);
    emu.adr_no_count(21usize, 24usize, 23usize, 2147892u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2147900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c63c));
    } else {
        emu.pc = 2147896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c638));
    }
}
#[inline(always)]
pub fn block_0x0020c638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 22usize, 8usize, 2147900u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c63c));
}
#[inline(never)]
pub fn block_0x0020c63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 19usize, 20usize, 2147904u32);
    emu.mulhu_no_count(22usize, 7usize, 12usize, 2147908u32);
    emu.mul_no_count(23usize, 6usize, 12usize, 2147912u32);
    emu.mulhu_no_count(24usize, 6usize, 12usize, 2147916u32);
    emu.mul_no_count(25usize, 7usize, 13usize, 2147920u32);
    emu.sltru_no_count(8usize, 20usize, 19usize, 2147924u32);
    emu.adr_no_count(8usize, 21usize, 8usize, 2147928u32);
    emu.mulhu_no_count(19usize, 7usize, 13usize, 2147932u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2147936u32);
    emu.sltru_no_count(21usize, 22usize, 23usize, 2147940u32);
    emu.adr_no_count(21usize, 24usize, 21usize, 2147944u32);
    emu.mul_no_count(23usize, 6usize, 13usize, 2147948u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2147952u32);
    emu.sltru_no_count(24usize, 22usize, 25usize, 2147956u32);
    emu.adr_no_count(19usize, 19usize, 24usize, 2147960u32);
    emu.mulhu_no_count(24usize, 6usize, 13usize, 2147964u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2147968u32);
    emu.sltru_no_count(21usize, 19usize, 21usize, 2147972u32);
    emu.adr_no_count(24usize, 24usize, 21usize, 2147976u32);
    emu.mul_no_count(21usize, 7usize, 12usize, 2147980u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2147984u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2147988u32);
    emu.adr_no_count(25usize, 22usize, 18usize, 2147992u32);
    emu.adr_no_count(22usize, 23usize, 19usize, 2147996u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2148000u32);
    emu.adr_no_count(19usize, 9usize, 25usize, 2148004u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2148008u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2148016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6b0));
    } else {
        emu.pc = 2148012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6ac));
    }
}
#[inline(always)]
pub fn block_0x0020c6ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 19usize, 9usize, 2148016u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c6b0));
}
#[inline(always)]
pub fn block_0x0020c6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 22usize, 18usize, 2148020u32);
    emu.adr_no_count(8usize, 19usize, 8usize, 2148024u32);
    emu.adr_no_count(20usize, 21usize, 20usize, 2148028u32);
    emu.sltru_no_count(22usize, 18usize, 22usize, 2148032u32);
    emu.sltru_no_count(21usize, 20usize, 21usize, 2148036u32);
    emu.adr_no_count(9usize, 8usize, 21usize, 2148040u32);
    emu.adr_no_count(8usize, 23usize, 22usize, 2148044u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2148052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6d4));
    } else {
        emu.pc = 2148048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6d0));
    }
}
#[inline(always)]
pub fn block_0x0020c6d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 9usize, 19usize, 2148052u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c6d4));
}
#[inline(never)]
pub fn block_0x0020c6d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 18usize, 21usize, 2148056u32);
    emu.sbr_no_count(22usize, 0usize, 6usize, 2148060u32);
    emu.mulhu_no_count(21usize, 7usize, 15usize, 2148064u32);
    emu.mulhu_no_count(23usize, 6usize, 15usize, 2148068u32);
    emu.adr_no_count(24usize, 7usize, 7usize, 2148072u32);
    emu.mulhu_no_count(25usize, 7usize, 14usize, 2148076u32);
    emu.adr_no_count(26usize, 6usize, 6usize, 2148080u32);
    emu.sltru_no_count(18usize, 19usize, 18usize, 2148084u32);
    emu.adr_no_count(18usize, 8usize, 18usize, 2148088u32);
    emu.mulhu_no_count(27usize, 6usize, 14usize, 2148092u32);
    emu.sbr_no_count(21usize, 21usize, 6usize, 2148096u32);
    emu.sltru_no_count(8usize, 21usize, 22usize, 2148100u32);
    emu.adr_no_count(8usize, 23usize, 8usize, 2148104u32);
    emu.sbr_no_count(22usize, 0usize, 24usize, 2148108u32);
    emu.sbr_no_count(24usize, 21usize, 24usize, 2148112u32);
    emu.sltru_no_count(22usize, 24usize, 22usize, 2148116u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2148120u32);
    emu.adr_no_count(23usize, 8usize, 22usize, 2148124u32);
    emu.sltru_no_count(22usize, 23usize, 8usize, 2148128u32);
    emu.adr_no_count(27usize, 27usize, 22usize, 2148132u32);
    emu.sbr_no_count(22usize, 31usize, 7usize, 2148136u32);
    emu.sbr_no_count(23usize, 23usize, 26usize, 2148140u32);
    emu.sbr_no_count(26usize, 0usize, 26usize, 2148144u32);
    emu.sltru_no_count(25usize, 22usize, 31usize, 2148148u32);
    emu.adr_no_count(31usize, 24usize, 25usize, 2148152u32);
    emu.sltru_no_count(24usize, 23usize, 26usize, 2148156u32);
    emu.adr_no_count(31usize, 30usize, 31usize, 2148160u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2148164u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2148172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c74c));
    } else {
        emu.pc = 2148168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c748));
    }
}
#[inline(always)]
pub fn block_0x0020c748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 31usize, 30usize, 2148172u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c74c));
}
#[inline(always)]
pub fn block_0x0020c74c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 23usize, 25usize, 2148176u32);
    emu.adr_no_count(18usize, 31usize, 18usize, 2148180u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2148184u32);
    emu.sltru_no_count(23usize, 30usize, 23usize, 2148188u32);
    emu.sltru_no_count(22usize, 19usize, 22usize, 2148192u32);
    emu.adr_no_count(18usize, 18usize, 22usize, 2148196u32);
    emu.adr_no_count(24usize, 24usize, 23usize, 2148200u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2148208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c770));
    } else {
        emu.pc = 2148204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c76c));
    }
}
#[inline(always)]
pub fn block_0x0020c76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 18usize, 31usize, 2148208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c770));
}
#[inline(always)]
pub fn block_0x0020c770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 30usize, 22usize, 2148212u32);
    emu.sbr_no_count(22usize, 29usize, 7usize, 2148216u32);
    emu.sltru_no_count(23usize, 31usize, 30usize, 2148220u32);
    emu.sltru_no_count(30usize, 22usize, 29usize, 2148224u32);
    emu.adr_no_count(29usize, 28usize, 21usize, 2148228u32);
    emu.adr_no_count(29usize, 29usize, 30usize, 2148232u32);
    emu.adr_no_count(24usize, 24usize, 23usize, 2148236u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2148244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c794));
    } else {
        emu.pc = 2148240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c790));
    }
}
#[inline(always)]
pub fn block_0x0020c790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 29usize, 28usize, 2148244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c794));
}
#[inline(always)]
pub fn block_0x0020c794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 8usize, 30usize, 2148248u32);
    emu.adr_no_count(30usize, 29usize, 24usize, 2148252u32);
    emu.adr_no_count(31usize, 22usize, 31usize, 2148256u32);
    emu.sltru_no_count(22usize, 31usize, 22usize, 2148260u32);
    emu.adr_no_count(30usize, 30usize, 22usize, 2148264u32);
    emu.sltru_no_count(21usize, 28usize, 8usize, 2148268u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2148276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7b4));
    } else {
        emu.pc = 2148272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7b0));
    }
}
#[inline(always)]
pub fn block_0x0020c7b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 30usize, 29usize, 2148276u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c7b4));
}
#[inline]
pub fn block_0x0020c7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 28usize, 22usize, 2148280u32);
    emu.adr_no_count(7usize, 5usize, 7usize, 2148284u32);
    emu.sltru_no_count(29usize, 22usize, 28usize, 2148288u32);
    emu.sltru_no_count(8usize, 7usize, 5usize, 2148292u32);
    emu.adr_no_count(28usize, 7usize, 22usize, 2148296u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2148300u32);
    emu.adr_no_count(21usize, 6usize, 8usize, 2148304u32);
    emu.adr_no_count(5usize, 21usize, 29usize, 2148308u32);
    emu.sltru_no_count(29usize, 28usize, 7usize, 2148312u32);
    emu.adr_no_count(7usize, 5usize, 29usize, 2148316u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2148324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7e4));
    } else {
        emu.pc = 2148320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7e0));
    }
}
#[inline(always)]
pub fn block_0x0020c7e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 21usize, 2148324u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c7e4));
}
#[inline(never)]
pub fn block_0x0020c7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 16u32, 2148328u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2148332u32)?;
    emu.ani_no_count(20usize, 20usize, 4294967294u32, 2148336u32);
    emu.mulhu_no_count(22usize, 6usize, 17usize, 2148340u32);
    emu.mul_no_count(23usize, 5usize, 17usize, 2148344u32);
    emu.mulhu_no_count(24usize, 5usize, 17usize, 2148348u32);
    emu.mul_no_count(25usize, 6usize, 16usize, 2148352u32);
    emu.mul_no_count(26usize, 6usize, 17usize, 2148356u32);
    emu.adr_no_count(26usize, 20usize, 26usize, 2148360u32);
    emu.sltru_no_count(20usize, 26usize, 20usize, 2148364u32);
    emu.mulhu_no_count(26usize, 6usize, 16usize, 2148368u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2148372u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2148376u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2148380u32);
    emu.mul_no_count(24usize, 5usize, 16usize, 2148384u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2148388u32);
    emu.sltru_no_count(25usize, 22usize, 25usize, 2148392u32);
    emu.adr_no_count(25usize, 26usize, 25usize, 2148396u32);
    emu.mulhu_no_count(26usize, 5usize, 16usize, 2148400u32);
    emu.adr_no_count(27usize, 22usize, 20usize, 2148404u32);
    emu.adr_no_count(22usize, 23usize, 25usize, 2148408u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2148412u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2148416u32);
    emu.sltru_no_count(25usize, 22usize, 24usize, 2148420u32);
    emu.adr_no_count(23usize, 26usize, 23usize, 2148424u32);
    emu.adr_no_count(24usize, 9usize, 27usize, 2148428u32);
    emu.adr_no_count(23usize, 23usize, 25usize, 2148432u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2148440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c858));
    } else {
        emu.pc = 2148436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c854));
    }
}
#[inline(always)]
pub fn block_0x0020c854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 24usize, 9usize, 2148440u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c858));
}
#[inline(never)]
pub fn block_0x0020c858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 22usize, 20usize, 2148444u32);
    emu.mulhu_no_count(24usize, 6usize, 12usize, 2148448u32);
    emu.mul_no_count(25usize, 5usize, 12usize, 2148452u32);
    emu.mulhu_no_count(26usize, 5usize, 12usize, 2148456u32);
    emu.mul_no_count(27usize, 6usize, 13usize, 2148460u32);
    emu.sltru_no_count(9usize, 20usize, 22usize, 2148464u32);
    emu.adr_no_count(9usize, 23usize, 9usize, 2148468u32);
    emu.mulhu_no_count(22usize, 6usize, 13usize, 2148472u32);
    emu.adr_no_count(24usize, 25usize, 24usize, 2148476u32);
    emu.sltru_no_count(23usize, 24usize, 25usize, 2148480u32);
    emu.adr_no_count(23usize, 26usize, 23usize, 2148484u32);
    emu.mul_no_count(26usize, 5usize, 13usize, 2148488u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2148492u32);
    emu.sltru_no_count(25usize, 24usize, 27usize, 2148496u32);
    emu.adr_no_count(22usize, 22usize, 25usize, 2148500u32);
    emu.mulhu_no_count(25usize, 5usize, 13usize, 2148504u32);
    emu.adr_no_count(27usize, 23usize, 22usize, 2148508u32);
    emu.sltru_no_count(22usize, 27usize, 23usize, 2148512u32);
    emu.adr_no_count(1usize, 25usize, 22usize, 2148516u32);
    emu.mul_no_count(22usize, 6usize, 12usize, 2148520u32);
    emu.adr_no_count(22usize, 19usize, 22usize, 2148524u32);
    emu.sltru_no_count(25usize, 22usize, 19usize, 2148528u32);
    emu.adr_no_count(19usize, 24usize, 25usize, 2148532u32);
    emu.adr_no_count(23usize, 26usize, 27usize, 2148536u32);
    emu.sltru_no_count(24usize, 23usize, 26usize, 2148540u32);
    emu.adr_no_count(19usize, 18usize, 19usize, 2148544u32);
    emu.adr_no_count(24usize, 1usize, 24usize, 2148548u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2148556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8cc));
    } else {
        emu.pc = 2148552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8c8));
    }
}
#[inline(always)]
pub fn block_0x0020c8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 18usize, 2148556u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c8cc));
}
#[inline]
pub fn block_0x0020c8cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 0usize, 21usize, 2148560u32);
    emu.adr_no_count(21usize, 23usize, 25usize, 2148564u32);
    emu.adr_no_count(9usize, 19usize, 9usize, 2148568u32);
    emu.adr_no_count(20usize, 22usize, 20usize, 2148572u32);
    emu.sltru_no_count(23usize, 21usize, 23usize, 2148576u32);
    emu.sltru_no_count(22usize, 20usize, 22usize, 2148580u32);
    emu.adr_no_count(9usize, 9usize, 22usize, 2148584u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2148588u32);
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2148592u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2148600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8f8));
    } else {
        emu.pc = 2148596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8f4));
    }
}
#[inline(always)]
pub fn block_0x0020c8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 9usize, 19usize, 2148600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c8f8));
}
#[inline(never)]
pub fn block_0x0020c8f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2148604u32);
    emu.adr_no_count(19usize, 21usize, 22usize, 2148608u32);
    emu.sbr_no_count(24usize, 0usize, 5usize, 2148612u32);
    emu.mulhu_no_count(22usize, 6usize, 15usize, 2148616u32);
    emu.mulhu_no_count(25usize, 5usize, 15usize, 2148620u32);
    emu.adr_no_count(26usize, 6usize, 6usize, 2148624u32);
    emu.mulhu_no_count(27usize, 6usize, 14usize, 2148628u32);
    emu.adr_no_count(1usize, 5usize, 5usize, 2148632u32);
    emu.sltru_no_count(21usize, 19usize, 21usize, 2148636u32);
    emu.adr_no_count(23usize, 23usize, 21usize, 2148640u32);
    emu.adi_no_count(14usize, 0usize, 4294967294u32, 2148644u32);
    emu.mulhu_no_count(14usize, 5usize, 14usize, 2148648u32);
    emu.sbr_no_count(22usize, 22usize, 5usize, 2148652u32);
    emu.sltru_no_count(21usize, 22usize, 24usize, 2148656u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2148660u32);
    emu.sbr_no_count(24usize, 0usize, 26usize, 2148664u32);
    emu.sbr_no_count(15usize, 22usize, 26usize, 2148668u32);
    emu.sltru_no_count(24usize, 15usize, 24usize, 2148672u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2148676u32);
    emu.adr_no_count(25usize, 21usize, 24usize, 2148680u32);
    emu.sltru_no_count(24usize, 25usize, 21usize, 2148684u32);
    emu.adr_no_count(14usize, 14usize, 24usize, 2148688u32);
    emu.sbr_no_count(24usize, 31usize, 6usize, 2148692u32);
    emu.sbr_no_count(25usize, 25usize, 1usize, 2148696u32);
    emu.sbr_no_count(27usize, 0usize, 1usize, 2148700u32);
    emu.sltru_no_count(26usize, 24usize, 31usize, 2148704u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2148708u32);
    emu.sltru_no_count(27usize, 25usize, 27usize, 2148712u32);
    emu.adr_no_count(31usize, 30usize, 15usize, 2148716u32);
    emu.adr_no_count(27usize, 14usize, 27usize, 2148720u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2148728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c978));
    } else {
        emu.pc = 2148724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c974));
    }
}
#[inline(always)]
pub fn block_0x0020c974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 31usize, 30usize, 2148728u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c978));
}
#[inline]
pub fn block_0x0020c978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(30usize, 18usize, 8usize, 2148732u32);
    emu.adr_no_count(26usize, 25usize, 26usize, 2148736u32);
    emu.adr_no_count(18usize, 31usize, 23usize, 2148740u32);
    emu.adr_no_count(19usize, 24usize, 19usize, 2148744u32);
    emu.sltru_no_count(23usize, 26usize, 25usize, 2148748u32);
    emu.sltru_no_count(8usize, 19usize, 24usize, 2148752u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2148756u32);
    emu.adr_no_count(23usize, 27usize, 23usize, 2148760u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2148768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9a0));
    } else {
        emu.pc = 2148764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c99c));
    }
}
#[inline(always)]
pub fn block_0x0020c99c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 18usize, 31usize, 2148768u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c9a0));
}
#[inline]
pub fn block_0x0020c9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 26usize, 8usize, 2148772u32);
    emu.sltru_no_count(14usize, 8usize, 26usize, 2148776u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2148780u32);
    emu.sbr_no_count(31usize, 28usize, 6usize, 2148784u32);
    emu.adr_no_count(14usize, 7usize, 22usize, 2148788u32);
    emu.sltru_no_count(22usize, 31usize, 28usize, 2148792u32);
    emu.adr_no_count(28usize, 14usize, 22usize, 2148796u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2148800u32);
    emu.adi_no_count(1usize, 0usize, 4294967295u32, 2148804u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2148812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9cc));
    } else {
        emu.pc = 2148808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9c8));
    }
}
#[inline(always)]
pub fn block_0x0020c9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 28usize, 7usize, 2148812u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c9cc));
}
#[inline(always)]
pub fn block_0x0020c9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 29usize, 30usize, 2148816u32);
    emu.adr_no_count(30usize, 21usize, 22usize, 2148820u32);
    emu.adr_no_count(23usize, 28usize, 23usize, 2148824u32);
    emu.adr_no_count(8usize, 31usize, 8usize, 2148828u32);
    emu.sltru_no_count(22usize, 8usize, 31usize, 2148832u32);
    emu.adr_no_count(31usize, 23usize, 22usize, 2148836u32);
    emu.sltru_no_count(21usize, 30usize, 21usize, 2148840u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2148848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9f0));
    } else {
        emu.pc = 2148844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9ec));
    }
}
#[inline(always)]
pub fn block_0x0020c9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 31usize, 28usize, 2148848u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c9f0));
}
#[inline(always)]
pub fn block_0x0020c9f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 30usize, 22usize, 2148852u32);
    emu.adr_no_count(22usize, 29usize, 6usize, 2148856u32);
    emu.sltru_no_count(14usize, 28usize, 30usize, 2148860u32);
    emu.sltru_no_count(6usize, 22usize, 29usize, 2148864u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2148868u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2148872u32);
    emu.adr_no_count(21usize, 21usize, 14usize, 2148876u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2148884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca14));
    } else {
        emu.pc = 2148880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca10));
    }
}
#[inline(always)]
pub fn block_0x0020ca10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 5usize, 7usize, 2148884u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca14));
}
#[inline(always)]
pub fn block_0x0020ca14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 22usize, 28usize, 2148888u32);
    emu.sltru_no_count(30usize, 29usize, 22usize, 2148892u32);
    emu.adr_no_count(28usize, 5usize, 21usize, 2148896u32);
    emu.adr_no_count(28usize, 28usize, 30usize, 2148900u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2148908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca2c));
    } else {
        emu.pc = 2148904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca28));
    }
}
#[inline(always)]
pub fn block_0x0020ca28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 28usize, 5usize, 2148908u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca2c));
}
#[inline(never)]
pub fn block_0x0020ca2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 11usize, 24u32, 2148912u32)?;
    emu.lw_no_count(7usize, 11usize, 28u32, 2148916u32)?;
    emu.ani_no_count(14usize, 20usize, 4294967294u32, 2148920u32);
    emu.mulhu_no_count(15usize, 5usize, 17usize, 2148924u32);
    emu.mul_no_count(21usize, 7usize, 17usize, 2148928u32);
    emu.mulhu_no_count(22usize, 7usize, 17usize, 2148932u32);
    emu.mul_no_count(23usize, 5usize, 16usize, 2148936u32);
    emu.mul_no_count(20usize, 5usize, 17usize, 2148940u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2148944u32);
    emu.sltru_no_count(20usize, 20usize, 14usize, 2148948u32);
    emu.mulhu_no_count(14usize, 5usize, 16usize, 2148952u32);
    emu.adr_no_count(15usize, 21usize, 15usize, 2148956u32);
    emu.sltru_no_count(21usize, 15usize, 21usize, 2148960u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2148964u32);
    emu.mul_no_count(22usize, 7usize, 16usize, 2148968u32);
    emu.adr_no_count(15usize, 23usize, 15usize, 2148972u32);
    emu.sltru_no_count(23usize, 15usize, 23usize, 2148976u32);
    emu.adr_no_count(14usize, 14usize, 23usize, 2148980u32);
    emu.mulhu_no_count(23usize, 7usize, 16usize, 2148984u32);
    emu.adr_no_count(15usize, 15usize, 20usize, 2148988u32);
    emu.adr_no_count(14usize, 21usize, 14usize, 2148992u32);
    emu.sltru_no_count(24usize, 14usize, 21usize, 2148996u32);
    emu.adr_no_count(21usize, 22usize, 14usize, 2149000u32);
    emu.sltru_no_count(22usize, 21usize, 22usize, 2149004u32);
    emu.adr_no_count(24usize, 23usize, 24usize, 2149008u32);
    emu.adr_no_count(23usize, 9usize, 15usize, 2149012u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2149016u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2149024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caa0));
    } else {
        emu.pc = 2149020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca9c));
    }
}
#[inline(always)]
pub fn block_0x0020ca9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 23usize, 9usize, 2149024u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149024u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020caa0));
}
#[inline(never)]
pub fn block_0x0020caa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 20usize, 2149028u32);
    emu.mulhu_no_count(14usize, 5usize, 12usize, 2149032u32);
    emu.mul_no_count(15usize, 7usize, 12usize, 2149036u32);
    emu.mulhu_no_count(23usize, 7usize, 12usize, 2149040u32);
    emu.mul_no_count(24usize, 5usize, 13usize, 2149044u32);
    emu.sltru_no_count(20usize, 9usize, 21usize, 2149048u32);
    emu.adr_no_count(20usize, 22usize, 20usize, 2149052u32);
    emu.mulhu_no_count(21usize, 5usize, 13usize, 2149056u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2149060u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2149064u32);
    emu.adr_no_count(15usize, 23usize, 15usize, 2149068u32);
    emu.mul_no_count(23usize, 7usize, 13usize, 2149072u32);
    emu.adr_no_count(14usize, 24usize, 14usize, 2149076u32);
    emu.sltru_no_count(22usize, 14usize, 24usize, 2149080u32);
    emu.adr_no_count(21usize, 21usize, 22usize, 2149084u32);
    emu.mulhu_no_count(22usize, 7usize, 13usize, 2149088u32);
    emu.adr_no_count(24usize, 15usize, 21usize, 2149092u32);
    emu.sltru_no_count(15usize, 24usize, 15usize, 2149096u32);
    emu.adr_no_count(15usize, 22usize, 15usize, 2149100u32);
    emu.mul_no_count(22usize, 5usize, 12usize, 2149104u32);
    emu.adr_no_count(22usize, 19usize, 22usize, 2149108u32);
    emu.sltru_no_count(21usize, 22usize, 19usize, 2149112u32);
    emu.adr_no_count(14usize, 14usize, 21usize, 2149116u32);
    emu.adr_no_count(24usize, 23usize, 24usize, 2149120u32);
    emu.sltru_no_count(25usize, 24usize, 23usize, 2149124u32);
    emu.adr_no_count(23usize, 18usize, 14usize, 2149128u32);
    emu.adr_no_count(25usize, 15usize, 25usize, 2149132u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2149140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb14));
    } else {
        emu.pc = 2149136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb10));
    }
}
#[inline(always)]
pub fn block_0x0020cb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 23usize, 18usize, 2149140u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb14));
}
#[inline(always)]
pub fn block_0x0020cb14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 24usize, 21usize, 2149144u32);
    emu.adr_no_count(20usize, 23usize, 20usize, 2149148u32);
    emu.adr_no_count(19usize, 22usize, 9usize, 2149152u32);
    emu.sltru_no_count(14usize, 21usize, 24usize, 2149156u32);
    emu.sltru_no_count(18usize, 19usize, 22usize, 2149160u32);
    emu.adr_no_count(9usize, 20usize, 18usize, 2149164u32);
    emu.adr_no_count(22usize, 25usize, 14usize, 2149168u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2149176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb38));
    } else {
        emu.pc = 2149172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb34));
    }
}
#[inline(always)]
pub fn block_0x0020cb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 9usize, 23usize, 2149176u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb38));
}
#[inline(never)]
pub fn block_0x0020cb38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 21usize, 18usize, 2149180u32);
    emu.sbr_no_count(14usize, 0usize, 7usize, 2149184u32);
    emu.mulhu_no_count(15usize, 5usize, 1usize, 2149188u32);
    emu.mulhu_no_count(20usize, 7usize, 1usize, 2149192u32);
    emu.adr_no_count(23usize, 5usize, 5usize, 2149196u32);
    emu.adi_no_count(25usize, 0usize, 4294967294u32, 2149200u32);
    emu.mulhu_no_count(24usize, 5usize, 25usize, 2149204u32);
    emu.adr_no_count(26usize, 7usize, 7usize, 2149208u32);
    emu.sltru_no_count(21usize, 18usize, 21usize, 2149212u32);
    emu.adr_no_count(22usize, 22usize, 21usize, 2149216u32);
    emu.mulhu_no_count(25usize, 7usize, 25usize, 2149220u32);
    emu.sbr_no_count(21usize, 15usize, 7usize, 2149224u32);
    emu.sltru_no_count(14usize, 21usize, 14usize, 2149228u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2149232u32);
    emu.sbr_no_count(14usize, 0usize, 23usize, 2149236u32);
    emu.sbr_no_count(15usize, 21usize, 23usize, 2149240u32);
    emu.sltru_no_count(14usize, 15usize, 14usize, 2149244u32);
    emu.adr_no_count(14usize, 24usize, 14usize, 2149248u32);
    emu.adr_no_count(14usize, 20usize, 14usize, 2149252u32);
    emu.sltru_no_count(23usize, 14usize, 20usize, 2149256u32);
    emu.adr_no_count(27usize, 25usize, 23usize, 2149260u32);
    emu.sbr_no_count(23usize, 8usize, 5usize, 2149264u32);
    emu.sbr_no_count(25usize, 14usize, 26usize, 2149268u32);
    emu.sbr_no_count(14usize, 0usize, 26usize, 2149272u32);
    emu.sltru_no_count(26usize, 23usize, 8usize, 2149276u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2149280u32);
    emu.sltru_no_count(14usize, 25usize, 14usize, 2149284u32);
    emu.adr_no_count(24usize, 31usize, 15usize, 2149288u32);
    emu.adr_no_count(27usize, 27usize, 14usize, 2149292u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2149300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbb4));
    } else {
        emu.pc = 2149296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbb0));
    }
}
#[inline(always)]
pub fn block_0x0020cbb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 24usize, 31usize, 2149300u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbb4));
}
#[inline(always)]
pub fn block_0x0020cbb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 25usize, 26usize, 2149304u32);
    emu.adr_no_count(8usize, 24usize, 22usize, 2149308u32);
    emu.adr_no_count(18usize, 23usize, 18usize, 2149312u32);
    emu.sltru_no_count(14usize, 26usize, 25usize, 2149316u32);
    emu.sltru_no_count(31usize, 18usize, 23usize, 2149320u32);
    emu.adr_no_count(8usize, 8usize, 31usize, 2149324u32);
    emu.adr_no_count(23usize, 27usize, 14usize, 2149328u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2149336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbd8));
    } else {
        emu.pc = 2149332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbd4));
    }
}
#[inline(always)]
pub fn block_0x0020cbd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 8usize, 24usize, 2149336u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149336u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbd8));
}
#[inline]
pub fn block_0x0020cbd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 26usize, 31usize, 2149340u32);
    emu.sltru_no_count(14usize, 31usize, 26usize, 2149344u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2149348u32);
    emu.sbr_no_count(22usize, 29usize, 5usize, 2149352u32);
    emu.adr_no_count(21usize, 28usize, 21usize, 2149356u32);
    emu.sltru_no_count(24usize, 22usize, 29usize, 2149360u32);
    emu.adr_no_count(21usize, 21usize, 24usize, 2149364u32);
    emu.adr_no_count(29usize, 6usize, 30usize, 2149368u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2149376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc00));
    } else {
        emu.pc = 2149372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbfc));
    }
}
#[inline(always)]
pub fn block_0x0020cbfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(24usize, 21usize, 28usize, 2149376u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc00));
}
#[inline(always)]
pub fn block_0x0020cc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 29usize, 6usize, 2149380u32);
    emu.adr_no_count(28usize, 20usize, 24usize, 2149384u32);
    emu.adr_no_count(30usize, 21usize, 23usize, 2149388u32);
    emu.adr_no_count(31usize, 22usize, 31usize, 2149392u32);
    emu.sltru_no_count(22usize, 31usize, 22usize, 2149396u32);
    emu.adr_no_count(30usize, 30usize, 22usize, 2149400u32);
    emu.sltru_no_count(20usize, 28usize, 20usize, 2149404u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2149412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc24));
    } else {
        emu.pc = 2149408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc20));
    }
}
#[inline(always)]
pub fn block_0x0020cc20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 30usize, 21usize, 2149412u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc24));
}
#[inline(always)]
pub fn block_0x0020cc24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 28usize, 22usize, 2149416u32);
    emu.adr_no_count(23usize, 29usize, 5usize, 2149420u32);
    emu.sltru_no_count(14usize, 21usize, 28usize, 2149424u32);
    emu.sltru_no_count(5usize, 23usize, 29usize, 2149428u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2149432u32);
    emu.adr_no_count(22usize, 7usize, 5usize, 2149436u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2149440u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2149448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc48));
    } else {
        emu.pc = 2149444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc44));
    }
}
#[inline(always)]
pub fn block_0x0020cc44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 22usize, 6usize, 2149448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc48));
}
#[inline(always)]
pub fn block_0x0020cc48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 23usize, 21usize, 2149452u32);
    emu.sltru_no_count(29usize, 28usize, 23usize, 2149456u32);
    emu.adr_no_count(7usize, 22usize, 20usize, 2149460u32);
    emu.adr_no_count(7usize, 7usize, 29usize, 2149464u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2149472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc60));
    } else {
        emu.pc = 2149468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc5c));
    }
}
#[inline(always)]
pub fn block_0x0020cc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 22usize, 2149472u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149472u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc60));
}
#[inline(never)]
pub fn block_0x0020cc60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 32u32, 2149476u32)?;
    emu.lw_no_count(11usize, 11usize, 36u32, 2149480u32)?;
    emu.ani_no_count(14usize, 19usize, 4294967294u32, 2149484u32);
    emu.mulhu_no_count(15usize, 6usize, 17usize, 2149488u32);
    emu.mul_no_count(20usize, 11usize, 17usize, 2149492u32);
    emu.mulhu_no_count(21usize, 11usize, 17usize, 2149496u32);
    emu.mul_no_count(22usize, 6usize, 16usize, 2149500u32);
    emu.mulhu_no_count(23usize, 6usize, 16usize, 2149504u32);
    emu.mul_no_count(17usize, 6usize, 17usize, 2149508u32);
    emu.adr_no_count(17usize, 14usize, 17usize, 2149512u32);
    emu.sltru_no_count(19usize, 17usize, 14usize, 2149516u32);
    emu.mul_no_count(14usize, 11usize, 16usize, 2149520u32);
    emu.mulhu_no_count(16usize, 11usize, 16usize, 2149524u32);
    emu.adr_no_count(15usize, 20usize, 15usize, 2149528u32);
    emu.sltru_no_count(17usize, 15usize, 20usize, 2149532u32);
    emu.adr_no_count(15usize, 22usize, 15usize, 2149536u32);
    emu.adr_no_count(21usize, 21usize, 17usize, 2149540u32);
    emu.sltru_no_count(17usize, 15usize, 22usize, 2149544u32);
    emu.adr_no_count(15usize, 15usize, 19usize, 2149548u32);
    emu.adr_no_count(17usize, 23usize, 17usize, 2149552u32);
    emu.adr_no_count(20usize, 21usize, 17usize, 2149556u32);
    emu.adr_no_count(17usize, 14usize, 20usize, 2149560u32);
    emu.sltru_no_count(20usize, 20usize, 21usize, 2149564u32);
    emu.sltru_no_count(14usize, 17usize, 14usize, 2149568u32);
    emu.adr_no_count(20usize, 16usize, 20usize, 2149572u32);
    emu.adr_no_count(16usize, 9usize, 15usize, 2149576u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2149580u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2149588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccd4));
    } else {
        emu.pc = 2149584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccd0));
    }
}
#[inline(always)]
pub fn block_0x0020ccd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(19usize, 16usize, 9usize, 2149588u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccd4));
}
#[inline(never)]
pub fn block_0x0020ccd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 17usize, 19usize, 2149592u32);
    emu.mulhu_no_count(14usize, 6usize, 12usize, 2149596u32);
    emu.mul_no_count(15usize, 11usize, 12usize, 2149600u32);
    emu.mulhu_no_count(9usize, 11usize, 12usize, 2149604u32);
    emu.mul_no_count(19usize, 6usize, 13usize, 2149608u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2149612u32);
    emu.adr_no_count(17usize, 20usize, 17usize, 2149616u32);
    emu.mulhu_no_count(20usize, 6usize, 13usize, 2149620u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2149624u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2149628u32);
    emu.adr_no_count(15usize, 9usize, 15usize, 2149632u32);
    emu.mul_no_count(9usize, 11usize, 13usize, 2149636u32);
    emu.mulhu_no_count(21usize, 11usize, 13usize, 2149640u32);
    emu.mul_no_count(13usize, 6usize, 12usize, 2149644u32);
    emu.adr_no_count(13usize, 18usize, 13usize, 2149648u32);
    emu.adr_no_count(14usize, 19usize, 14usize, 2149652u32);
    emu.sltru_no_count(12usize, 13usize, 18usize, 2149656u32);
    emu.sltru_no_count(18usize, 14usize, 19usize, 2149660u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2149664u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2149668u32);
    emu.adr_no_count(19usize, 15usize, 18usize, 2149672u32);
    emu.adr_no_count(18usize, 9usize, 19usize, 2149676u32);
    emu.sltru_no_count(15usize, 19usize, 15usize, 2149680u32);
    emu.sltru_no_count(19usize, 18usize, 9usize, 2149684u32);
    emu.adr_no_count(15usize, 21usize, 15usize, 2149688u32);
    emu.adr_no_count(9usize, 8usize, 14usize, 2149692u32);
    emu.adr_no_count(19usize, 15usize, 19usize, 2149696u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2149704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd48));
    } else {
        emu.pc = 2149700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd44));
    }
}
#[inline(always)]
pub fn block_0x0020cd44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 9usize, 8usize, 2149704u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd48));
}
#[inline(always)]
pub fn block_0x0020cd48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 12usize, 2149708u32);
    emu.adr_no_count(14usize, 9usize, 17usize, 2149712u32);
    emu.adr_no_count(12usize, 13usize, 16usize, 2149716u32);
    emu.sltru_no_count(16usize, 8usize, 18usize, 2149720u32);
    emu.sltru_no_count(17usize, 12usize, 13usize, 2149724u32);
    emu.adr_no_count(13usize, 14usize, 17usize, 2149728u32);
    emu.adr_no_count(16usize, 19usize, 16usize, 2149732u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2149740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd6c));
    } else {
        emu.pc = 2149736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd68));
    }
}
#[inline(always)]
pub fn block_0x0020cd68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 13usize, 9usize, 2149740u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd6c));
}
#[inline(never)]
pub fn block_0x0020cd6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 8usize, 17usize, 2149744u32);
    emu.sbr_no_count(14usize, 0usize, 11usize, 2149748u32);
    emu.mulhu_no_count(9usize, 6usize, 1usize, 2149752u32);
    emu.mulhu_no_count(18usize, 11usize, 1usize, 2149756u32);
    emu.adr_no_count(19usize, 6usize, 6usize, 2149760u32);
    emu.adi_no_count(22usize, 0usize, 4294967294u32, 2149764u32);
    emu.mulhu_no_count(20usize, 6usize, 22usize, 2149768u32);
    emu.sltru_no_count(15usize, 17usize, 8usize, 2149772u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2149776u32);
    emu.adr_no_count(21usize, 11usize, 11usize, 2149780u32);
    emu.mulhu_no_count(22usize, 11usize, 22usize, 2149784u32);
    emu.sbr_no_count(8usize, 9usize, 11usize, 2149788u32);
    emu.sltru_no_count(16usize, 8usize, 14usize, 2149792u32);
    emu.adr_no_count(16usize, 18usize, 16usize, 2149796u32);
    emu.sbr_no_count(14usize, 0usize, 19usize, 2149800u32);
    emu.sbr_no_count(19usize, 8usize, 19usize, 2149804u32);
    emu.sltru_no_count(14usize, 19usize, 14usize, 2149808u32);
    emu.adr_no_count(14usize, 20usize, 14usize, 2149812u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2149816u32);
    emu.sltru_no_count(9usize, 14usize, 16usize, 2149820u32);
    emu.adr_no_count(22usize, 22usize, 9usize, 2149824u32);
    emu.sbr_no_count(9usize, 31usize, 6usize, 2149828u32);
    emu.sbr_no_count(18usize, 14usize, 21usize, 2149832u32);
    emu.sbr_no_count(20usize, 0usize, 21usize, 2149836u32);
    emu.sltru_no_count(14usize, 9usize, 31usize, 2149840u32);
    emu.adr_no_count(31usize, 19usize, 14usize, 2149844u32);
    emu.sltru_no_count(19usize, 18usize, 20usize, 2149848u32);
    emu.adr_no_count(31usize, 30usize, 31usize, 2149852u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2149856u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2149864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cde8));
    } else {
        emu.pc = 2149860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cde4));
    }
}
#[inline(always)]
pub fn block_0x0020cde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 30usize, 2149864u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149864u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cde8));
}
#[inline(always)]
pub fn block_0x0020cde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 18usize, 14usize, 2149868u32);
    emu.adr_no_count(15usize, 31usize, 15usize, 2149872u32);
    emu.adr_no_count(14usize, 9usize, 17usize, 2149876u32);
    emu.sltru_no_count(18usize, 30usize, 18usize, 2149880u32);
    emu.sltru_no_count(17usize, 14usize, 9usize, 2149884u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2149888u32);
    emu.adr_no_count(9usize, 19usize, 18usize, 2149892u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce0c));
    } else {
        emu.pc = 2149896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce08));
    }
}
#[inline(always)]
pub fn block_0x0020ce08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 15usize, 31usize, 2149900u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce0c));
}
#[inline]
pub fn block_0x0020ce0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 30usize, 17usize, 2149904u32);
    emu.sltru_no_count(30usize, 17usize, 30usize, 2149908u32);
    emu.adr_no_count(9usize, 9usize, 30usize, 2149912u32);
    emu.sbr_no_count(30usize, 28usize, 6usize, 2149916u32);
    emu.adr_no_count(8usize, 7usize, 8usize, 2149920u32);
    emu.sltru_no_count(31usize, 30usize, 28usize, 2149924u32);
    emu.adr_no_count(28usize, 8usize, 31usize, 2149928u32);
    emu.adr_no_count(29usize, 5usize, 29usize, 2149932u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2149940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce34));
    } else {
        emu.pc = 2149936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce30));
    }
}
#[inline(always)]
pub fn block_0x0020ce30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 28usize, 7usize, 2149940u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce34));
}
#[inline(always)]
pub fn block_0x0020ce34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 29usize, 5usize, 2149944u32);
    emu.adr_no_count(31usize, 16usize, 31usize, 2149948u32);
    emu.adr_no_count(5usize, 28usize, 9usize, 2149952u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2149956u32);
    emu.sltru_no_count(8usize, 17usize, 30usize, 2149960u32);
    emu.adr_no_count(5usize, 5usize, 8usize, 2149964u32);
    emu.sltru_no_count(30usize, 31usize, 16usize, 2149968u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2149976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce58));
    } else {
        emu.pc = 2149972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce54));
    }
}
#[inline(always)]
pub fn block_0x0020ce54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 5usize, 28usize, 2149976u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149976u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce58));
}
#[inline(always)]
pub fn block_0x0020ce58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 31usize, 8usize, 2149980u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2149984u32);
    emu.sltru_no_count(31usize, 28usize, 31usize, 2149988u32);
    emu.sltru_no_count(16usize, 6usize, 29usize, 2149992u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2149996u32);
    emu.adr_no_count(29usize, 11usize, 16usize, 2150000u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2150004u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2150012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce7c));
    } else {
        emu.pc = 2150008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce78));
    }
}
#[inline(always)]
pub fn block_0x0020ce78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 29usize, 7usize, 2150012u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150012u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce7c));
}
#[inline(always)]
pub fn block_0x0020ce7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 6usize, 28usize, 2150016u32);
    emu.sltru_no_count(7usize, 11usize, 6usize, 2150020u32);
    emu.adr_no_count(6usize, 29usize, 30usize, 2150024u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2150028u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2150036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce94));
    } else {
        emu.pc = 2150032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce90));
    }
}
#[inline(always)]
pub fn block_0x0020ce90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 6usize, 29usize, 2150036u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150036u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ce94));
}
#[inline(never)]
pub fn block_0x0020ce94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 16usize, 7usize, 2150040u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2150044u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2150048u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2150052u32)?;
    emu.sw_no_count(15usize, 10usize, 12u32, 2150056u32)?;
    emu.sw_no_count(17usize, 10usize, 16u32, 2150060u32)?;
    emu.sw_no_count(5usize, 10usize, 20u32, 2150064u32)?;
    emu.sw_no_count(11usize, 10usize, 24u32, 2150068u32)?;
    emu.sw_no_count(6usize, 10usize, 28u32, 2150072u32)?;
    emu.sltru_no_count(11usize, 7usize, 16usize, 2150076u32);
    emu.sw_no_count(7usize, 10usize, 32u32, 2150080u32)?;
    emu.sw_no_count(11usize, 10usize, 36u32, 2150084u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2150088u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2150092u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2150096u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2150100u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2150104u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2150108u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2150112u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2150116u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2150120u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2150124u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2150128u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2150132u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2150136u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2150140u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150144u32;
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
pub fn block_0x0020cf00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967040u32, 2150148u32);
    emu.sw_no_count(1usize, 2usize, 252u32, 2150152u32)?;
    emu.sw_no_count(8usize, 2usize, 248u32, 2150156u32)?;
    emu.sw_no_count(9usize, 2usize, 244u32, 2150160u32)?;
    emu.sw_no_count(18usize, 2usize, 240u32, 2150164u32)?;
    emu.sw_no_count(19usize, 2usize, 236u32, 2150168u32)?;
    emu.sw_no_count(20usize, 2usize, 232u32, 2150172u32)?;
    emu.sw_no_count(21usize, 2usize, 228u32, 2150176u32)?;
    emu.sw_no_count(22usize, 2usize, 224u32, 2150180u32)?;
    emu.sw_no_count(23usize, 2usize, 220u32, 2150184u32)?;
    emu.sw_no_count(24usize, 2usize, 216u32, 2150188u32)?;
    emu.sw_no_count(25usize, 2usize, 212u32, 2150192u32)?;
    emu.sw_no_count(26usize, 2usize, 208u32, 2150196u32)?;
    emu.sw_no_count(27usize, 2usize, 204u32, 2150200u32)?;
    emu.sw_no_count(10usize, 2usize, 64u32, 2150204u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2150208u32);
    emu.apc_no_count(1usize, 2150208u32, 24576u32, 2150212u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020cf48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 96u32, 2150220u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2150224u32)?;
    emu.lw_no_count(10usize, 2usize, 100u32, 2150228u32)?;
    emu.sw_no_count(10usize, 2usize, 80u32, 2150232u32)?;
    emu.lw_no_count(10usize, 2usize, 104u32, 2150236u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2150240u32)?;
    emu.lw_no_count(10usize, 2usize, 108u32, 2150244u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2150248u32)?;
    emu.lw_no_count(9usize, 2usize, 112u32, 2150252u32)?;
    emu.lw_no_count(10usize, 2usize, 116u32, 2150256u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2150260u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2150264u32)?;
    emu.lw_no_count(20usize, 2usize, 124u32, 2150268u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2150272u32)?;
    emu.lw_no_count(17usize, 2usize, 132u32, 2150276u32)?;
    emu.lw_no_count(10usize, 2usize, 136u32, 2150280u32)?;
    emu.lw_no_count(11usize, 2usize, 140u32, 2150284u32)?;
    emu.lw_no_count(12usize, 2usize, 144u32, 2150288u32)?;
    emu.lw_no_count(13usize, 2usize, 148u32, 2150292u32)?;
    emu.lw_no_count(14usize, 2usize, 152u32, 2150296u32)?;
    emu.lw_no_count(15usize, 2usize, 156u32, 2150300u32)?;
    emu.sw_no_count(8usize, 2usize, 160u32, 2150304u32)?;
    emu.sw_no_count(20usize, 2usize, 164u32, 2150308u32)?;
    emu.sw_no_count(16usize, 2usize, 52u32, 2150312u32)?;
    emu.sw_no_count(16usize, 2usize, 168u32, 2150316u32)?;
    emu.sw_no_count(17usize, 2usize, 48u32, 2150320u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2150324u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2150328u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2150332u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2150336u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2150340u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2150344u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2150348u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2150352u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2150356u32);
    emu.apc_no_count(1usize, 2150356u32, 4294963200u32, 2150360u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150364u32;
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
pub fn block_0x0020cfdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 2usize, 96u32, 2150368u32)?;
    emu.lw_no_count(5usize, 2usize, 100u32, 2150372u32)?;
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2150376u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2150380u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2150384u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2150388u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 10usize, 1361u32, 2150392u32);
    emu.adi_no_count(28usize, 11usize, 4294965954u32, 2150396u32);
    emu.adi_no_count(24usize, 12usize, 4294966916u32, 2150400u32);
    emu.adi_no_count(26usize, 14usize, 4294965933u32, 2150404u32);
    emu.mul_no_count(10usize, 5usize, 13usize, 2150408u32);
    emu.mulhu_no_count(11usize, 15usize, 13usize, 2150412u32);
    emu.mulhu_no_count(12usize, 5usize, 13usize, 2150416u32);
    emu.mul_no_count(14usize, 15usize, 28usize, 2150420u32);
    emu.mulhu_no_count(16usize, 15usize, 28usize, 2150424u32);
    emu.mul_no_count(17usize, 5usize, 28usize, 2150428u32);
    emu.mul_no_count(6usize, 5usize, 24usize, 2150432u32);
    emu.mulhu_no_count(7usize, 15usize, 24usize, 2150436u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2150440u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2150444u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2150448u32);
    emu.mulhu_no_count(12usize, 5usize, 24usize, 2150452u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2150456u32);
    emu.sltru_no_count(6usize, 7usize, 6usize, 2150460u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2150464u32);
    emu.mul_no_count(6usize, 15usize, 26usize, 2150468u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2150472u32);
    emu.sw_no_count(11usize, 2usize, 76u32, 2150476u32)?;
    emu.sltru_no_count(11usize, 11usize, 14usize, 2150480u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2150484u32);
    emu.mulhu_no_count(14usize, 15usize, 26usize, 2150488u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2150492u32);
    emu.sltru_no_count(16usize, 7usize, 6usize, 2150496u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2150500u32);
    emu.sw_no_count(28usize, 2usize, 92u32, 2150504u32)?;
    emu.mulhu_no_count(16usize, 5usize, 28usize, 2150508u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2150512u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2150516u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2150520u32);
    emu.mulhu_no_count(16usize, 5usize, 26usize, 2150524u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2150528u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2150532u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2150536u32);
    emu.mul_no_count(12usize, 5usize, 26usize, 2150540u32);
    emu.adr_no_count(6usize, 17usize, 11usize, 2150544u32);
    emu.sltru_no_count(11usize, 6usize, 17usize, 2150548u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2150552u32);
    emu.mul_no_count(28usize, 15usize, 24usize, 2150556u32);
    emu.adr_no_count(10usize, 12usize, 14usize, 2150560u32);
    emu.sltru_no_count(14usize, 10usize, 12usize, 2150564u32);
    emu.adr_no_count(28usize, 6usize, 28usize, 2150568u32);
    emu.sltru_no_count(12usize, 28usize, 6usize, 2150572u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2150576u32);
    emu.adr_no_count(23usize, 7usize, 12usize, 2150580u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2150584u32);
    emu.add_memory_rw_events(55usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2150592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0c0));
    } else {
        emu.pc = 2150588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0bc));
    }
}
#[inline(always)]
pub fn block_0x0020d0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 23usize, 11usize, 2150592u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d0c0));
}
#[inline(never)]
pub fn block_0x0020d0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 2usize, 36u32, 2150596u32)?;
    emu.sw_no_count(8usize, 2usize, 44u32, 2150600u32)?;
    emu.adr_no_count(12usize, 10usize, 12usize, 2150604u32);
    emu.sbr_no_count(11usize, 0usize, 5usize, 2150608u32);
    emu.adi_no_count(7usize, 0usize, 4294967295u32, 2150612u32);
    emu.sbr_no_count(29usize, 0usize, 15usize, 2150616u32);
    emu.sltru_no_count(10usize, 12usize, 10usize, 2150620u32);
    emu.mulhu_no_count(16usize, 15usize, 7usize, 2150624u32);
    emu.mulhu_no_count(6usize, 5usize, 7usize, 2150628u32);
    emu.sbr_no_count(18usize, 12usize, 15usize, 2150632u32);
    emu.adr_no_count(14usize, 14usize, 10usize, 2150636u32);
    emu.sbr_no_count(17usize, 16usize, 5usize, 2150640u32);
    emu.sltru_no_count(31usize, 18usize, 12usize, 2150644u32);
    emu.sbr_no_count(10usize, 17usize, 15usize, 2150648u32);
    emu.sltru_no_count(12usize, 10usize, 29usize, 2150652u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2150656u32);
    emu.sltru_no_count(16usize, 17usize, 11usize, 2150660u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2150664u32);
    emu.adr_no_count(10usize, 14usize, 10usize, 2150668u32);
    emu.adr_no_count(19usize, 10usize, 31usize, 2150672u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2150676u32);
    emu.sbr_no_count(10usize, 12usize, 5usize, 2150680u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2150684u32);
    emu.sltru_no_count(27usize, 10usize, 11usize, 2150688u32);
    emu.adr_no_count(11usize, 6usize, 12usize, 2150692u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2150700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d12c));
    } else {
        emu.pc = 2150696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d128));
    }
}
#[inline(always)]
pub fn block_0x0020d128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 19usize, 14usize, 2150700u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d12c));
}
#[inline(never)]
pub fn block_0x0020d12c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 11usize, 27usize, 2150704u32);
    emu.lw_no_count(5usize, 2usize, 104u32, 2150708u32)?;
    emu.lw_no_count(30usize, 2usize, 108u32, 2150712u32)?;
    emu.adr_no_count(31usize, 10usize, 31usize, 2150716u32);
    emu.sltru_no_count(14usize, 31usize, 10usize, 2150720u32);
    emu.mulhu_no_count(10usize, 5usize, 13usize, 2150724u32);
    emu.mul_no_count(11usize, 30usize, 13usize, 2150728u32);
    emu.mulhu_no_count(12usize, 30usize, 13usize, 2150732u32);
    emu.lw_no_count(9usize, 2usize, 92u32, 2150736u32)?;
    emu.mul_no_count(6usize, 5usize, 9usize, 2150740u32);
    emu.mulhu_no_count(8usize, 5usize, 9usize, 2150744u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2150748u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2150752u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2150756u32);
    emu.mul_no_count(12usize, 30usize, 9usize, 2150760u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2150764u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2150768u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2150772u32);
    emu.mulhu_no_count(8usize, 30usize, 9usize, 2150776u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2150780u32);
    emu.sltru_no_count(11usize, 6usize, 11usize, 2150784u32);
    emu.adr_no_count(8usize, 8usize, 11usize, 2150788u32);
    emu.mul_no_count(11usize, 5usize, 13usize, 2150792u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2150796u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2150800u32)?;
    emu.sltru_no_count(11usize, 11usize, 28usize, 2150804u32);
    emu.adr_no_count(28usize, 10usize, 11usize, 2150808u32);
    emu.adr_no_count(10usize, 12usize, 6usize, 2150812u32);
    emu.sltru_no_count(6usize, 10usize, 12usize, 2150816u32);
    emu.adr_no_count(28usize, 23usize, 28usize, 2150820u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2150824u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2150832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1b0));
    } else {
        emu.pc = 2150828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1ac));
    }
}
#[inline(always)]
pub fn block_0x0020d1ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 28usize, 23usize, 2150832u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d1b0));
}
#[inline(never)]
pub fn block_0x0020d1b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(28usize, 2usize, 4u32, 2150836u32)?;
    emu.adr_no_count(27usize, 27usize, 14usize, 2150840u32);
    emu.adr_no_count(23usize, 10usize, 11usize, 2150844u32);
    emu.mulhu_no_count(11usize, 5usize, 24usize, 2150848u32);
    emu.mul_no_count(12usize, 30usize, 24usize, 2150852u32);
    emu.mulhu_no_count(14usize, 30usize, 24usize, 2150856u32);
    emu.mul_no_count(28usize, 5usize, 26usize, 2150860u32);
    emu.sltru_no_count(10usize, 23usize, 10usize, 2150864u32);
    emu.adr_no_count(6usize, 6usize, 10usize, 2150868u32);
    emu.mulhu_no_count(10usize, 5usize, 26usize, 2150872u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2150876u32);
    emu.sltru_no_count(12usize, 11usize, 12usize, 2150880u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2150884u32);
    emu.mul_no_count(14usize, 30usize, 26usize, 2150888u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2150892u32);
    emu.sltru_no_count(28usize, 11usize, 28usize, 2150896u32);
    emu.adr_no_count(10usize, 10usize, 28usize, 2150900u32);
    emu.mulhu_no_count(28usize, 30usize, 26usize, 2150904u32);
    emu.adr_no_count(8usize, 12usize, 10usize, 2150908u32);
    emu.sltru_no_count(10usize, 8usize, 12usize, 2150912u32);
    emu.adr_no_count(9usize, 28usize, 10usize, 2150916u32);
    emu.mul_no_count(28usize, 5usize, 24usize, 2150920u32);
    emu.adr_no_count(28usize, 18usize, 28usize, 2150924u32);
    emu.sltru_no_count(10usize, 28usize, 18usize, 2150928u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2150932u32);
    emu.adr_no_count(12usize, 14usize, 8usize, 2150936u32);
    emu.sltru_no_count(8usize, 12usize, 14usize, 2150940u32);
    emu.adr_no_count(11usize, 19usize, 11usize, 2150944u32);
    emu.adr_no_count(8usize, 9usize, 8usize, 2150948u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2150956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d22c));
    } else {
        emu.pc = 2150952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d228));
    }
}
#[inline(always)]
pub fn block_0x0020d228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 11usize, 19usize, 2150956u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d22c));
}
#[inline]
pub fn block_0x0020d22c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 27usize, 15usize, 2150960u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2150964u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2150968u32);
    emu.adr_no_count(23usize, 28usize, 23usize, 2150972u32);
    emu.sltru_no_count(12usize, 10usize, 12usize, 2150976u32);
    emu.sltru_no_count(25usize, 23usize, 28usize, 2150980u32);
    emu.adr_no_count(18usize, 6usize, 25usize, 2150984u32);
    emu.adr_no_count(8usize, 8usize, 12usize, 2150988u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2150996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d254));
    } else {
        emu.pc = 2150992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d250));
    }
}
#[inline(always)]
pub fn block_0x0020d250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 18usize, 11usize, 2150996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2150996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d254));
}
#[inline]
pub fn block_0x0020d254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(28usize, 29usize, 1u32, 2151000u32);
    emu.adr_no_count(25usize, 10usize, 25usize, 2151004u32);
    emu.sbr_no_count(29usize, 0usize, 30usize, 2151008u32);
    emu.mulhu_no_count(11usize, 5usize, 7usize, 2151012u32);
    emu.mulhu_no_count(6usize, 30usize, 7usize, 2151016u32);
    emu.sbr_no_count(12usize, 0usize, 5usize, 2151020u32);
    emu.sltru_no_count(10usize, 25usize, 10usize, 2151024u32);
    emu.sbr_no_count(9usize, 11usize, 30usize, 2151028u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2151032u32);
    emu.sbr_no_count(8usize, 9usize, 5usize, 2151036u32);
    emu.sltru_no_count(12usize, 8usize, 12usize, 2151040u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2151044u32);
    emu.sbr_no_count(7usize, 31usize, 5usize, 2151048u32);
    emu.sltru_no_count(1usize, 7usize, 31usize, 2151052u32);
    emu.sltru_no_count(31usize, 9usize, 29usize, 2151056u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2151060u32);
    emu.adr_no_count(11usize, 8usize, 1usize, 2151064u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2151068u32);
    emu.adr_no_count(8usize, 31usize, 12usize, 2151072u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2151080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2a8));
    } else {
        emu.pc = 2151076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2a4));
    }
}
#[inline(always)]
pub fn block_0x0020d2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(1usize, 11usize, 14usize, 2151080u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d2a8));
}
#[inline]
pub fn block_0x0020d2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 40u32, 2151084u32)?;
    emu.sltru_no_count(12usize, 14usize, 27usize, 2151088u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2151092u32);
    emu.sbr_no_count(9usize, 8usize, 30usize, 2151096u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2151100u32);
    emu.adr_no_count(25usize, 7usize, 25usize, 2151104u32);
    emu.sltru_no_count(22usize, 25usize, 7usize, 2151108u32);
    emu.adr_no_count(19usize, 10usize, 22usize, 2151112u32);
    emu.sltru_no_count(14usize, 8usize, 31usize, 2151116u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2151124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2d4));
    } else {
        emu.pc = 2151120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2d0));
    }
}
#[inline(always)]
pub fn block_0x0020d2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 19usize, 11usize, 2151124u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d2d4));
}
#[inline(never)]
pub fn block_0x0020d2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 33u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(7usize, 28usize, 12usize, 2151128u32);
    emu.sltru_no_count(20usize, 9usize, 29usize, 2151132u32);
    emu.lw_no_count(12usize, 2usize, 112u32, 2151136u32)?;
    emu.lw_no_count(27usize, 2usize, 116u32, 2151140u32)?;
    emu.adr_no_count(14usize, 6usize, 14usize, 2151144u32);
    emu.adr_no_count(1usize, 9usize, 1usize, 2151148u32);
    emu.mulhu_no_count(10usize, 12usize, 13usize, 2151152u32);
    emu.mul_no_count(11usize, 27usize, 13usize, 2151156u32);
    emu.mulhu_no_count(6usize, 27usize, 13usize, 2151160u32);
    emu.lw_no_count(30usize, 2usize, 92u32, 2151164u32)?;
    emu.mul_no_count(28usize, 12usize, 30usize, 2151168u32);
    emu.mulhu_no_count(29usize, 12usize, 30usize, 2151172u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2151176u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2151180u32);
    emu.adr_no_count(11usize, 6usize, 11usize, 2151184u32);
    emu.mul_no_count(6usize, 27usize, 30usize, 2151188u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2151192u32);
    emu.sltru_no_count(28usize, 10usize, 28usize, 2151196u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2151200u32);
    emu.mulhu_no_count(29usize, 27usize, 30usize, 2151204u32);
    emu.adr_no_count(28usize, 11usize, 28usize, 2151208u32);
    emu.sltru_no_count(11usize, 28usize, 11usize, 2151212u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2151216u32);
    emu.mul_no_count(29usize, 12usize, 13usize, 2151220u32);
    emu.adr_no_count(29usize, 23usize, 29usize, 2151224u32);
    emu.sw_no_count(29usize, 2usize, 0u32, 2151228u32)?;
    emu.sltru_no_count(8usize, 29usize, 23usize, 2151232u32);
    emu.adr_no_count(30usize, 10usize, 8usize, 2151236u32);
    emu.adr_no_count(10usize, 6usize, 28usize, 2151240u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2151244u32);
    emu.adr_no_count(30usize, 18usize, 30usize, 2151248u32);
    emu.adr_no_count(11usize, 11usize, 6usize, 2151252u32);
    emu.add_memory_rw_events(32usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2151260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d35c));
    } else {
        emu.pc = 2151256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d358));
    }
}
#[inline(always)]
pub fn block_0x0020d358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 30usize, 18usize, 2151260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d35c));
}
#[inline(never)]
pub fn block_0x0020d35c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 17usize, 7usize, 2151264u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2151268u32);
    emu.sltru_no_count(14usize, 1usize, 9usize, 2151272u32);
    emu.adr_no_count(23usize, 1usize, 22usize, 2151276u32);
    emu.adr_no_count(8usize, 10usize, 8usize, 2151280u32);
    emu.mulhu_no_count(28usize, 12usize, 24usize, 2151284u32);
    emu.mul_no_count(29usize, 27usize, 24usize, 2151288u32);
    emu.mulhu_no_count(31usize, 27usize, 24usize, 2151292u32);
    emu.mul_no_count(9usize, 12usize, 26usize, 2151296u32);
    emu.mulhu_no_count(21usize, 12usize, 26usize, 2151300u32);
    emu.sltru_no_count(7usize, 8usize, 10usize, 2151304u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2151308u32);
    emu.mul_no_count(11usize, 27usize, 26usize, 2151312u32);
    emu.adr_no_count(10usize, 29usize, 28usize, 2151316u32);
    emu.sltru_no_count(28usize, 10usize, 29usize, 2151320u32);
    emu.adr_no_count(31usize, 31usize, 28usize, 2151324u32);
    emu.mul_no_count(28usize, 12usize, 24usize, 2151328u32);
    emu.adr_no_count(28usize, 25usize, 28usize, 2151332u32);
    emu.adr_no_count(10usize, 9usize, 10usize, 2151336u32);
    emu.sltru_no_count(18usize, 28usize, 25usize, 2151340u32);
    emu.sltru_no_count(29usize, 10usize, 9usize, 2151344u32);
    emu.adr_no_count(10usize, 10usize, 18usize, 2151348u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2151352u32);
    emu.adr_no_count(9usize, 19usize, 10usize, 2151356u32);
    emu.adr_no_count(29usize, 31usize, 29usize, 2151360u32);
    emu.adr_no_count(10usize, 11usize, 29usize, 2151364u32);
    emu.sltru_no_count(31usize, 29usize, 31usize, 2151368u32);
    emu.mulhu_no_count(29usize, 27usize, 26usize, 2151372u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2151380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3d4));
    } else {
        emu.pc = 2151376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3d0));
    }
}
#[inline(always)]
pub fn block_0x0020d3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 9usize, 19usize, 2151380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d3d4));
}
#[inline]
pub fn block_0x0020d3d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(26usize, 2usize, 56u32, 2151384u32)?;
    emu.sw_no_count(24usize, 2usize, 60u32, 2151388u32)?;
    emu.sltru_no_count(21usize, 10usize, 11usize, 2151392u32);
    emu.adr_no_count(31usize, 29usize, 31usize, 2151396u32);
    emu.sltru_no_count(19usize, 6usize, 17usize, 2151400u32);
    emu.adr_no_count(11usize, 20usize, 14usize, 2151404u32);
    emu.sltru_no_count(29usize, 23usize, 1usize, 2151408u32);
    emu.adr_no_count(7usize, 9usize, 7usize, 2151412u32);
    emu.adr_no_count(14usize, 28usize, 8usize, 2151416u32);
    emu.sltru_no_count(25usize, 14usize, 28usize, 2151420u32);
    emu.adr_no_count(7usize, 7usize, 25usize, 2151424u32);
    emu.adr_no_count(17usize, 10usize, 18usize, 2151428u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2151436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d40c));
    } else {
        emu.pc = 2151432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d408));
    }
}
#[inline(always)]
pub fn block_0x0020d408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 7usize, 9usize, 2151436u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151436u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d40c));
}
#[inline(never)]
pub fn block_0x0020d40c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 31usize, 21usize, 2151440u32);
    emu.sltru_no_count(20usize, 17usize, 10usize, 2151444u32);
    emu.mul_no_count(10usize, 15usize, 13usize, 2151448u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2151452u32)?;
    emu.adr_no_count(16usize, 16usize, 19usize, 2151456u32);
    emu.lw_no_count(24usize, 2usize, 120u32, 2151460u32)?;
    emu.lw_no_count(26usize, 2usize, 124u32, 2151464u32)?;
    emu.adr_no_count(9usize, 11usize, 29usize, 2151468u32);
    emu.adr_no_count(25usize, 17usize, 25usize, 2151472u32);
    emu.mulhu_no_count(10usize, 24usize, 13usize, 2151476u32);
    emu.mul_no_count(11usize, 26usize, 13usize, 2151480u32);
    emu.mulhu_no_count(15usize, 26usize, 13usize, 2151484u32);
    emu.lw_no_count(22usize, 2usize, 92u32, 2151488u32)?;
    emu.mul_no_count(28usize, 24usize, 22usize, 2151492u32);
    emu.mulhu_no_count(29usize, 24usize, 22usize, 2151496u32);
    emu.adr_no_count(31usize, 11usize, 10usize, 2151500u32);
    emu.sltru_no_count(10usize, 31usize, 11usize, 2151504u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2151508u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2151512u32)?;
    emu.mul_no_count(18usize, 24usize, 13usize, 2151516u32);
    emu.adr_no_count(18usize, 14usize, 18usize, 2151520u32);
    emu.adr_no_count(31usize, 28usize, 31usize, 2151524u32);
    emu.sltru_no_count(21usize, 18usize, 14usize, 2151528u32);
    emu.sltru_no_count(11usize, 31usize, 28usize, 2151532u32);
    emu.adr_no_count(31usize, 31usize, 21usize, 2151536u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2151540u32);
    emu.adr_no_count(19usize, 7usize, 31usize, 2151544u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2151548u32);
    emu.mul_no_count(14usize, 26usize, 22usize, 2151552u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2151560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d488));
    } else {
        emu.pc = 2151556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d484));
    }
}
#[inline(always)]
pub fn block_0x0020d484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 19usize, 7usize, 2151560u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d488));
}
#[inline(never)]
pub fn block_0x0020d488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 8usize, 20usize, 2151564u32);
    emu.sltru_no_count(28usize, 25usize, 17usize, 2151568u32);
    emu.adr_no_count(20usize, 14usize, 11usize, 2151572u32);
    emu.sltru_no_count(1usize, 11usize, 10usize, 2151576u32);
    emu.lw_no_count(10usize, 2usize, 92u32, 2151580u32)?;
    emu.mulhu_no_count(22usize, 26usize, 10usize, 2151584u32);
    let a = 0u32.wrapping_add(1491623936u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2151588u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2151592u32;
    emu.update_insn_clock();
    emu.lw_no_count(8usize, 2usize, 128u32, 2151596u32)?;
    emu.lw_no_count(10usize, 2usize, 132u32, 2151600u32)?;
    let a = 0u32.wrapping_add(60612608u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2151604u32;
    emu.update_insn_clock();
    emu.adr_no_count(7usize, 6usize, 23usize, 2151608u32);
    emu.lw_no_count(23usize, 2usize, 72u32, 2151612u32)?;
    emu.lw_no_count(11usize, 2usize, 68u32, 2151616u32)?;
    emu.sltru_no_count(23usize, 23usize, 11usize, 2151620u32);
    emu.adi_no_count(11usize, 17usize, 380u32, 2151624u32);
    emu.adi_no_count(17usize, 29usize, 1362u32, 2151628u32);
    emu.adi_no_count(13usize, 31usize, 4294965935u32, 2151632u32);
    emu.sltru_no_count(29usize, 7usize, 6usize, 2151636u32);
    emu.mulhu_no_count(31usize, 24usize, 11usize, 2151640u32);
    emu.mul_no_count(6usize, 26usize, 11usize, 2151644u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2151648u32);
    emu.sw_no_count(17usize, 2usize, 20u32, 2151652u32)?;
    emu.mul_no_count(26usize, 24usize, 17usize, 2151656u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2151660u32)?;
    emu.mul_no_count(24usize, 24usize, 11usize, 2151664u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2151668u32);
    emu.adi_no_count(6usize, 23usize, 0u32, 2151672u32);
    emu.lw_no_count(9usize, 2usize, 80u32, 2151676u32)?;
    emu.lw_no_count(17usize, 2usize, 76u32, 2151680u32)?;
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2151696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d510));
    } else {
        emu.pc = 2151684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d504));
    }
}
#[inline(always)]
pub fn block_0x0020d504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 2usize, 80u32, 2151688u32)?;
    emu.lw_no_count(9usize, 2usize, 76u32, 2151692u32)?;
    emu.sltru_no_count(6usize, 6usize, 9usize, 2151696u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2151696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d510));
}
#[inline]
pub fn block_0x0020d510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 15usize, 28usize, 2151700u32);
    emu.sltru_no_count(14usize, 20usize, 14usize, 2151704u32);
    emu.adr_no_count(1usize, 22usize, 1usize, 2151708u32);
    emu.adr_no_count(21usize, 20usize, 21usize, 2151712u32);
    emu.adr_no_count(9usize, 26usize, 31usize, 2151716u32);
    emu.mulhu_no_count(26usize, 8usize, 13usize, 2151720u32);
    emu.sw_no_count(13usize, 2usize, 84u32, 2151724u32)?;
    emu.mul_no_count(15usize, 10usize, 13usize, 2151728u32);
    let a = 0u32.wrapping_add(205926400u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2151732u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1341u32, 2151736u32);
    emu.sw_no_count(10usize, 2usize, 88u32, 2151740u32)?;
    emu.adr_no_count(27usize, 5usize, 27usize, 2151744u32);
    emu.adr_no_count(31usize, 12usize, 24usize, 2151748u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2151752u32);
    emu.lw_no_count(24usize, 2usize, 28u32, 2151756u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2151760u32)?;
    emu.sltru_no_count(22usize, 24usize, 13usize, 2151764u32);
    emu.adr_no_count(25usize, 7usize, 25usize, 2151768u32);
    emu.adi_no_count(29usize, 22usize, 0u32, 2151772u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2151776u32)?;
    emu.lw_no_count(17usize, 2usize, 4u32, 2151780u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2151788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d56c));
    } else {
        emu.pc = 2151784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d568));
    }
}
#[inline(always)]
pub fn block_0x0020d568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 17usize, 2151788u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d56c));
}
#[inline(never)]
pub fn block_0x0020d56c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 1usize, 14usize, 2151792u32);
    emu.adi_no_count(1usize, 10usize, 0u32, 2151796u32);
    emu.sltru_no_count(10usize, 21usize, 20usize, 2151800u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2151804u32);
    emu.lw_no_count(11usize, 2usize, 88u32, 2151808u32)?;
    emu.mul_no_count(11usize, 8usize, 11usize, 2151812u32);
    emu.lw_no_count(5usize, 2usize, 84u32, 2151816u32)?;
    emu.mul_no_count(5usize, 8usize, 5usize, 2151820u32);
    emu.adr_no_count(8usize, 27usize, 9usize, 2151824u32);
    emu.sltru_no_count(20usize, 31usize, 12usize, 2151828u32);
    emu.adr_no_count(9usize, 16usize, 28usize, 2151832u32);
    emu.sltru_no_count(7usize, 25usize, 7usize, 2151836u32);
    emu.sbr_no_count(12usize, 0usize, 29usize, 2151840u32);
    emu.sbr_no_count(16usize, 1usize, 17usize, 2151844u32);
    emu.sbr_no_count(26usize, 24usize, 13usize, 2151848u32);
    emu.adr_no_count(29usize, 6usize, 29usize, 2151852u32);
    emu.sbr_no_count(24usize, 16usize, 22usize, 2151856u32);
    emu.sbr_no_count(22usize, 0usize, 29usize, 2151860u32);
    emu.sbr_no_count(28usize, 26usize, 6usize, 2151864u32);
    emu.sltru_no_count(16usize, 22usize, 12usize, 2151868u32);
    emu.sltru_no_count(26usize, 28usize, 26usize, 2151872u32);
    emu.sbr_no_count(12usize, 6usize, 26usize, 2151876u32);
    emu.sbr_no_count(12usize, 24usize, 12usize, 2151880u32);
    emu.sbr_no_count(16usize, 16usize, 29usize, 2151884u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2151888u32)?;
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2151896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d5d8));
    } else {
        emu.pc = 2151892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d5d4));
    }
}
#[inline(always)]
pub fn block_0x0020d5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 12usize, 24usize, 2151896u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d5d8));
}
#[inline]
pub fn block_0x0020d5d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 14usize, 10usize, 2151900u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2151904u32);
    emu.adr_no_count(8usize, 8usize, 20usize, 2151908u32);
    emu.adr_no_count(5usize, 31usize, 5usize, 2151912u32);
    emu.adr_no_count(15usize, 9usize, 7usize, 2151916u32);
    emu.adr_no_count(6usize, 25usize, 21usize, 2151920u32);
    emu.sbr_no_count(11usize, 26usize, 29usize, 2151924u32);
    emu.sltru_no_count(11usize, 11usize, 22usize, 2151928u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2151932u32);
    emu.lw_no_count(9usize, 2usize, 36u32, 2151936u32)?;
    emu.lw_no_count(17usize, 2usize, 0u32, 2151940u32)?;
    emu.sltru_no_count(29usize, 9usize, 17usize, 2151944u32);
    emu.sai_no_count(7usize, 11usize, 1055u32, 2151948u32);
    emu.adi_no_count(16usize, 29usize, 0u32, 2151952u32);
    emu.lw_no_count(20usize, 2usize, 32u32, 2151956u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2151964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d61c));
    } else {
        emu.pc = 2151960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d618));
    }
}
#[inline(always)]
pub fn block_0x0020d618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 30usize, 2151964u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2151964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d61c));
}
#[inline]
pub fn block_0x0020d61c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 8usize, 10usize, 2151968u32);
    emu.sltru_no_count(11usize, 5usize, 31usize, 2151972u32);
    emu.sltru_no_count(14usize, 6usize, 25usize, 2151976u32);
    emu.adr_no_count(15usize, 15usize, 24usize, 2151980u32);
    emu.sbr_no_count(8usize, 0usize, 16usize, 2151984u32);
    emu.sbr_no_count(30usize, 20usize, 30usize, 2151988u32);
    emu.sbr_no_count(9usize, 9usize, 17usize, 2151992u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2151996u32);
    emu.sbr_no_count(31usize, 30usize, 29usize, 2152000u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2152004u32);
    emu.adr_no_count(30usize, 9usize, 7usize, 2152008u32);
    emu.sltru_no_count(29usize, 30usize, 9usize, 2152012u32);
    emu.adr_no_count(7usize, 7usize, 29usize, 2152016u32);
    emu.adr_no_count(24usize, 31usize, 7usize, 2152020u32);
    emu.adr_no_count(7usize, 16usize, 8usize, 2152024u32);
    emu.lw_no_count(25usize, 2usize, 56u32, 2152028u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2152032u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2152036u32)?;
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2152044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d66c));
    } else {
        emu.pc = 2152040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d668));
    }
}
#[inline(always)]
pub fn block_0x0020d668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 24usize, 31usize, 2152044u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d66c));
}
#[inline]
pub fn block_0x0020d66c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 11usize, 2152048u32);
    emu.adr_no_count(15usize, 15usize, 14usize, 2152052u32);
    emu.lw_no_count(10usize, 2usize, 80u32, 2152056u32)?;
    emu.lw_no_count(14usize, 2usize, 76u32, 2152060u32)?;
    emu.sbr_no_count(10usize, 10usize, 14usize, 2152064u32);
    emu.adr_no_count(29usize, 16usize, 29usize, 2152068u32);
    emu.sltru_no_count(14usize, 29usize, 16usize, 2152072u32);
    emu.adr_no_count(7usize, 7usize, 14usize, 2152076u32);
    emu.lw_no_count(9usize, 2usize, 44u32, 2152080u32)?;
    emu.sltru_no_count(14usize, 9usize, 18usize, 2152084u32);
    emu.sai_no_count(29usize, 7usize, 1055u32, 2152088u32);
    emu.adi_no_count(31usize, 14usize, 0u32, 2152092u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2152100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6a4));
    } else {
        emu.pc = 2152096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6a0));
    }
}
#[inline(always)]
pub fn block_0x0020d6a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 20usize, 19usize, 2152100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d6a4));
}
#[inline]
pub fn block_0x0020d6a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 15usize, 2152104u32);
    emu.sltru_no_count(7usize, 5usize, 6usize, 2152108u32);
    emu.sbr_no_count(16usize, 5usize, 6usize, 2152112u32);
    emu.sbr_no_count(15usize, 10usize, 23usize, 2152116u32);
    emu.lw_no_count(10usize, 2usize, 72u32, 2152120u32)?;
    emu.lw_no_count(17usize, 2usize, 68u32, 2152124u32)?;
    emu.sbr_no_count(10usize, 10usize, 17usize, 2152128u32);
    emu.sbr_no_count(8usize, 0usize, 31usize, 2152132u32);
    emu.sbr_no_count(5usize, 20usize, 19usize, 2152136u32);
    emu.sbr_no_count(9usize, 9usize, 18usize, 2152140u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2152144u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2152148u32);
    emu.sltru_no_count(8usize, 6usize, 8usize, 2152152u32);
    emu.adr_no_count(14usize, 9usize, 29usize, 2152156u32);
    emu.sltru_no_count(31usize, 14usize, 9usize, 2152160u32);
    emu.adr_no_count(29usize, 29usize, 31usize, 2152164u32);
    emu.adr_no_count(18usize, 5usize, 29usize, 2152168u32);
    emu.adr_no_count(29usize, 6usize, 8usize, 2152172u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2152180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6f4));
    } else {
        emu.pc = 2152176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d6f0));
    }
}
#[inline(always)]
pub fn block_0x0020d6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 18usize, 5usize, 2152180u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d6f4));
}
#[inline]
pub fn block_0x0020d6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 7usize, 2152184u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2152188u32);
    emu.lw_no_count(5usize, 2usize, 84u32, 2152192u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2152196u32);
    emu.sltru_no_count(6usize, 31usize, 6usize, 2152200u32);
    emu.sltru_no_count(10usize, 5usize, 10usize, 2152204u32);
    emu.adr_no_count(29usize, 29usize, 6usize, 2152208u32);
    emu.lw_no_count(6usize, 2usize, 88u32, 2152212u32)?;
    emu.adr_no_count(6usize, 10usize, 6usize, 2152216u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2152220u32);
    emu.lw_no_count(31usize, 2usize, 52u32, 2152224u32)?;
    emu.adr_no_count(16usize, 31usize, 16usize, 2152228u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2152232u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2152240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d730));
    } else {
        emu.pc = 2152236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d72c));
    }
}
#[inline(always)]
pub fn block_0x0020d72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 15usize, 2152240u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d730));
}
#[inline]
pub fn block_0x0020d730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(7usize, 29usize, 1055u32, 2152244u32);
    emu.lw_no_count(15usize, 2usize, 48u32, 2152248u32)?;
    emu.adr_no_count(11usize, 15usize, 11usize, 2152252u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2152256u32);
    emu.adr_no_count(28usize, 15usize, 28usize, 2152260u32);
    emu.sltru_no_count(29usize, 28usize, 15usize, 2152264u32);
    emu.adr_no_count(10usize, 15usize, 29usize, 2152268u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2152272u32);
    emu.sltru_no_count(8usize, 16usize, 31usize, 2152276u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2152284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d75c));
    } else {
        emu.pc = 2152280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d758));
    }
}
#[inline(always)]
pub fn block_0x0020d758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 15usize, 2152284u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d75c));
}
#[inline]
pub fn block_0x0020d75c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 11usize, 8usize, 2152288u32);
    emu.adr_no_count(11usize, 15usize, 29usize, 2152292u32);
    emu.adr_no_count(29usize, 28usize, 26usize, 2152296u32);
    emu.sltru_no_count(31usize, 11usize, 15usize, 2152300u32);
    emu.sltru_no_count(12usize, 29usize, 28usize, 2152304u32);
    emu.adr_no_count(15usize, 15usize, 31usize, 2152308u32);
    emu.adr_no_count(31usize, 12usize, 23usize, 2152312u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2152316u32);
    emu.adr_no_count(28usize, 16usize, 7usize, 2152320u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2152328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d788));
    } else {
        emu.pc = 2152324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d784));
    }
}
#[inline(always)]
pub fn block_0x0020d784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 31usize, 10usize, 2152328u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d788));
}
#[inline]
pub fn block_0x0020d788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 8usize, 7usize, 2152332u32);
    emu.adi_no_count(10usize, 11usize, 4294967295u32, 2152336u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2152340u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2152344u32);
    emu.sbr_no_count(15usize, 15usize, 11usize, 2152348u32);
    emu.sltru_no_count(10usize, 12usize, 10usize, 2152352u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2152356u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2152360u32);
    emu.adr_no_count(30usize, 10usize, 30usize, 2152364u32);
    emu.sltru_no_count(11usize, 30usize, 10usize, 2152368u32);
    emu.adr_no_count(19usize, 10usize, 24usize, 2152372u32);
    emu.adr_no_count(19usize, 19usize, 11usize, 2152376u32);
    emu.sltru_no_count(15usize, 28usize, 16usize, 2152380u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2152388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d7c4));
    } else {
        emu.pc = 2152384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d7c0));
    }
}
#[inline(always)]
pub fn block_0x0020d7c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 19usize, 10usize, 2152388u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d7c4));
}
#[inline]
pub fn block_0x0020d7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 11usize, 2152392u32);
    emu.adi_no_count(9usize, 30usize, 1u32, 2152396u32);
    emu.sltru_no_count(12usize, 11usize, 10usize, 2152400u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2152404u32);
    emu.sltiu_no_count(12usize, 9usize, 1u32, 2152408u32);
    emu.adr_no_count(19usize, 19usize, 12usize, 2152412u32);
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2152416u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2152420u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2152424u32);
    emu.orr_no_count(11usize, 9usize, 19usize, 2152428u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2152432u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2152436u32);
    emu.sltru_no_count(11usize, 11usize, 12usize, 2152440u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2152444u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2152448u32);
    emu.adr_no_count(30usize, 10usize, 14usize, 2152452u32);
    emu.sltru_no_count(12usize, 30usize, 10usize, 2152456u32);
    emu.adr_no_count(11usize, 10usize, 18usize, 2152460u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2152464u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2152468u32);
    emu.lw_no_count(22usize, 2usize, 60u32, 2152472u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2152480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d820));
    } else {
        emu.pc = 2152476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d81c));
    }
}
#[inline(always)]
pub fn block_0x0020d81c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 10usize, 2152480u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d820));
}
#[inline]
pub fn block_0x0020d820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2152484u32);
    emu.adi_no_count(18usize, 11usize, 1u32, 2152488u32);
    emu.sltru_no_count(14usize, 12usize, 10usize, 2152492u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2152496u32);
    emu.adi_no_count(14usize, 12usize, 4294967295u32, 2152500u32);
    emu.sltru_no_count(11usize, 18usize, 11usize, 2152504u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2152508u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2152512u32);
    emu.sltru_no_count(11usize, 11usize, 14usize, 2152516u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2152520u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2152524u32);
    emu.sai_no_count(20usize, 10usize, 1055u32, 2152528u32);
    emu.adr_no_count(28usize, 20usize, 28usize, 2152532u32);
    emu.sltru_no_count(7usize, 28usize, 20usize, 2152536u32);
    emu.adr_no_count(14usize, 20usize, 15usize, 2152540u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2152544u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2152552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d868));
    } else {
        emu.pc = 2152548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d864));
    }
}
#[inline(always)]
pub fn block_0x0020d864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 20usize, 2152552u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d868));
}
#[inline]
pub fn block_0x0020d868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 20usize, 7usize, 2152556u32);
    emu.sltru_no_count(10usize, 7usize, 20usize, 2152560u32);
    emu.anr_no_count(11usize, 7usize, 13usize, 2152564u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2152568u32);
    emu.adr_no_count(5usize, 11usize, 5usize, 2152572u32);
    emu.lw_no_count(10usize, 2usize, 92u32, 2152576u32)?;
    emu.anr_no_count(10usize, 20usize, 10usize, 2152580u32);
    emu.sltru_no_count(16usize, 5usize, 11usize, 2152584u32);
    emu.adr_no_count(6usize, 10usize, 6usize, 2152588u32);
    emu.adr_no_count(21usize, 6usize, 16usize, 2152592u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2152600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d898));
    } else {
        emu.pc = 2152596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d894));
    }
}
#[inline(always)]
pub fn block_0x0020d894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 21usize, 10usize, 2152600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d898));
}
#[inline(always)]
pub fn block_0x0020d898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(11usize, 20usize, 25usize, 2152604u32);
    emu.anr_no_count(10usize, 7usize, 22usize, 2152608u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2152612u32);
    emu.sltru_no_count(10usize, 29usize, 10usize, 2152616u32);
    emu.adr_no_count(15usize, 11usize, 31usize, 2152620u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2152624u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2152632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8b8));
    } else {
        emu.pc = 2152628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8b4));
    }
}
#[inline(always)]
pub fn block_0x0020d8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 11usize, 2152632u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8b8));
}
#[inline]
pub fn block_0x0020d8b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 29usize, 16usize, 2152636u32);
    emu.adr_no_count(6usize, 7usize, 9usize, 2152640u32);
    emu.adr_no_count(12usize, 20usize, 19usize, 2152644u32);
    emu.sltru_no_count(29usize, 16usize, 29usize, 2152648u32);
    emu.sltru_no_count(11usize, 6usize, 7usize, 2152652u32);
    emu.adr_no_count(19usize, 15usize, 29usize, 2152656u32);
    emu.sltru_no_count(15usize, 19usize, 15usize, 2152660u32);
    emu.anr_no_count(9usize, 29usize, 15usize, 2152664u32);
    emu.adr_no_count(9usize, 10usize, 9usize, 2152668u32);
    emu.adr_no_count(12usize, 12usize, 11usize, 2152672u32);
    emu.sltru_no_count(10usize, 9usize, 10usize, 2152676u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2152684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8ec));
    } else {
        emu.pc = 2152680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d8e8));
    }
}
#[inline(always)]
pub fn block_0x0020d8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 12usize, 20usize, 2152684u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152684u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d8ec));
}
#[inline(always)]
pub fn block_0x0020d8ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 6usize, 9usize, 2152688u32);
    emu.sltru_no_count(15usize, 9usize, 6usize, 2152692u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2152696u32);
    emu.adr_no_count(31usize, 10usize, 15usize, 2152700u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2152708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d904));
    } else {
        emu.pc = 2152704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d900));
    }
}
#[inline(always)]
pub fn block_0x0020d900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 31usize, 12usize, 2152708u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d904));
}
#[inline(always)]
pub fn block_0x0020d904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 11usize, 15usize, 2152712u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2152716u32);
    emu.sltru_no_count(10usize, 15usize, 11usize, 2152720u32);
    emu.adr_no_count(29usize, 30usize, 15usize, 2152724u32);
    emu.sltru_no_count(8usize, 29usize, 30usize, 2152728u32);
    emu.adr_no_count(10usize, 18usize, 10usize, 2152732u32);
    emu.adr_no_count(6usize, 10usize, 8usize, 2152736u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2152744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d928));
    } else {
        emu.pc = 2152740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d924));
    }
}
#[inline(always)]
pub fn block_0x0020d924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 6usize, 18usize, 2152744u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d928));
}
#[inline(always)]
pub fn block_0x0020d928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 2usize, 84u32, 2152748u32)?;
    emu.adr_no_count(17usize, 5usize, 17usize, 2152752u32);
    emu.sltru_no_count(11usize, 17usize, 5usize, 2152756u32);
    emu.lw_no_count(5usize, 2usize, 88u32, 2152760u32)?;
    emu.adr_no_count(5usize, 11usize, 5usize, 2152764u32);
    emu.adr_no_count(5usize, 21usize, 5usize, 2152768u32);
    emu.sltru_no_count(10usize, 18usize, 20usize, 2152772u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2152780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d94c));
    } else {
        emu.pc = 2152776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d948));
    }
}
#[inline(always)]
pub fn block_0x0020d948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 5usize, 21usize, 2152780u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d94c));
}
#[inline(always)]
pub fn block_0x0020d94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2152784u32);
    emu.adr_no_count(12usize, 16usize, 11usize, 2152788u32);
    emu.sltru_no_count(15usize, 12usize, 16usize, 2152792u32);
    emu.adr_no_count(18usize, 19usize, 11usize, 2152796u32);
    emu.adr_no_count(18usize, 18usize, 15usize, 2152800u32);
    emu.adr_no_count(8usize, 10usize, 8usize, 2152804u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2152812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d96c));
    } else {
        emu.pc = 2152808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d968));
    }
}
#[inline(always)]
pub fn block_0x0020d968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 18usize, 19usize, 2152812u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d96c));
}
#[inline]
pub fn block_0x0020d96c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 8usize, 10usize, 2152816u32);
    emu.adr_no_count(30usize, 8usize, 28usize, 2152820u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2152824u32);
    emu.adr_no_count(16usize, 12usize, 26usize, 2152828u32);
    emu.sltru_no_count(28usize, 16usize, 12usize, 2152832u32);
    emu.adr_no_count(15usize, 18usize, 28usize, 2152836u32);
    emu.adr_no_count(15usize, 15usize, 23usize, 2152840u32);
    emu.sltru_no_count(12usize, 10usize, 11usize, 2152844u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2152848u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2152856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d998));
    } else {
        emu.pc = 2152852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d994));
    }
}
#[inline(always)]
pub fn block_0x0020d994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 15usize, 18usize, 2152856u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152856u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d998));
}
#[inline]
pub fn block_0x0020d998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2152860u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2152864u32);
    emu.sltru_no_count(8usize, 30usize, 8usize, 2152868u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2152872u32);
    emu.adr_no_count(28usize, 10usize, 28usize, 2152876u32);
    emu.sltru_no_count(10usize, 28usize, 10usize, 2152880u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2152884u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2152888u32);
    emu.adr_no_count(28usize, 31usize, 10usize, 2152892u32);
    emu.adr_no_count(11usize, 9usize, 10usize, 2152896u32);
    emu.sltru_no_count(12usize, 11usize, 9usize, 2152900u32);
    emu.adr_no_count(28usize, 28usize, 12usize, 2152904u32);
    emu.adr_no_count(9usize, 7usize, 14usize, 2152908u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2152916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9d4));
    } else {
        emu.pc = 2152912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d9d0));
    }
}
#[inline(always)]
pub fn block_0x0020d9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 28usize, 31usize, 2152916u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2152916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d9d4));
}
#[inline]
pub fn block_0x0020d9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2152920u32);
    emu.adi_no_count(7usize, 11usize, 1u32, 2152924u32);
    emu.sltru_no_count(11usize, 12usize, 10usize, 2152928u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2152932u32);
    emu.sltiu_no_count(11usize, 7usize, 1u32, 2152936u32);
    emu.adr_no_count(28usize, 28usize, 11usize, 2152940u32);
    emu.adi_no_count(11usize, 12usize, 4294967295u32, 2152944u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2152948u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2152952u32);
    emu.orr_no_count(12usize, 7usize, 28usize, 2152956u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2152960u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2152964u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2152968u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2152972u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2152976u32);
    emu.adr_no_count(11usize, 6usize, 10usize, 2152980u32);
    emu.adr_no_count(14usize, 29usize, 10usize, 2152984u32);
    emu.sltru_no_count(12usize, 14usize, 29usize, 2152988u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2152992u32);
    emu.adr_no_count(8usize, 9usize, 8usize, 2152996u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2153004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da2c));
    } else {
        emu.pc = 2153000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da28));
    }
}
#[inline(always)]
pub fn block_0x0020da28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 6usize, 2153004u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da2c));
}
#[inline]
pub fn block_0x0020da2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2153008u32);
    emu.adi_no_count(6usize, 11usize, 1u32, 2153012u32);
    emu.sltru_no_count(29usize, 12usize, 10usize, 2153016u32);
    emu.sltru_no_count(11usize, 6usize, 11usize, 2153020u32);
    emu.adr_no_count(10usize, 10usize, 29usize, 2153024u32);
    emu.adi_no_count(29usize, 12usize, 4294967295u32, 2153028u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2153032u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2153036u32);
    emu.sltru_no_count(11usize, 11usize, 29usize, 2153040u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2153044u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2153048u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2153052u32);
    emu.adr_no_count(30usize, 10usize, 30usize, 2153056u32);
    emu.sltru_no_count(29usize, 30usize, 10usize, 2153060u32);
    emu.adr_no_count(11usize, 10usize, 8usize, 2153064u32);
    emu.adr_no_count(11usize, 11usize, 29usize, 2153068u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2153076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da74));
    } else {
        emu.pc = 2153072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020da70));
    }
}
#[inline(always)]
pub fn block_0x0020da70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 11usize, 10usize, 2153076u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020da74));
}
#[inline]
pub fn block_0x0020da74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 10usize, 29usize, 2153080u32);
    emu.sltru_no_count(11usize, 29usize, 10usize, 2153084u32);
    emu.anr_no_count(12usize, 29usize, 13usize, 2153088u32);
    emu.adr_no_count(13usize, 10usize, 11usize, 2153092u32);
    emu.adr_no_count(17usize, 12usize, 17usize, 2153096u32);
    emu.lw_no_count(11usize, 2usize, 92u32, 2153100u32)?;
    emu.anr_no_count(11usize, 13usize, 11usize, 2153104u32);
    emu.sltru_no_count(10usize, 17usize, 12usize, 2153108u32);
    emu.adr_no_count(12usize, 11usize, 5usize, 2153112u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2153116u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2153124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020daa4));
    } else {
        emu.pc = 2153120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020daa0));
    }
}
#[inline(always)]
pub fn block_0x0020daa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 11usize, 2153124u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020daa4));
}
#[inline(always)]
pub fn block_0x0020daa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(30usize, 13usize, 25usize, 2153128u32);
    emu.anr_no_count(5usize, 29usize, 22usize, 2153132u32);
    emu.adr_no_count(11usize, 5usize, 16usize, 2153136u32);
    emu.sltru_no_count(16usize, 11usize, 5usize, 2153140u32);
    emu.adr_no_count(15usize, 30usize, 15usize, 2153144u32);
    emu.adr_no_count(5usize, 15usize, 16usize, 2153148u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2153156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dac4));
    } else {
        emu.pc = 2153152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020dac0));
    }
}
#[inline(always)]
pub fn block_0x0020dac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 5usize, 30usize, 2153156u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020dac4));
}
#[inline]
pub fn block_0x0020dac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 11usize, 10usize, 2153160u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2153164u32);
    emu.adr_no_count(28usize, 13usize, 28usize, 2153168u32);
    emu.sltru_no_count(30usize, 10usize, 11usize, 2153172u32);
    emu.sltru_no_count(11usize, 7usize, 29usize, 2153176u32);
    emu.adr_no_count(15usize, 5usize, 30usize, 2153180u32);
    emu.sltru_no_count(5usize, 15usize, 5usize, 2153184u32);
    emu.anr_no_count(5usize, 30usize, 5usize, 2153188u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2153192u32);
    emu.adr_no_count(28usize, 28usize, 11usize, 2153196u32);
    emu.sltru_no_count(29usize, 5usize, 16usize, 2153200u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2153208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020daf8));
    } else {
        emu.pc = 2153204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020daf4));
    }
}
#[inline(always)]
pub fn block_0x0020daf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 28usize, 13usize, 2153208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020daf8));
}
#[inline(always)]
pub fn block_0x0020daf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 7usize, 5usize, 2153212u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2153216u32);
    emu.adr_no_count(5usize, 28usize, 29usize, 2153220u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2153224u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2153232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db10));
    } else {
        emu.pc = 2153228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020db0c));
    }
}
#[inline(always)]
pub fn block_0x0020db0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 28usize, 2153232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2153232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020db10));
}
#[inline(never)]
pub fn block_0x0020db10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 11usize, 7usize, 2153236u32);
    emu.adr_no_count(13usize, 13usize, 6usize, 2153240u32);
    emu.lw_no_count(6usize, 2usize, 64u32, 2153244u32)?;
    emu.sw_no_count(17usize, 6usize, 0u32, 2153248u32)?;
    emu.sw_no_count(12usize, 6usize, 4u32, 2153252u32)?;
    emu.sw_no_count(10usize, 6usize, 8u32, 2153256u32)?;
    emu.sw_no_count(15usize, 6usize, 12u32, 2153260u32)?;
    emu.sltru_no_count(10usize, 7usize, 11usize, 2153264u32);
    emu.adr_no_count(7usize, 14usize, 7usize, 2153268u32);
    emu.sltru_no_count(11usize, 7usize, 14usize, 2153272u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2153276u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2153280u32);
    emu.sw_no_count(16usize, 6usize, 16u32, 2153284u32)?;
    emu.sw_no_count(5usize, 6usize, 20u32, 2153288u32)?;
    emu.sw_no_count(7usize, 6usize, 24u32, 2153292u32)?;
    emu.sw_no_count(10usize, 6usize, 28u32, 2153296u32)?;
    emu.lw_no_count(1usize, 2usize, 252u32, 2153300u32)?;
    emu.lw_no_count(8usize, 2usize, 248u32, 2153304u32)?;
    emu.lw_no_count(9usize, 2usize, 244u32, 2153308u32)?;
    emu.lw_no_count(18usize, 2usize, 240u32, 2153312u32)?;
    emu.lw_no_count(19usize, 2usize, 236u32, 2153316u32)?;
    emu.lw_no_count(20usize, 2usize, 232u32, 2153320u32)?;
    emu.lw_no_count(21usize, 2usize, 228u32, 2153324u32)?;
    emu.lw_no_count(22usize, 2usize, 224u32, 2153328u32)?;
    emu.lw_no_count(23usize, 2usize, 220u32, 2153332u32)?;
    emu.lw_no_count(24usize, 2usize, 216u32, 2153336u32)?;
    emu.lw_no_count(25usize, 2usize, 212u32, 2153340u32)?;
    emu.lw_no_count(26usize, 2usize, 208u32, 2153344u32)?;
    emu.lw_no_count(27usize, 2usize, 204u32, 2153348u32)?;
    emu.adi_no_count(2usize, 2usize, 256u32, 2153352u32);
    emu.add_memory_rw_events(31usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2153356u32;
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
