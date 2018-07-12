Rust For STM32F103

This project is a result of a bachelor's thesis from Faculty of Electrical Engineering and Computing, University of Zagreb. The topic of the thesis is Platform for Remote Control of Embedded System Based on Rust Programming Language.

This thesis elaborates the basic features of the Rust programming language, its main functionalities and novel concepts introduced by the language. Rust was compared with C/C++ programming languages, which are nowadays predominantly used in development of embedded systems. In addition to a description of the steps for establishing the Rust development environment for embedded systems, this thesis contains an overview of basic tools and the environment setup on the example of the STM32F103 microcontroller. The advantages and disadvantages of current Rust implementation for embedded systems were analyzed and the assessment of the maturity of the Rust ecosystem for development of production projects was performed. The practical applicability of Rust was examined through a smart home remote control system implementation. Software for this system was developed entirely in Rust, targeting STM32F103 platform, which was combined with various peripheral devices, among which the most important one was the Wi-Fi module ESP8266. Implemented prototype demonstrated that Rust can be used for development of practical applications, but also that there are still many difficulties due to lack of libraries and still relatively small development community.

This repository contains the practical part of the thesis - the software for the demo version of the platform. The hardware used for the platform is:

	1) STM32F103 microcontroller (BluePill board)
	2) ESP8266 WiFi module
	3) IR transmitter module (https://www.velleman.eu/products/view/?id=435546)
	4) Sound detection module (https://www.velleman.eu/products/view/?id=435532)
	5) 5 LEDs
	6) LED lightbulb (https://www.chipoteka.hr/artikl/136525/zarulja-led-e27-6w-u-boji-sa-daljinskim-vt-2022-6070850447)

These devices are connected as follows:

	1) ESP8266 module
		TX => PB11
		RX => PB10

	2) IR transmitter module
		DIN => GPIO2 (ESP8266)

	3) Sound detection module
		DOUT => PA0

	4) LED1 => PA8
	5) LED2 => PA9
	6) LED3 => PA10
	5) LED4 => PC13 (turns on by sound)
	6) LED5 => PB9 (PWM)


The software for the STM32F103 microcontroller in entirely Rust based, but the ESP8266 firmware is written in C usinf the Arduino IDE. The idea behind the platform is to allow the user to control various functionalities (represented by the LEDs and the external IR LED lightbulb) using his Android smartphone and the Blynk app (https://www.blynk.cc/).

To install all the tools needed for the project setup (tested on Ubuntu) follow the instructions from the dependencies section here: https://docs.rs/cortex-m-quickstart/0.2.5/cortex_m_quickstart/. After that clone this repository, add the target device to the Rust environment (step 4 from the link) and run the program as follows:

	1)open one terminal and run 
		openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg (assuming STM32F103 is connected via STLINK)
	2)open a second terminal (in the root of the project) and run
		cargo build –release (build the program)
		arm-none-eabi-gdb target/thumbv7m-none-eabi/release/stm32_f103_rust (flash the program and start a GDB session)