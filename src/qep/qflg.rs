#[doc = "Register `QFLG` reader"]
pub type R = crate::R<QflgSpec>;
#[doc = "Field `INT` reader - Global interrupt status flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `PCE` reader - Position counter error interrupt flag"]
pub type PceR = crate::BitReader;
#[doc = "Field `QPE` reader - Quadrature phase error interrupt flag"]
pub type QpeR = crate::BitReader;
#[doc = "Field `QDC` reader - Quadrature direction change interrupt flag"]
pub type QdcR = crate::BitReader;
#[doc = "Field `WTO` reader - Watchdog timeout interrupt flag"]
pub type WtoR = crate::BitReader;
#[doc = "Field `PCU` reader - Position counter underflow interrupt flag"]
pub type PcuR = crate::BitReader;
#[doc = "Field `PCO` reader - Position counter overflow interrupt flag"]
pub type PcoR = crate::BitReader;
#[doc = "Field `PCR` reader - Position-compare ready interrupt flag"]
pub type PcrR = crate::BitReader;
#[doc = "Field `PCM` reader - QEP compare match event interrupt flag"]
pub type PcmR = crate::BitReader;
#[doc = "Field `SEL` reader - Strobe event latch interrupt flag"]
pub type SelR = crate::BitReader;
#[doc = "Field `IEL` reader - Index event latch interrupt flag"]
pub type IelR = crate::BitReader;
#[doc = "Field `UTO` reader - Unit time out interrupt flag"]
pub type UtoR = crate::BitReader;
#[doc = "Field `QFLGLAT` reader - Latches QFLG\\[11:0\\]
on every QPOSCNT read"]
pub type QflglatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Global interrupt status flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Position counter error interrupt flag"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quadrature phase error interrupt flag"]
    #[inline(always)]
    pub fn qpe(&self) -> QpeR {
        QpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quadrature direction change interrupt flag"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog timeout interrupt flag"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Position counter underflow interrupt flag"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Position counter overflow interrupt flag"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Position-compare ready interrupt flag"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - QEP compare match event interrupt flag"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Strobe event latch interrupt flag"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Index event latch interrupt flag"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Unit time out interrupt flag"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Latches QFLG\\[11:0\\]
on every QPOSCNT read"]
    #[inline(always)]
    pub fn qflglat(&self) -> QflglatR {
        QflglatR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qflg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QflgSpec;
impl crate::RegisterSpec for QflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qflg::R`](R) reader structure"]
impl crate::Readable for QflgSpec {}
#[doc = "`reset()` method sets QFLG to value 0"]
impl crate::Resettable for QflgSpec {
    const RESET_VALUE: u32 = 0;
}
