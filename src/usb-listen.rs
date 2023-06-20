use std::{env, io, time::Duration};

use rusb::{Context, Device, HotplugBuilder, UsbContext};

struct HotPlugHandler;

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        println!("设备连接 {:?}", device);
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("设备离线 {:?}", device);
    }
}

struct HotDevice;

impl<T: UsbContext> rusb::Hotplug<T> for HotDevice {
    fn device_arrived(&mut self, device: Device<T>) {
        println!(
            "设备连接 {:#?}",
            device.device_descriptor().unwrap().product_id()
        );
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("设备离线 {:?}", device);
    }
}

fn main() -> rusb::Result<()> {
    // /dev/cu.usbmodem1301

    // listen_usb()?;
    Ok(())
}

fn listen_usb() -> rusb::Result<()> {
    if rusb::has_hotplug() {
        let context = Context::new()?;

        let mut reg = Some(
            HotplugBuilder::new()
                .enumerate(true)
                .register(&context, Box::new(HotDevice))?,
        );

        loop {
            context.handle_events(Some(Duration::from_secs(1))).unwrap();
            if let Some(reg) = reg.take() {
                println!("热插拔：{:?}", reg);
                context.unregister_callback(reg);
                listen_usb()?;
                break;
            }
        }
        Ok(())
    } else {
        eprint!("libusb hotplug api unsupported");
        Ok(())
    }
}
