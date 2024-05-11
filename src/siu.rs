#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    pwmsync: Pwmsync,
    servctl: Servctl,
    clkoutctl: Clkoutctl,
    remapaf: Remapaf,
    dmamux: Dmamux,
    _reserved5: [u8; 0x0fd8],
    chipid: Chipid,
}
impl RegisterBlock {
    #[doc = "0x10 - PWM syncronization control register"]
    #[inline(always)]
    pub const fn pwmsync(&self) -> &Pwmsync {
        &self.pwmsync
    }
    #[doc = "0x14 - Service mode control register"]
    #[inline(always)]
    pub const fn servctl(&self) -> &Servctl {
        &self.servctl
    }
    #[doc = "0x18 - Clock out control register"]
    #[inline(always)]
    pub const fn clkoutctl(&self) -> &Clkoutctl {
        &self.clkoutctl
    }
    #[doc = "0x1c - QEP altfunc control"]
    #[inline(always)]
    pub const fn remapaf(&self) -> &Remapaf {
        &self.remapaf
    }
    #[doc = "0x20 - DMA external requests mux control register"]
    #[inline(always)]
    pub const fn dmamux(&self) -> &Dmamux {
        &self.dmamux
    }
    #[doc = "0xffc - Chip identifier"]
    #[inline(always)]
    pub const fn chipid(&self) -> &Chipid {
        &self.chipid
    }
}
#[doc = "PWMSYNC (rw) register accessor: PWM syncronization control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwmsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwmsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmsync`]
module"]
#[doc(alias = "PWMSYNC")]
pub type Pwmsync = crate::Reg<pwmsync::PwmsyncSpec>;
#[doc = "PWM syncronization control register"]
pub mod pwmsync;
#[doc = "SERVCTL (rw) register accessor: Service mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`servctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`servctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@servctl`]
module"]
#[doc(alias = "SERVCTL")]
pub type Servctl = crate::Reg<servctl::ServctlSpec>;
#[doc = "Service mode control register"]
pub mod servctl;
#[doc = "CLKOUTCTL (rw) register accessor: Clock out control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutctl`]
module"]
#[doc(alias = "CLKOUTCTL")]
pub type Clkoutctl = crate::Reg<clkoutctl::ClkoutctlSpec>;
#[doc = "Clock out control register"]
pub mod clkoutctl;
#[doc = "REMAPAF (rw) register accessor: QEP altfunc control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remapaf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remapaf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remapaf`]
module"]
#[doc(alias = "REMAPAF")]
pub type Remapaf = crate::Reg<remapaf::RemapafSpec>;
#[doc = "QEP altfunc control"]
pub mod remapaf;
#[doc = "DMAMUX (rw) register accessor: DMA external requests mux control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux`]
module"]
#[doc(alias = "DMAMUX")]
pub type Dmamux = crate::Reg<dmamux::DmamuxSpec>;
#[doc = "DMA external requests mux control register"]
pub mod dmamux;
#[doc = "CHIPID (r) register accessor: Chip identifier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chipid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chipid`]
module"]
#[doc(alias = "CHIPID")]
pub type Chipid = crate::Reg<chipid::ChipidSpec>;
#[doc = "Chip identifier"]
pub mod chipid;
