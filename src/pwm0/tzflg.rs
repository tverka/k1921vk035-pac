#[doc = "Register `TZFLG` reader"]
pub type R = crate::R<TzflgSpec>;
#[doc = "Field `INT` reader - Latched trip interrupt status flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `CBC` reader - Latched status flag for Cycle-By-Cycle trip event"]
pub type CbcR = crate::BitReader;
#[doc = "Field `OST` reader - Latched status flag for a One-Shot trip event"]
pub type OstR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Latched trip interrupt status flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latched status flag for Cycle-By-Cycle trip event"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Latched status flag for a One-Shot trip event"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Trip-Zone Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzflg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzflgSpec;
impl crate::RegisterSpec for TzflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzflg::R`](R) reader structure"]
impl crate::Readable for TzflgSpec {}
#[doc = "`reset()` method sets TZFLG to value 0"]
impl crate::Resettable for TzflgSpec {
    const RESET_VALUE: u32 = 0;
}
