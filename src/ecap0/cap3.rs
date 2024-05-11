#[doc = "Register `CAP3` reader"]
pub type R = crate::R<Cap3Spec>;
#[doc = "Register `CAP3` writer"]
pub type W = crate::W<Cap3Spec>;
#[doc = "Field `VAL` reader - Capture 3 value in CAP mode"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Capture 3 value in CAP mode"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture 3 value in CAP mode"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture 3 value in CAP mode"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Cap3Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Capture register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap3Spec;
impl crate::RegisterSpec for Cap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap3::R`](R) reader structure"]
impl crate::Readable for Cap3Spec {}
#[doc = "`write(|w| ..)` method takes [`cap3::W`](W) writer structure"]
impl crate::Writable for Cap3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP3 to value 0"]
impl crate::Resettable for Cap3Spec {
    const RESET_VALUE: u32 = 0;
}
