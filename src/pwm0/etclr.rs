#[doc = "Register `ETCLR` reader"]
pub type R = crate::R<EtclrSpec>;
#[doc = "Register `ETCLR` writer"]
pub type W = crate::W<EtclrSpec>;
#[doc = "Field `INT` reader - Latched PWM Interrupt (PWM_INT) flag clear bit"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Latched PWM Interrupt (PWM_INT) flag clear bit"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCA` reader - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) flag clear bit"]
pub type SocaR = crate::BitReader;
#[doc = "Field `SOCA` writer - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) flag clear bit"]
pub type SocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCB` reader - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) flag clear bit"]
pub type SocbR = crate::BitReader;
#[doc = "Field `SOCB` writer - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) flag clear bit"]
pub type SocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQA` reader - Latched PWM DMA request A flag clear bit"]
pub type DrqaR = crate::BitReader;
#[doc = "Field `DRQA` writer - Latched PWM DMA request A flag clear bit"]
pub type DrqaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQB` reader - Latched PWM DMA request B flag clear bit"]
pub type DrqbR = crate::BitReader;
#[doc = "Field `DRQB` writer - Latched PWM DMA request B flag clear bit"]
pub type DrqbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Latched PWM Interrupt (PWM_INT) flag clear bit"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) flag clear bit"]
    #[inline(always)]
    pub fn soca(&self) -> SocaR {
        SocaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) flag clear bit"]
    #[inline(always)]
    pub fn socb(&self) -> SocbR {
        SocbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Latched PWM DMA request A flag clear bit"]
    #[inline(always)]
    pub fn drqa(&self) -> DrqaR {
        DrqaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Latched PWM DMA request B flag clear bit"]
    #[inline(always)]
    pub fn drqb(&self) -> DrqbR {
        DrqbR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latched PWM Interrupt (PWM_INT) flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<EtclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 2 - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn soca(&mut self) -> SocaW<EtclrSpec> {
        SocaW::new(self, 2)
    }
    #[doc = "Bit 3 - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn socb(&mut self) -> SocbW<EtclrSpec> {
        SocbW::new(self, 3)
    }
    #[doc = "Bit 4 - Latched PWM DMA request A flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqa(&mut self) -> DrqaW<EtclrSpec> {
        DrqaW::new(self, 4)
    }
    #[doc = "Bit 5 - Latched PWM DMA request B flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqb(&mut self) -> DrqbW<EtclrSpec> {
        DrqbW::new(self, 5)
    }
}
#[doc = "Event-Trigger Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtclrSpec;
impl crate::RegisterSpec for EtclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etclr::R`](R) reader structure"]
impl crate::Readable for EtclrSpec {}
#[doc = "`write(|w| ..)` method takes [`etclr::W`](W) writer structure"]
impl crate::Writable for EtclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETCLR to value 0"]
impl crate::Resettable for EtclrSpec {
    const RESET_VALUE: u32 = 0;
}
