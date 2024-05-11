#[doc = "Register `CAP1` reader"]
pub type R = crate::R<Cap1Spec>;
#[doc = "Register `CAP1` writer"]
pub type W = crate::W<Cap1Spec>;
#[doc = "Field `VAL` reader - Capture 1 value in CAP mode"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Capture 1 value in CAP mode"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture 1 value in CAP mode"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture 1 value in CAP mode"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Cap1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Capture register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap1Spec;
impl crate::RegisterSpec for Cap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap1::R`](R) reader structure"]
impl crate::Readable for Cap1Spec {}
#[doc = "`write(|w| ..)` method takes [`cap1::W`](W) writer structure"]
impl crate::Writable for Cap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP1 to value 0"]
impl crate::Resettable for Cap1Spec {
    const RESET_VALUE: u32 = 0;
}
