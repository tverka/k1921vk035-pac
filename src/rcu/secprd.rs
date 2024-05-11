#[doc = "Register `SECPRD` reader"]
pub type R = crate::R<SecprdSpec>;
#[doc = "Register `SECPRD` writer"]
pub type W = crate::W<SecprdSpec>;
#[doc = "Field `OSECLK` reader - Max counter value for external oscillator clock fail detection"]
pub type OseclkR = crate::FieldReader;
#[doc = "Field `OSECLK` writer - Max counter value for external oscillator clock fail detection"]
pub type OseclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLCLK` reader - Max counter value for PLL clock fail detection"]
pub type PllclkR = crate::FieldReader;
#[doc = "Field `PLLCLK` writer - Max counter value for PLL clock fail detection"]
pub type PllclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLDIVCLK` reader - Max counter value for PLL clock fail detection"]
pub type PlldivclkR = crate::FieldReader;
#[doc = "Field `PLLDIVCLK` writer - Max counter value for PLL clock fail detection"]
pub type PlldivclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Max counter value for external oscillator clock fail detection"]
    #[inline(always)]
    pub fn oseclk(&self) -> OseclkR {
        OseclkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Max counter value for PLL clock fail detection"]
    #[inline(always)]
    pub fn pllclk(&self) -> PllclkR {
        PllclkR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Max counter value for PLL clock fail detection"]
    #[inline(always)]
    pub fn plldivclk(&self) -> PlldivclkR {
        PlldivclkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Max counter value for external oscillator clock fail detection"]
    #[inline(always)]
    #[must_use]
    pub fn oseclk(&mut self) -> OseclkW<SecprdSpec> {
        OseclkW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Max counter value for PLL clock fail detection"]
    #[inline(always)]
    #[must_use]
    pub fn pllclk(&mut self) -> PllclkW<SecprdSpec> {
        PllclkW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Max counter value for PLL clock fail detection"]
    #[inline(always)]
    #[must_use]
    pub fn plldivclk(&mut self) -> PlldivclkW<SecprdSpec> {
        PlldivclkW::new(self, 24)
    }
}
#[doc = "Security sysytem clock period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecprdSpec;
impl crate::RegisterSpec for SecprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secprd::R`](R) reader structure"]
impl crate::Readable for SecprdSpec {}
#[doc = "`write(|w| ..)` method takes [`secprd::W`](W) writer structure"]
impl crate::Writable for SecprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECPRD to value 0"]
impl crate::Resettable for SecprdSpec {
    const RESET_VALUE: u32 = 0;
}
