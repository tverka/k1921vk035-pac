#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    qposcnt: Qposcnt,
    qposinit: Qposinit,
    qposmax: Qposmax,
    qposcmp: Qposcmp,
    qposilat: Qposilat,
    qposslat: Qposslat,
    qposlat: Qposlat,
    qutmr: Qutmr,
    quprd: Quprd,
    qwdtmr: Qwdtmr,
    qwdprd: Qwdprd,
    qdecctl: Qdecctl,
    qepctl: Qepctl,
    qcapctl: Qcapctl,
    qposctl: Qposctl,
    qeint: Qeint,
    qflg: Qflg,
    qclr: Qclr,
    qfrc: Qfrc,
    qepsts: Qepsts,
    qctmr: Qctmr,
    qcprd: Qcprd,
    qctmrlat: Qctmrlat,
    qcprdlat: Qcprdlat,
    dmareq: Dmareq,
    _reserved25: [u8; 0x0c],
    intclr: Intclr,
}
impl RegisterBlock {
    #[doc = "0x00 - Position Counter register"]
    #[inline(always)]
    pub const fn qposcnt(&self) -> &Qposcnt {
        &self.qposcnt
    }
    #[doc = "0x04 - Position Counter Initialization register"]
    #[inline(always)]
    pub const fn qposinit(&self) -> &Qposinit {
        &self.qposinit
    }
    #[doc = "0x08 - Maximum Position Count register"]
    #[inline(always)]
    pub const fn qposmax(&self) -> &Qposmax {
        &self.qposmax
    }
    #[doc = "0x0c - Position-compare register"]
    #[inline(always)]
    pub const fn qposcmp(&self) -> &Qposcmp {
        &self.qposcmp
    }
    #[doc = "0x10 - Index Position Latch register"]
    #[inline(always)]
    pub const fn qposilat(&self) -> &Qposilat {
        &self.qposilat
    }
    #[doc = "0x14 - Strobe Position Latch register"]
    #[inline(always)]
    pub const fn qposslat(&self) -> &Qposslat {
        &self.qposslat
    }
    #[doc = "0x18 - Position Counter Latch register"]
    #[inline(always)]
    pub const fn qposlat(&self) -> &Qposlat {
        &self.qposlat
    }
    #[doc = "0x1c - Unit Timer register"]
    #[inline(always)]
    pub const fn qutmr(&self) -> &Qutmr {
        &self.qutmr
    }
    #[doc = "0x20 - Unit Period register"]
    #[inline(always)]
    pub const fn quprd(&self) -> &Quprd {
        &self.quprd
    }
    #[doc = "0x24 - Watchdog Timer register"]
    #[inline(always)]
    pub const fn qwdtmr(&self) -> &Qwdtmr {
        &self.qwdtmr
    }
    #[doc = "0x28 - Watchdog Period register"]
    #[inline(always)]
    pub const fn qwdprd(&self) -> &Qwdprd {
        &self.qwdprd
    }
    #[doc = "0x2c - Decoder Control register"]
    #[inline(always)]
    pub const fn qdecctl(&self) -> &Qdecctl {
        &self.qdecctl
    }
    #[doc = "0x30 - Control register"]
    #[inline(always)]
    pub const fn qepctl(&self) -> &Qepctl {
        &self.qepctl
    }
    #[doc = "0x34 - Capture Control register"]
    #[inline(always)]
    pub const fn qcapctl(&self) -> &Qcapctl {
        &self.qcapctl
    }
    #[doc = "0x38 - Position-compare Control register"]
    #[inline(always)]
    pub const fn qposctl(&self) -> &Qposctl {
        &self.qposctl
    }
    #[doc = "0x3c - Interrupt Enable register"]
    #[inline(always)]
    pub const fn qeint(&self) -> &Qeint {
        &self.qeint
    }
    #[doc = "0x40 - Interrupt Flag register"]
    #[inline(always)]
    pub const fn qflg(&self) -> &Qflg {
        &self.qflg
    }
    #[doc = "0x44 - Interrupt Clear register"]
    #[inline(always)]
    pub const fn qclr(&self) -> &Qclr {
        &self.qclr
    }
    #[doc = "0x48 - Interrupt Force register"]
    #[inline(always)]
    pub const fn qfrc(&self) -> &Qfrc {
        &self.qfrc
    }
    #[doc = "0x4c - Status register"]
    #[inline(always)]
    pub const fn qepsts(&self) -> &Qepsts {
        &self.qepsts
    }
    #[doc = "0x50 - Capture Timer register"]
    #[inline(always)]
    pub const fn qctmr(&self) -> &Qctmr {
        &self.qctmr
    }
    #[doc = "0x54 - Capture Period register"]
    #[inline(always)]
    pub const fn qcprd(&self) -> &Qcprd {
        &self.qcprd
    }
    #[doc = "0x58 - Capture Timer Latch register"]
    #[inline(always)]
    pub const fn qctmrlat(&self) -> &Qctmrlat {
        &self.qctmrlat
    }
    #[doc = "0x5c - Capture Period Latch register"]
    #[inline(always)]
    pub const fn qcprdlat(&self) -> &Qcprdlat {
        &self.qcprdlat
    }
    #[doc = "0x60 - DMA request register"]
    #[inline(always)]
    pub const fn dmareq(&self) -> &Dmareq {
        &self.dmareq
    }
    #[doc = "0x70 - Clear active interrupt register"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
}
#[doc = "QPOSCNT (rw) register accessor: Position Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposcnt`]
module"]
#[doc(alias = "QPOSCNT")]
pub type Qposcnt = crate::Reg<qposcnt::QposcntSpec>;
#[doc = "Position Counter register"]
pub mod qposcnt;
#[doc = "QPOSINIT (rw) register accessor: Position Counter Initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposinit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposinit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposinit`]
module"]
#[doc(alias = "QPOSINIT")]
pub type Qposinit = crate::Reg<qposinit::QposinitSpec>;
#[doc = "Position Counter Initialization register"]
pub mod qposinit;
#[doc = "QPOSMAX (rw) register accessor: Maximum Position Count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposmax::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposmax::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposmax`]
module"]
#[doc(alias = "QPOSMAX")]
pub type Qposmax = crate::Reg<qposmax::QposmaxSpec>;
#[doc = "Maximum Position Count register"]
pub mod qposmax;
#[doc = "QPOSCMP (rw) register accessor: Position-compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposcmp`]
module"]
#[doc(alias = "QPOSCMP")]
pub type Qposcmp = crate::Reg<qposcmp::QposcmpSpec>;
#[doc = "Position-compare register"]
pub mod qposcmp;
#[doc = "QPOSILAT (r) register accessor: Index Position Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposilat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposilat`]
module"]
#[doc(alias = "QPOSILAT")]
pub type Qposilat = crate::Reg<qposilat::QposilatSpec>;
#[doc = "Index Position Latch register"]
pub mod qposilat;
#[doc = "QPOSSLAT (r) register accessor: Strobe Position Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposslat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposslat`]
module"]
#[doc(alias = "QPOSSLAT")]
pub type Qposslat = crate::Reg<qposslat::QposslatSpec>;
#[doc = "Strobe Position Latch register"]
pub mod qposslat;
#[doc = "QPOSLAT (r) register accessor: Position Counter Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposlat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposlat`]
module"]
#[doc(alias = "QPOSLAT")]
pub type Qposlat = crate::Reg<qposlat::QposlatSpec>;
#[doc = "Position Counter Latch register"]
pub mod qposlat;
#[doc = "QUTMR (rw) register accessor: Unit Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qutmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qutmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qutmr`]
module"]
#[doc(alias = "QUTMR")]
pub type Qutmr = crate::Reg<qutmr::QutmrSpec>;
#[doc = "Unit Timer register"]
pub mod qutmr;
#[doc = "QUPRD (rw) register accessor: Unit Period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quprd`]
module"]
#[doc(alias = "QUPRD")]
pub type Quprd = crate::Reg<quprd::QuprdSpec>;
#[doc = "Unit Period register"]
pub mod quprd;
#[doc = "QWDTMR (rw) register accessor: Watchdog Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qwdtmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qwdtmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qwdtmr`]
module"]
#[doc(alias = "QWDTMR")]
pub type Qwdtmr = crate::Reg<qwdtmr::QwdtmrSpec>;
#[doc = "Watchdog Timer register"]
pub mod qwdtmr;
#[doc = "QWDPRD (rw) register accessor: Watchdog Period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qwdprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qwdprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qwdprd`]
module"]
#[doc(alias = "QWDPRD")]
pub type Qwdprd = crate::Reg<qwdprd::QwdprdSpec>;
#[doc = "Watchdog Period register"]
pub mod qwdprd;
#[doc = "QDECCTL (rw) register accessor: Decoder Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdecctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdecctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdecctl`]
module"]
#[doc(alias = "QDECCTL")]
pub type Qdecctl = crate::Reg<qdecctl::QdecctlSpec>;
#[doc = "Decoder Control register"]
pub mod qdecctl;
#[doc = "QEPCTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qepctl`]
module"]
#[doc(alias = "QEPCTL")]
pub type Qepctl = crate::Reg<qepctl::QepctlSpec>;
#[doc = "Control register"]
pub mod qepctl;
#[doc = "QCAPCTL (rw) register accessor: Capture Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qcapctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qcapctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcapctl`]
module"]
#[doc(alias = "QCAPCTL")]
pub type Qcapctl = crate::Reg<qcapctl::QcapctlSpec>;
#[doc = "Capture Control register"]
pub mod qcapctl;
#[doc = "QPOSCTL (rw) register accessor: Position-compare Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qposctl`]
module"]
#[doc(alias = "QPOSCTL")]
pub type Qposctl = crate::Reg<qposctl::QposctlSpec>;
#[doc = "Position-compare Control register"]
pub mod qposctl;
#[doc = "QEINT (rw) register accessor: Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qeint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qeint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeint`]
module"]
#[doc(alias = "QEINT")]
pub type Qeint = crate::Reg<qeint::QeintSpec>;
#[doc = "Interrupt Enable register"]
pub mod qeint;
#[doc = "QFLG (r) register accessor: Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qflg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qflg`]
module"]
#[doc(alias = "QFLG")]
pub type Qflg = crate::Reg<qflg::QflgSpec>;
#[doc = "Interrupt Flag register"]
pub mod qflg;
#[doc = "QCLR (rw) register accessor: Interrupt Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qclr`]
module"]
#[doc(alias = "QCLR")]
pub type Qclr = crate::Reg<qclr::QclrSpec>;
#[doc = "Interrupt Clear register"]
pub mod qclr;
#[doc = "QFRC (rw) register accessor: Interrupt Force register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qfrc`]
module"]
#[doc(alias = "QFRC")]
pub type Qfrc = crate::Reg<qfrc::QfrcSpec>;
#[doc = "Interrupt Force register"]
pub mod qfrc;
#[doc = "QEPSTS (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qepsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qepsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qepsts`]
module"]
#[doc(alias = "QEPSTS")]
pub type Qepsts = crate::Reg<qepsts::QepstsSpec>;
#[doc = "Status register"]
pub mod qepsts;
#[doc = "QCTMR (rw) register accessor: Capture Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qctmr`]
module"]
#[doc(alias = "QCTMR")]
pub type Qctmr = crate::Reg<qctmr::QctmrSpec>;
#[doc = "Capture Timer register"]
pub mod qctmr;
#[doc = "QCPRD (rw) register accessor: Capture Period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qcprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qcprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcprd`]
module"]
#[doc(alias = "QCPRD")]
pub type Qcprd = crate::Reg<qcprd::QcprdSpec>;
#[doc = "Capture Period register"]
pub mod qcprd;
#[doc = "QCTMRLAT (r) register accessor: Capture Timer Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctmrlat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qctmrlat`]
module"]
#[doc(alias = "QCTMRLAT")]
pub type Qctmrlat = crate::Reg<qctmrlat::QctmrlatSpec>;
#[doc = "Capture Timer Latch register"]
pub mod qctmrlat;
#[doc = "QCPRDLAT (r) register accessor: Capture Period Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qcprdlat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcprdlat`]
module"]
#[doc(alias = "QCPRDLAT")]
pub type Qcprdlat = crate::Reg<qcprdlat::QcprdlatSpec>;
#[doc = "Capture Period Latch register"]
pub mod qcprdlat;
#[doc = "DMAREQ (rw) register accessor: DMA request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq`]
module"]
#[doc(alias = "DMAREQ")]
pub type Dmareq = crate::Reg<dmareq::DmareqSpec>;
#[doc = "DMA request register"]
pub mod dmareq;
#[doc = "INTCLR (rw) register accessor: Clear active interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Clear active interrupt register"]
pub mod intclr;
