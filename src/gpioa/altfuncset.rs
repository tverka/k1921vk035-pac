#[doc = "Register `ALTFUNCSET` reader"]
pub type R = crate::R<AltfuncsetSpec>;
#[doc = "Register `ALTFUNCSET` writer"]
pub type W = crate::W<AltfuncsetSpec>;
#[doc = "Field `PIN0` reader - Alternative function enable for pin 0"]
pub type Pin0R = crate::BitReader;
#[doc = "Field `PIN0` writer - Alternative function enable for pin 0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN1` reader - Alternative function enable for pin 1"]
pub type Pin1R = crate::BitReader;
#[doc = "Field `PIN1` writer - Alternative function enable for pin 1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN2` reader - Alternative function enable for pin 2"]
pub type Pin2R = crate::BitReader;
#[doc = "Field `PIN2` writer - Alternative function enable for pin 2"]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN3` reader - Alternative function enable for pin 3"]
pub type Pin3R = crate::BitReader;
#[doc = "Field `PIN3` writer - Alternative function enable for pin 3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN4` reader - Alternative function enable for pin 4"]
pub type Pin4R = crate::BitReader;
#[doc = "Field `PIN4` writer - Alternative function enable for pin 4"]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN5` reader - Alternative function enable for pin 5"]
pub type Pin5R = crate::BitReader;
#[doc = "Field `PIN5` writer - Alternative function enable for pin 5"]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN6` reader - Alternative function enable for pin 6"]
pub type Pin6R = crate::BitReader;
#[doc = "Field `PIN6` writer - Alternative function enable for pin 6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN7` reader - Alternative function enable for pin 7"]
pub type Pin7R = crate::BitReader;
#[doc = "Field `PIN7` writer - Alternative function enable for pin 7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN8` reader - Alternative function enable for pin 8"]
pub type Pin8R = crate::BitReader;
#[doc = "Field `PIN8` writer - Alternative function enable for pin 8"]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN9` reader - Alternative function enable for pin 9"]
pub type Pin9R = crate::BitReader;
#[doc = "Field `PIN9` writer - Alternative function enable for pin 9"]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN10` reader - Alternative function enable for pin 10"]
pub type Pin10R = crate::BitReader;
#[doc = "Field `PIN10` writer - Alternative function enable for pin 10"]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN11` reader - Alternative function enable for pin 11"]
pub type Pin11R = crate::BitReader;
#[doc = "Field `PIN11` writer - Alternative function enable for pin 11"]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN12` reader - Alternative function enable for pin 12"]
pub type Pin12R = crate::BitReader;
#[doc = "Field `PIN12` writer - Alternative function enable for pin 12"]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN13` reader - Alternative function enable for pin 13"]
pub type Pin13R = crate::BitReader;
#[doc = "Field `PIN13` writer - Alternative function enable for pin 13"]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN14` reader - Alternative function enable for pin 14"]
pub type Pin14R = crate::BitReader;
#[doc = "Field `PIN14` writer - Alternative function enable for pin 14"]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN15` reader - Alternative function enable for pin 15"]
pub type Pin15R = crate::BitReader;
#[doc = "Field `PIN15` writer - Alternative function enable for pin 15"]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alternative function enable for pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alternative function enable for pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternative function enable for pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternative function enable for pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alternative function enable for pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alternative function enable for pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alternative function enable for pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Alternative function enable for pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alternative function enable for pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alternative function enable for pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Alternative function enable for pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Alternative function enable for pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alternative function enable for pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alternative function enable for pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Alternative function enable for pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternative function enable for pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternative function enable for pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> Pin0W<AltfuncsetSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Alternative function enable for pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> Pin1W<AltfuncsetSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Alternative function enable for pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> Pin2W<AltfuncsetSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Alternative function enable for pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> Pin3W<AltfuncsetSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Alternative function enable for pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> Pin4W<AltfuncsetSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Alternative function enable for pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> Pin5W<AltfuncsetSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Alternative function enable for pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> Pin6W<AltfuncsetSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Alternative function enable for pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> Pin7W<AltfuncsetSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Alternative function enable for pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> Pin8W<AltfuncsetSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Alternative function enable for pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> Pin9W<AltfuncsetSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Alternative function enable for pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> Pin10W<AltfuncsetSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Alternative function enable for pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> Pin11W<AltfuncsetSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Alternative function enable for pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> Pin12W<AltfuncsetSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Alternative function enable for pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> Pin13W<AltfuncsetSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Alternative function enable for pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> Pin14W<AltfuncsetSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Alternative function enable for pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> Pin15W<AltfuncsetSpec> {
        Pin15W::new(self, 15)
    }
}
#[doc = "Alternative function enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altfuncset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altfuncset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltfuncsetSpec;
impl crate::RegisterSpec for AltfuncsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altfuncset::R`](R) reader structure"]
impl crate::Readable for AltfuncsetSpec {}
#[doc = "`write(|w| ..)` method takes [`altfuncset::W`](W) writer structure"]
impl crate::Writable for AltfuncsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALTFUNCSET to value 0"]
impl crate::Resettable for AltfuncsetSpec {
    const RESET_VALUE: u32 = 0;
}