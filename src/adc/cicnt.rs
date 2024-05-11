#[doc = "Register `CICNT` reader"]
pub type R = crate::R<CicntSpec>;
#[doc = "Register `CICNT` writer"]
pub type W = crate::W<CicntSpec>;
#[doc = "Field `ICNT0` reader - Clear interrupt counter on sequencer 0 start"]
pub type Icnt0R = crate::BitReader;
#[doc = "Field `ICNT0` writer - Clear interrupt counter on sequencer 0 start"]
pub type Icnt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT1` reader - Clear interrupt counter on sequencer 1 start"]
pub type Icnt1R = crate::BitReader;
#[doc = "Field `ICNT1` writer - Clear interrupt counter on sequencer 1 start"]
pub type Icnt1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear interrupt counter on sequencer 0 start"]
    #[inline(always)]
    pub fn icnt0(&self) -> Icnt0R {
        Icnt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear interrupt counter on sequencer 1 start"]
    #[inline(always)]
    pub fn icnt1(&self) -> Icnt1R {
        Icnt1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear interrupt counter on sequencer 0 start"]
    #[inline(always)]
    #[must_use]
    pub fn icnt0(&mut self) -> Icnt0W<CicntSpec> {
        Icnt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear interrupt counter on sequencer 1 start"]
    #[inline(always)]
    #[must_use]
    pub fn icnt1(&mut self) -> Icnt1W<CicntSpec> {
        Icnt1W::new(self, 1)
    }
}
#[doc = "Interrupt counter clear control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cicnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicntSpec;
impl crate::RegisterSpec for CicntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicnt::R`](R) reader structure"]
impl crate::Readable for CicntSpec {}
#[doc = "`write(|w| ..)` method takes [`cicnt::W`](W) writer structure"]
impl crate::Writable for CicntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CICNT to value 0"]
impl crate::Resettable for CicntSpec {
    const RESET_VALUE: u32 = 0;
}
