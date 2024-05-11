#[doc = "Register `NSR` reader"]
pub type R = crate::R<NsrSpec>;
#[doc = "Register `NSR` writer"]
pub type W = crate::W<NsrSpec>;
#[doc = "Last Error Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lec {
    #[doc = "0: no error"]
    NoErr = 0,
    #[doc = "1: stuff error"]
    StuffErr = 1,
    #[doc = "2: form error"]
    FormErr = 2,
    #[doc = "3: acknowlegment error"]
    AckErr = 3,
    #[doc = "4: bit 1 error"]
    Bit1err = 4,
    #[doc = "5: bit 0 error"]
    Bit0err = 5,
    #[doc = "6: CRC error"]
    Crcerr = 6,
    #[doc = "7: enable hardware write"]
    WriteEn = 7,
}
impl From<Lec> for u8 {
    #[inline(always)]
    fn from(variant: Lec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lec {
    type Ux = u8;
}
impl crate::IsEnum for Lec {}
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LecR = crate::FieldReader<Lec>;
impl LecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lec {
        match self.bits {
            0 => Lec::NoErr,
            1 => Lec::StuffErr,
            2 => Lec::FormErr,
            3 => Lec::AckErr,
            4 => Lec::Bit1err,
            5 => Lec::Bit0err,
            6 => Lec::Crcerr,
            7 => Lec::WriteEn,
            _ => unreachable!(),
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == Lec::NoErr
    }
    #[doc = "stuff error"]
    #[inline(always)]
    pub fn is_stuff_err(&self) -> bool {
        *self == Lec::StuffErr
    }
    #[doc = "form error"]
    #[inline(always)]
    pub fn is_form_err(&self) -> bool {
        *self == Lec::FormErr
    }
    #[doc = "acknowlegment error"]
    #[inline(always)]
    pub fn is_ack_err(&self) -> bool {
        *self == Lec::AckErr
    }
    #[doc = "bit 1 error"]
    #[inline(always)]
    pub fn is_bit1err(&self) -> bool {
        *self == Lec::Bit1err
    }
    #[doc = "bit 0 error"]
    #[inline(always)]
    pub fn is_bit0err(&self) -> bool {
        *self == Lec::Bit0err
    }
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn is_crcerr(&self) -> bool {
        *self == Lec::Crcerr
    }
    #[doc = "enable hardware write"]
    #[inline(always)]
    pub fn is_write_en(&self) -> bool {
        *self == Lec::WriteEn
    }
}
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lec, crate::Safe>;
impl<'a, REG> LecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no error"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::NoErr)
    }
    #[doc = "stuff error"]
    #[inline(always)]
    pub fn stuff_err(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::StuffErr)
    }
    #[doc = "form error"]
    #[inline(always)]
    pub fn form_err(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::FormErr)
    }
    #[doc = "acknowlegment error"]
    #[inline(always)]
    pub fn ack_err(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::AckErr)
    }
    #[doc = "bit 1 error"]
    #[inline(always)]
    pub fn bit1err(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Bit1err)
    }
    #[doc = "bit 0 error"]
    #[inline(always)]
    pub fn bit0err(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Bit0err)
    }
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn crcerr(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Crcerr)
    }
    #[doc = "enable hardware write"]
    #[inline(always)]
    pub fn write_en(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::WriteEn)
    }
}
#[doc = "Field `TXOK` reader - Message Transmitted Successfully"]
pub type TxokR = crate::BitReader;
#[doc = "Field `TXOK` writer - Message Transmitted Successfully"]
pub type TxokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOK` reader - Message Received Successfully"]
pub type RxokR = crate::BitReader;
#[doc = "Field `RXOK` writer - Message Received Successfully"]
pub type RxokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERT` reader - Alert Warning"]
pub type AlertR = crate::BitReader;
#[doc = "Field `ALERT` writer - Alert Warning"]
pub type AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWRN` reader - Error Warning Status"]
pub type EwrnR = crate::BitReader;
#[doc = "Field `BOFF` reader - Bus-Off Status"]
pub type BoffR = crate::BitReader;
#[doc = "Field `LLE` reader - List Length Error"]
pub type LleR = crate::BitReader;
#[doc = "Field `LLE` writer - List Length Error"]
pub type LleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOE` reader - List Object Error"]
pub type LoeR = crate::BitReader;
#[doc = "Field `LOE` writer - List Object Error"]
pub type LoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSACK` reader - Suspend Acknowledge"]
pub type SusackR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TxokR {
        TxokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RxokR {
        RxokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    pub fn alert(&self) -> AlertR {
        AlertR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Warning Status"]
    #[inline(always)]
    pub fn ewrn(&self) -> EwrnR {
        EwrnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus-Off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    pub fn lle(&self) -> LleR {
        LleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    pub fn loe(&self) -> LoeR {
        LoeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Suspend Acknowledge"]
    #[inline(always)]
    pub fn susack(&self) -> SusackR {
        SusackR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LecW<NsrSpec> {
        LecW::new(self, 0)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn txok(&mut self) -> TxokW<NsrSpec> {
        TxokW::new(self, 3)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn rxok(&mut self) -> RxokW<NsrSpec> {
        RxokW::new(self, 4)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    #[must_use]
    pub fn alert(&mut self) -> AlertW<NsrSpec> {
        AlertW::new(self, 5)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    #[must_use]
    pub fn lle(&mut self) -> LleW<NsrSpec> {
        LleW::new(self, 8)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    #[must_use]
    pub fn loe(&mut self) -> LoeW<NsrSpec> {
        LoeW::new(self, 9)
    }
}
#[doc = "Node Status Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsrSpec;
impl crate::RegisterSpec for NsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsr::R`](R) reader structure"]
impl crate::Readable for NsrSpec {}
#[doc = "`write(|w| ..)` method takes [`nsr::W`](W) writer structure"]
impl crate::Writable for NsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSR to value 0"]
impl crate::Resettable for NsrSpec {
    const RESET_VALUE: u32 = 0;
}
