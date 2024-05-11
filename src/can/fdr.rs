#[doc = "Register `FDR` reader"]
pub type R = crate::R<FdrSpec>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FdrSpec>;
#[doc = "Field `STEP` reader - Step Value"]
pub type StepR = crate::FieldReader<u16>;
#[doc = "Field `STEP` writer - Step Value"]
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SM` reader - Suspend Mode"]
pub type SmR = crate::BitReader;
#[doc = "Field `SM` writer - Suspend Mode"]
pub type SmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC` reader - Suspend Control"]
pub type ScR = crate::FieldReader;
#[doc = "Field `SC` writer - Suspend Control"]
pub type ScW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dm {
    #[doc = "0: counter disabled"]
    Disable = 0,
    #[doc = "1: normal operation mode"]
    NormalMode = 1,
    #[doc = "2: divider operation mode"]
    DividerMode = 2,
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(variant: Dm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dm {
    type Ux = u8;
}
impl crate::IsEnum for Dm {}
#[doc = "Field `DM` reader - "]
pub type DmR = crate::FieldReader<Dm>;
impl DmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dm> {
        match self.bits {
            0 => Some(Dm::Disable),
            1 => Some(Dm::NormalMode),
            2 => Some(Dm::DividerMode),
            _ => None,
        }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dm::Disable
    }
    #[doc = "normal operation mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Dm::NormalMode
    }
    #[doc = "divider operation mode"]
    #[inline(always)]
    pub fn is_divider_mode(&self) -> bool {
        *self == Dm::DividerMode
    }
}
#[doc = "Field `DM` writer - "]
pub type DmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dm>;
impl<'a, REG> DmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::Disable)
    }
    #[doc = "normal operation mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::NormalMode)
    }
    #[doc = "divider operation mode"]
    #[inline(always)]
    pub fn divider_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Dm::DividerMode)
    }
}
#[doc = "Field `RESULT` reader - Result Value"]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `SUSACK` reader - Suspend Mode Acknowledge"]
pub type SusackR = crate::BitReader;
#[doc = "Field `SUSREQ` reader - Suspend Mode Request"]
pub type SusreqR = crate::BitReader;
#[doc = "Field `ENHW` reader - Enable Hardware Clock Control"]
pub type EnhwR = crate::BitReader;
#[doc = "Field `ENHW` writer - Enable Hardware Clock Control"]
pub type EnhwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCLK` reader - Disable Clock"]
pub type DisclkR = crate::BitReader;
#[doc = "Field `DISCLK` writer - Disable Clock"]
pub type DisclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - Suspend Mode"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Suspend Control"]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Suspend Mode Acknowledge"]
    #[inline(always)]
    pub fn susack(&self) -> SusackR {
        SusackR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Suspend Mode Request"]
    #[inline(always)]
    pub fn susreq(&self) -> SusreqR {
        SusreqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Hardware Clock Control"]
    #[inline(always)]
    pub fn enhw(&self) -> EnhwR {
        EnhwR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable Clock"]
    #[inline(always)]
    pub fn disclk(&self) -> DisclkR {
        DisclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> StepW<FdrSpec> {
        StepW::new(self, 0)
    }
    #[doc = "Bit 11 - Suspend Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SmW<FdrSpec> {
        SmW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Suspend Control"]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> ScW<FdrSpec> {
        ScW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DmW<FdrSpec> {
        DmW::new(self, 14)
    }
    #[doc = "Bit 30 - Enable Hardware Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn enhw(&mut self) -> EnhwW<FdrSpec> {
        EnhwW::new(self, 30)
    }
    #[doc = "Bit 31 - Disable Clock"]
    #[inline(always)]
    #[must_use]
    pub fn disclk(&mut self) -> DisclkW<FdrSpec> {
        DisclkW::new(self, 31)
    }
}
#[doc = "Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdrSpec;
impl crate::RegisterSpec for FdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FdrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FdrSpec {
    const RESET_VALUE: u32 = 0;
}
