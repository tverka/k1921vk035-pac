#[doc = "Register `SERVCTL` reader"]
pub type R = crate::R<ServctlSpec>;
#[doc = "Register `SERVCTL` writer"]
pub type W = crate::W<ServctlSpec>;
#[doc = "Field `CHIPCLR` reader - On-chip memories full clear task start"]
pub type ChipclrR = crate::BitReader;
#[doc = "Field `CHIPCLR` writer - On-chip memories full clear task start"]
pub type ChipclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - Status of clear task"]
pub type DoneR = crate::BitReader;
#[doc = "Field `SERVEN` reader - "]
pub type ServenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - On-chip memories full clear task start"]
    #[inline(always)]
    pub fn chipclr(&self) -> ChipclrR {
        ChipclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of clear task"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn serven(&self) -> ServenR {
        ServenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - On-chip memories full clear task start"]
    #[inline(always)]
    #[must_use]
    pub fn chipclr(&mut self) -> ChipclrW<ServctlSpec> {
        ChipclrW::new(self, 0)
    }
}
#[doc = "Service mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`servctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`servctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ServctlSpec;
impl crate::RegisterSpec for ServctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`servctl::R`](R) reader structure"]
impl crate::Readable for ServctlSpec {}
#[doc = "`write(|w| ..)` method takes [`servctl::W`](W) writer structure"]
impl crate::Writable for ServctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SERVCTL to value 0"]
impl crate::Resettable for ServctlSpec {
    const RESET_VALUE: u32 = 0;
}
