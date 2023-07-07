// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pnet::packet::ethernet::{EtherType,EtherTypes};


/// Reverse Address Resolution Protocol (RARP) \[RFC903\].
#[allow(dead_code)]
const ETHER_TYPE: EtherType = EtherTypes::Rarp; // 0x8035
