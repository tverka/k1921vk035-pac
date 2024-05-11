#[doc = "Register `HRSTCFG` reader"]
pub type R = crate::R<HrstcfgSpec>;
#[doc = "Register `HRSTCFG` writer"]
pub type W = crate::W<HrstcfgSpec>;
#[doc = "Field `GPIOAEN` reader - Disable reset from GPIOA port"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - Disable reset from GPIOA port"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - Disable reset from GPIOB port"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - Disable reset from GPIOB port"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANEN` reader - Disable reset from CAN"]
pub type CanenR = crate::BitReader;
#[doc = "Field `CANEN` writer - Disable reset from CAN"]
pub type CanenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable reset from GPIOA port"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable reset from GPIOB port"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable reset from CAN"]
    #[inline(always)]
    pub fn canen(&self) -> CanenR {
        CanenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable reset from GPIOA port"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GpioaenW<HrstcfgSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable reset from GPIOB port"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GpiobenW<HrstcfgSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable reset from CAN"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CanenW<HrstcfgSpec> {
        CanenW::new(self, 2)
    }
}
#[doc = "AHB reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrstcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrstcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrstcfgSpec;
impl crate::RegisterSpec for HrstcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrstcfg::R`](R) reader structure"]
impl crate::Readable for HrstcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hrstcfg::W`](W) writer structure"]
impl crate::Writable for HrstcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRSTCFG to value 0"]
impl crate::Resettable for HrstcfgSpec {
    const RESET_VALUE: u32 = 0;
}
