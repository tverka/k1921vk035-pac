#[doc = "Register `CAP2` reader"]
pub type R = crate::R<Cap2Spec>;
#[doc = "Register `CAP2` writer"]
pub type W = crate::W<Cap2Spec>;
#[doc = "Field `VAL` reader - Capture 2 value in CAP mode"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Capture 2 value in CAP mode"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture 2 value in CAP mode"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture 2 value in CAP mode"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Cap2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Capture register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap2Spec;
impl crate::RegisterSpec for Cap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap2::R`](R) reader structure"]
impl crate::Readable for Cap2Spec {}
#[doc = "`write(|w| ..)` method takes [`cap2::W`](W) writer structure"]
impl crate::Writable for Cap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP2 to value 0"]
impl crate::Resettable for Cap2Spec {
    const RESET_VALUE: u32 = 0;
}
