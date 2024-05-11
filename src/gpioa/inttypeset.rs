#[doc = "Register `INTTYPESET` reader"]
pub type R = crate::R<InttypesetSpec>;
#[doc = "Register `INTTYPESET` writer"]
pub type W = crate::W<InttypesetSpec>;
#[doc = "Field `PIN0` reader - Interrupt type set for pin 0"]
pub type Pin0R = crate::BitReader;
#[doc = "Field `PIN0` writer - Interrupt type set for pin 0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN1` reader - Interrupt type set for pin 1"]
pub type Pin1R = crate::BitReader;
#[doc = "Field `PIN1` writer - Interrupt type set for pin 1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN2` reader - Interrupt type set for pin 2"]
pub type Pin2R = crate::BitReader;
#[doc = "Field `PIN2` writer - Interrupt type set for pin 2"]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN3` reader - Interrupt type set for pin 3"]
pub type Pin3R = crate::BitReader;
#[doc = "Field `PIN3` writer - Interrupt type set for pin 3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN4` reader - Interrupt type set for pin 4"]
pub type Pin4R = crate::BitReader;
#[doc = "Field `PIN4` writer - Interrupt type set for pin 4"]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN5` reader - Interrupt type set for pin 5"]
pub type Pin5R = crate::BitReader;
#[doc = "Field `PIN5` writer - Interrupt type set for pin 5"]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN6` reader - Interrupt type set for pin 6"]
pub type Pin6R = crate::BitReader;
#[doc = "Field `PIN6` writer - Interrupt type set for pin 6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN7` reader - Interrupt type set for pin 7"]
pub type Pin7R = crate::BitReader;
#[doc = "Field `PIN7` writer - Interrupt type set for pin 7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN8` reader - Interrupt type set for pin 8"]
pub type Pin8R = crate::BitReader;
#[doc = "Field `PIN8` writer - Interrupt type set for pin 8"]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN9` reader - Interrupt type set for pin 9"]
pub type Pin9R = crate::BitReader;
#[doc = "Field `PIN9` writer - Interrupt type set for pin 9"]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN10` reader - Interrupt type set for pin 10"]
pub type Pin10R = crate::BitReader;
#[doc = "Field `PIN10` writer - Interrupt type set for pin 10"]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN11` reader - Interrupt type set for pin 11"]
pub type Pin11R = crate::BitReader;
#[doc = "Field `PIN11` writer - Interrupt type set for pin 11"]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN12` reader - Interrupt type set for pin 12"]
pub type Pin12R = crate::BitReader;
#[doc = "Field `PIN12` writer - Interrupt type set for pin 12"]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN13` reader - Interrupt type set for pin 13"]
pub type Pin13R = crate::BitReader;
#[doc = "Field `PIN13` writer - Interrupt type set for pin 13"]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN14` reader - Interrupt type set for pin 14"]
pub type Pin14R = crate::BitReader;
#[doc = "Field `PIN14` writer - Interrupt type set for pin 14"]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN15` reader - Interrupt type set for pin 15"]
pub type Pin15R = crate::BitReader;
#[doc = "Field `PIN15` writer - Interrupt type set for pin 15"]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt type set for pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt type set for pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt type set for pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt type set for pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt type set for pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt type set for pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt type set for pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt type set for pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt type set for pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt type set for pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt type set for pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt type set for pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt type set for pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt type set for pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt type set for pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt type set for pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt type set for pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> Pin0W<InttypesetSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt type set for pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> Pin1W<InttypesetSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt type set for pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> Pin2W<InttypesetSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt type set for pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> Pin3W<InttypesetSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt type set for pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> Pin4W<InttypesetSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt type set for pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> Pin5W<InttypesetSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt type set for pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> Pin6W<InttypesetSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt type set for pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> Pin7W<InttypesetSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt type set for pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> Pin8W<InttypesetSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt type set for pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> Pin9W<InttypesetSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt type set for pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> Pin10W<InttypesetSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt type set for pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> Pin11W<InttypesetSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt type set for pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> Pin12W<InttypesetSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt type set for pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> Pin13W<InttypesetSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt type set for pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> Pin14W<InttypesetSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt type set for pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> Pin15W<InttypesetSpec> {
        Pin15W::new(self, 15)
    }
}
#[doc = "Interrupt type set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttypeset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttypeset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttypesetSpec;
impl crate::RegisterSpec for InttypesetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttypeset::R`](R) reader structure"]
impl crate::Readable for InttypesetSpec {}
#[doc = "`write(|w| ..)` method takes [`inttypeset::W`](W) writer structure"]
impl crate::Writable for InttypesetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTTYPESET to value 0"]
impl crate::Resettable for InttypesetSpec {
    const RESET_VALUE: u32 = 0;
}
