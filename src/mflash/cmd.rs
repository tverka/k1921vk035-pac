#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `RD` reader - Read enable command"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Read enable command"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` reader - Write enable command"]
pub type WrR = crate::BitReader;
#[doc = "Field `WR` writer - Write enable command"]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSEC` reader - Erase sector enable command"]
pub type ErsecR = crate::BitReader;
#[doc = "Field `ERSEC` writer - Erase sector enable command"]
pub type ErsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERALL` reader - Erase all enable command"]
pub type ErallR = crate::BitReader;
#[doc = "Field `ERALL` writer - Erase all enable command"]
pub type ErallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVRON` reader - NVR access bit"]
pub type NvronR = crate::BitReader;
#[doc = "Field `NVRON` writer - NVR access bit"]
pub type NvronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Magic Key for flash access \"C0DE\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    #[doc = "49374: magic Key for flash access"]
    Access = 49374,
}
impl From<Key> for u16 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u16;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Magic Key for flash access \"C0DE\""]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            49374 => Some(Key::Access),
            _ => None,
        }
    }
    #[doc = "magic Key for flash access"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Key::Access
    }
}
#[doc = "Field `KEY` writer - Magic Key for flash access \"C0DE\""]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "magic Key for flash access"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Access)
    }
}
impl R {
    #[doc = "Bit 0 - Read enable command"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write enable command"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Erase sector enable command"]
    #[inline(always)]
    pub fn ersec(&self) -> ErsecR {
        ErsecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Erase all enable command"]
    #[inline(always)]
    pub fn erall(&self) -> ErallR {
        ErallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - NVR access bit"]
    #[inline(always)]
    pub fn nvron(&self) -> NvronR {
        NvronR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Magic Key for flash access \"C0DE\""]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Read enable command"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<CmdSpec> {
        RdW::new(self, 0)
    }
    #[doc = "Bit 1 - Write enable command"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WrW<CmdSpec> {
        WrW::new(self, 1)
    }
    #[doc = "Bit 2 - Erase sector enable command"]
    #[inline(always)]
    #[must_use]
    pub fn ersec(&mut self) -> ErsecW<CmdSpec> {
        ErsecW::new(self, 2)
    }
    #[doc = "Bit 3 - Erase all enable command"]
    #[inline(always)]
    #[must_use]
    pub fn erall(&mut self) -> ErallW<CmdSpec> {
        ErallW::new(self, 3)
    }
    #[doc = "Bit 8 - NVR access bit"]
    #[inline(always)]
    #[must_use]
    pub fn nvron(&mut self) -> NvronW<CmdSpec> {
        NvronW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Magic Key for flash access \"C0DE\""]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CmdSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
