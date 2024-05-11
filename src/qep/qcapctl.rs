#[doc = "Register `QCAPCTL` reader"]
pub type R = crate::R<QcapctlSpec>;
#[doc = "Register `QCAPCTL` writer"]
pub type W = crate::W<QcapctlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upps {
    #[doc = "0: quad signal not divided"]
    Disable = 0,
    #[doc = "1: quad signal divided by 2"]
    Div2 = 1,
    #[doc = "2: quad signal divided by 4"]
    Div4 = 2,
    #[doc = "3: quad signal divided by 8"]
    Div8 = 3,
    #[doc = "4: quad signal divided by 16"]
    Div16 = 4,
    #[doc = "5: quad signal divided by 32"]
    Div32 = 5,
    #[doc = "6: quad signal divided by 64"]
    Div64 = 6,
    #[doc = "7: quad signal divided by 128"]
    Div128 = 7,
    #[doc = "8: quad signal divided by 256"]
    Div256 = 8,
    #[doc = "9: quad signal divided by 512"]
    Div512 = 9,
    #[doc = "10: quad signal divided by 1024"]
    Div1024 = 10,
    #[doc = "11: quad signal divided by 2048"]
    Div2048 = 11,
}
impl From<Upps> for u8 {
    #[inline(always)]
    fn from(variant: Upps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upps {
    type Ux = u8;
}
impl crate::IsEnum for Upps {}
#[doc = "Field `UPPS` reader - "]
pub type UppsR = crate::FieldReader<Upps>;
impl UppsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Upps> {
        match self.bits {
            0 => Some(Upps::Disable),
            1 => Some(Upps::Div2),
            2 => Some(Upps::Div4),
            3 => Some(Upps::Div8),
            4 => Some(Upps::Div16),
            5 => Some(Upps::Div32),
            6 => Some(Upps::Div64),
            7 => Some(Upps::Div128),
            8 => Some(Upps::Div256),
            9 => Some(Upps::Div512),
            10 => Some(Upps::Div1024),
            11 => Some(Upps::Div2048),
            _ => None,
        }
    }
    #[doc = "quad signal not divided"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Upps::Disable
    }
    #[doc = "quad signal divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Upps::Div2
    }
    #[doc = "quad signal divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Upps::Div4
    }
    #[doc = "quad signal divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Upps::Div8
    }
    #[doc = "quad signal divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Upps::Div16
    }
    #[doc = "quad signal divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Upps::Div32
    }
    #[doc = "quad signal divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Upps::Div64
    }
    #[doc = "quad signal divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Upps::Div128
    }
    #[doc = "quad signal divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Upps::Div256
    }
    #[doc = "quad signal divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Upps::Div512
    }
    #[doc = "quad signal divided by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Upps::Div1024
    }
    #[doc = "quad signal divided by 2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Upps::Div2048
    }
}
#[doc = "Field `UPPS` writer - "]
pub type UppsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Upps>;
impl<'a, REG> UppsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "quad signal not divided"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Disable)
    }
    #[doc = "quad signal divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div2)
    }
    #[doc = "quad signal divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div4)
    }
    #[doc = "quad signal divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div8)
    }
    #[doc = "quad signal divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div16)
    }
    #[doc = "quad signal divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div32)
    }
    #[doc = "quad signal divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div64)
    }
    #[doc = "quad signal divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div128)
    }
    #[doc = "quad signal divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div256)
    }
    #[doc = "quad signal divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div512)
    }
    #[doc = "quad signal divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div1024)
    }
    #[doc = "quad signal divided by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Upps::Div2048)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccps {
    #[doc = "0: no divider"]
    Disable = 0,
    #[doc = "1: sysclk divided by 2"]
    Div2 = 1,
    #[doc = "2: sysclk divided by 4"]
    Div4 = 2,
    #[doc = "3: sysclk divided by 8"]
    Div8 = 3,
    #[doc = "4: sysclk divided by 16"]
    Div16 = 4,
    #[doc = "5: sysclk divided by 32"]
    Div32 = 5,
    #[doc = "6: sysclk divided by 64"]
    Div64 = 6,
    #[doc = "7: sysclk divided by 128"]
    Div128 = 7,
}
impl From<Ccps> for u8 {
    #[inline(always)]
    fn from(variant: Ccps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccps {
    type Ux = u8;
}
impl crate::IsEnum for Ccps {}
#[doc = "Field `CCPS` reader - "]
pub type CcpsR = crate::FieldReader<Ccps>;
impl CcpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccps {
        match self.bits {
            0 => Ccps::Disable,
            1 => Ccps::Div2,
            2 => Ccps::Div4,
            3 => Ccps::Div8,
            4 => Ccps::Div16,
            5 => Ccps::Div32,
            6 => Ccps::Div64,
            7 => Ccps::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "no divider"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ccps::Disable
    }
    #[doc = "sysclk divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ccps::Div2
    }
    #[doc = "sysclk divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ccps::Div4
    }
    #[doc = "sysclk divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ccps::Div8
    }
    #[doc = "sysclk divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Ccps::Div16
    }
    #[doc = "sysclk divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Ccps::Div32
    }
    #[doc = "sysclk divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Ccps::Div64
    }
    #[doc = "sysclk divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Ccps::Div128
    }
}
#[doc = "Field `CCPS` writer - "]
pub type CcpsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccps, crate::Safe>;
impl<'a, REG> CcpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no divider"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Disable)
    }
    #[doc = "sysclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Div2)
    }
    #[doc = "sysclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Div4)
    }
    #[doc = "sysclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Div8)
    }
    #[doc = "sysclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Div16)
    }
    #[doc = "sysclk divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Div32)
    }
    #[doc = "sysclk divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Div64)
    }
    #[doc = "sysclk divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Ccps::Div128)
    }
}
#[doc = "Field `SELEVENT` reader - Reset timer control"]
pub type SeleventR = crate::BitReader;
#[doc = "Field `SELEVENT` writer - Reset timer control"]
pub type SeleventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - Enable eQEP capture"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Enable eQEP capture"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPSLD` reader - Enhanced prescalers load"]
pub type EpsldR = crate::BitReader;
#[doc = "Field `EPSLD` writer - Enhanced prescalers load"]
pub type EpsldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn upps(&self) -> UppsR {
        UppsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ccps(&self) -> CcpsR {
        CcpsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Reset timer control"]
    #[inline(always)]
    pub fn selevent(&self) -> SeleventR {
        SeleventR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable eQEP capture"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enhanced prescalers load"]
    #[inline(always)]
    pub fn epsld(&self) -> EpsldR {
        EpsldR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn upps(&mut self) -> UppsW<QcapctlSpec> {
        UppsW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn ccps(&mut self) -> CcpsW<QcapctlSpec> {
        CcpsW::new(self, 4)
    }
    #[doc = "Bit 7 - Reset timer control"]
    #[inline(always)]
    #[must_use]
    pub fn selevent(&mut self) -> SeleventW<QcapctlSpec> {
        SeleventW::new(self, 7)
    }
    #[doc = "Bit 15 - Enable eQEP capture"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<QcapctlSpec> {
        CenW::new(self, 15)
    }
    #[doc = "Bit 16 - Enhanced prescalers load"]
    #[inline(always)]
    #[must_use]
    pub fn epsld(&mut self) -> EpsldW<QcapctlSpec> {
        EpsldW::new(self, 16)
    }
}
#[doc = "Capture Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qcapctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qcapctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcapctlSpec;
impl crate::RegisterSpec for QcapctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qcapctl::R`](R) reader structure"]
impl crate::Readable for QcapctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qcapctl::W`](W) writer structure"]
impl crate::Writable for QcapctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCAPCTL to value 0"]
impl crate::Resettable for QcapctlSpec {
    const RESET_VALUE: u32 = 0;
}
