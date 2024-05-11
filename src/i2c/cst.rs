#[doc = "Register `CST` reader"]
pub type R = crate::R<CstSpec>;
#[doc = "Register `CST` writer"]
pub type W = crate::W<CstSpec>;
#[doc = "Field `BB` reader - Bus Busy"]
pub type BbR = crate::BitReader;
#[doc = "Field `BB` writer - Bus Busy"]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tocdiv {
    #[doc = "0: disable clock"]
    Disable = 0,
    #[doc = "1: clock divided by 4"]
    Div4 = 1,
    #[doc = "2: clock divided by 8"]
    Div8 = 2,
    #[doc = "3: clock divided by 16"]
    Div16 = 3,
}
impl From<Tocdiv> for u8 {
    #[inline(always)]
    fn from(variant: Tocdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tocdiv {
    type Ux = u8;
}
impl crate::IsEnum for Tocdiv {}
#[doc = "Field `TOCDIV` reader - "]
pub type TocdivR = crate::FieldReader<Tocdiv>;
impl TocdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tocdiv {
        match self.bits {
            0 => Tocdiv::Disable,
            1 => Tocdiv::Div4,
            2 => Tocdiv::Div8,
            3 => Tocdiv::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "disable clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tocdiv::Disable
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Tocdiv::Div4
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Tocdiv::Div8
    }
    #[doc = "clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Tocdiv::Div16
    }
}
#[doc = "Field `TOCDIV` writer - "]
pub type TocdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tocdiv, crate::Safe>;
impl<'a, REG> TocdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tocdiv::Disable)
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tocdiv::Div4)
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Tocdiv::Div8)
    }
    #[doc = "clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Tocdiv::Div16)
    }
}
#[doc = "Field `TOERR` reader - SMBus Timeout Error"]
pub type ToerrR = crate::BitReader;
#[doc = "Field `TOERR` writer - SMBus Timeout Error"]
pub type ToerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSDA` reader - Bit test SDA"]
pub type TsdaR = crate::BitReader;
#[doc = "Field `TSDA` writer - Bit test SDA"]
pub type TsdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGSCL` reader - Toggle SCL"]
pub type TgsclR = crate::BitReader;
#[doc = "Field `TGSCL` writer - Toggle SCL"]
pub type TgsclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECNEXT` reader - PEC Next"]
pub type PecnextR = crate::BitReader;
#[doc = "Field `PECNEXT` writer - PEC Next"]
pub type PecnextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECFAULT` reader - Packet Error Fault"]
pub type PecfaultR = crate::BitReader;
#[doc = "Field `PECFAULT` writer - Packet Error Fault"]
pub type PecfaultW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bus Busy"]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn tocdiv(&self) -> TocdivR {
        TocdivR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - SMBus Timeout Error"]
    #[inline(always)]
    pub fn toerr(&self) -> ToerrR {
        ToerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit test SDA"]
    #[inline(always)]
    pub fn tsda(&self) -> TsdaR {
        TsdaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Toggle SCL"]
    #[inline(always)]
    pub fn tgscl(&self) -> TgsclR {
        TgsclR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PEC Next"]
    #[inline(always)]
    pub fn pecnext(&self) -> PecnextR {
        PecnextR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Packet Error Fault"]
    #[inline(always)]
    pub fn pecfault(&self) -> PecfaultR {
        PecfaultR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Busy"]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BbW<CstSpec> {
        BbW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn tocdiv(&mut self) -> TocdivW<CstSpec> {
        TocdivW::new(self, 1)
    }
    #[doc = "Bit 3 - SMBus Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn toerr(&mut self) -> ToerrW<CstSpec> {
        ToerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Bit test SDA"]
    #[inline(always)]
    #[must_use]
    pub fn tsda(&mut self) -> TsdaW<CstSpec> {
        TsdaW::new(self, 4)
    }
    #[doc = "Bit 5 - Toggle SCL"]
    #[inline(always)]
    #[must_use]
    pub fn tgscl(&mut self) -> TgsclW<CstSpec> {
        TgsclW::new(self, 5)
    }
    #[doc = "Bit 6 - PEC Next"]
    #[inline(always)]
    #[must_use]
    pub fn pecnext(&mut self) -> PecnextW<CstSpec> {
        PecnextW::new(self, 6)
    }
    #[doc = "Bit 7 - Packet Error Fault"]
    #[inline(always)]
    #[must_use]
    pub fn pecfault(&mut self) -> PecfaultW<CstSpec> {
        PecfaultW::new(self, 7)
    }
}
#[doc = "Status and control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CstSpec;
impl crate::RegisterSpec for CstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cst::R`](R) reader structure"]
impl crate::Readable for CstSpec {}
#[doc = "`write(|w| ..)` method takes [`cst::W`](W) writer structure"]
impl crate::Writable for CstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CST to value 0"]
impl crate::Resettable for CstSpec {
    const RESET_VALUE: u32 = 0;
}
