#[doc = "Register `QCTMRLAT` reader"]
pub type R = crate::R<QctmrlatSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Capture Timer Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctmrlat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QctmrlatSpec;
impl crate::RegisterSpec for QctmrlatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qctmrlat::R`](R) reader structure"]
impl crate::Readable for QctmrlatSpec {}
#[doc = "`reset()` method sets QCTMRLAT to value 0"]
impl crate::Resettable for QctmrlatSpec {
    const RESET_VALUE: u32 = 0;
}
