#[doc = "Register `LIST` reader"]
pub type R = crate::R<ListSpec>;
#[doc = "Field `BEGIN` reader - List Begin"]
pub type BeginR = crate::FieldReader;
#[doc = "Field `END` reader - List End"]
pub type EndR = crate::FieldReader;
#[doc = "Field `SIZE` reader - List Size"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `EMPTY` reader - List Empty Indication"]
pub type EmptyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - List Begin"]
    #[inline(always)]
    pub fn begin(&self) -> BeginR {
        BeginR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - List End"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - List Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - List Empty Indication"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "List Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`list::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ListSpec;
impl crate::RegisterSpec for ListSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`list::R`](R) reader structure"]
impl crate::Readable for ListSpec {}
#[doc = "`reset()` method sets LIST to value 0"]
impl crate::Resettable for ListSpec {
    const RESET_VALUE: u32 = 0;
}
