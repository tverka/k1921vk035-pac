#[doc = "Register `CHCTL` reader"]
pub type R = crate::R<ChctlSpec>;
#[doc = "Register `CHCTL` writer"]
pub type W = crate::W<ChctlSpec>;
#[doc = "Field `OFFTRIM` reader - ADC channel offset trimm value"]
pub type OfftrimR = crate::FieldReader<u16>;
#[doc = "Field `OFFTRIM` writer - ADC channel offset trimm value"]
pub type OfftrimW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `GAINTRIM` reader - ADC channel gain trimm value"]
pub type GaintrimR = crate::FieldReader<u16>;
#[doc = "Field `GAINTRIM` writer - ADC channel gain trimm value"]
pub type GaintrimW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PRIORITY` reader - ADC channel priority level"]
pub type PriorityR = crate::BitReader;
#[doc = "Field `PRIORITY` writer - ADC channel priority level"]
pub type PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - ADC channel offset trimm value"]
    #[inline(always)]
    pub fn offtrim(&self) -> OfftrimR {
        OfftrimR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - ADC channel gain trimm value"]
    #[inline(always)]
    pub fn gaintrim(&self) -> GaintrimR {
        GaintrimR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 28 - ADC channel priority level"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - ADC channel offset trimm value"]
    #[inline(always)]
    #[must_use]
    pub fn offtrim(&mut self) -> OfftrimW<ChctlSpec> {
        OfftrimW::new(self, 0)
    }
    #[doc = "Bits 16:24 - ADC channel gain trimm value"]
    #[inline(always)]
    #[must_use]
    pub fn gaintrim(&mut self) -> GaintrimW<ChctlSpec> {
        GaintrimW::new(self, 16)
    }
    #[doc = "Bit 28 - ADC channel priority level"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<ChctlSpec> {
        PriorityW::new(self, 28)
    }
}
#[doc = "ADC channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctlSpec;
impl crate::RegisterSpec for ChctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl::R`](R) reader structure"]
impl crate::Readable for ChctlSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl::W`](W) writer structure"]
impl crate::Writable for ChctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCTL to value 0"]
impl crate::Resettable for ChctlSpec {
    const RESET_VALUE: u32 = 0;
}
