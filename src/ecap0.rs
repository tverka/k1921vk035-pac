#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tsctr: Tsctr,
    ctrphs: Ctrphs,
    _reserved_2_prd: [u8; 0x04],
    _reserved_3_cmp: [u8; 0x04],
    _reserved_4_cap2: [u8; 0x04],
    _reserved_5_cap3: [u8; 0x04],
    _reserved6: [u8; 0x10],
    ecctl0: Ecctl0,
    ecctl1: Ecctl1,
    eceint: Eceint,
    ecflg: Ecflg,
    ecclr: Ecclr,
    ecfrc: Ecfrc,
    peint: Peint,
}
impl RegisterBlock {
    #[doc = "0x00 - Counter register"]
    #[inline(always)]
    pub const fn tsctr(&self) -> &Tsctr {
        &self.tsctr
    }
    #[doc = "0x04 - Counter Phase Sync register"]
    #[inline(always)]
    pub const fn ctrphs(&self) -> &Ctrphs {
        &self.ctrphs
    }
    #[doc = "0x08 - Period register"]
    #[inline(always)]
    pub const fn prd(&self) -> &Prd {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Capture register 0"]
    #[inline(always)]
    pub const fn cap0(&self) -> &Cap0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Compare register"]
    #[inline(always)]
    pub const fn cmp(&self) -> &Cmp {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Capture register 1"]
    #[inline(always)]
    pub const fn cap1(&self) -> &Cap1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Period shadow register"]
    #[inline(always)]
    pub const fn prdshdw(&self) -> &Prdshdw {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Capture register 2"]
    #[inline(always)]
    pub const fn cap2(&self) -> &Cap2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Compare shadow register"]
    #[inline(always)]
    pub const fn cmpshdw(&self) -> &Cmpshdw {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Capture register 3"]
    #[inline(always)]
    pub const fn cap3(&self) -> &Cap3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x28 - Capture control register 0"]
    #[inline(always)]
    pub const fn ecctl0(&self) -> &Ecctl0 {
        &self.ecctl0
    }
    #[doc = "0x2c - Capture control register 1"]
    #[inline(always)]
    pub const fn ecctl1(&self) -> &Ecctl1 {
        &self.ecctl1
    }
    #[doc = "0x30 - Interrupt mask register"]
    #[inline(always)]
    pub const fn eceint(&self) -> &Eceint {
        &self.eceint
    }
    #[doc = "0x34 - Interrupt status register"]
    #[inline(always)]
    pub const fn ecflg(&self) -> &Ecflg {
        &self.ecflg
    }
    #[doc = "0x38 - Clear interrupt register"]
    #[inline(always)]
    pub const fn ecclr(&self) -> &Ecclr {
        &self.ecclr
    }
    #[doc = "0x3c - Force interrupt register"]
    #[inline(always)]
    pub const fn ecfrc(&self) -> &Ecfrc {
        &self.ecfrc
    }
    #[doc = "0x40 - Active interrupt status register"]
    #[inline(always)]
    pub const fn peint(&self) -> &Peint {
        &self.peint
    }
}
#[doc = "TSCTR (rw) register accessor: Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsctr`]
module"]
#[doc(alias = "TSCTR")]
pub type Tsctr = crate::Reg<tsctr::TsctrSpec>;
#[doc = "Counter register"]
pub mod tsctr;
#[doc = "CTRPHS (rw) register accessor: Counter Phase Sync register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrphs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrphs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrphs`]
module"]
#[doc(alias = "CTRPHS")]
pub type Ctrphs = crate::Reg<ctrphs::CtrphsSpec>;
#[doc = "Counter Phase Sync register"]
pub mod ctrphs;
#[doc = "CAP0 (rw) register accessor: Capture register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap0`]
module"]
#[doc(alias = "CAP0")]
pub type Cap0 = crate::Reg<cap0::Cap0Spec>;
#[doc = "Capture register 0"]
pub mod cap0;
#[doc = "PRD (rw) register accessor: Period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prd`]
module"]
#[doc(alias = "PRD")]
pub type Prd = crate::Reg<prd::PrdSpec>;
#[doc = "Period register"]
pub mod prd;
#[doc = "CAP1 (rw) register accessor: Capture register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap1`]
module"]
#[doc(alias = "CAP1")]
pub type Cap1 = crate::Reg<cap1::Cap1Spec>;
#[doc = "Capture register 1"]
pub mod cap1;
#[doc = "CMP (rw) register accessor: Compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
#[doc(alias = "CMP")]
pub type Cmp = crate::Reg<cmp::CmpSpec>;
#[doc = "Compare register"]
pub mod cmp;
#[doc = "CAP2 (rw) register accessor: Capture register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap2`]
module"]
#[doc(alias = "CAP2")]
pub type Cap2 = crate::Reg<cap2::Cap2Spec>;
#[doc = "Capture register 2"]
pub mod cap2;
#[doc = "PRDSHDW (rw) register accessor: Period shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prdshdw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prdshdw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prdshdw`]
module"]
#[doc(alias = "PRDSHDW")]
pub type Prdshdw = crate::Reg<prdshdw::PrdshdwSpec>;
#[doc = "Period shadow register"]
pub mod prdshdw;
#[doc = "CAP3 (rw) register accessor: Capture register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap3`]
module"]
#[doc(alias = "CAP3")]
pub type Cap3 = crate::Reg<cap3::Cap3Spec>;
#[doc = "Capture register 3"]
pub mod cap3;
#[doc = "CMPSHDW (rw) register accessor: Compare shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpshdw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpshdw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpshdw`]
module"]
#[doc(alias = "CMPSHDW")]
pub type Cmpshdw = crate::Reg<cmpshdw::CmpshdwSpec>;
#[doc = "Compare shadow register"]
pub mod cmpshdw;
#[doc = "ECCTL0 (rw) register accessor: Capture control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecctl0`]
module"]
#[doc(alias = "ECCTL0")]
pub type Ecctl0 = crate::Reg<ecctl0::Ecctl0Spec>;
#[doc = "Capture control register 0"]
pub mod ecctl0;
#[doc = "ECCTL1 (rw) register accessor: Capture control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecctl1`]
module"]
#[doc(alias = "ECCTL1")]
pub type Ecctl1 = crate::Reg<ecctl1::Ecctl1Spec>;
#[doc = "Capture control register 1"]
pub mod ecctl1;
#[doc = "ECEINT (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eceint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eceint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eceint`]
module"]
#[doc(alias = "ECEINT")]
pub type Eceint = crate::Reg<eceint::EceintSpec>;
#[doc = "Interrupt mask register"]
pub mod eceint;
#[doc = "ECFLG (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecflg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecflg`]
module"]
#[doc(alias = "ECFLG")]
pub type Ecflg = crate::Reg<ecflg::EcflgSpec>;
#[doc = "Interrupt status register"]
pub mod ecflg;
#[doc = "ECCLR (rw) register accessor: Clear interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecclr`]
module"]
#[doc(alias = "ECCLR")]
pub type Ecclr = crate::Reg<ecclr::EcclrSpec>;
#[doc = "Clear interrupt register"]
pub mod ecclr;
#[doc = "ECFRC (rw) register accessor: Force interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecfrc`]
module"]
#[doc(alias = "ECFRC")]
pub type Ecfrc = crate::Reg<ecfrc::EcfrcSpec>;
#[doc = "Force interrupt register"]
pub mod ecfrc;
#[doc = "PEINT (rw) register accessor: Active interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peint`]
module"]
#[doc(alias = "PEINT")]
pub type Peint = crate::Reg<peint::PeintSpec>;
#[doc = "Active interrupt status register"]
pub mod peint;
