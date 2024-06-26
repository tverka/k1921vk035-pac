#[repr(C)]
#[doc = "Node"]
pub struct Node {
    ncr: Ncr,
    nsr: Nsr,
    nipr: Nipr,
    npcr: Npcr,
    nbtr: Nbtr,
    necnt: Necnt,
    nfcr: Nfcr,
    _reserved7: [u8; 0xe0],
    reserved: Reserved,
}
impl Node {
    #[doc = "0x00 - Node control register0"]
    #[inline(always)]
    pub const fn ncr(&self) -> &Ncr {
        &self.ncr
    }
    #[doc = "0x04 - Node Status Register0"]
    #[inline(always)]
    pub const fn nsr(&self) -> &Nsr {
        &self.nsr
    }
    #[doc = "0x08 - Node Interrupt Pointer Register0"]
    #[inline(always)]
    pub const fn nipr(&self) -> &Nipr {
        &self.nipr
    }
    #[doc = "0x0c - Node Port Control Register0"]
    #[inline(always)]
    pub const fn npcr(&self) -> &Npcr {
        &self.npcr
    }
    #[doc = "0x10 - Node Bit Timing Register0"]
    #[inline(always)]
    pub const fn nbtr(&self) -> &Nbtr {
        &self.nbtr
    }
    #[doc = "0x14 - Node Error Counter Register0"]
    #[inline(always)]
    pub const fn necnt(&self) -> &Necnt {
        &self.necnt
    }
    #[doc = "0x18 - Node Frame Counter Register0"]
    #[inline(always)]
    pub const fn nfcr(&self) -> &Nfcr {
        &self.nfcr
    }
    #[doc = "0xfc - No description"]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "NCR (rw) register accessor: Node control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr`]
module"]
#[doc(alias = "NCR")]
pub type Ncr = crate::Reg<ncr::NcrSpec>;
#[doc = "Node control register0"]
pub mod ncr;
#[doc = "NSR (rw) register accessor: Node Status Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsr`]
module"]
#[doc(alias = "NSR")]
pub type Nsr = crate::Reg<nsr::NsrSpec>;
#[doc = "Node Status Register0"]
pub mod nsr;
#[doc = "NIPR (rw) register accessor: Node Interrupt Pointer Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nipr`]
module"]
#[doc(alias = "NIPR")]
pub type Nipr = crate::Reg<nipr::NiprSpec>;
#[doc = "Node Interrupt Pointer Register0"]
pub mod nipr;
#[doc = "NPCR (rw) register accessor: Node Port Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npcr`]
module"]
#[doc(alias = "NPCR")]
pub type Npcr = crate::Reg<npcr::NpcrSpec>;
#[doc = "Node Port Control Register0"]
pub mod npcr;
#[doc = "NBTR (rw) register accessor: Node Bit Timing Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtr`]
module"]
#[doc(alias = "NBTR")]
pub type Nbtr = crate::Reg<nbtr::NbtrSpec>;
#[doc = "Node Bit Timing Register0"]
pub mod nbtr;
#[doc = "NECNT (rw) register accessor: Node Error Counter Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`necnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`necnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@necnt`]
module"]
#[doc(alias = "NECNT")]
pub type Necnt = crate::Reg<necnt::NecntSpec>;
#[doc = "Node Error Counter Register0"]
pub mod necnt;
#[doc = "NFCR (rw) register accessor: Node Frame Counter Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfcr`]
module"]
#[doc(alias = "NFCR")]
pub type Nfcr = crate::Reg<nfcr::NfcrSpec>;
#[doc = "Node Frame Counter Register0"]
pub mod nfcr;
#[doc = "Reserved (rw) register accessor: No description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`]
module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = "No description"]
pub mod reserved;
