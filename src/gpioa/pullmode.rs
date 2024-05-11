#[doc = "Register `PULLMODE` reader"]
pub type R = crate::R<PullmodeSpec>;
#[doc = "Register `PULLMODE` writer"]
pub type W = crate::W<PullmodeSpec>;
#[doc = "Select pull mode for pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin0 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin0> for u8 {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin0 {
    type Ux = u8;
}
impl crate::IsEnum for Pin0 {}
#[doc = "Field `PIN0` reader - Select pull mode for pin 0"]
pub type Pin0R = crate::FieldReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin0> {
        match self.bits {
            0 => Some(Pin0::Disable),
            1 => Some(Pin0::Pu),
            2 => Some(Pin0::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin0::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin0::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin0::Pd
    }
}
#[doc = "Field `PIN0` writer - Select pull mode for pin 0"]
pub type Pin0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin0>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Pd)
    }
}
#[doc = "Select pull mode for pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin1 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin1> for u8 {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin1 {
    type Ux = u8;
}
impl crate::IsEnum for Pin1 {}
#[doc = "Field `PIN1` reader - Select pull mode for pin 1"]
pub type Pin1R = crate::FieldReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin1> {
        match self.bits {
            0 => Some(Pin1::Disable),
            1 => Some(Pin1::Pu),
            2 => Some(Pin1::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin1::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin1::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin1::Pd
    }
}
#[doc = "Field `PIN1` writer - Select pull mode for pin 1"]
pub type Pin1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin1>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Pd)
    }
}
#[doc = "Select pull mode for pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin2 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin2> for u8 {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin2 {
    type Ux = u8;
}
impl crate::IsEnum for Pin2 {}
#[doc = "Field `PIN2` reader - Select pull mode for pin 2"]
pub type Pin2R = crate::FieldReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin2> {
        match self.bits {
            0 => Some(Pin2::Disable),
            1 => Some(Pin2::Pu),
            2 => Some(Pin2::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin2::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin2::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin2::Pd
    }
}
#[doc = "Field `PIN2` writer - Select pull mode for pin 2"]
pub type Pin2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin2>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Pd)
    }
}
#[doc = "Select pull mode for pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin3 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin3> for u8 {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin3 {
    type Ux = u8;
}
impl crate::IsEnum for Pin3 {}
#[doc = "Field `PIN3` reader - Select pull mode for pin 3"]
pub type Pin3R = crate::FieldReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin3> {
        match self.bits {
            0 => Some(Pin3::Disable),
            1 => Some(Pin3::Pu),
            2 => Some(Pin3::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin3::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin3::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin3::Pd
    }
}
#[doc = "Field `PIN3` writer - Select pull mode for pin 3"]
pub type Pin3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin3>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Pd)
    }
}
#[doc = "Select pull mode for pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin4 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin4> for u8 {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin4 {
    type Ux = u8;
}
impl crate::IsEnum for Pin4 {}
#[doc = "Field `PIN4` reader - Select pull mode for pin 4"]
pub type Pin4R = crate::FieldReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin4> {
        match self.bits {
            0 => Some(Pin4::Disable),
            1 => Some(Pin4::Pu),
            2 => Some(Pin4::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin4::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin4::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin4::Pd
    }
}
#[doc = "Field `PIN4` writer - Select pull mode for pin 4"]
pub type Pin4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin4>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Pd)
    }
}
#[doc = "Select pull mode for pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin5 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin5> for u8 {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin5 {
    type Ux = u8;
}
impl crate::IsEnum for Pin5 {}
#[doc = "Field `PIN5` reader - Select pull mode for pin 5"]
pub type Pin5R = crate::FieldReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin5> {
        match self.bits {
            0 => Some(Pin5::Disable),
            1 => Some(Pin5::Pu),
            2 => Some(Pin5::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin5::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin5::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin5::Pd
    }
}
#[doc = "Field `PIN5` writer - Select pull mode for pin 5"]
pub type Pin5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin5>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Pd)
    }
}
#[doc = "Select pull mode for pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin6 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin6> for u8 {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin6 {
    type Ux = u8;
}
impl crate::IsEnum for Pin6 {}
#[doc = "Field `PIN6` reader - Select pull mode for pin 6"]
pub type Pin6R = crate::FieldReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin6> {
        match self.bits {
            0 => Some(Pin6::Disable),
            1 => Some(Pin6::Pu),
            2 => Some(Pin6::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin6::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin6::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin6::Pd
    }
}
#[doc = "Field `PIN6` writer - Select pull mode for pin 6"]
pub type Pin6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin6>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Pd)
    }
}
#[doc = "Select pull mode for pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin7 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin7> for u8 {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin7 {
    type Ux = u8;
}
impl crate::IsEnum for Pin7 {}
#[doc = "Field `PIN7` reader - Select pull mode for pin 7"]
pub type Pin7R = crate::FieldReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin7> {
        match self.bits {
            0 => Some(Pin7::Disable),
            1 => Some(Pin7::Pu),
            2 => Some(Pin7::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin7::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin7::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin7::Pd
    }
}
#[doc = "Field `PIN7` writer - Select pull mode for pin 7"]
pub type Pin7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin7>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Pd)
    }
}
#[doc = "Select pull mode for pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin8 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin8> for u8 {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin8 {
    type Ux = u8;
}
impl crate::IsEnum for Pin8 {}
#[doc = "Field `PIN8` reader - Select pull mode for pin 8"]
pub type Pin8R = crate::FieldReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin8> {
        match self.bits {
            0 => Some(Pin8::Disable),
            1 => Some(Pin8::Pu),
            2 => Some(Pin8::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin8::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin8::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin8::Pd
    }
}
#[doc = "Field `PIN8` writer - Select pull mode for pin 8"]
pub type Pin8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin8>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Pd)
    }
}
#[doc = "Select pull mode for pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin9 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin9> for u8 {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin9 {
    type Ux = u8;
}
impl crate::IsEnum for Pin9 {}
#[doc = "Field `PIN9` reader - Select pull mode for pin 9"]
pub type Pin9R = crate::FieldReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin9> {
        match self.bits {
            0 => Some(Pin9::Disable),
            1 => Some(Pin9::Pu),
            2 => Some(Pin9::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin9::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin9::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin9::Pd
    }
}
#[doc = "Field `PIN9` writer - Select pull mode for pin 9"]
pub type Pin9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin9>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Pd)
    }
}
#[doc = "Select pull mode for pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin10 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin10> for u8 {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin10 {
    type Ux = u8;
}
impl crate::IsEnum for Pin10 {}
#[doc = "Field `PIN10` reader - Select pull mode for pin 10"]
pub type Pin10R = crate::FieldReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin10> {
        match self.bits {
            0 => Some(Pin10::Disable),
            1 => Some(Pin10::Pu),
            2 => Some(Pin10::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin10::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin10::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin10::Pd
    }
}
#[doc = "Field `PIN10` writer - Select pull mode for pin 10"]
pub type Pin10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin10>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Pd)
    }
}
#[doc = "Select pull mode for pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin11 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin11> for u8 {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin11 {
    type Ux = u8;
}
impl crate::IsEnum for Pin11 {}
#[doc = "Field `PIN11` reader - Select pull mode for pin 11"]
pub type Pin11R = crate::FieldReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin11> {
        match self.bits {
            0 => Some(Pin11::Disable),
            1 => Some(Pin11::Pu),
            2 => Some(Pin11::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin11::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin11::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin11::Pd
    }
}
#[doc = "Field `PIN11` writer - Select pull mode for pin 11"]
pub type Pin11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin11>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Pd)
    }
}
#[doc = "Select pull mode for pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin12 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin12> for u8 {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin12 {
    type Ux = u8;
}
impl crate::IsEnum for Pin12 {}
#[doc = "Field `PIN12` reader - Select pull mode for pin 12"]
pub type Pin12R = crate::FieldReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin12> {
        match self.bits {
            0 => Some(Pin12::Disable),
            1 => Some(Pin12::Pu),
            2 => Some(Pin12::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin12::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin12::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin12::Pd
    }
}
#[doc = "Field `PIN12` writer - Select pull mode for pin 12"]
pub type Pin12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin12>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Pd)
    }
}
#[doc = "Select pull mode for pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin13 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin13> for u8 {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin13 {
    type Ux = u8;
}
impl crate::IsEnum for Pin13 {}
#[doc = "Field `PIN13` reader - Select pull mode for pin 13"]
pub type Pin13R = crate::FieldReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin13> {
        match self.bits {
            0 => Some(Pin13::Disable),
            1 => Some(Pin13::Pu),
            2 => Some(Pin13::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin13::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin13::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin13::Pd
    }
}
#[doc = "Field `PIN13` writer - Select pull mode for pin 13"]
pub type Pin13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin13>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Pd)
    }
}
#[doc = "Select pull mode for pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin14 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin14> for u8 {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin14 {
    type Ux = u8;
}
impl crate::IsEnum for Pin14 {}
#[doc = "Field `PIN14` reader - Select pull mode for pin 14"]
pub type Pin14R = crate::FieldReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin14> {
        match self.bits {
            0 => Some(Pin14::Disable),
            1 => Some(Pin14::Pu),
            2 => Some(Pin14::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin14::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin14::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin14::Pd
    }
}
#[doc = "Field `PIN14` writer - Select pull mode for pin 14"]
pub type Pin14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin14>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Pd)
    }
}
#[doc = "Select pull mode for pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin15 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
    #[doc = "2: Pull-down"]
    Pd = 2,
}
impl From<Pin15> for u8 {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin15 {
    type Ux = u8;
}
impl crate::IsEnum for Pin15 {}
#[doc = "Field `PIN15` reader - Select pull mode for pin 15"]
pub type Pin15R = crate::FieldReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin15> {
        match self.bits {
            0 => Some(Pin15::Disable),
            1 => Some(Pin15::Pu),
            2 => Some(Pin15::Pd),
            _ => None,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin15::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin15::Pu
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == Pin15::Pd
    }
}
#[doc = "Field `PIN15` writer - Select pull mode for pin 15"]
pub type Pin15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin15>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Pu)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Pd)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select pull mode for pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select pull mode for pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select pull mode for pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Select pull mode for pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Select pull mode for pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Select pull mode for pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Select pull mode for pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Select pull mode for pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Select pull mode for pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Select pull mode for pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Select pull mode for pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Select pull mode for pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Select pull mode for pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Select pull mode for pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Select pull mode for pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Select pull mode for pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select pull mode for pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> Pin0W<PullmodeSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Select pull mode for pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> Pin1W<PullmodeSpec> {
        Pin1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Select pull mode for pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> Pin2W<PullmodeSpec> {
        Pin2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Select pull mode for pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> Pin3W<PullmodeSpec> {
        Pin3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Select pull mode for pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> Pin4W<PullmodeSpec> {
        Pin4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Select pull mode for pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> Pin5W<PullmodeSpec> {
        Pin5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Select pull mode for pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> Pin6W<PullmodeSpec> {
        Pin6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Select pull mode for pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> Pin7W<PullmodeSpec> {
        Pin7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Select pull mode for pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> Pin8W<PullmodeSpec> {
        Pin8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Select pull mode for pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> Pin9W<PullmodeSpec> {
        Pin9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Select pull mode for pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> Pin10W<PullmodeSpec> {
        Pin10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Select pull mode for pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> Pin11W<PullmodeSpec> {
        Pin11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Select pull mode for pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> Pin12W<PullmodeSpec> {
        Pin12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Select pull mode for pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> Pin13W<PullmodeSpec> {
        Pin13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Select pull mode for pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> Pin14W<PullmodeSpec> {
        Pin14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Select pull mode for pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> Pin15W<PullmodeSpec> {
        Pin15W::new(self, 30)
    }
}
#[doc = "Select pull mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pullmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pullmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PullmodeSpec;
impl crate::RegisterSpec for PullmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pullmode::R`](R) reader structure"]
impl crate::Readable for PullmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`pullmode::W`](W) writer structure"]
impl crate::Writable for PullmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PULLMODE to value 0"]
impl crate::Resettable for PullmodeSpec {
    const RESET_VALUE: u32 = 0;
}
