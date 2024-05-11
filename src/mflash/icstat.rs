#[doc = "Register `ICSTAT` reader"]
pub type R = crate::R<IcstatSpec>;
#[doc = "Field `BUSY` reader - Busy flag for I-Cache flush/test system"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy flag for I-Cache flush/test system"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "ICACHE Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcstatSpec;
impl crate::RegisterSpec for IcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icstat::R`](R) reader structure"]
impl crate::Readable for IcstatSpec {}
#[doc = "`reset()` method sets ICSTAT to value 0"]
impl crate::Resettable for IcstatSpec {
    const RESET_VALUE: u32 = 0;
}
