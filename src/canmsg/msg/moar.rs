#[doc = "Register `MOAR` reader"]
pub type R = crate::R<MoarSpec>;
#[doc = "Register `MOAR` writer"]
pub type W = crate::W<MoarSpec>;
#[doc = "Field `ID` reader - CAN identifier of Message Object"]
pub type IdR = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - CAN identifier of Message Object"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `IDE` reader - Identifier Extension Bit of Messgae Object"]
pub type IdeR = crate::BitReader;
#[doc = "Field `IDE` writer - Identifier Extension Bit of Messgae Object"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRI` reader - Priority Class"]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - Priority Class"]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:28 - CAN identifier of Message Object"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Messgae Object"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - CAN identifier of Message Object"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<MoarSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 29 - Identifier Extension Bit of Messgae Object"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IdeW<MoarSpec> {
        IdeW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Priority Class"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<MoarSpec> {
        PriW::new(self, 30)
    }
}
#[doc = "Message Object Arbitration Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoarSpec;
impl crate::RegisterSpec for MoarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moar::R`](R) reader structure"]
impl crate::Readable for MoarSpec {}
#[doc = "`write(|w| ..)` method takes [`moar::W`](W) writer structure"]
impl crate::Writable for MoarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOAR to value 0"]
impl crate::Resettable for MoarSpec {
    const RESET_VALUE: u32 = 0;
}
