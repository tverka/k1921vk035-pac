#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    pudel: Pudel,
    pden: Pden,
    rxeven: Rxeven,
}
impl RegisterBlock {
    #[doc = "0x00 - PMU Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - PMU Powerup Delay Value"]
    #[inline(always)]
    pub const fn pudel(&self) -> &Pudel {
        &self.pudel
    }
    #[doc = "0x08 - PMU Enable Powerdown for peripheral"]
    #[inline(always)]
    pub const fn pden(&self) -> &Pden {
        &self.pden
    }
    #[doc = "0x0c - PMU RX Event generation enable register"]
    #[inline(always)]
    pub const fn rxeven(&self) -> &Rxeven {
        &self.rxeven
    }
}
#[doc = "CFG (rw) register accessor: PMU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "PMU Configuration Register"]
pub mod cfg;
#[doc = "PUDEL (rw) register accessor: PMU Powerup Delay Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pudel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pudel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pudel`]
module"]
#[doc(alias = "PUDEL")]
pub type Pudel = crate::Reg<pudel::PudelSpec>;
#[doc = "PMU Powerup Delay Value"]
pub mod pudel;
#[doc = "PDEN (rw) register accessor: PMU Enable Powerdown for peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pden::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pden::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pden`]
module"]
#[doc(alias = "PDEN")]
pub type Pden = crate::Reg<pden::PdenSpec>;
#[doc = "PMU Enable Powerdown for peripheral"]
pub mod pden;
#[doc = "RXEVEN (rw) register accessor: PMU RX Event generation enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxeven::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxeven::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxeven`]
module"]
#[doc(alias = "RXEVEN")]
pub type Rxeven = crate::Reg<rxeven::RxevenSpec>;
#[doc = "PMU RX Event generation enable register"]
pub mod rxeven;
