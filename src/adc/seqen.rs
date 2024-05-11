#[doc = "Register `SEQEN` reader"]
pub type R = crate::R<SeqenSpec>;
#[doc = "Register `SEQEN` writer"]
pub type W = crate::W<SeqenSpec>;
#[doc = "Field `SEQEN0` reader - Enable sequencer 0"]
pub type Seqen0R = crate::BitReader;
#[doc = "Field `SEQEN0` writer - Enable sequencer 0"]
pub type Seqen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN1` reader - Enable sequencer 1"]
pub type Seqen1R = crate::BitReader;
#[doc = "Field `SEQEN1` writer - Enable sequencer 1"]
pub type Seqen1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable sequencer 0"]
    #[inline(always)]
    pub fn seqen0(&self) -> Seqen0R {
        Seqen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1"]
    #[inline(always)]
    pub fn seqen1(&self) -> Seqen1R {
        Seqen1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sequencer 0"]
    #[inline(always)]
    #[must_use]
    pub fn seqen0(&mut self) -> Seqen0W<SeqenSpec> {
        Seqen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1"]
    #[inline(always)]
    #[must_use]
    pub fn seqen1(&mut self) -> Seqen1W<SeqenSpec> {
        Seqen1W::new(self, 1)
    }
}
#[doc = "Enable sequencer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqenSpec;
impl crate::RegisterSpec for SeqenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqen::R`](R) reader structure"]
impl crate::Readable for SeqenSpec {}
#[doc = "`write(|w| ..)` method takes [`seqen::W`](W) writer structure"]
impl crate::Writable for SeqenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQEN to value 0"]
impl crate::Resettable for SeqenSpec {
    const RESET_VALUE: u32 = 0;
}
