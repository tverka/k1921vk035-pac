#[doc = "Register `MOSTAT` reader"]
pub type R = crate::R<MostatSpec>;
#[doc = "Field `RXPND` reader - Receive Pending"]
pub type RxpndR = crate::BitReader;
#[doc = "Field `TXPND` reader - Transmit Pending"]
pub type TxpndR = crate::BitReader;
#[doc = "Field `RXUPD` reader - Receive Updating"]
pub type RxupdR = crate::BitReader;
#[doc = "Field `NEWDAT` reader - New Data"]
pub type NewdatR = crate::BitReader;
#[doc = "Field `MSGLST` reader - Message Lost"]
pub type MsglstR = crate::BitReader;
#[doc = "Field `MSGVAL` reader - Message Valid"]
pub type MsgvalR = crate::BitReader;
#[doc = "Field `RTSEL` reader - Receive/Transmit Selected"]
pub type RtselR = crate::BitReader;
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `TXRQ` reader - Transmit Request"]
pub type TxrqR = crate::BitReader;
#[doc = "Field `TXEN0` reader - Transmit Enable 0"]
pub type Txen0R = crate::BitReader;
#[doc = "Field `TXEN1` reader - Transmit Enable 1"]
pub type Txen1R = crate::BitReader;
#[doc = "Field `DIR` reader - Message Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `LIST` reader - List Allocation"]
pub type ListR = crate::FieldReader;
#[doc = "Field `PPREV` reader - Pointer To Previous Message Object"]
pub type PprevR = crate::FieldReader;
#[doc = "Field `PNEXT` reader - Pointer to Next Message Object"]
pub type PnextR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Pending"]
    #[inline(always)]
    pub fn rxpnd(&self) -> RxpndR {
        RxpndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Pending"]
    #[inline(always)]
    pub fn txpnd(&self) -> TxpndR {
        TxpndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Updating"]
    #[inline(always)]
    pub fn rxupd(&self) -> RxupdR {
        RxupdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New Data"]
    #[inline(always)]
    pub fn newdat(&self) -> NewdatR {
        NewdatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Lost"]
    #[inline(always)]
    pub fn msglst(&self) -> MsglstR {
        MsglstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MsgvalR {
        MsgvalR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive/Transmit Selected"]
    #[inline(always)]
    pub fn rtsel(&self) -> RtselR {
        RtselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrq(&self) -> TxrqR {
        TxrqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Enable 0"]
    #[inline(always)]
    pub fn txen0(&self) -> Txen0R {
        Txen0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Enable 1"]
    #[inline(always)]
    pub fn txen1(&self) -> Txen1R {
        Txen1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - List Allocation"]
    #[inline(always)]
    pub fn list(&self) -> ListR {
        ListR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Pointer To Previous Message Object"]
    #[inline(always)]
    pub fn pprev(&self) -> PprevR {
        PprevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Pointer to Next Message Object"]
    #[inline(always)]
    pub fn pnext(&self) -> PnextR {
        PnextR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Message Object Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mostat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MostatSpec;
impl crate::RegisterSpec for MostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mostat::R`](R) reader structure"]
impl crate::Readable for MostatSpec {}
#[doc = "`reset()` method sets MOSTAT to value 0"]
impl crate::Resettable for MostatSpec {
    const RESET_VALUE: u32 = 0;
}
