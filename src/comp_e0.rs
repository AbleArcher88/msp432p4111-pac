#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cex_ctl0: CEX_CTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cex_ctl1: CEX_CTL1,
    #[doc = "0x04 - Comparator Control Register 2"]
    pub cex_ctl2: CEX_CTL2,
    #[doc = "0x06 - Comparator Control Register 3"]
    pub cex_ctl3: CEX_CTL3,
    _reserved4: [u8; 0x04],
    #[doc = "0x0c - Comparator Interrupt Control Register"]
    pub cex_int: CEX_INT,
    #[doc = "0x0e - Comparator Interrupt Vector Word Register"]
    pub cex_iv: CEX_IV,
}
#[doc = "CExCTL0 (rw) register accessor: an alias for `Reg<CEX_CTL0_SPEC>`"]
pub type CEX_CTL0 = crate::Reg<cex_ctl0::CEX_CTL0_SPEC>;
#[doc = "Comparator Control Register 0"]
pub mod cex_ctl0;
#[doc = "CExCTL1 (rw) register accessor: an alias for `Reg<CEX_CTL1_SPEC>`"]
pub type CEX_CTL1 = crate::Reg<cex_ctl1::CEX_CTL1_SPEC>;
#[doc = "Comparator Control Register 1"]
pub mod cex_ctl1;
#[doc = "CExCTL2 (rw) register accessor: an alias for `Reg<CEX_CTL2_SPEC>`"]
pub type CEX_CTL2 = crate::Reg<cex_ctl2::CEX_CTL2_SPEC>;
#[doc = "Comparator Control Register 2"]
pub mod cex_ctl2;
#[doc = "CExCTL3 (rw) register accessor: an alias for `Reg<CEX_CTL3_SPEC>`"]
pub type CEX_CTL3 = crate::Reg<cex_ctl3::CEX_CTL3_SPEC>;
#[doc = "Comparator Control Register 3"]
pub mod cex_ctl3;
#[doc = "CExINT (rw) register accessor: an alias for `Reg<CEX_INT_SPEC>`"]
pub type CEX_INT = crate::Reg<cex_int::CEX_INT_SPEC>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cex_int;
#[doc = "CExIV (r) register accessor: an alias for `Reg<CEX_IV_SPEC>`"]
pub type CEX_IV = crate::Reg<cex_iv::CEX_IV_SPEC>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cex_iv;
