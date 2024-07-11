#![no_std]
#![no_main]

use core::ptr::write_volatile;


use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use microbit as _;
use rtt_target::{rprintln, rtt_init_print};

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

use core::cmp::max;

#[path = "../sorts.rs"] mod sorts;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut small_rng = SmallRng::seed_from_u64(42);
    const BLINK_PAUSE: u32 = 1000;
    const SORT_STAGE_SHOW: u32 = 80;
    let mut blink_rows: [BlinkRow; 5] = core::array::from_fn(|i| BlinkRow::init(1, i));
    loop {
        let mut numbers: [u32; 5] = core::array::from_fn(|_| max(1, small_rng.next_u32() % 6));
        rprintln!("input to sort {:?}", numbers);
        let sort_recording = sorts::bubble_sort(&mut numbers);
        rprintln!(" sort stages: {:?}", sort_recording.stages);
        for i in 0..sort_recording.count {
            for (index, value) in sort_recording.stages[i].iter().enumerate() {
                blink_rows[index] = BlinkRow::init(*value, index as usize);
            }
            for _ in 0..SORT_STAGE_SHOW {
                for blink_row in &blink_rows {
                    blink_row.turn(true);
                    nopit(BLINK_PAUSE);
                    blink_row.turn(false);
                }
            }
        }
    }
}


fn nopit(it: u32) {
    for _ in 0..it {
        nop();
    }
}

#[derive(Debug)]
struct BlinkRow {
    value: u32,
    i: usize,
}

impl BlinkRow {
    fn init(value: u32, row_index: usize) -> BlinkRow {
        unsafe {
            write_volatile(ROWS_ADDR[row_index], PINCNF_DRIVE_LED);
        }
        BlinkRow { value, i: row_index}
    }

    fn turn(&self, state: bool) -> () {
        unsafe {
            for col in COLS_ADDR {
                write_volatile(col, PINCNF_STOP_LED);
            }
            turn_row(state, ROWS_POS[self.i]);
            for col in 0..self.value {
                write_volatile(COLS_ADDR[col as usize], PINCNF_DRIVE_LED);
            }
        }
    }
}

fn turn_row(state: bool, row_pos: usize) -> () {
    unsafe {
        write_volatile(GPIO0_OUT_ADDR, (state as u32) << row_pos);
    }
}

const ROWS_ADDR: [*mut u32; 5] = [
    GPIO0_PINNCNF21_ROW1_ADDR,
    GPIO0_PINNCNF21_ROW2_ADDR,
    GPIO0_PINNCNF21_ROW3_ADDR,
    GPIO0_PINNCNF21_ROW4_ADDR,
    GPIO0_PINNCNF21_ROW5_ADDR,
];

const COLS_ADDR: [*mut u32; 5] = [
    GPIO0_PINNCNF21_COL1_ADDR,
    GPIO0_PINNCNF21_COL2_ADDR,
    GPIO0_PINNCNF21_COL3_ADDR,
    GPIO0_PINNCNF21_COL4_ADDR,
    GPIO0_PINNCNF21_COL5_ADDR,
];

const ROWS_POS: [usize; 5] = [
    GPIO0_OUT_ROW1_POS,
    GPIO0_OUT_ROW2_POS,
    GPIO0_OUT_ROW3_POS,
    GPIO0_OUT_ROW4_POS,
    GPIO0_OUT_ROW5_POS,
];


const GPIO0_PINNCNF21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
const GPIO0_PINNCNF21_ROW2_ADDR: *mut u32 = 0x5000_0758 as *mut u32;
const GPIO0_PINNCNF21_ROW3_ADDR: *mut u32 = 0x5000_073C as *mut u32;
const GPIO0_PINNCNF21_ROW4_ADDR: *mut u32 = 0x5000_0760 as *mut u32;
const GPIO0_PINNCNF21_ROW5_ADDR: *mut u32 = 0x5000_074C as *mut u32;
const GPIO0_PINNCNF21_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
const GPIO0_PINNCNF21_COL2_ADDR: *mut u32 = 0x5000_072C as *mut u32;
const GPIO0_PINNCNF21_COL3_ADDR: *mut u32 = 0x5000_077C as *mut u32;
const GPIO0_PINNCNF21_COL4_ADDR: *mut u32 = 0x5000_0A14 as *mut u32;
const GPIO0_PINNCNF21_COL5_ADDR: *mut u32 = 0x5000_0778 as *mut u32;

const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;

const GPIO0_OUT_ROW1_POS: usize = 21;
const GPIO0_OUT_ROW2_POS: usize = 22;
const GPIO0_OUT_ROW3_POS: usize = 15;
const GPIO0_OUT_ROW4_POS: usize = 24;
const GPIO0_OUT_ROW5_POS: usize = 19;

const DIR_OUTPUT_POS: u32 = 0;
const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
const PINCNF_STOP_LED: u32 = 0 << DIR_OUTPUT_POS;