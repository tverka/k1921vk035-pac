#[doc = "Register `ETFLG` reader"]
pub type R = crate::R<EtflgSpec>;
#[doc = "Field `INT` reader - Latched PWM Interrupt (PWM_INT) status flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `SOCA` reader - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) status flag"]
pub type SocaR = crate::BitReader;
#[doc = "Field `SOCB` reader - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) status flag"]
pub type SocbR = crate::BitReader;
#[doc = "Field `DRQA` reader - Latched PWM DMA request A status flag"]
pub type DrqaR = crate::BitReader;
#[doc = "Field `DRQB` reader - Latched PWM DMA request B status flag"]
pub type DrqbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Latched PWM Interrupt (PWM_INT) status flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) status flag"]
    #[inline(always)]
    pub fn soca(&self) -> SocaR {
        SocaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) status flag"]
    #[inline(always)]
    pub fn socb(&self) -> SocbR {
        SocbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Latched PWM DMA request A status flag"]
    #[inline(always)]
    pub fn drqa(&self) -> DrqaR {
        DrqaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Latched PWM DMA request B status flag"]
    #[inline(always)]
    pub fn drqb(&self) -> DrqbR {
        DrqbR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Event-Trigger Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etflg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtflgSpec;
impl crate::RegisterSpec for EtflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etflg::R`](R) reader structure"]
impl crate::Readable for EtflgSpec {}
#[doc = "`reset()` method sets ETFLG to value 0"]
impl crate::Resettable for EtflgSpec {
    const RESET_VALUE: u32 = 0;
}
