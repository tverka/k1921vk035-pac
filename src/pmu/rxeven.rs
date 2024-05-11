#[doc = "Register `RXEVEN` reader"]
pub type R = crate::R<RxevenSpec>;
#[doc = "Register `RXEVEN` writer"]
pub type W = crate::W<RxevenSpec>;
#[doc = "Field `GPIOAEV` reader - Enable RX event from GPIOA pins"]
pub type GpioaevR = crate::BitReader;
#[doc = "Field `GPIOAEV` writer - Enable RX event from GPIOA pins"]
pub type GpioaevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEV` reader - Enable RX event from GPIOB pins"]
pub type GpiobevR = crate::BitReader;
#[doc = "Field `GPIOBEV` writer - Enable RX event from GPIOB pins"]
pub type GpiobevW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable RX event from GPIOA pins"]
    #[inline(always)]
    pub fn gpioaev(&self) -> GpioaevR {
        GpioaevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RX event from GPIOB pins"]
    #[inline(always)]
    pub fn gpiobev(&self) -> GpiobevR {
        GpiobevR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RX event from GPIOA pins"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaev(&mut self) -> GpioaevW<RxevenSpec> {
        GpioaevW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable RX event from GPIOB pins"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobev(&mut self) -> GpiobevW<RxevenSpec> {
        GpiobevW::new(self, 1)
    }
}
#[doc = "PMU RX Event generation enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxeven::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxeven::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxevenSpec;
impl crate::RegisterSpec for RxevenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxeven::R`](R) reader structure"]
impl crate::Readable for RxevenSpec {}
#[doc = "`write(|w| ..)` method takes [`rxeven::W`](W) writer structure"]
impl crate::Writable for RxevenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXEVEN to value 0"]
impl crate::Resettable for RxevenSpec {
    const RESET_VALUE: u32 = 0;
}
