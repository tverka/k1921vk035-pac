#[doc = "Register `ECCLR` reader"]
pub type R = crate::R<EcclrSpec>;
#[doc = "Register `ECCLR` writer"]
pub type W = crate::W<EcclrSpec>;
#[doc = "Field `INT` reader - reset global interrupt"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - reset global interrupt"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT0` reader - reset intstatus"]
pub type Cevt0R = crate::BitReader;
#[doc = "Field `CEVT0` writer - reset intstatus"]
pub type Cevt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT1` reader - reset intstatus"]
pub type Cevt1R = crate::BitReader;
#[doc = "Field `CEVT1` writer - reset intstatus"]
pub type Cevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2` reader - reset intstatus"]
pub type Cevt2R = crate::BitReader;
#[doc = "Field `CEVT2` writer - reset intstatus"]
pub type Cevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3` reader - reset intstatus"]
pub type Cevt3R = crate::BitReader;
#[doc = "Field `CEVT3` writer - reset intstatus"]
pub type Cevt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTROVF` reader - reset intstatus"]
pub type CtrovfR = crate::BitReader;
#[doc = "Field `CTROVF` writer - reset intstatus"]
pub type CtrovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRPRD` reader - reset intstatus"]
pub type CtrprdR = crate::BitReader;
#[doc = "Field `CTRPRD` writer - reset intstatus"]
pub type CtrprdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRCMP` reader - reset intstatus"]
pub type CtrcmpR = crate::BitReader;
#[doc = "Field `CTRCMP` writer - reset intstatus"]
pub type CtrcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reset global interrupt"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reset intstatus"]
    #[inline(always)]
    pub fn cevt0(&self) -> Cevt0R {
        Cevt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reset intstatus"]
    #[inline(always)]
    pub fn cevt1(&self) -> Cevt1R {
        Cevt1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reset intstatus"]
    #[inline(always)]
    pub fn cevt2(&self) -> Cevt2R {
        Cevt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reset intstatus"]
    #[inline(always)]
    pub fn cevt3(&self) -> Cevt3R {
        Cevt3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reset intstatus"]
    #[inline(always)]
    pub fn ctrovf(&self) -> CtrovfR {
        CtrovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reset intstatus"]
    #[inline(always)]
    pub fn ctrprd(&self) -> CtrprdR {
        CtrprdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reset intstatus"]
    #[inline(always)]
    pub fn ctrcmp(&self) -> CtrcmpR {
        CtrcmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reset global interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<EcclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - reset intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn cevt0(&mut self) -> Cevt0W<EcclrSpec> {
        Cevt0W::new(self, 1)
    }
    #[doc = "Bit 2 - reset intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn cevt1(&mut self) -> Cevt1W<EcclrSpec> {
        Cevt1W::new(self, 2)
    }
    #[doc = "Bit 3 - reset intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn cevt2(&mut self) -> Cevt2W<EcclrSpec> {
        Cevt2W::new(self, 3)
    }
    #[doc = "Bit 4 - reset intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn cevt3(&mut self) -> Cevt3W<EcclrSpec> {
        Cevt3W::new(self, 4)
    }
    #[doc = "Bit 5 - reset intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn ctrovf(&mut self) -> CtrovfW<EcclrSpec> {
        CtrovfW::new(self, 5)
    }
    #[doc = "Bit 6 - reset intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn ctrprd(&mut self) -> CtrprdW<EcclrSpec> {
        CtrprdW::new(self, 6)
    }
    #[doc = "Bit 7 - reset intstatus"]
    #[inline(always)]
    #[must_use]
    pub fn ctrcmp(&mut self) -> CtrcmpW<EcclrSpec> {
        CtrcmpW::new(self, 7)
    }
}
#[doc = "Clear interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcclrSpec;
impl crate::RegisterSpec for EcclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecclr::R`](R) reader structure"]
impl crate::Readable for EcclrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecclr::W`](W) writer structure"]
impl crate::Writable for EcclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCLR to value 0"]
impl crate::Resettable for EcclrSpec {
    const RESET_VALUE: u32 = 0;
}
