//! Connect SDA to P0.03, SCL to P0.04
//! $ DEFMT_LOG=info cargo rb compass

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use nrf_embassy as _; // global logger + panicking-behavior

use core::f32::consts::PI;
use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::interrupt;
use embassy_nrf::twim::{self, Twim};
use embassy_time::{Duration, Timer};
use libm::atan2;
use qmc5883l_async::*;

// Need correct magnetic declination for your location for accurate
// readings. See http://www.magnetic-declination.com/
const DECLINATION_RADS: f32 = 0.024434609;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let i2c = Twim::new(p.TWISPI0, irq, p.P0_03, p.P0_04, config);

    let mut compass = QMC5883L::new(i2c).await.unwrap();
    compass.continuous().await.unwrap();

    loop {
        if let Ok((x, y, z)) = compass.mag().await {
            let mut heading = atan2(y as f64, x as f64) as f32 + DECLINATION_RADS;
            if heading < 0.0 {
                heading += 2.0 * PI;
            } else if heading > 2.0 * PI {
                heading -= 2.0 * PI;
            }
            let heading_degrees = heading * 180.0 / PI;
            info!(
                "x={}, y={}, z={}: heading={} degrees",
                x, y, z, heading_degrees
            );
            Timer::after(Duration::from_millis(500)).await;
        }
    }
}
