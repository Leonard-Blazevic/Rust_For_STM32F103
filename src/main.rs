//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_abort;

extern crate stm32f103xx_hal as hal;
#[macro_use(block)]
extern crate nb;

use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::Timer;
//use hal::delay::Delay;

use cortex_m::asm;

fn main() {
    let p = stm32f103xx::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);

    let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut pwm = p.TIM4
        .pwm(
            (c1, c2, c3, c4),
            &mut afio.mapr,
            1.khz(),
            clocks,
            &mut rcc.apb1,
        )
        .3;

    let max = pwm.get_max_duty();
    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    pwm.enable();

    loop {
        pwm.set_duty(max);
        block!(timer.wait()).unwrap();
        // dim
        pwm.set_duty(max / 4);
        block!(timer.wait()).unwrap();

        pwm.set_duty(max / 50);
        block!(timer.wait()).unwrap();

        pwm.set_duty(max / 100);
        block!(timer.wait()).unwrap();
        // zero
        pwm.set_duty(0);
        block!(timer.wait()).unwrap();


        /*block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();*/
    }
}





// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
