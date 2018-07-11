#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx_hal as hal;
#[macro_use(block)]
extern crate nb;
extern crate panic_abort;

mod pwm;

use hal::gpio::gpioc::{PC13};
use hal::gpio::gpioa::{PA8, PA9, PA10};
use hal::gpio::{Output, PushPull, Input, Floating};
use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::{Event, Timer};
use hal::serial::{Serial, Rx, Tx};
use hal::pwm::{Pwm, C4};
use rtfm::{app, Threshold};
use hal::stm32f103xx::interrupt::Interrupt;

const RED_YELLOW_PAUSE : u8 = 40;
const YELLOW_GREEN_PAUSE : u8 = 10;
const GREEN_YELLOW_PAUSE : u8 = 40;
const YELLOW_RED_PAUSE : u8 = 10;

struct Package {
    data: [u8; 2],
}

static mut LIGHTS_RUNNING: bool = false;
static mut LIGHTS_PAUSE: bool = false;
static mut LIGHTS_SPEED_FACTOR: u8 = 1;


app! {
    device: stm32f103xx,

    resources: {
        static LEDR: PA8<Output<PushPull>>;
        static LEDY: PA9<Output<PushPull>>;
        static LEDG: PA10<Output<PushPull>>;
        static TEMP: PC13<Output<PushPull>>;
        static TIMER2: Timer<hal::stm32f103xx::TIM2>;
        static RX: Rx<hal::stm32f103xx::USART3>;
        static TX: Tx<hal::stm32f103xx::USART3>;
        static RX2: Rx<hal::stm32f103xx::USART2>;
        static TX2: Tx<hal::stm32f103xx::USART2>;
        static PWM: Pwm<hal::stm32f103xx::TIM4, hal::pwm::C4>;
        static EXTR: hal::stm32f103xx::EXTI;
    },

    init: {
        path: init,
    },

    idle: {
        path: idle,
        resources: [RX, TX, PWM, RX2, TX2],
    },

    tasks: {
        TIM2: {
            path: tim2_irq,
            // If omitted priority is assumed to be 1
            priority: 1,
            resources: [TIMER2, TEMP, LEDR, LEDY, LEDG],
        },

        EXTI0: {
            path: exti0,
            priority: 2,
            resources: [TEMP, EXTR],
		},
    }
}

fn init(mut p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer2 = Timer::tim2(p.device.TIM2, 10.hz(), clocks, &mut rcc.apb1);

    p.device.AFIO.exticr1.write(|w| unsafe { w.exti0().bits(0) });

    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);

    let mut gpioc = p.device.GPIOC.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);// => odabir sabirnice
    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);// => odabir sabirnice

    let mut userLED = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    userLED.set_low();

    let mut ledR = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut ledY = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
	let mut ledG = gpioa.pa10.into_push_pull_output(&mut gpioa.crh);

    //let temp = gpiob.pb0.into_floating_input(&mut gpiob.crl);

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

	let tx2 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);// => odabir pinova za serijsku vezu
    let rx2 = gpioa.pa3;

	let serial2 = Serial::usart2(// => konfiguracija uarta
		p.device.USART2,
		(tx2, rx2),
		&mut afio.mapr,
		9_600.bps(),
		clocks,
		&mut rcc.apb1,
	);

	let (mut tx2, mut rx2) = serial2.split();

    let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    let mut pwm = p.device.TIM4
        .pwm(
            (c1, c2, c3, c4),
            &mut afio.mapr,
            1.khz(),
            clocks,
            &mut rcc.apb1,
        ).3;

   	let max = pwm.get_max_duty();
    pwm.enable();
    pwm.set_duty(9000);

    timer2.listen(Event::Update);


    let _int0 = gpioa.pa0.into_floating_input(&mut gpioa.crl);
    unsafe {
        p.core.NVIC.set_priority(hal::stm32f103xx::interrupt::Interrupt::EXTI0, 1);
    }
    p.core.NVIC.enable(
        hal::stm32f103xx::interrupt::Interrupt::EXTI0,
    );
    p.device.EXTI.imr.write(|w| w.mr0().set_bit()); // unmask the interrupt (EXTI)
    p.device.EXTI.emr.write(|w| w.mr0().set_bit());
    p.device.EXTI.rtsr.write(|w| w.tr0().set_bit()); // trigger interrupt on falling edge
	rtfm::set_pending(Interrupt::EXTI0);

    init::LateResources { LEDR: ledR, LEDY: ledY,  LEDG: ledG, TEMP: userLED, TIMER2: timer2, RX: rx, TX: tx, PWM: pwm, RX2: rx2, TX2: tx2, EXTR: p.device.EXTI }
}

fn idle(_t: &mut Threshold, _r: idle::Resources) -> ! {
	let mut package = Package { data: [0; 2] };
	let mut cnt = 0;
	let max_pwm = _r.PWM.get_max_duty();
    
    loop {
        if cnt < 2// => prima 2 podatka
        {
            package.data[cnt] = block!(_r.RX.read()).unwrap();
            cnt = cnt + 1;
        }
        else// => nakon prijema 2 podatka, iste podatke ispisuje i ovisno o njima pali/gasi ledicu ili ne radi nista
        {
            cnt = 0;

            if package.data[0] == b'0' {
            	let intensity : u16 = pwm::intensity(package.data[1]);
				_r.PWM.set_duty(intensity);
            }
            else if package.data[0] == b'1' {// => svjetla rade / ne rade
                if package.data[1] == b'1' {
                	unsafe {
                		LIGHTS_RUNNING = true;
                	}
                }
                else {
                	unsafe {
                    	LIGHTS_RUNNING = false;
                	}
                }
            }
            else if package.data[0] == b'2' {// => Slider za brzinu svjetala
            	unsafe {
					LIGHTS_SPEED_FACTOR = package.data[1] - 48;
				}
            }
            else if package.data[0] == b'3' { //=> svjetla pauzirana
                if package.data[1] == b'1' {
                	unsafe {
                		LIGHTS_PAUSE = true;
                	}
                }
                else {
                	unsafe {
                    	LIGHTS_PAUSE = false;
                	}
                }
            }
		}
	}
}

fn tim2_irq(_t: &mut Threshold, mut r: TIM2::Resources) {
    static mut cnt_irq: u8 = 0;

    r.TIMER2.clear_interrupt_pending();

    unsafe {
    	if LIGHTS_RUNNING {
    		if !LIGHTS_PAUSE {
	    		if cnt_irq == 0 {
		        	if r.LEDR.is_high() {
		        		if r.LEDY.is_high() { // stanje 2
		        			r.LEDR.set_low();
		        			r.LEDY.set_low();
		        			r.LEDG.set_high();

		        			cnt_irq = GREEN_YELLOW_PAUSE / LIGHTS_SPEED_FACTOR;
		        		}
		        		else { // stanje 1
		        			r.LEDY.set_high();

		        			cnt_irq = YELLOW_GREEN_PAUSE / LIGHTS_SPEED_FACTOR;
		        		}
		        	}
		        	else if r.LEDY.is_high() { // stanje 4
		        		r.LEDY.set_low();
		        		r.LEDR.set_high();

		        		cnt_irq = RED_YELLOW_PAUSE / LIGHTS_SPEED_FACTOR;
		        	}
		        	else { // stanje 3
		        		r.LEDY.set_high();
		        		r.LEDG.set_low();

		        		cnt_irq = YELLOW_RED_PAUSE / LIGHTS_SPEED_FACTOR;
		       		}
	        	}
		        else {
		            cnt_irq = cnt_irq - 1;
		        }
    		}
    	}
    	else {
	        r.LEDR.set_low();
	        r.LEDY.set_low();
	        r.LEDG.set_low();
    	}
    }
}

fn exti0(t: &mut Threshold, mut r: EXTI0::Resources) {
    r.EXTR.pr.modify(|_, w| w.pr0().set_bit());
    r.TEMP.toggle();
}