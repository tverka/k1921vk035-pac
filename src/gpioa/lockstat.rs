#[doc = "Register `LOCKSTAT` reader"]
pub type R = crate::R<LockstatSpec>;
#[doc = "Field `WREN` reader - LOCKSET/LOCKCLR write enable status"]
pub type WrenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LOCKSET/LOCKCLR write enable status"]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new((self.bits & 1) != 0)
    }
}
#[doc = "LOCKSET/LOCKCLR write enable status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockstatSpec;
impl crate::RegisterSpec for LockstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockstat::R`](R) reader structure"]
impl crate::Readable for LockstatSpec {}
#[doc = "`reset()` method sets LOCKSTAT to value 0"]
impl crate::Resettable for LockstatSpec {
    const RESET_VALUE: u32 = 0;
}
