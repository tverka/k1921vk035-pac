#[doc = "Register `PRIALTSET` reader"]
pub type R = crate::R<PrialtsetSpec>;
#[doc = "Register `PRIALTSET` writer"]
pub type W = crate::W<PrialtsetSpec>;
#[doc = "Field `CH0` reader - Set primary / alternate channel control data structure"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Set primary / alternate channel control data structure"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Set primary / alternate channel control data structure"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Set primary / alternate channel control data structure"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Set primary / alternate channel control data structure"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Set primary / alternate channel control data structure"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Set primary / alternate channel control data structure"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Set primary / alternate channel control data structure"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - Set primary / alternate channel control data structure"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH4` writer - Set primary / alternate channel control data structure"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - Set primary / alternate channel control data structure"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH5` writer - Set primary / alternate channel control data structure"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - Set primary / alternate channel control data structure"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH6` writer - Set primary / alternate channel control data structure"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - Set primary / alternate channel control data structure"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH7` writer - Set primary / alternate channel control data structure"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` reader - Set primary / alternate channel control data structure"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH8` writer - Set primary / alternate channel control data structure"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` reader - Set primary / alternate channel control data structure"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH9` writer - Set primary / alternate channel control data structure"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` reader - Set primary / alternate channel control data structure"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH10` writer - Set primary / alternate channel control data structure"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` reader - Set primary / alternate channel control data structure"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH11` writer - Set primary / alternate channel control data structure"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` reader - Set primary / alternate channel control data structure"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH12` writer - Set primary / alternate channel control data structure"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` reader - Set primary / alternate channel control data structure"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH13` writer - Set primary / alternate channel control data structure"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` reader - Set primary / alternate channel control data structure"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH14` writer - Set primary / alternate channel control data structure"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` reader - Set primary / alternate channel control data structure"]
pub type Ch15R = crate::BitReader;
#[doc = "Field `CH15` writer - Set primary / alternate channel control data structure"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<PrialtsetSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<PrialtsetSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<PrialtsetSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<PrialtsetSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<PrialtsetSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<PrialtsetSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<PrialtsetSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<PrialtsetSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<PrialtsetSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<PrialtsetSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<PrialtsetSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<PrialtsetSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<PrialtsetSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<PrialtsetSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<PrialtsetSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Set primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<PrialtsetSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Channel primary-alternate set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prialtset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prialtset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrialtsetSpec;
impl crate::RegisterSpec for PrialtsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prialtset::R`](R) reader structure"]
impl crate::Readable for PrialtsetSpec {}
#[doc = "`write(|w| ..)` method takes [`prialtset::W`](W) writer structure"]
impl crate::Writable for PrialtsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIALTSET to value 0"]
impl crate::Resettable for PrialtsetSpec {
    const RESET_VALUE: u32 = 0;
}
