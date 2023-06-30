// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use pnet::packet::ethernet::{EtherType,EtherTypes};


/// IETF TRILL Protocol \[IEEE\].
#[allow(dead_code)]
const ETHER_TYPE: EtherType = EtherTypes::Trill; // 0x22F3
