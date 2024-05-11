#[repr(C)]
#[doc = "MSID"]
#[doc(alias = "MSID")]
pub struct _Msid {
    msid: Msid,
}
impl _Msid {
    #[doc = "0x00 - Message Index Register0"]
    #[inline(always)]
    pub const fn msid(&self) -> &Msid {
        &self.msid
    }
}
#[doc = "MSID (r) register accessor: Message Index Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msid`]
module"]
#[doc(alias = "MSID")]
pub type Msid = crate::Reg<msid::MsidSpec>;
#[doc = "Message Index Register0"]
pub mod msid;
