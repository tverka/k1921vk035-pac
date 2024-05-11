#[doc = "Register `Reserved` reader"]
pub type R = crate::R<ReservedSpec>;
#[doc = "Register `Reserved` writer"]
pub type W = crate::W<ReservedSpec>;
impl W {}
#[doc = "No description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReservedSpec;
impl crate::RegisterSpec for ReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved::R`](R) reader structure"]
impl crate::Readable for ReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`reserved::W`](W) writer structure"]
impl crate::Writable for ReservedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Reserved to value 0"]
impl crate::Resettable for ReservedSpec {
    const RESET_VALUE: u32 = 0;
}
