#[doc = "Register `ALTBASEPTR` reader"]
pub type R = crate::R<AltbaseptrSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Channel alternate control data base pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altbaseptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltbaseptrSpec;
impl crate::RegisterSpec for AltbaseptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altbaseptr::R`](R) reader structure"]
impl crate::Readable for AltbaseptrSpec {}
#[doc = "`reset()` method sets ALTBASEPTR to value 0"]
impl crate::Resettable for AltbaseptrSpec {
    const RESET_VALUE: u32 = 0;
}
