#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "Field `RXMIS` reader - Receive masked interrupt status"]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - Transmit masked interrupt status"]
pub type TxmisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - Receive timeout masked interrupt status"]
pub type RtmisR = crate::BitReader;
#[doc = "Field `FEMIS` reader - Framing error masked interrupt status"]
pub type FemisR = crate::BitReader;
#[doc = "Field `PEMIS` reader - Parity error masked interrupt status"]
pub type PemisR = crate::BitReader;
#[doc = "Field `BEMIS` reader - Break error masked interrupt status"]
pub type BemisR = crate::BitReader;
#[doc = "Field `OEMIS` reader - Overrun error masked interrupt status"]
pub type OemisR = crate::BitReader;
#[doc = "Field `TDMIS` reader - Transmit done masked interrupt status"]
pub type TdmisR = crate::BitReader;
#[doc = "Field `TDMIS` writer - Transmit done masked interrupt status"]
pub type TdmisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Receive masked interrupt status"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit masked interrupt status"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout masked interrupt status"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error masked interrupt status"]
    #[inline(always)]
    pub fn femis(&self) -> FemisR {
        FemisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error masked interrupt status"]
    #[inline(always)]
    pub fn pemis(&self) -> PemisR {
        PemisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error masked interrupt status"]
    #[inline(always)]
    pub fn bemis(&self) -> BemisR {
        BemisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error masked interrupt status"]
    #[inline(always)]
    pub fn oemis(&self) -> OemisR {
        OemisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit done masked interrupt status"]
    #[inline(always)]
    pub fn tdmis(&self) -> TdmisR {
        TdmisR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Transmit done masked interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn tdmis(&mut self) -> TdmisW<MisSpec> {
        TdmisW::new(self, 11)
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
