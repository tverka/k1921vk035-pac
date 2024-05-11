#[doc = "Register `QFRC` reader"]
pub type R = crate::R<QfrcSpec>;
#[doc = "Register `QFRC` writer"]
pub type W = crate::W<QfrcSpec>;
#[doc = "Field `PCE` reader - Force position counter error interrupt"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Force position counter error interrupt"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPE` reader - Force quadrature phase error interrupt"]
pub type QpeR = crate::BitReader;
#[doc = "Field `QPE` writer - Force quadrature phase error interrupt"]
pub type QpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDC` reader - Force quadrature direction change interrupt"]
pub type QdcR = crate::BitReader;
#[doc = "Field `QDC` writer - Force quadrature direction change interrupt"]
pub type QdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTO` reader - Force watchdog time out interrupt"]
pub type WtoR = crate::BitReader;
#[doc = "Field `WTO` writer - Force watchdog time out interrupt"]
pub type WtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCU` reader - Force position counter underflow interrupt"]
pub type PcuR = crate::BitReader;
#[doc = "Field `PCU` writer - Force position counter underflow interrupt"]
pub type PcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCO` reader - Force position counter overflow interrupt"]
pub type PcoR = crate::BitReader;
#[doc = "Field `PCO` writer - Force position counter overflow interrupt"]
pub type PcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCR` reader - Force position-compare ready interrupt"]
pub type PcrR = crate::BitReader;
#[doc = "Field `PCR` writer - Force position-compare ready interrupt"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM` reader - Force position-compare match interrupt"]
pub type PcmR = crate::BitReader;
#[doc = "Field `PCM` writer - Force position-compare match interrupt"]
pub type PcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - Force strobe event latch interrupt"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - Force strobe event latch interrupt"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEL` reader - Force index event latch interrupt"]
pub type IelR = crate::BitReader;
#[doc = "Field `IEL` writer - Force index event latch interrupt"]
pub type IelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTO` reader - Force unit time out interrupt"]
pub type UtoR = crate::BitReader;
#[doc = "Field `UTO` writer - Force unit time out interrupt"]
pub type UtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Force position counter error interrupt"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force quadrature phase error interrupt"]
    #[inline(always)]
    pub fn qpe(&self) -> QpeR {
        QpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force quadrature direction change interrupt"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force watchdog time out interrupt"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force position counter underflow interrupt"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force position counter overflow interrupt"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force position-compare ready interrupt"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Force position-compare match interrupt"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force strobe event latch interrupt"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force index event latch interrupt"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force unit time out interrupt"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Force position counter error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<QfrcSpec> {
        PceW::new(self, 1)
    }
    #[doc = "Bit 2 - Force quadrature phase error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qpe(&mut self) -> QpeW<QfrcSpec> {
        QpeW::new(self, 2)
    }
    #[doc = "Bit 3 - Force quadrature direction change interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qdc(&mut self) -> QdcW<QfrcSpec> {
        QdcW::new(self, 3)
    }
    #[doc = "Bit 4 - Force watchdog time out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WtoW<QfrcSpec> {
        WtoW::new(self, 4)
    }
    #[doc = "Bit 5 - Force position counter underflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pcu(&mut self) -> PcuW<QfrcSpec> {
        PcuW::new(self, 5)
    }
    #[doc = "Bit 6 - Force position counter overflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pco(&mut self) -> PcoW<QfrcSpec> {
        PcoW::new(self, 6)
    }
    #[doc = "Bit 7 - Force position-compare ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PcrW<QfrcSpec> {
        PcrW::new(self, 7)
    }
    #[doc = "Bit 8 - Force position-compare match interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pcm(&mut self) -> PcmW<QfrcSpec> {
        PcmW::new(self, 8)
    }
    #[doc = "Bit 9 - Force strobe event latch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<QfrcSpec> {
        SelW::new(self, 9)
    }
    #[doc = "Bit 10 - Force index event latch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn iel(&mut self) -> IelW<QfrcSpec> {
        IelW::new(self, 10)
    }
    #[doc = "Bit 11 - Force unit time out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn uto(&mut self) -> UtoW<QfrcSpec> {
        UtoW::new(self, 11)
    }
}
#[doc = "Interrupt Force register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QfrcSpec;
impl crate::RegisterSpec for QfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qfrc::R`](R) reader structure"]
impl crate::Readable for QfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`qfrc::W`](W) writer structure"]
impl crate::Writable for QfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QFRC to value 0"]
impl crate::Resettable for QfrcSpec {
    const RESET_VALUE: u32 = 0;
}
