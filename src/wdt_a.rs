#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - Watchdog Timer Control Register"]
    pub wdtctl: WDTCTL,
}
#[doc = "WDTCTL (rw) register accessor: an alias for `Reg<WDTCTL_SPEC>`"]
pub type WDTCTL = crate::Reg<wdtctl::WDTCTL_SPEC>;
#[doc = "Watchdog Timer Control Register"]
pub mod wdtctl;
