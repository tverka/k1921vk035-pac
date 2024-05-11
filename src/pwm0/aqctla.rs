#[doc = "Register `AQCTLA` reader"]
pub type R = crate::R<AqctlaSpec>;
#[doc = "Register `AQCTLA` writer"]
pub type W = crate::W<AqctlaSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Zro {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Zro> for u8 {
    #[inline(always)]
    fn from(variant: Zro) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Zro {
    type Ux = u8;
}
impl crate::IsEnum for Zro {}
#[doc = "Field `ZRO` reader - "]
pub type ZroR = crate::FieldReader<Zro>;
impl ZroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zro {
        match self.bits {
            0 => Zro::NoAction,
            1 => Zro::Clear,
            2 => Zro::Set,
            3 => Zro::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Zro::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Zro::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Zro::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Zro::Toggle
    }
}
#[doc = "Field `ZRO` writer - "]
pub type ZroW<'a, REG> = crate::FieldWriter<'a, REG, 2, Zro, crate::Safe>;
impl<'a, REG> ZroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::Toggle)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prd {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Prd> for u8 {
    #[inline(always)]
    fn from(variant: Prd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prd {
    type Ux = u8;
}
impl crate::IsEnum for Prd {}
#[doc = "Field `PRD` reader - "]
pub type PrdR = crate::FieldReader<Prd>;
impl PrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prd {
        match self.bits {
            0 => Prd::NoAction,
            1 => Prd::Clear,
            2 => Prd::Set,
            3 => Prd::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Prd::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Prd::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Prd::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Prd::Toggle
    }
}
#[doc = "Field `PRD` writer - "]
pub type PrdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prd, crate::Safe>;
impl<'a, REG> PrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::Toggle)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cau {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cau> for u8 {
    #[inline(always)]
    fn from(variant: Cau) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cau {
    type Ux = u8;
}
impl crate::IsEnum for Cau {}
#[doc = "Field `CAU` reader - "]
pub type CauR = crate::FieldReader<Cau>;
impl CauR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cau {
        match self.bits {
            0 => Cau::NoAction,
            1 => Cau::Clear,
            2 => Cau::Set,
            3 => Cau::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cau::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cau::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cau::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cau::Toggle
    }
}
#[doc = "Field `CAU` writer - "]
pub type CauW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cau, crate::Safe>;
impl<'a, REG> CauW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::Toggle)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cad {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cad> for u8 {
    #[inline(always)]
    fn from(variant: Cad) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cad {
    type Ux = u8;
}
impl crate::IsEnum for Cad {}
#[doc = "Field `CAD` reader - "]
pub type CadR = crate::FieldReader<Cad>;
impl CadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cad {
        match self.bits {
            0 => Cad::NoAction,
            1 => Cad::Clear,
            2 => Cad::Set,
            3 => Cad::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cad::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cad::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cad::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cad::Toggle
    }
}
#[doc = "Field `CAD` writer - "]
pub type CadW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cad, crate::Safe>;
impl<'a, REG> CadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::Toggle)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cbu {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cbu> for u8 {
    #[inline(always)]
    fn from(variant: Cbu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cbu {
    type Ux = u8;
}
impl crate::IsEnum for Cbu {}
#[doc = "Field `CBU` reader - "]
pub type CbuR = crate::FieldReader<Cbu>;
impl CbuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbu {
        match self.bits {
            0 => Cbu::NoAction,
            1 => Cbu::Clear,
            2 => Cbu::Set,
            3 => Cbu::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cbu::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cbu::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cbu::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cbu::Toggle
    }
}
#[doc = "Field `CBU` writer - "]
pub type CbuW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cbu, crate::Safe>;
impl<'a, REG> CbuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::Toggle)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cbd {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cbd> for u8 {
    #[inline(always)]
    fn from(variant: Cbd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cbd {
    type Ux = u8;
}
impl crate::IsEnum for Cbd {}
#[doc = "Field `CBD` reader - "]
pub type CbdR = crate::FieldReader<Cbd>;
impl CbdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbd {
        match self.bits {
            0 => Cbd::NoAction,
            1 => Cbd::Clear,
            2 => Cbd::Set,
            3 => Cbd::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cbd::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cbd::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cbd::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cbd::Toggle
    }
}
#[doc = "Field `CBD` writer - "]
pub type CbdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cbd, crate::Safe>;
impl<'a, REG> CbdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::Toggle)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn zro(&self) -> ZroR {
        ZroR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cau(&self) -> CauR {
        CauR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cad(&self) -> CadR {
        CadR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cbu(&self) -> CbuR {
        CbuR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn cbd(&self) -> CbdR {
        CbdR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn zro(&mut self) -> ZroW<AqctlaSpec> {
        ZroW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn prd(&mut self) -> PrdW<AqctlaSpec> {
        PrdW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn cau(&mut self) -> CauW<AqctlaSpec> {
        CauW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn cad(&mut self) -> CadW<AqctlaSpec> {
        CadW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn cbu(&mut self) -> CbuW<AqctlaSpec> {
        CbuW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn cbd(&mut self) -> CbdW<AqctlaSpec> {
        CbdW::new(self, 10)
    }
}
#[doc = "Action-Qualifier Output A Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aqctla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aqctla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqctlaSpec;
impl crate::RegisterSpec for AqctlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqctla::R`](R) reader structure"]
impl crate::Readable for AqctlaSpec {}
#[doc = "`write(|w| ..)` method takes [`aqctla::W`](W) writer structure"]
impl crate::Writable for AqctlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AQCTLA to value 0"]
impl crate::Resettable for AqctlaSpec {
    const RESET_VALUE: u32 = 0;
}
