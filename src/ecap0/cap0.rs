#[doc = "Register `CAP0` reader"]
pub type R = crate::R<Cap0Spec>;
#[doc = "Register `CAP0` writer"]
pub type W = crate::W<Cap0Spec>;
#[doc = "Field `VAL` reader - Capture 0 value in CAP mode"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Capture 0 value in CAP mode"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture 0 value in CAP mode"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture 0 value in CAP mode"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Cap0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Capture register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap0Spec;
impl crate::RegisterSpec for Cap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap0::R`](R) reader structure"]
impl crate::Readable for Cap0Spec {}
#[doc = "`write(|w| ..)` method takes [`cap0::W`](W) writer structure"]
impl crate::Writable for Cap0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP0 to value 0"]
impl crate::Resettable for Cap0Spec {
    const RESET_VALUE: u32 = 0;
}
