//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: Nucleo-144 F303 or F746
//! 
//! This code does not work on the F303.  The problem seems to be something with how the pins
//! are configured for USB...ie they are not configured properly.  Not sure how to set
//! them to be USB as that is not in the list of alternate functions (AF).
//! Code is not tested on the F746.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_semihosting as _;

use stm32f3xx_hal as hal;

use cortex_m::asm::delay;
use cortex_m_rt::entry;

use hal::pac;
use hal::prelude::*;
use hal::usb::{Peripheral, UsbBus};

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .pclk2(24.MHz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    // F746
    // let mut gpiob = dp.GPIOB.split();

    // F303
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // F746
    // let mut led_blue = gpiob.pb7.into_push_pull_output();
    // let mut led_red = gpiob.pb14.into_push_pull_output();
    // led_blue.set_low(); // Turn off
    // led_red.set_low(); // Turn off
    // F303
    let mut led_blue = gpiob.pb7.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_red = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    led_blue.set_low().unwrap(); // Turn off
    led_red.set_low().unwrap(); // Turn off


    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    // ::<9_u8> is CAN bus which does not work
    let usb_dm = gpioa
        .pa11
        .into_af_push_pull::<9_u8>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let usb_dp = gpioa
        .pa12
        .into_af_push_pull::<9_u8>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };
    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();
    
    led_red.set_high().unwrap(); // Turn off
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led_blue.set_high().ok(); // Turn on

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        led_blue.set_low().ok();
        
        led_red.set_low().ok();
        delay(10_000);
    }
}