#[doc = "Register `ECCTL0` reader"]
pub type R = crate::R<Ecctl0Spec>;
#[doc = "Register `ECCTL0` writer"]
pub type W = crate::W<Ecctl0Spec>;
#[doc = "Field `CAP0POL` reader - Polarity select for capture 0"]
pub type Cap0polR = crate::BitReader;
#[doc = "Field `CAP0POL` writer - Polarity select for capture 0"]
pub type Cap0polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST0` reader - Reset counter after event 0"]
pub type Ctrrst0R = crate::BitReader;
#[doc = "Field `CTRRST0` writer - Reset counter after event 0"]
pub type Ctrrst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1POL` reader - Polarity select for capture 1"]
pub type Cap1polR = crate::BitReader;
#[doc = "Field `CAP1POL` writer - Polarity select for capture 1"]
pub type Cap1polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST1` reader - Reset counter after event 1"]
pub type Ctrrst1R = crate::BitReader;
#[doc = "Field `CTRRST1` writer - Reset counter after event 1"]
pub type Ctrrst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2POL` reader - Polarity select for capture 2"]
pub type Cap2polR = crate::BitReader;
#[doc = "Field `CAP2POL` writer - Polarity select for capture 2"]
pub type Cap2polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST2` reader - Reset counter after event 2"]
pub type Ctrrst2R = crate::BitReader;
#[doc = "Field `CTRRST2` writer - Reset counter after event 2"]
pub type Ctrrst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP3POL` reader - Polarity select for capture 3"]
pub type Cap3polR = crate::BitReader;
#[doc = "Field `CAP3POL` writer - Polarity select for capture 3"]
pub type Cap3polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRRST3` reader - Reset counter after event 3"]
pub type Ctrrst3R = crate::BitReader;
#[doc = "Field `CTRRST3` writer - Reset counter after event 3"]
pub type Ctrrst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPLDEN` reader - enable capture"]
pub type CapldenR = crate::BitReader;
#[doc = "Field `CAPLDEN` writer - enable capture"]
pub type CapldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALE` reader - "]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - "]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freesoft {
    #[doc = "0: stop timer immedeatelly"]
    Stop = 0,
    #[doc = "1: stop timer when reach zero"]
    StopAtZero = 1,
    #[doc = "2: normal work"]
    Free = 2,
}
impl From<Freesoft> for u8 {
    #[inline(always)]
    fn from(variant: Freesoft) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freesoft {
    type Ux = u8;
}
impl crate::IsEnum for Freesoft {}
#[doc = "Field `FREESOFT` reader - "]
pub type FreesoftR = crate::FieldReader<Freesoft>;
impl FreesoftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freesoft> {
        match self.bits {
            0 => Some(Freesoft::Stop),
            1 => Some(Freesoft::StopAtZero),
            2 => Some(Freesoft::Free),
            _ => None,
        }
    }
    #[doc = "stop timer immedeatelly"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Freesoft::Stop
    }
    #[doc = "stop timer when reach zero"]
    #[inline(always)]
    pub fn is_stop_at_zero(&self) -> bool {
        *self == Freesoft::StopAtZero
    }
    #[doc = "normal work"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == Freesoft::Free
    }
}
#[doc = "Field `FREESOFT` writer - "]
pub type FreesoftW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freesoft>;
impl<'a, REG> FreesoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "stop timer immedeatelly"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::Stop)
    }
    #[doc = "stop timer when reach zero"]
    #[inline(always)]
    pub fn stop_at_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::StopAtZero)
    }
    #[doc = "normal work"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::Free)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity select for capture 0"]
    #[inline(always)]
    pub fn cap0pol(&self) -> Cap0polR {
        Cap0polR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset counter after event 0"]
    #[inline(always)]
    pub fn ctrrst0(&self) -> Ctrrst0R {
        Ctrrst0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Polarity select for capture 1"]
    #[inline(always)]
    pub fn cap1pol(&self) -> Cap1polR {
        Cap1polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset counter after event 1"]
    #[inline(always)]
    pub fn ctrrst1(&self) -> Ctrrst1R {
        Ctrrst1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Polarity select for capture 2"]
    #[inline(always)]
    pub fn cap2pol(&self) -> Cap2polR {
        Cap2polR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset counter after event 2"]
    #[inline(always)]
    pub fn ctrrst2(&self) -> Ctrrst2R {
        Ctrrst2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Polarity select for capture 3"]
    #[inline(always)]
    pub fn cap3pol(&self) -> Cap3polR {
        Cap3polR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset counter after event 3"]
    #[inline(always)]
    pub fn ctrrst3(&self) -> Ctrrst3R {
        Ctrrst3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable capture"]
    #[inline(always)]
    pub fn caplden(&self) -> CapldenR {
        CapldenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn freesoft(&self) -> FreesoftR {
        FreesoftR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity select for capture 0"]
    #[inline(always)]
    #[must_use]
    pub fn cap0pol(&mut self) -> Cap0polW<Ecctl0Spec> {
        Cap0polW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset counter after event 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst0(&mut self) -> Ctrrst0W<Ecctl0Spec> {
        Ctrrst0W::new(self, 1)
    }
    #[doc = "Bit 2 - Polarity select for capture 1"]
    #[inline(always)]
    #[must_use]
    pub fn cap1pol(&mut self) -> Cap1polW<Ecctl0Spec> {
        Cap1polW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset counter after event 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst1(&mut self) -> Ctrrst1W<Ecctl0Spec> {
        Ctrrst1W::new(self, 3)
    }
    #[doc = "Bit 4 - Polarity select for capture 2"]
    #[inline(always)]
    #[must_use]
    pub fn cap2pol(&mut self) -> Cap2polW<Ecctl0Spec> {
        Cap2polW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset counter after event 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst2(&mut self) -> Ctrrst2W<Ecctl0Spec> {
        Ctrrst2W::new(self, 5)
    }
    #[doc = "Bit 6 - Polarity select for capture 3"]
    #[inline(always)]
    #[must_use]
    pub fn cap3pol(&mut self) -> Cap3polW<Ecctl0Spec> {
        Cap3polW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset counter after event 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctrrst3(&mut self) -> Ctrrst3W<Ecctl0Spec> {
        Ctrrst3W::new(self, 7)
    }
    #[doc = "Bit 8 - enable capture"]
    #[inline(always)]
    #[must_use]
    pub fn caplden(&mut self) -> CapldenW<Ecctl0Spec> {
        CapldenW::new(self, 8)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PrescaleW<Ecctl0Spec> {
        PrescaleW::new(self, 9)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn freesoft(&mut self) -> FreesoftW<Ecctl0Spec> {
        FreesoftW::new(self, 14)
    }
}
#[doc = "Capture control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecctl0Spec;
impl crate::RegisterSpec for Ecctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecctl0::R`](R) reader structure"]
impl crate::Readable for Ecctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecctl0::W`](W) writer structure"]
impl crate::Writable for Ecctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCTL0 to value 0"]
impl crate::Resettable for Ecctl0Spec {
    const RESET_VALUE: u32 = 0;
}
