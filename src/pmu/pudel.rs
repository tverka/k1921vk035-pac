#[doc = "Register `PUDEL` reader"]
pub type R = crate::R<PudelSpec>;
#[doc = "Register `PUDEL` writer"]
pub type W = crate::W<PudelSpec>;
#[doc = "Field `VAL` reader - Delay value for powerup peripheral blocks (in OSICLK ticks)"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Delay value for powerup peripheral blocks (in OSICLK ticks)"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Delay value for powerup peripheral blocks (in OSICLK ticks)"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delay value for powerup peripheral blocks (in OSICLK ticks)"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<PudelSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "PMU Powerup Delay Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pudel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pudel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PudelSpec;
impl crate::RegisterSpec for PudelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pudel::R`](R) reader structure"]
impl crate::Readable for PudelSpec {}
#[doc = "`write(|w| ..)` method takes [`pudel::W`](W) writer structure"]
impl crate::Writable for PudelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUDEL to value 0"]
impl crate::Resettable for PudelSpec {
    const RESET_VALUE: u32 = 0;
}
