#[doc = "Register `TZCLR` reader"]
pub type R = crate::R<TzclrSpec>;
#[doc = "Register `TZCLR` writer"]
pub type W = crate::W<TzclrSpec>;
#[doc = "Field `INT` reader - Clear trip-zone interrupt flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Clear trip-zone interrupt flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC` reader - Clear flag for Cycle-By-Cycle trip latch"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Clear flag for Cycle-By-Cycle trip latch"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Clear flag for One-Shot trip latch"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Clear flag for One-Shot trip latch"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear trip-zone interrupt flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear flag for Cycle-By-Cycle trip latch"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear flag for One-Shot trip latch"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear trip-zone interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<TzclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear flag for Cycle-By-Cycle trip latch"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<TzclrSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear flag for One-Shot trip latch"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<TzclrSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Trip-Zone Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzclrSpec;
impl crate::RegisterSpec for TzclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzclr::R`](R) reader structure"]
impl crate::Readable for TzclrSpec {}
#[doc = "`write(|w| ..)` method takes [`tzclr::W`](W) writer structure"]
impl crate::Writable for TzclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZCLR to value 0"]
impl crate::Resettable for TzclrSpec {
    const RESET_VALUE: u32 = 0;
}
