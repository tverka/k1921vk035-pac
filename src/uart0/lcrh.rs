#[doc = "Register `LCRH` reader"]
pub type R = crate::R<LcrhSpec>;
#[doc = "Register `LCRH` writer"]
pub type W = crate::W<LcrhSpec>;
#[doc = "Field `BRK` reader - Send break"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - Send break"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - Even parity select"]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - Even parity select"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP2` reader - Two stop bits select"]
pub type Stp2R = crate::BitReader;
#[doc = "Field `STP2` writer - Two stop bits select"]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - Enable FIFOs"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - Enable FIFOs"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen {
    #[doc = "0: 5 bit in informational word"]
    _5bit = 0,
    #[doc = "1: 6 bit in informational word"]
    _6bit = 1,
    #[doc = "2: 7 bit in informational word"]
    _7bit = 2,
    #[doc = "3: 8 bit in informational word"]
    _8bit = 3,
}
impl From<Wlen> for u8 {
    #[inline(always)]
    fn from(variant: Wlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wlen {
    type Ux = u8;
}
impl crate::IsEnum for Wlen {}
#[doc = "Field `WLEN` reader - "]
pub type WlenR = crate::FieldReader<Wlen>;
impl WlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wlen {
        match self.bits {
            0 => Wlen::_5bit,
            1 => Wlen::_6bit,
            2 => Wlen::_7bit,
            3 => Wlen::_8bit,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bit in informational word"]
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == Wlen::_5bit
    }
    #[doc = "6 bit in informational word"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Wlen::_6bit
    }
    #[doc = "7 bit in informational word"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Wlen::_7bit
    }
    #[doc = "8 bit in informational word"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Wlen::_8bit
    }
}
#[doc = "Field `WLEN` writer - "]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wlen, crate::Safe>;
impl<'a, REG> WlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bit in informational word"]
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_5bit)
    }
    #[doc = "6 bit in informational word"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_6bit)
    }
    #[doc = "7 bit in informational word"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_7bit)
    }
    #[doc = "8 bit in informational word"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_8bit)
    }
}
#[doc = "Field `SPS` reader - Stick parity select"]
pub type SpsR = crate::BitReader;
#[doc = "Field `SPS` writer - Stick parity select"]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Two stop bits select"]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BrkW<LcrhSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<LcrhSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EpsW<LcrhSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Two stop bits select"]
    #[inline(always)]
    #[must_use]
    pub fn stp2(&mut self) -> Stp2W<LcrhSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FenW<LcrhSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<LcrhSpec> {
        WlenW::new(self, 5)
    }
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SpsW<LcrhSpec> {
        SpsW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrhSpec;
impl crate::RegisterSpec for LcrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrh::R`](R) reader structure"]
impl crate::Readable for LcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`lcrh::W`](W) writer structure"]
impl crate::Writable for LcrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LcrhSpec {
    const RESET_VALUE: u32 = 0;
}
