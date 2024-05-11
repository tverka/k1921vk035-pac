#[doc = "Register `PWMSYNC` reader"]
pub type R = crate::R<PwmsyncSpec>;
#[doc = "Register `PWMSYNC` writer"]
pub type W = crate::W<PwmsyncSpec>;
#[doc = "Field `PRESCRST` reader - PWM prescalers reset control"]
pub type PrescrstR = crate::FieldReader;
#[doc = "Field `PRESCRST` writer - PWM prescalers reset control"]
pub type PrescrstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:10 - PWM prescalers reset control"]
    #[inline(always)]
    pub fn prescrst(&self) -> PrescrstR {
        PrescrstR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - PWM prescalers reset control"]
    #[inline(always)]
    #[must_use]
    pub fn prescrst(&mut self) -> PrescrstW<PwmsyncSpec> {
        PrescrstW::new(self, 8)
    }
}
#[doc = "PWM syncronization control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwmsync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwmsync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmsyncSpec;
impl crate::RegisterSpec for PwmsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmsync::R`](R) reader structure"]
impl crate::Readable for PwmsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmsync::W`](W) writer structure"]
impl crate::Writable for PwmsyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMSYNC to value 0"]
impl crate::Resettable for PwmsyncSpec {
    const RESET_VALUE: u32 = 0;
}
