#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msg: [Msg; 64],
}
impl RegisterBlock {
    #[doc = "0x00..0x800 - Msg"]
    #[inline(always)]
    pub const fn msg(&self, n: usize) -> &Msg {
        &self.msg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x800 - Msg"]
    #[inline(always)]
    pub fn msg_iter(&self) -> impl Iterator<Item = &Msg> {
        self.msg.iter()
    }
}
#[doc = "Msg"]
pub use self::msg::Msg;
#[doc = r"Cluster"]
#[doc = "Msg"]
pub mod msg;
