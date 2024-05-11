#[doc = "Register `CMPCTL` reader"]
pub type R = crate::R<CmpctlSpec>;
#[doc = "Register `CMPCTL` writer"]
pub type W = crate::W<CmpctlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Loadamode {
    #[doc = "0: shadow load for CMPx (x=A,B) when CTR = 0"]
    CtreqZero = 0,
    #[doc = "1: shadow load for CMPx (x=A,B) when CTR = PRD"]
    CtreqPrd = 1,
    #[doc = "2: shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    CtreqZeroPrd = 2,
    #[doc = "3: shadow load for CMPx (x=A,B) disabled"]
    Disable = 3,
}
impl From<Loadamode> for u8 {
    #[inline(always)]
    fn from(variant: Loadamode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Loadamode {
    type Ux = u8;
}
impl crate::IsEnum for Loadamode {}
#[doc = "Field `LOADAMODE` reader - "]
pub type LoadamodeR = crate::FieldReader<Loadamode>;
impl LoadamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loadamode {
        match self.bits {
            0 => Loadamode::CtreqZero,
            1 => Loadamode::CtreqPrd,
            2 => Loadamode::CtreqZeroPrd,
            3 => Loadamode::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Loadamode::CtreqZero
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Loadamode::CtreqPrd
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_zero_prd(&self) -> bool {
        *self == Loadamode::CtreqZeroPrd
    }
    #[doc = "shadow load for CMPx (x=A,B) disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Loadamode::Disable
    }
}
#[doc = "Field `LOADAMODE` writer - "]
pub type LoadamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Loadamode, crate::Safe>;
impl<'a, REG> LoadamodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Loadamode::CtreqZero)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Loadamode::CtreqPrd)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_zero_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Loadamode::CtreqZeroPrd)
    }
    #[doc = "shadow load for CMPx (x=A,B) disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Loadamode::Disable)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Loadbmode {
    #[doc = "0: shadow load for CMPx (x=A,B) when CTR = 0"]
    CtreqZero = 0,
    #[doc = "1: shadow load for CMPx (x=A,B) when CTR = PRD"]
    CtreqPrd = 1,
    #[doc = "2: shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    CtreqZeroPrd = 2,
    #[doc = "3: shadow load for CMPx (x=A,B) disabled"]
    Disable = 3,
}
impl From<Loadbmode> for u8 {
    #[inline(always)]
    fn from(variant: Loadbmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Loadbmode {
    type Ux = u8;
}
impl crate::IsEnum for Loadbmode {}
#[doc = "Field `LOADBMODE` reader - "]
pub type LoadbmodeR = crate::FieldReader<Loadbmode>;
impl LoadbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loadbmode {
        match self.bits {
            0 => Loadbmode::CtreqZero,
            1 => Loadbmode::CtreqPrd,
            2 => Loadbmode::CtreqZeroPrd,
            3 => Loadbmode::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Loadbmode::CtreqZero
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Loadbmode::CtreqPrd
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_zero_prd(&self) -> bool {
        *self == Loadbmode::CtreqZeroPrd
    }
    #[doc = "shadow load for CMPx (x=A,B) disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Loadbmode::Disable
    }
}
#[doc = "Field `LOADBMODE` writer - "]
pub type LoadbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Loadbmode, crate::Safe>;
impl<'a, REG> LoadbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Loadbmode::CtreqZero)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Loadbmode::CtreqPrd)
    }
    #[doc = "shadow load for CMPx (x=A,B) when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_zero_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Loadbmode::CtreqZeroPrd)
    }
    #[doc = "shadow load for CMPx (x=A,B) disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Loadbmode::Disable)
    }
}
#[doc = "Field `SHDWAMODE` reader - CMPA register operating mode"]
pub type ShdwamodeR = crate::BitReader;
#[doc = "Field `SHDWAMODE` writer - CMPA register operating mode"]
pub type ShdwamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWBMODE` reader - CMPB register operating mode"]
pub type ShdwbmodeR = crate::BitReader;
#[doc = "Field `SHDWBMODE` writer - CMPB register operating mode"]
pub type ShdwbmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHDWAFULL` reader - CMPA shadow register full status flag"]
pub type ShdwafullR = crate::BitReader;
#[doc = "Field `SHDWBFULL` reader - CMPB shadow register full status flag"]
pub type ShdwbfullR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn loadamode(&self) -> LoadamodeR {
        LoadamodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn loadbmode(&self) -> LoadbmodeR {
        LoadbmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - CMPA register operating mode"]
    #[inline(always)]
    pub fn shdwamode(&self) -> ShdwamodeR {
        ShdwamodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CMPB register operating mode"]
    #[inline(always)]
    pub fn shdwbmode(&self) -> ShdwbmodeR {
        ShdwbmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CMPA shadow register full status flag"]
    #[inline(always)]
    pub fn shdwafull(&self) -> ShdwafullR {
        ShdwafullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMPB shadow register full status flag"]
    #[inline(always)]
    pub fn shdwbfull(&self) -> ShdwbfullR {
        ShdwbfullR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn loadamode(&mut self) -> LoadamodeW<CmpctlSpec> {
        LoadamodeW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn loadbmode(&mut self) -> LoadbmodeW<CmpctlSpec> {
        LoadbmodeW::new(self, 2)
    }
    #[doc = "Bit 4 - CMPA register operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn shdwamode(&mut self) -> ShdwamodeW<CmpctlSpec> {
        ShdwamodeW::new(self, 4)
    }
    #[doc = "Bit 6 - CMPB register operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn shdwbmode(&mut self) -> ShdwbmodeW<CmpctlSpec> {
        ShdwbmodeW::new(self, 6)
    }
}
#[doc = "Counter-Compare Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpctlSpec;
impl crate::RegisterSpec for CmpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpctl::R`](R) reader structure"]
impl crate::Readable for CmpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpctl::W`](W) writer structure"]
impl crate::Writable for CmpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CmpctlSpec {
    const RESET_VALUE: u32 = 0;
}
