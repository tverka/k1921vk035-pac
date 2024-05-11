#[doc = "Register `MASKLB` reader"]
pub type R = crate::R<MasklbSpec>;
#[doc = "Register `MASKLB` writer"]
pub type W = crate::W<MasklbSpec>;
#[doc = "Field `VAL` reader - Mask low byte"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Mask low byte"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Mask low byte"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mask low byte"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<MasklbSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Mask register low byte of port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masklb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masklb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasklbSpec;
impl crate::RegisterSpec for MasklbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`masklb::R`](R) reader structure"]
impl crate::Readable for MasklbSpec {}
#[doc = "`write(|w| ..)` method takes [`masklb::W`](W) writer structure"]
impl crate::Writable for MasklbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASKLB to value 0"]
impl crate::Resettable for MasklbSpec {
    const RESET_VALUE: u32 = 0;
}
