#[doc = "Register `OUTMODE` reader"]
pub type R = crate::R<OutmodeSpec>;
#[doc = "Register `OUTMODE` writer"]
pub type W = crate::W<OutmodeSpec>;
#[doc = "Select output mode for pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin0 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN0` reader - Select output mode for pin 0"]
pub type Pin0R = crate::FieldReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin0> {
        match self.bits {
            0 => Some(Pin0::Pp),
            1 => Some(Pin0::Od),
            2 => Some(Pin0::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin0::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin0::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin0::Os
    }
}
#[doc = "Field `PIN0` writer - Select output mode for pin 0"]
pub type Pin0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin0>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Os)
    }
}
#[doc = "Select output mode for pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin1 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN1` reader - Select output mode for pin 1"]
pub type Pin1R = crate::FieldReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin1> {
        match self.bits {
            0 => Some(Pin1::Pp),
            1 => Some(Pin1::Od),
            2 => Some(Pin1::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin1::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin1::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin1::Os
    }
}
#[doc = "Field `PIN1` writer - Select output mode for pin 1"]
pub type Pin1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin1>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Os)
    }
}
#[doc = "Select output mode for pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin2 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN2` reader - Select output mode for pin 2"]
pub type Pin2R = crate::FieldReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin2> {
        match self.bits {
            0 => Some(Pin2::Pp),
            1 => Some(Pin2::Od),
            2 => Some(Pin2::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin2::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin2::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin2::Os
    }
}
#[doc = "Field `PIN2` writer - Select output mode for pin 2"]
pub type Pin2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin2>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Os)
    }
}
#[doc = "Select output mode for pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin3 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN3` reader - Select output mode for pin 3"]
pub type Pin3R = crate::FieldReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin3> {
        match self.bits {
            0 => Some(Pin3::Pp),
            1 => Some(Pin3::Od),
            2 => Some(Pin3::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin3::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin3::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin3::Os
    }
}
#[doc = "Field `PIN3` writer - Select output mode for pin 3"]
pub type Pin3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin3>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Os)
    }
}
#[doc = "Select output mode for pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin4 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN4` reader - Select output mode for pin 4"]
pub type Pin4R = crate::FieldReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin4> {
        match self.bits {
            0 => Some(Pin4::Pp),
            1 => Some(Pin4::Od),
            2 => Some(Pin4::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin4::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin4::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin4::Os
    }
}
#[doc = "Field `PIN4` writer - Select output mode for pin 4"]
pub type Pin4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin4>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Os)
    }
}
#[doc = "Select output mode for pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin5 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN5` reader - Select output mode for pin 5"]
pub type Pin5R = crate::FieldReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin5> {
        match self.bits {
            0 => Some(Pin5::Pp),
            1 => Some(Pin5::Od),
            2 => Some(Pin5::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin5::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin5::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin5::Os
    }
}
#[doc = "Field `PIN5` writer - Select output mode for pin 5"]
pub type Pin5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin5>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Os)
    }
}
#[doc = "Select output mode for pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin6 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN6` reader - Select output mode for pin 6"]
pub type Pin6R = crate::FieldReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin6> {
        match self.bits {
            0 => Some(Pin6::Pp),
            1 => Some(Pin6::Od),
            2 => Some(Pin6::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin6::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin6::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin6::Os
    }
}
#[doc = "Field `PIN6` writer - Select output mode for pin 6"]
pub type Pin6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin6>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Os)
    }
}
#[doc = "Select output mode for pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin7 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN7` reader - Select output mode for pin 7"]
pub type Pin7R = crate::FieldReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin7> {
        match self.bits {
            0 => Some(Pin7::Pp),
            1 => Some(Pin7::Od),
            2 => Some(Pin7::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin7::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin7::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin7::Os
    }
}
#[doc = "Field `PIN7` writer - Select output mode for pin 7"]
pub type Pin7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin7>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Os)
    }
}
#[doc = "Select output mode for pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin8 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN8` reader - Select output mode for pin 8"]
pub type Pin8R = crate::FieldReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin8> {
        match self.bits {
            0 => Some(Pin8::Pp),
            1 => Some(Pin8::Od),
            2 => Some(Pin8::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin8::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin8::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin8::Os
    }
}
#[doc = "Field `PIN8` writer - Select output mode for pin 8"]
pub type Pin8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin8>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Os)
    }
}
#[doc = "Select output mode for pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin9 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN9` reader - Select output mode for pin 9"]
pub type Pin9R = crate::FieldReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin9> {
        match self.bits {
            0 => Some(Pin9::Pp),
            1 => Some(Pin9::Od),
            2 => Some(Pin9::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin9::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin9::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin9::Os
    }
}
#[doc = "Field `PIN9` writer - Select output mode for pin 9"]
pub type Pin9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin9>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Os)
    }
}
#[doc = "Select output mode for pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin10 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN10` reader - Select output mode for pin 10"]
pub type Pin10R = crate::FieldReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin10> {
        match self.bits {
            0 => Some(Pin10::Pp),
            1 => Some(Pin10::Od),
            2 => Some(Pin10::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin10::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin10::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin10::Os
    }
}
#[doc = "Field `PIN10` writer - Select output mode for pin 10"]
pub type Pin10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin10>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Os)
    }
}
#[doc = "Select output mode for pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin11 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN11` reader - Select output mode for pin 11"]
pub type Pin11R = crate::FieldReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin11> {
        match self.bits {
            0 => Some(Pin11::Pp),
            1 => Some(Pin11::Od),
            2 => Some(Pin11::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin11::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin11::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin11::Os
    }
}
#[doc = "Field `PIN11` writer - Select output mode for pin 11"]
pub type Pin11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin11>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Os)
    }
}
#[doc = "Select output mode for pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin12 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN12` reader - Select output mode for pin 12"]
pub type Pin12R = crate::FieldReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin12> {
        match self.bits {
            0 => Some(Pin12::Pp),
            1 => Some(Pin12::Od),
            2 => Some(Pin12::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin12::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin12::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin12::Os
    }
}
#[doc = "Field `PIN12` writer - Select output mode for pin 12"]
pub type Pin12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin12>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Os)
    }
}
#[doc = "Select output mode for pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin13 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN13` reader - Select output mode for pin 13"]
pub type Pin13R = crate::FieldReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin13> {
        match self.bits {
            0 => Some(Pin13::Pp),
            1 => Some(Pin13::Od),
            2 => Some(Pin13::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin13::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin13::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin13::Os
    }
}
#[doc = "Field `PIN13` writer - Select output mode for pin 13"]
pub type Pin13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin13>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Os)
    }
}
#[doc = "Select output mode for pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin14 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN14` reader - Select output mode for pin 14"]
pub type Pin14R = crate::FieldReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin14> {
        match self.bits {
            0 => Some(Pin14::Pp),
            1 => Some(Pin14::Od),
            2 => Some(Pin14::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin14::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin14::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin14::Os
    }
}
#[doc = "Field `PIN14` writer - Select output mode for pin 14"]
pub type Pin14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin14>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Os)
    }
}
#[doc = "Select output mode for pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin15 {
    #[doc = "0: Push-pull output"]
    Pp = 0,
    #[doc = "1: Open drain output"]
    Od = 1,
    #[doc = "2: Open source output"]
    Os = 2,
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
#[doc = "Field `PIN15` reader - Select output mode for pin 15"]
pub type Pin15R = crate::FieldReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin15> {
        match self.bits {
            0 => Some(Pin15::Pp),
            1 => Some(Pin15::Od),
            2 => Some(Pin15::Os),
            _ => None,
        }
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Pin15::Pp
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == Pin15::Od
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn is_os(&self) -> bool {
        *self == Pin15::Os
    }
}
#[doc = "Field `PIN15` writer - Select output mode for pin 15"]
pub type Pin15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pin15>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Pp)
    }
    #[doc = "Open drain output"]
    #[inline(always)]
    pub fn od(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Od)
    }
    #[doc = "Open source output"]
    #[inline(always)]
    pub fn os(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Os)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select output mode for pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select output mode for pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select output mode for pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Select output mode for pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Select output mode for pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Select output mode for pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Select output mode for pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Select output mode for pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Select output mode for pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Select output mode for pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Select output mode for pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Select output mode for pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Select output mode for pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Select output mode for pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Select output mode for pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Select output mode for pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select output mode for pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> Pin0W<OutmodeSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Select output mode for pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> Pin1W<OutmodeSpec> {
        Pin1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Select output mode for pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> Pin2W<OutmodeSpec> {
        Pin2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Select output mode for pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> Pin3W<OutmodeSpec> {
        Pin3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Select output mode for pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> Pin4W<OutmodeSpec> {
        Pin4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Select output mode for pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> Pin5W<OutmodeSpec> {
        Pin5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Select output mode for pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> Pin6W<OutmodeSpec> {
        Pin6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Select output mode for pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> Pin7W<OutmodeSpec> {
        Pin7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Select output mode for pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> Pin8W<OutmodeSpec> {
        Pin8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Select output mode for pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> Pin9W<OutmodeSpec> {
        Pin9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Select output mode for pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> Pin10W<OutmodeSpec> {
        Pin10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Select output mode for pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> Pin11W<OutmodeSpec> {
        Pin11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Select output mode for pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> Pin12W<OutmodeSpec> {
        Pin12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Select output mode for pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> Pin13W<OutmodeSpec> {
        Pin13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Select output mode for pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> Pin14W<OutmodeSpec> {
        Pin14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Select output mode for pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> Pin15W<OutmodeSpec> {
        Pin15W::new(self, 30)
    }
}
#[doc = "Select output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutmodeSpec;
impl crate::RegisterSpec for OutmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outmode::R`](R) reader structure"]
impl crate::Readable for OutmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`outmode::W`](W) writer structure"]
impl crate::Writable for OutmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTMODE to value 0"]
impl crate::Resettable for OutmodeSpec {
    const RESET_VALUE: u32 = 0;
}
