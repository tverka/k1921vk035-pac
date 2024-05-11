#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: Data,
    dataout: Dataout,
    dataoutset: Dataoutset,
    dataoutclr: Dataoutclr,
    dataouttgl: Dataouttgl,
    denset: Denset,
    denclr: Denclr,
    inmode: Inmode,
    pullmode: Pullmode,
    outmode: Outmode,
    drivemode: Drivemode,
    outenset: Outenset,
    outenclr: Outenclr,
    altfuncset: Altfuncset,
    altfuncclr: Altfuncclr,
    _reserved15: [u8; 0x08],
    syncset: Syncset,
    syncclr: Syncclr,
    qualset: Qualset,
    qualclr: Qualclr,
    qualmodeset: Qualmodeset,
    qualmodeclr: Qualmodeclr,
    qualsample: Qualsample,
    intenset: Intenset,
    intenclr: Intenclr,
    inttypeset: Inttypeset,
    inttypeclr: Inttypeclr,
    intpolset: Intpolset,
    intpolclr: Intpolclr,
    intedgeset: Intedgeset,
    intedgeclr: Intedgeclr,
    intstatus: Intstatus,
    dmareqset: Dmareqset,
    dmareqclr: Dmareqclr,
    adcsocset: Adcsocset,
    adcsocclr: Adcsocclr,
    rxevset: Rxevset,
    rxevclr: Rxevclr,
    _reserved_37_lockkey: [u8; 0x04],
    lockset: Lockset,
    lockclr: Lockclr,
    _reserved40: [u8; 0x0358],
    masklb: [Masklb; 256],
    maskhb: [Maskhb; 256],
}
impl RegisterBlock {
    #[doc = "0x00 - Data Input register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x04 - Data output register"]
    #[inline(always)]
    pub const fn dataout(&self) -> &Dataout {
        &self.dataout
    }
    #[doc = "0x08 - Data output set bits register"]
    #[inline(always)]
    pub const fn dataoutset(&self) -> &Dataoutset {
        &self.dataoutset
    }
    #[doc = "0x0c - Data output clear bits register"]
    #[inline(always)]
    pub const fn dataoutclr(&self) -> &Dataoutclr {
        &self.dataoutclr
    }
    #[doc = "0x10 - Data output toogle bits register"]
    #[inline(always)]
    pub const fn dataouttgl(&self) -> &Dataouttgl {
        &self.dataouttgl
    }
    #[doc = "0x14 - Digital function (PAD) enable register"]
    #[inline(always)]
    pub const fn denset(&self) -> &Denset {
        &self.denset
    }
    #[doc = "0x18 - Digital function (PAD) disable register"]
    #[inline(always)]
    pub const fn denclr(&self) -> &Denclr {
        &self.denclr
    }
    #[doc = "0x1c - Select input mode register"]
    #[inline(always)]
    pub const fn inmode(&self) -> &Inmode {
        &self.inmode
    }
    #[doc = "0x20 - Select pull mode register"]
    #[inline(always)]
    pub const fn pullmode(&self) -> &Pullmode {
        &self.pullmode
    }
    #[doc = "0x24 - Select output mode register"]
    #[inline(always)]
    pub const fn outmode(&self) -> &Outmode {
        &self.outmode
    }
    #[doc = "0x28 - Select drive mode register"]
    #[inline(always)]
    pub const fn drivemode(&self) -> &Drivemode {
        &self.drivemode
    }
    #[doc = "0x2c - Output enable register"]
    #[inline(always)]
    pub const fn outenset(&self) -> &Outenset {
        &self.outenset
    }
    #[doc = "0x30 - Output disable register"]
    #[inline(always)]
    pub const fn outenclr(&self) -> &Outenclr {
        &self.outenclr
    }
    #[doc = "0x34 - Alternative function enable register"]
    #[inline(always)]
    pub const fn altfuncset(&self) -> &Altfuncset {
        &self.altfuncset
    }
    #[doc = "0x38 - Alternative function disable register"]
    #[inline(always)]
    pub const fn altfuncclr(&self) -> &Altfuncclr {
        &self.altfuncclr
    }
    #[doc = "0x44 - Additional double flip-flop syncronization enable register"]
    #[inline(always)]
    pub const fn syncset(&self) -> &Syncset {
        &self.syncset
    }
    #[doc = "0x48 - Additional double flip-flop syncronization disable register"]
    #[inline(always)]
    pub const fn syncclr(&self) -> &Syncclr {
        &self.syncclr
    }
    #[doc = "0x4c - Qualifier enable register"]
    #[inline(always)]
    pub const fn qualset(&self) -> &Qualset {
        &self.qualset
    }
    #[doc = "0x50 - Qualifier disable register"]
    #[inline(always)]
    pub const fn qualclr(&self) -> &Qualclr {
        &self.qualclr
    }
    #[doc = "0x54 - Qualifier mode set register"]
    #[inline(always)]
    pub const fn qualmodeset(&self) -> &Qualmodeset {
        &self.qualmodeset
    }
    #[doc = "0x58 - Qualifier mode clear register"]
    #[inline(always)]
    pub const fn qualmodeclr(&self) -> &Qualmodeclr {
        &self.qualmodeclr
    }
    #[doc = "0x5c - Qualifier sample period register"]
    #[inline(always)]
    pub const fn qualsample(&self) -> &Qualsample {
        &self.qualsample
    }
    #[doc = "0x60 - Interrupt enable register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x64 - Interrupt disable register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x68 - Interrupt type set register"]
    #[inline(always)]
    pub const fn inttypeset(&self) -> &Inttypeset {
        &self.inttypeset
    }
    #[doc = "0x6c - Interrupt type clear register"]
    #[inline(always)]
    pub const fn inttypeclr(&self) -> &Inttypeclr {
        &self.inttypeclr
    }
    #[doc = "0x70 - Interrupt polarity set register"]
    #[inline(always)]
    pub const fn intpolset(&self) -> &Intpolset {
        &self.intpolset
    }
    #[doc = "0x74 - Interrupt polarity clear register"]
    #[inline(always)]
    pub const fn intpolclr(&self) -> &Intpolclr {
        &self.intpolclr
    }
    #[doc = "0x78 - Interrupt every edge set register"]
    #[inline(always)]
    pub const fn intedgeset(&self) -> &Intedgeset {
        &self.intedgeset
    }
    #[doc = "0x7c - Interrupt every edge clear register"]
    #[inline(always)]
    pub const fn intedgeclr(&self) -> &Intedgeclr {
        &self.intedgeclr
    }
    #[doc = "0x80 - Interrupt status"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0x84 - DMA request enable register"]
    #[inline(always)]
    pub const fn dmareqset(&self) -> &Dmareqset {
        &self.dmareqset
    }
    #[doc = "0x88 - DMA request disable register"]
    #[inline(always)]
    pub const fn dmareqclr(&self) -> &Dmareqclr {
        &self.dmareqclr
    }
    #[doc = "0x8c - ADC Start Of Conversion enable register"]
    #[inline(always)]
    pub const fn adcsocset(&self) -> &Adcsocset {
        &self.adcsocset
    }
    #[doc = "0x90 - ADC Start Of Conversion disable register"]
    #[inline(always)]
    pub const fn adcsocclr(&self) -> &Adcsocclr {
        &self.adcsocclr
    }
    #[doc = "0x94 - Core RXEV request enable register"]
    #[inline(always)]
    pub const fn rxevset(&self) -> &Rxevset {
        &self.rxevset
    }
    #[doc = "0x98 - Core RXEV request disable register"]
    #[inline(always)]
    pub const fn rxevclr(&self) -> &Rxevclr {
        &self.rxevclr
    }
    #[doc = "0x9c - LOCKSET/LOCKCLR write enable status register"]
    #[inline(always)]
    pub const fn lockstat(&self) -> &Lockstat {
        unsafe { &*(self as *const Self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - Key register to unlock LOCKSET/LOCKCLR registers for write (KEY=0xADEADBEE)"]
    #[inline(always)]
    pub const fn lockkey(&self) -> &Lockkey {
        unsafe { &*(self as *const Self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0xa0 - Lock pins configuration enable register"]
    #[inline(always)]
    pub const fn lockset(&self) -> &Lockset {
        &self.lockset
    }
    #[doc = "0xa4 - Lock pins configuration disable register"]
    #[inline(always)]
    pub const fn lockclr(&self) -> &Lockclr {
        &self.lockclr
    }
    #[doc = "0x400..0x800 - MASKLB"]
    #[inline(always)]
    pub const fn masklb(&self, n: usize) -> &Masklb {
        &self.masklb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x800 - MASKLB"]
    #[inline(always)]
    pub fn masklb_iter(&self) -> impl Iterator<Item = &Masklb> {
        self.masklb.iter()
    }
    #[doc = "0x800..0xc00 - MASKHB"]
    #[inline(always)]
    pub const fn maskhb(&self, n: usize) -> &Maskhb {
        &self.maskhb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0xc00 - MASKHB"]
    #[inline(always)]
    pub fn maskhb_iter(&self) -> impl Iterator<Item = &Maskhb> {
        self.maskhb.iter()
    }
}
#[doc = "DATA (rw) register accessor: Data Input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data Input register"]
pub mod data;
#[doc = "DATAOUT (rw) register accessor: Data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataout`]
module"]
#[doc(alias = "DATAOUT")]
pub type Dataout = crate::Reg<dataout::DataoutSpec>;
#[doc = "Data output register"]
pub mod dataout;
#[doc = "DATAOUTSET (rw) register accessor: Data output set bits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataoutset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataoutset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataoutset`]
module"]
#[doc(alias = "DATAOUTSET")]
pub type Dataoutset = crate::Reg<dataoutset::DataoutsetSpec>;
#[doc = "Data output set bits register"]
pub mod dataoutset;
#[doc = "DATAOUTCLR (rw) register accessor: Data output clear bits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataoutclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataoutclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataoutclr`]
module"]
#[doc(alias = "DATAOUTCLR")]
pub type Dataoutclr = crate::Reg<dataoutclr::DataoutclrSpec>;
#[doc = "Data output clear bits register"]
pub mod dataoutclr;
#[doc = "DATAOUTTGL (rw) register accessor: Data output toogle bits register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataouttgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataouttgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataouttgl`]
module"]
#[doc(alias = "DATAOUTTGL")]
pub type Dataouttgl = crate::Reg<dataouttgl::DataouttglSpec>;
#[doc = "Data output toogle bits register"]
pub mod dataouttgl;
#[doc = "DENSET (rw) register accessor: Digital function (PAD) enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denset`]
module"]
#[doc(alias = "DENSET")]
pub type Denset = crate::Reg<denset::DensetSpec>;
#[doc = "Digital function (PAD) enable register"]
pub mod denset;
#[doc = "DENCLR (rw) register accessor: Digital function (PAD) disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denclr`]
module"]
#[doc(alias = "DENCLR")]
pub type Denclr = crate::Reg<denclr::DenclrSpec>;
#[doc = "Digital function (PAD) disable register"]
pub mod denclr;
#[doc = "INMODE (rw) register accessor: Select input mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inmode`]
module"]
#[doc(alias = "INMODE")]
pub type Inmode = crate::Reg<inmode::InmodeSpec>;
#[doc = "Select input mode register"]
pub mod inmode;
#[doc = "PULLMODE (rw) register accessor: Select pull mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pullmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pullmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pullmode`]
module"]
#[doc(alias = "PULLMODE")]
pub type Pullmode = crate::Reg<pullmode::PullmodeSpec>;
#[doc = "Select pull mode register"]
pub mod pullmode;
#[doc = "OUTMODE (rw) register accessor: Select output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outmode`]
module"]
#[doc(alias = "OUTMODE")]
pub type Outmode = crate::Reg<outmode::OutmodeSpec>;
#[doc = "Select output mode register"]
pub mod outmode;
#[doc = "DRIVEMODE (rw) register accessor: Select drive mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drivemode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drivemode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drivemode`]
module"]
#[doc(alias = "DRIVEMODE")]
pub type Drivemode = crate::Reg<drivemode::DrivemodeSpec>;
#[doc = "Select drive mode register"]
pub mod drivemode;
#[doc = "OUTENSET (rw) register accessor: Output enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outenset`]
module"]
#[doc(alias = "OUTENSET")]
pub type Outenset = crate::Reg<outenset::OutensetSpec>;
#[doc = "Output enable register"]
pub mod outenset;
#[doc = "OUTENCLR (rw) register accessor: Output disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outenclr`]
module"]
#[doc(alias = "OUTENCLR")]
pub type Outenclr = crate::Reg<outenclr::OutenclrSpec>;
#[doc = "Output disable register"]
pub mod outenclr;
#[doc = "ALTFUNCSET (rw) register accessor: Alternative function enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altfuncset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altfuncset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altfuncset`]
module"]
#[doc(alias = "ALTFUNCSET")]
pub type Altfuncset = crate::Reg<altfuncset::AltfuncsetSpec>;
#[doc = "Alternative function enable register"]
pub mod altfuncset;
#[doc = "ALTFUNCCLR (rw) register accessor: Alternative function disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altfuncclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altfuncclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altfuncclr`]
module"]
#[doc(alias = "ALTFUNCCLR")]
pub type Altfuncclr = crate::Reg<altfuncclr::AltfuncclrSpec>;
#[doc = "Alternative function disable register"]
pub mod altfuncclr;
#[doc = "SYNCSET (rw) register accessor: Additional double flip-flop syncronization enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncset`]
module"]
#[doc(alias = "SYNCSET")]
pub type Syncset = crate::Reg<syncset::SyncsetSpec>;
#[doc = "Additional double flip-flop syncronization enable register"]
pub mod syncset;
#[doc = "SYNCCLR (rw) register accessor: Additional double flip-flop syncronization disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncclr`]
module"]
#[doc(alias = "SYNCCLR")]
pub type Syncclr = crate::Reg<syncclr::SyncclrSpec>;
#[doc = "Additional double flip-flop syncronization disable register"]
pub mod syncclr;
#[doc = "QUALSET (rw) register accessor: Qualifier enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qualset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qualset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qualset`]
module"]
#[doc(alias = "QUALSET")]
pub type Qualset = crate::Reg<qualset::QualsetSpec>;
#[doc = "Qualifier enable register"]
pub mod qualset;
#[doc = "QUALCLR (rw) register accessor: Qualifier disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qualclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qualclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qualclr`]
module"]
#[doc(alias = "QUALCLR")]
pub type Qualclr = crate::Reg<qualclr::QualclrSpec>;
#[doc = "Qualifier disable register"]
pub mod qualclr;
#[doc = "QUALMODESET (rw) register accessor: Qualifier mode set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qualmodeset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qualmodeset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qualmodeset`]
module"]
#[doc(alias = "QUALMODESET")]
pub type Qualmodeset = crate::Reg<qualmodeset::QualmodesetSpec>;
#[doc = "Qualifier mode set register"]
pub mod qualmodeset;
#[doc = "QUALMODECLR (rw) register accessor: Qualifier mode clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qualmodeclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qualmodeclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qualmodeclr`]
module"]
#[doc(alias = "QUALMODECLR")]
pub type Qualmodeclr = crate::Reg<qualmodeclr::QualmodeclrSpec>;
#[doc = "Qualifier mode clear register"]
pub mod qualmodeclr;
#[doc = "QUALSAMPLE (rw) register accessor: Qualifier sample period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qualsample::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qualsample::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qualsample`]
module"]
#[doc(alias = "QUALSAMPLE")]
pub type Qualsample = crate::Reg<qualsample::QualsampleSpec>;
#[doc = "Qualifier sample period register"]
pub mod qualsample;
#[doc = "INTENSET (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt enable register"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt disable register"]
pub mod intenclr;
#[doc = "INTTYPESET (rw) register accessor: Interrupt type set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttypeset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttypeset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttypeset`]
module"]
#[doc(alias = "INTTYPESET")]
pub type Inttypeset = crate::Reg<inttypeset::InttypesetSpec>;
#[doc = "Interrupt type set register"]
pub mod inttypeset;
#[doc = "INTTYPECLR (rw) register accessor: Interrupt type clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttypeclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttypeclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttypeclr`]
module"]
#[doc(alias = "INTTYPECLR")]
pub type Inttypeclr = crate::Reg<inttypeclr::InttypeclrSpec>;
#[doc = "Interrupt type clear register"]
pub mod inttypeclr;
#[doc = "INTPOLSET (rw) register accessor: Interrupt polarity set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpolset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpolset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpolset`]
module"]
#[doc(alias = "INTPOLSET")]
pub type Intpolset = crate::Reg<intpolset::IntpolsetSpec>;
#[doc = "Interrupt polarity set register"]
pub mod intpolset;
#[doc = "INTPOLCLR (rw) register accessor: Interrupt polarity clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpolclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpolclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpolclr`]
module"]
#[doc(alias = "INTPOLCLR")]
pub type Intpolclr = crate::Reg<intpolclr::IntpolclrSpec>;
#[doc = "Interrupt polarity clear register"]
pub mod intpolclr;
#[doc = "INTEDGESET (rw) register accessor: Interrupt every edge set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intedgeset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intedgeset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intedgeset`]
module"]
#[doc(alias = "INTEDGESET")]
pub type Intedgeset = crate::Reg<intedgeset::IntedgesetSpec>;
#[doc = "Interrupt every edge set register"]
pub mod intedgeset;
#[doc = "INTEDGECLR (rw) register accessor: Interrupt every edge clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intedgeclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intedgeclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intedgeclr`]
module"]
#[doc(alias = "INTEDGECLR")]
pub type Intedgeclr = crate::Reg<intedgeclr::IntedgeclrSpec>;
#[doc = "Interrupt every edge clear register"]
pub mod intedgeclr;
#[doc = "INTSTATUS (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt status"]
pub mod intstatus;
#[doc = "DMAREQSET (rw) register accessor: DMA request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareqset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareqset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareqset`]
module"]
#[doc(alias = "DMAREQSET")]
pub type Dmareqset = crate::Reg<dmareqset::DmareqsetSpec>;
#[doc = "DMA request enable register"]
pub mod dmareqset;
#[doc = "DMAREQCLR (rw) register accessor: DMA request disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareqclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareqclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareqclr`]
module"]
#[doc(alias = "DMAREQCLR")]
pub type Dmareqclr = crate::Reg<dmareqclr::DmareqclrSpec>;
#[doc = "DMA request disable register"]
pub mod dmareqclr;
#[doc = "ADCSOCSET (rw) register accessor: ADC Start Of Conversion enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsocset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsocset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsocset`]
module"]
#[doc(alias = "ADCSOCSET")]
pub type Adcsocset = crate::Reg<adcsocset::AdcsocsetSpec>;
#[doc = "ADC Start Of Conversion enable register"]
pub mod adcsocset;
#[doc = "ADCSOCCLR (rw) register accessor: ADC Start Of Conversion disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsocclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsocclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsocclr`]
module"]
#[doc(alias = "ADCSOCCLR")]
pub type Adcsocclr = crate::Reg<adcsocclr::AdcsocclrSpec>;
#[doc = "ADC Start Of Conversion disable register"]
pub mod adcsocclr;
#[doc = "RXEVSET (rw) register accessor: Core RXEV request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxevset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxevset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxevset`]
module"]
#[doc(alias = "RXEVSET")]
pub type Rxevset = crate::Reg<rxevset::RxevsetSpec>;
#[doc = "Core RXEV request enable register"]
pub mod rxevset;
#[doc = "RXEVCLR (rw) register accessor: Core RXEV request disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxevclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxevclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxevclr`]
module"]
#[doc(alias = "RXEVCLR")]
pub type Rxevclr = crate::Reg<rxevclr::RxevclrSpec>;
#[doc = "Core RXEV request disable register"]
pub mod rxevclr;
#[doc = "LOCKKEY (w) register accessor: Key register to unlock LOCKSET/LOCKCLR registers for write (KEY=0xADEADBEE)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockkey`]
module"]
#[doc(alias = "LOCKKEY")]
pub type Lockkey = crate::Reg<lockkey::LockkeySpec>;
#[doc = "Key register to unlock LOCKSET/LOCKCLR registers for write (KEY=0xADEADBEE)"]
pub mod lockkey;
#[doc = "LOCKSTAT (r) register accessor: LOCKSET/LOCKCLR write enable status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockstat`]
module"]
#[doc(alias = "LOCKSTAT")]
pub type Lockstat = crate::Reg<lockstat::LockstatSpec>;
#[doc = "LOCKSET/LOCKCLR write enable status register"]
pub mod lockstat;
#[doc = "LOCKSET (rw) register accessor: Lock pins configuration enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockset`]
module"]
#[doc(alias = "LOCKSET")]
pub type Lockset = crate::Reg<lockset::LocksetSpec>;
#[doc = "Lock pins configuration enable register"]
pub mod lockset;
#[doc = "LOCKCLR (rw) register accessor: Lock pins configuration disable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockclr`]
module"]
#[doc(alias = "LOCKCLR")]
pub type Lockclr = crate::Reg<lockclr::LockclrSpec>;
#[doc = "Lock pins configuration disable register"]
pub mod lockclr;
#[doc = "MASKLB"]
pub use self::masklb::Masklb;
#[doc = r"Cluster"]
#[doc = "MASKLB"]
pub mod masklb;
#[doc = "MASKHB"]
pub use self::maskhb::Maskhb;
#[doc = r"Cluster"]
#[doc = "MASKHB"]
pub mod maskhb;
