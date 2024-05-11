#[doc = "Register `MOIPR` reader"]
pub type R = crate::R<MoiprSpec>;
#[doc = "Register `MOIPR` writer"]
pub type W = crate::W<MoiprSpec>;
#[doc = "Field `RXINP` reader - Receive Interrupt Node Pointer"]
pub type RxinpR = crate::FieldReader;
#[doc = "Field `RXINP` writer - Receive Interrupt Node Pointer"]
pub type RxinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXINP` reader - Transmit Interrupt Node Pointer"]
pub type TxinpR = crate::FieldReader;
#[doc = "Field `TXINP` writer - Transmit Interrupt Node Pointer"]
pub type TxinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MPN` reader - Message Pending Number"]
pub type MpnR = crate::FieldReader;
#[doc = "Field `MPN` writer - Message Pending Number"]
pub type MpnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFCVAL` reader - CAN Frame Counter Value"]
pub type CfcvalR = crate::FieldReader<u16>;
#[doc = "Field `CFCVAL` writer - CAN Frame Counter Value"]
pub type CfcvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&self) -> RxinpR {
        RxinpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&self) -> TxinpR {
        TxinpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&self) -> MpnR {
        MpnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&self) -> CfcvalR {
        CfcvalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rxinp(&mut self) -> RxinpW<MoiprSpec> {
        RxinpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn txinp(&mut self) -> TxinpW<MoiprSpec> {
        TxinpW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    #[must_use]
    pub fn mpn(&mut self) -> MpnW<MoiprSpec> {
        MpnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn cfcval(&mut self) -> CfcvalW<MoiprSpec> {
        CfcvalW::new(self, 16)
    }
}
#[doc = "Message Object Interrupt Pointer Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoiprSpec;
impl crate::RegisterSpec for MoiprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moipr::R`](R) reader structure"]
impl crate::Readable for MoiprSpec {}
#[doc = "`write(|w| ..)` method takes [`moipr::W`](W) writer structure"]
impl crate::Writable for MoiprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOIPR to value 0"]
impl crate::Resettable for MoiprSpec {
    const RESET_VALUE: u32 = 0;
}
