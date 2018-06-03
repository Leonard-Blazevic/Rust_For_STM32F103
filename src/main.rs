//! Blinks an LED from timer2 interrupt

#![feature(proc_macro)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx_hal as hal;
extern crate panic_abort;

use hal::gpio::gpioc::PC13;
use hal::gpio::{Output, PushPull};
use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::{Event, Timer};
use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    resources: {
        static LED: PC13<Output<PushPull>>;
        static TIMER2: Timer<hal::stm32f103xx::TIM2>;
    },

    tasks: {
        TIM2: {
            path: tim2_irq,
            resources: [LED, TIMER2],
        },
    }
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = p.device.GPIOC.split(&mut rcc.apb2);
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer2 = Timer::tim2(p.device.TIM2, 1.hz(), clocks, &mut rcc.apb1);
    timer2.listen(Event::Update);

    init::LateResources { LED: led, TIMER2: timer2 }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn tim2_irq(_t: &mut Threshold, mut r: TIM2::Resources) {
    r.TIMER2.clear_interrupt_pending();
    r.LED.toggle();
}