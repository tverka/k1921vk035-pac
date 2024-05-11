#[doc = "Register `SFLOAD` reader"]
pub type R = crate::R<SfloadSpec>;
#[doc = "Field `VAL` reader - Sequencer FIFO current load value"]
pub type ValR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Sequencer FIFO current load value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Sequencer FIFO load status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfload::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfloadSpec;
impl crate::RegisterSpec for SfloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfload::R`](R) reader structure"]
impl crate::Readable for SfloadSpec {}
#[doc = "`reset()` method sets SFLOAD to value 0"]
impl crate::Resettable for SfloadSpec {
    const RESET_VALUE: u32 = 0;
}
