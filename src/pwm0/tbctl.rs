#[doc = "Register `TBCTL` reader"]
pub type R = crate::R<TbctlSpec>;
#[doc = "Register `TBCTL` writer"]
pub type W = crate::W<TbctlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctrmode {
    #[doc = "0: count direction up"]
    Up = 0,
    #[doc = "1: count direction down"]
    Down = 1,
    #[doc = "2: count direction up-down"]
    UpDown = 2,
    #[doc = "3: counter stopped"]
    Stop = 3,
}
impl From<Ctrmode> for u8 {
    #[inline(always)]
    fn from(variant: Ctrmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctrmode {
    type Ux = u8;
}
impl crate::IsEnum for Ctrmode {}
#[doc = "Field `CTRMODE` reader - "]
pub type CtrmodeR = crate::FieldReader<Ctrmode>;
impl CtrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrmode {
        match self.bits {
            0 => Ctrmode::Up,
            1 => Ctrmode::Down,
            2 => Ctrmode::UpDown,
            3 => Ctrmode::Stop,
            _ => unreachable!(),
        }
    }
    #[doc = "count direction up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Ctrmode::Up
    }
    #[doc = "count direction down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Ctrmode::Down
    }
    #[doc = "count direction up-down"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == Ctrmode::UpDown
    }
    #[doc = "counter stopped"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Ctrmode::Stop
    }
}
#[doc = "Field `CTRMODE` writer - "]
pub type CtrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctrmode, crate::Safe>;
impl<'a, REG> CtrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "count direction up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrmode::Up)
    }
    #[doc = "count direction down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrmode::Down)
    }
    #[doc = "count direction up-down"]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrmode::UpDown)
    }
    #[doc = "counter stopped"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrmode::Stop)
    }
}
#[doc = "Field `PHSEN` reader - Counter register load from phase register enable"]
pub type PhsenR = crate::BitReader;
#[doc = "Field `PHSEN` writer - Counter register load from phase register enable"]
pub type PhsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDLD` reader - Active period register load from shadow register select"]
pub type PrdldR = crate::BitReader;
#[doc = "Field `PRDLD` writer - Active period register load from shadow register select"]
pub type PrdldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syncosel {
    #[doc = "0: PWM_SYNCI is source for PWM_SYNCO"]
    Synci = 0,
    #[doc = "1: CTR = 0000h is source for PWM_SYNCO"]
    CtreqZero = 1,
    #[doc = "2: CTR = CMPB is source for PWM_SYNCO"]
    CtreqCmpb = 2,
    #[doc = "3: PWM_SYNCO generation disabled"]
    Disable = 3,
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
    pub const fn variant(&self) -> Syncosel {
        match self.bits {
            0 => Syncosel::Synci,
            1 => Syncosel::CtreqZero,
            2 => Syncosel::CtreqCmpb,
            3 => Syncosel::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "PWM_SYNCI is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn is_synci(&self) -> bool {
        *self == Syncosel::Synci
    }
    #[doc = "CTR = 0000h is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Syncosel::CtreqZero
    }
    #[doc = "CTR = CMPB is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn is_ctreq_cmpb(&self) -> bool {
        *self == Syncosel::CtreqCmpb
    }
    #[doc = "PWM_SYNCO generation disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Syncosel::Disable
    }
}
#[doc = "Field `SYNCOSEL` writer - "]
pub type SyncoselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Syncosel, crate::Safe>;
impl<'a, REG> SyncoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM_SYNCI is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn synci(self) -> &'a mut crate::W<REG> {
        self.variant(Syncosel::Synci)
    }
    #[doc = "CTR = 0000h is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Syncosel::CtreqZero)
    }
    #[doc = "CTR = CMPB is source for PWM_SYNCO"]
    #[inline(always)]
    pub fn ctreq_cmpb(self) -> &'a mut crate::W<REG> {
        self.variant(Syncosel::CtreqCmpb)
    }
    #[doc = "PWM_SYNCO generation disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syncosel::Disable)
    }
}
#[doc = "Field `SWFSYNC` reader - Software forced synchronization pulse"]
pub type SwfsyncR = crate::BitReader;
#[doc = "Field `SWFSYNC` writer - Software forced synchronization pulse"]
pub type SwfsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hspclkdiv {
    #[doc = "0: clock not divided"]
    Div1 = 0,
    #[doc = "1: clock divided by 2"]
    Div2 = 1,
    #[doc = "2: clock divided by 4"]
    Div4 = 2,
    #[doc = "3: clock divided by 6"]
    Div6 = 3,
    #[doc = "4: clock divided by 8"]
    Div8 = 4,
    #[doc = "5: clock divided by 10"]
    Div10 = 5,
    #[doc = "6: clock divided by 12"]
    Div12 = 6,
    #[doc = "7: clock divided by 14"]
    Div14 = 7,
}
impl From<Hspclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Hspclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hspclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Hspclkdiv {}
#[doc = "Field `HSPCLKDIV` reader - "]
pub type HspclkdivR = crate::FieldReader<Hspclkdiv>;
impl HspclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hspclkdiv {
        match self.bits {
            0 => Hspclkdiv::Div1,
            1 => Hspclkdiv::Div2,
            2 => Hspclkdiv::Div4,
            3 => Hspclkdiv::Div6,
            4 => Hspclkdiv::Div8,
            5 => Hspclkdiv::Div10,
            6 => Hspclkdiv::Div12,
            7 => Hspclkdiv::Div14,
            _ => unreachable!(),
        }
    }
    #[doc = "clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Hspclkdiv::Div1
    }
    #[doc = "clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hspclkdiv::Div2
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Hspclkdiv::Div4
    }
    #[doc = "clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Hspclkdiv::Div6
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Hspclkdiv::Div8
    }
    #[doc = "clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Hspclkdiv::Div10
    }
    #[doc = "clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Hspclkdiv::Div12
    }
    #[doc = "clock divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Hspclkdiv::Div14
    }
}
#[doc = "Field `HSPCLKDIV` writer - "]
pub type HspclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hspclkdiv, crate::Safe>;
impl<'a, REG> HspclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div1)
    }
    #[doc = "clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div2)
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div4)
    }
    #[doc = "clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div6)
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div8)
    }
    #[doc = "clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div10)
    }
    #[doc = "clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div12)
    }
    #[doc = "clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Hspclkdiv::Div14)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv {
    #[doc = "0: clock not divided"]
    Div1 = 0,
    #[doc = "1: clock divided by 2"]
    Div2 = 1,
    #[doc = "2: clock divided by 4"]
    Div4 = 2,
    #[doc = "3: clock divided by 8"]
    Div8 = 3,
    #[doc = "4: clock divided by 16"]
    Div16 = 4,
    #[doc = "5: clock divided by 32"]
    Div32 = 5,
    #[doc = "6: clock divided by 64"]
    Div64 = 6,
    #[doc = "7: clock divided by 128"]
    Div128 = 7,
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv {}
#[doc = "Field `CLKDIV` reader - "]
pub type ClkdivR = crate::FieldReader<Clkdiv>;
impl ClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkdiv {
        match self.bits {
            0 => Clkdiv::Div1,
            1 => Clkdiv::Div2,
            2 => Clkdiv::Div4,
            3 => Clkdiv::Div8,
            4 => Clkdiv::Div16,
            5 => Clkdiv::Div32,
            6 => Clkdiv::Div64,
            7 => Clkdiv::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Clkdiv::Div1
    }
    #[doc = "clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Clkdiv::Div2
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Clkdiv::Div4
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Clkdiv::Div8
    }
    #[doc = "clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Clkdiv::Div16
    }
    #[doc = "clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Clkdiv::Div32
    }
    #[doc = "clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Clkdiv::Div64
    }
    #[doc = "clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Clkdiv::Div128
    }
}
#[doc = "Field `CLKDIV` writer - "]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkdiv, crate::Safe>;
impl<'a, REG> ClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div1)
    }
    #[doc = "clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div2)
    }
    #[doc = "clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div4)
    }
    #[doc = "clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div8)
    }
    #[doc = "clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div16)
    }
    #[doc = "clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div32)
    }
    #[doc = "clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div64)
    }
    #[doc = "clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div128)
    }
}
#[doc = "Field `PHSDIR` reader - Phase direction bit"]
pub type PhsdirR = crate::BitReader;
#[doc = "Field `PHSDIR` writer - Phase direction bit"]
pub type PhsdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freesoft {
    #[doc = "0: stop timer at next TBCLK tact"]
    StopAtTbclk = 0,
    #[doc = "1: stop timer when period ends"]
    StopAtPeriod = 1,
    #[doc = "2: free run mode"]
    FreeRun = 2,
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
            0 => Some(Freesoft::StopAtTbclk),
            1 => Some(Freesoft::StopAtPeriod),
            2 => Some(Freesoft::FreeRun),
            _ => None,
        }
    }
    #[doc = "stop timer at next TBCLK tact"]
    #[inline(always)]
    pub fn is_stop_at_tbclk(&self) -> bool {
        *self == Freesoft::StopAtTbclk
    }
    #[doc = "stop timer when period ends"]
    #[inline(always)]
    pub fn is_stop_at_period(&self) -> bool {
        *self == Freesoft::StopAtPeriod
    }
    #[doc = "free run mode"]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == Freesoft::FreeRun
    }
}
#[doc = "Field `FREESOFT` writer - "]
pub type FreesoftW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freesoft>;
impl<'a, REG> FreesoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "stop timer at next TBCLK tact"]
    #[inline(always)]
    pub fn stop_at_tbclk(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::StopAtTbclk)
    }
    #[doc = "stop timer when period ends"]
    #[inline(always)]
    pub fn stop_at_period(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::StopAtPeriod)
    }
    #[doc = "free run mode"]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::FreeRun)
    }
}
#[doc = "Field `SHDWGLOB` reader - Global enable for all shadow loads"]
pub type ShdwglobR = crate::BitReader;
#[doc = "Field `SHDWGLOB` writer - Global enable for all shadow loads"]
pub type ShdwglobW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ctrmode(&self) -> CtrmodeR {
        CtrmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Counter register load from phase register enable"]
    #[inline(always)]
    pub fn phsen(&self) -> PhsenR {
        PhsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active period register load from shadow register select"]
    #[inline(always)]
    pub fn prdld(&self) -> PrdldR {
        PrdldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn syncosel(&self) -> SyncoselR {
        SyncoselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Software forced synchronization pulse"]
    #[inline(always)]
    pub fn swfsync(&self) -> SwfsyncR {
        SwfsyncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn hspclkdiv(&self) -> HspclkdivR {
        HspclkdivR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - Phase direction bit"]
    #[inline(always)]
    pub fn phsdir(&self) -> PhsdirR {
        PhsdirR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn freesoft(&self) -> FreesoftR {
        FreesoftR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Global enable for all shadow loads"]
    #[inline(always)]
    pub fn shdwglob(&self) -> ShdwglobR {
        ShdwglobR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn ctrmode(&mut self) -> CtrmodeW<TbctlSpec> {
        CtrmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - Counter register load from phase register enable"]
    #[inline(always)]
    #[must_use]
    pub fn phsen(&mut self) -> PhsenW<TbctlSpec> {
        PhsenW::new(self, 2)
    }
    #[doc = "Bit 3 - Active period register load from shadow register select"]
    #[inline(always)]
    #[must_use]
    pub fn prdld(&mut self) -> PrdldW<TbctlSpec> {
        PrdldW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn syncosel(&mut self) -> SyncoselW<TbctlSpec> {
        SyncoselW::new(self, 4)
    }
    #[doc = "Bit 6 - Software forced synchronization pulse"]
    #[inline(always)]
    #[must_use]
    pub fn swfsync(&mut self) -> SwfsyncW<TbctlSpec> {
        SwfsyncW::new(self, 6)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn hspclkdiv(&mut self) -> HspclkdivW<TbctlSpec> {
        HspclkdivW::new(self, 7)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<TbctlSpec> {
        ClkdivW::new(self, 10)
    }
    #[doc = "Bit 13 - Phase direction bit"]
    #[inline(always)]
    #[must_use]
    pub fn phsdir(&mut self) -> PhsdirW<TbctlSpec> {
        PhsdirW::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn freesoft(&mut self) -> FreesoftW<TbctlSpec> {
        FreesoftW::new(self, 14)
    }
    #[doc = "Bit 16 - Global enable for all shadow loads"]
    #[inline(always)]
    #[must_use]
    pub fn shdwglob(&mut self) -> ShdwglobW<TbctlSpec> {
        ShdwglobW::new(self, 16)
    }
}
#[doc = "Time-Base Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctlSpec;
impl crate::RegisterSpec for TbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctl::R`](R) reader structure"]
impl crate::Readable for TbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctl::W`](W) writer structure"]
impl crate::Writable for TbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBCTL to value 0"]
impl crate::Resettable for TbctlSpec {
    const RESET_VALUE: u32 = 0;
}
