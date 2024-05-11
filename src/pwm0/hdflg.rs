#[doc = "Register `HDFLG` reader"]
pub type R = crate::R<HdflgSpec>;
#[doc = "Field `INT` reader - Latched hold detector interrupt status flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `CBC` reader - Latched status flag for hold detector Cycle-by-Cycle event"]
pub type CbcR = crate::BitReader;
#[doc = "Field `OST` reader - Latched status flag for hold detector One-Shot event"]
pub type OstR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Latched hold detector interrupt status flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Latched status flag for hold detector Cycle-by-Cycle event"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Latched status flag for hold detector One-Shot event"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Hold Detector Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdflg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdflgSpec;
impl crate::RegisterSpec for HdflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdflg::R`](R) reader structure"]
impl crate::Readable for HdflgSpec {}
#[doc = "`reset()` method sets HDFLG to value 0"]
impl crate::Resettable for HdflgSpec {
    const RESET_VALUE: u32 = 0;
}
