#[doc = "Register `QCPRDLAT` reader"]
pub type R = crate::R<QcprdlatSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Capture Period Latch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qcprdlat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcprdlatSpec;
impl crate::RegisterSpec for QcprdlatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qcprdlat::R`](R) reader structure"]
impl crate::Readable for QcprdlatSpec {}
#[doc = "`reset()` method sets QCPRDLAT to value 0"]
impl crate::Resettable for QcprdlatSpec {
    const RESET_VALUE: u32 = 0;
}
