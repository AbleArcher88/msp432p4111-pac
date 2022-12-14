#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Configuration Status"]
    pub dma_device_cfg: DMA_DEVICE_CFG,
    #[doc = "0x04 - Software Channel Trigger Register"]
    pub dma_sw_chtrig: DMA_SW_CHTRIG,
    _reserved2: [u8; 0x08],
    #[doc = "0x10..0x90 - Channel n Source Configuration Register"]
    pub dma_ch_srccfg: [DMA_CH_SRCCFG; 32],
    _reserved3: [u8; 0x70],
    #[doc = "0x100 - Interrupt 1 Source Channel Configuration"]
    pub dma_int1_srccfg: DMA_INT1_SRCCFG,
    #[doc = "0x104 - Interrupt 2 Source Channel Configuration Register"]
    pub dma_int2_srccfg: DMA_INT2_SRCCFG,
    #[doc = "0x108 - Interrupt 3 Source Channel Configuration Register"]
    pub dma_int3_srccfg: DMA_INT3_SRCCFG,
    _reserved6: [u8; 0x04],
    #[doc = "0x110 - Interrupt 0 Source Channel Flag Register"]
    pub dma_int0_srcflg: DMA_INT0_SRCFLG,
    #[doc = "0x114 - Interrupt 0 Source Channel Clear Flag Register"]
    pub dma_int0_clrflg: DMA_INT0_CLRFLG,
    _reserved8: [u8; 0x0ee8],
    #[doc = "0x1000 - Status Register"]
    pub dma_stat: DMA_STAT,
    #[doc = "0x1004 - Configuration Register"]
    pub dma_cfg: DMA_CFG,
    #[doc = "0x1008 - Channel Control Data Base Pointer Register"]
    pub dma_ctlbase: DMA_CTLBASE,
    #[doc = "0x100c - Channel Alternate Control Data Base Pointer Register"]
    pub dma_altbase: DMA_ALTBASE,
    #[doc = "0x1010 - Channel Wait on Request Status Register"]
    pub dma_waitstat: DMA_WAITSTAT,
    #[doc = "0x1014 - Channel Software Request Register"]
    pub dma_swreq: DMA_SWREQ,
    #[doc = "0x1018 - Channel Useburst Set Register"]
    pub dma_useburstset: DMA_USEBURSTSET,
    #[doc = "0x101c - Channel Useburst Clear Register"]
    pub dma_useburstclr: DMA_USEBURSTCLR,
    #[doc = "0x1020 - Channel Request Mask Set Register"]
    pub dma_reqmaskset: DMA_REQMASKSET,
    #[doc = "0x1024 - Channel Request Mask Clear Register"]
    pub dma_reqmaskclr: DMA_REQMASKCLR,
    #[doc = "0x1028 - Channel Enable Set Register"]
    pub dma_enaset: DMA_ENASET,
    #[doc = "0x102c - Channel Enable Clear Register"]
    pub dma_enaclr: DMA_ENACLR,
    #[doc = "0x1030 - Channel Primary-Alternate Set Register"]
    pub dma_altset: DMA_ALTSET,
    #[doc = "0x1034 - Channel Primary-Alternate Clear Register"]
    pub dma_altclr: DMA_ALTCLR,
    #[doc = "0x1038 - Channel Priority Set Register"]
    pub dma_prioset: DMA_PRIOSET,
    #[doc = "0x103c - Channel Priority Clear Register"]
    pub dma_prioclr: DMA_PRIOCLR,
    _reserved24: [u8; 0x0c],
    #[doc = "0x104c - Bus Error Clear Register"]
    pub dma_errclr: DMA_ERRCLR,
}
#[doc = "DMA_DEVICE_CFG (r) register accessor: an alias for `Reg<DMA_DEVICE_CFG_SPEC>`"]
pub type DMA_DEVICE_CFG = crate::Reg<dma_device_cfg::DMA_DEVICE_CFG_SPEC>;
#[doc = "Device Configuration Status"]
pub mod dma_device_cfg;
#[doc = "DMA_SW_CHTRIG (rw) register accessor: an alias for `Reg<DMA_SW_CHTRIG_SPEC>`"]
pub type DMA_SW_CHTRIG = crate::Reg<dma_sw_chtrig::DMA_SW_CHTRIG_SPEC>;
#[doc = "Software Channel Trigger Register"]
pub mod dma_sw_chtrig;
#[doc = "DMA_CH_SRCCFG (rw) register accessor: an alias for `Reg<DMA_CH_SRCCFG_SPEC>`"]
pub type DMA_CH_SRCCFG = crate::Reg<dma_ch_srccfg::DMA_CH_SRCCFG_SPEC>;
#[doc = "Channel n Source Configuration Register"]
pub mod dma_ch_srccfg;
#[doc = "DMA_INT1_SRCCFG (rw) register accessor: an alias for `Reg<DMA_INT1_SRCCFG_SPEC>`"]
pub type DMA_INT1_SRCCFG = crate::Reg<dma_int1_srccfg::DMA_INT1_SRCCFG_SPEC>;
#[doc = "Interrupt 1 Source Channel Configuration"]
pub mod dma_int1_srccfg;
#[doc = "DMA_INT2_SRCCFG (rw) register accessor: an alias for `Reg<DMA_INT2_SRCCFG_SPEC>`"]
pub type DMA_INT2_SRCCFG = crate::Reg<dma_int2_srccfg::DMA_INT2_SRCCFG_SPEC>;
#[doc = "Interrupt 2 Source Channel Configuration Register"]
pub mod dma_int2_srccfg;
#[doc = "DMA_INT3_SRCCFG (rw) register accessor: an alias for `Reg<DMA_INT3_SRCCFG_SPEC>`"]
pub type DMA_INT3_SRCCFG = crate::Reg<dma_int3_srccfg::DMA_INT3_SRCCFG_SPEC>;
#[doc = "Interrupt 3 Source Channel Configuration Register"]
pub mod dma_int3_srccfg;
#[doc = "DMA_INT0_SRCFLG (r) register accessor: an alias for `Reg<DMA_INT0_SRCFLG_SPEC>`"]
pub type DMA_INT0_SRCFLG = crate::Reg<dma_int0_srcflg::DMA_INT0_SRCFLG_SPEC>;
#[doc = "Interrupt 0 Source Channel Flag Register"]
pub mod dma_int0_srcflg;
#[doc = "DMA_INT0_CLRFLG (w) register accessor: an alias for `Reg<DMA_INT0_CLRFLG_SPEC>`"]
pub type DMA_INT0_CLRFLG = crate::Reg<dma_int0_clrflg::DMA_INT0_CLRFLG_SPEC>;
#[doc = "Interrupt 0 Source Channel Clear Flag Register"]
pub mod dma_int0_clrflg;
#[doc = "DMA_STAT (r) register accessor: an alias for `Reg<DMA_STAT_SPEC>`"]
pub type DMA_STAT = crate::Reg<dma_stat::DMA_STAT_SPEC>;
#[doc = "Status Register"]
pub mod dma_stat;
#[doc = "DMA_CFG (w) register accessor: an alias for `Reg<DMA_CFG_SPEC>`"]
pub type DMA_CFG = crate::Reg<dma_cfg::DMA_CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod dma_cfg;
#[doc = "DMA_CTLBASE (rw) register accessor: an alias for `Reg<DMA_CTLBASE_SPEC>`"]
pub type DMA_CTLBASE = crate::Reg<dma_ctlbase::DMA_CTLBASE_SPEC>;
#[doc = "Channel Control Data Base Pointer Register"]
pub mod dma_ctlbase;
#[doc = "DMA_ALTBASE (r) register accessor: an alias for `Reg<DMA_ALTBASE_SPEC>`"]
pub type DMA_ALTBASE = crate::Reg<dma_altbase::DMA_ALTBASE_SPEC>;
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod dma_altbase;
#[doc = "DMA_WAITSTAT (r) register accessor: an alias for `Reg<DMA_WAITSTAT_SPEC>`"]
pub type DMA_WAITSTAT = crate::Reg<dma_waitstat::DMA_WAITSTAT_SPEC>;
#[doc = "Channel Wait on Request Status Register"]
pub mod dma_waitstat;
#[doc = "DMA_SWREQ (w) register accessor: an alias for `Reg<DMA_SWREQ_SPEC>`"]
pub type DMA_SWREQ = crate::Reg<dma_swreq::DMA_SWREQ_SPEC>;
#[doc = "Channel Software Request Register"]
pub mod dma_swreq;
#[doc = "DMA_USEBURSTSET (rw) register accessor: an alias for `Reg<DMA_USEBURSTSET_SPEC>`"]
pub type DMA_USEBURSTSET = crate::Reg<dma_useburstset::DMA_USEBURSTSET_SPEC>;
#[doc = "Channel Useburst Set Register"]
pub mod dma_useburstset;
#[doc = "DMA_USEBURSTCLR (w) register accessor: an alias for `Reg<DMA_USEBURSTCLR_SPEC>`"]
pub type DMA_USEBURSTCLR = crate::Reg<dma_useburstclr::DMA_USEBURSTCLR_SPEC>;
#[doc = "Channel Useburst Clear Register"]
pub mod dma_useburstclr;
#[doc = "DMA_REQMASKSET (rw) register accessor: an alias for `Reg<DMA_REQMASKSET_SPEC>`"]
pub type DMA_REQMASKSET = crate::Reg<dma_reqmaskset::DMA_REQMASKSET_SPEC>;
#[doc = "Channel Request Mask Set Register"]
pub mod dma_reqmaskset;
#[doc = "DMA_REQMASKCLR (w) register accessor: an alias for `Reg<DMA_REQMASKCLR_SPEC>`"]
pub type DMA_REQMASKCLR = crate::Reg<dma_reqmaskclr::DMA_REQMASKCLR_SPEC>;
#[doc = "Channel Request Mask Clear Register"]
pub mod dma_reqmaskclr;
#[doc = "DMA_ENASET (rw) register accessor: an alias for `Reg<DMA_ENASET_SPEC>`"]
pub type DMA_ENASET = crate::Reg<dma_enaset::DMA_ENASET_SPEC>;
#[doc = "Channel Enable Set Register"]
pub mod dma_enaset;
#[doc = "DMA_ENACLR (w) register accessor: an alias for `Reg<DMA_ENACLR_SPEC>`"]
pub type DMA_ENACLR = crate::Reg<dma_enaclr::DMA_ENACLR_SPEC>;
#[doc = "Channel Enable Clear Register"]
pub mod dma_enaclr;
#[doc = "DMA_ALTSET (rw) register accessor: an alias for `Reg<DMA_ALTSET_SPEC>`"]
pub type DMA_ALTSET = crate::Reg<dma_altset::DMA_ALTSET_SPEC>;
#[doc = "Channel Primary-Alternate Set Register"]
pub mod dma_altset;
#[doc = "DMA_ALTCLR (w) register accessor: an alias for `Reg<DMA_ALTCLR_SPEC>`"]
pub type DMA_ALTCLR = crate::Reg<dma_altclr::DMA_ALTCLR_SPEC>;
#[doc = "Channel Primary-Alternate Clear Register"]
pub mod dma_altclr;
#[doc = "DMA_PRIOSET (rw) register accessor: an alias for `Reg<DMA_PRIOSET_SPEC>`"]
pub type DMA_PRIOSET = crate::Reg<dma_prioset::DMA_PRIOSET_SPEC>;
#[doc = "Channel Priority Set Register"]
pub mod dma_prioset;
#[doc = "DMA_PRIOCLR (w) register accessor: an alias for `Reg<DMA_PRIOCLR_SPEC>`"]
pub type DMA_PRIOCLR = crate::Reg<dma_prioclr::DMA_PRIOCLR_SPEC>;
#[doc = "Channel Priority Clear Register"]
pub mod dma_prioclr;
#[doc = "DMA_ERRCLR (rw) register accessor: an alias for `Reg<DMA_ERRCLR_SPEC>`"]
pub type DMA_ERRCLR = crate::Reg<dma_errclr::DMA_ERRCLR_SPEC>;
#[doc = "Bus Error Clear Register"]
pub mod dma_errclr;
