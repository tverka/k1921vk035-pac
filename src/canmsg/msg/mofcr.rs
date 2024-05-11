#[doc = "Register `MOFCR` reader"]
pub type R = crate::R<MofcrSpec>;
#[doc = "Register `MOFCR` writer"]
pub type W = crate::W<MofcrSpec>;
#[doc = "Message Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmc {
    #[doc = "0: message object"]
    MsgObj = 0,
    #[doc = "1: receiver FIFO structure object"]
    Rxobj = 1,
    #[doc = "2: transmitter FIFO structure object"]
    Txobj = 2,
    #[doc = "3: transmitter FIFO structure slave object"]
    SlaveTxobj = 3,
    #[doc = "4: gateway source object"]
    SrcObj = 4,
}
impl From<Mmc> for u8 {
    #[inline(always)]
    fn from(variant: Mmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmc {
    type Ux = u8;
}
impl crate::IsEnum for Mmc {}
#[doc = "Field `MMC` reader - Message Mode Control"]
pub type MmcR = crate::FieldReader<Mmc>;
impl MmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmc> {
        match self.bits {
            0 => Some(Mmc::MsgObj),
            1 => Some(Mmc::Rxobj),
            2 => Some(Mmc::Txobj),
            3 => Some(Mmc::SlaveTxobj),
            4 => Some(Mmc::SrcObj),
            _ => None,
        }
    }
    #[doc = "message object"]
    #[inline(always)]
    pub fn is_msg_obj(&self) -> bool {
        *self == Mmc::MsgObj
    }
    #[doc = "receiver FIFO structure object"]
    #[inline(always)]
    pub fn is_rxobj(&self) -> bool {
        *self == Mmc::Rxobj
    }
    #[doc = "transmitter FIFO structure object"]
    #[inline(always)]
    pub fn is_txobj(&self) -> bool {
        *self == Mmc::Txobj
    }
    #[doc = "transmitter FIFO structure slave object"]
    #[inline(always)]
    pub fn is_slave_txobj(&self) -> bool {
        *self == Mmc::SlaveTxobj
    }
    #[doc = "gateway source object"]
    #[inline(always)]
    pub fn is_src_obj(&self) -> bool {
        *self == Mmc::SrcObj
    }
}
#[doc = "Field `MMC` writer - Message Mode Control"]
pub type MmcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mmc>;
impl<'a, REG> MmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "message object"]
    #[inline(always)]
    pub fn msg_obj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::MsgObj)
    }
    #[doc = "receiver FIFO structure object"]
    #[inline(always)]
    pub fn rxobj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Rxobj)
    }
    #[doc = "transmitter FIFO structure object"]
    #[inline(always)]
    pub fn txobj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Txobj)
    }
    #[doc = "transmitter FIFO structure slave object"]
    #[inline(always)]
    pub fn slave_txobj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::SlaveTxobj)
    }
    #[doc = "gateway source object"]
    #[inline(always)]
    pub fn src_obj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::SrcObj)
    }
}
#[doc = "Field `GDFS` reader - Gateway Data Frame Selected"]
pub type GdfsR = crate::BitReader;
#[doc = "Field `GDFS` writer - Gateway Data Frame Selected"]
pub type GdfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDC` reader - Identifier Copy"]
pub type IdcR = crate::BitReader;
#[doc = "Field `IDC` writer - Identifier Copy"]
pub type IdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLCC` reader - Data Lengh Code Copy"]
pub type DlccR = crate::BitReader;
#[doc = "Field `DLCC` writer - Data Lengh Code Copy"]
pub type DlccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATC` reader - Data Copy"]
pub type DatcR = crate::BitReader;
#[doc = "Field `DATC` writer - Data Copy"]
pub type DatcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub type RxieR = crate::BitReader;
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub type TxieR = crate::BitReader;
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIE` reader - Overflow Interrupt Enable"]
pub type OvieR = crate::BitReader;
#[doc = "Field `OVIE` writer - Overflow Interrupt Enable"]
pub type OvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRREN` reader - Foreign Remote Request Enable"]
pub type FrrenR = crate::BitReader;
#[doc = "Field `FRREN` writer - Foreign Remote Request Enable"]
pub type FrrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMM` reader - Transmit Object Remote Monitoring"]
pub type RmmR = crate::BitReader;
#[doc = "Field `RMM` writer - Transmit Object Remote Monitoring"]
pub type RmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDT` reader - Single Data Transfer"]
pub type SdtR = crate::BitReader;
#[doc = "Field `SDT` writer - Single Data Transfer"]
pub type SdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STT` reader - Single Transmit Trial"]
pub type SttR = crate::BitReader;
#[doc = "Field `STT` writer - Single Transmit Trial"]
pub type SttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Gateway Data Frame Selected"]
    #[inline(always)]
    pub fn gdfs(&self) -> GdfsR {
        GdfsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    pub fn idc(&self) -> IdcR {
        IdcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Lengh Code Copy"]
    #[inline(always)]
    pub fn dlcc(&self) -> DlccR {
        DlccR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    pub fn datc(&self) -> DatcR {
        DatcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OvieR {
        OvieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    pub fn frren(&self) -> FrrenR {
        FrrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    pub fn rmm(&self) -> RmmR {
        RmmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    pub fn sdt(&self) -> SdtR {
        SdtR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    pub fn stt(&self) -> SttR {
        SttR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn mmc(&mut self) -> MmcW<MofcrSpec> {
        MmcW::new(self, 0)
    }
    #[doc = "Bit 8 - Gateway Data Frame Selected"]
    #[inline(always)]
    #[must_use]
    pub fn gdfs(&mut self) -> GdfsW<MofcrSpec> {
        GdfsW::new(self, 8)
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IdcW<MofcrSpec> {
        IdcW::new(self, 9)
    }
    #[doc = "Bit 10 - Data Lengh Code Copy"]
    #[inline(always)]
    #[must_use]
    pub fn dlcc(&mut self) -> DlccW<MofcrSpec> {
        DlccW::new(self, 10)
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline(always)]
    #[must_use]
    pub fn datc(&mut self) -> DatcW<MofcrSpec> {
        DatcW::new(self, 11)
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RxieW<MofcrSpec> {
        RxieW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TxieW<MofcrSpec> {
        TxieW::new(self, 17)
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovie(&mut self) -> OvieW<MofcrSpec> {
        OvieW::new(self, 18)
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frren(&mut self) -> FrrenW<MofcrSpec> {
        FrrenW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn rmm(&mut self) -> RmmW<MofcrSpec> {
        RmmW::new(self, 21)
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sdt(&mut self) -> SdtW<MofcrSpec> {
        SdtW::new(self, 22)
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline(always)]
    #[must_use]
    pub fn stt(&mut self) -> SttW<MofcrSpec> {
        SttW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DlcW<MofcrSpec> {
        DlcW::new(self, 24)
    }
}
#[doc = "Message Object Function Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MofcrSpec;
impl crate::RegisterSpec for MofcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mofcr::R`](R) reader structure"]
impl crate::Readable for MofcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mofcr::W`](W) writer structure"]
impl crate::Writable for MofcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOFCR to value 0"]
impl crate::Resettable for MofcrSpec {
    const RESET_VALUE: u32 = 0;
}
