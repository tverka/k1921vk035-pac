#[doc = "Register `QWDTMR` reader"]
pub type R = crate::R<QwdtmrSpec>;
#[doc = "Register `QWDTMR` writer"]
pub type W = crate::W<QwdtmrSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<QwdtmrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Watchdog Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qwdtmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qwdtmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QwdtmrSpec;
impl crate::RegisterSpec for QwdtmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qwdtmr::R`](R) reader structure"]
impl crate::Readable for QwdtmrSpec {}
#[doc = "`write(|w| ..)` method takes [`qwdtmr::W`](W) writer structure"]
impl crate::Writable for QwdtmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QWDTMR to value 0"]
impl crate::Resettable for QwdtmrSpec {
    const RESET_VALUE: u32 = 0;
}
