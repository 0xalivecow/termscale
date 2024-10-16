use serde_json::{Result, Value};
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

pub enum CurrentScreen {
    Main,
    Exiting,
}

struct Ip {
    ipv4: Ipv4Addr,
    ipv6: Ipv6Addr,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub self_address: Ip,
}

impl App {
    pub fn new(val: &Value) -> App {
        App {
            self_address: Ip {
                ipv4: Ipv4Addr::from_str(val["Self"]["TailscaleIPs"][0].as_str().unwrap()).unwrap(),
                ipv6: Ipv6Addr::from_str(val["Self"]["TailscaleIPs"][1].as_str().unwrap()).unwrap(),
            },
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn update_self_ip(&mut self, val: &Value) {
        self.self_address.ipv4;
        todo!()
    }
}
