#[doc = "Register `ETFRC` reader"]
pub type R = crate::R<EtfrcSpec>;
#[doc = "Register `ETFRC` writer"]
pub type W = crate::W<EtfrcSpec>;
#[doc = "Field `INT` reader - PWM_INT force bit."]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - PWM_INT force bit."]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCA` reader - PWM_SOCA force bit"]
pub type SocaR = crate::BitReader;
#[doc = "Field `SOCA` writer - PWM_SOCA force bit"]
pub type SocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCB` reader - PWM_SOCB force bit"]
pub type SocbR = crate::BitReader;
#[doc = "Field `SOCB` writer - PWM_SOCB force bit"]
pub type SocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQA` reader - PWM DMA request A force bit"]
pub type DrqaR = crate::BitReader;
#[doc = "Field `DRQA` writer - PWM DMA request A force bit"]
pub type DrqaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQB` reader - PWM DMA request B force bit"]
pub type DrqbR = crate::BitReader;
#[doc = "Field `DRQB` writer - PWM DMA request B force bit"]
pub type DrqbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM_INT force bit."]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PWM_SOCA force bit"]
    #[inline(always)]
    pub fn soca(&self) -> SocaR {
        SocaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM_SOCB force bit"]
    #[inline(always)]
    pub fn socb(&self) -> SocbR {
        SocbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM DMA request A force bit"]
    #[inline(always)]
    pub fn drqa(&self) -> DrqaR {
        DrqaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM DMA request B force bit"]
    #[inline(always)]
    pub fn drqb(&self) -> DrqbR {
        DrqbR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM_INT force bit."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<EtfrcSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 2 - PWM_SOCA force bit"]
    #[inline(always)]
    #[must_use]
    pub fn soca(&mut self) -> SocaW<EtfrcSpec> {
        SocaW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM_SOCB force bit"]
    #[inline(always)]
    #[must_use]
    pub fn socb(&mut self) -> SocbW<EtfrcSpec> {
        SocbW::new(self, 3)
    }
    #[doc = "Bit 4 - PWM DMA request A force bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqa(&mut self) -> DrqaW<EtfrcSpec> {
        DrqaW::new(self, 4)
    }
    #[doc = "Bit 5 - PWM DMA request B force bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqb(&mut self) -> DrqbW<EtfrcSpec> {
        DrqbW::new(self, 5)
    }
}
#[doc = "Event-Trigger Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtfrcSpec;
impl crate::RegisterSpec for EtfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etfrc::R`](R) reader structure"]
impl crate::Readable for EtfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`etfrc::W`](W) writer structure"]
impl crate::Writable for EtfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETFRC to value 0"]
impl crate::Resettable for EtfrcSpec {
    const RESET_VALUE: u32 = 0;
}
