#[doc = "Register `ST` reader"]
pub type R = crate::R<StSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: General - Idle, no valid status information available"]
    Idle = 0,
    #[doc = "1: FS master - Start condition generated"]
    Stdone = 1,
    #[doc = "2: FS master - Repeated start condition generated"]
    Rsdone = 2,
    #[doc = "3: FS master - Arbitration lost, unaddressed slave mode entered"]
    Idlarl = 3,
    #[doc = "4: FS master transmit - Slave address sent, positive ACK"]
    Mtadpa = 4,
    #[doc = "5: FS master transmit - Slave address sent, negative ACK"]
    Mtadna = 5,
    #[doc = "6: FS master transmit - Data byte sent, positive ACK"]
    Mtdapa = 6,
    #[doc = "7: FS master transmit - Data byte sent, negative ACK"]
    Mtdana = 7,
    #[doc = "8: FS master receive - Slave addres sent, positive ACK"]
    Mradpa = 8,
    #[doc = "9: FS master receive - Slave addres sent, negative ACK"]
    Mradna = 9,
    #[doc = "10: FS master receive - Data byte received, positive ACK"]
    Mrdapa = 10,
    #[doc = "11: FS master receive - Data byte received, negative ACK"]
    Mrdana = 11,
    #[doc = "12: FS master - Mastercode transmitted, error detected (positive ACK)"]
    Mtmcer = 12,
    #[doc = "16: FS slave receive - Slave address received, positive ACK"]
    Sradpa = 16,
    #[doc = "17: FS slave receive - Slave address received after arbitration loss, positive ACK"]
    Sraapa = 17,
    #[doc = "18: FS slave receive - Data byte received, positive ACK"]
    Srdapa = 18,
    #[doc = "19: FS slave receive - Data byte received, negative ACK"]
    Srdana = 19,
    #[doc = "20: FS slave transmit - Slave address received, positive ACK"]
    Stadpa = 20,
    #[doc = "21: FS slave transmit - Slave address received, negative ACK"]
    Staapa = 21,
    #[doc = "22: FS slave transmit - Data byte sent, positive ACK"]
    Stdapa = 22,
    #[doc = "23: FS slave transmit - Data byte sent, negative ACK"]
    Stdana = 23,
    #[doc = "24: FS slave transmit alert response - Alert response address received, positive ACK"]
    Satadp = 24,
    #[doc = "25: FS slave transmit alert response - Alert response address received after arbitration loss, positive ACK"]
    Sataap = 25,
    #[doc = "26: FS slave transmit alert response - Alert response data byte sent, positive ACK"]
    Satdap = 26,
    #[doc = "27: FS slave transmit alert response - Alert response data byte sent, negative ACK"]
    Satdan = 27,
    #[doc = "28: FS slave - Slave mode stop condition detected"]
    Sstop = 28,
    #[doc = "29: FS slave - Global call address received, positive ACK"]
    Sgadpa = 29,
    #[doc = "30: FS slave - Global call address received after arbitration loss, positive ACK"]
    Sdaapa = 30,
    #[doc = "31: General - Bus error detected (invalid start or stop condition"]
    Berror = 31,
    #[doc = "33: HS master - Master code transmitted OK - switched to HS mode"]
    Hmtmcok = 33,
    #[doc = "34: HS master - Repeated start condition generated"]
    Hrsdone = 34,
    #[doc = "35: HS master - Arbitration lost, HS unaddressed slave mode entered"]
    Hidlarl = 35,
    #[doc = "36: HS master transmit - Slave address sent, positive ACK"]
    Hmtadpa = 36,
    #[doc = "37: HS master transmit - Slave address sent, negative ACK"]
    Hmtadna = 37,
    #[doc = "38: HS master transmit - Data byte sent, positive ACK"]
    Hmtdapa = 38,
    #[doc = "39: HS master transmit - Data byte sent, negative ACK"]
    Hmtdana = 39,
    #[doc = "40: HS master receive - Slave address sent, positive ACK"]
    Hmradpa = 40,
    #[doc = "41: HS master receive - Slave address sent, negative ACK"]
    Hmradna = 41,
    #[doc = "42: HS master receive - Data byte received, positive ACK"]
    Hmrdapa = 42,
    #[doc = "43: HS master receive - Data byte received, negative ACK"]
    Hmrdana = 43,
    #[doc = "48: HS slave receive - Slave address received, positive ACK"]
    Hsradpa = 48,
    #[doc = "50: HS slave receive - Data byte received, positive ACK"]
    Hsrdapa = 50,
    #[doc = "51: HS slave receive - Data byte received, negative ACK"]
    Hsrdana = 51,
    #[doc = "52: HS slave transmit - Slave address received, positive ACK"]
    Hstadpa = 52,
    #[doc = "54: HS slave transmit - Data byte sent, positive ACK"]
    Hstdapa = 54,
    #[doc = "55: HS slave transmit - Data byte sent, negative ACK"]
    Hstdana = 55,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - "]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Idle),
            1 => Some(Mode::Stdone),
            2 => Some(Mode::Rsdone),
            3 => Some(Mode::Idlarl),
            4 => Some(Mode::Mtadpa),
            5 => Some(Mode::Mtadna),
            6 => Some(Mode::Mtdapa),
            7 => Some(Mode::Mtdana),
            8 => Some(Mode::Mradpa),
            9 => Some(Mode::Mradna),
            10 => Some(Mode::Mrdapa),
            11 => Some(Mode::Mrdana),
            12 => Some(Mode::Mtmcer),
            16 => Some(Mode::Sradpa),
            17 => Some(Mode::Sraapa),
            18 => Some(Mode::Srdapa),
            19 => Some(Mode::Srdana),
            20 => Some(Mode::Stadpa),
            21 => Some(Mode::Staapa),
            22 => Some(Mode::Stdapa),
            23 => Some(Mode::Stdana),
            24 => Some(Mode::Satadp),
            25 => Some(Mode::Sataap),
            26 => Some(Mode::Satdap),
            27 => Some(Mode::Satdan),
            28 => Some(Mode::Sstop),
            29 => Some(Mode::Sgadpa),
            30 => Some(Mode::Sdaapa),
            31 => Some(Mode::Berror),
            33 => Some(Mode::Hmtmcok),
            34 => Some(Mode::Hrsdone),
            35 => Some(Mode::Hidlarl),
            36 => Some(Mode::Hmtadpa),
            37 => Some(Mode::Hmtadna),
            38 => Some(Mode::Hmtdapa),
            39 => Some(Mode::Hmtdana),
            40 => Some(Mode::Hmradpa),
            41 => Some(Mode::Hmradna),
            42 => Some(Mode::Hmrdapa),
            43 => Some(Mode::Hmrdana),
            48 => Some(Mode::Hsradpa),
            50 => Some(Mode::Hsrdapa),
            51 => Some(Mode::Hsrdana),
            52 => Some(Mode::Hstadpa),
            54 => Some(Mode::Hstdapa),
            55 => Some(Mode::Hstdana),
            _ => None,
        }
    }
    #[doc = "General - Idle, no valid status information available"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Mode::Idle
    }
    #[doc = "FS master - Start condition generated"]
    #[inline(always)]
    pub fn is_stdone(&self) -> bool {
        *self == Mode::Stdone
    }
    #[doc = "FS master - Repeated start condition generated"]
    #[inline(always)]
    pub fn is_rsdone(&self) -> bool {
        *self == Mode::Rsdone
    }
    #[doc = "FS master - Arbitration lost, unaddressed slave mode entered"]
    #[inline(always)]
    pub fn is_idlarl(&self) -> bool {
        *self == Mode::Idlarl
    }
    #[doc = "FS master transmit - Slave address sent, positive ACK"]
    #[inline(always)]
    pub fn is_mtadpa(&self) -> bool {
        *self == Mode::Mtadpa
    }
    #[doc = "FS master transmit - Slave address sent, negative ACK"]
    #[inline(always)]
    pub fn is_mtadna(&self) -> bool {
        *self == Mode::Mtadna
    }
    #[doc = "FS master transmit - Data byte sent, positive ACK"]
    #[inline(always)]
    pub fn is_mtdapa(&self) -> bool {
        *self == Mode::Mtdapa
    }
    #[doc = "FS master transmit - Data byte sent, negative ACK"]
    #[inline(always)]
    pub fn is_mtdana(&self) -> bool {
        *self == Mode::Mtdana
    }
    #[doc = "FS master receive - Slave addres sent, positive ACK"]
    #[inline(always)]
    pub fn is_mradpa(&self) -> bool {
        *self == Mode::Mradpa
    }
    #[doc = "FS master receive - Slave addres sent, negative ACK"]
    #[inline(always)]
    pub fn is_mradna(&self) -> bool {
        *self == Mode::Mradna
    }
    #[doc = "FS master receive - Data byte received, positive ACK"]
    #[inline(always)]
    pub fn is_mrdapa(&self) -> bool {
        *self == Mode::Mrdapa
    }
    #[doc = "FS master receive - Data byte received, negative ACK"]
    #[inline(always)]
    pub fn is_mrdana(&self) -> bool {
        *self == Mode::Mrdana
    }
    #[doc = "FS master - Mastercode transmitted, error detected (positive ACK)"]
    #[inline(always)]
    pub fn is_mtmcer(&self) -> bool {
        *self == Mode::Mtmcer
    }
    #[doc = "FS slave receive - Slave address received, positive ACK"]
    #[inline(always)]
    pub fn is_sradpa(&self) -> bool {
        *self == Mode::Sradpa
    }
    #[doc = "FS slave receive - Slave address received after arbitration loss, positive ACK"]
    #[inline(always)]
    pub fn is_sraapa(&self) -> bool {
        *self == Mode::Sraapa
    }
    #[doc = "FS slave receive - Data byte received, positive ACK"]
    #[inline(always)]
    pub fn is_srdapa(&self) -> bool {
        *self == Mode::Srdapa
    }
    #[doc = "FS slave receive - Data byte received, negative ACK"]
    #[inline(always)]
    pub fn is_srdana(&self) -> bool {
        *self == Mode::Srdana
    }
    #[doc = "FS slave transmit - Slave address received, positive ACK"]
    #[inline(always)]
    pub fn is_stadpa(&self) -> bool {
        *self == Mode::Stadpa
    }
    #[doc = "FS slave transmit - Slave address received, negative ACK"]
    #[inline(always)]
    pub fn is_staapa(&self) -> bool {
        *self == Mode::Staapa
    }
    #[doc = "FS slave transmit - Data byte sent, positive ACK"]
    #[inline(always)]
    pub fn is_stdapa(&self) -> bool {
        *self == Mode::Stdapa
    }
    #[doc = "FS slave transmit - Data byte sent, negative ACK"]
    #[inline(always)]
    pub fn is_stdana(&self) -> bool {
        *self == Mode::Stdana
    }
    #[doc = "FS slave transmit alert response - Alert response address received, positive ACK"]
    #[inline(always)]
    pub fn is_satadp(&self) -> bool {
        *self == Mode::Satadp
    }
    #[doc = "FS slave transmit alert response - Alert response address received after arbitration loss, positive ACK"]
    #[inline(always)]
    pub fn is_sataap(&self) -> bool {
        *self == Mode::Sataap
    }
    #[doc = "FS slave transmit alert response - Alert response data byte sent, positive ACK"]
    #[inline(always)]
    pub fn is_satdap(&self) -> bool {
        *self == Mode::Satdap
    }
    #[doc = "FS slave transmit alert response - Alert response data byte sent, negative ACK"]
    #[inline(always)]
    pub fn is_satdan(&self) -> bool {
        *self == Mode::Satdan
    }
    #[doc = "FS slave - Slave mode stop condition detected"]
    #[inline(always)]
    pub fn is_sstop(&self) -> bool {
        *self == Mode::Sstop
    }
    #[doc = "FS slave - Global call address received, positive ACK"]
    #[inline(always)]
    pub fn is_sgadpa(&self) -> bool {
        *self == Mode::Sgadpa
    }
    #[doc = "FS slave - Global call address received after arbitration loss, positive ACK"]
    #[inline(always)]
    pub fn is_sdaapa(&self) -> bool {
        *self == Mode::Sdaapa
    }
    #[doc = "General - Bus error detected (invalid start or stop condition"]
    #[inline(always)]
    pub fn is_berror(&self) -> bool {
        *self == Mode::Berror
    }
    #[doc = "HS master - Master code transmitted OK - switched to HS mode"]
    #[inline(always)]
    pub fn is_hmtmcok(&self) -> bool {
        *self == Mode::Hmtmcok
    }
    #[doc = "HS master - Repeated start condition generated"]
    #[inline(always)]
    pub fn is_hrsdone(&self) -> bool {
        *self == Mode::Hrsdone
    }
    #[doc = "HS master - Arbitration lost, HS unaddressed slave mode entered"]
    #[inline(always)]
    pub fn is_hidlarl(&self) -> bool {
        *self == Mode::Hidlarl
    }
    #[doc = "HS master transmit - Slave address sent, positive ACK"]
    #[inline(always)]
    pub fn is_hmtadpa(&self) -> bool {
        *self == Mode::Hmtadpa
    }
    #[doc = "HS master transmit - Slave address sent, negative ACK"]
    #[inline(always)]
    pub fn is_hmtadna(&self) -> bool {
        *self == Mode::Hmtadna
    }
    #[doc = "HS master transmit - Data byte sent, positive ACK"]
    #[inline(always)]
    pub fn is_hmtdapa(&self) -> bool {
        *self == Mode::Hmtdapa
    }
    #[doc = "HS master transmit - Data byte sent, negative ACK"]
    #[inline(always)]
    pub fn is_hmtdana(&self) -> bool {
        *self == Mode::Hmtdana
    }
    #[doc = "HS master receive - Slave address sent, positive ACK"]
    #[inline(always)]
    pub fn is_hmradpa(&self) -> bool {
        *self == Mode::Hmradpa
    }
    #[doc = "HS master receive - Slave address sent, negative ACK"]
    #[inline(always)]
    pub fn is_hmradna(&self) -> bool {
        *self == Mode::Hmradna
    }
    #[doc = "HS master receive - Data byte received, positive ACK"]
    #[inline(always)]
    pub fn is_hmrdapa(&self) -> bool {
        *self == Mode::Hmrdapa
    }
    #[doc = "HS master receive - Data byte received, negative ACK"]
    #[inline(always)]
    pub fn is_hmrdana(&self) -> bool {
        *self == Mode::Hmrdana
    }
    #[doc = "HS slave receive - Slave address received, positive ACK"]
    #[inline(always)]
    pub fn is_hsradpa(&self) -> bool {
        *self == Mode::Hsradpa
    }
    #[doc = "HS slave receive - Data byte received, positive ACK"]
    #[inline(always)]
    pub fn is_hsrdapa(&self) -> bool {
        *self == Mode::Hsrdapa
    }
    #[doc = "HS slave receive - Data byte received, negative ACK"]
    #[inline(always)]
    pub fn is_hsrdana(&self) -> bool {
        *self == Mode::Hsrdana
    }
    #[doc = "HS slave transmit - Slave address received, positive ACK"]
    #[inline(always)]
    pub fn is_hstadpa(&self) -> bool {
        *self == Mode::Hstadpa
    }
    #[doc = "HS slave transmit - Data byte sent, positive ACK"]
    #[inline(always)]
    pub fn is_hstdapa(&self) -> bool {
        *self == Mode::Hstdapa
    }
    #[doc = "HS slave transmit - Data byte sent, negative ACK"]
    #[inline(always)]
    pub fn is_hstdana(&self) -> bool {
        *self == Mode::Hstdana
    }
}
#[doc = "Field `INT` reader - Interrupt flag"]
pub type IntR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Interrupt flag"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StSpec;
impl crate::RegisterSpec for StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st::R`](R) reader structure"]
impl crate::Readable for StSpec {}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for StSpec {
    const RESET_VALUE: u32 = 0;
}
