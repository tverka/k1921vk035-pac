#[doc = "Register `SDC` reader"]
pub type R = crate::R<SdcSpec>;
#[doc = "Register `SDC` writer"]
pub type W = crate::W<SdcSpec>;
#[doc = "Field `DC0` reader - Enable DC 0"]
pub type Dc0R = crate::BitReader;
#[doc = "Field `DC0` writer - Enable DC 0"]
pub type Dc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC1` reader - Enable DC 1"]
pub type Dc1R = crate::BitReader;
#[doc = "Field `DC1` writer - Enable DC 1"]
pub type Dc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC2` reader - Enable DC 2"]
pub type Dc2R = crate::BitReader;
#[doc = "Field `DC2` writer - Enable DC 2"]
pub type Dc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC3` reader - Enable DC 3"]
pub type Dc3R = crate::BitReader;
#[doc = "Field `DC3` writer - Enable DC 3"]
pub type Dc3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable DC 0"]
    #[inline(always)]
    pub fn dc0(&self) -> Dc0R {
        Dc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DC 1"]
    #[inline(always)]
    pub fn dc1(&self) -> Dc1R {
        Dc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable DC 2"]
    #[inline(always)]
    pub fn dc2(&self) -> Dc2R {
        Dc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DC 3"]
    #[inline(always)]
    pub fn dc3(&self) -> Dc3R {
        Dc3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DC 0"]
    #[inline(always)]
    #[must_use]
    pub fn dc0(&mut self) -> Dc0W<SdcSpec> {
        Dc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable DC 1"]
    #[inline(always)]
    #[must_use]
    pub fn dc1(&mut self) -> Dc1W<SdcSpec> {
        Dc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable DC 2"]
    #[inline(always)]
    #[must_use]
    pub fn dc2(&mut self) -> Dc2W<SdcSpec> {
        Dc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable DC 3"]
    #[inline(always)]
    #[must_use]
    pub fn dc3(&mut self) -> Dc3W<SdcSpec> {
        Dc3W::new(self, 3)
    }
}
#[doc = "Sequencer digital comparator selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdcSpec;
impl crate::RegisterSpec for SdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdc::R`](R) reader structure"]
impl crate::Readable for SdcSpec {}
#[doc = "`write(|w| ..)` method takes [`sdc::W`](W) writer structure"]
impl crate::Writable for SdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDC to value 0"]
impl crate::Resettable for SdcSpec {
    const RESET_VALUE: u32 = 0;
}
