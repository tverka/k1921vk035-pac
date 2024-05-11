#[repr(C)]
#[doc = "SEQ"]
#[doc(alias = "SEQ")]
pub struct Seq {
    srqsel: Srqsel,
    _reserved1: [u8; 0x0c],
    srqctl: Srqctl,
    srqstat: Srqstat,
    sdmactl: Sdmactl,
    scctl: Scctl,
    scval: Scval,
    sdc: Sdc,
    srtmr: Srtmr,
    sfload: Sfload,
    sfifo: Sfifo,
}
impl Seq {
    #[doc = "0x00 - Sequencer request ADC channels selection register"]
    #[inline(always)]
    pub const fn srqsel(&self) -> &Srqsel {
        &self.srqsel
    }
    #[doc = "0x10 - Sequencer request control register"]
    #[inline(always)]
    pub const fn srqctl(&self) -> &Srqctl {
        &self.srqctl
    }
    #[doc = "0x14 - Sequencer request status register"]
    #[inline(always)]
    pub const fn srqstat(&self) -> &Srqstat {
        &self.srqstat
    }
    #[doc = "0x18 - Sequencer DMA control register"]
    #[inline(always)]
    pub const fn sdmactl(&self) -> &Sdmactl {
        &self.sdmactl
    }
    #[doc = "0x1c - Sequencer ADC interrupt and restart counter control register"]
    #[inline(always)]
    pub const fn scctl(&self) -> &Scctl {
        &self.scctl
    }
    #[doc = "0x20 - Sequencer ADC interrupt and restart counter current value register"]
    #[inline(always)]
    pub const fn scval(&self) -> &Scval {
        &self.scval
    }
    #[doc = "0x24 - Sequencer digital comparator selection register"]
    #[inline(always)]
    pub const fn sdc(&self) -> &Sdc {
        &self.sdc
    }
    #[doc = "0x28 - Sequencer ADC restart timer register"]
    #[inline(always)]
    pub const fn srtmr(&self) -> &Srtmr {
        &self.srtmr
    }
    #[doc = "0x2c - Sequencer FIFO load status register"]
    #[inline(always)]
    pub const fn sfload(&self) -> &Sfload {
        &self.sfload
    }
    #[doc = "0x30 - Sequencer FIFO register"]
    #[inline(always)]
    pub const fn sfifo(&self) -> &Sfifo {
        &self.sfifo
    }
}
#[doc = "SRQSEL (rw) register accessor: Sequencer request ADC channels selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqsel`]
module"]
#[doc(alias = "SRQSEL")]
pub type Srqsel = crate::Reg<srqsel::SrqselSpec>;
#[doc = "Sequencer request ADC channels selection register"]
pub mod srqsel;
#[doc = "SRQCTL (rw) register accessor: Sequencer request control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srqctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqctl`]
module"]
#[doc(alias = "SRQCTL")]
pub type Srqctl = crate::Reg<srqctl::SrqctlSpec>;
#[doc = "Sequencer request control register"]
pub mod srqctl;
#[doc = "SRQSTAT (r) register accessor: Sequencer request status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqstat`]
module"]
#[doc(alias = "SRQSTAT")]
pub type Srqstat = crate::Reg<srqstat::SrqstatSpec>;
#[doc = "Sequencer request status register"]
pub mod srqstat;
#[doc = "SDMACTL (rw) register accessor: Sequencer DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmactl`]
module"]
#[doc(alias = "SDMACTL")]
pub type Sdmactl = crate::Reg<sdmactl::SdmactlSpec>;
#[doc = "Sequencer DMA control register"]
pub mod sdmactl;
#[doc = "SCCTL (rw) register accessor: Sequencer ADC interrupt and restart counter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scctl`]
module"]
#[doc(alias = "SCCTL")]
pub type Scctl = crate::Reg<scctl::ScctlSpec>;
#[doc = "Sequencer ADC interrupt and restart counter control register"]
pub mod scctl;
#[doc = "SCVAL (rw) register accessor: Sequencer ADC interrupt and restart counter current value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scval`]
module"]
#[doc(alias = "SCVAL")]
pub type Scval = crate::Reg<scval::ScvalSpec>;
#[doc = "Sequencer ADC interrupt and restart counter current value register"]
pub mod scval;
#[doc = "SDC (rw) register accessor: Sequencer digital comparator selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdc`]
module"]
#[doc(alias = "SDC")]
pub type Sdc = crate::Reg<sdc::SdcSpec>;
#[doc = "Sequencer digital comparator selection register"]
pub mod sdc;
#[doc = "SRTMR (rw) register accessor: Sequencer ADC restart timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srtmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srtmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srtmr`]
module"]
#[doc(alias = "SRTMR")]
pub type Srtmr = crate::Reg<srtmr::SrtmrSpec>;
#[doc = "Sequencer ADC restart timer register"]
pub mod srtmr;
#[doc = "SFLOAD (r) register accessor: Sequencer FIFO load status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfload::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfload`]
module"]
#[doc(alias = "SFLOAD")]
pub type Sfload = crate::Reg<sfload::SfloadSpec>;
#[doc = "Sequencer FIFO load status register"]
pub mod sfload;
#[doc = "SFIFO (r) register accessor: Sequencer FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfifo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfifo`]
module"]
#[doc(alias = "SFIFO")]
pub type Sfifo = crate::Reg<sfifo::SfifoSpec>;
#[doc = "Sequencer FIFO register"]
pub mod sfifo;
