#[doc = "Register `QEPCTL` reader"]
pub type R = crate::R<QepctlSpec>;
#[doc = "Register `QEPCTL` writer"]
pub type W = crate::W<QepctlSpec>;
#[doc = "Field `WDE` reader - QEP watchdog enable"]
pub type WdeR = crate::BitReader;
#[doc = "Field `WDE` writer - QEP watchdog enable"]
pub type WdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTE` reader - QEP unit timer enable"]
pub type UteR = crate::BitReader;
#[doc = "Field `UTE` writer - QEP unit timer enable"]
pub type UteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QCLM` reader - QEP capture latch mode"]
pub type QclmR = crate::BitReader;
#[doc = "Field `QCLM` writer - QEP capture latch mode"]
pub type QclmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPEN` reader - Quadrature position counter enable/software reset"]
pub type QpenR = crate::BitReader;
#[doc = "Field `QPEN` writer - Quadrature position counter enable/software reset"]
pub type QpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iel {
    #[doc = "0: no position counter latch"]
    NoLatch = 0,
    #[doc = "1: latch on index signal posedge"]
    IndPos = 1,
    #[doc = "2: latch on index signal negedge"]
    IndNeg = 2,
    #[doc = "3: latch on index marker"]
    IndMark = 3,
}
impl From<Iel> for u8 {
    #[inline(always)]
    fn from(variant: Iel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iel {
    type Ux = u8;
}
impl crate::IsEnum for Iel {}
#[doc = "Field `IEL` reader - "]
pub type IelR = crate::FieldReader<Iel>;
impl IelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iel {
        match self.bits {
            0 => Iel::NoLatch,
            1 => Iel::IndPos,
            2 => Iel::IndNeg,
            3 => Iel::IndMark,
            _ => unreachable!(),
        }
    }
    #[doc = "no position counter latch"]
    #[inline(always)]
    pub fn is_no_latch(&self) -> bool {
        *self == Iel::NoLatch
    }
    #[doc = "latch on index signal posedge"]
    #[inline(always)]
    pub fn is_ind_pos(&self) -> bool {
        *self == Iel::IndPos
    }
    #[doc = "latch on index signal negedge"]
    #[inline(always)]
    pub fn is_ind_neg(&self) -> bool {
        *self == Iel::IndNeg
    }
    #[doc = "latch on index marker"]
    #[inline(always)]
    pub fn is_ind_mark(&self) -> bool {
        *self == Iel::IndMark
    }
}
#[doc = "Field `IEL` writer - "]
pub type IelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iel, crate::Safe>;
impl<'a, REG> IelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no position counter latch"]
    #[inline(always)]
    pub fn no_latch(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::NoLatch)
    }
    #[doc = "latch on index signal posedge"]
    #[inline(always)]
    pub fn ind_pos(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::IndPos)
    }
    #[doc = "latch on index signal negedge"]
    #[inline(always)]
    pub fn ind_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::IndNeg)
    }
    #[doc = "latch on index marker"]
    #[inline(always)]
    pub fn ind_mark(self) -> &'a mut crate::W<REG> {
        self.variant(Iel::IndMark)
    }
}
#[doc = "Field `SEL` reader - Strobe event latch of position counter"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - Strobe event latch of position counter"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI` reader - Software initialization of position counter"]
pub type SwiR = crate::BitReader;
#[doc = "Field `SWI` writer - Software initialization of position counter"]
pub type SwiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iei {
    #[doc = "0: no initialization"]
    NoInit = 0,
    #[doc = "2: init on posedge QEPI"]
    Qepipos = 2,
    #[doc = "3: init on negedge QEPI"]
    Qepineg = 3,
}
impl From<Iei> for u8 {
    #[inline(always)]
    fn from(variant: Iei) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iei {
    type Ux = u8;
}
impl crate::IsEnum for Iei {}
#[doc = "Field `IEI` reader - "]
pub type IeiR = crate::FieldReader<Iei>;
impl IeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iei> {
        match self.bits {
            0 => Some(Iei::NoInit),
            2 => Some(Iei::Qepipos),
            3 => Some(Iei::Qepineg),
            _ => None,
        }
    }
    #[doc = "no initialization"]
    #[inline(always)]
    pub fn is_no_init(&self) -> bool {
        *self == Iei::NoInit
    }
    #[doc = "init on posedge QEPI"]
    #[inline(always)]
    pub fn is_qepipos(&self) -> bool {
        *self == Iei::Qepipos
    }
    #[doc = "init on negedge QEPI"]
    #[inline(always)]
    pub fn is_qepineg(&self) -> bool {
        *self == Iei::Qepineg
    }
}
#[doc = "Field `IEI` writer - "]
pub type IeiW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iei>;
impl<'a, REG> IeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no initialization"]
    #[inline(always)]
    pub fn no_init(self) -> &'a mut crate::W<REG> {
        self.variant(Iei::NoInit)
    }
    #[doc = "init on posedge QEPI"]
    #[inline(always)]
    pub fn qepipos(self) -> &'a mut crate::W<REG> {
        self.variant(Iei::Qepipos)
    }
    #[doc = "init on negedge QEPI"]
    #[inline(always)]
    pub fn qepineg(self) -> &'a mut crate::W<REG> {
        self.variant(Iei::Qepineg)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sei {
    #[doc = "0: no initialization"]
    NoInit = 0,
    #[doc = "2: init on posedge QEPI"]
    Qepspos = 2,
    #[doc = "3: init depends on direction - on posedge if direction is up, on negedge if direction is down"]
    Qepsdir = 3,
}
impl From<Sei> for u8 {
    #[inline(always)]
    fn from(variant: Sei) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sei {
    type Ux = u8;
}
impl crate::IsEnum for Sei {}
#[doc = "Field `SEI` reader - "]
pub type SeiR = crate::FieldReader<Sei>;
impl SeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sei> {
        match self.bits {
            0 => Some(Sei::NoInit),
            2 => Some(Sei::Qepspos),
            3 => Some(Sei::Qepsdir),
            _ => None,
        }
    }
    #[doc = "no initialization"]
    #[inline(always)]
    pub fn is_no_init(&self) -> bool {
        *self == Sei::NoInit
    }
    #[doc = "init on posedge QEPI"]
    #[inline(always)]
    pub fn is_qepspos(&self) -> bool {
        *self == Sei::Qepspos
    }
    #[doc = "init depends on direction - on posedge if direction is up, on negedge if direction is down"]
    #[inline(always)]
    pub fn is_qepsdir(&self) -> bool {
        *self == Sei::Qepsdir
    }
}
#[doc = "Field `SEI` writer - "]
pub type SeiW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sei>;
impl<'a, REG> SeiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no initialization"]
    #[inline(always)]
    pub fn no_init(self) -> &'a mut crate::W<REG> {
        self.variant(Sei::NoInit)
    }
    #[doc = "init on posedge QEPI"]
    #[inline(always)]
    pub fn qepspos(self) -> &'a mut crate::W<REG> {
        self.variant(Sei::Qepspos)
    }
    #[doc = "init depends on direction - on posedge if direction is up, on negedge if direction is down"]
    #[inline(always)]
    pub fn qepsdir(self) -> &'a mut crate::W<REG> {
        self.variant(Sei::Qepsdir)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcrm {
    #[doc = "0: reset on index"]
    Ind = 0,
    #[doc = "1: reset on max position count"]
    PosMax = 1,
    #[doc = "2: reset on the first index"]
    FirstInd = 2,
    #[doc = "3: reset on time counter"]
    Time = 3,
}
impl From<Pcrm> for u8 {
    #[inline(always)]
    fn from(variant: Pcrm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcrm {
    type Ux = u8;
}
impl crate::IsEnum for Pcrm {}
#[doc = "Field `PCRM` reader - "]
pub type PcrmR = crate::FieldReader<Pcrm>;
impl PcrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcrm {
        match self.bits {
            0 => Pcrm::Ind,
            1 => Pcrm::PosMax,
            2 => Pcrm::FirstInd,
            3 => Pcrm::Time,
            _ => unreachable!(),
        }
    }
    #[doc = "reset on index"]
    #[inline(always)]
    pub fn is_ind(&self) -> bool {
        *self == Pcrm::Ind
    }
    #[doc = "reset on max position count"]
    #[inline(always)]
    pub fn is_pos_max(&self) -> bool {
        *self == Pcrm::PosMax
    }
    #[doc = "reset on the first index"]
    #[inline(always)]
    pub fn is_first_ind(&self) -> bool {
        *self == Pcrm::FirstInd
    }
    #[doc = "reset on time counter"]
    #[inline(always)]
    pub fn is_time(&self) -> bool {
        *self == Pcrm::Time
    }
}
#[doc = "Field `PCRM` writer - "]
pub type PcrmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pcrm, crate::Safe>;
impl<'a, REG> PcrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reset on index"]
    #[inline(always)]
    pub fn ind(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::Ind)
    }
    #[doc = "reset on max position count"]
    #[inline(always)]
    pub fn pos_max(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::PosMax)
    }
    #[doc = "reset on the first index"]
    #[inline(always)]
    pub fn first_ind(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::FirstInd)
    }
    #[doc = "reset on time counter"]
    #[inline(always)]
    pub fn time(self) -> &'a mut crate::W<REG> {
        self.variant(Pcrm::Time)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freesoft {
    #[doc = "0: counters are blocked"]
    Stop = 0,
    #[doc = "1: stop after overflow"]
    StopAtOvf = 1,
    #[doc = "2: no count stop in debug mode"]
    Free = 2,
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
            0 => Some(Freesoft::Stop),
            1 => Some(Freesoft::StopAtOvf),
            2 => Some(Freesoft::Free),
            _ => None,
        }
    }
    #[doc = "counters are blocked"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Freesoft::Stop
    }
    #[doc = "stop after overflow"]
    #[inline(always)]
    pub fn is_stop_at_ovf(&self) -> bool {
        *self == Freesoft::StopAtOvf
    }
    #[doc = "no count stop in debug mode"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == Freesoft::Free
    }
}
#[doc = "Field `FREESOFT` writer - "]
pub type FreesoftW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freesoft>;
impl<'a, REG> FreesoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "counters are blocked"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::Stop)
    }
    #[doc = "stop after overflow"]
    #[inline(always)]
    pub fn stop_at_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::StopAtOvf)
    }
    #[doc = "no count stop in debug mode"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::Free)
    }
}
impl R {
    #[doc = "Bit 0 - QEP watchdog enable"]
    #[inline(always)]
    pub fn wde(&self) -> WdeR {
        WdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QEP unit timer enable"]
    #[inline(always)]
    pub fn ute(&self) -> UteR {
        UteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - QEP capture latch mode"]
    #[inline(always)]
    pub fn qclm(&self) -> QclmR {
        QclmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quadrature position counter enable/software reset"]
    #[inline(always)]
    pub fn qpen(&self) -> QpenR {
        QpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Strobe event latch of position counter"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software initialization of position counter"]
    #[inline(always)]
    pub fn swi(&self) -> SwiR {
        SwiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn iei(&self) -> IeiR {
        IeiR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sei(&self) -> SeiR {
        SeiR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pcrm(&self) -> PcrmR {
        PcrmR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn freesoft(&self) -> FreesoftR {
        FreesoftR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - QEP watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WdeW<QepctlSpec> {
        WdeW::new(self, 0)
    }
    #[doc = "Bit 1 - QEP unit timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn ute(&mut self) -> UteW<QepctlSpec> {
        UteW::new(self, 1)
    }
    #[doc = "Bit 2 - QEP capture latch mode"]
    #[inline(always)]
    #[must_use]
    pub fn qclm(&mut self) -> QclmW<QepctlSpec> {
        QclmW::new(self, 2)
    }
    #[doc = "Bit 3 - Quadrature position counter enable/software reset"]
    #[inline(always)]
    #[must_use]
    pub fn qpen(&mut self) -> QpenW<QepctlSpec> {
        QpenW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn iel(&mut self) -> IelW<QepctlSpec> {
        IelW::new(self, 4)
    }
    #[doc = "Bit 6 - Strobe event latch of position counter"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<QepctlSpec> {
        SelW::new(self, 6)
    }
    #[doc = "Bit 7 - Software initialization of position counter"]
    #[inline(always)]
    #[must_use]
    pub fn swi(&mut self) -> SwiW<QepctlSpec> {
        SwiW::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn iei(&mut self) -> IeiW<QepctlSpec> {
        IeiW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn sei(&mut self) -> SeiW<QepctlSpec> {
        SeiW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn pcrm(&mut self) -> PcrmW<QepctlSpec> {
        PcrmW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn freesoft(&mut self) -> FreesoftW<QepctlSpec> {
        FreesoftW::new(self, 14)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qepctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qepctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QepctlSpec;
impl crate::RegisterSpec for QepctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qepctl::R`](R) reader structure"]
impl crate::Readable for QepctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qepctl::W`](W) writer structure"]
impl crate::Writable for QepctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QEPCTL to value 0"]
impl crate::Resettable for QepctlSpec {
    const RESET_VALUE: u32 = 0;
}
