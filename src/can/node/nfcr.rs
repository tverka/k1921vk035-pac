#[doc = "Register `NFCR` reader"]
pub type R = crate::R<NfcrSpec>;
#[doc = "Register `NFCR` writer"]
pub type W = crate::W<NfcrSpec>;
#[doc = "Field `CFC` reader - CAN Frame Counter"]
pub type CfcR = crate::FieldReader<u16>;
#[doc = "Field `CFC` writer - CAN Frame Counter"]
pub type CfcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CFSEL` reader - CAN Frame Count Selection"]
pub type CfselR = crate::FieldReader;
#[doc = "Field `CFSEL` writer - CAN Frame Count Selection"]
pub type CfselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CFMOD` reader - CAN Frame Counter Mode"]
pub type CfmodR = crate::FieldReader;
#[doc = "Field `CFMOD` writer - CAN Frame Counter Mode"]
pub type CfmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CFCIE` reader - CAN Frame Counter Interrupt Enable"]
pub type CfcieR = crate::BitReader;
#[doc = "Field `CFCIE` writer - CAN Frame Counter Interrupt Enable"]
pub type CfcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFCOV` reader - CAN Frame Counter Overflow Flag"]
pub type CfcovR = crate::BitReader;
#[doc = "Field `CFCOV` writer - CAN Frame Counter Overflow Flag"]
pub type CfcovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&self) -> CfcR {
        CfcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&self) -> CfselR {
        CfselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&self) -> CfmodR {
        CfmodR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 22 - CAN Frame Counter Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&self) -> CfcieR {
        CfcieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&self) -> CfcovR {
        CfcovR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cfc(&mut self) -> CfcW<NfcrSpec> {
        CfcW::new(self, 0)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cfsel(&mut self) -> CfselW<NfcrSpec> {
        CfselW::new(self, 16)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfmod(&mut self) -> CfmodW<NfcrSpec> {
        CfmodW::new(self, 19)
    }
    #[doc = "Bit 22 - CAN Frame Counter Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfcie(&mut self) -> CfcieW<NfcrSpec> {
        CfcieW::new(self, 22)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfcov(&mut self) -> CfcovW<NfcrSpec> {
        CfcovW::new(self, 23)
    }
}
#[doc = "Node Frame Counter Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NfcrSpec;
impl crate::RegisterSpec for NfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcr::R`](R) reader structure"]
impl crate::Readable for NfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`nfcr::W`](W) writer structure"]
impl crate::Writable for NfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NFCR to value 0"]
impl crate::Resettable for NfcrSpec {
    const RESET_VALUE: u32 = 0;
}
