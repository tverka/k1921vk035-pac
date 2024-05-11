#[doc = "Register `AQSFRC` reader"]
pub type R = crate::R<AqsfrcSpec>;
#[doc = "Register `AQSFRC` writer"]
pub type W = crate::W<AqsfrcSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actsfa {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Actsfa> for u8 {
    #[inline(always)]
    fn from(variant: Actsfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actsfa {
    type Ux = u8;
}
impl crate::IsEnum for Actsfa {}
#[doc = "Field `ACTSFA` reader - "]
pub type ActsfaR = crate::FieldReader<Actsfa>;
impl ActsfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actsfa {
        match self.bits {
            0 => Actsfa::NoAction,
            1 => Actsfa::Clear,
            2 => Actsfa::Set,
            3 => Actsfa::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Actsfa::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Actsfa::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Actsfa::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Actsfa::Toggle
    }
}
#[doc = "Field `ACTSFA` writer - "]
pub type ActsfaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Actsfa, crate::Safe>;
impl<'a, REG> ActsfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfa::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfa::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfa::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfa::Toggle)
    }
}
#[doc = "Field `OTSFA` reader - One-time software forced event on output A"]
pub type OtsfaR = crate::BitReader;
#[doc = "Field `OTSFA` writer - One-time software forced event on output A"]
pub type OtsfaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actsfb {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Actsfb> for u8 {
    #[inline(always)]
    fn from(variant: Actsfb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actsfb {
    type Ux = u8;
}
impl crate::IsEnum for Actsfb {}
#[doc = "Field `ACTSFB` reader - "]
pub type ActsfbR = crate::FieldReader<Actsfb>;
impl ActsfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actsfb {
        match self.bits {
            0 => Actsfb::NoAction,
            1 => Actsfb::Clear,
            2 => Actsfb::Set,
            3 => Actsfb::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Actsfb::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Actsfb::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Actsfb::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Actsfb::Toggle
    }
}
#[doc = "Field `ACTSFB` writer - "]
pub type ActsfbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Actsfb, crate::Safe>;
impl<'a, REG> ActsfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfb::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfb::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfb::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Actsfb::Toggle)
    }
}
#[doc = "Field `OTSFB` reader - One-time software forced event on output B"]
pub type OtsfbR = crate::BitReader;
#[doc = "Field `OTSFB` writer - One-time software forced event on output B"]
pub type OtsfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rldcsf {
    #[doc = "0: load when CTR = 0"]
    CtreqZero = 0,
    #[doc = "1: load when CTR = PRD"]
    CtreqPrd = 1,
    #[doc = "2: load when CTR = 0 or CTR = PRD"]
    CtreqZeroPrd = 2,
    #[doc = "3: load immediatelly"]
    NoShadow = 3,
}
impl From<Rldcsf> for u8 {
    #[inline(always)]
    fn from(variant: Rldcsf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rldcsf {
    type Ux = u8;
}
impl crate::IsEnum for Rldcsf {}
#[doc = "Field `RLDCSF` reader - "]
pub type RldcsfR = crate::FieldReader<Rldcsf>;
impl RldcsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rldcsf {
        match self.bits {
            0 => Rldcsf::CtreqZero,
            1 => Rldcsf::CtreqPrd,
            2 => Rldcsf::CtreqZeroPrd,
            3 => Rldcsf::NoShadow,
            _ => unreachable!(),
        }
    }
    #[doc = "load when CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Rldcsf::CtreqZero
    }
    #[doc = "load when CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Rldcsf::CtreqPrd
    }
    #[doc = "load when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_zero_prd(&self) -> bool {
        *self == Rldcsf::CtreqZeroPrd
    }
    #[doc = "load immediatelly"]
    #[inline(always)]
    pub fn is_no_shadow(&self) -> bool {
        *self == Rldcsf::NoShadow
    }
}
#[doc = "Field `RLDCSF` writer - "]
pub type RldcsfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rldcsf, crate::Safe>;
impl<'a, REG> RldcsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "load when CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Rldcsf::CtreqZero)
    }
    #[doc = "load when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Rldcsf::CtreqPrd)
    }
    #[doc = "load when CTR = 0 or CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_zero_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Rldcsf::CtreqZeroPrd)
    }
    #[doc = "load immediatelly"]
    #[inline(always)]
    pub fn no_shadow(self) -> &'a mut crate::W<REG> {
        self.variant(Rldcsf::NoShadow)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn actsfa(&self) -> ActsfaR {
        ActsfaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - One-time software forced event on output A"]
    #[inline(always)]
    pub fn otsfa(&self) -> OtsfaR {
        OtsfaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn actsfb(&self) -> ActsfbR {
        ActsfbR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - One-time software forced event on output B"]
    #[inline(always)]
    pub fn otsfb(&self) -> OtsfbR {
        OtsfbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rldcsf(&self) -> RldcsfR {
        RldcsfR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn actsfa(&mut self) -> ActsfaW<AqsfrcSpec> {
        ActsfaW::new(self, 0)
    }
    #[doc = "Bit 2 - One-time software forced event on output A"]
    #[inline(always)]
    #[must_use]
    pub fn otsfa(&mut self) -> OtsfaW<AqsfrcSpec> {
        OtsfaW::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn actsfb(&mut self) -> ActsfbW<AqsfrcSpec> {
        ActsfbW::new(self, 3)
    }
    #[doc = "Bit 5 - One-time software forced event on output B"]
    #[inline(always)]
    #[must_use]
    pub fn otsfb(&mut self) -> OtsfbW<AqsfrcSpec> {
        OtsfbW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn rldcsf(&mut self) -> RldcsfW<AqsfrcSpec> {
        RldcsfW::new(self, 6)
    }
}
#[doc = "Action-Qualifier Software Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aqsfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aqsfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqsfrcSpec;
impl crate::RegisterSpec for AqsfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqsfrc::R`](R) reader structure"]
impl crate::Readable for AqsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`aqsfrc::W`](W) writer structure"]
impl crate::Writable for AqsfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AQSFRC to value 0"]
impl crate::Resettable for AqsfrcSpec {
    const RESET_VALUE: u32 = 0;
}
