#[doc = "Register `QCLR` reader"]
pub type R = crate::R<QclrSpec>;
#[doc = "Register `QCLR` writer"]
pub type W = crate::W<QclrSpec>;
#[doc = "Field `INT` reader - Global interrupt clear flag"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Global interrupt clear flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Clear position counter error interrupt flag"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Clear position counter error interrupt flag"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPE` reader - Clear quadrature phase error interrupt flag"]
pub type QpeR = crate::BitReader;
#[doc = "Field `QPE` writer - Clear quadrature phase error interrupt flag"]
pub type QpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDC` reader - Clear quadrature direction change interrupt flag"]
pub type QdcR = crate::BitReader;
#[doc = "Field `QDC` writer - Clear quadrature direction change interrupt flag"]
pub type QdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTO` reader - Clear watchdog timeout interrupt flag"]
pub type WtoR = crate::BitReader;
#[doc = "Field `WTO` writer - Clear watchdog timeout interrupt flag"]
pub type WtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCU` reader - Clear position counter underflow interrupt flag"]
pub type PcuR = crate::BitReader;
#[doc = "Field `PCU` writer - Clear position counter underflow interrupt flag"]
pub type PcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCO` reader - Clear position counter overflow interrupt flag"]
pub type PcoR = crate::BitReader;
#[doc = "Field `PCO` writer - Clear position counter overflow interrupt flag"]
pub type PcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCR` reader - Clear position-compare ready interrupt flag"]
pub type PcrR = crate::BitReader;
#[doc = "Field `PCR` writer - Clear position-compare ready interrupt flag"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM` reader - Clear eQEP compare match event interrupt flag"]
pub type PcmR = crate::BitReader;
#[doc = "Field `PCM` writer - Clear eQEP compare match event interrupt flag"]
pub type PcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - Clear strobe event latch interrupt flag"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - Clear strobe event latch interrupt flag"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEL` reader - Clear index event latch interrupt flag"]
pub type IelR = crate::BitReader;
#[doc = "Field `IEL` writer - Clear index event latch interrupt flag"]
pub type IelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTO` reader - Clear unit time out interrupt flag"]
pub type UtoR = crate::BitReader;
#[doc = "Field `UTO` writer - Clear unit time out interrupt flag"]
pub type UtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt clear flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear position counter error interrupt flag"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear quadrature phase error interrupt flag"]
    #[inline(always)]
    pub fn qpe(&self) -> QpeR {
        QpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear quadrature direction change interrupt flag"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear watchdog timeout interrupt flag"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear position counter underflow interrupt flag"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear position counter overflow interrupt flag"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear position-compare ready interrupt flag"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear eQEP compare match event interrupt flag"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear strobe event latch interrupt flag"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear index event latch interrupt flag"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clear unit time out interrupt flag"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<QclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear position counter error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<QclrSpec> {
        PceW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear quadrature phase error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn qpe(&mut self) -> QpeW<QclrSpec> {
        QpeW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear quadrature direction change interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdc(&mut self) -> QdcW<QclrSpec> {
        QdcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear watchdog timeout interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WtoW<QclrSpec> {
        WtoW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear position counter underflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcu(&mut self) -> PcuW<QclrSpec> {
        PcuW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear position counter overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn pco(&mut self) -> PcoW<QclrSpec> {
        PcoW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear position-compare ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PcrW<QclrSpec> {
        PcrW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear eQEP compare match event interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcm(&mut self) -> PcmW<QclrSpec> {
        PcmW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear strobe event latch interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<QclrSpec> {
        SelW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear index event latch interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn iel(&mut self) -> IelW<QclrSpec> {
        IelW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear unit time out interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn uto(&mut self) -> UtoW<QclrSpec> {
        UtoW::new(self, 11)
    }
}
#[doc = "Interrupt Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QclrSpec;
impl crate::RegisterSpec for QclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qclr::R`](R) reader structure"]
impl crate::Readable for QclrSpec {}
#[doc = "`write(|w| ..)` method takes [`qclr::W`](W) writer structure"]
impl crate::Writable for QclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCLR to value 0"]
impl crate::Resettable for QclrSpec {
    const RESET_VALUE: u32 = 0;
}
