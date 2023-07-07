// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pnet::packet::ethernet::{EtherType,EtherTypes};


/// Q-in-Q Vlan Tagging \[IEEE 802.1Q\].
#[allow(dead_code)]
const ETHER_TYPE: EtherType = EtherTypes::QinQ; // 0x9100
