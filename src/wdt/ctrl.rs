#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `INTEN` reader - Enable the interrupt event"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Enable the interrupt event"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEN` reader - Enable watchdog reset output"]
pub type ResenR = crate::BitReader;
#[doc = "Field `RESEN` writer - Enable watchdog reset output"]
pub type ResenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the interrupt event"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    pub fn resen(&self) -> ResenR {
        ResenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt event"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<CtrlSpec> {
        IntenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    #[must_use]
    pub fn resen(&mut self) -> ResenW<CtrlSpec> {
        ResenW::new(self, 1)
    }
}
#[doc = "Watchdog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
