#[doc = "Register `PLLDIV` reader"]
pub type R = crate::R<PlldivSpec>;
#[doc = "Register `PLLDIV` writer"]
pub type W = crate::W<PlldivSpec>;
#[doc = "Field `DIVEN` reader - PLL Divider enable bit"]
pub type DivenR = crate::BitReader;
#[doc = "Field `DIVEN` writer - PLL Divider enable bit"]
pub type DivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - PLL divider coefficent"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - PLL divider coefficent"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - PLL Divider enable bit"]
    #[inline(always)]
    pub fn diven(&self) -> DivenR {
        DivenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - PLL divider coefficent"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Divider enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn diven(&mut self) -> DivenW<PlldivSpec> {
        DivenW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PLL divider coefficent"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<PlldivSpec> {
        DivW::new(self, 8)
    }
}
#[doc = "PLL divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plldiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plldiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldivSpec;
impl crate::RegisterSpec for PlldivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldiv::R`](R) reader structure"]
impl crate::Readable for PlldivSpec {}
#[doc = "`write(|w| ..)` method takes [`plldiv::W`](W) writer structure"]
impl crate::Writable for PlldivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIV to value 0"]
impl crate::Resettable for PlldivSpec {
    const RESET_VALUE: u32 = 0;
}
