#[doc = "Register `OSICFG` reader"]
pub type R = crate::R<OsicfgSpec>;
#[doc = "Register `OSICFG` writer"]
pub type W = crate::W<OsicfgSpec>;
#[doc = "Field `EN` reader - Oscillator 8MHz enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Oscillator 8MHz enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL` reader - Oscillator 8MHz calibration value"]
pub type CalR = crate::FieldReader<u16>;
#[doc = "Field `CAL` writer - Oscillator 8MHz calibration value"]
pub type CalW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Oscillator 8MHz enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:25 - Oscillator 8MHz calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator 8MHz enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<OsicfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Oscillator 8MHz calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CalW<OsicfgSpec> {
        CalW::new(self, 16)
    }
}
#[doc = "Internal oscillator configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osicfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osicfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsicfgSpec;
impl crate::RegisterSpec for OsicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osicfg::R`](R) reader structure"]
impl crate::Readable for OsicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`osicfg::W`](W) writer structure"]
impl crate::Writable for OsicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSICFG to value 0"]
impl crate::Resettable for OsicfgSpec {
    const RESET_VALUE: u32 = 0;
}
