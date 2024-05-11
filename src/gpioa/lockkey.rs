#[doc = "Register `LOCKKEY` writer"]
pub type W = crate::W<LockkeySpec>;
#[doc = "Key to unlock LOCKSET/LOCKCLR registers for write (KEY=0xADEADBEE)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Val {
    #[doc = "4008636142: 0xEEEEEEEE, key to lock registers"]
    Lock = 4008636142,
    #[doc = "2917850094: 0xADEADBEE, key to unlock registers"]
    Unlock = 2917850094,
}
impl From<Val> for u32 {
    #[inline(always)]
    fn from(variant: Val) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Val {
    type Ux = u32;
}
impl crate::IsEnum for Val {}
#[doc = "Field `VAL` writer - Key to unlock LOCKSET/LOCKCLR registers for write (KEY=0xADEADBEE)"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, Val>;
impl<'a, REG> ValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "0xEEEEEEEE, key to lock registers"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Lock)
    }
    #[doc = "0xADEADBEE, key to unlock registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Val::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key to unlock LOCKSET/LOCKCLR registers for write (KEY=0xADEADBEE)"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<LockkeySpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Key register to unlock LOCKSET/LOCKCLR registers for write (KEY=0xADEADBEE)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockkeySpec;
impl crate::RegisterSpec for LockkeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lockkey::W`](W) writer structure"]
impl crate::Writable for LockkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKKEY to value 0"]
impl crate::Resettable for LockkeySpec {
    const RESET_VALUE: u32 = 0;
}
