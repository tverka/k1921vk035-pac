#[doc = "Register `HDFRC` reader"]
pub type R = crate::R<HdfrcSpec>;
#[doc = "Register `HDFRC` writer"]
pub type W = crate::W<HdfrcSpec>;
#[doc = "Field `CBC` reader - Force a Cycle-by-Cycle hold detector event via software"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Force a Cycle-by-Cycle hold detector event via software"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Force a One-Shot hold detector event via software"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Force a One-Shot hold detector event via software"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Force a Cycle-by-Cycle hold detector event via software"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force a One-Shot hold detector event via software"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Force a Cycle-by-Cycle hold detector event via software"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<HdfrcSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Force a One-Shot hold detector event via software"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<HdfrcSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Hold Detector Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdfrcSpec;
impl crate::RegisterSpec for HdfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdfrc::R`](R) reader structure"]
impl crate::Readable for HdfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`hdfrc::W`](W) writer structure"]
impl crate::Writable for HdfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDFRC to value 0"]
impl crate::Resettable for HdfrcSpec {
    const RESET_VALUE: u32 = 0;
}
