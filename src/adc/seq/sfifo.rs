#[doc = "Register `SFIFO` reader"]
pub type R = crate::R<SfifoSpec>;
#[doc = "Field `DATA` reader - AD conversion value"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - AD conversion value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Sequencer FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfifo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfifoSpec;
impl crate::RegisterSpec for SfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfifo::R`](R) reader structure"]
impl crate::Readable for SfifoSpec {}
#[doc = "`reset()` method sets SFIFO to value 0"]
impl crate::Resettable for SfifoSpec {
    const RESET_VALUE: u32 = 0;
}
