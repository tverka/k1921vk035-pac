#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    #[doc = "0: interrupt on 1/8"]
    Lvl18 = 0,
    #[doc = "1: interrupt on 1/4"]
    Lvl14 = 1,
    #[doc = "2: interrupt on 1/2"]
    Lvl12 = 2,
    #[doc = "3: interrupt on 3/4"]
    Lvl34 = 3,
    #[doc = "4: interrupt on 7/8"]
    Lvl78 = 4,
}
impl From<Txiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Txiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Txiflsel {}
#[doc = "Field `TXIFLSEL` reader - "]
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txiflsel> {
        match self.bits {
            0 => Some(Txiflsel::Lvl18),
            1 => Some(Txiflsel::Lvl14),
            2 => Some(Txiflsel::Lvl12),
            3 => Some(Txiflsel::Lvl34),
            4 => Some(Txiflsel::Lvl78),
            _ => None,
        }
    }
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn is_lvl18(&self) -> bool {
        *self == Txiflsel::Lvl18
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn is_lvl14(&self) -> bool {
        *self == Txiflsel::Lvl14
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn is_lvl12(&self) -> bool {
        *self == Txiflsel::Lvl12
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn is_lvl34(&self) -> bool {
        *self == Txiflsel::Lvl34
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn is_lvl78(&self) -> bool {
        *self == Txiflsel::Lvl78
    }
}
#[doc = "Field `TXIFLSEL` writer - "]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txiflsel>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn lvl18(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl18)
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn lvl14(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl14)
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn lvl12(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl12)
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn lvl34(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl34)
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn lvl78(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl78)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    #[doc = "0: interrupt on 1/8"]
    Lvl18 = 0,
    #[doc = "1: interrupt on 1/4"]
    Lvl14 = 1,
    #[doc = "2: interrupt on 1/2"]
    Lvl12 = 2,
    #[doc = "3: interrupt on 3/4"]
    Lvl34 = 3,
    #[doc = "4: interrupt on 7/8"]
    Lvl78 = 4,
}
impl From<Rxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxiflsel {}
#[doc = "Field `RXIFLSEL` reader - "]
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxiflsel> {
        match self.bits {
            0 => Some(Rxiflsel::Lvl18),
            1 => Some(Rxiflsel::Lvl14),
            2 => Some(Rxiflsel::Lvl12),
            3 => Some(Rxiflsel::Lvl34),
            4 => Some(Rxiflsel::Lvl78),
            _ => None,
        }
    }
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn is_lvl18(&self) -> bool {
        *self == Rxiflsel::Lvl18
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn is_lvl14(&self) -> bool {
        *self == Rxiflsel::Lvl14
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn is_lvl12(&self) -> bool {
        *self == Rxiflsel::Lvl12
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn is_lvl34(&self) -> bool {
        *self == Rxiflsel::Lvl34
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn is_lvl78(&self) -> bool {
        *self == Rxiflsel::Lvl78
    }
}
#[doc = "Field `RXIFLSEL` writer - "]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxiflsel>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn lvl18(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl18)
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn lvl14(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl14)
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn lvl12(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl12)
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn lvl34(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl34)
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn lvl78(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl78)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TxiflselW<IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RxiflselW<IflsSpec> {
        RxiflselW::new(self, 3)
    }
}
#[doc = "Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0;
}
