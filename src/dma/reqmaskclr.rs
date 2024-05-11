#[doc = "Register `REQMASKCLR` writer"]
pub type W = crate::W<ReqmaskclrSpec>;
#[doc = "Field `CH0` writer - External requests are disabled for channel"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - External requests are disabled for channel"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - External requests are disabled for channel"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - External requests are disabled for channel"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - External requests are disabled for channel"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - External requests are disabled for channel"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - External requests are disabled for channel"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - External requests are disabled for channel"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - External requests are disabled for channel"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - External requests are disabled for channel"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - External requests are disabled for channel"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - External requests are disabled for channel"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - External requests are disabled for channel"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - External requests are disabled for channel"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - External requests are disabled for channel"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - External requests are disabled for channel"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ReqmaskclrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ReqmaskclrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<ReqmaskclrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<ReqmaskclrSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<ReqmaskclrSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<ReqmaskclrSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<ReqmaskclrSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<ReqmaskclrSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<ReqmaskclrSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<ReqmaskclrSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<ReqmaskclrSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<ReqmaskclrSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<ReqmaskclrSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<ReqmaskclrSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<ReqmaskclrSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - External requests are disabled for channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<ReqmaskclrSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Channel request mask clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqmaskclrSpec;
impl crate::RegisterSpec for ReqmaskclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reqmaskclr::W`](W) writer structure"]
impl crate::Writable for ReqmaskclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQMASKCLR to value 0"]
impl crate::Resettable for ReqmaskclrSpec {
    const RESET_VALUE: u32 = 0;
}
