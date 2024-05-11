#[doc = "Register `OSECFG` reader"]
pub type R = crate::R<OsecfgSpec>;
#[doc = "Register `OSECFG` writer"]
pub type W = crate::W<OsecfgSpec>;
#[doc = "Field `XOEN` reader - Enable output XO_OSC from external oscillator"]
pub type XoenR = crate::BitReader;
#[doc = "Field `XOEN` writer - Enable output XO_OSC from external oscillator"]
pub type XoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Enable external oscallator"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable external oscallator"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable output XO_OSC from external oscillator"]
    #[inline(always)]
    pub fn xoen(&self) -> XoenR {
        XoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable external oscallator"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable output XO_OSC from external oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn xoen(&mut self) -> XoenW<OsecfgSpec> {
        XoenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable external oscallator"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<OsecfgSpec> {
        EnW::new(self, 1)
    }
}
#[doc = "External oscillator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osecfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osecfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsecfgSpec;
impl crate::RegisterSpec for OsecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osecfg::R`](R) reader structure"]
impl crate::Readable for OsecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`osecfg::W`](W) writer structure"]
impl crate::Writable for OsecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSECFG to value 0"]
impl crate::Resettable for OsecfgSpec {
    const RESET_VALUE: u32 = 0;
}
