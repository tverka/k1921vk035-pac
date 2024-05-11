#[doc = "Register `ENSET` reader"]
pub type R = crate::R<EnsetSpec>;
#[doc = "Register `ENSET` writer"]
pub type W = crate::W<EnsetSpec>;
#[doc = "Field `CH0` reader - Enable channel"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Enable channel"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Enable channel"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Enable channel"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Enable channel"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Enable channel"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Enable channel"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Enable channel"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - Enable channel"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH4` writer - Enable channel"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - Enable channel"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH5` writer - Enable channel"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - Enable channel"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH6` writer - Enable channel"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - Enable channel"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH7` writer - Enable channel"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` reader - Enable channel"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH8` writer - Enable channel"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` reader - Enable channel"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH9` writer - Enable channel"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` reader - Enable channel"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH10` writer - Enable channel"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` reader - Enable channel"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH11` writer - Enable channel"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` reader - Enable channel"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH12` writer - Enable channel"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` reader - Enable channel"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH13` writer - Enable channel"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` reader - Enable channel"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH14` writer - Enable channel"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` reader - Enable channel"]
pub type Ch15R = crate::BitReader;
#[doc = "Field `CH15` writer - Enable channel"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable channel"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable channel"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable channel"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable channel"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable channel"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable channel"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable channel"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable channel"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable channel"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable channel"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable channel"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable channel"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable channel"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable channel"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable channel"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable channel"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<EnsetSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<EnsetSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<EnsetSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<EnsetSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<EnsetSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<EnsetSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<EnsetSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<EnsetSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<EnsetSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<EnsetSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<EnsetSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<EnsetSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<EnsetSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<EnsetSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<EnsetSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<EnsetSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Channel enable set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnsetSpec;
impl crate::RegisterSpec for EnsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enset::R`](R) reader structure"]
impl crate::Readable for EnsetSpec {}
#[doc = "`write(|w| ..)` method takes [`enset::W`](W) writer structure"]
impl crate::Writable for EnsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENSET to value 0"]
impl crate::Resettable for EnsetSpec {
    const RESET_VALUE: u32 = 0;
}
