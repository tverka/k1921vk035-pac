#[doc = "Register `SYSCLKCFG` reader"]
pub type R = crate::R<SysclkcfgSpec>;
#[doc = "Register `SYSCLKCFG` writer"]
pub type W = crate::W<SysclkcfgSpec>;
#[doc = "System clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syssel {
    #[doc = "0: internal oscillator"]
    Osiclk = 0,
    #[doc = "1: external oscillator"]
    Oseclk = 1,
    #[doc = "2: PLL output clock"]
    Pllclk = 2,
    #[doc = "3: PLL divided clock"]
    Plldivclk = 3,
}
impl From<Syssel> for u8 {
    #[inline(always)]
    fn from(variant: Syssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syssel {
    type Ux = u8;
}
impl crate::IsEnum for Syssel {}
#[doc = "Field `SYSSEL` reader - System clock source selection"]
pub type SysselR = crate::FieldReader<Syssel>;
impl SysselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syssel {
        match self.bits {
            0 => Syssel::Osiclk,
            1 => Syssel::Oseclk,
            2 => Syssel::Pllclk,
            3 => Syssel::Plldivclk,
            _ => unreachable!(),
        }
    }
    #[doc = "internal oscillator"]
    #[inline(always)]
    pub fn is_osiclk(&self) -> bool {
        *self == Syssel::Osiclk
    }
    #[doc = "external oscillator"]
    #[inline(always)]
    pub fn is_oseclk(&self) -> bool {
        *self == Syssel::Oseclk
    }
    #[doc = "PLL output clock"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == Syssel::Pllclk
    }
    #[doc = "PLL divided clock"]
    #[inline(always)]
    pub fn is_plldivclk(&self) -> bool {
        *self == Syssel::Plldivclk
    }
}
#[doc = "Field `SYSSEL` writer - System clock source selection"]
pub type SysselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Syssel, crate::Safe>;
impl<'a, REG> SysselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "internal oscillator"]
    #[inline(always)]
    pub fn osiclk(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Osiclk)
    }
    #[doc = "external oscillator"]
    #[inline(always)]
    pub fn oseclk(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Oseclk)
    }
    #[doc = "PLL output clock"]
    #[inline(always)]
    pub fn pllclk(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Pllclk)
    }
    #[doc = "PLL divided clock"]
    #[inline(always)]
    pub fn plldivclk(self) -> &'a mut crate::W<REG> {
        self.variant(Syssel::Plldivclk)
    }
}
#[doc = "Field `SECEN` reader - Enable clock security system"]
pub type SecenR = crate::BitReader;
#[doc = "Field `SECEN` writer - Enable clock security system"]
pub type SecenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - System clock source selection"]
    #[inline(always)]
    pub fn syssel(&self) -> SysselR {
        SysselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - Enable clock security system"]
    #[inline(always)]
    pub fn secen(&self) -> SecenR {
        SecenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SysselW<SysclkcfgSpec> {
        SysselW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable clock security system"]
    #[inline(always)]
    #[must_use]
    pub fn secen(&mut self) -> SecenW<SysclkcfgSpec> {
        SecenW::new(self, 16)
    }
}
#[doc = "System clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysclkcfgSpec;
impl crate::RegisterSpec for SysclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclkcfg::R`](R) reader structure"]
impl crate::Readable for SysclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysclkcfg::W`](W) writer structure"]
impl crate::Writable for SysclkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCLKCFG to value 0"]
impl crate::Resettable for SysclkcfgSpec {
    const RESET_VALUE: u32 = 0;
}
