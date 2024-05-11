#[doc = "Register `CLKOUTCTL` reader"]
pub type R = crate::R<ClkoutctlSpec>;
#[doc = "Register `CLKOUTCTL` writer"]
pub type W = crate::W<ClkoutctlSpec>;
#[doc = "Field `CLKOUTEN` reader - Enable clockout pin"]
pub type ClkoutenR = crate::BitReader;
#[doc = "Field `CLKOUTEN` writer - Enable clockout pin"]
pub type ClkoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable clockout pin"]
    #[inline(always)]
    pub fn clkouten(&self) -> ClkoutenR {
        ClkoutenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable clockout pin"]
    #[inline(always)]
    #[must_use]
    pub fn clkouten(&mut self) -> ClkoutenW<ClkoutctlSpec> {
        ClkoutenW::new(self, 0)
    }
}
#[doc = "Clock out control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutctlSpec;
impl crate::RegisterSpec for ClkoutctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutctl::R`](R) reader structure"]
impl crate::Readable for ClkoutctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkoutctl::W`](W) writer structure"]
impl crate::Writable for ClkoutctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUTCTL to value 0"]
impl crate::Resettable for ClkoutctlSpec {
    const RESET_VALUE: u32 = 0;
}
