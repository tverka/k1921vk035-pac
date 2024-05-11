#[doc = "Register `QEINT` reader"]
pub type R = crate::R<QeintSpec>;
#[doc = "Register `QEINT` writer"]
pub type W = crate::W<QeintSpec>;
#[doc = "Field `PCE` reader - Position counter error interrupt enable"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Position counter error interrupt enable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPE` reader - Quadrature phase error interrupt enable"]
pub type QpeR = crate::BitReader;
#[doc = "Field `QPE` writer - Quadrature phase error interrupt enable"]
pub type QpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDC` reader - Quadrature direction change interrupt enable"]
pub type QdcR = crate::BitReader;
#[doc = "Field `QDC` writer - Quadrature direction change interrupt enable"]
pub type QdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTO` reader - Watchdog time out interrupt enable"]
pub type WtoR = crate::BitReader;
#[doc = "Field `WTO` writer - Watchdog time out interrupt enable"]
pub type WtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCU` reader - Position counter underflow interrupt enable"]
pub type PcuR = crate::BitReader;
#[doc = "Field `PCU` writer - Position counter underflow interrupt enable"]
pub type PcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCO` reader - Position counter overflow interrupt enable"]
pub type PcoR = crate::BitReader;
#[doc = "Field `PCO` writer - Position counter overflow interrupt enable"]
pub type PcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCR` reader - Position-compare ready interrupt enable"]
pub type PcrR = crate::BitReader;
#[doc = "Field `PCR` writer - Position-compare ready interrupt enable"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM` reader - Position-compare match interrupt enable"]
pub type PcmR = crate::BitReader;
#[doc = "Field `PCM` writer - Position-compare match interrupt enable"]
pub type PcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - Strobe event latch interrupt enable"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - Strobe event latch interrupt enable"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEL` reader - Index event latch interrupt enable"]
pub type IelR = crate::BitReader;
#[doc = "Field `IEL` writer - Index event latch interrupt enable"]
pub type IelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTO` reader - Unit time out interrupt enable"]
pub type UtoR = crate::BitReader;
#[doc = "Field `UTO` writer - Unit time out interrupt enable"]
pub type UtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Position counter error interrupt enable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quadrature phase error interrupt enable"]
    #[inline(always)]
    pub fn qpe(&self) -> QpeR {
        QpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quadrature direction change interrupt enable"]
    #[inline(always)]
    pub fn qdc(&self) -> QdcR {
        QdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog time out interrupt enable"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Position counter underflow interrupt enable"]
    #[inline(always)]
    pub fn pcu(&self) -> PcuR {
        PcuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Position counter overflow interrupt enable"]
    #[inline(always)]
    pub fn pco(&self) -> PcoR {
        PcoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Position-compare ready interrupt enable"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Position-compare match interrupt enable"]
    #[inline(always)]
    pub fn pcm(&self) -> PcmR {
        PcmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Strobe event latch interrupt enable"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Index event latch interrupt enable"]
    #[inline(always)]
    pub fn iel(&self) -> IelR {
        IelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Unit time out interrupt enable"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Position counter error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<QeintSpec> {
        PceW::new(self, 1)
    }
    #[doc = "Bit 2 - Quadrature phase error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn qpe(&mut self) -> QpeW<QeintSpec> {
        QpeW::new(self, 2)
    }
    #[doc = "Bit 3 - Quadrature direction change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn qdc(&mut self) -> QdcW<QeintSpec> {
        QdcW::new(self, 3)
    }
    #[doc = "Bit 4 - Watchdog time out interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WtoW<QeintSpec> {
        WtoW::new(self, 4)
    }
    #[doc = "Bit 5 - Position counter underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcu(&mut self) -> PcuW<QeintSpec> {
        PcuW::new(self, 5)
    }
    #[doc = "Bit 6 - Position counter overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pco(&mut self) -> PcoW<QeintSpec> {
        PcoW::new(self, 6)
    }
    #[doc = "Bit 7 - Position-compare ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PcrW<QeintSpec> {
        PcrW::new(self, 7)
    }
    #[doc = "Bit 8 - Position-compare match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcm(&mut self) -> PcmW<QeintSpec> {
        PcmW::new(self, 8)
    }
    #[doc = "Bit 9 - Strobe event latch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<QeintSpec> {
        SelW::new(self, 9)
    }
    #[doc = "Bit 10 - Index event latch interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iel(&mut self) -> IelW<QeintSpec> {
        IelW::new(self, 10)
    }
    #[doc = "Bit 11 - Unit time out interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uto(&mut self) -> UtoW<QeintSpec> {
        UtoW::new(self, 11)
    }
}
#[doc = "Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qeint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qeint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QeintSpec;
impl crate::RegisterSpec for QeintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qeint::R`](R) reader structure"]
impl crate::Readable for QeintSpec {}
#[doc = "`write(|w| ..)` method takes [`qeint::W`](W) writer structure"]
impl crate::Writable for QeintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QEINT to value 0"]
impl crate::Resettable for QeintSpec {
    const RESET_VALUE: u32 = 0;
}
