#[doc = "Register `HCLKCFG` reader"]
pub type R = crate::R<HclkcfgSpec>;
#[doc = "Register `HCLKCFG` writer"]
pub type W = crate::W<HclkcfgSpec>;
#[doc = "Field `GPIOAEN` reader - Enable clock for GPIOA port"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - Enable clock for GPIOA port"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - Enable clock for GPIOB port"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - Enable clock for GPIOB port"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANEN` reader - Enable clock for CAN"]
pub type CanenR = crate::BitReader;
#[doc = "Field `CANEN` writer - Enable clock for CAN"]
pub type CanenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable clock for GPIOA port"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable clock for GPIOB port"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable clock for CAN"]
    #[inline(always)]
    pub fn canen(&self) -> CanenR {
        CanenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable clock for GPIOA port"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GpioaenW<HclkcfgSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable clock for GPIOB port"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GpiobenW<HclkcfgSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable clock for CAN"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CanenW<HclkcfgSpec> {
        CanenW::new(self, 2)
    }
}
#[doc = "AHB clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hclkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hclkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HclkcfgSpec;
impl crate::RegisterSpec for HclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hclkcfg::R`](R) reader structure"]
impl crate::Readable for HclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hclkcfg::W`](W) writer structure"]
impl crate::Writable for HclkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCLKCFG to value 0"]
impl crate::Resettable for HclkcfgSpec {
    const RESET_VALUE: u32 = 0;
}
