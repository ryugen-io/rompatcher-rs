//! VCDIFF default code table (RFC 3284)

use crate::xdelta::constants::{VCD_ADD, VCD_COPY, VCD_NOOP, VCD_RUN};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub inst_type: u8,
    pub size: u8,
    pub mode: u8,
}

static DEFAULT_CODE_TABLE: OnceLock<[[Instruction; 2]; 256]> = OnceLock::new();

pub fn get_default_code_table() -> &'static [[Instruction; 2]; 256] {
    DEFAULT_CODE_TABLE.get_or_init(|| {
        let mut table = [[Instruction {
            inst_type: VCD_NOOP,
            size: 0,
            mode: 0,
        }; 2]; 256];
        let mut i = 0;

        let empty = Instruction {
            inst_type: VCD_NOOP,
            size: 0,
            mode: 0,
        };

        // 0: RUN, NOOP
        table[i] = [
            Instruction {
                inst_type: VCD_RUN,
                size: 0,
                mode: 0,
            },
            empty,
        ];
        i += 1;

        // 1-18: ADD size 0-17
        for size in 0..18 {
            table[i] = [
                Instruction {
                    inst_type: VCD_ADD,
                    size,
                    mode: 0,
                },
                empty,
            ];
            i += 1;
        }

        // 19-162: COPY mode 0-8
        for mode in 0..9 {
            table[i] = [
                Instruction {
                    inst_type: VCD_COPY,
                    size: 0,
                    mode,
                },
                empty,
            ];
            i += 1;
            for size in 4..19 {
                table[i] = [
                    Instruction {
                        inst_type: VCD_COPY,
                        size,
                        mode,
                    },
                    empty,
                ];
                i += 1;
            }
        }

        // 163-234: ADD size 1-4 + COPY size 4-6 mode 0-5
        for mode in 0..6 {
            for add_size in 1..5 {
                for copy_size in 4..7 {
                    table[i] = [
                        Instruction {
                            inst_type: VCD_ADD,
                            size: add_size,
                            mode: 0,
                        },
                        Instruction {
                            inst_type: VCD_COPY,
                            size: copy_size,
                            mode,
                        },
                    ];
                    i += 1;
                }
            }
        }

        // 235-246: ADD size 1-4 + COPY size 4 mode 6-8
        for mode in 6..9 {
            for add_size in 1..5 {
                table[i] = [
                    Instruction {
                        inst_type: VCD_ADD,
                        size: add_size,
                        mode: 0,
                    },
                    Instruction {
                        inst_type: VCD_COPY,
                        size: 4,
                        mode,
                    },
                ];
                i += 1;
            }
        }

        // 247-255: COPY size 4 mode 0-8 + ADD size 1
        for mode in 0..9 {
            table[i] = [
                Instruction {
                    inst_type: VCD_COPY,
                    size: 4,
                    mode,
                },
                Instruction {
                    inst_type: VCD_ADD,
                    size: 1,
                    mode: 0,
                },
            ];
            i += 1;
        }

        table
    })
}
