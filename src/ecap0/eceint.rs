#[doc = "Register `ECEINT` reader"]
pub type R = crate::R<EceintSpec>;
#[doc = "Register `ECEINT` writer"]
pub type W = crate::W<EceintSpec>;
#[doc = "Field `CEVT0` reader - enable int CEVT0"]
pub type Cevt0R = crate::BitReader;
#[doc = "Field `CEVT0` writer - enable int CEVT0"]
pub type Cevt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT1` reader - enable int CEVT1"]
pub type Cevt1R = crate::BitReader;
#[doc = "Field `CEVT1` writer - enable int CEVT1"]
pub type Cevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2` reader - enable int CEVT2"]
pub type Cevt2R = crate::BitReader;
#[doc = "Field `CEVT2` writer - enable int CEVT2"]
pub type Cevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3` reader - enable int CEVT3"]
pub type Cevt3R = crate::BitReader;
#[doc = "Field `CEVT3` writer - enable int CEVT3"]
pub type Cevt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTROVF` reader - enable int CTR_OVF"]
pub type CtrovfR = crate::BitReader;
#[doc = "Field `CTROVF` writer - enable int CTR_OVF"]
pub type CtrovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRPRD` reader - enable int CTR=PRD"]
pub type CtrprdR = crate::BitReader;
#[doc = "Field `CTRPRD` writer - enable int CTR=PRD"]
pub type CtrprdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRCMP` reader - enable int CTR=CMP"]
pub type CtrcmpR = crate::BitReader;
#[doc = "Field `CTRCMP` writer - enable int CTR=CMP"]
pub type CtrcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - enable int CEVT0"]
    #[inline(always)]
    pub fn cevt0(&self) -> Cevt0R {
        Cevt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable int CEVT1"]
    #[inline(always)]
    pub fn cevt1(&self) -> Cevt1R {
        Cevt1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable int CEVT2"]
    #[inline(always)]
    pub fn cevt2(&self) -> Cevt2R {
        Cevt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable int CEVT3"]
    #[inline(always)]
    pub fn cevt3(&self) -> Cevt3R {
        Cevt3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable int CTR_OVF"]
    #[inline(always)]
    pub fn ctrovf(&self) -> CtrovfR {
        CtrovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable int CTR=PRD"]
    #[inline(always)]
    pub fn ctrprd(&self) -> CtrprdR {
        CtrprdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable int CTR=CMP"]
    #[inline(always)]
    pub fn ctrcmp(&self) -> CtrcmpR {
        CtrcmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - enable int CEVT0"]
    #[inline(always)]
    #[must_use]
    pub fn cevt0(&mut self) -> Cevt0W<EceintSpec> {
        Cevt0W::new(self, 1)
    }
    #[doc = "Bit 2 - enable int CEVT1"]
    #[inline(always)]
    #[must_use]
    pub fn cevt1(&mut self) -> Cevt1W<EceintSpec> {
        Cevt1W::new(self, 2)
    }
    #[doc = "Bit 3 - enable int CEVT2"]
    #[inline(always)]
    #[must_use]
    pub fn cevt2(&mut self) -> Cevt2W<EceintSpec> {
        Cevt2W::new(self, 3)
    }
    #[doc = "Bit 4 - enable int CEVT3"]
    #[inline(always)]
    #[must_use]
    pub fn cevt3(&mut self) -> Cevt3W<EceintSpec> {
        Cevt3W::new(self, 4)
    }
    #[doc = "Bit 5 - enable int CTR_OVF"]
    #[inline(always)]
    #[must_use]
    pub fn ctrovf(&mut self) -> CtrovfW<EceintSpec> {
        CtrovfW::new(self, 5)
    }
    #[doc = "Bit 6 - enable int CTR=PRD"]
    #[inline(always)]
    #[must_use]
    pub fn ctrprd(&mut self) -> CtrprdW<EceintSpec> {
        CtrprdW::new(self, 6)
    }
    #[doc = "Bit 7 - enable int CTR=CMP"]
    #[inline(always)]
    #[must_use]
    pub fn ctrcmp(&mut self) -> CtrcmpW<EceintSpec> {
        CtrcmpW::new(self, 7)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eceint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eceint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EceintSpec;
impl crate::RegisterSpec for EceintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eceint::R`](R) reader structure"]
impl crate::Readable for EceintSpec {}
#[doc = "`write(|w| ..)` method takes [`eceint::W`](W) writer structure"]
impl crate::Writable for EceintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECEINT to value 0"]
impl crate::Resettable for EceintSpec {
    const RESET_VALUE: u32 = 0;
}
