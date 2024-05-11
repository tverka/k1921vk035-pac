#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tbctl: Tbctl,
    tbsts: Tbsts,
    tbphs: Tbphs,
    tbctr: Tbctr,
    tbprd: Tbprd,
    cmpctl: Cmpctl,
    cmpa: Cmpa,
    cmpb: Cmpb,
    aqctla: Aqctla,
    aqctlb: Aqctlb,
    aqsfrc: Aqsfrc,
    aqcsfrc: Aqcsfrc,
    dbctl: Dbctl,
    dbred: Dbred,
    dbfed: Dbfed,
    tzsel: Tzsel,
    tzctl: Tzctl,
    tzeint: Tzeint,
    tzflg: Tzflg,
    tzclr: Tzclr,
    tzfrc: Tzfrc,
    etsel: Etsel,
    etps: Etps,
    etflg: Etflg,
    etclr: Etclr,
    etfrc: Etfrc,
    pcctl: Pcctl,
    _reserved27: [u8; 0x04],
    fwdth: Fwdth,
    _reserved28: [u8; 0x14],
    hdsel: Hdsel,
    hdctl: Hdctl,
    hdeint: Hdeint,
    hdflg: Hdflg,
    hdclr: Hdclr,
    hdfrc: Hdfrc,
    hdintclr: Hdintclr,
    tzintclr: Tzintclr,
    intclr: Intclr,
}
impl RegisterBlock {
    #[doc = "0x00 - Time-Base Control Register"]
    #[inline(always)]
    pub const fn tbctl(&self) -> &Tbctl {
        &self.tbctl
    }
    #[doc = "0x04 - Time-Base Status Register"]
    #[inline(always)]
    pub const fn tbsts(&self) -> &Tbsts {
        &self.tbsts
    }
    #[doc = "0x08 - Time-Base Phase Register"]
    #[inline(always)]
    pub const fn tbphs(&self) -> &Tbphs {
        &self.tbphs
    }
    #[doc = "0x0c - Time-Base Counter Register"]
    #[inline(always)]
    pub const fn tbctr(&self) -> &Tbctr {
        &self.tbctr
    }
    #[doc = "0x10 - Time-Base Period Register"]
    #[inline(always)]
    pub const fn tbprd(&self) -> &Tbprd {
        &self.tbprd
    }
    #[doc = "0x14 - Counter-Compare Control Register"]
    #[inline(always)]
    pub const fn cmpctl(&self) -> &Cmpctl {
        &self.cmpctl
    }
    #[doc = "0x18 - Counter-Compare A Register"]
    #[inline(always)]
    pub const fn cmpa(&self) -> &Cmpa {
        &self.cmpa
    }
    #[doc = "0x1c - Counter-Compare B Register"]
    #[inline(always)]
    pub const fn cmpb(&self) -> &Cmpb {
        &self.cmpb
    }
    #[doc = "0x20 - Action-Qualifier Output A Control Register"]
    #[inline(always)]
    pub const fn aqctla(&self) -> &Aqctla {
        &self.aqctla
    }
    #[doc = "0x24 - Action-Qualifier Output B Control Register"]
    #[inline(always)]
    pub const fn aqctlb(&self) -> &Aqctlb {
        &self.aqctlb
    }
    #[doc = "0x28 - Action-Qualifier Software Force Register"]
    #[inline(always)]
    pub const fn aqsfrc(&self) -> &Aqsfrc {
        &self.aqsfrc
    }
    #[doc = "0x2c - Action-Qualifier Continuous Software Force Register"]
    #[inline(always)]
    pub const fn aqcsfrc(&self) -> &Aqcsfrc {
        &self.aqcsfrc
    }
    #[doc = "0x30 - Dead-Band Generator Control Register"]
    #[inline(always)]
    pub const fn dbctl(&self) -> &Dbctl {
        &self.dbctl
    }
    #[doc = "0x34 - Dead-Band Generator Rising Edge Delay Register"]
    #[inline(always)]
    pub const fn dbred(&self) -> &Dbred {
        &self.dbred
    }
    #[doc = "0x38 - Dead-Band Generator Falling Edge Delay Register"]
    #[inline(always)]
    pub const fn dbfed(&self) -> &Dbfed {
        &self.dbfed
    }
    #[doc = "0x3c - Trip-Zone Select Register"]
    #[inline(always)]
    pub const fn tzsel(&self) -> &Tzsel {
        &self.tzsel
    }
    #[doc = "0x40 - Trip-Zone Control Register"]
    #[inline(always)]
    pub const fn tzctl(&self) -> &Tzctl {
        &self.tzctl
    }
    #[doc = "0x44 - Trip-Zone Enable Interrupt Register"]
    #[inline(always)]
    pub const fn tzeint(&self) -> &Tzeint {
        &self.tzeint
    }
    #[doc = "0x48 - Trip-Zone Flag Register"]
    #[inline(always)]
    pub const fn tzflg(&self) -> &Tzflg {
        &self.tzflg
    }
    #[doc = "0x4c - Trip-Zone Clear Register"]
    #[inline(always)]
    pub const fn tzclr(&self) -> &Tzclr {
        &self.tzclr
    }
    #[doc = "0x50 - Trip-Zone Force Register"]
    #[inline(always)]
    pub const fn tzfrc(&self) -> &Tzfrc {
        &self.tzfrc
    }
    #[doc = "0x54 - Event-Trigger Selection Register"]
    #[inline(always)]
    pub const fn etsel(&self) -> &Etsel {
        &self.etsel
    }
    #[doc = "0x58 - Event-Trigger Prescale Register"]
    #[inline(always)]
    pub const fn etps(&self) -> &Etps {
        &self.etps
    }
    #[doc = "0x5c - Event-Trigger Flag Register"]
    #[inline(always)]
    pub const fn etflg(&self) -> &Etflg {
        &self.etflg
    }
    #[doc = "0x60 - Event-Trigger Clear Register"]
    #[inline(always)]
    pub const fn etclr(&self) -> &Etclr {
        &self.etclr
    }
    #[doc = "0x64 - Event-Trigger Force Register"]
    #[inline(always)]
    pub const fn etfrc(&self) -> &Etfrc {
        &self.etfrc
    }
    #[doc = "0x68 - PWM-Chopper Control Register"]
    #[inline(always)]
    pub const fn pcctl(&self) -> &Pcctl {
        &self.pcctl
    }
    #[doc = "0x70 - Filter Width select Register"]
    #[inline(always)]
    pub const fn fwdth(&self) -> &Fwdth {
        &self.fwdth
    }
    #[doc = "0x88 - Hold Detector event Select Register"]
    #[inline(always)]
    pub const fn hdsel(&self) -> &Hdsel {
        &self.hdsel
    }
    #[doc = "0x8c - Hold Detector Control register"]
    #[inline(always)]
    pub const fn hdctl(&self) -> &Hdctl {
        &self.hdctl
    }
    #[doc = "0x90 - Hold Detector Enable Interrupt Register"]
    #[inline(always)]
    pub const fn hdeint(&self) -> &Hdeint {
        &self.hdeint
    }
    #[doc = "0x94 - Hold Detector Flag Register"]
    #[inline(always)]
    pub const fn hdflg(&self) -> &Hdflg {
        &self.hdflg
    }
    #[doc = "0x98 - Register clear HD flag"]
    #[inline(always)]
    pub const fn hdclr(&self) -> &Hdclr {
        &self.hdclr
    }
    #[doc = "0x9c - Hold Detector Force Register"]
    #[inline(always)]
    pub const fn hdfrc(&self) -> &Hdfrc {
        &self.hdfrc
    }
    #[doc = "0xa0 - Hold Detector Interrupt pending Clear Register"]
    #[inline(always)]
    pub const fn hdintclr(&self) -> &Hdintclr {
        &self.hdintclr
    }
    #[doc = "0xa4 - Trip-Zone Interrupt pending Clear Register"]
    #[inline(always)]
    pub const fn tzintclr(&self) -> &Tzintclr {
        &self.tzintclr
    }
    #[doc = "0xa8 - PWM Interrupt pending Clear Register"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
}
#[doc = "TBCTL (rw) register accessor: Time-Base Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctl`]
module"]
#[doc(alias = "TBCTL")]
pub type Tbctl = crate::Reg<tbctl::TbctlSpec>;
#[doc = "Time-Base Control Register"]
pub mod tbctl;
#[doc = "TBSTS (rw) register accessor: Time-Base Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbsts`]
module"]
#[doc(alias = "TBSTS")]
pub type Tbsts = crate::Reg<tbsts::TbstsSpec>;
#[doc = "Time-Base Status Register"]
pub mod tbsts;
#[doc = "TBPHS (rw) register accessor: Time-Base Phase Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbphs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbphs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbphs`]
module"]
#[doc(alias = "TBPHS")]
pub type Tbphs = crate::Reg<tbphs::TbphsSpec>;
#[doc = "Time-Base Phase Register"]
pub mod tbphs;
#[doc = "TBCTR (rw) register accessor: Time-Base Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctr`]
module"]
#[doc(alias = "TBCTR")]
pub type Tbctr = crate::Reg<tbctr::TbctrSpec>;
#[doc = "Time-Base Counter Register"]
pub mod tbctr;
#[doc = "TBPRD (rw) register accessor: Time-Base Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbprd`]
module"]
#[doc(alias = "TBPRD")]
pub type Tbprd = crate::Reg<tbprd::TbprdSpec>;
#[doc = "Time-Base Period Register"]
pub mod tbprd;
#[doc = "CMPCTL (rw) register accessor: Counter-Compare Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpctl`]
module"]
#[doc(alias = "CMPCTL")]
pub type Cmpctl = crate::Reg<cmpctl::CmpctlSpec>;
#[doc = "Counter-Compare Control Register"]
pub mod cmpctl;
#[doc = "CMPA (rw) register accessor: Counter-Compare A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpa`]
module"]
#[doc(alias = "CMPA")]
pub type Cmpa = crate::Reg<cmpa::CmpaSpec>;
#[doc = "Counter-Compare A Register"]
pub mod cmpa;
#[doc = "CMPB (rw) register accessor: Counter-Compare B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpb`]
module"]
#[doc(alias = "CMPB")]
pub type Cmpb = crate::Reg<cmpb::CmpbSpec>;
#[doc = "Counter-Compare B Register"]
pub mod cmpb;
#[doc = "AQCTLA (rw) register accessor: Action-Qualifier Output A Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aqctla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aqctla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aqctla`]
module"]
#[doc(alias = "AQCTLA")]
pub type Aqctla = crate::Reg<aqctla::AqctlaSpec>;
#[doc = "Action-Qualifier Output A Control Register"]
pub mod aqctla;
#[doc = "AQCTLB (rw) register accessor: Action-Qualifier Output B Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aqctlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aqctlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aqctlb`]
module"]
#[doc(alias = "AQCTLB")]
pub type Aqctlb = crate::Reg<aqctlb::AqctlbSpec>;
#[doc = "Action-Qualifier Output B Control Register"]
pub mod aqctlb;
#[doc = "AQSFRC (rw) register accessor: Action-Qualifier Software Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aqsfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aqsfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aqsfrc`]
module"]
#[doc(alias = "AQSFRC")]
pub type Aqsfrc = crate::Reg<aqsfrc::AqsfrcSpec>;
#[doc = "Action-Qualifier Software Force Register"]
pub mod aqsfrc;
#[doc = "AQCSFRC (rw) register accessor: Action-Qualifier Continuous Software Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aqcsfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aqcsfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aqcsfrc`]
module"]
#[doc(alias = "AQCSFRC")]
pub type Aqcsfrc = crate::Reg<aqcsfrc::AqcsfrcSpec>;
#[doc = "Action-Qualifier Continuous Software Force Register"]
pub mod aqcsfrc;
#[doc = "DBCTL (rw) register accessor: Dead-Band Generator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbctl`]
module"]
#[doc(alias = "DBCTL")]
pub type Dbctl = crate::Reg<dbctl::DbctlSpec>;
#[doc = "Dead-Band Generator Control Register"]
pub mod dbctl;
#[doc = "DBRED (rw) register accessor: Dead-Band Generator Rising Edge Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbred::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbred::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbred`]
module"]
#[doc(alias = "DBRED")]
pub type Dbred = crate::Reg<dbred::DbredSpec>;
#[doc = "Dead-Band Generator Rising Edge Delay Register"]
pub mod dbred;
#[doc = "DBFED (rw) register accessor: Dead-Band Generator Falling Edge Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbfed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbfed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbfed`]
module"]
#[doc(alias = "DBFED")]
pub type Dbfed = crate::Reg<dbfed::DbfedSpec>;
#[doc = "Dead-Band Generator Falling Edge Delay Register"]
pub mod dbfed;
#[doc = "TZSEL (rw) register accessor: Trip-Zone Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsel`]
module"]
#[doc(alias = "TZSEL")]
pub type Tzsel = crate::Reg<tzsel::TzselSpec>;
#[doc = "Trip-Zone Select Register"]
pub mod tzsel;
#[doc = "TZCTL (rw) register accessor: Trip-Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzctl`]
module"]
#[doc(alias = "TZCTL")]
pub type Tzctl = crate::Reg<tzctl::TzctlSpec>;
#[doc = "Trip-Zone Control Register"]
pub mod tzctl;
#[doc = "TZEINT (rw) register accessor: Trip-Zone Enable Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzeint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzeint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzeint`]
module"]
#[doc(alias = "TZEINT")]
pub type Tzeint = crate::Reg<tzeint::TzeintSpec>;
#[doc = "Trip-Zone Enable Interrupt Register"]
pub mod tzeint;
#[doc = "TZFLG (r) register accessor: Trip-Zone Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzflg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzflg`]
module"]
#[doc(alias = "TZFLG")]
pub type Tzflg = crate::Reg<tzflg::TzflgSpec>;
#[doc = "Trip-Zone Flag Register"]
pub mod tzflg;
#[doc = "TZCLR (rw) register accessor: Trip-Zone Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzclr`]
module"]
#[doc(alias = "TZCLR")]
pub type Tzclr = crate::Reg<tzclr::TzclrSpec>;
#[doc = "Trip-Zone Clear Register"]
pub mod tzclr;
#[doc = "TZFRC (rw) register accessor: Trip-Zone Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzfrc`]
module"]
#[doc(alias = "TZFRC")]
pub type Tzfrc = crate::Reg<tzfrc::TzfrcSpec>;
#[doc = "Trip-Zone Force Register"]
pub mod tzfrc;
#[doc = "ETSEL (rw) register accessor: Event-Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etsel`]
module"]
#[doc(alias = "ETSEL")]
pub type Etsel = crate::Reg<etsel::EtselSpec>;
#[doc = "Event-Trigger Selection Register"]
pub mod etsel;
#[doc = "ETPS (rw) register accessor: Event-Trigger Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etps`]
module"]
#[doc(alias = "ETPS")]
pub type Etps = crate::Reg<etps::EtpsSpec>;
#[doc = "Event-Trigger Prescale Register"]
pub mod etps;
#[doc = "ETFLG (r) register accessor: Event-Trigger Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etflg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etflg`]
module"]
#[doc(alias = "ETFLG")]
pub type Etflg = crate::Reg<etflg::EtflgSpec>;
#[doc = "Event-Trigger Flag Register"]
pub mod etflg;
#[doc = "ETCLR (rw) register accessor: Event-Trigger Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etclr`]
module"]
#[doc(alias = "ETCLR")]
pub type Etclr = crate::Reg<etclr::EtclrSpec>;
#[doc = "Event-Trigger Clear Register"]
pub mod etclr;
#[doc = "ETFRC (rw) register accessor: Event-Trigger Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etfrc`]
module"]
#[doc(alias = "ETFRC")]
pub type Etfrc = crate::Reg<etfrc::EtfrcSpec>;
#[doc = "Event-Trigger Force Register"]
pub mod etfrc;
#[doc = "PCCTL (rw) register accessor: PWM-Chopper Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcctl`]
module"]
#[doc(alias = "PCCTL")]
pub type Pcctl = crate::Reg<pcctl::PcctlSpec>;
#[doc = "PWM-Chopper Control Register"]
pub mod pcctl;
#[doc = "FWDTH (rw) register accessor: Filter Width select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwdth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwdth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwdth`]
module"]
#[doc(alias = "FWDTH")]
pub type Fwdth = crate::Reg<fwdth::FwdthSpec>;
#[doc = "Filter Width select Register"]
pub mod fwdth;
#[doc = "HDSEL (rw) register accessor: Hold Detector event Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdsel`]
module"]
#[doc(alias = "HDSEL")]
pub type Hdsel = crate::Reg<hdsel::HdselSpec>;
#[doc = "Hold Detector event Select Register"]
pub mod hdsel;
#[doc = "HDCTL (rw) register accessor: Hold Detector Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdctl`]
module"]
#[doc(alias = "HDCTL")]
pub type Hdctl = crate::Reg<hdctl::HdctlSpec>;
#[doc = "Hold Detector Control register"]
pub mod hdctl;
#[doc = "HDEINT (rw) register accessor: Hold Detector Enable Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdeint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdeint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdeint`]
module"]
#[doc(alias = "HDEINT")]
pub type Hdeint = crate::Reg<hdeint::HdeintSpec>;
#[doc = "Hold Detector Enable Interrupt Register"]
pub mod hdeint;
#[doc = "HDFLG (r) register accessor: Hold Detector Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdflg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdflg`]
module"]
#[doc(alias = "HDFLG")]
pub type Hdflg = crate::Reg<hdflg::HdflgSpec>;
#[doc = "Hold Detector Flag Register"]
pub mod hdflg;
#[doc = "HDCLR (rw) register accessor: Register clear HD flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdclr`]
module"]
#[doc(alias = "HDCLR")]
pub type Hdclr = crate::Reg<hdclr::HdclrSpec>;
#[doc = "Register clear HD flag"]
pub mod hdclr;
#[doc = "HDFRC (rw) register accessor: Hold Detector Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdfrc`]
module"]
#[doc(alias = "HDFRC")]
pub type Hdfrc = crate::Reg<hdfrc::HdfrcSpec>;
#[doc = "Hold Detector Force Register"]
pub mod hdfrc;
#[doc = "HDINTCLR (w) register accessor: Hold Detector Interrupt pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdintclr`]
module"]
#[doc(alias = "HDINTCLR")]
pub type Hdintclr = crate::Reg<hdintclr::HdintclrSpec>;
#[doc = "Hold Detector Interrupt pending Clear Register"]
pub mod hdintclr;
#[doc = "TZINTCLR (w) register accessor: Trip-Zone Interrupt pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzintclr`]
module"]
#[doc(alias = "TZINTCLR")]
pub type Tzintclr = crate::Reg<tzintclr::TzintclrSpec>;
#[doc = "Trip-Zone Interrupt pending Clear Register"]
pub mod tzintclr;
#[doc = "INTCLR (w) register accessor: PWM Interrupt pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "PWM Interrupt pending Clear Register"]
pub mod intclr;
