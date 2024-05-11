#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RORRIS` reader - Interrupt status before masking SSPRORINTR"]
pub type RorrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - Interrupt status before masking SSPRTINTR"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RXRIS` reader - Interrupt status before masking SSPRXINTR"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - Interrupt status before masking SSPTXINTR"]
pub type TxrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt status before masking SSPRORINTR"]
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status before masking SSPRTINTR"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status before masking SSPRXINTR"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status before masking SSPTXINTR"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status register interrupt without mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
