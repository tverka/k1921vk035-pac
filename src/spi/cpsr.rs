#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CpsrSpec>;
#[doc = "Register `CPSR` writer"]
pub type W = crate::W<CpsrSpec>;
#[doc = "Field `CPSDVSR` reader - "]
pub type CpsdvsrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Clock division factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsrSpec;
impl crate::RegisterSpec for CpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsr::W`](W) writer structure"]
impl crate::Writable for CpsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CpsrSpec {
    const RESET_VALUE: u32 = 0;
}
