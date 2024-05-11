#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `S10ADR` reader - "]
pub type S10adrR = crate::FieldReader;
#[doc = "Field `S10ADR` writer - "]
pub type S10adrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `S10EN` reader - Enabled 10-bit slave address"]
pub type S10enR = crate::BitReader;
#[doc = "Field `S10EN` writer - Enabled 10-bit slave address"]
pub type S10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDIV` reader - "]
pub type HsdivR = crate::FieldReader;
#[doc = "Field `HSDIV` writer - "]
pub type HsdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn s10adr(&self) -> S10adrR {
        S10adrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enabled 10-bit slave address"]
    #[inline(always)]
    pub fn s10en(&self) -> S10enR {
        S10enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hsdiv(&self) -> HsdivR {
        HsdivR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn s10adr(&mut self) -> S10adrW<Ctl2Spec> {
        S10adrW::new(self, 0)
    }
    #[doc = "Bit 3 - Enabled 10-bit slave address"]
    #[inline(always)]
    #[must_use]
    pub fn s10en(&mut self) -> S10enW<Ctl2Spec> {
        S10enW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn hsdiv(&mut self) -> HsdivW<Ctl2Spec> {
        HsdivW::new(self, 4)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0;
}
