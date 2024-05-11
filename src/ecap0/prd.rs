#[doc = "Register `PRD` reader"]
pub type R = crate::R<PrdSpec>;
#[doc = "Register `PRD` writer"]
pub type W = crate::W<PrdSpec>;
#[doc = "Field `VAL` reader - Period value in APWM mode"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Period value in APWM mode"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Period value in APWM mode"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Period value in APWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<PrdSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrdSpec;
impl crate::RegisterSpec for PrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prd::R`](R) reader structure"]
impl crate::Readable for PrdSpec {}
#[doc = "`write(|w| ..)` method takes [`prd::W`](W) writer structure"]
impl crate::Writable for PrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRD to value 0"]
impl crate::Resettable for PrdSpec {
    const RESET_VALUE: u32 = 0;
}
