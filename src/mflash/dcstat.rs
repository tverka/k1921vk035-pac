#[doc = "Register `DCSTAT` reader"]
pub type R = crate::R<DcstatSpec>;
#[doc = "Field `BUSY` reader - Busy flag for D-Cache flush/test system"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy flag for D-Cache flush/test system"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "DCACHE Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcstatSpec;
impl crate::RegisterSpec for DcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcstat::R`](R) reader structure"]
impl crate::Readable for DcstatSpec {}
#[doc = "`reset()` method sets DCSTAT to value 0"]
impl crate::Resettable for DcstatSpec {
    const RESET_VALUE: u32 = 0;
}
