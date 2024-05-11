#[doc = "Register `ECFLG` reader"]
pub type R = crate::R<EcflgSpec>;
#[doc = "Field `INT` reader - indicate global interrupt"]
pub type IntR = crate::BitReader;
#[doc = "Field `CEVT0` reader - Hap interrupt CEVT0"]
pub type Cevt0R = crate::BitReader;
#[doc = "Field `CEVT1` reader - Hap interrupt CEVT1"]
pub type Cevt1R = crate::BitReader;
#[doc = "Field `CEVT2` reader - Hap interrupt CEVT2"]
pub type Cevt2R = crate::BitReader;
#[doc = "Field `CEVT3` reader - Hap interrupt CEVT3"]
pub type Cevt3R = crate::BitReader;
#[doc = "Field `CTROVF` reader - Hap interrupt CTROVF"]
pub type CtrovfR = crate::BitReader;
#[doc = "Field `CTRPRD` reader - Hap interrupt CTR=PRD"]
pub type CtrprdR = crate::BitReader;
#[doc = "Field `CTRCMP` reader - Hap interrupt CTR=CMP"]
pub type CtrcmpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - indicate global interrupt"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hap interrupt CEVT0"]
    #[inline(always)]
    pub fn cevt0(&self) -> Cevt0R {
        Cevt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hap interrupt CEVT1"]
    #[inline(always)]
    pub fn cevt1(&self) -> Cevt1R {
        Cevt1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hap interrupt CEVT2"]
    #[inline(always)]
    pub fn cevt2(&self) -> Cevt2R {
        Cevt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hap interrupt CEVT3"]
    #[inline(always)]
    pub fn cevt3(&self) -> Cevt3R {
        Cevt3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hap interrupt CTROVF"]
    #[inline(always)]
    pub fn ctrovf(&self) -> CtrovfR {
        CtrovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hap interrupt CTR=PRD"]
    #[inline(always)]
    pub fn ctrprd(&self) -> CtrprdR {
        CtrprdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hap interrupt CTR=CMP"]
    #[inline(always)]
    pub fn ctrcmp(&self) -> CtrcmpR {
        CtrcmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecflg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcflgSpec;
impl crate::RegisterSpec for EcflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecflg::R`](R) reader structure"]
impl crate::Readable for EcflgSpec {}
#[doc = "`reset()` method sets ECFLG to value 0"]
impl crate::Resettable for EcflgSpec {
    const RESET_VALUE: u32 = 0;
}
