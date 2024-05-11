#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "Field `RXRIS` reader - Receive interrupt status"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - Transmit interrupt status"]
pub type TxrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - Receive timeout interrupt status"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `FERIS` reader - Framing error interrupt status"]
pub type FerisR = crate::BitReader;
#[doc = "Field `PERIS` reader - Parity error interrupt status"]
pub type PerisR = crate::BitReader;
#[doc = "Field `BERIS` reader - Break error interrupt status"]
pub type BerisR = crate::BitReader;
#[doc = "Field `OERIS` reader - Overrun error interrupt status"]
pub type OerisR = crate::BitReader;
#[doc = "Field `TDRIS` reader - Transmit done raw interrupt status"]
pub type TdrisR = crate::BitReader;
#[doc = "Field `TDRIS` writer - Transmit done raw interrupt status"]
pub type TdrisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Receive interrupt status"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt status"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt status"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt status"]
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt status"]
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt status"]
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt status"]
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit done raw interrupt status"]
    #[inline(always)]
    pub fn tdris(&self) -> TdrisR {
        TdrisR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Transmit done raw interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn tdris(&mut self) -> TdrisW<RisSpec> {
        TdrisW::new(self, 11)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
