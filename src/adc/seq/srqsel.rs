#[doc = "Register `SRQSEL` reader"]
pub type R = crate::R<SrqselSpec>;
#[doc = "Register `SRQSEL` writer"]
pub type W = crate::W<SrqselSpec>;
#[doc = "Field `RQ0` reader - Select ADC channel for request 0"]
pub type Rq0R = crate::FieldReader;
#[doc = "Field `RQ0` writer - Select ADC channel for request 0"]
pub type Rq0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RQ1` reader - Select ADC channel for request 1"]
pub type Rq1R = crate::FieldReader;
#[doc = "Field `RQ1` writer - Select ADC channel for request 1"]
pub type Rq1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RQ2` reader - Select ADC channel for request 2"]
pub type Rq2R = crate::FieldReader;
#[doc = "Field `RQ2` writer - Select ADC channel for request 2"]
pub type Rq2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RQ3` reader - Select ADC channel for request 3"]
pub type Rq3R = crate::FieldReader;
#[doc = "Field `RQ3` writer - Select ADC channel for request 3"]
pub type Rq3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Select ADC channel for request 0"]
    #[inline(always)]
    pub fn rq0(&self) -> Rq0R {
        Rq0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select ADC channel for request 1"]
    #[inline(always)]
    pub fn rq1(&self) -> Rq1R {
        Rq1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Select ADC channel for request 2"]
    #[inline(always)]
    pub fn rq2(&self) -> Rq2R {
        Rq2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Select ADC channel for request 3"]
    #[inline(always)]
    pub fn rq3(&self) -> Rq3R {
        Rq3R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select ADC channel for request 0"]
    #[inline(always)]
    #[must_use]
    pub fn rq0(&mut self) -> Rq0W<SrqselSpec> {
        Rq0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Select ADC channel for request 1"]
    #[inline(always)]
    #[must_use]
    pub fn rq1(&mut self) -> Rq1W<SrqselSpec> {
        Rq1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Select ADC channel for request 2"]
    #[inline(always)]
    #[must_use]
    pub fn rq2(&mut self) -> Rq2W<SrqselSpec> {
        Rq2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Select ADC channel for request 3"]
    #[inline(always)]
    #[must_use]
    pub fn rq3(&mut self) -> Rq3W<SrqselSpec> {
        Rq3W::new(self, 12)
    }
}
#[doc = "Sequencer request ADC channels selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srqsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrqselSpec;
impl crate::RegisterSpec for SrqselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqsel::R`](R) reader structure"]
impl crate::Readable for SrqselSpec {}
#[doc = "`write(|w| ..)` method takes [`srqsel::W`](W) writer structure"]
impl crate::Writable for SrqselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRQSEL to value 0"]
impl crate::Resettable for SrqselSpec {
    const RESET_VALUE: u32 = 0;
}
