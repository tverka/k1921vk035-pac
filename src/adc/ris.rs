#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `SEQRIS0` reader - Sequencer 0 raw interrupt status"]
pub type Seqris0R = crate::BitReader;
#[doc = "Field `SEQRIS1` reader - Sequencer 1 raw interrupt status"]
pub type Seqris1R = crate::BitReader;
#[doc = "Field `DCRIS0` reader - Raw interrupt status of Digital Comparator 0"]
pub type Dcris0R = crate::BitReader;
#[doc = "Field `DCRIS1` reader - Raw interrupt status of Digital Comparator 1"]
pub type Dcris1R = crate::BitReader;
#[doc = "Field `DCRIS2` reader - Raw interrupt status of Digital Comparator 2"]
pub type Dcris2R = crate::BitReader;
#[doc = "Field `DCRIS3` reader - Raw interrupt status of Digital Comparator 3"]
pub type Dcris3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sequencer 0 raw interrupt status"]
    #[inline(always)]
    pub fn seqris0(&self) -> Seqris0R {
        Seqris0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 raw interrupt status"]
    #[inline(always)]
    pub fn seqris1(&self) -> Seqris1R {
        Seqris1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status of Digital Comparator 0"]
    #[inline(always)]
    pub fn dcris0(&self) -> Dcris0R {
        Dcris0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status of Digital Comparator 1"]
    #[inline(always)]
    pub fn dcris1(&self) -> Dcris1R {
        Dcris1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status of Digital Comparator 2"]
    #[inline(always)]
    pub fn dcris2(&self) -> Dcris2R {
        Dcris2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status of Digital Comparator 3"]
    #[inline(always)]
    pub fn dcris3(&self) -> Dcris3R {
        Dcris3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
