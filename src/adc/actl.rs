#[doc = "Register `ACTL` reader"]
pub type R = crate::R<ActlSpec>;
#[doc = "Register `ACTL` writer"]
pub type W = crate::W<ActlSpec>;
#[doc = "Field `ADCEN` reader - Enable ADC module"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - Enable ADC module"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRDY` reader - ADC ready for conversions"]
pub type AdcrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable ADC module"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC ready for conversions"]
    #[inline(always)]
    pub fn adcrdy(&self) -> AdcrdyR {
        AdcrdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ADC module"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<ActlSpec> {
        AdcenW::new(self, 0)
    }
}
#[doc = "ADC module control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActlSpec;
impl crate::RegisterSpec for ActlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actl::R`](R) reader structure"]
impl crate::Readable for ActlSpec {}
#[doc = "`write(|w| ..)` method takes [`actl::W`](W) writer structure"]
impl crate::Writable for ActlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTL to value 0"]
impl crate::Resettable for ActlSpec {
    const RESET_VALUE: u32 = 0;
}
