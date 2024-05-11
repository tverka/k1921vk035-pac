#[doc = "Register `QPOSCTL` reader"]
pub type R = crate::R<QposctlSpec>;
#[doc = "Register `QPOSCTL` writer"]
pub type W = crate::W<QposctlSpec>;
#[doc = "Field `PCSPW` reader - "]
pub type PcspwR = crate::FieldReader<u16>;
#[doc = "Field `PCSPW` writer - "]
pub type PcspwW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PCE` reader - Position-compare enable/disable"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Position-compare enable/disable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPOL` reader - Polarity of sync output"]
pub type PcpolR = crate::BitReader;
#[doc = "Field `PCPOL` writer - Polarity of sync output"]
pub type PcpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLOAD` reader - Position-compare shadow load mode"]
pub type PcloadR = crate::BitReader;
#[doc = "Field `PCLOAD` writer - Position-compare shadow load mode"]
pub type PcloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSHDW` reader - Position-compare shadow enable"]
pub type PcshdwR = crate::BitReader;
#[doc = "Field `PCSHDW` writer - Position-compare shadow enable"]
pub type PcshdwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pcspw(&self) -> PcspwR {
        PcspwR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Position-compare enable/disable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Polarity of sync output"]
    #[inline(always)]
    pub fn pcpol(&self) -> PcpolR {
        PcpolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Position-compare shadow load mode"]
    #[inline(always)]
    pub fn pcload(&self) -> PcloadR {
        PcloadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Position-compare shadow enable"]
    #[inline(always)]
    pub fn pcshdw(&self) -> PcshdwR {
        PcshdwR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn pcspw(&mut self) -> PcspwW<QposctlSpec> {
        PcspwW::new(self, 0)
    }
    #[doc = "Bit 12 - Position-compare enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PceW<QposctlSpec> {
        PceW::new(self, 12)
    }
    #[doc = "Bit 13 - Polarity of sync output"]
    #[inline(always)]
    #[must_use]
    pub fn pcpol(&mut self) -> PcpolW<QposctlSpec> {
        PcpolW::new(self, 13)
    }
    #[doc = "Bit 14 - Position-compare shadow load mode"]
    #[inline(always)]
    #[must_use]
    pub fn pcload(&mut self) -> PcloadW<QposctlSpec> {
        PcloadW::new(self, 14)
    }
    #[doc = "Bit 15 - Position-compare shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcshdw(&mut self) -> PcshdwW<QposctlSpec> {
        PcshdwW::new(self, 15)
    }
}
#[doc = "Position-compare Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qposctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qposctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QposctlSpec;
impl crate::RegisterSpec for QposctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qposctl::R`](R) reader structure"]
impl crate::Readable for QposctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qposctl::W`](W) writer structure"]
impl crate::Writable for QposctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QPOSCTL to value 0"]
impl crate::Resettable for QposctlSpec {
    const RESET_VALUE: u32 = 0;
}
