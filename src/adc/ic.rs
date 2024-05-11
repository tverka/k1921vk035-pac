#[doc = "Register `IC` writer"]
pub type W = crate::W<IcSpec>;
#[doc = "Field `SEQIC0` writer - Sequencer 0 interrupt status clear"]
pub type Seqic0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC1` writer - Sequencer 1 interrupt status clear"]
pub type Seqic1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC0` writer - DC 0 interrupt status clear"]
pub type Dcic0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC1` writer - DC 1 interrupt status clear"]
pub type Dcic1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC2` writer - DC 2 interrupt status clear"]
pub type Dcic2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC3` writer - DC 3 interrupt status clear"]
pub type Dcic3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Sequencer 0 interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn seqic0(&mut self) -> Seqic0W<IcSpec> {
        Seqic0W::new(self, 0)
    }
    #[doc = "Bit 1 - Sequencer 1 interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn seqic1(&mut self) -> Seqic1W<IcSpec> {
        Seqic1W::new(self, 1)
    }
    #[doc = "Bit 8 - DC 0 interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn dcic0(&mut self) -> Dcic0W<IcSpec> {
        Dcic0W::new(self, 8)
    }
    #[doc = "Bit 9 - DC 1 interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn dcic1(&mut self) -> Dcic1W<IcSpec> {
        Dcic1W::new(self, 9)
    }
    #[doc = "Bit 10 - DC 2 interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn dcic2(&mut self) -> Dcic2W<IcSpec> {
        Dcic2W::new(self, 10)
    }
    #[doc = "Bit 11 - DC 3 interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn dcic3(&mut self) -> Dcic3W<IcSpec> {
        Dcic3W::new(self, 11)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSpec;
impl crate::RegisterSpec for IcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ic::W`](W) writer structure"]
impl crate::Writable for IcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IcSpec {
    const RESET_VALUE: u32 = 0;
}
