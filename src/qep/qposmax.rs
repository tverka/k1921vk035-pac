#[doc = "Register `QPOSMAX` reader"]
pub type R = crate::R<QposmaxSpec>;
#[doc = "Register `QPOSMAX` writer"]
pub type W = crate::W<QposmaxSpec>;
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
    pub fn val(&mut self) -> ValW<QposmaxSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Maximum Position Count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposmax::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposmax::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposmaxSpec;
impl crate::RegisterSpec for QposmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposmax::R`](R) reader structure"]
impl crate::Readable for QposmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`qposmax::W`](W) writer structure"]
impl crate::Writable for QposmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QPOSMAX to value 0"]
impl crate::Resettable for QposmaxSpec {
    const RESET_VALUE: u32 = 0;
}
