#[repr(C)]
#[doc = "MASKHB"]
#[doc(alias = "MASKHB")]
pub struct _Maskhb {
    maskhb: Maskhb,
}
impl _Maskhb {
    #[doc = "0x00 - Mask register High byte of port"]
    #[inline(always)]
    pub const fn maskhb(&self) -> &Maskhb {
        &self.maskhb
    }
}
#[doc = "MASKHB (rw) register accessor: Mask register High byte of port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskhb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskhb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskhb`]
module"]
#[doc(alias = "MASKHB")]
pub type Maskhb = crate::Reg<maskhb::MaskhbSpec>;
#[doc = "Mask register High byte of port"]
pub mod maskhb;
