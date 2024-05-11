#[doc = "Register `SYSRSTCFG` reader"]
pub type R = crate::R<SysrstcfgSpec>;
#[doc = "Register `SYSRSTCFG` writer"]
pub type W = crate::W<SysrstcfgSpec>;
#[doc = "Field `LOCKUPEN` reader - Enable reset when processor in LOCKUP state"]
pub type LockupenR = crate::BitReader;
#[doc = "Field `LOCKUPEN` writer - Enable reset when processor in LOCKUP state"]
pub type LockupenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable reset when processor in LOCKUP state"]
    #[inline(always)]
    pub fn lockupen(&self) -> LockupenR {
        LockupenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable reset when processor in LOCKUP state"]
    #[inline(always)]
    #[must_use]
    pub fn lockupen(&mut self) -> LockupenW<SysrstcfgSpec> {
        LockupenW::new(self, 0)
    }
}
#[doc = "System reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysrstcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysrstcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysrstcfgSpec;
impl crate::RegisterSpec for SysrstcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysrstcfg::R`](R) reader structure"]
impl crate::Readable for SysrstcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysrstcfg::W`](W) writer structure"]
impl crate::Writable for SysrstcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSRSTCFG to value 0"]
impl crate::Resettable for SysrstcfgSpec {
    const RESET_VALUE: u32 = 0;
}
