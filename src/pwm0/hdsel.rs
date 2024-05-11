#[doc = "Register `HDSEL` reader"]
pub type R = crate::R<HdselSpec>;
#[doc = "Register `HDSEL` writer"]
pub type W = crate::W<HdselSpec>;
#[doc = "Field `ADCDC0` reader - Hold detector event by ADC Digital Comparator 0 enable"]
pub type Adcdc0R = crate::BitReader;
#[doc = "Field `ADCDC0` writer - Hold detector event by ADC Digital Comparator 0 enable"]
pub type Adcdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDC1` reader - Hold detector event by ADC Digital Comparator 1 enable"]
pub type Adcdc1R = crate::BitReader;
#[doc = "Field `ADCDC1` writer - Hold detector event by ADC Digital Comparator 1 enable"]
pub type Adcdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDC2` reader - Hold detector event by ADC Digital Comparator 2 enable"]
pub type Adcdc2R = crate::BitReader;
#[doc = "Field `ADCDC2` writer - Hold detector event by ADC Digital Comparator 2 enable"]
pub type Adcdc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDC3` reader - Hold detector event by ADC Digital Comparator 3 enable"]
pub type Adcdc3R = crate::BitReader;
#[doc = "Field `ADCDC3` writer - Hold detector event by ADC Digital Comparator 3 enable"]
pub type Adcdc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC` reader - Cycle-by-Cycle hold detector enable"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Cycle-by-Cycle hold detector enable"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - One-Shot hold detector enable"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - One-Shot hold detector enable"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hold detector event by ADC Digital Comparator 0 enable"]
    #[inline(always)]
    pub fn adcdc0(&self) -> Adcdc0R {
        Adcdc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hold detector event by ADC Digital Comparator 1 enable"]
    #[inline(always)]
    pub fn adcdc1(&self) -> Adcdc1R {
        Adcdc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hold detector event by ADC Digital Comparator 2 enable"]
    #[inline(always)]
    pub fn adcdc2(&self) -> Adcdc2R {
        Adcdc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hold detector event by ADC Digital Comparator 3 enable"]
    #[inline(always)]
    pub fn adcdc3(&self) -> Adcdc3R {
        Adcdc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 28 - Cycle-by-Cycle hold detector enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - One-Shot hold detector enable"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hold detector event by ADC Digital Comparator 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcdc0(&mut self) -> Adcdc0W<HdselSpec> {
        Adcdc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Hold detector event by ADC Digital Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcdc1(&mut self) -> Adcdc1W<HdselSpec> {
        Adcdc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Hold detector event by ADC Digital Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcdc2(&mut self) -> Adcdc2W<HdselSpec> {
        Adcdc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Hold detector event by ADC Digital Comparator 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcdc3(&mut self) -> Adcdc3W<HdselSpec> {
        Adcdc3W::new(self, 3)
    }
    #[doc = "Bit 28 - Cycle-by-Cycle hold detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<HdselSpec> {
        CbcW::new(self, 28)
    }
    #[doc = "Bit 31 - One-Shot hold detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<HdselSpec> {
        OstW::new(self, 31)
    }
}
#[doc = "Hold Detector event Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdselSpec;
impl crate::RegisterSpec for HdselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdsel::R`](R) reader structure"]
impl crate::Readable for HdselSpec {}
#[doc = "`write(|w| ..)` method takes [`hdsel::W`](W) writer structure"]
impl crate::Writable for HdselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDSEL to value 0"]
impl crate::Resettable for HdselSpec {
    const RESET_VALUE: u32 = 0;
}
