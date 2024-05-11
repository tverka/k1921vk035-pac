#[doc = "Register `ETSEL` reader"]
pub type R = crate::R<EtselSpec>;
#[doc = "Register `ETSEL` writer"]
pub type W = crate::W<EtselSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intsel {
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Intsel> for u8 {
    #[inline(always)]
    fn from(variant: Intsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intsel {
    type Ux = u8;
}
impl crate::IsEnum for Intsel {}
#[doc = "Field `INTSEL` reader - "]
pub type IntselR = crate::FieldReader<Intsel>;
impl IntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Intsel> {
        match self.bits {
            1 => Some(Intsel::CtreqZero),
            2 => Some(Intsel::CtreqPrd),
            4 => Some(Intsel::CtreqCmpaOnUp),
            5 => Some(Intsel::CtreqCmpaOnDown),
            6 => Some(Intsel::CtreqCmpbOnUp),
            7 => Some(Intsel::CtreqCmpbOnDown),
            _ => None,
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Intsel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Intsel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Intsel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Intsel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Intsel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Intsel::CtreqCmpbOnDown
    }
}
#[doc = "Field `INTSEL` writer - "]
pub type IntselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Intsel>;
impl<'a, REG> IntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `INTEN` reader - Enable PWM_INT interrupt generation"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Enable PWM_INT interrupt generation"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Socasel {
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Socasel> for u8 {
    #[inline(always)]
    fn from(variant: Socasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Socasel {
    type Ux = u8;
}
impl crate::IsEnum for Socasel {}
#[doc = "Field `SOCASEL` reader - "]
pub type SocaselR = crate::FieldReader<Socasel>;
impl SocaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Socasel> {
        match self.bits {
            1 => Some(Socasel::CtreqZero),
            2 => Some(Socasel::CtreqPrd),
            4 => Some(Socasel::CtreqCmpaOnUp),
            5 => Some(Socasel::CtreqCmpaOnDown),
            6 => Some(Socasel::CtreqCmpbOnUp),
            7 => Some(Socasel::CtreqCmpbOnDown),
            _ => None,
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Socasel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Socasel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Socasel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Socasel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Socasel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Socasel::CtreqCmpbOnDown
    }
}
#[doc = "Field `SOCASEL` writer - "]
pub type SocaselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Socasel>;
impl<'a, REG> SocaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `SOCAEN` reader - Enable the ADC start of conversion A PWM_SOCA pulse"]
pub type SocaenR = crate::BitReader;
#[doc = "Field `SOCAEN` writer - Enable the ADC start of conversion A PWM_SOCA pulse"]
pub type SocaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Socbsel {
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Socbsel> for u8 {
    #[inline(always)]
    fn from(variant: Socbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Socbsel {
    type Ux = u8;
}
impl crate::IsEnum for Socbsel {}
#[doc = "Field `SOCBSEL` reader - "]
pub type SocbselR = crate::FieldReader<Socbsel>;
impl SocbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Socbsel> {
        match self.bits {
            1 => Some(Socbsel::CtreqZero),
            2 => Some(Socbsel::CtreqPrd),
            4 => Some(Socbsel::CtreqCmpaOnUp),
            5 => Some(Socbsel::CtreqCmpaOnDown),
            6 => Some(Socbsel::CtreqCmpbOnUp),
            7 => Some(Socbsel::CtreqCmpbOnDown),
            _ => None,
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Socbsel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Socbsel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Socbsel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Socbsel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Socbsel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Socbsel::CtreqCmpbOnDown
    }
}
#[doc = "Field `SOCBSEL` writer - "]
pub type SocbselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Socbsel>;
impl<'a, REG> SocbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `SOCBEN` reader - Enable the ADC start of conversion B PWM_SOCB pulse"]
pub type SocbenR = crate::BitReader;
#[doc = "Field `SOCBEN` writer - Enable the ADC start of conversion B PWM_SOCB pulse"]
pub type SocbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drqasel {
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Drqasel> for u8 {
    #[inline(always)]
    fn from(variant: Drqasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drqasel {
    type Ux = u8;
}
impl crate::IsEnum for Drqasel {}
#[doc = "Field `DRQASEL` reader - "]
pub type DrqaselR = crate::FieldReader<Drqasel>;
impl DrqaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Drqasel> {
        match self.bits {
            1 => Some(Drqasel::CtreqZero),
            2 => Some(Drqasel::CtreqPrd),
            4 => Some(Drqasel::CtreqCmpaOnUp),
            5 => Some(Drqasel::CtreqCmpaOnDown),
            6 => Some(Drqasel::CtreqCmpbOnUp),
            7 => Some(Drqasel::CtreqCmpbOnDown),
            _ => None,
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Drqasel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Drqasel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Drqasel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Drqasel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Drqasel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Drqasel::CtreqCmpbOnDown
    }
}
#[doc = "Field `DRQASEL` writer - "]
pub type DrqaselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Drqasel>;
impl<'a, REG> DrqaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `DRQAEN` reader - Enable the DMA request from PWM A"]
pub type DrqaenR = crate::BitReader;
#[doc = "Field `DRQAEN` writer - Enable the DMA request from PWM A"]
pub type DrqaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drqbsel {
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Drqbsel> for u8 {
    #[inline(always)]
    fn from(variant: Drqbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drqbsel {
    type Ux = u8;
}
impl crate::IsEnum for Drqbsel {}
#[doc = "Field `DRQBSEL` reader - "]
pub type DrqbselR = crate::FieldReader<Drqbsel>;
impl DrqbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Drqbsel> {
        match self.bits {
            1 => Some(Drqbsel::CtreqZero),
            2 => Some(Drqbsel::CtreqPrd),
            4 => Some(Drqbsel::CtreqCmpaOnUp),
            5 => Some(Drqbsel::CtreqCmpaOnDown),
            6 => Some(Drqbsel::CtreqCmpbOnUp),
            7 => Some(Drqbsel::CtreqCmpbOnDown),
            _ => None,
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Drqbsel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Drqbsel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Drqbsel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Drqbsel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Drqbsel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Drqbsel::CtreqCmpbOnDown
    }
}
#[doc = "Field `DRQBSEL` writer - "]
pub type DrqbselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Drqbsel>;
impl<'a, REG> DrqbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `DRQBEN` reader - Enable the DMA request from PWM B"]
pub type DrqbenR = crate::BitReader;
#[doc = "Field `DRQBEN` writer - Enable the DMA request from PWM B"]
pub type DrqbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable PWM_INT interrupt generation"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn socasel(&self) -> SocaselR {
        SocaselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable the ADC start of conversion A PWM_SOCA pulse"]
    #[inline(always)]
    pub fn socaen(&self) -> SocaenR {
        SocaenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn socbsel(&self) -> SocbselR {
        SocbselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable the ADC start of conversion B PWM_SOCB pulse"]
    #[inline(always)]
    pub fn socben(&self) -> SocbenR {
        SocbenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn drqasel(&self) -> DrqaselR {
        DrqaselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Enable the DMA request from PWM A"]
    #[inline(always)]
    pub fn drqaen(&self) -> DrqaenR {
        DrqaenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn drqbsel(&self) -> DrqbselR {
        DrqbselR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Enable the DMA request from PWM B"]
    #[inline(always)]
    pub fn drqben(&self) -> DrqbenR {
        DrqbenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn intsel(&mut self) -> IntselW<EtselSpec> {
        IntselW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable PWM_INT interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<EtselSpec> {
        IntenW::new(self, 3)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn socasel(&mut self) -> SocaselW<EtselSpec> {
        SocaselW::new(self, 8)
    }
    #[doc = "Bit 11 - Enable the ADC start of conversion A PWM_SOCA pulse"]
    #[inline(always)]
    #[must_use]
    pub fn socaen(&mut self) -> SocaenW<EtselSpec> {
        SocaenW::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn socbsel(&mut self) -> SocbselW<EtselSpec> {
        SocbselW::new(self, 12)
    }
    #[doc = "Bit 15 - Enable the ADC start of conversion B PWM_SOCB pulse"]
    #[inline(always)]
    #[must_use]
    pub fn socben(&mut self) -> SocbenW<EtselSpec> {
        SocbenW::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn drqasel(&mut self) -> DrqaselW<EtselSpec> {
        DrqaselW::new(self, 16)
    }
    #[doc = "Bit 19 - Enable the DMA request from PWM A"]
    #[inline(always)]
    #[must_use]
    pub fn drqaen(&mut self) -> DrqaenW<EtselSpec> {
        DrqaenW::new(self, 19)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn drqbsel(&mut self) -> DrqbselW<EtselSpec> {
        DrqbselW::new(self, 20)
    }
    #[doc = "Bit 23 - Enable the DMA request from PWM B"]
    #[inline(always)]
    #[must_use]
    pub fn drqben(&mut self) -> DrqbenW<EtselSpec> {
        DrqbenW::new(self, 23)
    }
}
#[doc = "Event-Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtselSpec;
impl crate::RegisterSpec for EtselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etsel::R`](R) reader structure"]
impl crate::Readable for EtselSpec {}
#[doc = "`write(|w| ..)` method takes [`etsel::W`](W) writer structure"]
impl crate::Writable for EtselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETSEL to value 0"]
impl crate::Resettable for EtselSpec {
    const RESET_VALUE: u32 = 0;
}
