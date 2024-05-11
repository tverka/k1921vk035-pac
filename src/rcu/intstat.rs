#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `OSECLKERR` reader - Status external oscillator clock fail signal"]
pub type OseclkerrR = crate::BitReader;
#[doc = "Field `OSECLKERR` writer - Status external oscillator clock fail signal"]
pub type OseclkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLCLKERR` reader - Status PLL clock fail signal"]
pub type PllclkerrR = crate::BitReader;
#[doc = "Field `PLLCLKERR` writer - Status PLL clock fail signal"]
pub type PllclkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLDIVCLKERR` reader - Status PLLDIV clock fail signal"]
pub type PlldivclkerrR = crate::BitReader;
#[doc = "Field `PLLDIVCLKERR` writer - Status PLLDIV clock fail signal"]
pub type PlldivclkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSECLKOK` reader - Status external oscillator clock good signal"]
pub type OseclkokR = crate::BitReader;
#[doc = "Field `OSECLKOK` writer - Status external oscillator clock good signal"]
pub type OseclkokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLCLKOK` reader - Status PLL clock good signal"]
pub type PllclkokR = crate::BitReader;
#[doc = "Field `PLLCLKOK` writer - Status PLL clock good signal"]
pub type PllclkokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLDIVCLKOK` reader - Status PLLDIV clock good signal"]
pub type PlldivclkokR = crate::BitReader;
#[doc = "Field `PLLDIVCLKOK` writer - Status PLLDIV clock good signal"]
pub type PlldivclkokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLOCK` reader - Status pll lock signal"]
pub type PlllockR = crate::BitReader;
#[doc = "Field `PLLLOCK` writer - Status pll lock signal"]
pub type PlllockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFAIL` reader - Current clock failed status"]
pub type SysfailR = crate::BitReader;
#[doc = "Field `SYSFAIL` writer - Current clock failed status"]
pub type SysfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Status external oscillator clock fail signal"]
    #[inline(always)]
    pub fn oseclkerr(&self) -> OseclkerrR {
        OseclkerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status PLL clock fail signal"]
    #[inline(always)]
    pub fn pllclkerr(&self) -> PllclkerrR {
        PllclkerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status PLLDIV clock fail signal"]
    #[inline(always)]
    pub fn plldivclkerr(&self) -> PlldivclkerrR {
        PlldivclkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Status external oscillator clock good signal"]
    #[inline(always)]
    pub fn oseclkok(&self) -> OseclkokR {
        OseclkokR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Status PLL clock good signal"]
    #[inline(always)]
    pub fn pllclkok(&self) -> PllclkokR {
        PllclkokR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status PLLDIV clock good signal"]
    #[inline(always)]
    pub fn plldivclkok(&self) -> PlldivclkokR {
        PlldivclkokR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Status pll lock signal"]
    #[inline(always)]
    pub fn plllock(&self) -> PlllockR {
        PlllockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Current clock failed status"]
    #[inline(always)]
    pub fn sysfail(&self) -> SysfailR {
        SysfailR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Status external oscillator clock fail signal"]
    #[inline(always)]
    #[must_use]
    pub fn oseclkerr(&mut self) -> OseclkerrW<IntstatSpec> {
        OseclkerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Status PLL clock fail signal"]
    #[inline(always)]
    #[must_use]
    pub fn pllclkerr(&mut self) -> PllclkerrW<IntstatSpec> {
        PllclkerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Status PLLDIV clock fail signal"]
    #[inline(always)]
    #[must_use]
    pub fn plldivclkerr(&mut self) -> PlldivclkerrW<IntstatSpec> {
        PlldivclkerrW::new(self, 3)
    }
    #[doc = "Bit 9 - Status external oscillator clock good signal"]
    #[inline(always)]
    #[must_use]
    pub fn oseclkok(&mut self) -> OseclkokW<IntstatSpec> {
        OseclkokW::new(self, 9)
    }
    #[doc = "Bit 10 - Status PLL clock good signal"]
    #[inline(always)]
    #[must_use]
    pub fn pllclkok(&mut self) -> PllclkokW<IntstatSpec> {
        PllclkokW::new(self, 10)
    }
    #[doc = "Bit 11 - Status PLLDIV clock good signal"]
    #[inline(always)]
    #[must_use]
    pub fn plldivclkok(&mut self) -> PlldivclkokW<IntstatSpec> {
        PlldivclkokW::new(self, 11)
    }
    #[doc = "Bit 16 - Status pll lock signal"]
    #[inline(always)]
    #[must_use]
    pub fn plllock(&mut self) -> PlllockW<IntstatSpec> {
        PlllockW::new(self, 16)
    }
    #[doc = "Bit 20 - Current clock failed status"]
    #[inline(always)]
    #[must_use]
    pub fn sysfail(&mut self) -> SysfailW<IntstatSpec> {
        SysfailW::new(self, 20)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
