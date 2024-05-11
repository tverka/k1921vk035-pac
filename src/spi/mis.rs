#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RORRIS` reader - Masked interrupt status SSPRORINTR"]
pub type RorrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - Masked interrupt status SSPRTINTR"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RXRIS` reader - Masked interrupt status SSPRXINTR"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - Masked interrupt status SSPTXINTR"]
pub type TxrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked interrupt status SSPRORINTR"]
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt status SSPRTINTR"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt status SSPRXINTR"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt status SSPTXINTR"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status register interrupt masking account\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
