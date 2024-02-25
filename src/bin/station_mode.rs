#![no_std]
#![no_main]

// use esp32c3_hal::{aes::Mode, clock::ClockControl, peripherals::Peripherals, prelude::*, systimer::SystemTimer, Delay, Rng};

// use embedded_io::*;
// use embedded_svc::{
//     ipv4::Interface,
//     wifi::{AccessPointInfo, AuthMethod, ClientConfiguration, Configuration, Wifi},
// };

// use esp_println::{print, println};
// use esp_wifi::{
//     current_millis, initialize,
//     wifi::{utils::create_network_interface, WifiError, WifiMode, WifiStaDevice},
//     wifi_interface::WifiStack,
//     EspWifiInitFor,
// };
// use smoltcp::{
//     iface::SocketStorage,
//     wire::{IpAddress, Ipv4Address},
// };

// Minimal imports for an empty main to run
use esp32c3_hal::prelude::*;
use esp_backtrace as _;
#[entry]
fn main() -> ! {
    panic!("Empty main");
}

// // use thiserror_no_std::Error;

// // const SSID: &str = env!("SSID");
// // const PASSWORD: &str = env!("PASSWORD");

// // #[derive(Debug, Error)]
// // pub enum WifiControllerError {
// //     #[error("Cannot connect to wifi if the controller is not in AP_STA or STA mode")]
// //     NotStationMode
// // }

// // pub struct WifiController {
// //     pub ssid: String,
// //     pub password: String,
// //     pub mode: WifiMode
// // }

// // impl WifiController {
// //     pub fn new(ssid: String, password: String, mode:WifiMode) -> WifiController {
// //         WifiController {
// //             ssid,
// //             password,
// //             mode,
// //         }
// //     }

// //     pub fn connect_to_wifi(&self) -> Result<(), WifiControllerError> {
// //         if self.mode != WifiMode::ApSta || self.mode != WifiMode::Sta {
// //             return Err(WifiControllerError::NotStationMode)
// //         }

// //         Ok(())
// //     }
// // }

// // #[entry]
// // fn main() -> ! {
// //     let peripherals = Peripherals::take();

// //     let system = peripherals.SYSTEM.split();
// //     let clocks = ClockControl::max(system.clock_control).freeze();
// //     let mut delay = Delay::new(&clocks);

// //     #[cfg(target_arch = "riscv32")]
// //     let timer = esp32c3_hal::systimer::SystemTimer::new(peripherals.SYSTIMER).alarm0;
// //     let init = initialize(
// //         EspWifiInitFor::Wifi,
// //         timer,
// //         Rng::new(peripherals.RNG),
// //         system.radio_clock_control,
// //         &clocks,
// //     )
// //     .unwrap();

// //     let wifi = peripherals.WIFI;
// //     let mut socket_set_entries: [SocketStorage; 3] = Default::default();
// //     let (iface, device, mut controller, sockets) =
// //         create_network_interface(&init, wifi, WifiStaDevice, &mut socket_set_entries).unwrap();
// //     let wifi_stack = WifiStack::new(iface, device, sockets, current_millis);

// //     let client_config = Configuration::Client(ClientConfiguration {
// //         ssid: SSID.try_into().unwrap(),
// //         password: PASSWORD.try_into().unwrap(),
// //         ..Default::default()
// //     });
// //     let res = controller.set_configuration(&client_config);
// //     println!("wifi_set_configuration returned {:?}", res);

// //     controller.start().unwrap();
// //     println!("is wifi started: {:?}", controller.is_started());

// //     println!("Start Wifi Scan");
// //     let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> = controller.scan_n();
// //     if let Ok((res, _count)) = res {
// //         for ap in res {
// //             println!("{:?}", ap);
// //         }
// //     }

// //     println!("{:?}", controller.get_capabilities());

// //     // wait to get connected
// //     println!("Wait to get connected");
// //     loop {
// //         while controller.connect().is_err() {}

// //         let res = controller.is_connected();
// //         match res {
// //             Ok(connected) => {
// //                 if connected {
// //                     break;
// //                 }
// //             }
// //             Err(err) => {
// //                 println!("{:?}", err);
// //                 delay.delay_ms(1000u32);
// //                 continue;
// //             }
// //         }
// //     }
// //     println!("{:?}", controller.is_connected());

// //     // wait for getting an ip address
// //     println!("Wait to get an ip address");
// //     loop {
// //         wifi_stack.work();

// //         if wifi_stack.is_iface_up() {
// //             println!("got ip {:?}", wifi_stack.get_ip_info());
// //             break;
// //         }
// //     }

// //     println!("Start busy loop on main");
// //     loop {

// //     }
// // }
