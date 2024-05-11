#[doc = "Register `TZSEL` reader"]
pub type R = crate::R<TzselSpec>;
#[doc = "Register `TZSEL` writer"]
pub type W = crate::W<TzselSpec>;
#[doc = "Field `CBC` reader - Cycle-by-Cycle trip-zone 0 enable"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Cycle-by-Cycle trip-zone 0 enable"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - One-Shot trip-zone 0 enable"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - One-Shot trip-zone 0 enable"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cycle-by-Cycle trip-zone 0 enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - One-Shot trip-zone 0 enable"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cycle-by-Cycle trip-zone 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<TzselSpec> {
        CbcW::new(self, 0)
    }
    #[doc = "Bit 8 - One-Shot trip-zone 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<TzselSpec> {
        OstW::new(self, 8)
    }
}
#[doc = "Trip-Zone Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzselSpec;
impl crate::RegisterSpec for TzselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsel::R`](R) reader structure"]
impl crate::Readable for TzselSpec {}
#[doc = "`write(|w| ..)` method takes [`tzsel::W`](W) writer structure"]
impl crate::Writable for TzselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSEL to value 0"]
impl crate::Resettable for TzselSpec {
    const RESET_VALUE: u32 = 0;
}
