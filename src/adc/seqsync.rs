#[doc = "Register `SEQSYNC` reader"]
pub type R = crate::R<SeqsyncSpec>;
#[doc = "Register `SEQSYNC` writer"]
pub type W = crate::W<SeqsyncSpec>;
#[doc = "Field `SYNC0` reader - Enable sequencer 0 software sync"]
pub type Sync0R = crate::BitReader;
#[doc = "Field `SYNC0` writer - Enable sequencer 0 software sync"]
pub type Sync0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1` reader - Enable sequencer 1 software sync"]
pub type Sync1R = crate::BitReader;
#[doc = "Field `SYNC1` writer - Enable sequencer 1 software sync"]
pub type Sync1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSYNC` writer - Sync all sequencers"]
pub type GsyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable sequencer 0 software sync"]
    #[inline(always)]
    pub fn sync0(&self) -> Sync0R {
        Sync0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1 software sync"]
    #[inline(always)]
    pub fn sync1(&self) -> Sync1R {
        Sync1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sequencer 0 software sync"]
    #[inline(always)]
    #[must_use]
    pub fn sync0(&mut self) -> Sync0W<SeqsyncSpec> {
        Sync0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1 software sync"]
    #[inline(always)]
    #[must_use]
    pub fn sync1(&mut self) -> Sync1W<SeqsyncSpec> {
        Sync1W::new(self, 1)
    }
    #[doc = "Bit 31 - Sync all sequencers"]
    #[inline(always)]
    #[must_use]
    pub fn gsync(&mut self) -> GsyncW<SeqsyncSpec> {
        GsyncW::new(self, 31)
    }
}
#[doc = "Sequencer sync register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqsync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqsync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqsyncSpec;
impl crate::RegisterSpec for SeqsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqsync::R`](R) reader structure"]
impl crate::Readable for SeqsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`seqsync::W`](W) writer structure"]
impl crate::Writable for SeqsyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQSYNC to value 0"]
impl crate::Resettable for SeqsyncSpec {
    const RESET_VALUE: u32 = 0;
}
