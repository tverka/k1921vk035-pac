#[doc = "Register `TBPHS` reader"]
pub type R = crate::R<TbphsSpec>;
#[doc = "Register `TBPHS` writer"]
pub type W = crate::W<TbphsSpec>;
#[doc = "Field `TBPHS` reader - "]
pub type TbphsR = crate::FieldReader<u16>;
#[doc = "Field `TBPHS` writer - "]
pub type TbphsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tbphs(&self) -> TbphsR {
        TbphsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn tbphs(&mut self) -> TbphsW<TbphsSpec> {
        TbphsW::new(self, 16)
    }
}
#[doc = "Time-Base Phase Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbphs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbphs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbphsSpec;
impl crate::RegisterSpec for TbphsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbphs::R`](R) reader structure"]
impl crate::Readable for TbphsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbphs::W`](W) writer structure"]
impl crate::Writable for TbphsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBPHS to value 0"]
impl crate::Resettable for TbphsSpec {
    const RESET_VALUE: u32 = 0;
}
