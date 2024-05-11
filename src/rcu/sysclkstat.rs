#[doc = "Register `SYSCLKSTAT` reader"]
pub type R = crate::R<SysclkstatSpec>;
#[doc = "Current system source clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sysstat {
    #[doc = "0: internal oscillator"]
    Osiclk = 0,
    #[doc = "1: external oscillator"]
    Oseclk = 1,
    #[doc = "2: PLL output clock"]
    Pllclk = 2,
    #[doc = "3: PLL divided clock"]
    Plldivclk = 3,
}
impl From<Sysstat> for u8 {
    #[inline(always)]
    fn from(variant: Sysstat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysstat {
    type Ux = u8;
}
impl crate::IsEnum for Sysstat {}
#[doc = "Field `SYSSTAT` reader - Current system source clock"]
pub type SysstatR = crate::FieldReader<Sysstat>;
impl SysstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysstat {
        match self.bits {
            0 => Sysstat::Osiclk,
            1 => Sysstat::Oseclk,
            2 => Sysstat::Pllclk,
            3 => Sysstat::Plldivclk,
            _ => unreachable!(),
        }
    }
    #[doc = "internal oscillator"]
    #[inline(always)]
    pub fn is_osiclk(&self) -> bool {
        *self == Sysstat::Osiclk
    }
    #[doc = "external oscillator"]
    #[inline(always)]
    pub fn is_oseclk(&self) -> bool {
        *self == Sysstat::Oseclk
    }
    #[doc = "PLL output clock"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == Sysstat::Pllclk
    }
    #[doc = "PLL divided clock"]
    #[inline(always)]
    pub fn is_plldivclk(&self) -> bool {
        *self == Sysstat::Plldivclk
    }
}
#[doc = "Field `BUSY` reader - Clock manager is busy, for example, when change clock source"]
pub type BusyR = crate::BitReader;
#[doc = "Field `SYSFAIL` reader - Error in current clock was detected"]
pub type SysfailR = crate::BitReader;
#[doc = "Field `OSECLKERR` reader - External oscillator clock fail"]
pub type OseclkerrR = crate::BitReader;
#[doc = "Field `PLLCLKERR` reader - PLL source clock fail"]
pub type PllclkerrR = crate::BitReader;
#[doc = "Field `PLLDIVCLKERR` reader - PLL divided clock fail"]
pub type PlldivclkerrR = crate::BitReader;
#[doc = "Field `OSECLKOK` reader - External oscillator clock good"]
pub type OseclkokR = crate::BitReader;
#[doc = "Field `PLLCLKOK` reader - PLL clock good"]
pub type PllclkokR = crate::BitReader;
#[doc = "Field `PLLDIVCLKOK` reader - PLL divided clock good"]
pub type PlldivclkokR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Current system source clock"]
    #[inline(always)]
    pub fn sysstat(&self) -> SysstatR {
        SysstatR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Clock manager is busy, for example, when change clock source"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Error in current clock was detected"]
    #[inline(always)]
    pub fn sysfail(&self) -> SysfailR {
        SysfailR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 17 - External oscillator clock fail"]
    #[inline(always)]
    pub fn oseclkerr(&self) -> OseclkerrR {
        OseclkerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL source clock fail"]
    #[inline(always)]
    pub fn pllclkerr(&self) -> PllclkerrR {
        PllclkerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PLL divided clock fail"]
    #[inline(always)]
    pub fn plldivclkerr(&self) -> PlldivclkerrR {
        PlldivclkerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - External oscillator clock good"]
    #[inline(always)]
    pub fn oseclkok(&self) -> OseclkokR {
        OseclkokR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL clock good"]
    #[inline(always)]
    pub fn pllclkok(&self) -> PllclkokR {
        PllclkokR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL divided clock good"]
    #[inline(always)]
    pub fn plldivclkok(&self) -> PlldivclkokR {
        PlldivclkokR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "System clock status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysclkstatSpec;
impl crate::RegisterSpec for SysclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclkstat::R`](R) reader structure"]
impl crate::Readable for SysclkstatSpec {}
#[doc = "`reset()` method sets SYSCLKSTAT to value 0"]
impl crate::Resettable for SysclkstatSpec {
    const RESET_VALUE: u32 = 0;
}
