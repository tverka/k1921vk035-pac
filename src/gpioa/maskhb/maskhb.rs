#[doc = "Register `MASKHB` reader"]
pub type R = crate::R<MaskhbSpec>;
#[doc = "Register `MASKHB` writer"]
pub type W = crate::W<MaskhbSpec>;
#[doc = "Field `VAL` reader - Mask high byte"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Mask high byte"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Mask high byte"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Mask high byte"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<MaskhbSpec> {
        ValW::new(self, 8)
    }
}
#[doc = "Mask register High byte of port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskhb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskhb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskhbSpec;
impl crate::RegisterSpec for MaskhbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskhb::R`](R) reader structure"]
impl crate::Readable for MaskhbSpec {}
#[doc = "`write(|w| ..)` method takes [`maskhb::W`](W) writer structure"]
impl crate::Writable for MaskhbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASKHB to value 0"]
impl crate::Resettable for MaskhbSpec {
    const RESET_VALUE: u32 = 0;
}
