#[doc = "Register `WAITONREQ` reader"]
pub type R = crate::R<WaitonreqSpec>;
#[doc = "Field `CH0` reader - Returns the status of the DMA request signals"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - Returns the status of the DMA request signals"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - Returns the status of the DMA request signals"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - Returns the status of the DMA request signals"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH4` reader - Returns the status of the DMA request signals"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH5` reader - Returns the status of the DMA request signals"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH6` reader - Returns the status of the DMA request signals"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH7` reader - Returns the status of the DMA request signals"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH8` reader - Returns the status of the DMA request signals"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH9` reader - Returns the status of the DMA request signals"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH10` reader - Returns the status of the DMA request signals"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH11` reader - Returns the status of the DMA request signals"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH12` reader - Returns the status of the DMA request signals"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH13` reader - Returns the status of the DMA request signals"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH14` reader - Returns the status of the DMA request signals"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH15` reader - Returns the status of the DMA request signals"]
pub type Ch15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Returns the status of the DMA request signals"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Channel wait on request status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waitonreq::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaitonreqSpec;
impl crate::RegisterSpec for WaitonreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waitonreq::R`](R) reader structure"]
impl crate::Readable for WaitonreqSpec {}
#[doc = "`reset()` method sets WAITONREQ to value 0"]
impl crate::Resettable for WaitonreqSpec {
    const RESET_VALUE: u32 = 0;
}
