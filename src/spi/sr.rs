#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TFE` reader - FIFO buffer empty flag transmitter"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TNF` reader - Indicator the transmitter FIFO buffer is not full"]
pub type TnfR = crate::BitReader;
#[doc = "Field `RNE` reader - Indicate not empty receive buffer"]
pub type RneR = crate::BitReader;
#[doc = "Field `RFF` reader - Indicate full receive buffer"]
pub type RffR = crate::BitReader;
#[doc = "Field `BSY` reader - Activity flag"]
pub type BsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FIFO buffer empty flag transmitter"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicator the transmitter FIFO buffer is not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicate not empty receive buffer"]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicate full receive buffer"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Activity flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "State register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
