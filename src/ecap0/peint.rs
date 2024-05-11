#[doc = "Register `PEINT` reader"]
pub type R = crate::R<PeintSpec>;
#[doc = "Register `PEINT` writer"]
pub type W = crate::W<PeintSpec>;
#[doc = "Field `PEINT` reader - active interrupt flag"]
pub type PeintR = crate::BitReader;
#[doc = "Field `PEINT` writer - active interrupt flag"]
pub type PeintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - active interrupt flag"]
    #[inline(always)]
    pub fn peint(&self) -> PeintR {
        PeintR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - active interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn peint(&mut self) -> PeintW<PeintSpec> {
        PeintW::new(self, 0)
    }
}
#[doc = "Active interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeintSpec;
impl crate::RegisterSpec for PeintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peint::R`](R) reader structure"]
impl crate::Readable for PeintSpec {}
#[doc = "`write(|w| ..)` method takes [`peint::W`](W) writer structure"]
impl crate::Writable for PeintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEINT to value 0"]
impl crate::Resettable for PeintSpec {
    const RESET_VALUE: u32 = 0;
}
