#[doc = "Register `PRDSHDW` reader"]
pub type R = crate::R<PrdshdwSpec>;
#[doc = "Register `PRDSHDW` writer"]
pub type W = crate::W<PrdshdwSpec>;
#[doc = "Field `VAL` reader - Period shadow value in APWM mode"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Period shadow value in APWM mode"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Period shadow value in APWM mode"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Period shadow value in APWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<PrdshdwSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Period shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prdshdw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prdshdw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrdshdwSpec;
impl crate::RegisterSpec for PrdshdwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prdshdw::R`](R) reader structure"]
impl crate::Readable for PrdshdwSpec {}
#[doc = "`write(|w| ..)` method takes [`prdshdw::W`](W) writer structure"]
impl crate::Writable for PrdshdwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRDSHDW to value 0"]
impl crate::Resettable for PrdshdwSpec {
    const RESET_VALUE: u32 = 0;
}
