#![no_std]
#![no_main]

use core::usize;

use cortex_m_rt::entry;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit as _;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

use embedded_hal::delay::DelayNs;

use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{Timer},
    pac::{self, interrupt, TIMER1},
};

#[path = "../sorts.rs"]
mod sorts;
use sorts::{bubble_sort, selection_sort, insertion_sort, quick_sort, cycle_sort};


const N: usize = 5;
const BRIGHTNESS: [u32; N] = [0, 2, 5, 8, 9];

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));


#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let display = Display::new(board.TIMER1, board.display_pins);
    let mut timer0 = Timer::new(board.TIMER0);

    cortex_m::interrupt::free(|cs| {
        DISPLAY.borrow(cs).replace(Some(display));
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }
    
    let mut small_rng = SmallRng::seed_from_u64(42);
    
    loop {
        let mut used_indexes: [bool; N] = [false; 5];
        let input: [u32; N] = core::array::from_fn(|_| {
            let mut random_index:usize;
            loop {
                random_index = small_rng.next_u32() as usize % BRIGHTNESS.len() as usize;
                if !used_indexes[random_index] {
                    used_indexes[random_index] = true;
                    break;
                }
            }
            BRIGHTNESS[random_index]
        });
        rprintln!("input to sort {:?}", input);
        
        let bubble_sort_recording = bubble_sort(&mut input.clone());
        let selection_sort_recording = selection_sort(&mut input.clone());
        let insertion_sort_recording = insertion_sort(&mut input.clone());
        let quick_sort_recording = quick_sort(&mut input.clone());
        let cycle_sort_recording = cycle_sort(&mut input.clone());
        
        let mut max_stages_count = bubble_sort_recording.count;
        for &count in [selection_sort_recording.count, insertion_sort_recording.count, quick_sort_recording.count, cycle_sort_recording.count].iter() {
            if count > max_stages_count {
                max_stages_count = count;
            }
        }
        
        for i in 0..max_stages_count {
            cortex_m::interrupt::free(|cs| {
                if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                    display.show(&GreyscaleImage::new(&[
                        to_u8(bubble_sort_recording.get_stage(i)),
                        to_u8(selection_sort_recording.get_stage(i)),
                        to_u8(insertion_sort_recording.get_stage(i)),
                        to_u8(quick_sort_recording.get_stage(i)),
                        to_u8(cycle_sort_recording.get_stage(i))
                    ]));
                }
            });
            timer0.delay_ms(300u32);
        }
    }
}

fn to_u8(sort_stage: [u32; N]) -> [u8; N] {
    core::array::from_fn(|i| sort_stage[i] as u8)
}

#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    });
}

