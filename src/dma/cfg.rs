#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `MASTEREN` writer - Enable DMA"]
pub type MasterenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHPROT` writer - Sets the AHB-Lite protection"]
pub type ChprotW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl W {
    #[doc = "Bit 0 - Enable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn masteren(&mut self) -> MasterenW<CfgSpec> {
        MasterenW::new(self, 0)
    }
    #[doc = "Bits 5:7 - Sets the AHB-Lite protection"]
    #[inline(always)]
    #[must_use]
    pub fn chprot(&mut self) -> ChprotW<CfgSpec> {
        ChprotW::new(self, 5)
    }
}
#[doc = "DMA configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
