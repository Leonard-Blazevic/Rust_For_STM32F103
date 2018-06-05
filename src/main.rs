//! Blinks an LED from timer2 interrupt

#![feature(proc_macro)]
//#![deny(unsafe_code)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx_hal as hal;
#[macro_use(block)]
extern crate nb;
extern crate panic_abort;

mod lcd;

use hal::gpio::gpioc::PC13;
use hal::gpio::gpiob::PB0;
use hal::gpio::{Output, PushPull, Input, Floating};
use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::{Event, Timer};
use hal::serial::{Serial, Rx, Tx};
use rtfm::{app, Threshold};

app! {
    device: stm32f103xx,

    resources: {
        static LED: PC13<Output<PushPull>>;
        static TEMP: PB0<Input<Floating>>;
        static TIMER2: Timer<hal::stm32f103xx::TIM2>;
        static RX: Rx<hal::stm32f103xx::USART3>;
        static TX: Tx<hal::stm32f103xx::USART3>;
    },

    init: {
        // This is the path to the `init` function
        //
        // `init` doesn't necessarily has to be in the root of the crate
        path: init,
    },

    idle: {
        // This is a path to the `idle` function
        //
        // `idle` doesn't necessarily has to be in the root of the crate
        path: idle,
        resources: [RX, TX],
    },

    tasks: {
        TIM2: {
            path: tim2_irq,
            // If omitted priority is assumed to be 1
            // priority: 1,
            resources: [LED, TIMER2, RX, TX, TEMP],
        },
    }
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer2 = Timer::tim2(p.device.TIM2, 10.hz(), clocks, &mut rcc.apb1);

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    let mut gpioc = p.device.GPIOC.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);// => odabir sabirnice

    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let temp = gpiob.pb0.into_floating_input(&mut gpiob.crl);

    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);// => odabir pinova za serijsku vezu
    let rx = gpiob.pb11;

    let serial = Serial::usart3(// => konfiguracija uarta
        p.device.USART3,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb1,
    );

    let (mut tx, mut rx) = serial.split();

    timer2.listen(Event::Update);

    init::LateResources { LED: led, TEMP: temp, TIMER2: timer2, RX: rx, TX: tx }
}

fn idle(_t: &mut Threshold, _r: idle::Resources) -> ! {
    
     loop {
        rtfm::wfi();
    }
}

fn tim2_irq(_t: &mut Threshold, mut r: TIM2::Resources) {
    static mut cnt: u8 = 0;
    //let result = lcd::sum(2,3);

    r.TIMER2.clear_interrupt_pending();

    unsafe {
        if(cnt == 50){
            r.LED.toggle();
            cnt = 0;

            let mut received = r.TEMP.is_high();
            let mut forSending;

            if received {
                forSending = b'0';
            }
            else {
                forSending = b'1';
            }

            block!(r.TX.write(forSending)).ok();
        }
        else {
            cnt = cnt + 1;
        }
    }
}