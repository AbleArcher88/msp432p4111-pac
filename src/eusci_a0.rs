#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    pub ucax_ctlw0: UCAX_CTLW0,
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub ucax_ctlw1: UCAX_CTLW1,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    pub ucax_brw: UCAX_BRW,
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub ucax_mctlw: UCAX_MCTLW,
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    pub ucax_statw: UCAX_STATW,
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    pub ucax_rxbuf: UCAX_RXBUF,
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    pub ucax_txbuf: UCAX_TXBUF,
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub ucax_abctl: UCAX_ABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub ucax_irctl: UCAX_IRCTL,
    _reserved9: [u8; 0x06],
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    pub ucax_ie: UCAX_IE,
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    pub ucax_ifg: UCAX_IFG,
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    pub ucax_iv: UCAX_IV,
}
#[doc = "UCAxCTLW0 (rw) register accessor: an alias for `Reg<UCAX_CTLW0_SPEC>`"]
pub type UCAX_CTLW0 = crate::Reg<ucax_ctlw0::UCAX_CTLW0_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod ucax_ctlw0;
#[doc = "UCAxCTLW1 (rw) register accessor: an alias for `Reg<UCAX_CTLW1_SPEC>`"]
pub type UCAX_CTLW1 = crate::Reg<ucax_ctlw1::UCAX_CTLW1_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod ucax_ctlw1;
#[doc = "UCAxBRW (rw) register accessor: an alias for `Reg<UCAX_BRW_SPEC>`"]
pub type UCAX_BRW = crate::Reg<ucax_brw::UCAX_BRW_SPEC>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod ucax_brw;
#[doc = "UCAxMCTLW (rw) register accessor: an alias for `Reg<UCAX_MCTLW_SPEC>`"]
pub type UCAX_MCTLW = crate::Reg<ucax_mctlw::UCAX_MCTLW_SPEC>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod ucax_mctlw;
#[doc = "UCAxSTATW (rw) register accessor: an alias for `Reg<UCAX_STATW_SPEC>`"]
pub type UCAX_STATW = crate::Reg<ucax_statw::UCAX_STATW_SPEC>;
#[doc = "eUSCI_Ax Status Register"]
pub mod ucax_statw;
#[doc = "UCAxRXBUF (r) register accessor: an alias for `Reg<UCAX_RXBUF_SPEC>`"]
pub type UCAX_RXBUF = crate::Reg<ucax_rxbuf::UCAX_RXBUF_SPEC>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod ucax_rxbuf;
#[doc = "UCAxTXBUF (rw) register accessor: an alias for `Reg<UCAX_TXBUF_SPEC>`"]
pub type UCAX_TXBUF = crate::Reg<ucax_txbuf::UCAX_TXBUF_SPEC>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod ucax_txbuf;
#[doc = "UCAxABCTL (rw) register accessor: an alias for `Reg<UCAX_ABCTL_SPEC>`"]
pub type UCAX_ABCTL = crate::Reg<ucax_abctl::UCAX_ABCTL_SPEC>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod ucax_abctl;
#[doc = "UCAxIRCTL (rw) register accessor: an alias for `Reg<UCAX_IRCTL_SPEC>`"]
pub type UCAX_IRCTL = crate::Reg<ucax_irctl::UCAX_IRCTL_SPEC>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod ucax_irctl;
#[doc = "UCAxIE (rw) register accessor: an alias for `Reg<UCAX_IE_SPEC>`"]
pub type UCAX_IE = crate::Reg<ucax_ie::UCAX_IE_SPEC>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod ucax_ie;
#[doc = "UCAxIFG (rw) register accessor: an alias for `Reg<UCAX_IFG_SPEC>`"]
pub type UCAX_IFG = crate::Reg<ucax_ifg::UCAX_IFG_SPEC>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod ucax_ifg;
#[doc = "UCAxIV (r) register accessor: an alias for `Reg<UCAX_IV_SPEC>`"]
pub type UCAX_IV = crate::Reg<ucax_iv::UCAX_IV_SPEC>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod ucax_iv;
