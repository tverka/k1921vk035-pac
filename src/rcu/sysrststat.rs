#[doc = "Register `SYSRSTSTAT` reader"]
pub type R = crate::R<SysrststatSpec>;
#[doc = "Register `SYSRSTSTAT` writer"]
pub type W = crate::W<SysrststatSpec>;
#[doc = "Field `POR` reader - PowerOn Reset status"]
pub type PorR = crate::BitReader;
#[doc = "Field `POR` writer - PowerOn Reset status"]
pub type PorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG` reader - WatchDog Reset status"]
pub type WdogR = crate::BitReader;
#[doc = "Field `WDOG` writer - WatchDog Reset status"]
pub type WdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRST` reader - System Reset Status"]
pub type SysrstR = crate::BitReader;
#[doc = "Field `SYSRST` writer - System Reset Status"]
pub type SysrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP` reader - Lockup Reset Status"]
pub type LockupR = crate::BitReader;
#[doc = "Field `LOCKUP` writer - Lockup Reset Status"]
pub type LockupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PowerOn Reset status"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WatchDog Reset status"]
    #[inline(always)]
    pub fn wdog(&self) -> WdogR {
        WdogR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Reset Status"]
    #[inline(always)]
    pub fn sysrst(&self) -> SysrstR {
        SysrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lockup Reset Status"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PowerOn Reset status"]
    #[inline(always)]
    #[must_use]
    pub fn por(&mut self) -> PorW<SysrststatSpec> {
        PorW::new(self, 0)
    }
    #[doc = "Bit 1 - WatchDog Reset status"]
    #[inline(always)]
    #[must_use]
    pub fn wdog(&mut self) -> WdogW<SysrststatSpec> {
        WdogW::new(self, 1)
    }
    #[doc = "Bit 2 - System Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysrst(&mut self) -> SysrstW<SysrststatSpec> {
        SysrstW::new(self, 2)
    }
    #[doc = "Bit 3 - Lockup Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LockupW<SysrststatSpec> {
        LockupW::new(self, 3)
    }
}
#[doc = "Reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysrststat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysrststat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysrststatSpec;
impl crate::RegisterSpec for SysrststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysrststat::R`](R) reader structure"]
impl crate::Readable for SysrststatSpec {}
#[doc = "`write(|w| ..)` method takes [`sysrststat::W`](W) writer structure"]
impl crate::Writable for SysrststatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSRSTSTAT to value 0"]
impl crate::Resettable for SysrststatSpec {
    const RESET_VALUE: u32 = 0;
}
