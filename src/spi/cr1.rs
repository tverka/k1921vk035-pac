#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `SSE` reader - Enable transceiver"]
pub type SseR = crate::BitReader;
#[doc = "Field `SSE` writer - Enable transceiver"]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - Select mode"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - Select mode"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOD` reader - Disable bit data"]
pub type SodR = crate::BitReader;
#[doc = "Field `SOD` writer - Disable bit data"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIFLSEL` reader - "]
pub type RxiflselR = crate::FieldReader;
#[doc = "Field `RXIFLSEL` writer - "]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXIFLSEL` reader - "]
pub type TxiflselR = crate::FieldReader;
#[doc = "Field `TXIFLSEL` writer - "]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1 - Enable transceiver"]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select mode"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable bit data"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable transceiver"]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SseW<Cr1Spec> {
        SseW::new(self, 1)
    }
    #[doc = "Bit 2 - Select mode"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<Cr1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable bit data"]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SodW<Cr1Spec> {
        SodW::new(self, 3)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RxiflselW<Cr1Spec> {
        RxiflselW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TxiflselW<Cr1Spec> {
        TxiflselW::new(self, 12)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
