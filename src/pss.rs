#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key Register"]
    pub psskey: PSSKEY,
    #[doc = "0x04 - Control 0 Register"]
    pub pssctl0: PSSCTL0,
    _reserved2: [u8; 0x2c],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub pssie: PSSIE,
    #[doc = "0x38 - Interrupt Flag Register"]
    pub pssifg: PSSIFG,
    #[doc = "0x3c - Clear Interrupt Flag Register"]
    pub pssclrifg: PSSCLRIFG,
}
#[doc = "PSSKEY (rw) register accessor: an alias for `Reg<PSSKEY_SPEC>`"]
pub type PSSKEY = crate::Reg<psskey::PSSKEY_SPEC>;
#[doc = "Key Register"]
pub mod psskey;
#[doc = "PSSCTL0 (rw) register accessor: an alias for `Reg<PSSCTL0_SPEC>`"]
pub type PSSCTL0 = crate::Reg<pssctl0::PSSCTL0_SPEC>;
#[doc = "Control 0 Register"]
pub mod pssctl0;
#[doc = "PSSIE (rw) register accessor: an alias for `Reg<PSSIE_SPEC>`"]
pub type PSSIE = crate::Reg<pssie::PSSIE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pssie;
#[doc = "PSSIFG (r) register accessor: an alias for `Reg<PSSIFG_SPEC>`"]
pub type PSSIFG = crate::Reg<pssifg::PSSIFG_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod pssifg;
#[doc = "PSSCLRIFG (rw) register accessor: an alias for `Reg<PSSCLRIFG_SPEC>`"]
pub type PSSCLRIFG = crate::Reg<pssclrifg::PSSCLRIFG_SPEC>;
#[doc = "Clear Interrupt Flag Register"]
pub mod pssclrifg;
