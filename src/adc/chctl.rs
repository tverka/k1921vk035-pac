#[repr(C)]
#[doc = "CHCTL"]
#[doc(alias = "CHCTL")]
pub struct _Chctl {
    chctl: Chctl,
}
impl _Chctl {
    #[doc = "0x00 - ADC channel control register"]
    #[inline(always)]
    pub const fn chctl(&self) -> &Chctl {
        &self.chctl
    }
}
#[doc = "CHCTL (rw) register accessor: ADC channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl`]
module"]
#[doc(alias = "CHCTL")]
pub type Chctl = crate::Reg<chctl::ChctlSpec>;
#[doc = "ADC channel control register"]
pub mod chctl;
