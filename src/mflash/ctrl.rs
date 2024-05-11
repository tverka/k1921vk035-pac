#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `PEN` reader - Prefetch enable bit"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Prefetch enable bit"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN` reader - I-Cache enable bit"]
pub type IcenR = crate::BitReader;
#[doc = "Field `ICEN` writer - I-Cache enable bit"]
pub type IcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEN` reader - D-Cache enable bit"]
pub type DcenR = crate::BitReader;
#[doc = "Field `DCEN` writer - D-Cache enable bit"]
pub type DcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQEN` reader - Interrupt enable bit"]
pub type IrqenR = crate::BitReader;
#[doc = "Field `IRQEN` writer - Interrupt enable bit"]
pub type IrqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFLUSH` writer - Flush I-Cache request bit"]
pub type IflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLUSH` writer - Flush D-Cache request bit"]
pub type DflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAT` reader - Flash latency"]
pub type LatR = crate::FieldReader;
#[doc = "Field `LAT` writer - Flash latency"]
pub type LatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Prefetch enable bit"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I-Cache enable bit"]
    #[inline(always)]
    pub fn icen(&self) -> IcenR {
        IcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - D-Cache enable bit"]
    #[inline(always)]
    pub fn dcen(&self) -> DcenR {
        DcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable bit"]
    #[inline(always)]
    pub fn irqen(&self) -> IrqenR {
        IrqenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Flash latency"]
    #[inline(always)]
    pub fn lat(&self) -> LatR {
        LatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Prefetch enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<CtrlSpec> {
        PenW::new(self, 0)
    }
    #[doc = "Bit 1 - I-Cache enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> IcenW<CtrlSpec> {
        IcenW::new(self, 1)
    }
    #[doc = "Bit 2 - D-Cache enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DcenW<CtrlSpec> {
        DcenW::new(self, 2)
    }
    #[doc = "Bit 4 - Interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn irqen(&mut self) -> IrqenW<CtrlSpec> {
        IrqenW::new(self, 4)
    }
    #[doc = "Bit 8 - Flush I-Cache request bit"]
    #[inline(always)]
    #[must_use]
    pub fn iflush(&mut self) -> IflushW<CtrlSpec> {
        IflushW::new(self, 8)
    }
    #[doc = "Bit 9 - Flush D-Cache request bit"]
    #[inline(always)]
    #[must_use]
    pub fn dflush(&mut self) -> DflushW<CtrlSpec> {
        DflushW::new(self, 9)
    }
    #[doc = "Bits 16:19 - Flash latency"]
    #[inline(always)]
    #[must_use]
    pub fn lat(&mut self) -> LatW<CtrlSpec> {
        LatW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
