#[doc = "Register `TBPRD` reader"]
pub type R = crate::R<TbprdSpec>;
#[doc = "Register `TBPRD` writer"]
pub type W = crate::W<TbprdSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<TbprdSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Time-Base Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbprdSpec;
impl crate::RegisterSpec for TbprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbprd::R`](R) reader structure"]
impl crate::Readable for TbprdSpec {}
#[doc = "`write(|w| ..)` method takes [`tbprd::W`](W) writer structure"]
impl crate::Writable for TbprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPRD to value 0"]
impl crate::Resettable for TbprdSpec {
    const RESET_VALUE: u32 = 0;
}
