#[doc = "Register `SRQSTAT` reader"]
pub type R = crate::R<SrqstatSpec>;
#[doc = "Field `RQPTR` reader - Pointer to queue current request"]
pub type RqptrR = crate::FieldReader;
#[doc = "Field `RQBUSY` reader - Active request status"]
pub type RqbusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Pointer to queue current request"]
    #[inline(always)]
    pub fn rqptr(&self) -> RqptrR {
        RqptrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Active request status"]
    #[inline(always)]
    pub fn rqbusy(&self) -> RqbusyR {
        RqbusyR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Sequencer request status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srqstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrqstatSpec;
impl crate::RegisterSpec for SrqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqstat::R`](R) reader structure"]
impl crate::Readable for SrqstatSpec {}
#[doc = "`reset()` method sets SRQSTAT to value 0"]
impl crate::Resettable for SrqstatSpec {
    const RESET_VALUE: u32 = 0;
}
