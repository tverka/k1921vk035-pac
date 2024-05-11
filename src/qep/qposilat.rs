#[doc = "Register `QPOSILAT` reader"]
pub type R = crate::R<QposilatSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Index Position Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposilat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposilatSpec;
impl crate::RegisterSpec for QposilatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposilat::R`](R) reader structure"]
impl crate::Readable for QposilatSpec {}
#[doc = "`reset()` method sets QPOSILAT to value 0"]
impl crate::Resettable for QposilatSpec {
    const RESET_VALUE: u32 = 0;
}
