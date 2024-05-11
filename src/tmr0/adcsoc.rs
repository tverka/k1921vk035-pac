#[doc = "Register `ADCSOC` reader"]
pub type R = crate::R<AdcsocSpec>;
#[doc = "Register `ADCSOC` writer"]
pub type W = crate::W<AdcsocSpec>;
#[doc = "Field `EN` reader - "]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<AdcsocSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "ADC start of conversion register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsoc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsoc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcsocSpec;
impl crate::RegisterSpec for AdcsocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsoc::R`](R) reader structure"]
impl crate::Readable for AdcsocSpec {}
#[doc = "`write(|w| ..)` method takes [`adcsoc::W`](W) writer structure"]
impl crate::Writable for AdcsocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCSOC to value 0"]
impl crate::Resettable for AdcsocSpec {
    const RESET_VALUE: u32 = 0;
}
