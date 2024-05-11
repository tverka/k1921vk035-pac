#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FstatSpec>;
#[doc = "Register `FSTAT` writer"]
pub type W = crate::W<FstatSpec>;
#[doc = "Field `OV0` reader - Sequencer 0 FIFO overflow"]
pub type Ov0R = crate::BitReader;
#[doc = "Field `OV0` writer - Sequencer 0 FIFO overflow"]
pub type Ov0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV1` reader - Sequencer 1 FIFO overflow"]
pub type Ov1R = crate::BitReader;
#[doc = "Field `OV1` writer - Sequencer 1 FIFO overflow"]
pub type Ov1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN0` reader - Sequencer 0 FIFO underflow"]
pub type Un0R = crate::BitReader;
#[doc = "Field `UN0` writer - Sequencer 0 FIFO underflow"]
pub type Un0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN1` reader - Sequencer 1 FIFO underflow"]
pub type Un1R = crate::BitReader;
#[doc = "Field `UN1` writer - Sequencer 1 FIFO underflow"]
pub type Un1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV0` reader - Sequencer 0 FIFO DMA request overflow"]
pub type Dov0R = crate::BitReader;
#[doc = "Field `DOV0` writer - Sequencer 0 FIFO DMA request overflow"]
pub type Dov0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV1` reader - Sequencer 1 FIFO DMA request overflow"]
pub type Dov1R = crate::BitReader;
#[doc = "Field `DOV1` writer - Sequencer 1 FIFO DMA request overflow"]
pub type Dov1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sequencer 0 FIFO overflow"]
    #[inline(always)]
    pub fn ov0(&self) -> Ov0R {
        Ov0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO overflow"]
    #[inline(always)]
    pub fn ov1(&self) -> Ov1R {
        Ov1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequencer 0 FIFO underflow"]
    #[inline(always)]
    pub fn un0(&self) -> Un0R {
        Un0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sequencer 1 FIFO underflow"]
    #[inline(always)]
    pub fn un1(&self) -> Un1R {
        Un1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Sequencer 0 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov0(&self) -> Dov0R {
        Dov0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sequencer 1 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov1(&self) -> Dov1R {
        Dov1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sequencer 0 FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ov0(&mut self) -> Ov0W<FstatSpec> {
        Ov0W::new(self, 0)
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ov1(&mut self) -> Ov1W<FstatSpec> {
        Ov1W::new(self, 1)
    }
    #[doc = "Bit 8 - Sequencer 0 FIFO underflow"]
    #[inline(always)]
    #[must_use]
    pub fn un0(&mut self) -> Un0W<FstatSpec> {
        Un0W::new(self, 8)
    }
    #[doc = "Bit 9 - Sequencer 1 FIFO underflow"]
    #[inline(always)]
    #[must_use]
    pub fn un1(&mut self) -> Un1W<FstatSpec> {
        Un1W::new(self, 9)
    }
    #[doc = "Bit 16 - Sequencer 0 FIFO DMA request overflow"]
    #[inline(always)]
    #[must_use]
    pub fn dov0(&mut self) -> Dov0W<FstatSpec> {
        Dov0W::new(self, 16)
    }
    #[doc = "Bit 17 - Sequencer 1 FIFO DMA request overflow"]
    #[inline(always)]
    #[must_use]
    pub fn dov1(&mut self) -> Dov1W<FstatSpec> {
        Dov1W::new(self, 17)
    }
}
#[doc = "FIFO overflow status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstatSpec;
impl crate::RegisterSpec for FstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fstat::W`](W) writer structure"]
impl crate::Writable for FstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FstatSpec {
    const RESET_VALUE: u32 = 0;
}
