#[doc = "Register `QUALSAMPLE` reader"]
pub type R = crate::R<QualsampleSpec>;
#[doc = "Register `QUALSAMPLE` writer"]
pub type W = crate::W<QualsampleSpec>;
#[doc = "Field `VAL` reader - Qualifier sample period"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Qualifier sample period"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Qualifier sample period"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qualifier sample period"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<QualsampleSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Qualifier sample period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qualsample::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qualsample::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QualsampleSpec;
impl crate::RegisterSpec for QualsampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qualsample::R`](R) reader structure"]
impl crate::Readable for QualsampleSpec {}
#[doc = "`write(|w| ..)` method takes [`qualsample::W`](W) writer structure"]
impl crate::Writable for QualsampleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUALSAMPLE to value 0"]
impl crate::Resettable for QualsampleSpec {
    const RESET_VALUE: u32 = 0;
}
