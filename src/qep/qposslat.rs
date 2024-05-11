#[doc = "Register `QPOSSLAT` reader"]
pub type R = crate::R<QposslatSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Strobe Position Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposslat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposslatSpec;
impl crate::RegisterSpec for QposslatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposslat::R`](R) reader structure"]
impl crate::Readable for QposslatSpec {}
#[doc = "`reset()` method sets QPOSSLAT to value 0"]
impl crate::Resettable for QposslatSpec {
    const RESET_VALUE: u32 = 0;
}
