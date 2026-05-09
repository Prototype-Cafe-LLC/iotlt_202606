//! Potentiometer on **A0 (GPIO26)** → SG90 on **D4 / GPIO6** (**PWM3 A**), dimming on
//! **USER_LED_R (GPIO17)** via **PWM0 B** at **~1 kHz** carrier (≥ 120 Hz; servo stays 50 Hz
//! on a separate slice).
//!
//! I2C is not used; D4/GPIO6 is repurposed for servo PWM. Wiring matches `AGENTS.md`.

#![no_std]
#![no_main]

use embedded_hal::pwm::SetDutyCycle;
use embedded_hal_02::adc::OneShot;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal::adc::AdcPin;
use seeeduino_xiao_rp2040::hal::gpio::PinState;
use seeeduino_xiao_rp2040::hal::prelude::*;
use seeeduino_xiao_rp2040::hal::{self, pac};

/// SG90 pulse width (µs) at minimum ADC.
const SERVO_MIN_US: u16 = 1_000;
/// SG90 pulse width (µs) at maximum ADC.
const SERVO_MAX_US: u16 = 2_000;
/// 20 ms frame @ 1 µs tick after `set_div_int(125)` on a 125 MHz sysclk → 50 Hz servo frame.
const SERVO_TICKS: u16 = 20_000;

/// Integer part of PWM clock divider for LED slice (125 MHz / 125 = 1 MHz tick).
const LED_PWM_DIV_INT: u8 = 125;
/// `TOP` register: period = (`LED_PWM_TOP` + 1) ticks → 1 MHz / 1000 ≈ **1 kHz** (≥ 120 Hz).
const LED_PWM_TOP: u16 = 999;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        seeeduino_xiao_rp2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);
    let pins = seeeduino_xiao_rp2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Onboard RGB: blue/green off; red uses PWM0 B at high carrier.
    let mut _led_blue = pins.led_blue.into_push_pull_output_in_state(PinState::High);
    let mut _led_green = pins
        .led_green
        .into_push_pull_output_in_state(PinState::High);

    let mut adc = hal::adc::Adc::new(pac.ADC, &mut pac.RESETS);
    let mut volume = AdcPin::new(pins.a0).unwrap();

    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let (pwm0, pwm3) = (&mut pwm_slices.pwm0, &mut pwm_slices.pwm3);

    pwm0.clr_ph_correct();
    pwm0.set_div_int(LED_PWM_DIV_INT);
    pwm0.set_div_frac(0);
    pwm0.set_top(LED_PWM_TOP);
    pwm0.enable();

    pwm3.clr_ph_correct();
    pwm3.set_div_int(125);
    pwm3.set_div_frac(0);
    pwm3.set_top(SERVO_TICKS - 1);
    pwm3.enable();

    let servo = &mut pwm3.channel_a;
    let led = &mut pwm0.channel_b;
    let _servo_pin = servo.output_to(pins.sda);
    let _led_pin = led.output_to(pins.led_red);

    loop {
        let raw: u16 = adc.read(&mut volume).unwrap();
        let sample = u32::from(raw);

        let span = u32::from(SERVO_MAX_US - SERVO_MIN_US);
        let pulse = u32::from(SERVO_MIN_US) + (sample * span) / 4095;
        let _ = servo.set_duty_cycle(pulse as u16);

        let led_max = u32::from(led.max_duty_cycle());
        let led_duty = ((sample * led_max) / 4095) as u16;
        let _ = led.set_duty_cycle(led_duty);

        delay.delay_us(5);
    }
}
