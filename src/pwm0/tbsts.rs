#[doc = "Register `TBSTS` reader"]
pub type R = crate::R<TbstsSpec>;
#[doc = "Register `TBSTS` writer"]
pub type W = crate::W<TbstsSpec>;
#[doc = "Field `CTRDIR` reader - Time-Base counter direction status bit"]
pub type CtrdirR = crate::BitReader;
#[doc = "Field `CTRDIR` writer - Time-Base counter direction status bit"]
pub type CtrdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCI` reader - Input synchronization latched status bit"]
pub type SynciR = crate::BitReader;
#[doc = "Field `SYNCI` writer - Input synchronization latched status bit"]
pub type SynciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRMAX` reader - Time-Base counter max latched status bit"]
pub type CtrmaxR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Time-Base counter direction status bit"]
    #[inline(always)]
    pub fn ctrdir(&self) -> CtrdirR {
        CtrdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input synchronization latched status bit"]
    #[inline(always)]
    pub fn synci(&self) -> SynciR {
        SynciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time-Base counter max latched status bit"]
    #[inline(always)]
    pub fn ctrmax(&self) -> CtrmaxR {
        CtrmaxR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time-Base counter direction status bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrdir(&mut self) -> CtrdirW<TbstsSpec> {
        CtrdirW::new(self, 0)
    }
    #[doc = "Bit 1 - Input synchronization latched status bit"]
    #[inline(always)]
    #[must_use]
    pub fn synci(&mut self) -> SynciW<TbstsSpec> {
        SynciW::new(self, 1)
    }
}
#[doc = "Time-Base Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbstsSpec;
impl crate::RegisterSpec for TbstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbsts::R`](R) reader structure"]
impl crate::Readable for TbstsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbsts::W`](W) writer structure"]
impl crate::Writable for TbstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBSTS to value 0"]
impl crate::Resettable for TbstsSpec {
    const RESET_VALUE: u32 = 0;
}
