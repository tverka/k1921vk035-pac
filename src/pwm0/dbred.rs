#[doc = "Register `DBRED` reader"]
pub type R = crate::R<DbredSpec>;
#[doc = "Register `DBRED` writer"]
pub type W = crate::W<DbredSpec>;
#[doc = "Field `DEL` reader - "]
pub type DelR = crate::FieldReader<u16>;
#[doc = "Field `DEL` writer - "]
pub type DelW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn del(&self) -> DelR {
        DelR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn del(&mut self) -> DelW<DbredSpec> {
        DelW::new(self, 0)
    }
}
#[doc = "Dead-Band Generator Rising Edge Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbred::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbred::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbredSpec;
impl crate::RegisterSpec for DbredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbred::R`](R) reader structure"]
impl crate::Readable for DbredSpec {}
#[doc = "`write(|w| ..)` method takes [`dbred::W`](W) writer structure"]
impl crate::Writable for DbredSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBRED to value 0"]
impl crate::Resettable for DbredSpec {
    const RESET_VALUE: u32 = 0;
}
