#[doc = "Register `DMAMUX` reader"]
pub type R = crate::R<DmamuxSpec>;
#[doc = "Register `DMAMUX` writer"]
pub type W = crate::W<DmamuxSpec>;
#[doc = "Request source select for DMA channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel8 {
    #[doc = "0: request by QEP"]
    Qep = 0,
    #[doc = "1: request by GPIOA"]
    Gpioa = 1,
}
impl From<Srcsel8> for bool {
    #[inline(always)]
    fn from(variant: Srcsel8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL8` reader - Request source select for DMA channel 8"]
pub type Srcsel8R = crate::BitReader<Srcsel8>;
impl Srcsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel8 {
        match self.bits {
            false => Srcsel8::Qep,
            true => Srcsel8::Gpioa,
        }
    }
    #[doc = "request by QEP"]
    #[inline(always)]
    pub fn is_qep(&self) -> bool {
        *self == Srcsel8::Qep
    }
    #[doc = "request by GPIOA"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == Srcsel8::Gpioa
    }
}
#[doc = "Field `SRCSEL8` writer - Request source select for DMA channel 8"]
pub type Srcsel8W<'a, REG> = crate::BitWriter<'a, REG, Srcsel8>;
impl<'a, REG> Srcsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by QEP"]
    #[inline(always)]
    pub fn qep(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel8::Qep)
    }
    #[doc = "request by GPIOA"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel8::Gpioa)
    }
}
#[doc = "Request source select for DMA channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel9 {
    #[doc = "0: request by TMR0"]
    Tmr0 = 0,
    #[doc = "1: request by GPIOB"]
    Gpiob = 1,
}
impl From<Srcsel9> for bool {
    #[inline(always)]
    fn from(variant: Srcsel9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL9` reader - Request source select for DMA channel 9"]
pub type Srcsel9R = crate::BitReader<Srcsel9>;
impl Srcsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel9 {
        match self.bits {
            false => Srcsel9::Tmr0,
            true => Srcsel9::Gpiob,
        }
    }
    #[doc = "request by TMR0"]
    #[inline(always)]
    pub fn is_tmr0(&self) -> bool {
        *self == Srcsel9::Tmr0
    }
    #[doc = "request by GPIOB"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == Srcsel9::Gpiob
    }
}
#[doc = "Field `SRCSEL9` writer - Request source select for DMA channel 9"]
pub type Srcsel9W<'a, REG> = crate::BitWriter<'a, REG, Srcsel9>;
impl<'a, REG> Srcsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by TMR0"]
    #[inline(always)]
    pub fn tmr0(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel9::Tmr0)
    }
    #[doc = "request by GPIOB"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel9::Gpiob)
    }
}
#[doc = "Request source select for DMA channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel10 {
    #[doc = "0: request by TMR1"]
    Tmr1 = 0,
    #[doc = "1: request by PWM0B"]
    Pwm0b = 1,
}
impl From<Srcsel10> for bool {
    #[inline(always)]
    fn from(variant: Srcsel10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL10` reader - Request source select for DMA channel 10"]
pub type Srcsel10R = crate::BitReader<Srcsel10>;
impl Srcsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel10 {
        match self.bits {
            false => Srcsel10::Tmr1,
            true => Srcsel10::Pwm0b,
        }
    }
    #[doc = "request by TMR1"]
    #[inline(always)]
    pub fn is_tmr1(&self) -> bool {
        *self == Srcsel10::Tmr1
    }
    #[doc = "request by PWM0B"]
    #[inline(always)]
    pub fn is_pwm0b(&self) -> bool {
        *self == Srcsel10::Pwm0b
    }
}
#[doc = "Field `SRCSEL10` writer - Request source select for DMA channel 10"]
pub type Srcsel10W<'a, REG> = crate::BitWriter<'a, REG, Srcsel10>;
impl<'a, REG> Srcsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by TMR1"]
    #[inline(always)]
    pub fn tmr1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel10::Tmr1)
    }
    #[doc = "request by PWM0B"]
    #[inline(always)]
    pub fn pwm0b(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel10::Pwm0b)
    }
}
#[doc = "Request source select for DMA channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel11 {
    #[doc = "0: request by TMR2"]
    Tmr2 = 0,
    #[doc = "1: request by PWM1B"]
    Pwm1b = 1,
}
impl From<Srcsel11> for bool {
    #[inline(always)]
    fn from(variant: Srcsel11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL11` reader - Request source select for DMA channel 11"]
pub type Srcsel11R = crate::BitReader<Srcsel11>;
impl Srcsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel11 {
        match self.bits {
            false => Srcsel11::Tmr2,
            true => Srcsel11::Pwm1b,
        }
    }
    #[doc = "request by TMR2"]
    #[inline(always)]
    pub fn is_tmr2(&self) -> bool {
        *self == Srcsel11::Tmr2
    }
    #[doc = "request by PWM1B"]
    #[inline(always)]
    pub fn is_pwm1b(&self) -> bool {
        *self == Srcsel11::Pwm1b
    }
}
#[doc = "Field `SRCSEL11` writer - Request source select for DMA channel 11"]
pub type Srcsel11W<'a, REG> = crate::BitWriter<'a, REG, Srcsel11>;
impl<'a, REG> Srcsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by TMR2"]
    #[inline(always)]
    pub fn tmr2(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel11::Tmr2)
    }
    #[doc = "request by PWM1B"]
    #[inline(always)]
    pub fn pwm1b(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel11::Pwm1b)
    }
}
#[doc = "Request source select for DMA channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel12 {
    #[doc = "0: request by TMR3"]
    Tmr3 = 0,
    #[doc = "1: request by PWM2B"]
    Pwm2b = 1,
}
impl From<Srcsel12> for bool {
    #[inline(always)]
    fn from(variant: Srcsel12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL12` reader - Request source select for DMA channel 12"]
pub type Srcsel12R = crate::BitReader<Srcsel12>;
impl Srcsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel12 {
        match self.bits {
            false => Srcsel12::Tmr3,
            true => Srcsel12::Pwm2b,
        }
    }
    #[doc = "request by TMR3"]
    #[inline(always)]
    pub fn is_tmr3(&self) -> bool {
        *self == Srcsel12::Tmr3
    }
    #[doc = "request by PWM2B"]
    #[inline(always)]
    pub fn is_pwm2b(&self) -> bool {
        *self == Srcsel12::Pwm2b
    }
}
#[doc = "Field `SRCSEL12` writer - Request source select for DMA channel 12"]
pub type Srcsel12W<'a, REG> = crate::BitWriter<'a, REG, Srcsel12>;
impl<'a, REG> Srcsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by TMR3"]
    #[inline(always)]
    pub fn tmr3(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel12::Tmr3)
    }
    #[doc = "request by PWM2B"]
    #[inline(always)]
    pub fn pwm2b(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel12::Pwm2b)
    }
}
#[doc = "Request source select for DMA channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel13 {
    #[doc = "0: request by PWM0A"]
    Pwm0a = 0,
}
impl From<Srcsel13> for bool {
    #[inline(always)]
    fn from(variant: Srcsel13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL13` reader - Request source select for DMA channel 13"]
pub type Srcsel13R = crate::BitReader<Srcsel13>;
impl Srcsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel13 {
        match self.bits {
            false => Srcsel13::Pwm0a,
            _ => unreachable!(),
        }
    }
    #[doc = "request by PWM0A"]
    #[inline(always)]
    pub fn is_pwm0a(&self) -> bool {
        *self == Srcsel13::Pwm0a
    }
}
#[doc = "Field `SRCSEL13` writer - Request source select for DMA channel 13"]
pub type Srcsel13W<'a, REG> = crate::BitWriter<'a, REG, Srcsel13>;
impl<'a, REG> Srcsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by PWM0A"]
    #[inline(always)]
    pub fn pwm0a(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel13::Pwm0a)
    }
}
#[doc = "Request source select for DMA channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel14 {
    #[doc = "0: request by PWM1A"]
    Pwm1a = 0,
}
impl From<Srcsel14> for bool {
    #[inline(always)]
    fn from(variant: Srcsel14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL14` reader - Request source select for DMA channel 14"]
pub type Srcsel14R = crate::BitReader<Srcsel14>;
impl Srcsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel14 {
        match self.bits {
            false => Srcsel14::Pwm1a,
            _ => unreachable!(),
        }
    }
    #[doc = "request by PWM1A"]
    #[inline(always)]
    pub fn is_pwm1a(&self) -> bool {
        *self == Srcsel14::Pwm1a
    }
}
#[doc = "Field `SRCSEL14` writer - Request source select for DMA channel 14"]
pub type Srcsel14W<'a, REG> = crate::BitWriter<'a, REG, Srcsel14>;
impl<'a, REG> Srcsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by PWM1A"]
    #[inline(always)]
    pub fn pwm1a(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel14::Pwm1a)
    }
}
#[doc = "Request source select for DMA channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcsel15 {
    #[doc = "0: request by PWM2A"]
    Pwm2a = 0,
}
impl From<Srcsel15> for bool {
    #[inline(always)]
    fn from(variant: Srcsel15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCSEL15` reader - Request source select for DMA channel 15"]
pub type Srcsel15R = crate::BitReader<Srcsel15>;
impl Srcsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcsel15 {
        match self.bits {
            false => Srcsel15::Pwm2a,
            _ => unreachable!(),
        }
    }
    #[doc = "request by PWM2A"]
    #[inline(always)]
    pub fn is_pwm2a(&self) -> bool {
        *self == Srcsel15::Pwm2a
    }
}
#[doc = "Field `SRCSEL15` writer - Request source select for DMA channel 15"]
pub type Srcsel15W<'a, REG> = crate::BitWriter<'a, REG, Srcsel15>;
impl<'a, REG> Srcsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request by PWM2A"]
    #[inline(always)]
    pub fn pwm2a(self) -> &'a mut crate::W<REG> {
        self.variant(Srcsel15::Pwm2a)
    }
}
impl R {
    #[doc = "Bit 0 - Request source select for DMA channel 8"]
    #[inline(always)]
    pub fn srcsel8(&self) -> Srcsel8R {
        Srcsel8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Request source select for DMA channel 9"]
    #[inline(always)]
    pub fn srcsel9(&self) -> Srcsel9R {
        Srcsel9R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Request source select for DMA channel 10"]
    #[inline(always)]
    pub fn srcsel10(&self) -> Srcsel10R {
        Srcsel10R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Request source select for DMA channel 11"]
    #[inline(always)]
    pub fn srcsel11(&self) -> Srcsel11R {
        Srcsel11R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Request source select for DMA channel 12"]
    #[inline(always)]
    pub fn srcsel12(&self) -> Srcsel12R {
        Srcsel12R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Request source select for DMA channel 13"]
    #[inline(always)]
    pub fn srcsel13(&self) -> Srcsel13R {
        Srcsel13R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Request source select for DMA channel 14"]
    #[inline(always)]
    pub fn srcsel14(&self) -> Srcsel14R {
        Srcsel14R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Request source select for DMA channel 15"]
    #[inline(always)]
    pub fn srcsel15(&self) -> Srcsel15R {
        Srcsel15R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request source select for DMA channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel8(&mut self) -> Srcsel8W<DmamuxSpec> {
        Srcsel8W::new(self, 0)
    }
    #[doc = "Bit 4 - Request source select for DMA channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel9(&mut self) -> Srcsel9W<DmamuxSpec> {
        Srcsel9W::new(self, 4)
    }
    #[doc = "Bit 8 - Request source select for DMA channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel10(&mut self) -> Srcsel10W<DmamuxSpec> {
        Srcsel10W::new(self, 8)
    }
    #[doc = "Bit 12 - Request source select for DMA channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel11(&mut self) -> Srcsel11W<DmamuxSpec> {
        Srcsel11W::new(self, 12)
    }
    #[doc = "Bit 16 - Request source select for DMA channel 12"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel12(&mut self) -> Srcsel12W<DmamuxSpec> {
        Srcsel12W::new(self, 16)
    }
    #[doc = "Bit 20 - Request source select for DMA channel 13"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel13(&mut self) -> Srcsel13W<DmamuxSpec> {
        Srcsel13W::new(self, 20)
    }
    #[doc = "Bit 24 - Request source select for DMA channel 14"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel14(&mut self) -> Srcsel14W<DmamuxSpec> {
        Srcsel14W::new(self, 24)
    }
    #[doc = "Bit 28 - Request source select for DMA channel 15"]
    #[inline(always)]
    #[must_use]
    pub fn srcsel15(&mut self) -> Srcsel15W<DmamuxSpec> {
        Srcsel15W::new(self, 28)
    }
}
#[doc = "DMA external requests mux control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamuxSpec;
impl crate::RegisterSpec for DmamuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux::R`](R) reader structure"]
impl crate::Readable for DmamuxSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamux::W`](W) writer structure"]
impl crate::Writable for DmamuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX to value 0"]
impl crate::Resettable for DmamuxSpec {
    const RESET_VALUE: u32 = 0;
}
