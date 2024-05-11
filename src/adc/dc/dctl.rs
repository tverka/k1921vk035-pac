#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DctlSpec>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DctlSpec>;
#[doc = "DC interrupt generation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cim {
    #[doc = "0: multiple trigger mode"]
    Multiple = 0,
    #[doc = "1: single trigger mode"]
    Single = 1,
    #[doc = "2: multiple trigger mode with hysteresis"]
    MultipleHyst = 2,
    #[doc = "3: single trigger mode with hysteresis"]
    SingleHyst = 3,
}
impl From<Cim> for u8 {
    #[inline(always)]
    fn from(variant: Cim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cim {
    type Ux = u8;
}
impl crate::IsEnum for Cim {}
#[doc = "Field `CIM` reader - DC interrupt generation mode"]
pub type CimR = crate::FieldReader<Cim>;
impl CimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cim {
        match self.bits {
            0 => Cim::Multiple,
            1 => Cim::Single,
            2 => Cim::MultipleHyst,
            3 => Cim::SingleHyst,
            _ => unreachable!(),
        }
    }
    #[doc = "multiple trigger mode"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == Cim::Multiple
    }
    #[doc = "single trigger mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Cim::Single
    }
    #[doc = "multiple trigger mode with hysteresis"]
    #[inline(always)]
    pub fn is_multiple_hyst(&self) -> bool {
        *self == Cim::MultipleHyst
    }
    #[doc = "single trigger mode with hysteresis"]
    #[inline(always)]
    pub fn is_single_hyst(&self) -> bool {
        *self == Cim::SingleHyst
    }
}
#[doc = "Field `CIM` writer - DC interrupt generation mode"]
pub type CimW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cim, crate::Safe>;
impl<'a, REG> CimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "multiple trigger mode"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Cim::Multiple)
    }
    #[doc = "single trigger mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Cim::Single)
    }
    #[doc = "multiple trigger mode with hysteresis"]
    #[inline(always)]
    pub fn multiple_hyst(self) -> &'a mut crate::W<REG> {
        self.variant(Cim::MultipleHyst)
    }
    #[doc = "single trigger mode with hysteresis"]
    #[inline(always)]
    pub fn single_hyst(self) -> &'a mut crate::W<REG> {
        self.variant(Cim::SingleHyst)
    }
}
#[doc = "DC interrupt generation compare conditions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cic {
    #[doc = "0: result lower or equal COMP0"]
    Low = 0,
    #[doc = "1: result between COMP0 and COMP1 or equal any of them"]
    Window = 1,
    #[doc = "2: result higher or equal COMP1"]
    High = 2,
}
impl From<Cic> for u8 {
    #[inline(always)]
    fn from(variant: Cic) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cic {
    type Ux = u8;
}
impl crate::IsEnum for Cic {}
#[doc = "Field `CIC` reader - DC interrupt generation compare conditions"]
pub type CicR = crate::FieldReader<Cic>;
impl CicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cic> {
        match self.bits {
            0 => Some(Cic::Low),
            1 => Some(Cic::Window),
            2 => Some(Cic::High),
            _ => None,
        }
    }
    #[doc = "result lower or equal COMP0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cic::Low
    }
    #[doc = "result between COMP0 and COMP1 or equal any of them"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == Cic::Window
    }
    #[doc = "result higher or equal COMP1"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cic::High
    }
}
#[doc = "Field `CIC` writer - DC interrupt generation compare conditions"]
pub type CicW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cic>;
impl<'a, REG> CicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "result lower or equal COMP0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cic::Low)
    }
    #[doc = "result between COMP0 and COMP1 or equal any of them"]
    #[inline(always)]
    pub fn window(self) -> &'a mut crate::W<REG> {
        self.variant(Cic::Window)
    }
    #[doc = "result higher or equal COMP1"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cic::High)
    }
}
#[doc = "Field `CIE` reader - Enable DC interrupt generation"]
pub type CieR = crate::BitReader;
#[doc = "Field `CIE` writer - Enable DC interrupt generation"]
pub type CieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DC output trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctm {
    #[doc = "0: multiple trigger mode"]
    Multiple = 0,
    #[doc = "1: single trigger mode"]
    Single = 1,
    #[doc = "2: multiple trigger mode with hysteresis"]
    MultipleHyst = 2,
    #[doc = "3: single trigger mode with hysteresis"]
    SingleHyst = 3,
}
impl From<Ctm> for u8 {
    #[inline(always)]
    fn from(variant: Ctm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctm {
    type Ux = u8;
}
impl crate::IsEnum for Ctm {}
#[doc = "Field `CTM` reader - DC output trigger mode"]
pub type CtmR = crate::FieldReader<Ctm>;
impl CtmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctm {
        match self.bits {
            0 => Ctm::Multiple,
            1 => Ctm::Single,
            2 => Ctm::MultipleHyst,
            3 => Ctm::SingleHyst,
            _ => unreachable!(),
        }
    }
    #[doc = "multiple trigger mode"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == Ctm::Multiple
    }
    #[doc = "single trigger mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Ctm::Single
    }
    #[doc = "multiple trigger mode with hysteresis"]
    #[inline(always)]
    pub fn is_multiple_hyst(&self) -> bool {
        *self == Ctm::MultipleHyst
    }
    #[doc = "single trigger mode with hysteresis"]
    #[inline(always)]
    pub fn is_single_hyst(&self) -> bool {
        *self == Ctm::SingleHyst
    }
}
#[doc = "Field `CTM` writer - DC output trigger mode"]
pub type CtmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctm, crate::Safe>;
impl<'a, REG> CtmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "multiple trigger mode"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::Multiple)
    }
    #[doc = "single trigger mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::Single)
    }
    #[doc = "multiple trigger mode with hysteresis"]
    #[inline(always)]
    pub fn multiple_hyst(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::MultipleHyst)
    }
    #[doc = "single trigger mode with hysteresis"]
    #[inline(always)]
    pub fn single_hyst(self) -> &'a mut crate::W<REG> {
        self.variant(Ctm::SingleHyst)
    }
}
#[doc = "DC output trigger compare conditions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctc {
    #[doc = "0: result lower or equal COMP0"]
    Low = 0,
    #[doc = "1: result between COMP0 and COMP1 or equal any of them"]
    Window = 1,
    #[doc = "2: result higher or equal COMP1"]
    High = 2,
}
impl From<Ctc> for u8 {
    #[inline(always)]
    fn from(variant: Ctc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctc {
    type Ux = u8;
}
impl crate::IsEnum for Ctc {}
#[doc = "Field `CTC` reader - DC output trigger compare conditions"]
pub type CtcR = crate::FieldReader<Ctc>;
impl CtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctc> {
        match self.bits {
            0 => Some(Ctc::Low),
            1 => Some(Ctc::Window),
            2 => Some(Ctc::High),
            _ => None,
        }
    }
    #[doc = "result lower or equal COMP0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ctc::Low
    }
    #[doc = "result between COMP0 and COMP1 or equal any of them"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == Ctc::Window
    }
    #[doc = "result higher or equal COMP1"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ctc::High
    }
}
#[doc = "Field `CTC` writer - DC output trigger compare conditions"]
pub type CtcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctc>;
impl<'a, REG> CtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "result lower or equal COMP0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ctc::Low)
    }
    #[doc = "result between COMP0 and COMP1 or equal any of them"]
    #[inline(always)]
    pub fn window(self) -> &'a mut crate::W<REG> {
        self.variant(Ctc::Window)
    }
    #[doc = "result higher or equal COMP1"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ctc::High)
    }
}
#[doc = "Field `CTE` reader - Enable DC output trigger"]
pub type CteR = crate::BitReader;
#[doc = "Field `CTE` writer - Enable DC output trigger"]
pub type CteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL` reader - ADC channel selection"]
pub type ChnlR = crate::FieldReader;
#[doc = "Field `CHNL` writer - ADC channel selection"]
pub type ChnlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRC` reader - Select data source for comparation: ADC module (0) or sequencer(1)"]
pub type SrcR = crate::BitReader;
#[doc = "Field `SRC` writer - Select data source for comparation: ADC module (0) or sequencer(1)"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DC interrupt generation mode"]
    #[inline(always)]
    pub fn cim(&self) -> CimR {
        CimR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DC interrupt generation compare conditions"]
    #[inline(always)]
    pub fn cic(&self) -> CicR {
        CicR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable DC interrupt generation"]
    #[inline(always)]
    pub fn cie(&self) -> CieR {
        CieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - DC output trigger mode"]
    #[inline(always)]
    pub fn ctm(&self) -> CtmR {
        CtmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DC output trigger compare conditions"]
    #[inline(always)]
    pub fn ctc(&self) -> CtcR {
        CtcR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable DC output trigger"]
    #[inline(always)]
    pub fn cte(&self) -> CteR {
        CteR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - ADC channel selection"]
    #[inline(always)]
    pub fn chnl(&self) -> ChnlR {
        ChnlR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Select data source for comparation: ADC module (0) or sequencer(1)"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DC interrupt generation mode"]
    #[inline(always)]
    #[must_use]
    pub fn cim(&mut self) -> CimW<DctlSpec> {
        CimW::new(self, 0)
    }
    #[doc = "Bits 2:3 - DC interrupt generation compare conditions"]
    #[inline(always)]
    #[must_use]
    pub fn cic(&mut self) -> CicW<DctlSpec> {
        CicW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable DC interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn cie(&mut self) -> CieW<DctlSpec> {
        CieW::new(self, 4)
    }
    #[doc = "Bits 8:9 - DC output trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctm(&mut self) -> CtmW<DctlSpec> {
        CtmW::new(self, 8)
    }
    #[doc = "Bits 10:11 - DC output trigger compare conditions"]
    #[inline(always)]
    #[must_use]
    pub fn ctc(&mut self) -> CtcW<DctlSpec> {
        CtcW::new(self, 10)
    }
    #[doc = "Bit 12 - Enable DC output trigger"]
    #[inline(always)]
    #[must_use]
    pub fn cte(&mut self) -> CteW<DctlSpec> {
        CteW::new(self, 12)
    }
    #[doc = "Bits 16:17 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn chnl(&mut self) -> ChnlW<DctlSpec> {
        ChnlW::new(self, 16)
    }
    #[doc = "Bit 24 - Select data source for comparation: ADC module (0) or sequencer(1)"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<DctlSpec> {
        SrcW::new(self, 24)
    }
}
#[doc = "Digital comparator control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctlSpec;
impl crate::RegisterSpec for DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DctlSpec {
    const RESET_VALUE: u32 = 0;
}
