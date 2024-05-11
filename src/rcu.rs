#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    osicfg: Osicfg,
    osecfg: Osecfg,
    pllcfg: Pllcfg,
    plldiv: Plldiv,
    sysclkcfg: Sysclkcfg,
    sysclkstat: Sysclkstat,
    secprd: Secprd,
    sysrstcfg: Sysrstcfg,
    sysrststat: Sysrststat,
    inten: Inten,
    intstat: Intstat,
    tracecfg: Tracecfg,
    clkoutcfg: Clkoutcfg,
    wdtcfg: Wdtcfg,
    _reserved14: [u8; 0x28],
    uartcfg: [Uartcfg; 2],
    _reserved15: [u8; 0x18],
    spicfg: Spicfg,
    _reserved16: [u8; 0x1c],
    adccfg: Adccfg,
    _reserved17: [u8; 0x3c],
    pclkcfg: Pclkcfg,
    _reserved18: [u8; 0x0c],
    prstcfg: Prstcfg,
    _reserved19: [u8; 0x0c],
    hclkcfg: Hclkcfg,
    hrstcfg: Hrstcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal oscillator configuration register"]
    #[inline(always)]
    pub const fn osicfg(&self) -> &Osicfg {
        &self.osicfg
    }
    #[doc = "0x04 - External oscillator configuration register"]
    #[inline(always)]
    pub const fn osecfg(&self) -> &Osecfg {
        &self.osecfg
    }
    #[doc = "0x08 - PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfg(&self) -> &Pllcfg {
        &self.pllcfg
    }
    #[doc = "0x0c - PLL divider register"]
    #[inline(always)]
    pub const fn plldiv(&self) -> &Plldiv {
        &self.plldiv
    }
    #[doc = "0x10 - System clock configuration register"]
    #[inline(always)]
    pub const fn sysclkcfg(&self) -> &Sysclkcfg {
        &self.sysclkcfg
    }
    #[doc = "0x14 - System clock status register"]
    #[inline(always)]
    pub const fn sysclkstat(&self) -> &Sysclkstat {
        &self.sysclkstat
    }
    #[doc = "0x18 - Security sysytem clock period register"]
    #[inline(always)]
    pub const fn secprd(&self) -> &Secprd {
        &self.secprd
    }
    #[doc = "0x1c - System reset configuration register"]
    #[inline(always)]
    pub const fn sysrstcfg(&self) -> &Sysrstcfg {
        &self.sysrstcfg
    }
    #[doc = "0x20 - Reset status register"]
    #[inline(always)]
    pub const fn sysrststat(&self) -> &Sysrststat {
        &self.sysrststat
    }
    #[doc = "0x24 - Interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x28 - Interrupt status register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x2c - Trace clock configuration register"]
    #[inline(always)]
    pub const fn tracecfg(&self) -> &Tracecfg {
        &self.tracecfg
    }
    #[doc = "0x30 - Clockout configuration register"]
    #[inline(always)]
    pub const fn clkoutcfg(&self) -> &Clkoutcfg {
        &self.clkoutcfg
    }
    #[doc = "0x34 - WatchDog configuration register"]
    #[inline(always)]
    pub const fn wdtcfg(&self) -> &Wdtcfg {
        &self.wdtcfg
    }
    #[doc = "0x60..0x68 - UARTCFG"]
    #[inline(always)]
    pub const fn uartcfg(&self, n: usize) -> &Uartcfg {
        &self.uartcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x68 - UARTCFG"]
    #[inline(always)]
    pub fn uartcfg_iter(&self) -> impl Iterator<Item = &Uartcfg> {
        self.uartcfg.iter()
    }
    #[doc = "0x80 - SPI clock and reset configuration register"]
    #[inline(always)]
    pub const fn spicfg(&self) -> &Spicfg {
        &self.spicfg
    }
    #[doc = "0xa0 - ADC clock and reset configuration register"]
    #[inline(always)]
    pub const fn adccfg(&self) -> &Adccfg {
        &self.adccfg
    }
    #[doc = "0xe0 - APB clock configuration register"]
    #[inline(always)]
    pub const fn pclkcfg(&self) -> &Pclkcfg {
        &self.pclkcfg
    }
    #[doc = "0xf0 - APB reset configuration register"]
    #[inline(always)]
    pub const fn prstcfg(&self) -> &Prstcfg {
        &self.prstcfg
    }
    #[doc = "0x100 - AHB clock configuration register"]
    #[inline(always)]
    pub const fn hclkcfg(&self) -> &Hclkcfg {
        &self.hclkcfg
    }
    #[doc = "0x104 - AHB reset configuration register"]
    #[inline(always)]
    pub const fn hrstcfg(&self) -> &Hrstcfg {
        &self.hrstcfg
    }
}
#[doc = "OSICFG (rw) register accessor: Internal oscillator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osicfg`]
module"]
#[doc(alias = "OSICFG")]
pub type Osicfg = crate::Reg<osicfg::OsicfgSpec>;
#[doc = "Internal oscillator configuration register"]
pub mod osicfg;
#[doc = "OSECFG (rw) register accessor: External oscillator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osecfg`]
module"]
#[doc(alias = "OSECFG")]
pub type Osecfg = crate::Reg<osecfg::OsecfgSpec>;
#[doc = "External oscillator configuration register"]
pub mod osecfg;
#[doc = "PLLCFG (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfg`]
module"]
#[doc(alias = "PLLCFG")]
pub type Pllcfg = crate::Reg<pllcfg::PllcfgSpec>;
#[doc = "PLL configuration register"]
pub mod pllcfg;
#[doc = "PLLDIV (rw) register accessor: PLL divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plldiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plldiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plldiv`]
module"]
#[doc(alias = "PLLDIV")]
pub type Plldiv = crate::Reg<plldiv::PlldivSpec>;
#[doc = "PLL divider register"]
pub mod plldiv;
#[doc = "SYSCLKCFG (rw) register accessor: System clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkcfg`]
module"]
#[doc(alias = "SYSCLKCFG")]
pub type Sysclkcfg = crate::Reg<sysclkcfg::SysclkcfgSpec>;
#[doc = "System clock configuration register"]
pub mod sysclkcfg;
#[doc = "SYSCLKSTAT (r) register accessor: System clock status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkstat`]
module"]
#[doc(alias = "SYSCLKSTAT")]
pub type Sysclkstat = crate::Reg<sysclkstat::SysclkstatSpec>;
#[doc = "System clock status register"]
pub mod sysclkstat;
#[doc = "SECPRD (rw) register accessor: Security sysytem clock period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secprd`]
module"]
#[doc(alias = "SECPRD")]
pub type Secprd = crate::Reg<secprd::SecprdSpec>;
#[doc = "Security sysytem clock period register"]
pub mod secprd;
#[doc = "SYSRSTCFG (rw) register accessor: System reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysrstcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysrstcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrstcfg`]
module"]
#[doc(alias = "SYSRSTCFG")]
pub type Sysrstcfg = crate::Reg<sysrstcfg::SysrstcfgSpec>;
#[doc = "System reset configuration register"]
pub mod sysrstcfg;
#[doc = "SYSRSTSTAT (rw) register accessor: Reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysrststat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysrststat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrststat`]
module"]
#[doc(alias = "SYSRSTSTAT")]
pub type Sysrststat = crate::Reg<sysrststat::SysrststatSpec>;
#[doc = "Reset status register"]
pub mod sysrststat;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "TRACECFG (rw) register accessor: Trace clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tracecfg`]
module"]
#[doc(alias = "TRACECFG")]
pub type Tracecfg = crate::Reg<tracecfg::TracecfgSpec>;
#[doc = "Trace clock configuration register"]
pub mod tracecfg;
#[doc = "CLKOUTCFG (rw) register accessor: Clockout configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutcfg`]
module"]
#[doc(alias = "CLKOUTCFG")]
pub type Clkoutcfg = crate::Reg<clkoutcfg::ClkoutcfgSpec>;
#[doc = "Clockout configuration register"]
pub mod clkoutcfg;
#[doc = "WDTCFG (rw) register accessor: WatchDog configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcfg`]
module"]
#[doc(alias = "WDTCFG")]
pub type Wdtcfg = crate::Reg<wdtcfg::WdtcfgSpec>;
#[doc = "WatchDog configuration register"]
pub mod wdtcfg;
#[doc = "UARTCFG"]
pub use self::uartcfg::Uartcfg;
#[doc = r"Cluster"]
#[doc = "UARTCFG"]
pub mod uartcfg;
#[doc = "SPICFG (rw) register accessor: SPI clock and reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spicfg`]
module"]
#[doc(alias = "SPICFG")]
pub type Spicfg = crate::Reg<spicfg::SpicfgSpec>;
#[doc = "SPI clock and reset configuration register"]
pub mod spicfg;
#[doc = "ADCCFG (rw) register accessor: ADC clock and reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccfg`]
module"]
#[doc(alias = "ADCCFG")]
pub type Adccfg = crate::Reg<adccfg::AdccfgSpec>;
#[doc = "ADC clock and reset configuration register"]
pub mod adccfg;
#[doc = "PCLKCFG (rw) register accessor: APB clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkcfg`]
module"]
#[doc(alias = "PCLKCFG")]
pub type Pclkcfg = crate::Reg<pclkcfg::PclkcfgSpec>;
#[doc = "APB clock configuration register"]
pub mod pclkcfg;
#[doc = "PRSTCFG (rw) register accessor: APB reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prstcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstcfg`]
module"]
#[doc(alias = "PRSTCFG")]
pub type Prstcfg = crate::Reg<prstcfg::PrstcfgSpec>;
#[doc = "APB reset configuration register"]
pub mod prstcfg;
#[doc = "HCLKCFG (rw) register accessor: AHB clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hclkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hclkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hclkcfg`]
module"]
#[doc(alias = "HCLKCFG")]
pub type Hclkcfg = crate::Reg<hclkcfg::HclkcfgSpec>;
#[doc = "AHB clock configuration register"]
pub mod hclkcfg;
#[doc = "HRSTCFG (rw) register accessor: AHB reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrstcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrstcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrstcfg`]
module"]
#[doc(alias = "HRSTCFG")]
pub type Hrstcfg = crate::Reg<hrstcfg::HrstcfgSpec>;
#[doc = "AHB reset configuration register"]
pub mod hrstcfg;
