#[doc = "Register `NCR` reader"]
pub type R = crate::R<NcrSpec>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NcrSpec>;
#[doc = "Field `INIT` reader - Node Initialization"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Node Initialization"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIE` reader - Transfer Interrupt Enable"]
pub type TrieR = crate::BitReader;
#[doc = "Field `TRIE` writer - Transfer Interrupt Enable"]
pub type TrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LECIE` reader - LEC Indicated Error Interrupt Enable"]
pub type LecieR = crate::BitReader;
#[doc = "Field `LECIE` writer - LEC Indicated Error Interrupt Enable"]
pub type LecieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIE` reader - Alert Interrupt Enable"]
pub type AlieR = crate::BitReader;
#[doc = "Field `ALIE` writer - Alert Interrupt Enable"]
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANDIS` reader - CAN Disable"]
pub type CandisR = crate::BitReader;
#[doc = "Field `CANDIS` writer - CAN Disable"]
pub type CandisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALM` reader - CAN Analyzer Mode"]
pub type CalmR = crate::BitReader;
#[doc = "Field `CALM` writer - CAN Analyzer Mode"]
pub type CalmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSEN` reader - Suspend Enable"]
pub type SusenR = crate::BitReader;
#[doc = "Field `SUSEN` writer - Suspend Enable"]
pub type SusenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&self) -> TrieR {
        TrieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    pub fn lecie(&self) -> LecieR {
        LecieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    pub fn candis(&self) -> CandisR {
        CandisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    pub fn calm(&self) -> CalmR {
        CalmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Suspend Enable"]
    #[inline(always)]
    pub fn susen(&self) -> SusenR {
        SusenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Node Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<NcrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trie(&mut self) -> TrieW<NcrSpec> {
        TrieW::new(self, 1)
    }
    #[doc = "Bit 2 - LEC Indicated Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LecieW<NcrSpec> {
        LecieW::new(self, 2)
    }
    #[doc = "Bit 3 - Alert Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> AlieW<NcrSpec> {
        AlieW::new(self, 3)
    }
    #[doc = "Bit 4 - CAN Disable"]
    #[inline(always)]
    #[must_use]
    pub fn candis(&mut self) -> CandisW<NcrSpec> {
        CandisW::new(self, 4)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CceW<NcrSpec> {
        CceW::new(self, 6)
    }
    #[doc = "Bit 7 - CAN Analyzer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CalmW<NcrSpec> {
        CalmW::new(self, 7)
    }
    #[doc = "Bit 8 - Suspend Enable"]
    #[inline(always)]
    #[must_use]
    pub fn susen(&mut self) -> SusenW<NcrSpec> {
        SusenW::new(self, 8)
    }
}
#[doc = "Node control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcrSpec;
impl crate::RegisterSpec for NcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NcrSpec {
    const RESET_VALUE: u32 = 0;
}
