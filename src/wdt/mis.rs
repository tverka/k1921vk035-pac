#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `WDTINT` reader - Enabled interrupt status from the counter"]
pub type WdtintR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enabled interrupt status from the counter"]
    #[inline(always)]
    pub fn wdtint(&self) -> WdtintR {
        WdtintR::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
