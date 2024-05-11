#[doc = "Register `USEBURSTCLR` writer"]
pub type W = crate::W<UseburstclrSpec>;
#[doc = "Field `CH0` writer - Disable single requests"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Disable single requests"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Disable single requests"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Disable single requests"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Disable single requests"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Disable single requests"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Disable single requests"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Disable single requests"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Disable single requests"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Disable single requests"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Disable single requests"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Disable single requests"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Disable single requests"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Disable single requests"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Disable single requests"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Disable single requests"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<UseburstclrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<UseburstclrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<UseburstclrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<UseburstclrSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<UseburstclrSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<UseburstclrSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<UseburstclrSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<UseburstclrSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<UseburstclrSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<UseburstclrSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<UseburstclrSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<UseburstclrSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<UseburstclrSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<UseburstclrSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<UseburstclrSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Disable single requests"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<UseburstclrSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Channel useburst clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useburstclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UseburstclrSpec;
impl crate::RegisterSpec for UseburstclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`useburstclr::W`](W) writer structure"]
impl crate::Writable for UseburstclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USEBURSTCLR to value 0"]
impl crate::Resettable for UseburstclrSpec {
    const RESET_VALUE: u32 = 0;
}
