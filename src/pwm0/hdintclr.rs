#[doc = "Register `HDINTCLR` writer"]
pub type W = crate::W<HdintclrSpec>;
#[doc = "Field `INT` writer - Clear HD interrupt pending"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear HD interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<HdintclrSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Hold Detector Interrupt pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdintclrSpec;
impl crate::RegisterSpec for HdintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdintclr::W`](W) writer structure"]
impl crate::Writable for HdintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDINTCLR to value 0"]
impl crate::Resettable for HdintclrSpec {
    const RESET_VALUE: u32 = 0;
}
