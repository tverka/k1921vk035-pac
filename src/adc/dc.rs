#[repr(C)]
#[doc = "DC"]
#[doc(alias = "DC")]
pub struct Dc {
    dctl: Dctl,
    dcmp: Dcmp,
    ddata: Ddata,
}
impl Dc {
    #[doc = "0x00 - Digital comparator control register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x04 - Digital comparator range register"]
    #[inline(always)]
    pub const fn dcmp(&self) -> &Dcmp {
        &self.dcmp
    }
    #[doc = "0x08 - Digital comparator last compared data register"]
    #[inline(always)]
    pub const fn ddata(&self) -> &Ddata {
        &self.ddata
    }
}
#[doc = "DCTL (rw) register accessor: Digital comparator control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "Digital comparator control register"]
pub mod dctl;
#[doc = "DCMP (rw) register accessor: Digital comparator range register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcmp`]
module"]
#[doc(alias = "DCMP")]
pub type Dcmp = crate::Reg<dcmp::DcmpSpec>;
#[doc = "Digital comparator range register"]
pub mod dcmp;
#[doc = "DDATA (r) register accessor: Digital comparator last compared data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddata`]
module"]
#[doc(alias = "DDATA")]
pub type Ddata = crate::Reg<ddata::DdataSpec>;
#[doc = "Digital comparator last compared data register"]
pub mod ddata;
