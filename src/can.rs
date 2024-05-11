#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clc: Clc,
    _reserved1: [u8; 0x04],
    id: Id,
    fdr: Fdr,
    _reserved3: [u8; 0xf0],
    list: [List; 8],
    _reserved4: [u8; 0x20],
    mspnd: [Mspnd; 4],
    _reserved5: [u8; 0x30],
    msid: [Msid; 4],
    _reserved6: [u8; 0x30],
    msimask: Msimask,
    panctr: Panctr,
    mcr: Mcr,
    mitr: Mitr,
    _reserved10: [u8; 0x30],
    node: [Node; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - CAN Clock Control Register"]
    #[inline(always)]
    pub const fn clc(&self) -> &Clc {
        &self.clc
    }
    #[doc = "0x08 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x0c - Fractional Divider Register"]
    #[inline(always)]
    pub const fn fdr(&self) -> &Fdr {
        &self.fdr
    }
    #[doc = "0x100..0x120 - LIST"]
    #[inline(always)]
    pub const fn list(&self, n: usize) -> &List {
        &self.list[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - LIST"]
    #[inline(always)]
    pub fn list_iter(&self) -> impl Iterator<Item = &List> {
        self.list.iter()
    }
    #[doc = "0x140..0x150 - MSPND"]
    #[inline(always)]
    pub const fn mspnd(&self, n: usize) -> &Mspnd {
        &self.mspnd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - MSPND"]
    #[inline(always)]
    pub fn mspnd_iter(&self) -> impl Iterator<Item = &Mspnd> {
        self.mspnd.iter()
    }
    #[doc = "0x180..0x190 - MSID"]
    #[inline(always)]
    pub const fn msid(&self, n: usize) -> &Msid {
        &self.msid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x190 - MSID"]
    #[inline(always)]
    pub fn msid_iter(&self) -> impl Iterator<Item = &Msid> {
        self.msid.iter()
    }
    #[doc = "0x1c0 - Message Index Mask Register"]
    #[inline(always)]
    pub const fn msimask(&self) -> &Msimask {
        &self.msimask
    }
    #[doc = "0x1c4 - Panel Control Register"]
    #[inline(always)]
    pub const fn panctr(&self) -> &Panctr {
        &self.panctr
    }
    #[doc = "0x1c8 - No description"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x1cc - Module Interrupt Trigger Register"]
    #[inline(always)]
    pub const fn mitr(&self) -> &Mitr {
        &self.mitr
    }
    #[doc = "0x200..0x400 - Node"]
    #[inline(always)]
    pub const fn node(&self, n: usize) -> &Node {
        &self.node[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x400 - Node"]
    #[inline(always)]
    pub fn node_iter(&self) -> impl Iterator<Item = &Node> {
        self.node.iter()
    }
}
#[doc = "CLC (rw) register accessor: CAN Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clc`]
module"]
#[doc(alias = "CLC")]
pub type Clc = crate::Reg<clc::ClcSpec>;
#[doc = "CAN Clock Control Register"]
pub mod clc;
#[doc = "ID (rw) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "FDR (rw) register accessor: Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
#[doc = "Fractional Divider Register"]
pub mod fdr;
#[doc = "LIST"]
pub use self::list::List;
#[doc = r"Cluster"]
#[doc = "LIST"]
pub mod list;
#[doc = "MSPND"]
pub use self::mspnd::Mspnd;
#[doc = r"Cluster"]
#[doc = "MSPND"]
pub mod mspnd;
#[doc = "MSID"]
pub use self::msid::Msid;
#[doc = r"Cluster"]
#[doc = "MSID"]
pub mod msid;
#[doc = "MSIMASK (rw) register accessor: Message Index Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msimask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msimask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msimask`]
module"]
#[doc(alias = "MSIMASK")]
pub type Msimask = crate::Reg<msimask::MsimaskSpec>;
#[doc = "Message Index Mask Register"]
pub mod msimask;
#[doc = "PANCTR (rw) register accessor: Panel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`panctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`panctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@panctr`]
module"]
#[doc(alias = "PANCTR")]
pub type Panctr = crate::Reg<panctr::PanctrSpec>;
#[doc = "Panel Control Register"]
pub mod panctr;
#[doc = "MCR (rw) register accessor: No description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "No description"]
pub mod mcr;
#[doc = "MITR (w) register accessor: Module Interrupt Trigger Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mitr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mitr`]
module"]
#[doc(alias = "MITR")]
pub type Mitr = crate::Reg<mitr::MitrSpec>;
#[doc = "Module Interrupt Trigger Register"]
pub mod mitr;
#[doc = "Node"]
pub use self::node::Node;
#[doc = r"Cluster"]
#[doc = "Node"]
pub mod node;
