#[doc = "Register `BDIS` reader"]
pub type R = crate::R<BdisSpec>;
#[doc = "Register `BDIS` writer"]
pub type W = crate::W<BdisSpec>;
#[doc = "Field `BMDIS` reader - Disable boot mode after system reset command"]
pub type BmdisR = crate::BitReader;
#[doc = "Field `BMDIS` writer - Disable boot mode after system reset command"]
pub type BmdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable boot mode after system reset command"]
    #[inline(always)]
    pub fn bmdis(&self) -> BmdisR {
        BmdisR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable boot mode after system reset command"]
    #[inline(always)]
    #[must_use]
    pub fn bmdis(&mut self) -> BmdisW<BdisSpec> {
        BmdisW::new(self, 0)
    }
}
#[doc = "Boot Mode Disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdisSpec;
impl crate::RegisterSpec for BdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdis::R`](R) reader structure"]
impl crate::Readable for BdisSpec {}
#[doc = "`write(|w| ..)` method takes [`bdis::W`](W) writer structure"]
impl crate::Writable for BdisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDIS to value 0"]
impl crate::Resettable for BdisSpec {
    const RESET_VALUE: u32 = 0;
}
