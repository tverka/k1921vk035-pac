#[repr(C)]
#[doc = "MASKLB"]
#[doc(alias = "MASKLB")]
pub struct _Masklb {
    masklb: Masklb,
}
impl _Masklb {
    #[doc = "0x00 - Mask register low byte of port"]
    #[inline(always)]
    pub const fn masklb(&self) -> &Masklb {
        &self.masklb
    }
}
#[doc = "MASKLB (rw) register accessor: Mask register low byte of port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masklb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masklb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masklb`]
module"]
#[doc(alias = "MASKLB")]
pub type Masklb = crate::Reg<masklb::MasklbSpec>;
#[doc = "Mask register low byte of port"]
pub mod masklb;
