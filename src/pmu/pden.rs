#[doc = "Register `PDEN` reader"]
pub type R = crate::R<PdenSpec>;
#[doc = "Register `PDEN` writer"]
pub type W = crate::W<PdenSpec>;
#[doc = "Field `PLLPD` reader - Enable powerdown for PLL"]
pub type PllpdR = crate::BitReader;
#[doc = "Field `PLLPD` writer - Enable powerdown for PLL"]
pub type PllpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFLASHPD` reader - Enable powerdown for MFLASH"]
pub type MflashpdR = crate::BitReader;
#[doc = "Field `MFLASHPD` writer - Enable powerdown for MFLASH"]
pub type MflashpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEPD` reader - Enable powerdown for external oscillator"]
pub type OsepdR = crate::BitReader;
#[doc = "Field `OSEPD` writer - Enable powerdown for external oscillator"]
pub type OsepdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable powerdown for PLL"]
    #[inline(always)]
    pub fn pllpd(&self) -> PllpdR {
        PllpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable powerdown for MFLASH"]
    #[inline(always)]
    pub fn mflashpd(&self) -> MflashpdR {
        MflashpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable powerdown for external oscillator"]
    #[inline(always)]
    pub fn osepd(&self) -> OsepdR {
        OsepdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable powerdown for PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pllpd(&mut self) -> PllpdW<PdenSpec> {
        PllpdW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable powerdown for MFLASH"]
    #[inline(always)]
    #[must_use]
    pub fn mflashpd(&mut self) -> MflashpdW<PdenSpec> {
        MflashpdW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable powerdown for external oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn osepd(&mut self) -> OsepdW<PdenSpec> {
        OsepdW::new(self, 2)
    }
}
#[doc = "PMU Enable Powerdown for peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pden::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pden::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdenSpec;
impl crate::RegisterSpec for PdenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pden::R`](R) reader structure"]
impl crate::Readable for PdenSpec {}
#[doc = "`write(|w| ..)` method takes [`pden::W`](W) writer structure"]
impl crate::Writable for PdenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDEN to value 0"]
impl crate::Resettable for PdenSpec {
    const RESET_VALUE: u32 = 0;
}
