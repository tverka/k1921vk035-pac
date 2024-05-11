#[doc = "Register `QCTMR` reader"]
pub type R = crate::R<QctmrSpec>;
#[doc = "Register `QCTMR` writer"]
pub type W = crate::W<QctmrSpec>;
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
    pub fn val(&mut self) -> ValW<QctmrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Capture Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QctmrSpec;
impl crate::RegisterSpec for QctmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qctmr::R`](R) reader structure"]
impl crate::Readable for QctmrSpec {}
#[doc = "`write(|w| ..)` method takes [`qctmr::W`](W) writer structure"]
impl crate::Writable for QctmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCTMR to value 0"]
impl crate::Resettable for QctmrSpec {
    const RESET_VALUE: u32 = 0;
}
