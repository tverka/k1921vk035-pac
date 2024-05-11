#[doc = "Register `SWREQ` writer"]
pub type W = crate::W<SwreqSpec>;
#[doc = "Field `CH0` writer - Set software request on channel"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Set software request on channel"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Set software request on channel"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Set software request on channel"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Set software request on channel"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Set software request on channel"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Set software request on channel"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Set software request on channel"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Set software request on channel"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Set software request on channel"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Set software request on channel"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Set software request on channel"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Set software request on channel"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Set software request on channel"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Set software request on channel"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Set software request on channel"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<SwreqSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<SwreqSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<SwreqSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<SwreqSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<SwreqSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<SwreqSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<SwreqSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<SwreqSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<SwreqSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<SwreqSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<SwreqSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<SwreqSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<SwreqSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<SwreqSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<SwreqSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Set software request on channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<SwreqSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Channel software request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwreqSpec;
impl crate::RegisterSpec for SwreqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreq::W`](W) writer structure"]
impl crate::Writable for SwreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SwreqSpec {
    const RESET_VALUE: u32 = 0;
}
