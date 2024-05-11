#[doc = "Register `BSTAT` reader"]
pub type R = crate::R<BstatSpec>;
#[doc = "Field `SEQBUSY0` reader - Sequencer 0 busy"]
pub type Seqbusy0R = crate::BitReader;
#[doc = "Field `SEQBUSY1` reader - Sequencer 1 busy"]
pub type Seqbusy1R = crate::BitReader;
#[doc = "Field `ADCBUSY` reader - ADC module conversion busy"]
pub type AdcbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sequencer 0 busy"]
    #[inline(always)]
    pub fn seqbusy0(&self) -> Seqbusy0R {
        Seqbusy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 busy"]
    #[inline(always)]
    pub fn seqbusy1(&self) -> Seqbusy1R {
        Seqbusy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC module conversion busy"]
    #[inline(always)]
    pub fn adcbusy(&self) -> AdcbusyR {
        AdcbusyR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Busy status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BstatSpec;
impl crate::RegisterSpec for BstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bstat::R`](R) reader structure"]
impl crate::Readable for BstatSpec {}
#[doc = "`reset()` method sets BSTAT to value 0"]
impl crate::Resettable for BstatSpec {
    const RESET_VALUE: u32 = 0;
}
