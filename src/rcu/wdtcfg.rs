#[doc = "Register `WDTCFG` reader"]
pub type R = crate::R<WdtcfgSpec>;
#[doc = "Register `WDTCFG` writer"]
pub type W = crate::W<WdtcfgSpec>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDIS` reader - Reset disable"]
pub type RstdisR = crate::BitReader;
#[doc = "Field `RSTDIS` writer - Reset disable"]
pub type RstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: internal oscillator"]
    Osiclk = 0,
    #[doc = "1: external oscillator"]
    Oseclk = 1,
    #[doc = "2: PLL output clock"]
    Pllclk = 2,
    #[doc = "3: PLL divided clock"]
    Plldivclk = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock source select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Osiclk,
            1 => Clksel::Oseclk,
            2 => Clksel::Pllclk,
            3 => Clksel::Plldivclk,
            _ => unreachable!(),
        }
    }
    #[doc = "internal oscillator"]
    #[inline(always)]
    pub fn is_osiclk(&self) -> bool {
        *self == Clksel::Osiclk
    }
    #[doc = "external oscillator"]
    #[inline(always)]
    pub fn is_oseclk(&self) -> bool {
        *self == Clksel::Oseclk
    }
    #[doc = "PLL output clock"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == Clksel::Pllclk
    }
    #[doc = "PLL divided clock"]
    #[inline(always)]
    pub fn is_plldivclk(&self) -> bool {
        *self == Clksel::Plldivclk
    }
}
#[doc = "Field `CLKSEL` writer - Clock source select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "internal oscillator"]
    #[inline(always)]
    pub fn osiclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Osiclk)
    }
    #[doc = "external oscillator"]
    #[inline(always)]
    pub fn oseclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Oseclk)
    }
    #[doc = "PLL output clock"]
    #[inline(always)]
    pub fn pllclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Pllclk)
    }
    #[doc = "PLL divided clock"]
    #[inline(always)]
    pub fn plldivclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Plldivclk)
    }
}
#[doc = "Field `DIVEN` reader - Enable divider"]
pub type DivenR = crate::BitReader;
#[doc = "Field `DIVEN` writer - Enable divider"]
pub type DivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVN` reader - Divider coefficient"]
pub type DivnR = crate::FieldReader;
#[doc = "Field `DIVN` writer - Divider coefficient"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Reset disable"]
    #[inline(always)]
    pub fn rstdis(&self) -> RstdisR {
        RstdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock source select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable divider"]
    #[inline(always)]
    pub fn diven(&self) -> DivenR {
        DivenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<WdtcfgSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 4 - Reset disable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdis(&mut self) -> RstdisW<WdtcfgSpec> {
        RstdisW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<WdtcfgSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable divider"]
    #[inline(always)]
    #[must_use]
    pub fn diven(&mut self) -> DivenW<WdtcfgSpec> {
        DivenW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Divider coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn divn(&mut self) -> DivnW<WdtcfgSpec> {
        DivnW::new(self, 24)
    }
}
#[doc = "WatchDog configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtcfgSpec;
impl crate::RegisterSpec for WdtcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtcfg::R`](R) reader structure"]
impl crate::Readable for WdtcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtcfg::W`](W) writer structure"]
impl crate::Writable for WdtcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCFG to value 0"]
impl crate::Resettable for WdtcfgSpec {
    const RESET_VALUE: u32 = 0;
}
