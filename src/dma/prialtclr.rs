#[doc = "Register `PRIALTCLR` writer"]
pub type W = crate::W<PrialtclrSpec>;
#[doc = "Field `CH0` writer - Clear primary / alternate channel control data structure"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Clear primary / alternate channel control data structure"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Clear primary / alternate channel control data structure"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Clear primary / alternate channel control data structure"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Clear primary / alternate channel control data structure"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Clear primary / alternate channel control data structure"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Clear primary / alternate channel control data structure"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Clear primary / alternate channel control data structure"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Clear primary / alternate channel control data structure"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Clear primary / alternate channel control data structure"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Clear primary / alternate channel control data structure"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Clear primary / alternate channel control data structure"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Clear primary / alternate channel control data structure"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Clear primary / alternate channel control data structure"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Clear primary / alternate channel control data structure"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Clear primary / alternate channel control data structure"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<PrialtclrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<PrialtclrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<PrialtclrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<PrialtclrSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<PrialtclrSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<PrialtclrSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<PrialtclrSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<PrialtclrSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<PrialtclrSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<PrialtclrSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<PrialtclrSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<PrialtclrSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<PrialtclrSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<PrialtclrSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<PrialtclrSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear primary / alternate channel control data structure"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<PrialtclrSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Channel primary-alternate clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prialtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrialtclrSpec;
impl crate::RegisterSpec for PrialtclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prialtclr::W`](W) writer structure"]
impl crate::Writable for PrialtclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIALTCLR to value 0"]
impl crate::Resettable for PrialtclrSpec {
    const RESET_VALUE: u32 = 0;
}
