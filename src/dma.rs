#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    cfg: Cfg,
    baseptr: Baseptr,
    altbaseptr: Altbaseptr,
    waitonreq: Waitonreq,
    swreq: Swreq,
    useburstset: Useburstset,
    useburstclr: Useburstclr,
    reqmaskset: Reqmaskset,
    reqmaskclr: Reqmaskclr,
    enset: Enset,
    enclr: Enclr,
    prialtset: Prialtset,
    prialtclr: Prialtclr,
    priorityset: Priorityset,
    priorityclr: Priorityclr,
    _reserved16: [u8; 0x0c],
    errclr: Errclr,
}
impl RegisterBlock {
    #[doc = "0x00 - Status DMA register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - DMA configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Channel control data base pointer"]
    #[inline(always)]
    pub const fn baseptr(&self) -> &Baseptr {
        &self.baseptr
    }
    #[doc = "0x0c - Channel alternate control data base pointer"]
    #[inline(always)]
    pub const fn altbaseptr(&self) -> &Altbaseptr {
        &self.altbaseptr
    }
    #[doc = "0x10 - Channel wait on request status"]
    #[inline(always)]
    pub const fn waitonreq(&self) -> &Waitonreq {
        &self.waitonreq
    }
    #[doc = "0x14 - Channel software request"]
    #[inline(always)]
    pub const fn swreq(&self) -> &Swreq {
        &self.swreq
    }
    #[doc = "0x18 - Channel useburst set"]
    #[inline(always)]
    pub const fn useburstset(&self) -> &Useburstset {
        &self.useburstset
    }
    #[doc = "0x1c - Channel useburst clear"]
    #[inline(always)]
    pub const fn useburstclr(&self) -> &Useburstclr {
        &self.useburstclr
    }
    #[doc = "0x20 - Channel request mask set"]
    #[inline(always)]
    pub const fn reqmaskset(&self) -> &Reqmaskset {
        &self.reqmaskset
    }
    #[doc = "0x24 - Channel request mask clear"]
    #[inline(always)]
    pub const fn reqmaskclr(&self) -> &Reqmaskclr {
        &self.reqmaskclr
    }
    #[doc = "0x28 - Channel enable set"]
    #[inline(always)]
    pub const fn enset(&self) -> &Enset {
        &self.enset
    }
    #[doc = "0x2c - Channel enable clear"]
    #[inline(always)]
    pub const fn enclr(&self) -> &Enclr {
        &self.enclr
    }
    #[doc = "0x30 - Channel primary-alternate set"]
    #[inline(always)]
    pub const fn prialtset(&self) -> &Prialtset {
        &self.prialtset
    }
    #[doc = "0x34 - Channel primary-alternate clear"]
    #[inline(always)]
    pub const fn prialtclr(&self) -> &Prialtclr {
        &self.prialtclr
    }
    #[doc = "0x38 - Channel priority set"]
    #[inline(always)]
    pub const fn priorityset(&self) -> &Priorityset {
        &self.priorityset
    }
    #[doc = "0x3c - Channel priority clear"]
    #[inline(always)]
    pub const fn priorityclr(&self) -> &Priorityclr {
        &self.priorityclr
    }
    #[doc = "0x4c - Bus error register"]
    #[inline(always)]
    pub const fn errclr(&self) -> &Errclr {
        &self.errclr
    }
}
#[doc = "STATUS (r) register accessor: Status DMA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status DMA register"]
pub mod status;
#[doc = "CFG (w) register accessor: DMA configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "DMA configuration register"]
pub mod cfg;
#[doc = "BASEPTR (rw) register accessor: Channel control data base pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baseptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baseptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baseptr`]
module"]
#[doc(alias = "BASEPTR")]
pub type Baseptr = crate::Reg<baseptr::BaseptrSpec>;
#[doc = "Channel control data base pointer"]
pub mod baseptr;
#[doc = "ALTBASEPTR (r) register accessor: Channel alternate control data base pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altbaseptr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altbaseptr`]
module"]
#[doc(alias = "ALTBASEPTR")]
pub type Altbaseptr = crate::Reg<altbaseptr::AltbaseptrSpec>;
#[doc = "Channel alternate control data base pointer"]
pub mod altbaseptr;
#[doc = "WAITONREQ (r) register accessor: Channel wait on request status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waitonreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waitonreq`]
module"]
#[doc(alias = "WAITONREQ")]
pub type Waitonreq = crate::Reg<waitonreq::WaitonreqSpec>;
#[doc = "Channel wait on request status"]
pub mod waitonreq;
#[doc = "SWREQ (w) register accessor: Channel software request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`]
module"]
#[doc(alias = "SWREQ")]
pub type Swreq = crate::Reg<swreq::SwreqSpec>;
#[doc = "Channel software request"]
pub mod swreq;
#[doc = "USEBURSTSET (rw) register accessor: Channel useburst set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`useburstset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useburstset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useburstset`]
module"]
#[doc(alias = "USEBURSTSET")]
pub type Useburstset = crate::Reg<useburstset::UseburstsetSpec>;
#[doc = "Channel useburst set"]
pub mod useburstset;
#[doc = "USEBURSTCLR (w) register accessor: Channel useburst clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useburstclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useburstclr`]
module"]
#[doc(alias = "USEBURSTCLR")]
pub type Useburstclr = crate::Reg<useburstclr::UseburstclrSpec>;
#[doc = "Channel useburst clear"]
pub mod useburstclr;
#[doc = "REQMASKSET (rw) register accessor: Channel request mask set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqmaskset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqmaskset`]
module"]
#[doc(alias = "REQMASKSET")]
pub type Reqmaskset = crate::Reg<reqmaskset::ReqmasksetSpec>;
#[doc = "Channel request mask set"]
pub mod reqmaskset;
#[doc = "REQMASKCLR (w) register accessor: Channel request mask clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqmaskclr`]
module"]
#[doc(alias = "REQMASKCLR")]
pub type Reqmaskclr = crate::Reg<reqmaskclr::ReqmaskclrSpec>;
#[doc = "Channel request mask clear"]
pub mod reqmaskclr;
#[doc = "ENSET (rw) register accessor: Channel enable set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enset`]
module"]
#[doc(alias = "ENSET")]
pub type Enset = crate::Reg<enset::EnsetSpec>;
#[doc = "Channel enable set"]
pub mod enset;
#[doc = "ENCLR (w) register accessor: Channel enable clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enclr`]
module"]
#[doc(alias = "ENCLR")]
pub type Enclr = crate::Reg<enclr::EnclrSpec>;
#[doc = "Channel enable clear"]
pub mod enclr;
#[doc = "PRIALTSET (rw) register accessor: Channel primary-alternate set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prialtset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prialtset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prialtset`]
module"]
#[doc(alias = "PRIALTSET")]
pub type Prialtset = crate::Reg<prialtset::PrialtsetSpec>;
#[doc = "Channel primary-alternate set"]
pub mod prialtset;
#[doc = "PRIALTCLR (w) register accessor: Channel primary-alternate clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prialtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prialtclr`]
module"]
#[doc(alias = "PRIALTCLR")]
pub type Prialtclr = crate::Reg<prialtclr::PrialtclrSpec>;
#[doc = "Channel primary-alternate clear"]
pub mod prialtclr;
#[doc = "PRIORITYSET (rw) register accessor: Channel priority set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priorityset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priorityset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priorityset`]
module"]
#[doc(alias = "PRIORITYSET")]
pub type Priorityset = crate::Reg<priorityset::PrioritysetSpec>;
#[doc = "Channel priority set"]
pub mod priorityset;
#[doc = "PRIORITYCLR (w) register accessor: Channel priority clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priorityclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priorityclr`]
module"]
#[doc(alias = "PRIORITYCLR")]
pub type Priorityclr = crate::Reg<priorityclr::PriorityclrSpec>;
#[doc = "Channel priority clear"]
pub mod priorityclr;
#[doc = "ERRCLR (rw) register accessor: Bus error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errclr`]
module"]
#[doc(alias = "ERRCLR")]
pub type Errclr = crate::Reg<errclr::ErrclrSpec>;
#[doc = "Bus error register"]
pub mod errclr;
