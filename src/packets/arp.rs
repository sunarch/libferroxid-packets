// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pnet::packet::ethernet::{EtherType,EtherTypes};


/// Address Resolution Protocol (ARP) \[RFC7042\].
#[allow(dead_code)]
const ETHER_TYPE: EtherType = EtherTypes::Arp; // 0x0806
