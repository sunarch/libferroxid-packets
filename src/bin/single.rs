// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::util::MacAddr;


// Invoke as 'single <interface name>'
fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, _) =
        match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };
    
    let mut content: [u8;34] = [0, 1, 2, 3, 4, 5,
                                6, 7, 8, 9, 10, 11,
                                12, 13,
        0x45, // version 4, header length: 20
        0x00, // differentiated services
        0x00, 0x49, // total length
        0x0F, 0xC2, // identification: (4034)
        0x02, 0x40, // flags: don't fragment, fragment offset
        0x40, // Time to Live: 64
        0x11, // Protocol: UDP (17)
        0x2C, 0xAC, // Header Checksum: 0x2cac [validation disabled]
        0x7F, 0x00, 0x00, 0x01, // Source Address: 127.0.0.1
        0x7F, 0x00, 0x00, 0x01, // Destination Address: 127.0.0.1
    ];
    let mut packet = match MutableEthernetPacket::new(&mut content) {
        Some(x) => x,
        None => panic!(),
    };
    packet.set_source(MacAddr::default());
    packet.set_destination(MacAddr::default());
    packet.set_ethertype(EtherTypes::Ipv4);

    tx.send_to(packet.packet(), None);
}
