#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `MASTEREN` reader - Indicate enable DMA"]
pub type MasterenR = crate::BitReader;
#[doc = "State of DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: At rest"]
    Free = 0,
    #[doc = "1: Reading the config data structure"]
    ReadConfigData = 1,
    #[doc = "2: Reading sourse data end pointer"]
    ReadSrcDataEndPtr = 2,
    #[doc = "3: Reading destination data end pointer"]
    ReadDstDataEndPtr = 3,
    #[doc = "4: Reading source data"]
    ReadSrcData = 4,
    #[doc = "5: Writing data to the destination"]
    WrireDstData = 5,
    #[doc = "6: Waiting for a request"]
    WaitReq = 6,
    #[doc = "7: Write config structure of the channel"]
    WriteConfigData = 7,
    #[doc = "8: Suspended"]
    Pause = 8,
    #[doc = "9: Executed"]
    Done = 9,
    #[doc = "10: mode \"peripheral scather-gather\""]
    PeriphScatGath = 10,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - State of DMA"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            0 => Some(State::Free),
            1 => Some(State::ReadConfigData),
            2 => Some(State::ReadSrcDataEndPtr),
            3 => Some(State::ReadDstDataEndPtr),
            4 => Some(State::ReadSrcData),
            5 => Some(State::WrireDstData),
            6 => Some(State::WaitReq),
            7 => Some(State::WriteConfigData),
            8 => Some(State::Pause),
            9 => Some(State::Done),
            10 => Some(State::PeriphScatGath),
            _ => None,
        }
    }
    #[doc = "At rest"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == State::Free
    }
    #[doc = "Reading the config data structure"]
    #[inline(always)]
    pub fn is_read_config_data(&self) -> bool {
        *self == State::ReadConfigData
    }
    #[doc = "Reading sourse data end pointer"]
    #[inline(always)]
    pub fn is_read_src_data_end_ptr(&self) -> bool {
        *self == State::ReadSrcDataEndPtr
    }
    #[doc = "Reading destination data end pointer"]
    #[inline(always)]
    pub fn is_read_dst_data_end_ptr(&self) -> bool {
        *self == State::ReadDstDataEndPtr
    }
    #[doc = "Reading source data"]
    #[inline(always)]
    pub fn is_read_src_data(&self) -> bool {
        *self == State::ReadSrcData
    }
    #[doc = "Writing data to the destination"]
    #[inline(always)]
    pub fn is_wrire_dst_data(&self) -> bool {
        *self == State::WrireDstData
    }
    #[doc = "Waiting for a request"]
    #[inline(always)]
    pub fn is_wait_req(&self) -> bool {
        *self == State::WaitReq
    }
    #[doc = "Write config structure of the channel"]
    #[inline(always)]
    pub fn is_write_config_data(&self) -> bool {
        *self == State::WriteConfigData
    }
    #[doc = "Suspended"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == State::Pause
    }
    #[doc = "Executed"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == State::Done
    }
    #[doc = "mode \"peripheral scather-gather\""]
    #[inline(always)]
    pub fn is_periph_scat_gath(&self) -> bool {
        *self == State::PeriphScatGath
    }
}
#[doc = "Field `CHNLS` reader - Number channel DMA (write: N-1)"]
pub type ChnlsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicate enable DMA"]
    #[inline(always)]
    pub fn masteren(&self) -> MasterenR {
        MasterenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - State of DMA"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number channel DMA (write: N-1)"]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Status DMA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
