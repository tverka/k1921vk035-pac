#[doc = "Register `NPCR` reader"]
pub type R = crate::R<NpcrSpec>;
#[doc = "Register `NPCR` writer"]
pub type W = crate::W<NpcrSpec>;
#[doc = "Field `RXSEL` reader - Receive Select"]
pub type RxselR = crate::FieldReader;
#[doc = "Field `RXSEL` writer - Receive Select"]
pub type RxselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LBM` reader - Loop-Back Mode"]
pub type LbmR = crate::BitReader;
#[doc = "Field `LBM` writer - Loop-Back Mode"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&self) -> RxselR {
        RxselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RxselW<NpcrSpec> {
        RxselW::new(self, 0)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LbmW<NpcrSpec> {
        LbmW::new(self, 8)
    }
}
#[doc = "Node Port Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NpcrSpec;
impl crate::RegisterSpec for NpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npcr::R`](R) reader structure"]
impl crate::Readable for NpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`npcr::W`](W) writer structure"]
impl crate::Writable for NpcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NPCR to value 0"]
impl crate::Resettable for NpcrSpec {
    const RESET_VALUE: u32 = 0;
}
