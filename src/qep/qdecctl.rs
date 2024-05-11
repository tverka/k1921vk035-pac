#[doc = "Register `QDECCTL` reader"]
pub type R = crate::R<QdecctlSpec>;
#[doc = "Register `QDECCTL` writer"]
pub type W = crate::W<QdecctlSpec>;
#[doc = "Field `QSP` reader - QEPS input polarity"]
pub type QspR = crate::BitReader;
#[doc = "Field `QSP` writer - QEPS input polarity"]
pub type QspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QIP` reader - QEPI input polarity"]
pub type QipR = crate::BitReader;
#[doc = "Field `QIP` writer - QEPI input polarity"]
pub type QipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QBP` reader - QEPB input polarity"]
pub type QbpR = crate::BitReader;
#[doc = "Field `QBP` writer - QEPB input polarity"]
pub type QbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QAP` reader - QEPA input polarity"]
pub type QapR = crate::BitReader;
#[doc = "Field `QAP` writer - QEPA input polarity"]
pub type QapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGATE` reader - Index pulse gating option"]
pub type IgateR = crate::BitReader;
#[doc = "Field `IGATE` writer - Index pulse gating option"]
pub type IgateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - Swap quadrature clock inputs"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap quadrature clock inputs"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCR` reader - External clock rate"]
pub type XcrR = crate::BitReader;
#[doc = "Field `XCR` writer - External clock rate"]
pub type XcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPSEL` reader - Sync output pin selection"]
pub type SpselR = crate::BitReader;
#[doc = "Field `SPSEL` writer - Sync output pin selection"]
pub type SpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOEN` reader - Sync output-enable"]
pub type SoenR = crate::BitReader;
#[doc = "Field `SOEN` writer - Sync output-enable"]
pub type SoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qsrc {
    #[doc = "0: quadrature mode"]
    Quad = 0,
    #[doc = "1: count/direction mode"]
    CountDir = 1,
    #[doc = "2: count up"]
    Up = 2,
    #[doc = "3: count down"]
    Down = 3,
}
impl From<Qsrc> for u8 {
    #[inline(always)]
    fn from(variant: Qsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qsrc {
    type Ux = u8;
}
impl crate::IsEnum for Qsrc {}
#[doc = "Field `QSRC` reader - "]
pub type QsrcR = crate::FieldReader<Qsrc>;
impl QsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qsrc {
        match self.bits {
            0 => Qsrc::Quad,
            1 => Qsrc::CountDir,
            2 => Qsrc::Up,
            3 => Qsrc::Down,
            _ => unreachable!(),
        }
    }
    #[doc = "quadrature mode"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Qsrc::Quad
    }
    #[doc = "count/direction mode"]
    #[inline(always)]
    pub fn is_count_dir(&self) -> bool {
        *self == Qsrc::CountDir
    }
    #[doc = "count up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Qsrc::Up
    }
    #[doc = "count down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Qsrc::Down
    }
}
#[doc = "Field `QSRC` writer - "]
pub type QsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Qsrc, crate::Safe>;
impl<'a, REG> QsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "quadrature mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::Quad)
    }
    #[doc = "count/direction mode"]
    #[inline(always)]
    pub fn count_dir(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::CountDir)
    }
    #[doc = "count up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::Up)
    }
    #[doc = "count down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Qsrc::Down)
    }
}
impl R {
    #[doc = "Bit 5 - QEPS input polarity"]
    #[inline(always)]
    pub fn qsp(&self) -> QspR {
        QspR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - QEPI input polarity"]
    #[inline(always)]
    pub fn qip(&self) -> QipR {
        QipR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - QEPB input polarity"]
    #[inline(always)]
    pub fn qbp(&self) -> QbpR {
        QbpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - QEPA input polarity"]
    #[inline(always)]
    pub fn qap(&self) -> QapR {
        QapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Index pulse gating option"]
    #[inline(always)]
    pub fn igate(&self) -> IgateR {
        IgateR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Swap quadrature clock inputs"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External clock rate"]
    #[inline(always)]
    pub fn xcr(&self) -> XcrR {
        XcrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sync output pin selection"]
    #[inline(always)]
    pub fn spsel(&self) -> SpselR {
        SpselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Sync output-enable"]
    #[inline(always)]
    pub fn soen(&self) -> SoenR {
        SoenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn qsrc(&self) -> QsrcR {
        QsrcR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - QEPS input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qsp(&mut self) -> QspW<QdecctlSpec> {
        QspW::new(self, 5)
    }
    #[doc = "Bit 6 - QEPI input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qip(&mut self) -> QipW<QdecctlSpec> {
        QipW::new(self, 6)
    }
    #[doc = "Bit 7 - QEPB input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qbp(&mut self) -> QbpW<QdecctlSpec> {
        QbpW::new(self, 7)
    }
    #[doc = "Bit 8 - QEPA input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qap(&mut self) -> QapW<QdecctlSpec> {
        QapW::new(self, 8)
    }
    #[doc = "Bit 9 - Index pulse gating option"]
    #[inline(always)]
    #[must_use]
    pub fn igate(&mut self) -> IgateW<QdecctlSpec> {
        IgateW::new(self, 9)
    }
    #[doc = "Bit 10 - Swap quadrature clock inputs"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SwapW<QdecctlSpec> {
        SwapW::new(self, 10)
    }
    #[doc = "Bit 11 - External clock rate"]
    #[inline(always)]
    #[must_use]
    pub fn xcr(&mut self) -> XcrW<QdecctlSpec> {
        XcrW::new(self, 11)
    }
    #[doc = "Bit 12 - Sync output pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn spsel(&mut self) -> SpselW<QdecctlSpec> {
        SpselW::new(self, 12)
    }
    #[doc = "Bit 13 - Sync output-enable"]
    #[inline(always)]
    #[must_use]
    pub fn soen(&mut self) -> SoenW<QdecctlSpec> {
        SoenW::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn qsrc(&mut self) -> QsrcW<QdecctlSpec> {
        QsrcW::new(self, 14)
    }
}
#[doc = "Decoder Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdecctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdecctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QdecctlSpec;
impl crate::RegisterSpec for QdecctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdecctl::R`](R) reader structure"]
impl crate::Readable for QdecctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qdecctl::W`](W) writer structure"]
impl crate::Writable for QdecctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDECCTL to value 0"]
impl crate::Resettable for QdecctlSpec {
    const RESET_VALUE: u32 = 0;
}
