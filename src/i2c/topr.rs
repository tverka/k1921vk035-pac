#[doc = "Register `TOPR` reader"]
pub type R = crate::R<ToprSpec>;
#[doc = "Register `TOPR` writer"]
pub type W = crate::W<ToprSpec>;
#[doc = "Field `SMBTOPR` reader - "]
pub type SmbtoprR = crate::FieldReader;
#[doc = "Field `SMBTOPR` writer - "]
pub type SmbtoprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn smbtopr(&self) -> SmbtoprR {
        SmbtoprR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn smbtopr(&mut self) -> SmbtoprW<ToprSpec> {
        SmbtoprW::new(self, 0)
    }
}
#[doc = "Prescaler load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`topr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`topr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToprSpec;
impl crate::RegisterSpec for ToprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`topr::R`](R) reader structure"]
impl crate::Readable for ToprSpec {}
#[doc = "`write(|w| ..)` method takes [`topr::W`](W) writer structure"]
impl crate::Writable for ToprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOPR to value 0"]
impl crate::Resettable for ToprSpec {
    const RESET_VALUE: u32 = 0;
}
