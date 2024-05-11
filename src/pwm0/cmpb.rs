#[doc = "Register `CMPB` reader"]
pub type R = crate::R<CmpbSpec>;
#[doc = "Register `CMPB` writer"]
pub type W = crate::W<CmpbSpec>;
#[doc = "Field `CMPB` reader - "]
pub type CmpbR = crate::FieldReader<u16>;
#[doc = "Field `CMPB` writer - "]
pub type CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cmpb(&self) -> CmpbR {
        CmpbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmpb(&mut self) -> CmpbW<CmpbSpec> {
        CmpbW::new(self, 16)
    }
}
#[doc = "Counter-Compare B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpbSpec;
impl crate::RegisterSpec for CmpbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpb::R`](R) reader structure"]
impl crate::Readable for CmpbSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpb::W`](W) writer structure"]
impl crate::Writable for CmpbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPB to value 0"]
impl crate::Resettable for CmpbSpec {
    const RESET_VALUE: u32 = 0;
}
