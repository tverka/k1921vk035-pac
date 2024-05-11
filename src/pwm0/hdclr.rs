#[doc = "Register `HDCLR` reader"]
pub type R = crate::R<HdclrSpec>;
#[doc = "Register `HDCLR` writer"]
pub type W = crate::W<HdclrSpec>;
#[doc = "Field `INT` reader - Clear hold detector interrupt flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Clear hold detector interrupt flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC` reader - Clear flag for Cycle-By-Cycle hold detector latch"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Clear flag for Cycle-By-Cycle hold detector latch"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - Clear flag for One-Shot hold detector latch"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - Clear flag for One-Shot hold detector latch"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear hold detector interrupt flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear flag for Cycle-By-Cycle hold detector latch"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear flag for One-Shot hold detector latch"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear hold detector interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<HdclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear flag for Cycle-By-Cycle hold detector latch"]
    #[inline(always)]
    #[must_use]
    pub fn cbc(&mut self) -> CbcW<HdclrSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear flag for One-Shot hold detector latch"]
    #[inline(always)]
    #[must_use]
    pub fn ost(&mut self) -> OstW<HdclrSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Register clear HD flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdclrSpec;
impl crate::RegisterSpec for HdclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdclr::R`](R) reader structure"]
impl crate::Readable for HdclrSpec {}
#[doc = "`write(|w| ..)` method takes [`hdclr::W`](W) writer structure"]
impl crate::Writable for HdclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDCLR to value 0"]
impl crate::Resettable for HdclrSpec {
    const RESET_VALUE: u32 = 0;
}
