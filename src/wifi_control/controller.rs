use embedded_svc::{
    ipv4::{
        ClientConfiguration as Ipv4ClientConfiguration, 
        Configuration as Ipv4Configuration,
        ClientSettings as Ipv4ClientSettings,
        Ipv4Addr,
        Subnet as Ipv4Subnet,
        Mask as Ipv4Mask,
        Interface,
    },
    io::{Read, Write},
    wifi::{AccessPointConfiguration, AccessPointInfo, ClientConfiguration, Configuration, Wifi},
};

use esp32c3_hal::{
    clock::Clocks,
    peripherals::{RNG, SYSTIMER, WIFI},
    prelude::*,
    system::RadioClockControl,
    Delay, Rng,
};

use esp_wifi::{
    current_millis, initialize,
    wifi::{
        utils::{create_ap_sta_network_interface, create_network_interface, ApStaInterface},
        WifiError, WifiMode, WifiStaDevice,
    },
    wifi_interface::WifiStack,
    EspWifiInitFor, InitializationError,
};

use esp_backtrace as _;
use esp_println::println;

use smoltcp::iface::SocketStorage;
use thiserror_no_std::Error;

#[derive(Debug, Error)]
pub enum WifiControllerError {
    #[error("Cannot connect to wifi if the controller is not in AP_STA or STA mode")]
    NotStationMode,

    #[error("Failed to initialize Wifi")]
    WifiInitError(#[from] InitializationError),

    #[error("Access Point only mode does not allow the device to connect to a wifi network")]
    WifiConnectionInAP,
}

pub struct WifiController<'a> {
    pub ssid: &'a str,
    pub password: &'a str,
    pub mode: WifiMode,
}

impl<'a> WifiController<'a> {
    pub fn new(ssid: &'a str, password: &'a str, mode: WifiMode) -> WifiController<'a> {
        WifiController {
            ssid,
            password,
            mode,
        }
    }

    pub fn connect_to_wifi_sta(
        &self,
        sys_timer_peripheral: SYSTIMER,
        rng_peripheral: RNG,
        radio_clock_control: RadioClockControl,
        clocks: &Clocks,
        wifi: WIFI,
        delay: &mut Delay,
    ) -> Result<(), WifiControllerError> {
        let timer = esp32c3_hal::systimer::SystemTimer::new(sys_timer_peripheral).alarm0;

        let init = initialize(
            EspWifiInitFor::Wifi,
            timer,
            Rng::new(rng_peripheral),
            radio_clock_control,
            clocks,
        )?;

        let mut socket_set_entries: [SocketStorage; 3] = Default::default();
        let (iface, device, mut controller, sockets) =
            create_network_interface(&init, wifi, WifiStaDevice, &mut socket_set_entries).unwrap();
        let wifi_stack = WifiStack::new(iface, device, sockets, current_millis);

        let client_config = Configuration::Client(ClientConfiguration {
            ssid: self.ssid.try_into().unwrap(),
            password: self.password.try_into().unwrap(),
            ..Default::default()
        });
        let res = controller.set_configuration(&client_config);

        controller.start().unwrap();

        println!("Start Wifi Scan");
        let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> =
            controller.scan_n();
        if let Ok((res, _count)) = res {
            for ap in res {
                println!("{:?}", ap);
            }
        }

        println!("{:?}", controller.get_capabilities());

        // wait to get connected
        println!("Wait to get connected");
        loop {
            while controller.connect().is_err() {}

            let res = controller.is_connected();
            match res {
                Ok(connected) => {
                    if connected {
                        break;
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    delay.delay_ms(1000u32);
                    continue;
                }
            }
        }
        println!("{:?}", controller.is_connected());

        // wait for getting an ip address
        println!("Wait to get an ip address");
        loop {
            wifi_stack.work();

            if wifi_stack.is_iface_up() {
                println!("got ip {:?}", wifi_stack.get_ip_info());
                break;
            }
        }

        Ok(())
    }

    pub fn connect_to_wifi_apsta(
        &self,
        sys_timer_peripheral: SYSTIMER,
        rng_peripheral: RNG,
        radio_clock_control: RadioClockControl,
        clocks: &Clocks,
        wifi: WIFI,
        delay: &mut Delay,
    ) -> Result<(), WifiControllerError> {
        let timer = esp32c3_hal::systimer::SystemTimer::new(sys_timer_peripheral).alarm0;

        let init = initialize(
            EspWifiInitFor::Wifi,
            timer,
            Rng::new(rng_peripheral),
            radio_clock_control,
            &clocks,
        )
        .unwrap();

        let mut ap_socket_set_entries: [SocketStorage; 3] = Default::default();
        let mut sta_socket_set_entries: [SocketStorage; 3] = Default::default();

        let ApStaInterface {
            ap_interface,
            sta_interface,
            ap_device,
            sta_device,
            mut controller,
            ap_socket_set,
            sta_socket_set,
        } = create_ap_sta_network_interface(
            &init,
            wifi,
            &mut ap_socket_set_entries,
            &mut sta_socket_set_entries,
        )
        .unwrap();

        let mut wifi_ap_stack =
            WifiStack::new(ap_interface, ap_device, ap_socket_set, current_millis);
        let wifi_sta_stack =
            WifiStack::new(sta_interface, sta_device, sta_socket_set, current_millis);

        let client_config = Configuration::Mixed(
            ClientConfiguration {
                ssid: self.ssid.try_into().unwrap(),
                password: self.password.try_into().unwrap(),
                ..Default::default()
            },
            AccessPointConfiguration {
                ssid: "esp-wifi".try_into().unwrap(),
                ..Default::default()
            },
        );

        let res = controller.set_configuration(&client_config);
        println!("wifi_set_configuration returned {:?}", res);

        controller.start().unwrap();
        println!("is wifi started: {:?}", controller.is_started());

        println!("{:?}", controller.get_capabilities());

        wifi_ap_stack
            .set_iface_configuration(&Ipv4Configuration::Client(
                Ipv4ClientConfiguration::Fixed(
                    Ipv4ClientSettings {
                        ip: Ipv4Addr::from(WifiController::parse_ip("192.168.2.1")),
                        subnet:Ipv4Subnet {
                            gateway: Ipv4Addr::from(WifiController::parse_ip("192.168.2.1")),
                            mask: Ipv4Mask(24),
                        },
                        dns: None,
                        secondary_dns: None,
                    },
                ),
            ))
            .unwrap();

        println!("wifi_connect {:?}", controller.connect());

        // wait for STA getting an ip address
        println!("Wait to get an ip address");
        loop {
            wifi_sta_stack.work();

            if wifi_sta_stack.is_iface_up() {
                println!("got ip {:?}", wifi_sta_stack.get_ip_info());
                break;
            }
        }

        println!("Start busy loop on main. Connect to the AP `esp-wifi` and point your browser to http://192.168.2.1:8080/");
        println!("Use a static IP in the range 192.168.2.2 .. 192.168.2.255, use gateway 192.168.2.1");

        // web server part
        
        let mut rx_buffer = [0u8; 1536];
        let mut tx_buffer = [0u8; 1536];
        let mut socket = wifi_ap_stack.get_socket(&mut rx_buffer, &mut tx_buffer);
    
        socket.listen(8080).unwrap();
    
        loop {
            socket.work();
    
            if !socket.is_open() {
                socket.listen(8080).unwrap();
            }
    
            if socket.is_connected() {
                println!("Connected");
    
                let mut time_out = false;
                let wait_end = current_millis() + 20 * 1000;
                let mut buffer = [0u8; 1024];
                let mut pos = 0;
                loop {
                    if let Ok(len) = socket.read(&mut buffer[pos..]) {
                        let to_print =
                            unsafe { core::str::from_utf8_unchecked(&buffer[..(pos + len)]) };
    
                        if to_print.contains("\r\n\r\n") {
                            println!("{}", to_print);
                            println!();
                            break;
                        }
    
                        pos += len;
                    } else {
                        break;
                    }
    
                    if current_millis() > wait_end {
                        println!("Timeout");
                        time_out = true;
                        break;
                    }
                }
    
                if !time_out {
                    socket
                        .write_all(
                            b"HTTP/1.0 200 OK\r\n\r\n\
                        <html>\
                            <body>\
                                <h1>Hello Rust! Hello esp-wifi!</h1>\
                            </body>\
                        </html>\r\n\
                        ",
                        )
                        .unwrap();
    
                    socket.flush().unwrap();
                }
    
                socket.close();
    
                println!("Done\n");
                println!();
            }
    
            let wait_end = current_millis() + 5 * 1000;
            while current_millis() < wait_end {
                socket.work();
            }
        }

        Ok(())
    }

    // Todo: move all these parameters to the WifiController constructor
    pub fn connect_to_wifi(
        &self,
        sys_timer_peripheral: SYSTIMER,
        rng_peripheral: RNG,
        radio_clock_control: RadioClockControl,
        clocks: &Clocks,
        wifi: WIFI,
        delay: &mut Delay,
    ) -> Result<(), WifiControllerError> {
        // Do not connect to a ssid and pass if the controller is not
        // in Access Point - Station or Station mode

        match self.mode {
            WifiMode::Sta => self.connect_to_wifi(
                sys_timer_peripheral,
                rng_peripheral,
                radio_clock_control,
                clocks,
                wifi,
                delay,
            ),
            WifiMode::Ap => Err(WifiControllerError::WifiConnectionInAP),
            WifiMode::ApSta => self.connect_to_wifi_apsta(
                sys_timer_peripheral,
                rng_peripheral,
                radio_clock_control,
                clocks,
                wifi,
                delay,
            ),
        }
    }
    
    fn parse_ip(ip: &str) -> [u8; 4] {
        let mut result = [0u8; 4];
        for (idx, octet) in ip.split(".").into_iter().enumerate() {
            result[idx] = u8::from_str_radix(octet, 10).unwrap();
        }
        result
    }
}
