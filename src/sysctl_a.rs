#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reboot Control Register"]
    pub sys_reboot_ctl: SYS_REBOOT_CTL,
    #[doc = "0x04 - NMI Control and Status Register"]
    pub sys_nmi_ctlstat: SYS_NMI_CTLSTAT,
    #[doc = "0x08 - Watchdog Reset Control Register"]
    pub sys_wdtreset_ctl: SYS_WDTRESET_CTL,
    #[doc = "0x0c - Peripheral Halt Control Register"]
    pub sys_perihalt_ctl: SYS_PERIHALT_CTL,
    #[doc = "0x10 - SRAM Size Register"]
    pub sys_sram_size: SYS_SRAM_SIZE,
    #[doc = "0x14 - SRAM Number of Banks Register"]
    pub sys_sram_numbanks: SYS_SRAM_NUMBANKS,
    #[doc = "0x18 - SRAM Number of Blocks Register"]
    pub sys_sram_numblocks: SYS_SRAM_NUMBLOCKS,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Flash Main Memory Size Register"]
    pub sys_mainflash_size: SYS_MAINFLASH_SIZE,
    #[doc = "0x24 - Flash Information Memory Size Register"]
    pub sys_infoflash_size: SYS_INFOFLASH_SIZE,
    _reserved9: [u8; 0x08],
    #[doc = "0x30 - Digital I/O Glitch Filter Control Register"]
    pub sys_dio_gltflt_ctl: SYS_DIO_GLTFLT_CTL,
    _reserved10: [u8; 0x0c],
    #[doc = "0x40 - IP Protected Secure Zone Data Access Unlock Register"]
    pub sys_secdata_unlock: SYS_SECDATA_UNLOCK,
    _reserved11: [u8; 0x0c],
    #[doc = "0x50 - SRAM Bank Enable Control Register 0"]
    pub sys_sram_banken_ctl0: SYS_SRAM_BANKEN_CTL0,
    #[doc = "0x54 - SRAM Bank Enable Control Register 1"]
    pub sys_sram_banken_ctl1: SYS_SRAM_BANKEN_CTL1,
    #[doc = "0x58 - SRAM Bank Enable Control Register 2"]
    pub sys_sram_banken_ctl2: SYS_SRAM_BANKEN_CTL2,
    #[doc = "0x5c - SRAM Bank Enable Control Register 3"]
    pub sys_sram_banken_ctl3: SYS_SRAM_BANKEN_CTL3,
    _reserved15: [u8; 0x10],
    #[doc = "0x70 - SRAM Block Retention Control Register 0"]
    pub sys_sram_blkret_ctl0: SYS_SRAM_BLKRET_CTL0,
    #[doc = "0x74 - SRAM Block Retention Control Register 1"]
    pub sys_sram_blkret_ctl1: SYS_SRAM_BLKRET_CTL1,
    #[doc = "0x78 - SRAM Block Retention Control Register 2"]
    pub sys_sram_blkret_ctl2: SYS_SRAM_BLKRET_CTL2,
    #[doc = "0x7c - SRAM Block Retention Control Register 3"]
    pub sys_sram_blkret_ctl3: SYS_SRAM_BLKRET_CTL3,
    _reserved19: [u8; 0x10],
    #[doc = "0x90 - SRAM Status Register"]
    pub sys_sram_stat: SYS_SRAM_STAT,
    _reserved20: [u8; 0x0f6c],
    #[doc = "0x1000 - Master Unlock Register"]
    pub sys_master_unlock: SYS_MASTER_UNLOCK,
    #[doc = "0x1004..0x100c - Boot Override Request Register"]
    pub sys_bootover_req: [SYS_BOOTOVER_REQ; 2],
    #[doc = "0x100c - Boot Override Acknowledge Register"]
    pub sys_bootover_ack: SYS_BOOTOVER_ACK,
    #[doc = "0x1010 - Reset Request Register"]
    pub sys_reset_req: SYS_RESET_REQ,
    #[doc = "0x1014 - Reset Status and Override Register"]
    pub sys_reset_statover: SYS_RESET_STATOVER,
    _reserved25: [u8; 0x08],
    #[doc = "0x1020 - System Status Register"]
    pub sys_system_stat: SYS_SYSTEM_STAT,
}
#[doc = "SYS_REBOOT_CTL (rw) register accessor: an alias for `Reg<SYS_REBOOT_CTL_SPEC>`"]
pub type SYS_REBOOT_CTL = crate::Reg<sys_reboot_ctl::SYS_REBOOT_CTL_SPEC>;
#[doc = "Reboot Control Register"]
pub mod sys_reboot_ctl;
#[doc = "SYS_NMI_CTLSTAT (rw) register accessor: an alias for `Reg<SYS_NMI_CTLSTAT_SPEC>`"]
pub type SYS_NMI_CTLSTAT = crate::Reg<sys_nmi_ctlstat::SYS_NMI_CTLSTAT_SPEC>;
#[doc = "NMI Control and Status Register"]
pub mod sys_nmi_ctlstat;
#[doc = "SYS_WDTRESET_CTL (rw) register accessor: an alias for `Reg<SYS_WDTRESET_CTL_SPEC>`"]
pub type SYS_WDTRESET_CTL = crate::Reg<sys_wdtreset_ctl::SYS_WDTRESET_CTL_SPEC>;
#[doc = "Watchdog Reset Control Register"]
pub mod sys_wdtreset_ctl;
#[doc = "SYS_PERIHALT_CTL (rw) register accessor: an alias for `Reg<SYS_PERIHALT_CTL_SPEC>`"]
pub type SYS_PERIHALT_CTL = crate::Reg<sys_perihalt_ctl::SYS_PERIHALT_CTL_SPEC>;
#[doc = "Peripheral Halt Control Register"]
pub mod sys_perihalt_ctl;
#[doc = "SYS_SRAM_SIZE (r) register accessor: an alias for `Reg<SYS_SRAM_SIZE_SPEC>`"]
pub type SYS_SRAM_SIZE = crate::Reg<sys_sram_size::SYS_SRAM_SIZE_SPEC>;
#[doc = "SRAM Size Register"]
pub mod sys_sram_size;
#[doc = "SYS_SRAM_NUMBANKS (r) register accessor: an alias for `Reg<SYS_SRAM_NUMBANKS_SPEC>`"]
pub type SYS_SRAM_NUMBANKS = crate::Reg<sys_sram_numbanks::SYS_SRAM_NUMBANKS_SPEC>;
#[doc = "SRAM Number of Banks Register"]
pub mod sys_sram_numbanks;
#[doc = "SYS_SRAM_NUMBLOCKS (r) register accessor: an alias for `Reg<SYS_SRAM_NUMBLOCKS_SPEC>`"]
pub type SYS_SRAM_NUMBLOCKS = crate::Reg<sys_sram_numblocks::SYS_SRAM_NUMBLOCKS_SPEC>;
#[doc = "SRAM Number of Blocks Register"]
pub mod sys_sram_numblocks;
#[doc = "SYS_MAINFLASH_SIZE (r) register accessor: an alias for `Reg<SYS_MAINFLASH_SIZE_SPEC>`"]
pub type SYS_MAINFLASH_SIZE = crate::Reg<sys_mainflash_size::SYS_MAINFLASH_SIZE_SPEC>;
#[doc = "Flash Main Memory Size Register"]
pub mod sys_mainflash_size;
#[doc = "SYS_INFOFLASH_SIZE (r) register accessor: an alias for `Reg<SYS_INFOFLASH_SIZE_SPEC>`"]
pub type SYS_INFOFLASH_SIZE = crate::Reg<sys_infoflash_size::SYS_INFOFLASH_SIZE_SPEC>;
#[doc = "Flash Information Memory Size Register"]
pub mod sys_infoflash_size;
#[doc = "SYS_DIO_GLTFLT_CTL (rw) register accessor: an alias for `Reg<SYS_DIO_GLTFLT_CTL_SPEC>`"]
pub type SYS_DIO_GLTFLT_CTL = crate::Reg<sys_dio_gltflt_ctl::SYS_DIO_GLTFLT_CTL_SPEC>;
#[doc = "Digital I/O Glitch Filter Control Register"]
pub mod sys_dio_gltflt_ctl;
#[doc = "SYS_SECDATA_UNLOCK (rw) register accessor: an alias for `Reg<SYS_SECDATA_UNLOCK_SPEC>`"]
pub type SYS_SECDATA_UNLOCK = crate::Reg<sys_secdata_unlock::SYS_SECDATA_UNLOCK_SPEC>;
#[doc = "IP Protected Secure Zone Data Access Unlock Register"]
pub mod sys_secdata_unlock;
#[doc = "SYS_SRAM_BANKEN_CTL0 (rw) register accessor: an alias for `Reg<SYS_SRAM_BANKEN_CTL0_SPEC>`"]
pub type SYS_SRAM_BANKEN_CTL0 = crate::Reg<sys_sram_banken_ctl0::SYS_SRAM_BANKEN_CTL0_SPEC>;
#[doc = "SRAM Bank Enable Control Register 0"]
pub mod sys_sram_banken_ctl0;
#[doc = "SYS_SRAM_BANKEN_CTL1 (rw) register accessor: an alias for `Reg<SYS_SRAM_BANKEN_CTL1_SPEC>`"]
pub type SYS_SRAM_BANKEN_CTL1 = crate::Reg<sys_sram_banken_ctl1::SYS_SRAM_BANKEN_CTL1_SPEC>;
#[doc = "SRAM Bank Enable Control Register 1"]
pub mod sys_sram_banken_ctl1;
#[doc = "SYS_SRAM_BANKEN_CTL2 (rw) register accessor: an alias for `Reg<SYS_SRAM_BANKEN_CTL2_SPEC>`"]
pub type SYS_SRAM_BANKEN_CTL2 = crate::Reg<sys_sram_banken_ctl2::SYS_SRAM_BANKEN_CTL2_SPEC>;
#[doc = "SRAM Bank Enable Control Register 2"]
pub mod sys_sram_banken_ctl2;
#[doc = "SYS_SRAM_BANKEN_CTL3 (rw) register accessor: an alias for `Reg<SYS_SRAM_BANKEN_CTL3_SPEC>`"]
pub type SYS_SRAM_BANKEN_CTL3 = crate::Reg<sys_sram_banken_ctl3::SYS_SRAM_BANKEN_CTL3_SPEC>;
#[doc = "SRAM Bank Enable Control Register 3"]
pub mod sys_sram_banken_ctl3;
#[doc = "SYS_SRAM_BLKRET_CTL0 (rw) register accessor: an alias for `Reg<SYS_SRAM_BLKRET_CTL0_SPEC>`"]
pub type SYS_SRAM_BLKRET_CTL0 = crate::Reg<sys_sram_blkret_ctl0::SYS_SRAM_BLKRET_CTL0_SPEC>;
#[doc = "SRAM Block Retention Control Register 0"]
pub mod sys_sram_blkret_ctl0;
#[doc = "SYS_SRAM_BLKRET_CTL1 (rw) register accessor: an alias for `Reg<SYS_SRAM_BLKRET_CTL1_SPEC>`"]
pub type SYS_SRAM_BLKRET_CTL1 = crate::Reg<sys_sram_blkret_ctl1::SYS_SRAM_BLKRET_CTL1_SPEC>;
#[doc = "SRAM Block Retention Control Register 1"]
pub mod sys_sram_blkret_ctl1;
#[doc = "SYS_SRAM_BLKRET_CTL2 (rw) register accessor: an alias for `Reg<SYS_SRAM_BLKRET_CTL2_SPEC>`"]
pub type SYS_SRAM_BLKRET_CTL2 = crate::Reg<sys_sram_blkret_ctl2::SYS_SRAM_BLKRET_CTL2_SPEC>;
#[doc = "SRAM Block Retention Control Register 2"]
pub mod sys_sram_blkret_ctl2;
#[doc = "SYS_SRAM_BLKRET_CTL3 (rw) register accessor: an alias for `Reg<SYS_SRAM_BLKRET_CTL3_SPEC>`"]
pub type SYS_SRAM_BLKRET_CTL3 = crate::Reg<sys_sram_blkret_ctl3::SYS_SRAM_BLKRET_CTL3_SPEC>;
#[doc = "SRAM Block Retention Control Register 3"]
pub mod sys_sram_blkret_ctl3;
#[doc = "SYS_SRAM_STAT (r) register accessor: an alias for `Reg<SYS_SRAM_STAT_SPEC>`"]
pub type SYS_SRAM_STAT = crate::Reg<sys_sram_stat::SYS_SRAM_STAT_SPEC>;
#[doc = "SRAM Status Register"]
pub mod sys_sram_stat;
#[doc = "SYS_MASTER_UNLOCK (rw) register accessor: an alias for `Reg<SYS_MASTER_UNLOCK_SPEC>`"]
pub type SYS_MASTER_UNLOCK = crate::Reg<sys_master_unlock::SYS_MASTER_UNLOCK_SPEC>;
#[doc = "Master Unlock Register"]
pub mod sys_master_unlock;
#[doc = "SYS_BOOTOVER_REQ (rw) register accessor: an alias for `Reg<SYS_BOOTOVER_REQ_SPEC>`"]
pub type SYS_BOOTOVER_REQ = crate::Reg<sys_bootover_req::SYS_BOOTOVER_REQ_SPEC>;
#[doc = "Boot Override Request Register"]
pub mod sys_bootover_req;
#[doc = "SYS_BOOTOVER_ACK (rw) register accessor: an alias for `Reg<SYS_BOOTOVER_ACK_SPEC>`"]
pub type SYS_BOOTOVER_ACK = crate::Reg<sys_bootover_ack::SYS_BOOTOVER_ACK_SPEC>;
#[doc = "Boot Override Acknowledge Register"]
pub mod sys_bootover_ack;
#[doc = "SYS_RESET_REQ (rw) register accessor: an alias for `Reg<SYS_RESET_REQ_SPEC>`"]
pub type SYS_RESET_REQ = crate::Reg<sys_reset_req::SYS_RESET_REQ_SPEC>;
#[doc = "Reset Request Register"]
pub mod sys_reset_req;
#[doc = "SYS_RESET_STATOVER (rw) register accessor: an alias for `Reg<SYS_RESET_STATOVER_SPEC>`"]
pub type SYS_RESET_STATOVER = crate::Reg<sys_reset_statover::SYS_RESET_STATOVER_SPEC>;
#[doc = "Reset Status and Override Register"]
pub mod sys_reset_statover;
#[doc = "SYS_SYSTEM_STAT (r) register accessor: an alias for `Reg<SYS_SYSTEM_STAT_SPEC>`"]
pub type SYS_SYSTEM_STAT = crate::Reg<sys_system_stat::SYS_SYSTEM_STAT_SPEC>;
#[doc = "System Status Register"]
pub mod sys_system_stat;
