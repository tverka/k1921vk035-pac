#[doc = "Register `ETPS` reader"]
pub type R = crate::R<EtpsSpec>;
#[doc = "Register `ETPS` writer"]
pub type W = crate::W<EtpsSpec>;
#[doc = "Field `INTPRD` reader - "]
pub type IntprdR = crate::FieldReader;
#[doc = "Field `INTPRD` writer - "]
pub type IntprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTCNT` reader - "]
pub type IntcntR = crate::FieldReader;
#[doc = "Field `SOCAPRD` reader - "]
pub type SocaprdR = crate::FieldReader;
#[doc = "Field `SOCAPRD` writer - "]
pub type SocaprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOCACNT` reader - "]
pub type SocacntR = crate::FieldReader;
#[doc = "Field `SOCBPRD` reader - "]
pub type SocbprdR = crate::FieldReader;
#[doc = "Field `SOCBPRD` writer - "]
pub type SocbprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOCBCNT` reader - "]
pub type SocbcntR = crate::FieldReader;
#[doc = "Field `DRQAPRD` reader - "]
pub type DrqaprdR = crate::FieldReader;
#[doc = "Field `DRQAPRD` writer - "]
pub type DrqaprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRQACNT` reader - "]
pub type DrqacntR = crate::FieldReader;
#[doc = "Field `DRQBPRD` reader - "]
pub type DrqbprdR = crate::FieldReader;
#[doc = "Field `DRQBPRD` writer - "]
pub type DrqbprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRQBCNT` reader - "]
pub type DrqbcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn intprd(&self) -> IntprdR {
        IntprdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn intcnt(&self) -> IntcntR {
        IntcntR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn socaprd(&self) -> SocaprdR {
        SocaprdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn socacnt(&self) -> SocacntR {
        SocacntR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn socbprd(&self) -> SocbprdR {
        SocbprdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn socbcnt(&self) -> SocbcntR {
        SocbcntR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn drqaprd(&self) -> DrqaprdR {
        DrqaprdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn drqacnt(&self) -> DrqacntR {
        DrqacntR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn drqbprd(&self) -> DrqbprdR {
        DrqbprdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn drqbcnt(&self) -> DrqbcntR {
        DrqbcntR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn intprd(&mut self) -> IntprdW<EtpsSpec> {
        IntprdW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn socaprd(&mut self) -> SocaprdW<EtpsSpec> {
        SocaprdW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn socbprd(&mut self) -> SocbprdW<EtpsSpec> {
        SocbprdW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn drqaprd(&mut self) -> DrqaprdW<EtpsSpec> {
        DrqaprdW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn drqbprd(&mut self) -> DrqbprdW<EtpsSpec> {
        DrqbprdW::new(self, 20)
    }
}
#[doc = "Event-Trigger Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtpsSpec;
impl crate::RegisterSpec for EtpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etps::R`](R) reader structure"]
impl crate::Readable for EtpsSpec {}
#[doc = "`write(|w| ..)` method takes [`etps::W`](W) writer structure"]
impl crate::Writable for EtpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETPS to value 0"]
impl crate::Resettable for EtpsSpec {
    const RESET_VALUE: u32 = 0;
}
