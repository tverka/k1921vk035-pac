#[doc = "Register `DCTRIG` reader"]
pub type R = crate::R<DctrigSpec>;
#[doc = "Register `DCTRIG` writer"]
pub type W = crate::W<DctrigSpec>;
#[doc = "Field `TOS0` reader - DC 0 output trigger status"]
pub type Tos0R = crate::BitReader;
#[doc = "Field `TOS0` writer - DC 0 output trigger status"]
pub type Tos0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS1` reader - DC 1 output trigger status"]
pub type Tos1R = crate::BitReader;
#[doc = "Field `TOS1` writer - DC 1 output trigger status"]
pub type Tos1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS2` reader - DC 2 output trigger status"]
pub type Tos2R = crate::BitReader;
#[doc = "Field `TOS2` writer - DC 2 output trigger status"]
pub type Tos2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS3` reader - DC 3 output trigger status"]
pub type Tos3R = crate::BitReader;
#[doc = "Field `TOS3` writer - DC 3 output trigger status"]
pub type Tos3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV0` reader - Digital compare event 0"]
pub type Dcev0R = crate::BitReader;
#[doc = "Field `DCEV0` writer - Digital compare event 0"]
pub type Dcev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV1` reader - Digital compare event 1"]
pub type Dcev1R = crate::BitReader;
#[doc = "Field `DCEV1` writer - Digital compare event 1"]
pub type Dcev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV2` reader - Digital compare event 2"]
pub type Dcev2R = crate::BitReader;
#[doc = "Field `DCEV2` writer - Digital compare event 2"]
pub type Dcev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV3` reader - Digital compare event 3"]
pub type Dcev3R = crate::BitReader;
#[doc = "Field `DCEV3` writer - Digital compare event 3"]
pub type Dcev3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DC 0 output trigger status"]
    #[inline(always)]
    pub fn tos0(&self) -> Tos0R {
        Tos0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DC 1 output trigger status"]
    #[inline(always)]
    pub fn tos1(&self) -> Tos1R {
        Tos1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DC 2 output trigger status"]
    #[inline(always)]
    pub fn tos2(&self) -> Tos2R {
        Tos2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DC 3 output trigger status"]
    #[inline(always)]
    pub fn tos3(&self) -> Tos3R {
        Tos3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital compare event 0"]
    #[inline(always)]
    pub fn dcev0(&self) -> Dcev0R {
        Dcev0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Digital compare event 1"]
    #[inline(always)]
    pub fn dcev1(&self) -> Dcev1R {
        Dcev1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Digital compare event 2"]
    #[inline(always)]
    pub fn dcev2(&self) -> Dcev2R {
        Dcev2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital compare event 3"]
    #[inline(always)]
    pub fn dcev3(&self) -> Dcev3R {
        Dcev3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DC 0 output trigger status"]
    #[inline(always)]
    #[must_use]
    pub fn tos0(&mut self) -> Tos0W<DctrigSpec> {
        Tos0W::new(self, 0)
    }
    #[doc = "Bit 1 - DC 1 output trigger status"]
    #[inline(always)]
    #[must_use]
    pub fn tos1(&mut self) -> Tos1W<DctrigSpec> {
        Tos1W::new(self, 1)
    }
    #[doc = "Bit 2 - DC 2 output trigger status"]
    #[inline(always)]
    #[must_use]
    pub fn tos2(&mut self) -> Tos2W<DctrigSpec> {
        Tos2W::new(self, 2)
    }
    #[doc = "Bit 3 - DC 3 output trigger status"]
    #[inline(always)]
    #[must_use]
    pub fn tos3(&mut self) -> Tos3W<DctrigSpec> {
        Tos3W::new(self, 3)
    }
    #[doc = "Bit 16 - Digital compare event 0"]
    #[inline(always)]
    #[must_use]
    pub fn dcev0(&mut self) -> Dcev0W<DctrigSpec> {
        Dcev0W::new(self, 16)
    }
    #[doc = "Bit 17 - Digital compare event 1"]
    #[inline(always)]
    #[must_use]
    pub fn dcev1(&mut self) -> Dcev1W<DctrigSpec> {
        Dcev1W::new(self, 17)
    }
    #[doc = "Bit 18 - Digital compare event 2"]
    #[inline(always)]
    #[must_use]
    pub fn dcev2(&mut self) -> Dcev2W<DctrigSpec> {
        Dcev2W::new(self, 18)
    }
    #[doc = "Bit 19 - Digital compare event 3"]
    #[inline(always)]
    #[must_use]
    pub fn dcev3(&mut self) -> Dcev3W<DctrigSpec> {
        Dcev3W::new(self, 19)
    }
}
#[doc = "Digital comparator output trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctrigSpec;
impl crate::RegisterSpec for DctrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctrig::R`](R) reader structure"]
impl crate::Readable for DctrigSpec {}
#[doc = "`write(|w| ..)` method takes [`dctrig::W`](W) writer structure"]
impl crate::Writable for DctrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTRIG to value 0"]
impl crate::Resettable for DctrigSpec {
    const RESET_VALUE: u32 = 0;
}
