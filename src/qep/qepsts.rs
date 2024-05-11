#[doc = "Register `QEPSTS` reader"]
pub type R = crate::R<QepstsSpec>;
#[doc = "Register `QEPSTS` writer"]
pub type W = crate::W<QepstsSpec>;
#[doc = "Field `PCEF` reader - Position counter error flag"]
pub type PcefR = crate::BitReader;
#[doc = "Field `PCEF` writer - Position counter error flag"]
pub type PcefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIMF` reader - First index marker flag"]
pub type FimfR = crate::BitReader;
#[doc = "Field `FIMF` writer - First index marker flag"]
pub type FimfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEF` reader - Capture direction error flag"]
pub type CdefR = crate::BitReader;
#[doc = "Field `CDEF` writer - Capture direction error flag"]
pub type CdefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COEF` reader - Capture overflow error flag"]
pub type CoefR = crate::BitReader;
#[doc = "Field `COEF` writer - Capture overflow error flag"]
pub type CoefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDLF` reader - QEP direction latch flag"]
pub type QdlfR = crate::BitReader;
#[doc = "Field `QDLF` writer - QEP direction latch flag"]
pub type QdlfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDF` reader - Quadrature direction flag"]
pub type QdfR = crate::BitReader;
#[doc = "Field `QDF` writer - Quadrature direction flag"]
pub type QdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIDF` reader - Direction on the first index marker"]
pub type FidfR = crate::BitReader;
#[doc = "Field `FIDF` writer - Direction on the first index marker"]
pub type FidfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPEVNT` reader - Unit position event flag"]
pub type UpevntR = crate::BitReader;
#[doc = "Field `UPEVNT` writer - Unit position event flag"]
pub type UpevntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCF` reader - Direction change flag"]
pub type DcfR = crate::BitReader;
#[doc = "Field `DCF` writer - Direction change flag"]
pub type DcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Position counter error flag"]
    #[inline(always)]
    pub fn pcef(&self) -> PcefR {
        PcefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - First index marker flag"]
    #[inline(always)]
    pub fn fimf(&self) -> FimfR {
        FimfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture direction error flag"]
    #[inline(always)]
    pub fn cdef(&self) -> CdefR {
        CdefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture overflow error flag"]
    #[inline(always)]
    pub fn coef(&self) -> CoefR {
        CoefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - QEP direction latch flag"]
    #[inline(always)]
    pub fn qdlf(&self) -> QdlfR {
        QdlfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quadrature direction flag"]
    #[inline(always)]
    pub fn qdf(&self) -> QdfR {
        QdfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction on the first index marker"]
    #[inline(always)]
    pub fn fidf(&self) -> FidfR {
        FidfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unit position event flag"]
    #[inline(always)]
    pub fn upevnt(&self) -> UpevntR {
        UpevntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Direction change flag"]
    #[inline(always)]
    pub fn dcf(&self) -> DcfR {
        DcfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Position counter error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcef(&mut self) -> PcefW<QepstsSpec> {
        PcefW::new(self, 0)
    }
    #[doc = "Bit 1 - First index marker flag"]
    #[inline(always)]
    #[must_use]
    pub fn fimf(&mut self) -> FimfW<QepstsSpec> {
        FimfW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture direction error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cdef(&mut self) -> CdefW<QepstsSpec> {
        CdefW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture overflow error flag"]
    #[inline(always)]
    #[must_use]
    pub fn coef(&mut self) -> CoefW<QepstsSpec> {
        CoefW::new(self, 3)
    }
    #[doc = "Bit 4 - QEP direction latch flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdlf(&mut self) -> QdlfW<QepstsSpec> {
        QdlfW::new(self, 4)
    }
    #[doc = "Bit 5 - Quadrature direction flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdf(&mut self) -> QdfW<QepstsSpec> {
        QdfW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction on the first index marker"]
    #[inline(always)]
    #[must_use]
    pub fn fidf(&mut self) -> FidfW<QepstsSpec> {
        FidfW::new(self, 6)
    }
    #[doc = "Bit 7 - Unit position event flag"]
    #[inline(always)]
    #[must_use]
    pub fn upevnt(&mut self) -> UpevntW<QepstsSpec> {
        UpevntW::new(self, 7)
    }
    #[doc = "Bit 8 - Direction change flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcf(&mut self) -> DcfW<QepstsSpec> {
        DcfW::new(self, 8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qepsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qepsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QepstsSpec;
impl crate::RegisterSpec for QepstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qepsts::R`](R) reader structure"]
impl crate::Readable for QepstsSpec {}
#[doc = "`write(|w| ..)` method takes [`qepsts::W`](W) writer structure"]
impl crate::Writable for QepstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QEPSTS to value 0"]
impl crate::Resettable for QepstsSpec {
    const RESET_VALUE: u32 = 0;
}
