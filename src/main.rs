#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;

// Use by all display examples
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitive_style;
use embedded_graphics::primitives::Rectangle;
use longan_nano::{lcd, lcd_pins};

// Used by Ferris display example
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::raw::LittleEndian;
const FERRIS: &[u8] = include_bytes!("ferris.raw");

// Used by display text example
use embedded_graphics::text_style;
use embedded_graphics::fonts::{Font6x8, Text};

// Used by LED code
use longan_nano::led::{Led, rgb};
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::blocking::delay::DelayMs;

// Used by the UART example
use longan_nano::sprintln;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    // Hello world example - need a serial connection to the UART pins 
    longan_nano::stdout::configure(dp.USART0, gpioa.pa9, gpioa.pa10, 115_200.bps(), &mut afio, &mut rcu);
    sprintln!("Hello, world");

    // ---- Ferris display example ---- //

    // Clear screen
    Rectangle::new(Point::new(0, 0), Point::new(width - 1, height - 1))
        .into_styled(primitive_style!(fill_color = Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    // Load Image Data
    let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&FERRIS, 86, 64);
    Image::new(&raw_image, Point::new(width / 2 - 43, height / 2 - 32))
        .draw(&mut lcd)
        .unwrap();

    // ---- Text display example ---- //
    let style = text_style!(
        font = Font6x8,
        text_color = Rgb565::BLACK,
        background_color = Rgb565::GREEN
    );

    // Create a text box above Ferris and draw it using style defined above
    Text::new(" Hello Rust! ", Point::new(42, 0))
        .into_styled(style)
        .draw(&mut lcd)
        .unwrap();
    
    // ---- LED Example ---- //
    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    let leds: [&mut dyn Led; 3] = [&mut red, &mut green, &mut blue];

    let mut delay = McycleDelay::new(&rcu.clocks);

    let mut i = 0;
    loop {
        let inext = (i + 1) % leds.len();
        leds[i].off();
        leds[inext].on();
        delay.delay_ms(500);

        i = inext;
    }
}
