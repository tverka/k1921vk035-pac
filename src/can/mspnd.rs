#[repr(C)]
#[doc = "MSPND"]
#[doc(alias = "MSPND")]
pub struct _Mspnd {
    mspnd: Mspnd,
}
impl _Mspnd {
    #[doc = "0x00 - Message Pending Register0"]
    #[inline(always)]
    pub const fn mspnd(&self) -> &Mspnd {
        &self.mspnd
    }
}
#[doc = "MSPND (rw) register accessor: Message Pending Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspnd`]
module"]
#[doc(alias = "MSPND")]
pub type Mspnd = crate::Reg<mspnd::MspndSpec>;
#[doc = "Message Pending Register0"]
pub mod mspnd;
