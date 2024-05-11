#[doc = "Register `QPOSLAT` reader"]
pub type R = crate::R<QposlatSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Position Counter Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposlat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposlatSpec;
impl crate::RegisterSpec for QposlatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposlat::R`](R) reader structure"]
impl crate::Readable for QposlatSpec {}
#[doc = "`reset()` method sets QPOSLAT to value 0"]
impl crate::Resettable for QposlatSpec {
    const RESET_VALUE: u32 = 0;
}
