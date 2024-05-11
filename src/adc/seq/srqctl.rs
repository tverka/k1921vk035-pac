#[doc = "Register `SRQCTL` reader"]
pub type R = crate::R<SrqctlSpec>;
#[doc = "Register `SRQCTL` writer"]
pub type W = crate::W<SrqctlSpec>;
#[doc = "Field `RQMAX` reader - Request queue max depth"]
pub type RqmaxR = crate::FieldReader;
#[doc = "Field `RQMAX` writer - Request queue max depth"]
pub type RqmaxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QAVGEN` reader - Queue avearage (scanning) enable"]
pub type QavgenR = crate::BitReader;
#[doc = "Field `QAVGEN` writer - Queue avearage (scanning) enable"]
pub type QavgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Queue average value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qavgval {
    #[doc = "0: Average disabled"]
    Disable = 0,
    #[doc = "1: Average with 2 measures"]
    Average2 = 1,
    #[doc = "2: Average with 4 measures"]
    Average4 = 2,
    #[doc = "3: Average with 8 measures"]
    Average8 = 3,
    #[doc = "4: Average with 16 measures"]
    Average16 = 4,
    #[doc = "5: Average with 32 measures"]
    Average32 = 5,
    #[doc = "6: Average with 64 measures"]
    Average64 = 6,
}
impl From<Qavgval> for u8 {
    #[inline(always)]
    fn from(variant: Qavgval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qavgval {
    type Ux = u8;
}
impl crate::IsEnum for Qavgval {}
#[doc = "Field `QAVGVAL` reader - Queue average value"]
pub type QavgvalR = crate::FieldReader<Qavgval>;
impl QavgvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qavgval> {
        match self.bits {
            0 => Some(Qavgval::Disable),
            1 => Some(Qavgval::Average2),
            2 => Some(Qavgval::Average4),
            3 => Some(Qavgval::Average8),
            4 => Some(Qavgval::Average16),
            5 => Some(Qavgval::Average32),
            6 => Some(Qavgval::Average64),
            _ => None,
        }
    }
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Qavgval::Disable
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn is_average2(&self) -> bool {
        *self == Qavgval::Average2
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn is_average4(&self) -> bool {
        *self == Qavgval::Average4
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn is_average8(&self) -> bool {
        *self == Qavgval::Average8
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn is_average16(&self) -> bool {
        *self == Qavgval::Average16
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn is_average32(&self) -> bool {
        *self == Qavgval::Average32
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn is_average64(&self) -> bool {
        *self == Qavgval::Average64
    }
}
#[doc = "Field `QAVGVAL` writer - Queue average value"]
pub type QavgvalW<'a, REG> = crate::FieldWriter<'a, REG, 3, Qavgval>;
impl<'a, REG> QavgvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Average disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Qavgval::Disable)
    }
    #[doc = "Average with 2 measures"]
    #[inline(always)]
    pub fn average2(self) -> &'a mut crate::W<REG> {
        self.variant(Qavgval::Average2)
    }
    #[doc = "Average with 4 measures"]
    #[inline(always)]
    pub fn average4(self) -> &'a mut crate::W<REG> {
        self.variant(Qavgval::Average4)
    }
    #[doc = "Average with 8 measures"]
    #[inline(always)]
    pub fn average8(self) -> &'a mut crate::W<REG> {
        self.variant(Qavgval::Average8)
    }
    #[doc = "Average with 16 measures"]
    #[inline(always)]
    pub fn average16(self) -> &'a mut crate::W<REG> {
        self.variant(Qavgval::Average16)
    }
    #[doc = "Average with 32 measures"]
    #[inline(always)]
    pub fn average32(self) -> &'a mut crate::W<REG> {
        self.variant(Qavgval::Average32)
    }
    #[doc = "Average with 64 measures"]
    #[inline(always)]
    pub fn average64(self) -> &'a mut crate::W<REG> {
        self.variant(Qavgval::Average64)
    }
}
impl R {
    #[doc = "Bits 0:1 - Request queue max depth"]
    #[inline(always)]
    pub fn rqmax(&self) -> RqmaxR {
        RqmaxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Queue avearage (scanning) enable"]
    #[inline(always)]
    pub fn qavgen(&self) -> QavgenR {
        QavgenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Queue average value"]
    #[inline(always)]
    pub fn qavgval(&self) -> QavgvalR {
        QavgvalR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Request queue max depth"]
    #[inline(always)]
    #[must_use]
    pub fn rqmax(&mut self) -> RqmaxW<SrqctlSpec> {
        RqmaxW::new(self, 0)
    }
    #[doc = "Bit 8 - Queue avearage (scanning) enable"]
    #[inline(always)]
    #[must_use]
    pub fn qavgen(&mut self) -> QavgenW<SrqctlSpec> {
        QavgenW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Queue average value"]
    #[inline(always)]
    #[must_use]
    pub fn qavgval(&mut self) -> QavgvalW<SrqctlSpec> {
        QavgvalW::new(self, 9)
    }
}
#[doc = "Sequencer request control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srqctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrqctlSpec;
impl crate::RegisterSpec for SrqctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqctl::R`](R) reader structure"]
impl crate::Readable for SrqctlSpec {}
#[doc = "`write(|w| ..)` method takes [`srqctl::W`](W) writer structure"]
impl crate::Writable for SrqctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRQCTL to value 0"]
impl crate::Resettable for SrqctlSpec {
    const RESET_VALUE: u32 = 0;
}
