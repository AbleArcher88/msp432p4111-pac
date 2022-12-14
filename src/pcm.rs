#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Register"]
    pub pcmctl0: PCMCTL0,
    #[doc = "0x04 - Control 1 Register"]
    pub pcmctl1: PCMCTL1,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub pcmie: PCMIE,
    #[doc = "0x0c - Interrupt Flag Register"]
    pub pcmifg: PCMIFG,
    #[doc = "0x10 - Clear Interrupt Flag Register"]
    pub pcmclrifg: PCMCLRIFG,
}
#[doc = "PCMCTL0 (rw) register accessor: an alias for `Reg<PCMCTL0_SPEC>`"]
pub type PCMCTL0 = crate::Reg<pcmctl0::PCMCTL0_SPEC>;
#[doc = "Control 0 Register"]
pub mod pcmctl0;
#[doc = "PCMCTL1 (rw) register accessor: an alias for `Reg<PCMCTL1_SPEC>`"]
pub type PCMCTL1 = crate::Reg<pcmctl1::PCMCTL1_SPEC>;
#[doc = "Control 1 Register"]
pub mod pcmctl1;
#[doc = "PCMIE (rw) register accessor: an alias for `Reg<PCMIE_SPEC>`"]
pub type PCMIE = crate::Reg<pcmie::PCMIE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pcmie;
#[doc = "PCMIFG (r) register accessor: an alias for `Reg<PCMIFG_SPEC>`"]
pub type PCMIFG = crate::Reg<pcmifg::PCMIFG_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod pcmifg;
#[doc = "PCMCLRIFG (w) register accessor: an alias for `Reg<PCMCLRIFG_SPEC>`"]
pub type PCMCLRIFG = crate::Reg<pcmclrifg::PCMCLRIFG_SPEC>;
#[doc = "Clear Interrupt Flag Register"]
pub mod pcmclrifg;
