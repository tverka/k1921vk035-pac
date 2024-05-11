#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `SEQMIS0` reader - Sequencer 0 masked interrupt status"]
pub type Seqmis0R = crate::BitReader;
#[doc = "Field `SEQMIS1` reader - Sequencer 1 masked interrupt status"]
pub type Seqmis1R = crate::BitReader;
#[doc = "Field `DCMIS0` reader - DC 0 masked interrupt status"]
pub type Dcmis0R = crate::BitReader;
#[doc = "Field `DCMIS1` reader - DC 1 masked interrupt status"]
pub type Dcmis1R = crate::BitReader;
#[doc = "Field `DCMIS2` reader - DC 2 masked interrupt status"]
pub type Dcmis2R = crate::BitReader;
#[doc = "Field `DCMIS3` reader - DC 3 masked interrupt status"]
pub type Dcmis3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sequencer 0 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis0(&self) -> Seqmis0R {
        Seqmis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis1(&self) -> Seqmis1R {
        Seqmis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DC 0 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis0(&self) -> Dcmis0R {
        Dcmis0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DC 1 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis1(&self) -> Dcmis1R {
        Dcmis1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DC 2 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis2(&self) -> Dcmis2R {
        Dcmis2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DC 3 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis3(&self) -> Dcmis3R {
        Dcmis3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Masked interrupt status and clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
