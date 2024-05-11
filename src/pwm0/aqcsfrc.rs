#[doc = "Register `AQCSFRC` reader"]
pub type R = crate::R<AqcsfrcSpec>;
#[doc = "Register `AQCSFRC` writer"]
pub type W = crate::W<AqcsfrcSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csfa {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
}
impl From<Csfa> for u8 {
    #[inline(always)]
    fn from(variant: Csfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csfa {
    type Ux = u8;
}
impl crate::IsEnum for Csfa {}
#[doc = "Field `CSFA` reader - "]
pub type CsfaR = crate::FieldReader<Csfa>;
impl CsfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csfa> {
        match self.bits {
            0 => Some(Csfa::NoAction),
            1 => Some(Csfa::Clear),
            2 => Some(Csfa::Set),
            _ => None,
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Csfa::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Csfa::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Csfa::Set
    }
}
#[doc = "Field `CSFA` writer - "]
pub type CsfaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csfa>;
impl<'a, REG> CsfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Csfa::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Csfa::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Csfa::Set)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csfb {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
}
impl From<Csfb> for u8 {
    #[inline(always)]
    fn from(variant: Csfb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csfb {
    type Ux = u8;
}
impl crate::IsEnum for Csfb {}
#[doc = "Field `CSFB` reader - "]
pub type CsfbR = crate::FieldReader<Csfb>;
impl CsfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csfb> {
        match self.bits {
            0 => Some(Csfb::NoAction),
            1 => Some(Csfb::Clear),
            2 => Some(Csfb::Set),
            _ => None,
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Csfb::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Csfb::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Csfb::Set
    }
}
#[doc = "Field `CSFB` writer - "]
pub type CsfbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csfb>;
impl<'a, REG> CsfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Csfb::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Csfb::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Csfb::Set)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn csfa(&self) -> CsfaR {
        CsfaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn csfb(&self) -> CsfbR {
        CsfbR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn csfa(&mut self) -> CsfaW<AqcsfrcSpec> {
        CsfaW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn csfb(&mut self) -> CsfbW<AqcsfrcSpec> {
        CsfbW::new(self, 2)
    }
}
#[doc = "Action-Qualifier Continuous Software Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aqcsfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aqcsfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqcsfrcSpec;
impl crate::RegisterSpec for AqcsfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqcsfrc::R`](R) reader structure"]
impl crate::Readable for AqcsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`aqcsfrc::W`](W) writer structure"]
impl crate::Writable for AqcsfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AQCSFRC to value 0"]
impl crate::Resettable for AqcsfrcSpec {
    const RESET_VALUE: u32 = 0;
}
