#[doc = "Register `PRSTCFG` reader"]
pub type R = crate::R<PrstcfgSpec>;
#[doc = "Register `PRSTCFG` writer"]
pub type W = crate::W<PrstcfgSpec>;
#[doc = "Field `TMR0EN` reader - Disable reset from TMR0"]
pub type Tmr0enR = crate::BitReader;
#[doc = "Field `TMR0EN` writer - Disable reset from TMR0"]
pub type Tmr0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1EN` reader - Disable reset from TMR1"]
pub type Tmr1enR = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Disable reset from TMR1"]
pub type Tmr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2EN` reader - Disable reset from TMR2"]
pub type Tmr2enR = crate::BitReader;
#[doc = "Field `TMR2EN` writer - Disable reset from TMR2"]
pub type Tmr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3EN` reader - Disable reset from TMR3"]
pub type Tmr3enR = crate::BitReader;
#[doc = "Field `TMR3EN` writer - Disable reset from TMR3"]
pub type Tmr3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0EN` reader - Disable reset from PWM0"]
pub type Pwm0enR = crate::BitReader;
#[doc = "Field `PWM0EN` writer - Disable reset from PWM0"]
pub type Pwm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1EN` reader - Disable reset from PWM1"]
pub type Pwm1enR = crate::BitReader;
#[doc = "Field `PWM1EN` writer - Disable reset from PWM1"]
pub type Pwm1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2EN` reader - Disable reset from PWM2"]
pub type Pwm2enR = crate::BitReader;
#[doc = "Field `PWM2EN` writer - Disable reset from PWM2"]
pub type Pwm2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CEN` reader - Disable reset from I2C"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - Disable reset from I2C"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QEPEN` reader - Disable reset from QEP"]
pub type QepenR = crate::BitReader;
#[doc = "Field `QEPEN` writer - Disable reset from QEP"]
pub type QepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP0EN` reader - Disable reset from ECAP0"]
pub type Ecap0enR = crate::BitReader;
#[doc = "Field `ECAP0EN` writer - Disable reset from ECAP0"]
pub type Ecap0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP1EN` reader - Disable reset from ECAP1"]
pub type Ecap1enR = crate::BitReader;
#[doc = "Field `ECAP1EN` writer - Disable reset from ECAP1"]
pub type Ecap1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP2EN` reader - Disable reset from ECAP2"]
pub type Ecap2enR = crate::BitReader;
#[doc = "Field `ECAP2EN` writer - Disable reset from ECAP2"]
pub type Ecap2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable reset from TMR0"]
    #[inline(always)]
    pub fn tmr0en(&self) -> Tmr0enR {
        Tmr0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable reset from TMR1"]
    #[inline(always)]
    pub fn tmr1en(&self) -> Tmr1enR {
        Tmr1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable reset from TMR2"]
    #[inline(always)]
    pub fn tmr2en(&self) -> Tmr2enR {
        Tmr2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable reset from TMR3"]
    #[inline(always)]
    pub fn tmr3en(&self) -> Tmr3enR {
        Tmr3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable reset from PWM0"]
    #[inline(always)]
    pub fn pwm0en(&self) -> Pwm0enR {
        Pwm0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable reset from PWM1"]
    #[inline(always)]
    pub fn pwm1en(&self) -> Pwm1enR {
        Pwm1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable reset from PWM2"]
    #[inline(always)]
    pub fn pwm2en(&self) -> Pwm2enR {
        Pwm2enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable reset from I2C"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable reset from QEP"]
    #[inline(always)]
    pub fn qepen(&self) -> QepenR {
        QepenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable reset from ECAP0"]
    #[inline(always)]
    pub fn ecap0en(&self) -> Ecap0enR {
        Ecap0enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable reset from ECAP1"]
    #[inline(always)]
    pub fn ecap1en(&self) -> Ecap1enR {
        Ecap1enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable reset from ECAP2"]
    #[inline(always)]
    pub fn ecap2en(&self) -> Ecap2enR {
        Ecap2enR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable reset from TMR0"]
    #[inline(always)]
    #[must_use]
    pub fn tmr0en(&mut self) -> Tmr0enW<PrstcfgSpec> {
        Tmr0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable reset from TMR1"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1en(&mut self) -> Tmr1enW<PrstcfgSpec> {
        Tmr1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable reset from TMR2"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2en(&mut self) -> Tmr2enW<PrstcfgSpec> {
        Tmr2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable reset from TMR3"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3en(&mut self) -> Tmr3enW<PrstcfgSpec> {
        Tmr3enW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable reset from PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0en(&mut self) -> Pwm0enW<PrstcfgSpec> {
        Pwm0enW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable reset from PWM1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1en(&mut self) -> Pwm1enW<PrstcfgSpec> {
        Pwm1enW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable reset from PWM2"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2en(&mut self) -> Pwm2enW<PrstcfgSpec> {
        Pwm2enW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable reset from I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2cenW<PrstcfgSpec> {
        I2cenW::new(self, 7)
    }
    #[doc = "Bit 8 - Disable reset from QEP"]
    #[inline(always)]
    #[must_use]
    pub fn qepen(&mut self) -> QepenW<PrstcfgSpec> {
        QepenW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable reset from ECAP0"]
    #[inline(always)]
    #[must_use]
    pub fn ecap0en(&mut self) -> Ecap0enW<PrstcfgSpec> {
        Ecap0enW::new(self, 9)
    }
    #[doc = "Bit 10 - Disable reset from ECAP1"]
    #[inline(always)]
    #[must_use]
    pub fn ecap1en(&mut self) -> Ecap1enW<PrstcfgSpec> {
        Ecap1enW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable reset from ECAP2"]
    #[inline(always)]
    #[must_use]
    pub fn ecap2en(&mut self) -> Ecap2enW<PrstcfgSpec> {
        Ecap2enW::new(self, 11)
    }
}
#[doc = "APB reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prstcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrstcfgSpec;
impl crate::RegisterSpec for PrstcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstcfg::R`](R) reader structure"]
impl crate::Readable for PrstcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`prstcfg::W`](W) writer structure"]
impl crate::Writable for PrstcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSTCFG to value 0"]
impl crate::Resettable for PrstcfgSpec {
    const RESET_VALUE: u32 = 0;
}
