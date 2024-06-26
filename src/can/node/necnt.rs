#[doc = "Register `NECNT` reader"]
pub type R = crate::R<NecntSpec>;
#[doc = "Register `NECNT` writer"]
pub type W = crate::W<NecntSpec>;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type RecR = crate::FieldReader;
#[doc = "Field `REC` writer - Receive Error Counter"]
pub type RecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `TEC` writer - Transmit Error Counter"]
pub type TecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EWRNLVL` reader - Error Warning Level"]
pub type EwrnlvlR = crate::FieldReader;
#[doc = "Field `EWRNLVL` writer - Error Warning Level"]
pub type EwrnlvlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LETD` reader - Last Error Transfer Direction"]
pub type LetdR = crate::BitReader;
#[doc = "Field `LEINC` reader - Last Error Increment"]
pub type LeincR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Error Warning Level"]
    #[inline(always)]
    pub fn ewrnlvl(&self) -> EwrnlvlR {
        EwrnlvlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Last Error Transfer Direction"]
    #[inline(always)]
    pub fn letd(&self) -> LetdR {
        LetdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Last Error Increment"]
    #[inline(always)]
    pub fn leinc(&self) -> LeincR {
        LeincR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> RecW<NecntSpec> {
        RecW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmit Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TecW<NecntSpec> {
        TecW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Error Warning Level"]
    #[inline(always)]
    #[must_use]
    pub fn ewrnlvl(&mut self) -> EwrnlvlW<NecntSpec> {
        EwrnlvlW::new(self, 16)
    }
}
#[doc = "Node Error Counter Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`necnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`necnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NecntSpec;
impl crate::RegisterSpec for NecntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`necnt::R`](R) reader structure"]
impl crate::Readable for NecntSpec {}
#[doc = "`write(|w| ..)` method takes [`necnt::W`](W) writer structure"]
impl crate::Writable for NecntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NECNT to value 0"]
impl crate::Resettable for NecntSpec {
    const RESET_VALUE: u32 = 0;
}
