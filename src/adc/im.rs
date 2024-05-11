#[doc = "Register `IM` reader"]
pub type R = crate::R<ImSpec>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<ImSpec>;
#[doc = "Field `SEQIM0` reader - Sequencer 0 interrupt mask"]
pub type Seqim0R = crate::BitReader;
#[doc = "Field `SEQIM0` writer - Sequencer 0 interrupt mask"]
pub type Seqim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIM1` reader - Sequencer 1 interrupt mask"]
pub type Seqim1R = crate::BitReader;
#[doc = "Field `SEQIM1` writer - Sequencer 1 interrupt mask"]
pub type Seqim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIM0` reader - Interrupt mask of Digital Comparator 0"]
pub type Dcim0R = crate::BitReader;
#[doc = "Field `DCIM0` writer - Interrupt mask of Digital Comparator 0"]
pub type Dcim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIM1` reader - Interrupt mask of Digital Comparator 1"]
pub type Dcim1R = crate::BitReader;
#[doc = "Field `DCIM1` writer - Interrupt mask of Digital Comparator 1"]
pub type Dcim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIM2` reader - Interrupt mask of Digital Comparator 2"]
pub type Dcim2R = crate::BitReader;
#[doc = "Field `DCIM2` writer - Interrupt mask of Digital Comparator 2"]
pub type Dcim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIM3` reader - Interrupt mask of Digital Comparator 3"]
pub type Dcim3R = crate::BitReader;
#[doc = "Field `DCIM3` writer - Interrupt mask of Digital Comparator 3"]
pub type Dcim3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sequencer 0 interrupt mask"]
    #[inline(always)]
    pub fn seqim0(&self) -> Seqim0R {
        Seqim0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 interrupt mask"]
    #[inline(always)]
    pub fn seqim1(&self) -> Seqim1R {
        Seqim1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt mask of Digital Comparator 0"]
    #[inline(always)]
    pub fn dcim0(&self) -> Dcim0R {
        Dcim0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt mask of Digital Comparator 1"]
    #[inline(always)]
    pub fn dcim1(&self) -> Dcim1R {
        Dcim1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt mask of Digital Comparator 2"]
    #[inline(always)]
    pub fn dcim2(&self) -> Dcim2R {
        Dcim2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt mask of Digital Comparator 3"]
    #[inline(always)]
    pub fn dcim3(&self) -> Dcim3R {
        Dcim3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sequencer 0 interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn seqim0(&mut self) -> Seqim0W<ImSpec> {
        Seqim0W::new(self, 0)
    }
    #[doc = "Bit 1 - Sequencer 1 interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn seqim1(&mut self) -> Seqim1W<ImSpec> {
        Seqim1W::new(self, 1)
    }
    #[doc = "Bit 8 - Interrupt mask of Digital Comparator 0"]
    #[inline(always)]
    #[must_use]
    pub fn dcim0(&mut self) -> Dcim0W<ImSpec> {
        Dcim0W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt mask of Digital Comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn dcim1(&mut self) -> Dcim1W<ImSpec> {
        Dcim1W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt mask of Digital Comparator 2"]
    #[inline(always)]
    #[must_use]
    pub fn dcim2(&mut self) -> Dcim2W<ImSpec> {
        Dcim2W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt mask of Digital Comparator 3"]
    #[inline(always)]
    #[must_use]
    pub fn dcim3(&mut self) -> Dcim3W<ImSpec> {
        Dcim3W::new(self, 11)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImSpec;
impl crate::RegisterSpec for ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for ImSpec {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for ImSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for ImSpec {
    const RESET_VALUE: u32 = 0;
}
