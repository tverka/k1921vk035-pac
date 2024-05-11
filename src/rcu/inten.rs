#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `OSECLKERR` reader - Enable OSECLK fail interrupt"]
pub type OseclkerrR = crate::BitReader;
#[doc = "Field `OSECLKERR` writer - Enable OSECLK fail interrupt"]
pub type OseclkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLCLKERR` reader - Enable PLLCLK fail interrupt"]
pub type PllclkerrR = crate::BitReader;
#[doc = "Field `PLLCLKERR` writer - Enable PLLCLK fail interrupt"]
pub type PllclkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLDIVCLKERR` reader - Enable PLLDIVCLK fail interrupt"]
pub type PlldivclkerrR = crate::BitReader;
#[doc = "Field `PLLDIVCLKERR` writer - Enable PLLDIVCLK fail interrupt"]
pub type PlldivclkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSECLKOK` reader - Enable OSECLK good interrupt"]
pub type OseclkokR = crate::BitReader;
#[doc = "Field `OSECLKOK` writer - Enable OSECLK good interrupt"]
pub type OseclkokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLCLKOK` reader - Enable PLLCLK good interrupt"]
pub type PllclkokR = crate::BitReader;
#[doc = "Field `PLLCLKOK` writer - Enable PLLCLK good interrupt"]
pub type PllclkokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLDIVCLKOK` reader - Enable PLLDIVCLK good interrupt"]
pub type PlldivclkokR = crate::BitReader;
#[doc = "Field `PLLDIVCLKOK` writer - Enable PLLDIVCLK good interrupt"]
pub type PlldivclkokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLOCK` reader - Enable int from pll lock signal"]
pub type PlllockR = crate::BitReader;
#[doc = "Field `PLLLOCK` writer - Enable int from pll lock signal"]
pub type PlllockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable OSECLK fail interrupt"]
    #[inline(always)]
    pub fn oseclkerr(&self) -> OseclkerrR {
        OseclkerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable PLLCLK fail interrupt"]
    #[inline(always)]
    pub fn pllclkerr(&self) -> PllclkerrR {
        PllclkerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable PLLDIVCLK fail interrupt"]
    #[inline(always)]
    pub fn plldivclkerr(&self) -> PlldivclkerrR {
        PlldivclkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable OSECLK good interrupt"]
    #[inline(always)]
    pub fn oseclkok(&self) -> OseclkokR {
        OseclkokR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PLLCLK good interrupt"]
    #[inline(always)]
    pub fn pllclkok(&self) -> PllclkokR {
        PllclkokR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable PLLDIVCLK good interrupt"]
    #[inline(always)]
    pub fn plldivclkok(&self) -> PlldivclkokR {
        PlldivclkokR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable int from pll lock signal"]
    #[inline(always)]
    pub fn plllock(&self) -> PlllockR {
        PlllockR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable OSECLK fail interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn oseclkerr(&mut self) -> OseclkerrW<IntenSpec> {
        OseclkerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable PLLCLK fail interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pllclkerr(&mut self) -> PllclkerrW<IntenSpec> {
        PllclkerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable PLLDIVCLK fail interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn plldivclkerr(&mut self) -> PlldivclkerrW<IntenSpec> {
        PlldivclkerrW::new(self, 3)
    }
    #[doc = "Bit 9 - Enable OSECLK good interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn oseclkok(&mut self) -> OseclkokW<IntenSpec> {
        OseclkokW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PLLCLK good interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pllclkok(&mut self) -> PllclkokW<IntenSpec> {
        PllclkokW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable PLLDIVCLK good interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn plldivclkok(&mut self) -> PlldivclkokW<IntenSpec> {
        PlldivclkokW::new(self, 11)
    }
    #[doc = "Bit 16 - Enable int from pll lock signal"]
    #[inline(always)]
    #[must_use]
    pub fn plllock(&mut self) -> PlllockW<IntenSpec> {
        PlllockW::new(self, 16)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
