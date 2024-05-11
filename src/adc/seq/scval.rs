#[doc = "Register `SCVAL` reader"]
pub type R = crate::R<ScvalSpec>;
#[doc = "Register `SCVAL` writer"]
pub type W = crate::W<ScvalSpec>;
#[doc = "Field `RCNT` reader - Current number of ADC restarts by sequencer"]
pub type RcntR = crate::FieldReader;
#[doc = "Field `ICNT` reader - Current number of ADC requests for interrupt generation"]
pub type IcntR = crate::FieldReader;
#[doc = "Field `ICLR` writer - Clear interrupt counter"]
pub type IclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Current number of ADC restarts by sequencer"]
    #[inline(always)]
    pub fn rcnt(&self) -> RcntR {
        RcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current number of ADC requests for interrupt generation"]
    #[inline(always)]
    pub fn icnt(&self) -> IcntR {
        IcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Clear interrupt counter"]
    #[inline(always)]
    #[must_use]
    pub fn iclr(&mut self) -> IclrW<ScvalSpec> {
        IclrW::new(self, 24)
    }
}
#[doc = "Sequencer ADC interrupt and restart counter current value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScvalSpec;
impl crate::RegisterSpec for ScvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scval::R`](R) reader structure"]
impl crate::Readable for ScvalSpec {}
#[doc = "`write(|w| ..)` method takes [`scval::W`](W) writer structure"]
impl crate::Writable for ScvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCVAL to value 0"]
impl crate::Resettable for ScvalSpec {
    const RESET_VALUE: u32 = 0;
}
