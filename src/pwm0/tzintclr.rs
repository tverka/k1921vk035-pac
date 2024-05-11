#[doc = "Register `TZINTCLR` writer"]
pub type W = crate::W<TzintclrSpec>;
#[doc = "Field `INT` writer - Clear TZ interrupt pending"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TZ interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<TzintclrSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Trip-Zone Interrupt pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzintclrSpec;
impl crate::RegisterSpec for TzintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tzintclr::W`](W) writer structure"]
impl crate::Writable for TzintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZINTCLR to value 0"]
impl crate::Resettable for TzintclrSpec {
    const RESET_VALUE: u32 = 0;
}
