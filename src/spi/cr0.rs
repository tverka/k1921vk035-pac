#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "3: data size 4 bit"]
    _4bit = 3,
    #[doc = "4: data size 5 bit"]
    _5bit = 4,
    #[doc = "5: data size 6 bit"]
    _6bit = 5,
    #[doc = "6: data size 7 bit"]
    _7bit = 6,
    #[doc = "7: data size 8 bit"]
    _8bit = 7,
    #[doc = "8: data size 9 bit"]
    _9bit = 8,
    #[doc = "9: data size 10 bit"]
    _10bit = 9,
    #[doc = "10: data size 11 bit"]
    _11bit = 10,
    #[doc = "11: data size 12 bit"]
    _12bit = 11,
    #[doc = "12: data size 13 bit"]
    _13bit = 12,
    #[doc = "13: data size 14 bit"]
    _14bit = 13,
    #[doc = "14: data size 15 bit"]
    _15bit = 14,
    #[doc = "15: data size 16 bit"]
    _16bit = 15,
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(variant: Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dss {
    type Ux = u8;
}
impl crate::IsEnum for Dss {}
#[doc = "Field `DSS` reader - "]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            3 => Some(Dss::_4bit),
            4 => Some(Dss::_5bit),
            5 => Some(Dss::_6bit),
            6 => Some(Dss::_7bit),
            7 => Some(Dss::_8bit),
            8 => Some(Dss::_9bit),
            9 => Some(Dss::_10bit),
            10 => Some(Dss::_11bit),
            11 => Some(Dss::_12bit),
            12 => Some(Dss::_13bit),
            13 => Some(Dss::_14bit),
            14 => Some(Dss::_15bit),
            15 => Some(Dss::_16bit),
            _ => None,
        }
    }
    #[doc = "data size 4 bit"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == Dss::_4bit
    }
    #[doc = "data size 5 bit"]
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == Dss::_5bit
    }
    #[doc = "data size 6 bit"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Dss::_6bit
    }
    #[doc = "data size 7 bit"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Dss::_7bit
    }
    #[doc = "data size 8 bit"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Dss::_8bit
    }
    #[doc = "data size 9 bit"]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == Dss::_9bit
    }
    #[doc = "data size 10 bit"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Dss::_10bit
    }
    #[doc = "data size 11 bit"]
    #[inline(always)]
    pub fn is_11bit(&self) -> bool {
        *self == Dss::_11bit
    }
    #[doc = "data size 12 bit"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Dss::_12bit
    }
    #[doc = "data size 13 bit"]
    #[inline(always)]
    pub fn is_13bit(&self) -> bool {
        *self == Dss::_13bit
    }
    #[doc = "data size 14 bit"]
    #[inline(always)]
    pub fn is_14bit(&self) -> bool {
        *self == Dss::_14bit
    }
    #[doc = "data size 15 bit"]
    #[inline(always)]
    pub fn is_15bit(&self) -> bool {
        *self == Dss::_15bit
    }
    #[doc = "data size 16 bit"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Dss::_16bit
    }
}
#[doc = "Field `DSS` writer - "]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "data size 4 bit"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_4bit)
    }
    #[doc = "data size 5 bit"]
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_5bit)
    }
    #[doc = "data size 6 bit"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_6bit)
    }
    #[doc = "data size 7 bit"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_7bit)
    }
    #[doc = "data size 8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_8bit)
    }
    #[doc = "data size 9 bit"]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_9bit)
    }
    #[doc = "data size 10 bit"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_10bit)
    }
    #[doc = "data size 11 bit"]
    #[inline(always)]
    pub fn _11bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_11bit)
    }
    #[doc = "data size 12 bit"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_12bit)
    }
    #[doc = "data size 13 bit"]
    #[inline(always)]
    pub fn _13bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_13bit)
    }
    #[doc = "data size 14 bit"]
    #[inline(always)]
    pub fn _14bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_14bit)
    }
    #[doc = "data size 15 bit"]
    #[inline(always)]
    pub fn _15bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_15bit)
    }
    #[doc = "data size 16 bit"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_16bit)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "0: SPI of Motorola"]
    Spi = 0,
    #[doc = "1: SSI of Texas Instruments"]
    Ssi = 1,
    #[doc = "2: Microwire of National Semiconductor"]
    Microwire = 2,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
impl crate::IsEnum for Frf {}
#[doc = "Field `FRF` reader - "]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frf> {
        match self.bits {
            0 => Some(Frf::Spi),
            1 => Some(Frf::Ssi),
            2 => Some(Frf::Microwire),
            _ => None,
        }
    }
    #[doc = "SPI of Motorola"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Frf::Spi
    }
    #[doc = "SSI of Texas Instruments"]
    #[inline(always)]
    pub fn is_ssi(&self) -> bool {
        *self == Frf::Ssi
    }
    #[doc = "Microwire of National Semiconductor"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == Frf::Microwire
    }
}
#[doc = "Field `FRF` writer - "]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI of Motorola"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Spi)
    }
    #[doc = "SSI of Texas Instruments"]
    #[inline(always)]
    pub fn ssi(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Ssi)
    }
    #[doc = "Microwire of National Semiconductor"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Microwire)
    }
}
#[doc = "Field `SPO` reader - Polarity SSPCLKOUT"]
pub type SpoR = crate::BitReader;
#[doc = "Field `SPO` writer - Polarity SSPCLKOUT"]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPH` reader - Phase SSPCLKOUT"]
pub type SphR = crate::BitReader;
#[doc = "Field `SPH` writer - Phase SSPCLKOUT"]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCR` reader - "]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - "]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Polarity SSPCLKOUT"]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Phase SSPCLKOUT"]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DssW<Cr0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FrfW<Cr0Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 6 - Polarity SSPCLKOUT"]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SpoW<Cr0Spec> {
        SpoW::new(self, 6)
    }
    #[doc = "Bit 7 - Phase SSPCLKOUT"]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SphW<Cr0Spec> {
        SphW::new(self, 7)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<Cr0Spec> {
        ScrW::new(self, 8)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {
    const RESET_VALUE: u32 = 0;
}
