#[doc = "Register `RSR` reader"]
pub type R = crate::R<RsrSpec>;
#[doc = "Register `RSR` writer"]
pub type W = crate::W<RsrSpec>;
#[doc = "Field `FE` reader - Framing error"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - Framing error"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Parity error"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Parity error"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Break error"]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - Break error"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - Overrun error"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - Overrun error"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Framing error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break error"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Framing error"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<RsrSpec> {
        FeW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<RsrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Break error"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<RsrSpec> {
        BeW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<RsrSpec> {
        OeW::new(self, 3)
    }
}
#[doc = "Receive Status Register/Error Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsrSpec;
impl crate::RegisterSpec for RsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rsr::W`](W) writer structure"]
impl crate::Writable for RsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RsrSpec {
    const RESET_VALUE: u32 = 0;
}
