#[doc = "Register `QPOSCNT` reader"]
pub type R = crate::R<QposcntSpec>;
#[doc = "Register `QPOSCNT` writer"]
pub type W = crate::W<QposcntSpec>;
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
    pub fn val(&mut self) -> ValW<QposcntSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Position Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposcntSpec;
impl crate::RegisterSpec for QposcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposcnt::R`](R) reader structure"]
impl crate::Readable for QposcntSpec {}
#[doc = "`write(|w| ..)` method takes [`qposcnt::W`](W) writer structure"]
impl crate::Writable for QposcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QPOSCNT to value 0"]
impl crate::Resettable for QposcntSpec {
    const RESET_VALUE: u32 = 0;
}
