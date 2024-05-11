#[doc = "Register `CHIPID` reader"]
pub type R = crate::R<ChipidSpec>;
#[doc = "Field `REV` reader - Revision number"]
pub type RevR = crate::FieldReader;
#[doc = "Field `ID` reader - Model ID"]
pub type IdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - Revision number"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - Model ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "Chip identifier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chipid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChipidSpec;
impl crate::RegisterSpec for ChipidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid::R`](R) reader structure"]
impl crate::Readable for ChipidSpec {}
#[doc = "`reset()` method sets CHIPID to value 0"]
impl crate::Resettable for ChipidSpec {
    const RESET_VALUE: u32 = 0;
}
