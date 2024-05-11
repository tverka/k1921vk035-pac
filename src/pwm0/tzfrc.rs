#[doc = "Register `TZFRC` reader"]
pub type R = crate::R<TzfrcSpec>;
#[doc = "Register `TZFRC` writer"]
pub type W = crate::W<TzfrcSpec>;
#[doc = "Field `CBC` reader - Force a Cycle-by-Cycle trip event via software"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Force a Cycle-by-Cycle trip event via software"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Force a One-Shot trip event via software"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Force a One-Shot trip event via software"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Force a Cycle-by-Cycle trip event via software"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force a One-Shot trip event via software"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Force a Cycle-by-Cycle trip event via software"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<TzfrcSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Force a One-Shot trip event via software"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<TzfrcSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Trip-Zone Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzfrcSpec;
impl crate::RegisterSpec for TzfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzfrc::R`](R) reader structure"]
impl crate::Readable for TzfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`tzfrc::W`](W) writer structure"]
impl crate::Writable for TzfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZFRC to value 0"]
impl crate::Resettable for TzfrcSpec {
    const RESET_VALUE: u32 = 0;
}
