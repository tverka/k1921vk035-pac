#[repr(C)]
#[doc = "LIST"]
#[doc(alias = "LIST")]
pub struct _List {
    list: List,
}
impl _List {
    #[doc = "0x00 - List Register0"]
    #[inline(always)]
    pub const fn list(&self) -> &List {
        &self.list
    }
}
#[doc = "LIST (r) register accessor: List Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`list::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@list`]
module"]
#[doc(alias = "LIST")]
pub type List = crate::Reg<list::ListSpec>;
#[doc = "List Register0"]
pub mod list;
