use bitcoin::util::address::Address;
use bitcoin::util::bip32::ExtendedPubKey;

use serde::Deserialize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct HWIExtendedPubKey {
    pub xpub: ExtendedPubKey,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct HWISignature {
    pub signature: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct HWIAddress {
    pub address: Address,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct HWIPartiallySignedTransaction {
    pub psbt: String,
}

// TODO: use Descriptors
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct HWIDescriptor {
    pub internal: Vec<String>,
    pub receive: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct HWIKeyPoolElement {
    pub desc: String,
    pub range: Vec<u32>,
    pub timestamp: String,
    pub internal: bool,
    pub keypool: bool,
    pub watchonly: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub enum HWIAddressType {
    Pkh,
    ShWpkh,
    Wpkh,
}
