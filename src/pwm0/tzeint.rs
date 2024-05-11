#[doc = "Register `TZEINT` reader"]
pub type R = crate::R<TzeintSpec>;
#[doc = "Register `TZEINT` writer"]
pub type W = crate::W<TzeintSpec>;
#[doc = "Field `CBC` reader - Trip-zone Cycle-by-Cycle interrupt enable"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Trip-zone Cycle-by-Cycle interrupt enable"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Trip-zone One-Shot interrupt enable"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Trip-zone One-Shot interrupt enable"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Trip-zone Cycle-by-Cycle interrupt enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trip-zone One-Shot interrupt enable"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Trip-zone Cycle-by-Cycle interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<TzeintSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Trip-zone One-Shot interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<TzeintSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Trip-Zone Enable Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzeint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzeint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzeintSpec;
impl crate::RegisterSpec for TzeintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzeint::R`](R) reader structure"]
impl crate::Readable for TzeintSpec {}
#[doc = "`write(|w| ..)` method takes [`tzeint::W`](W) writer structure"]
impl crate::Writable for TzeintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZEINT to value 0"]
impl crate::Resettable for TzeintSpec {
    const RESET_VALUE: u32 = 0;
}
