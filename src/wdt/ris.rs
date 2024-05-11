#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RAWWDTINT` reader - Raw interrupt status from the counter"]
pub type RawwdtintR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw interrupt status from the counter"]
    #[inline(always)]
    pub fn rawwdtint(&self) -> RawwdtintR {
        RawwdtintR::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
