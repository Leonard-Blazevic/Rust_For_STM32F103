//! Serial interface test with led
//!
//! Enter onn in the terminal to turn on the user led and off to turn it off

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate panic_abort;

extern crate stm32f103xx_hal as hal;
#[macro_use(block)]
extern crate nb;
use hal::prelude::*;
use hal::stm32f103xx;
use hal::serial::Serial;

use cortex_m::asm;

struct Package {
    data: [u8; 2],
}

fn main() {
    let p = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);// => odabir sabirnice
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);// => sabirnica na kojoj se nalazi user led

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut tv = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
    let mut air_conditioner = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
    let mut lamp = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);


    // USART1
    // let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);// => odabir pinova za serijsku vezu
    let rx = gpiob.pb11;

    let serial = Serial::usart3(// => konfiguracija uarta
        p.USART3,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb1,
    );

    let (mut tx, mut rx) = serial.split();

    let mut package = Package { data: [0; 2] };// => inicijalizacija paketa kojeg šaljemo/primamo preko serija (sadrži 2 podatka)
    let mut cnt = 0;
    let mut led_state = 0;

    loop {
        /*for mut dataPoint in package.data.iter() {
            *dataPoint = block!(rx.read()).unwrap();
        }
        for dataPoint in package.data.iter() {
            block!(tx.write(*dataPoint)).ok();
        }*/


        if(cnt < 2)// => prima 2 podatka
        {
            package.data[cnt] = block!(rx.read()).unwrap();
            cnt = cnt + 1;
        }
        else// => nakon prijema 2 podatka, iste podatke ispisuje i ovisno o njima pali/gasi ledicu ili ne radi nista
        {
            cnt = 0;

            if(package.data[0] == b'1'){// => funkcije set_low i set_high su obrnute (set_low pali ledicu)
                if(package.data[1] == b'1'){
                    lamp.set_high();
                }
                else {
                    lamp.set_low();
                }
            }
            else if(package.data[0] == b'2'){
                if(package.data[1] == b'1'){
                    tv.set_high();
                }
                else {
                    tv.set_low();
                }
            }
            else if(package.data[0] == b'3'){
                if(package.data[1] == b'1'){
                    air_conditioner.set_high();
                }
                else {
                    air_conditioner.set_low();
                }
            }

            /*for i in 0..2 {
                block!(tx.write(package.data[i])).ok();
            }

            block!(tx.write(b'\n')).ok();*/
        }
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}