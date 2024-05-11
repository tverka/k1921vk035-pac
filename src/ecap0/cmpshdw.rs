#[doc = "Register `CMPSHDW` reader"]
pub type R = crate::R<CmpshdwSpec>;
#[doc = "Register `CMPSHDW` writer"]
pub type W = crate::W<CmpshdwSpec>;
#[doc = "Field `VAL` reader - Compare shadow value in APWM mode"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Compare shadow value in APWM mode"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare shadow value in APWM mode"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare shadow value in APWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CmpshdwSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Compare shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpshdw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpshdw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpshdwSpec;
impl crate::RegisterSpec for CmpshdwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpshdw::R`](R) reader structure"]
impl crate::Readable for CmpshdwSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpshdw::W`](W) writer structure"]
impl crate::Writable for CmpshdwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPSHDW to value 0"]
impl crate::Resettable for CmpshdwSpec {
    const RESET_VALUE: u32 = 0;
}
