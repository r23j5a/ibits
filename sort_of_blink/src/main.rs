#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

use core::cmp::max;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut small_rng = SmallRng::seed_from_u64(42);
    const BLINK_PAUSE: u32 = 1000;
    const SORT_STATE_SHOW: u32 = 80;
    let mut blink_rows: [BlinkRow; 5] = core::array::from_fn(|i| BlinkRow::new(1, (i + 1) as u32));
    loop {
        let mut numbers: [u32; 5] = core::array::from_fn(|_| max(1, small_rng.next_u32() % 6));
        rprintln!("input to sort {:?}", numbers);
        let (sort_states, sort_states_count) = insertion_sort(&mut numbers);
        rprintln!(" sort states: {:?}", &sort_states[0..sort_states_count]);
    
        for i in 0..sort_states_count {
            for (index, value) in sort_states[i].iter().enumerate() {
                blink_rows[index] = BlinkRow::new(*value, (index + 1) as u32);
            }
            for _ in 0..SORT_STATE_SHOW {
                for blink_row in &blink_rows {
                    blink_row.turn(true);
                    nopit(BLINK_PAUSE);
                    blink_row.turn(false);
                }
            }
        }
    }
}

fn insertion_sort(input: &mut [u32]) -> ([[u32; 5]; 10 + 2], usize) {
    let input_length = input.len();
    let mut sort_states: [[u32; 5]; 10 + 2] = [[0; 5]; 10 + 2];
    let mut sort_states_count = 0;
    sort_states[sort_states_count].copy_from_slice(input);
    sort_states_count += 1;
    for i in 1..input_length {
        let mut j = i;
        while j > 0 && input[j - 1] > input[j] {
            input.swap(j, j - 1);
            sort_states[sort_states_count].copy_from_slice(input);
            sort_states_count += 1;
            j -= 1;
        }
    }
    sort_states[sort_states_count].copy_from_slice(input);
    sort_states_count += 1;
    return (sort_states, sort_states_count);
}

fn turn_row(state: bool, gpio: *mut u32, row_pos: u32) -> () {
    unsafe {
        write_volatile(gpio, (state as u32) << row_pos);
    }
}

fn nopit(it: u32) {
    for _ in 0..it {
        nop();
    }
}

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

const GPIO0_OUT_ROW1_POS: u32 = 21;
const GPIO0_OUT_ROW2_POS: u32 = 22;
const GPIO0_OUT_ROW3_POS: u32 = 15;
const GPIO0_OUT_ROW4_POS: u32 = 24;
const GPIO0_OUT_ROW5_POS: u32 = 19;

const DIR_OUTPUT_POS: u32 = 0;
const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
const PINCNF_STOP_LED: u32 = 0 << DIR_OUTPUT_POS;

#[derive(Debug)]
enum I {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

#[derive(Debug)]
struct BlinkRow {
    value: I,
    i: I,
}

impl BlinkRow {
    fn new(value: u32, row: u32) -> BlinkRow {
        let value: I = match value {
            1 => I::ONE,
            2 => I::TWO,
            3 => I::THREE,
            4 => I::FOUR,
            5 => I::FIVE,
            _ => panic!("Invalid value for conversion to I enum"),
        };
        let row: I = match row {
            1 => I::ONE,
            2 => I::TWO,
            3 => I::THREE,
            4 => I::FOUR,
            5 => I::FIVE,
            _ => panic!("Invalid value for conversion to I enum"),
        };
        unsafe {
            match row {
                I::ONE => write_volatile(GPIO0_PINNCNF21_ROW1_ADDR, PINCNF_DRIVE_LED),
                I::TWO => write_volatile(GPIO0_PINNCNF21_ROW2_ADDR, PINCNF_DRIVE_LED),
                I::THREE => write_volatile(GPIO0_PINNCNF21_ROW3_ADDR, PINCNF_DRIVE_LED),
                I::FOUR => write_volatile(GPIO0_PINNCNF21_ROW4_ADDR, PINCNF_DRIVE_LED),
                I::FIVE => write_volatile(GPIO0_PINNCNF21_ROW5_ADDR, PINCNF_DRIVE_LED),
            }
        }
        BlinkRow { value, i: row }
    }

    fn turn(&self, state: bool) -> () {
        unsafe {
            write_volatile(GPIO0_PINNCNF21_COL1_ADDR, PINCNF_STOP_LED);
            write_volatile(GPIO0_PINNCNF21_COL2_ADDR, PINCNF_STOP_LED);
            write_volatile(GPIO0_PINNCNF21_COL3_ADDR, PINCNF_STOP_LED);
            write_volatile(GPIO0_PINNCNF21_COL4_ADDR, PINCNF_STOP_LED);
            write_volatile(GPIO0_PINNCNF21_COL5_ADDR, PINCNF_STOP_LED);
            match &self.i {
                I::ONE => turn_row(state, GPIO0_OUT_ADDR, GPIO0_OUT_ROW1_POS),
                I::TWO => turn_row(state, GPIO0_OUT_ADDR, GPIO0_OUT_ROW2_POS),
                I::THREE => turn_row(state, GPIO0_OUT_ADDR, GPIO0_OUT_ROW3_POS),
                I::FOUR => turn_row(state, GPIO0_OUT_ADDR, GPIO0_OUT_ROW4_POS),
                I::FIVE => turn_row(state, GPIO0_OUT_ADDR, GPIO0_OUT_ROW5_POS),
            }
            let cols: &[I] = match &self.value {
                I::ONE => &[I::ONE],
                I::TWO => &[I::ONE, I::TWO],
                I::THREE => &[I::ONE, I::TWO, I::THREE],
                I::FOUR => &[I::ONE, I::TWO, I::THREE, I::FOUR],
                I::FIVE => &[I::ONE, I::TWO, I::THREE, I::FOUR, I::FIVE],
            };
            for col in cols {
                match col {
                    I::ONE => write_volatile(GPIO0_PINNCNF21_COL1_ADDR, PINCNF_DRIVE_LED),
                    I::TWO => write_volatile(GPIO0_PINNCNF21_COL2_ADDR, PINCNF_DRIVE_LED),
                    I::THREE => write_volatile(GPIO0_PINNCNF21_COL3_ADDR, PINCNF_DRIVE_LED),
                    I::FOUR => write_volatile(GPIO0_PINNCNF21_COL4_ADDR, PINCNF_DRIVE_LED),
                    I::FIVE => write_volatile(GPIO0_PINNCNF21_COL5_ADDR, PINCNF_DRIVE_LED),
                }
            }
        }
    }
}

