#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    addr: Addr,
    data: [Data; 2],
    _reserved2: [u8; 0x18],
    cmd: Cmd,
    stat: Stat,
    ctrl: Ctrl,
    _reserved5: [u8; 0x04],
    icstat: Icstat,
    dcstat: Dcstat,
    _reserved7: [u8; 0x3c],
    bdis: Bdis,
}
impl RegisterBlock {
    #[doc = "0x00 - Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04..0x0c - DATA"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - DATA"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
    #[doc = "0x24 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x28 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x2c - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x34 - ICACHE Status Register"]
    #[inline(always)]
    pub const fn icstat(&self) -> &Icstat {
        &self.icstat
    }
    #[doc = "0x38 - DCACHE Status Register"]
    #[inline(always)]
    pub const fn dcstat(&self) -> &Dcstat {
        &self.dcstat
    }
    #[doc = "0x78 - Boot Mode Disable register"]
    #[inline(always)]
    pub const fn bdis(&self) -> &Bdis {
        &self.bdis
    }
}
#[doc = "ADDR (rw) register accessor: Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address Register"]
pub mod addr;
#[doc = "DATA"]
pub use self::data::Data;
#[doc = r"Cluster"]
#[doc = "DATA"]
pub mod data;
#[doc = "CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STAT (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ICSTAT (r) register accessor: ICACHE Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icstat`]
module"]
#[doc(alias = "ICSTAT")]
pub type Icstat = crate::Reg<icstat::IcstatSpec>;
#[doc = "ICACHE Status Register"]
pub mod icstat;
#[doc = "DCSTAT (r) register accessor: DCACHE Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcstat`]
module"]
#[doc(alias = "DCSTAT")]
pub type Dcstat = crate::Reg<dcstat::DcstatSpec>;
#[doc = "DCACHE Status Register"]
pub mod dcstat;
#[doc = "BDIS (rw) register accessor: Boot Mode Disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdis`]
module"]
#[doc(alias = "BDIS")]
pub type Bdis = crate::Reg<bdis::BdisSpec>;
#[doc = "Boot Mode Disable register"]
pub mod bdis;
