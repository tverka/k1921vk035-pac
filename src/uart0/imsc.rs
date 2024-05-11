#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RXIM` reader - Receive interrupt mask"]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - Receive interrupt mask"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - Transmit interrupt mask"]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - Transmit interrupt mask"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - Framing error interrupt mask"]
pub type FeimR = crate::BitReader;
#[doc = "Field `FEIM` writer - Framing error interrupt mask"]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - Parity error interrupt mask"]
pub type PeimR = crate::BitReader;
#[doc = "Field `PEIM` writer - Parity error interrupt mask"]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - Break error interrupt mask"]
pub type BeimR = crate::BitReader;
#[doc = "Field `BEIM` writer - Break error interrupt mask"]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - Overrun error interrupt mask"]
pub type OeimR = crate::BitReader;
#[doc = "Field `OEIM` writer - Overrun error interrupt mask"]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIM` reader - Transmit done interrupt mask"]
pub type TdimR = crate::BitReader;
#[doc = "Field `TDIM` writer - Transmit done interrupt mask"]
pub type TdimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit done interrupt mask"]
    #[inline(always)]
    pub fn tdim(&self) -> TdimR {
        TdimR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<ImscSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<ImscSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<ImscSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FeimW<ImscSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PeimW<ImscSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BeimW<ImscSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OeimW<ImscSpec> {
        OeimW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit done interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tdim(&mut self) -> TdimW<ImscSpec> {
        TdimW::new(self, 11)
    }
}
#[doc = "Interrupt Mask Set/Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for ImscSpec {
    const RESET_VALUE: u32 = 0;
}
