#[doc = "Register `PANCTR` reader"]
pub type R = crate::R<PanctrSpec>;
#[doc = "Register `PANCTR` writer"]
pub type W = crate::W<PanctrSpec>;
#[doc = "Field `PANCMD` reader - Panel Command"]
pub type PancmdR = crate::FieldReader;
#[doc = "Field `PANCMD` writer - Panel Command"]
pub type PancmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSY` reader - Panel Busy Flag"]
pub type BusyR = crate::BitReader;
#[doc = "Field `RBUSY` reader - Result Busy Flag"]
pub type RbusyR = crate::BitReader;
#[doc = "Field `PANAR1` reader - Panel argument 1"]
pub type Panar1R = crate::FieldReader;
#[doc = "Field `PANAR1` writer - Panel argument 1"]
pub type Panar1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PANAR2` reader - Panel argument 2"]
pub type Panar2R = crate::FieldReader;
#[doc = "Field `PANAR2` writer - Panel argument 2"]
pub type Panar2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    pub fn pancmd(&self) -> PancmdR {
        PancmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Panel Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Result Busy Flag"]
    #[inline(always)]
    pub fn rbusy(&self) -> RbusyR {
        RbusyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Panel argument 1"]
    #[inline(always)]
    pub fn panar1(&self) -> Panar1R {
        Panar1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Panel argument 2"]
    #[inline(always)]
    pub fn panar2(&self) -> Panar2R {
        Panar2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Panel Command"]
    #[inline(always)]
    #[must_use]
    pub fn pancmd(&mut self) -> PancmdW<PanctrSpec> {
        PancmdW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Panel argument 1"]
    #[inline(always)]
    #[must_use]
    pub fn panar1(&mut self) -> Panar1W<PanctrSpec> {
        Panar1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Panel argument 2"]
    #[inline(always)]
    #[must_use]
    pub fn panar2(&mut self) -> Panar2W<PanctrSpec> {
        Panar2W::new(self, 24)
    }
}
#[doc = "Panel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`panctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`panctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PanctrSpec;
impl crate::RegisterSpec for PanctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`panctr::R`](R) reader structure"]
impl crate::Readable for PanctrSpec {}
#[doc = "`write(|w| ..)` method takes [`panctr::W`](W) writer structure"]
impl crate::Writable for PanctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PANCTR to value 0"]
impl crate::Resettable for PanctrSpec {
    const RESET_VALUE: u32 = 0;
}
