#[doc = "Register `ECCTL1` reader"]
pub type R = crate::R<Ecctl1Spec>;
#[doc = "Register `ECCTL1` writer"]
pub type W = crate::W<Ecctl1Spec>;
#[doc = "Field `CONTOST` reader - Capture mode"]
pub type ContostR = crate::BitReader;
#[doc = "Field `CONTOST` writer - Capture mode"]
pub type ContostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPWRAP` reader - "]
pub type StopwrapR = crate::FieldReader;
#[doc = "Field `STOPWRAP` writer - "]
pub type StopwrapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REARM` reader - Reset and enable controller, capture reg load"]
pub type RearmR = crate::BitReader;
#[doc = "Field `REARM` writer - Reset and enable controller, capture reg load"]
pub type RearmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRSTOP` reader - Enable Timer"]
pub type TsctrstopR = crate::BitReader;
#[doc = "Field `TSCTRSTOP` writer - Enable Timer"]
pub type TsctrstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCIEN` reader - Sync in enable"]
pub type SyncienR = crate::BitReader;
#[doc = "Field `SYNCIEN` writer - Sync in enable"]
pub type SyncienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syncosel {
    #[doc = "0: sync in connected with sync out"]
    Bypass = 0,
    #[doc = "1: sync out generated when CTR = PRD"]
    CtreqPrd = 1,
    #[doc = "2: sync out generate disabled"]
    Disable = 2,
}
impl From<Syncosel> for u8 {
    #[inline(always)]
    fn from(variant: Syncosel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syncosel {
    type Ux = u8;
}
impl crate::IsEnum for Syncosel {}
#[doc = "Field `SYNCOSEL` reader - "]
pub type SyncoselR = crate::FieldReader<Syncosel>;
impl SyncoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syncosel> {
        match self.bits {
            0 => Some(Syncosel::Bypass),
            1 => Some(Syncosel::CtreqPrd),
            2 => Some(Syncosel::Disable),
            _ => None,
        }
    }
    #[doc = "sync in connected with sync out"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Syncosel::Bypass
    }
    #[doc = "sync out generated when CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Syncosel::CtreqPrd
    }
    #[doc = "sync out generate disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Syncosel::Disable
    }
}
#[doc = "Field `SYNCOSEL` writer - "]
pub type SyncoselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Syncosel>;
impl<'a, REG> SyncoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "sync in connected with sync out"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Syncosel::Bypass)
    }
    #[doc = "sync out generated when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Syncosel::CtreqPrd)
    }
    #[doc = "sync out generate disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syncosel::Disable)
    }
}
#[doc = "Field `SWSYNC` reader - Software timers sync"]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - Software timers sync"]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPAPWM` reader - Capture mode or APWM mode"]
pub type CapapwmR = crate::BitReader;
#[doc = "Field `CAPAPWM` writer - Capture mode or APWM mode"]
pub type CapapwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APWMPOL` reader - High/low level APWM"]
pub type ApwmpolR = crate::BitReader;
#[doc = "Field `APWMPOL` writer - High/low level APWM"]
pub type ApwmpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture mode"]
    #[inline(always)]
    pub fn contost(&self) -> ContostR {
        ContostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn stopwrap(&self) -> StopwrapR {
        StopwrapR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Reset and enable controller, capture reg load"]
    #[inline(always)]
    pub fn rearm(&self) -> RearmR {
        RearmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Timer"]
    #[inline(always)]
    pub fn tsctrstop(&self) -> TsctrstopR {
        TsctrstopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync in enable"]
    #[inline(always)]
    pub fn syncien(&self) -> SyncienR {
        SyncienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn syncosel(&self) -> SyncoselR {
        SyncoselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Software timers sync"]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture mode or APWM mode"]
    #[inline(always)]
    pub fn capapwm(&self) -> CapapwmR {
        CapapwmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High/low level APWM"]
    #[inline(always)]
    pub fn apwmpol(&self) -> ApwmpolR {
        ApwmpolR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn contost(&mut self) -> ContostW<Ecctl1Spec> {
        ContostW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn stopwrap(&mut self) -> StopwrapW<Ecctl1Spec> {
        StopwrapW::new(self, 1)
    }
    #[doc = "Bit 3 - Reset and enable controller, capture reg load"]
    #[inline(always)]
    #[must_use]
    pub fn rearm(&mut self) -> RearmW<Ecctl1Spec> {
        RearmW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Timer"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrstop(&mut self) -> TsctrstopW<Ecctl1Spec> {
        TsctrstopW::new(self, 4)
    }
    #[doc = "Bit 5 - Sync in enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncien(&mut self) -> SyncienW<Ecctl1Spec> {
        SyncienW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn syncosel(&mut self) -> SyncoselW<Ecctl1Spec> {
        SyncoselW::new(self, 6)
    }
    #[doc = "Bit 8 - Software timers sync"]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SwsyncW<Ecctl1Spec> {
        SwsyncW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture mode or APWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn capapwm(&mut self) -> CapapwmW<Ecctl1Spec> {
        CapapwmW::new(self, 9)
    }
    #[doc = "Bit 10 - High/low level APWM"]
    #[inline(always)]
    #[must_use]
    pub fn apwmpol(&mut self) -> ApwmpolW<Ecctl1Spec> {
        ApwmpolW::new(self, 10)
    }
}
#[doc = "Capture control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecctl1Spec;
impl crate::RegisterSpec for Ecctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecctl1::R`](R) reader structure"]
impl crate::Readable for Ecctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ecctl1::W`](W) writer structure"]
impl crate::Writable for Ecctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCTL1 to value 0"]
impl crate::Resettable for Ecctl1Spec {
    const RESET_VALUE: u32 = 0;
}
