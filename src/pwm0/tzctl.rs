#[doc = "Register `TZCTL` reader"]
pub type R = crate::R<TzctlSpec>;
#[doc = "Register `TZCTL` writer"]
pub type W = crate::W<TzctlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tza {
    #[doc = "0: PWMA/PWMB go to Z on failture"]
    Z = 0,
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    Set = 1,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    Clear = 2,
    #[doc = "3: no action on failture"]
    NoAction = 3,
}
impl From<Tza> for u8 {
    #[inline(always)]
    fn from(variant: Tza) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tza {
    type Ux = u8;
}
impl crate::IsEnum for Tza {}
#[doc = "Field `TZA` reader - "]
pub type TzaR = crate::FieldReader<Tza>;
impl TzaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tza {
        match self.bits {
            0 => Tza::Z,
            1 => Tza::Set,
            2 => Tza::Clear,
            3 => Tza::NoAction,
            _ => unreachable!(),
        }
    }
    #[doc = "PWMA/PWMB go to Z on failture"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == Tza::Z
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tza::Set
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Tza::Clear
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Tza::NoAction
    }
}
#[doc = "Field `TZA` writer - "]
pub type TzaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tza, crate::Safe>;
impl<'a, REG> TzaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWMA/PWMB go to Z on failture"]
    #[inline(always)]
    pub fn z(self) -> &'a mut crate::W<REG> {
        self.variant(Tza::Z)
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tza::Set)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tza::Clear)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Tza::NoAction)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tzb {
    #[doc = "0: PWMA/PWMB go to Z on failture"]
    Z = 0,
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    Set = 1,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    Clear = 2,
    #[doc = "3: no action on failture"]
    NoAction = 3,
}
impl From<Tzb> for u8 {
    #[inline(always)]
    fn from(variant: Tzb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tzb {
    type Ux = u8;
}
impl crate::IsEnum for Tzb {}
#[doc = "Field `TZB` reader - "]
pub type TzbR = crate::FieldReader<Tzb>;
impl TzbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tzb {
        match self.bits {
            0 => Tzb::Z,
            1 => Tzb::Set,
            2 => Tzb::Clear,
            3 => Tzb::NoAction,
            _ => unreachable!(),
        }
    }
    #[doc = "PWMA/PWMB go to Z on failture"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == Tzb::Z
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Tzb::Set
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Tzb::Clear
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Tzb::NoAction
    }
}
#[doc = "Field `TZB` writer - "]
pub type TzbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tzb, crate::Safe>;
impl<'a, REG> TzbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWMA/PWMB go to Z on failture"]
    #[inline(always)]
    pub fn z(self) -> &'a mut crate::W<REG> {
        self.variant(Tzb::Z)
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Tzb::Set)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tzb::Clear)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Tzb::NoAction)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tza(&self) -> TzaR {
        TzaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzb(&self) -> TzbR {
        TzbR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tza(&mut self) -> TzaW<TzctlSpec> {
        TzaW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn tzb(&mut self) -> TzbW<TzctlSpec> {
        TzbW::new(self, 2)
    }
}
#[doc = "Trip-Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzctlSpec;
impl crate::RegisterSpec for TzctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzctl::R`](R) reader structure"]
impl crate::Readable for TzctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzctl::W`](W) writer structure"]
impl crate::Writable for TzctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZCTL to value 0"]
impl crate::Resettable for TzctlSpec {
    const RESET_VALUE: u32 = 0;
}
