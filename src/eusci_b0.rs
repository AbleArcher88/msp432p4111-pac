#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    pub ucbx_ctlw0: UCBX_CTLW0,
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucbx_ctlw1: UCBX_CTLW1,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    pub ucbx_brw: UCBX_BRW,
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    pub ucbx_statw: UCBX_STATW,
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucbx_tbcnt: UCBX_TBCNT,
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    pub ucbx_rxbuf: UCBX_RXBUF,
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    pub ucbx_txbuf: UCBX_TXBUF,
    _reserved7: [u8; 0x04],
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucbx_i2coa0: UCBX_I2COA0,
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    pub ucbx_i2coa1: UCBX_I2COA1,
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    pub ucbx_i2coa2: UCBX_I2COA2,
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    pub ucbx_i2coa3: UCBX_I2COA3,
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    pub ucbx_addrx: UCBX_ADDRX,
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    pub ucbx_addmask: UCBX_ADDMASK,
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    pub ucbx_i2csa: UCBX_I2CSA,
    _reserved14: [u8; 0x08],
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    pub ucbx_ie: UCBX_IE,
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    pub ucbx_ifg: UCBX_IFG,
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    pub ucbx_iv: UCBX_IV,
}
#[doc = "UCBxCTLW0 (rw) register accessor: an alias for `Reg<UCBX_CTLW0_SPEC>`"]
pub type UCBX_CTLW0 = crate::Reg<ucbx_ctlw0::UCBX_CTLW0_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucbx_ctlw0;
#[doc = "UCBxCTLW1 (rw) register accessor: an alias for `Reg<UCBX_CTLW1_SPEC>`"]
pub type UCBX_CTLW1 = crate::Reg<ucbx_ctlw1::UCBX_CTLW1_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucbx_ctlw1;
#[doc = "UCBxBRW (rw) register accessor: an alias for `Reg<UCBX_BRW_SPEC>`"]
pub type UCBX_BRW = crate::Reg<ucbx_brw::UCBX_BRW_SPEC>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucbx_brw;
#[doc = "UCBxSTATW (rw) register accessor: an alias for `Reg<UCBX_STATW_SPEC>`"]
pub type UCBX_STATW = crate::Reg<ucbx_statw::UCBX_STATW_SPEC>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucbx_statw;
#[doc = "UCBxTBCNT (rw) register accessor: an alias for `Reg<UCBX_TBCNT_SPEC>`"]
pub type UCBX_TBCNT = crate::Reg<ucbx_tbcnt::UCBX_TBCNT_SPEC>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucbx_tbcnt;
#[doc = "UCBxRXBUF (r) register accessor: an alias for `Reg<UCBX_RXBUF_SPEC>`"]
pub type UCBX_RXBUF = crate::Reg<ucbx_rxbuf::UCBX_RXBUF_SPEC>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucbx_rxbuf;
#[doc = "UCBxTXBUF (rw) register accessor: an alias for `Reg<UCBX_TXBUF_SPEC>`"]
pub type UCBX_TXBUF = crate::Reg<ucbx_txbuf::UCBX_TXBUF_SPEC>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucbx_txbuf;
#[doc = "UCBxI2COA0 (rw) register accessor: an alias for `Reg<UCBX_I2COA0_SPEC>`"]
pub type UCBX_I2COA0 = crate::Reg<ucbx_i2coa0::UCBX_I2COA0_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucbx_i2coa0;
#[doc = "UCBxI2COA1 (rw) register accessor: an alias for `Reg<UCBX_I2COA1_SPEC>`"]
pub type UCBX_I2COA1 = crate::Reg<ucbx_i2coa1::UCBX_I2COA1_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucbx_i2coa1;
#[doc = "UCBxI2COA2 (rw) register accessor: an alias for `Reg<UCBX_I2COA2_SPEC>`"]
pub type UCBX_I2COA2 = crate::Reg<ucbx_i2coa2::UCBX_I2COA2_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucbx_i2coa2;
#[doc = "UCBxI2COA3 (rw) register accessor: an alias for `Reg<UCBX_I2COA3_SPEC>`"]
pub type UCBX_I2COA3 = crate::Reg<ucbx_i2coa3::UCBX_I2COA3_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucbx_i2coa3;
#[doc = "UCBxADDRX (r) register accessor: an alias for `Reg<UCBX_ADDRX_SPEC>`"]
pub type UCBX_ADDRX = crate::Reg<ucbx_addrx::UCBX_ADDRX_SPEC>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucbx_addrx;
#[doc = "UCBxADDMASK (rw) register accessor: an alias for `Reg<UCBX_ADDMASK_SPEC>`"]
pub type UCBX_ADDMASK = crate::Reg<ucbx_addmask::UCBX_ADDMASK_SPEC>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucbx_addmask;
#[doc = "UCBxI2CSA (rw) register accessor: an alias for `Reg<UCBX_I2CSA_SPEC>`"]
pub type UCBX_I2CSA = crate::Reg<ucbx_i2csa::UCBX_I2CSA_SPEC>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucbx_i2csa;
#[doc = "UCBxIE (rw) register accessor: an alias for `Reg<UCBX_IE_SPEC>`"]
pub type UCBX_IE = crate::Reg<ucbx_ie::UCBX_IE_SPEC>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucbx_ie;
#[doc = "UCBxIFG (rw) register accessor: an alias for `Reg<UCBX_IFG_SPEC>`"]
pub type UCBX_IFG = crate::Reg<ucbx_ifg::UCBX_IFG_SPEC>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucbx_ifg;
#[doc = "UCBxIV (r) register accessor: an alias for `Reg<UCBX_IV_SPEC>`"]
pub type UCBX_IV = crate::Reg<ucbx_iv::UCBX_IV_SPEC>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucbx_iv;
