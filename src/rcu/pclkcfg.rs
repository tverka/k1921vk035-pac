#[doc = "Register `PCLKCFG` reader"]
pub type R = crate::R<PclkcfgSpec>;
#[doc = "Register `PCLKCFG` writer"]
pub type W = crate::W<PclkcfgSpec>;
#[doc = "Field `TMR0EN` reader - Enable clock for TMR0"]
pub type Tmr0enR = crate::BitReader;
#[doc = "Field `TMR0EN` writer - Enable clock for TMR0"]
pub type Tmr0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1EN` reader - Enable clock for TMR1"]
pub type Tmr1enR = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Enable clock for TMR1"]
pub type Tmr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2EN` reader - Enable clock for TMR2"]
pub type Tmr2enR = crate::BitReader;
#[doc = "Field `TMR2EN` writer - Enable clock for TMR2"]
pub type Tmr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3EN` reader - Enable clock for TMR3"]
pub type Tmr3enR = crate::BitReader;
#[doc = "Field `TMR3EN` writer - Enable clock for TMR3"]
pub type Tmr3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0EN` reader - Enable clock for PWM0"]
pub type Pwm0enR = crate::BitReader;
#[doc = "Field `PWM0EN` writer - Enable clock for PWM0"]
pub type Pwm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1EN` reader - Enable clock for PWM1"]
pub type Pwm1enR = crate::BitReader;
#[doc = "Field `PWM1EN` writer - Enable clock for PWM1"]
pub type Pwm1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2EN` reader - Enable clock for PWM2"]
pub type Pwm2enR = crate::BitReader;
#[doc = "Field `PWM2EN` writer - Enable clock for PWM2"]
pub type Pwm2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CEN` reader - Enable clock for I2C"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - Enable clock for I2C"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QEPEN` reader - Enable clock for QEP"]
pub type QepenR = crate::BitReader;
#[doc = "Field `QEPEN` writer - Enable clock for QEP"]
pub type QepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP0EN` reader - Enable clock for ECAP0"]
pub type Ecap0enR = crate::BitReader;
#[doc = "Field `ECAP0EN` writer - Enable clock for ECAP0"]
pub type Ecap0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP1EN` reader - Enable clock for ECAP1"]
pub type Ecap1enR = crate::BitReader;
#[doc = "Field `ECAP1EN` writer - Enable clock for ECAP1"]
pub type Ecap1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP2EN` reader - Enable clock for ECAP2"]
pub type Ecap2enR = crate::BitReader;
#[doc = "Field `ECAP2EN` writer - Enable clock for ECAP2"]
pub type Ecap2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable clock for TMR0"]
    #[inline(always)]
    pub fn tmr0en(&self) -> Tmr0enR {
        Tmr0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable clock for TMR1"]
    #[inline(always)]
    pub fn tmr1en(&self) -> Tmr1enR {
        Tmr1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable clock for TMR2"]
    #[inline(always)]
    pub fn tmr2en(&self) -> Tmr2enR {
        Tmr2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable clock for TMR3"]
    #[inline(always)]
    pub fn tmr3en(&self) -> Tmr3enR {
        Tmr3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable clock for PWM0"]
    #[inline(always)]
    pub fn pwm0en(&self) -> Pwm0enR {
        Pwm0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable clock for PWM1"]
    #[inline(always)]
    pub fn pwm1en(&self) -> Pwm1enR {
        Pwm1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable clock for PWM2"]
    #[inline(always)]
    pub fn pwm2en(&self) -> Pwm2enR {
        Pwm2enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable clock for I2C"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable clock for QEP"]
    #[inline(always)]
    pub fn qepen(&self) -> QepenR {
        QepenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable clock for ECAP0"]
    #[inline(always)]
    pub fn ecap0en(&self) -> Ecap0enR {
        Ecap0enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable clock for ECAP1"]
    #[inline(always)]
    pub fn ecap1en(&self) -> Ecap1enR {
        Ecap1enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable clock for ECAP2"]
    #[inline(always)]
    pub fn ecap2en(&self) -> Ecap2enR {
        Ecap2enR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable clock for TMR0"]
    #[inline(always)]
    #[must_use]
    pub fn tmr0en(&mut self) -> Tmr0enW<PclkcfgSpec> {
        Tmr0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable clock for TMR1"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1en(&mut self) -> Tmr1enW<PclkcfgSpec> {
        Tmr1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable clock for TMR2"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2en(&mut self) -> Tmr2enW<PclkcfgSpec> {
        Tmr2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable clock for TMR3"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3en(&mut self) -> Tmr3enW<PclkcfgSpec> {
        Tmr3enW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable clock for PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0en(&mut self) -> Pwm0enW<PclkcfgSpec> {
        Pwm0enW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable clock for PWM1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1en(&mut self) -> Pwm1enW<PclkcfgSpec> {
        Pwm1enW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable clock for PWM2"]
    #[inline(always)]
    #[must_use]
    pub fn pwm2en(&mut self) -> Pwm2enW<PclkcfgSpec> {
        Pwm2enW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable clock for I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2cenW<PclkcfgSpec> {
        I2cenW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable clock for QEP"]
    #[inline(always)]
    #[must_use]
    pub fn qepen(&mut self) -> QepenW<PclkcfgSpec> {
        QepenW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable clock for ECAP0"]
    #[inline(always)]
    #[must_use]
    pub fn ecap0en(&mut self) -> Ecap0enW<PclkcfgSpec> {
        Ecap0enW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable clock for ECAP1"]
    #[inline(always)]
    #[must_use]
    pub fn ecap1en(&mut self) -> Ecap1enW<PclkcfgSpec> {
        Ecap1enW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable clock for ECAP2"]
    #[inline(always)]
    #[must_use]
    pub fn ecap2en(&mut self) -> Ecap2enW<PclkcfgSpec> {
        Ecap2enW::new(self, 11)
    }
}
#[doc = "APB clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PclkcfgSpec;
impl crate::RegisterSpec for PclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkcfg::R`](R) reader structure"]
impl crate::Readable for PclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pclkcfg::W`](W) writer structure"]
impl crate::Writable for PclkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKCFG to value 0"]
impl crate::Resettable for PclkcfgSpec {
    const RESET_VALUE: u32 = 0;
}
