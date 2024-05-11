#[doc = "Register `NIPR` reader"]
pub type R = crate::R<NiprSpec>;
#[doc = "Register `NIPR` writer"]
pub type W = crate::W<NiprSpec>;
#[doc = "Field `ALINP` reader - Alert Interrupt Node Pointer"]
pub type AlinpR = crate::FieldReader;
#[doc = "Field `ALINP` writer - Alert Interrupt Node Pointer"]
pub type AlinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LECINP` reader - Last Error Code Interrupt Node Pointer"]
pub type LecinpR = crate::FieldReader;
#[doc = "Field `LECINP` writer - Last Error Code Interrupt Node Pointer"]
pub type LecinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRINP` reader - Transfer OK Interrupt Node Pointer"]
pub type TrinpR = crate::FieldReader;
#[doc = "Field `TRINP` writer - Transfer OK Interrupt Node Pointer"]
pub type TrinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFCINP` reader - Frame Counter Interrupt Node Pointer"]
pub type CfcinpR = crate::FieldReader;
#[doc = "Field `CFCINP` writer - Frame Counter Interrupt Node Pointer"]
pub type CfcinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&self) -> AlinpR {
        AlinpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&self) -> LecinpR {
        LecinpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&self) -> TrinpR {
        TrinpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CfcinpR {
        CfcinpR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn alinp(&mut self) -> AlinpW<NiprSpec> {
        AlinpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn lecinp(&mut self) -> LecinpW<NiprSpec> {
        LecinpW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn trinp(&mut self) -> TrinpW<NiprSpec> {
        TrinpW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn cfcinp(&mut self) -> CfcinpW<NiprSpec> {
        CfcinpW::new(self, 12)
    }
}
#[doc = "Node Interrupt Pointer Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NiprSpec;
impl crate::RegisterSpec for NiprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nipr::R`](R) reader structure"]
impl crate::Readable for NiprSpec {}
#[doc = "`write(|w| ..)` method takes [`nipr::W`](W) writer structure"]
impl crate::Writable for NiprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NIPR to value 0"]
impl crate::Resettable for NiprSpec {
    const RESET_VALUE: u32 = 0;
}
