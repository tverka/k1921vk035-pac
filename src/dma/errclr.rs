#[doc = "Register `ERRCLR` reader"]
pub type R = crate::R<ErrclrSpec>;
#[doc = "Register `ERRCLR` writer"]
pub type W = crate::W<ErrclrSpec>;
#[doc = "Field `VAL` reader - Indicate Error on bus AHB-Lite"]
pub type ValR = crate::BitReader;
#[doc = "Field `VAL` writer - Indicate Error on bus AHB-Lite"]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicate Error on bus AHB-Lite"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicate Error on bus AHB-Lite"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<ErrclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Bus error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrclrSpec;
impl crate::RegisterSpec for ErrclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errclr::R`](R) reader structure"]
impl crate::Readable for ErrclrSpec {}
#[doc = "`write(|w| ..)` method takes [`errclr::W`](W) writer structure"]
impl crate::Writable for ErrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRCLR to value 0"]
impl crate::Resettable for ErrclrSpec {
    const RESET_VALUE: u32 = 0;
}
