#[repr(C)]
#[doc = "UARTCFG"]
#[doc(alias = "UARTCFG")]
pub struct _Uartcfg {
    uartcfg: Uartcfg,
}
impl _Uartcfg {
    #[doc = "0x00 - UART clock and reset configuration register"]
    #[inline(always)]
    pub const fn uartcfg(&self) -> &Uartcfg {
        &self.uartcfg
    }
}
#[doc = "UARTCFG (rw) register accessor: UART clock and reset configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartcfg`]
module"]
#[doc(alias = "UARTCFG")]
pub type Uartcfg = crate::Reg<uartcfg::UartcfgSpec>;
#[doc = "UART clock and reset configuration register"]
pub mod uartcfg;
