#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
};

fn is_prime(primes: &[u32], number: u32) -> bool {
    primes
        .iter()
        .find(|&&prime| prime * prime > number || number % prime == 0)
        .and_then(|&prime| {
            if number % prime != 0 {
                Some(number)
            } else {
                None
            }
        })
        .is_some()
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);

    let mut primes = [0_u32; 10000];
    let mut length = 1;
    primes[0] = 2;
    println!("2");
    let mut number = 3;

    while length < primes.len() {
        if is_prime(&primes[..length], number) {
            primes[length] = number;
            length += 1;
            println!("{number}");
        }
        number += 2;
    }

    let max = *primes.last().unwrap();
    for number in (number..max * max).step_by(2) {
        if is_prime(&primes[..length], number) {
            println!("{number}");
        }
    }

    loop {
        delay.delay_ms(u32::MAX);
    }
}
