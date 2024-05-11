#[doc = "Register `REMAPAF` reader"]
pub type R = crate::R<RemapafSpec>;
#[doc = "Register `REMAPAF` writer"]
pub type W = crate::W<RemapafSpec>;
#[doc = "Field `QEPEN` reader - Enable QEP altfunc"]
pub type QepenR = crate::BitReader;
#[doc = "Field `QEPEN` writer - Enable QEP altfunc"]
pub type QepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP0EN` reader - Enable ECAP0 altfunc"]
pub type Ecap0enR = crate::BitReader;
#[doc = "Field `ECAP0EN` writer - Enable ECAP0 altfunc"]
pub type Ecap0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP1EN` reader - Enable ECAP1 altfunc"]
pub type Ecap1enR = crate::BitReader;
#[doc = "Field `ECAP1EN` writer - Enable ECAP1 altfunc"]
pub type Ecap1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP2EN` reader - Enable ECAP2 altfunc"]
pub type Ecap2enR = crate::BitReader;
#[doc = "Field `ECAP2EN` writer - Enable ECAP2 altfunc"]
pub type Ecap2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable QEP altfunc"]
    #[inline(always)]
    pub fn qepen(&self) -> QepenR {
        QepenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ECAP0 altfunc"]
    #[inline(always)]
    pub fn ecap0en(&self) -> Ecap0enR {
        Ecap0enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable ECAP1 altfunc"]
    #[inline(always)]
    pub fn ecap1en(&self) -> Ecap1enR {
        Ecap1enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable ECAP2 altfunc"]
    #[inline(always)]
    pub fn ecap2en(&self) -> Ecap2enR {
        Ecap2enR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable QEP altfunc"]
    #[inline(always)]
    #[must_use]
    pub fn qepen(&mut self) -> QepenW<RemapafSpec> {
        QepenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ECAP0 altfunc"]
    #[inline(always)]
    #[must_use]
    pub fn ecap0en(&mut self) -> Ecap0enW<RemapafSpec> {
        Ecap0enW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable ECAP1 altfunc"]
    #[inline(always)]
    #[must_use]
    pub fn ecap1en(&mut self) -> Ecap1enW<RemapafSpec> {
        Ecap1enW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable ECAP2 altfunc"]
    #[inline(always)]
    #[must_use]
    pub fn ecap2en(&mut self) -> Ecap2enW<RemapafSpec> {
        Ecap2enW::new(self, 3)
    }
}
#[doc = "QEP altfunc control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remapaf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remapaf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemapafSpec;
impl crate::RegisterSpec for RemapafSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remapaf::R`](R) reader structure"]
impl crate::Readable for RemapafSpec {}
#[doc = "`write(|w| ..)` method takes [`remapaf::W`](W) writer structure"]
impl crate::Writable for RemapafSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAPAF to value 0"]
impl crate::Resettable for RemapafSpec {
    const RESET_VALUE: u32 = 0;
}
