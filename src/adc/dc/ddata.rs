#[doc = "Register `DDATA` reader"]
pub type R = crate::R<DdataSpec>;
#[doc = "Field `VAL` reader - Value of last compared AD conversion result"]
pub type ValR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Value of last compared AD conversion result"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Digital comparator last compared data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdataSpec;
impl crate::RegisterSpec for DdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata::R`](R) reader structure"]
impl crate::Readable for DdataSpec {}
#[doc = "`reset()` method sets DDATA to value 0"]
impl crate::Resettable for DdataSpec {
    const RESET_VALUE: u32 = 0;
}
