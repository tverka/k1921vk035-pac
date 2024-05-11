#[doc = "Register `EMUX` reader"]
pub type R = crate::R<EmuxSpec>;
#[doc = "Register `EMUX` writer"]
pub type W = crate::W<EmuxSpec>;
#[doc = "Select start event for sequencer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em0 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: GPIOA interrupt"]
    Gpioa = 1,
    #[doc = "2: GPIOB interrupt"]
    Gpiob = 2,
    #[doc = "3: Timer 0 request"]
    Tmr0 = 3,
    #[doc = "4: Timer 1 request"]
    Tmr1 = 4,
    #[doc = "5: Timer 2 request"]
    Tmr2 = 5,
    #[doc = "6: Timer 3 request"]
    Tmr3 = 6,
    #[doc = "7: PWM0,1,2 A channel request"]
    Pwm012a = 7,
    #[doc = "8: PWM0,1,2 B channel request"]
    Pwm012b = 8,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em0> for u8 {
    #[inline(always)]
    fn from(variant: Em0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em0 {
    type Ux = u8;
}
impl crate::IsEnum for Em0 {}
#[doc = "Field `EM0` reader - Select start event for sequencer 0"]
pub type Em0R = crate::FieldReader<Em0>;
impl Em0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em0> {
        match self.bits {
            0 => Some(Em0::SwReq),
            1 => Some(Em0::Gpioa),
            2 => Some(Em0::Gpiob),
            3 => Some(Em0::Tmr0),
            4 => Some(Em0::Tmr1),
            5 => Some(Em0::Tmr2),
            6 => Some(Em0::Tmr3),
            7 => Some(Em0::Pwm012a),
            8 => Some(Em0::Pwm012b),
            15 => Some(Em0::Cycle),
            _ => None,
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em0::SwReq
    }
    #[doc = "GPIOA interrupt"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == Em0::Gpioa
    }
    #[doc = "GPIOB interrupt"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == Em0::Gpiob
    }
    #[doc = "Timer 0 request"]
    #[inline(always)]
    pub fn is_tmr0(&self) -> bool {
        *self == Em0::Tmr0
    }
    #[doc = "Timer 1 request"]
    #[inline(always)]
    pub fn is_tmr1(&self) -> bool {
        *self == Em0::Tmr1
    }
    #[doc = "Timer 2 request"]
    #[inline(always)]
    pub fn is_tmr2(&self) -> bool {
        *self == Em0::Tmr2
    }
    #[doc = "Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr3(&self) -> bool {
        *self == Em0::Tmr3
    }
    #[doc = "PWM0,1,2 A channel request"]
    #[inline(always)]
    pub fn is_pwm012a(&self) -> bool {
        *self == Em0::Pwm012a
    }
    #[doc = "PWM0,1,2 B channel request"]
    #[inline(always)]
    pub fn is_pwm012b(&self) -> bool {
        *self == Em0::Pwm012b
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em0::Cycle
    }
}
#[doc = "Field `EM0` writer - Select start event for sequencer 0"]
pub type Em0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em0>;
impl<'a, REG> Em0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::SwReq)
    }
    #[doc = "GPIOA interrupt"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Gpioa)
    }
    #[doc = "GPIOB interrupt"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Gpiob)
    }
    #[doc = "Timer 0 request"]
    #[inline(always)]
    pub fn tmr0(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmr0)
    }
    #[doc = "Timer 1 request"]
    #[inline(always)]
    pub fn tmr1(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmr1)
    }
    #[doc = "Timer 2 request"]
    #[inline(always)]
    pub fn tmr2(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmr2)
    }
    #[doc = "Timer 3 request"]
    #[inline(always)]
    pub fn tmr3(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmr3)
    }
    #[doc = "PWM0,1,2 A channel request"]
    #[inline(always)]
    pub fn pwm012a(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Pwm012a)
    }
    #[doc = "PWM0,1,2 B channel request"]
    #[inline(always)]
    pub fn pwm012b(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Pwm012b)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Cycle)
    }
}
#[doc = "Select start event for sequencer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em1 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: GPIOA interrupt"]
    Gpioa = 1,
    #[doc = "2: GPIOB interrupt"]
    Gpiob = 2,
    #[doc = "3: Timer 0 request"]
    Tmr0 = 3,
    #[doc = "4: Timer 1 request"]
    Tmr1 = 4,
    #[doc = "5: Timer 2 request"]
    Tmr2 = 5,
    #[doc = "6: Timer 3 request"]
    Tmr3 = 6,
    #[doc = "7: PWM0,1,2 A channel request"]
    Pwm012a = 7,
    #[doc = "8: PWM0,1,2 B channel request"]
    Pwm012b = 8,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em1> for u8 {
    #[inline(always)]
    fn from(variant: Em1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em1 {
    type Ux = u8;
}
impl crate::IsEnum for Em1 {}
#[doc = "Field `EM1` reader - Select start event for sequencer 1"]
pub type Em1R = crate::FieldReader<Em1>;
impl Em1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em1> {
        match self.bits {
            0 => Some(Em1::SwReq),
            1 => Some(Em1::Gpioa),
            2 => Some(Em1::Gpiob),
            3 => Some(Em1::Tmr0),
            4 => Some(Em1::Tmr1),
            5 => Some(Em1::Tmr2),
            6 => Some(Em1::Tmr3),
            7 => Some(Em1::Pwm012a),
            8 => Some(Em1::Pwm012b),
            15 => Some(Em1::Cycle),
            _ => None,
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em1::SwReq
    }
    #[doc = "GPIOA interrupt"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == Em1::Gpioa
    }
    #[doc = "GPIOB interrupt"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == Em1::Gpiob
    }
    #[doc = "Timer 0 request"]
    #[inline(always)]
    pub fn is_tmr0(&self) -> bool {
        *self == Em1::Tmr0
    }
    #[doc = "Timer 1 request"]
    #[inline(always)]
    pub fn is_tmr1(&self) -> bool {
        *self == Em1::Tmr1
    }
    #[doc = "Timer 2 request"]
    #[inline(always)]
    pub fn is_tmr2(&self) -> bool {
        *self == Em1::Tmr2
    }
    #[doc = "Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr3(&self) -> bool {
        *self == Em1::Tmr3
    }
    #[doc = "PWM0,1,2 A channel request"]
    #[inline(always)]
    pub fn is_pwm012a(&self) -> bool {
        *self == Em1::Pwm012a
    }
    #[doc = "PWM0,1,2 B channel request"]
    #[inline(always)]
    pub fn is_pwm012b(&self) -> bool {
        *self == Em1::Pwm012b
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em1::Cycle
    }
}
#[doc = "Field `EM1` writer - Select start event for sequencer 1"]
pub type Em1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em1>;
impl<'a, REG> Em1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::SwReq)
    }
    #[doc = "GPIOA interrupt"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Gpioa)
    }
    #[doc = "GPIOB interrupt"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Gpiob)
    }
    #[doc = "Timer 0 request"]
    #[inline(always)]
    pub fn tmr0(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmr0)
    }
    #[doc = "Timer 1 request"]
    #[inline(always)]
    pub fn tmr1(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmr1)
    }
    #[doc = "Timer 2 request"]
    #[inline(always)]
    pub fn tmr2(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmr2)
    }
    #[doc = "Timer 3 request"]
    #[inline(always)]
    pub fn tmr3(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmr3)
    }
    #[doc = "PWM0,1,2 A channel request"]
    #[inline(always)]
    pub fn pwm012a(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Pwm012a)
    }
    #[doc = "PWM0,1,2 B channel request"]
    #[inline(always)]
    pub fn pwm012b(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Pwm012b)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Cycle)
    }
}
impl R {
    #[doc = "Bits 0:3 - Select start event for sequencer 0"]
    #[inline(always)]
    pub fn em0(&self) -> Em0R {
        Em0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select start event for sequencer 1"]
    #[inline(always)]
    pub fn em1(&self) -> Em1R {
        Em1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select start event for sequencer 0"]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> Em0W<EmuxSpec> {
        Em0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select start event for sequencer 1"]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> Em1W<EmuxSpec> {
        Em1W::new(self, 4)
    }
}
#[doc = "Sequencer start event selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmuxSpec;
impl crate::RegisterSpec for EmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emux::R`](R) reader structure"]
impl crate::Readable for EmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`emux::W`](W) writer structure"]
impl crate::Writable for EmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMUX to value 0"]
impl crate::Resettable for EmuxSpec {
    const RESET_VALUE: u32 = 0;
}
