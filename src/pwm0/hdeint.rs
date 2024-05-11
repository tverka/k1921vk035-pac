#[doc = "Register `HDEINT` reader"]
pub type R = crate::R<HdeintSpec>;
#[doc = "Register `HDEINT` writer"]
pub type W = crate::W<HdeintSpec>;
#[doc = "Field `CBC` reader - Hold detector Cycle-by-Cycle interrupt enable"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Hold detector Cycle-by-Cycle interrupt enable"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Hold detector One-Shot interrupt enable"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Hold detector One-Shot interrupt enable"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Hold detector Cycle-by-Cycle interrupt enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hold detector One-Shot interrupt enable"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Hold detector Cycle-by-Cycle interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<HdeintSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Hold detector One-Shot interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<HdeintSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Hold Detector Enable Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdeint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdeint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdeintSpec;
impl crate::RegisterSpec for HdeintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdeint::R`](R) reader structure"]
impl crate::Readable for HdeintSpec {}
#[doc = "`write(|w| ..)` method takes [`hdeint::W`](W) writer structure"]
impl crate::Writable for HdeintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDEINT to value 0"]
impl crate::Resettable for HdeintSpec {
    const RESET_VALUE: u32 = 0;
}
