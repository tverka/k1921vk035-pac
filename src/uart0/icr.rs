#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RXIC` writer - Receive interrupt clear"]
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIC` writer - Transmit interrupt clear"]
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - Receive timeout interrupt clear"]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIC` writer - Framing error interrupt clear"]
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIC` writer - Parity error interrupt clear"]
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIC` writer - Break error interrupt clear"]
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIC` writer - Overrun error interrupt clear"]
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIC` reader - Transmit done interrupt clear"]
pub type TdicR = crate::BitReader;
#[doc = "Field `TDIC` writer - Transmit done interrupt clear"]
pub type TdicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - Transmit done interrupt clear"]
    #[inline(always)]
    pub fn tdic(&self) -> TdicR {
        TdicR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Receive interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RxicW<IcrSpec> {
        RxicW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TxicW<IcrSpec> {
        TxicW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RticW<IcrSpec> {
        RticW::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FeicW<IcrSpec> {
        FeicW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PeicW<IcrSpec> {
        PeicW::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BeicW<IcrSpec> {
        BeicW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OeicW<IcrSpec> {
        OeicW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit done interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn tdic(&mut self) -> TdicW<IcrSpec> {
        TdicW::new(self, 11)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
