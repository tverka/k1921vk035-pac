#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ON` reader - Enable Timer"]
pub type OnR = crate::BitReader;
#[doc = "Field `ON` writer - Enable Timer"]
pub type OnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINEN` reader - Enable external input as ENABLE"]
pub type ExtinenR = crate::BitReader;
#[doc = "Field `EXTINEN` writer - Enable external input as ENABLE"]
pub type ExtinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINCLK` reader - Enable external input as CLK"]
pub type ExtinclkR = crate::BitReader;
#[doc = "Field `EXTINCLK` writer - Enable external input as CLK"]
pub type ExtinclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - Enable Timer interrupt"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Enable Timer interrupt"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn on(&self) -> OnR {
        OnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable external input as ENABLE"]
    #[inline(always)]
    pub fn extinen(&self) -> ExtinenR {
        ExtinenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable external input as CLK"]
    #[inline(always)]
    pub fn extinclk(&self) -> ExtinclkR {
        ExtinclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Timer interrupt"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    #[must_use]
    pub fn on(&mut self) -> OnW<CtrlSpec> {
        OnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable external input as ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn extinen(&mut self) -> ExtinenW<CtrlSpec> {
        ExtinenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable external input as CLK"]
    #[inline(always)]
    #[must_use]
    pub fn extinclk(&mut self) -> ExtinclkW<CtrlSpec> {
        ExtinclkW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<CtrlSpec> {
        IntenW::new(self, 3)
    }
}
#[doc = "Control Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
