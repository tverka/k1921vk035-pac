#[doc = "Register `ECFRC` reader"]
pub type R = crate::R<EcfrcSpec>;
#[doc = "Register `ECFRC` writer"]
pub type W = crate::W<EcfrcSpec>;
#[doc = "Field `CEVT0` reader - gen test interrupt"]
pub type Cevt0R = crate::BitReader;
#[doc = "Field `CEVT0` writer - gen test interrupt"]
pub type Cevt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT1` reader - gen test interrupt"]
pub type Cevt1R = crate::BitReader;
#[doc = "Field `CEVT1` writer - gen test interrupt"]
pub type Cevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2` reader - gen test interrupt"]
pub type Cevt2R = crate::BitReader;
#[doc = "Field `CEVT2` writer - gen test interrupt"]
pub type Cevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3` reader - gen test interrupt"]
pub type Cevt3R = crate::BitReader;
#[doc = "Field `CEVT3` writer - gen test interrupt"]
pub type Cevt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTROVF` reader - gen test interrupt"]
pub type CtrovfR = crate::BitReader;
#[doc = "Field `CTROVF` writer - gen test interrupt"]
pub type CtrovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRPRD` reader - gen test interrupt"]
pub type CtrprdR = crate::BitReader;
#[doc = "Field `CTRPRD` writer - gen test interrupt"]
pub type CtrprdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRCMP` reader - gen test interrupt"]
pub type CtrcmpR = crate::BitReader;
#[doc = "Field `CTRCMP` writer - gen test interrupt"]
pub type CtrcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - gen test interrupt"]
    #[inline(always)]
    pub fn cevt0(&self) -> Cevt0R {
        Cevt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gen test interrupt"]
    #[inline(always)]
    pub fn cevt1(&self) -> Cevt1R {
        Cevt1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gen test interrupt"]
    #[inline(always)]
    pub fn cevt2(&self) -> Cevt2R {
        Cevt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - gen test interrupt"]
    #[inline(always)]
    pub fn cevt3(&self) -> Cevt3R {
        Cevt3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - gen test interrupt"]
    #[inline(always)]
    pub fn ctrovf(&self) -> CtrovfR {
        CtrovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - gen test interrupt"]
    #[inline(always)]
    pub fn ctrprd(&self) -> CtrprdR {
        CtrprdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - gen test interrupt"]
    #[inline(always)]
    pub fn ctrcmp(&self) -> CtrcmpR {
        CtrcmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - gen test interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cevt0(&mut self) -> Cevt0W<EcfrcSpec> {
        Cevt0W::new(self, 1)
    }
    #[doc = "Bit 2 - gen test interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cevt1(&mut self) -> Cevt1W<EcfrcSpec> {
        Cevt1W::new(self, 2)
    }
    #[doc = "Bit 3 - gen test interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cevt2(&mut self) -> Cevt2W<EcfrcSpec> {
        Cevt2W::new(self, 3)
    }
    #[doc = "Bit 4 - gen test interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cevt3(&mut self) -> Cevt3W<EcfrcSpec> {
        Cevt3W::new(self, 4)
    }
    #[doc = "Bit 5 - gen test interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ctrovf(&mut self) -> CtrovfW<EcfrcSpec> {
        CtrovfW::new(self, 5)
    }
    #[doc = "Bit 6 - gen test interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ctrprd(&mut self) -> CtrprdW<EcfrcSpec> {
        CtrprdW::new(self, 6)
    }
    #[doc = "Bit 7 - gen test interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ctrcmp(&mut self) -> CtrcmpW<EcfrcSpec> {
        CtrcmpW::new(self, 7)
    }
}
#[doc = "Force interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcfrcSpec;
impl crate::RegisterSpec for EcfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecfrc::R`](R) reader structure"]
impl crate::Readable for EcfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ecfrc::W`](W) writer structure"]
impl crate::Writable for EcfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECFRC to value 0"]
impl crate::Resettable for EcfrcSpec {
    const RESET_VALUE: u32 = 0;
}
