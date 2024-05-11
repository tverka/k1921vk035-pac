#[doc = "Register `PLLCFG` reader"]
pub type R = crate::R<PllcfgSpec>;
#[doc = "Register `PLLCFG` writer"]
pub type W = crate::W<PllcfgSpec>;
#[doc = "Field `M` reader - PLL M Coefficient"]
pub type MR = crate::FieldReader;
#[doc = "Field `M` writer - PLL M Coefficient"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `N` reader - PLL N Coefficient"]
pub type NR = crate::FieldReader;
#[doc = "Field `N` writer - PLL N Coefficient"]
pub type NW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "PLL OD Coefficient\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Od {
    #[doc = "0: disabled"]
    Disable = 0,
    #[doc = "1: divide by 2"]
    Div2 = 1,
    #[doc = "2: divide by 4"]
    Div4 = 2,
    #[doc = "3: divide by 8"]
    Div8 = 3,
}
impl From<Od> for u8 {
    #[inline(always)]
    fn from(variant: Od) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Od {
    type Ux = u8;
}
impl crate::IsEnum for Od {}
#[doc = "Field `OD` reader - PLL OD Coefficient"]
pub type OdR = crate::FieldReader<Od>;
impl OdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Od {
        match self.bits {
            0 => Od::Disable,
            1 => Od::Div2,
            2 => Od::Div4,
            3 => Od::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Od::Disable
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Od::Div2
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Od::Div4
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Od::Div8
    }
}
#[doc = "Field `OD` writer - PLL OD Coefficient"]
pub type OdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Od, crate::Safe>;
impl<'a, REG> OdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Disable)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Div2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Div4)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Div8)
    }
}
#[doc = "PLL Reference source select bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refsrc {
    #[doc = "0: external oscillator"]
    Oseclk = 0,
    #[doc = "1: internal oscillator"]
    Osiclk = 1,
}
impl From<Refsrc> for bool {
    #[inline(always)]
    fn from(variant: Refsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFSRC` reader - PLL Reference source select bit"]
pub type RefsrcR = crate::BitReader<Refsrc>;
impl RefsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsrc {
        match self.bits {
            false => Refsrc::Oseclk,
            true => Refsrc::Osiclk,
        }
    }
    #[doc = "external oscillator"]
    #[inline(always)]
    pub fn is_oseclk(&self) -> bool {
        *self == Refsrc::Oseclk
    }
    #[doc = "internal oscillator"]
    #[inline(always)]
    pub fn is_osiclk(&self) -> bool {
        *self == Refsrc::Osiclk
    }
}
#[doc = "Field `REFSRC` writer - PLL Reference source select bit"]
pub type RefsrcW<'a, REG> = crate::BitWriter<'a, REG, Refsrc>;
impl<'a, REG> RefsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external oscillator"]
    #[inline(always)]
    pub fn oseclk(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::Oseclk)
    }
    #[doc = "internal oscillator"]
    #[inline(always)]
    pub fn osiclk(self) -> &'a mut crate::W<REG> {
        self.variant(Refsrc::Osiclk)
    }
}
#[doc = "Field `BYPASS` reader - PLL Bypass enable"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - PLL Bypass enable"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEN` reader - Enable PLL out"]
pub type OutenR = crate::BitReader;
#[doc = "Field `OUTEN` writer - Enable PLL out"]
pub type OutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - PLL status lock"]
pub type LockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - PLL M Coefficient"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - PLL N Coefficient"]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - PLL OD Coefficient"]
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - PLL Reference source select bit"]
    #[inline(always)]
    pub fn refsrc(&self) -> RefsrcR {
        RefsrcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL Bypass enable"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable PLL out"]
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL status lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL M Coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<PllcfgSpec> {
        MW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PLL N Coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> NW<PllcfgSpec> {
        NW::new(self, 8)
    }
    #[doc = "Bits 16:17 - PLL OD Coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn od(&mut self) -> OdW<PllcfgSpec> {
        OdW::new(self, 16)
    }
    #[doc = "Bit 20 - PLL Reference source select bit"]
    #[inline(always)]
    #[must_use]
    pub fn refsrc(&mut self) -> RefsrcW<PllcfgSpec> {
        RefsrcW::new(self, 20)
    }
    #[doc = "Bit 24 - PLL Bypass enable"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<PllcfgSpec> {
        BypassW::new(self, 24)
    }
    #[doc = "Bit 26 - Enable PLL out"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OutenW<PllcfgSpec> {
        OutenW::new(self, 26)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgSpec;
impl crate::RegisterSpec for PllcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfg::R`](R) reader structure"]
impl crate::Readable for PllcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfg::W`](W) writer structure"]
impl crate::Writable for PllcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCFG to value 0"]
impl crate::Resettable for PllcfgSpec {
    const RESET_VALUE: u32 = 0;
}
