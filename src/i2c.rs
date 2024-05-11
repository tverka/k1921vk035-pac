#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sda: Sda,
    st: St,
    cst: Cst,
    ctl0: Ctl0,
    addr: Addr,
    ctl1: Ctl1,
    topr: Topr,
    ctl2: Ctl2,
    ctl3: Ctl3,
    ctl4: Ctl4,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn sda(&self) -> &Sda {
        &self.sda
    }
    #[doc = "0x04 - Status register"]
    #[inline(always)]
    pub const fn st(&self) -> &St {
        &self.st
    }
    #[doc = "0x08 - Status and control register"]
    #[inline(always)]
    pub const fn cst(&self) -> &Cst {
        &self.cst
    }
    #[doc = "0x0c - Control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x10 - Register own address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x14 - Control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x18 - Prescaler load register"]
    #[inline(always)]
    pub const fn topr(&self) -> &Topr {
        &self.topr
    }
    #[doc = "0x1c - Control register 2"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &Ctl2 {
        &self.ctl2
    }
    #[doc = "0x20 - Control register 3"]
    #[inline(always)]
    pub const fn ctl3(&self) -> &Ctl3 {
        &self.ctl3
    }
    #[doc = "0x24 - Control Register 4"]
    #[inline(always)]
    pub const fn ctl4(&self) -> &Ctl4 {
        &self.ctl4
    }
}
#[doc = "SDA (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda`]
module"]
#[doc(alias = "SDA")]
pub type Sda = crate::Reg<sda::SdaSpec>;
#[doc = "Data register"]
pub mod sda;
#[doc = "ST (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st`]
module"]
#[doc(alias = "ST")]
pub type St = crate::Reg<st::StSpec>;
#[doc = "Status register"]
pub mod st;
#[doc = "CST (rw) register accessor: Status and control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cst`]
module"]
#[doc(alias = "CST")]
pub type Cst = crate::Reg<cst::CstSpec>;
#[doc = "Status and control register"]
pub mod cst;
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "ADDR (rw) register accessor: Register own address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Register own address"]
pub mod addr;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "TOPR (rw) register accessor: Prescaler load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`topr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`topr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topr`]
module"]
#[doc(alias = "TOPR")]
pub type Topr = crate::Reg<topr::ToprSpec>;
#[doc = "Prescaler load register"]
pub mod topr;
#[doc = "CTL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`]
module"]
#[doc(alias = "CTL2")]
pub type Ctl2 = crate::Reg<ctl2::Ctl2Spec>;
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "CTL3 (rw) register accessor: Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl3`]
module"]
#[doc(alias = "CTL3")]
pub type Ctl3 = crate::Reg<ctl3::Ctl3Spec>;
#[doc = "Control register 3"]
pub mod ctl3;
#[doc = "CTL4 (rw) register accessor: Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl4`]
module"]
#[doc(alias = "CTL4")]
pub type Ctl4 = crate::Reg<ctl4::Ctl4Spec>;
#[doc = "Control Register 4"]
pub mod ctl4;
