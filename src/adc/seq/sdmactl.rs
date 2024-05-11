#[doc = "Register `SDMACTL` reader"]
pub type R = crate::R<SdmactlSpec>;
#[doc = "Register `SDMACTL` writer"]
pub type W = crate::W<SdmactlSpec>;
#[doc = "Field `DMAEN` reader - Enable DMA use"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - Enable DMA use"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FIFO load threshold for DMA request generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wmark {
    #[doc = "1: 1 measure for dma request"]
    Level1 = 1,
    #[doc = "2: 2 measures for dma request"]
    Level2 = 2,
    #[doc = "3: 4 measures for dma request"]
    Level4 = 3,
    #[doc = "4: 8 measures for dma request"]
    Level8 = 4,
    #[doc = "5: 16 measures for dma request"]
    Level16 = 5,
    #[doc = "6: 32 measures for dma request"]
    Level32 = 6,
}
impl From<Wmark> for u8 {
    #[inline(always)]
    fn from(variant: Wmark) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wmark {
    type Ux = u8;
}
impl crate::IsEnum for Wmark {}
#[doc = "Field `WMARK` reader - FIFO load threshold for DMA request generation"]
pub type WmarkR = crate::FieldReader<Wmark>;
impl WmarkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wmark> {
        match self.bits {
            1 => Some(Wmark::Level1),
            2 => Some(Wmark::Level2),
            3 => Some(Wmark::Level4),
            4 => Some(Wmark::Level8),
            5 => Some(Wmark::Level16),
            6 => Some(Wmark::Level32),
            _ => None,
        }
    }
    #[doc = "1 measure for dma request"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Wmark::Level1
    }
    #[doc = "2 measures for dma request"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Wmark::Level2
    }
    #[doc = "4 measures for dma request"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == Wmark::Level4
    }
    #[doc = "8 measures for dma request"]
    #[inline(always)]
    pub fn is_level8(&self) -> bool {
        *self == Wmark::Level8
    }
    #[doc = "16 measures for dma request"]
    #[inline(always)]
    pub fn is_level16(&self) -> bool {
        *self == Wmark::Level16
    }
    #[doc = "32 measures for dma request"]
    #[inline(always)]
    pub fn is_level32(&self) -> bool {
        *self == Wmark::Level32
    }
}
#[doc = "Field `WMARK` writer - FIFO load threshold for DMA request generation"]
pub type WmarkW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wmark>;
impl<'a, REG> WmarkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 measure for dma request"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(Wmark::Level1)
    }
    #[doc = "2 measures for dma request"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(Wmark::Level2)
    }
    #[doc = "4 measures for dma request"]
    #[inline(always)]
    pub fn level4(self) -> &'a mut crate::W<REG> {
        self.variant(Wmark::Level4)
    }
    #[doc = "8 measures for dma request"]
    #[inline(always)]
    pub fn level8(self) -> &'a mut crate::W<REG> {
        self.variant(Wmark::Level8)
    }
    #[doc = "16 measures for dma request"]
    #[inline(always)]
    pub fn level16(self) -> &'a mut crate::W<REG> {
        self.variant(Wmark::Level16)
    }
    #[doc = "32 measures for dma request"]
    #[inline(always)]
    pub fn level32(self) -> &'a mut crate::W<REG> {
        self.variant(Wmark::Level32)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DMA use"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - FIFO load threshold for DMA request generation"]
    #[inline(always)]
    pub fn wmark(&self) -> WmarkR {
        WmarkR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA use"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<SdmactlSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bits 8:10 - FIFO load threshold for DMA request generation"]
    #[inline(always)]
    #[must_use]
    pub fn wmark(&mut self) -> WmarkW<SdmactlSpec> {
        WmarkW::new(self, 8)
    }
}
#[doc = "Sequencer DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmactlSpec;
impl crate::RegisterSpec for SdmactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmactl::R`](R) reader structure"]
impl crate::Readable for SdmactlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmactl::W`](W) writer structure"]
impl crate::Writable for SdmactlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMACTL to value 0"]
impl crate::Resettable for SdmactlSpec {
    const RESET_VALUE: u32 = 0;
}
