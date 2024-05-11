#[doc = "Register `DMAREQ` reader"]
pub type R = crate::R<DmareqSpec>;
#[doc = "Register `DMAREQ` writer"]
pub type W = crate::W<DmareqSpec>;
#[doc = "Field `DMAEN` reader - DMA request enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA request enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA request enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<DmareqSpec> {
        DmaenW::new(self, 0)
    }
}
#[doc = "DMA request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmareqSpec;
impl crate::RegisterSpec for DmareqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmareq::R`](R) reader structure"]
impl crate::Readable for DmareqSpec {}
#[doc = "`write(|w| ..)` method takes [`dmareq::W`](W) writer structure"]
impl crate::Writable for DmareqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAREQ to value 0"]
impl crate::Resettable for DmareqSpec {
    const RESET_VALUE: u32 = 0;
}
