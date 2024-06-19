#[doc = "Register `WUIR` reader"]
pub type R = crate::R<WuirSpec>;
#[doc = "Register `WUIR` writer"]
pub type W = crate::W<WuirSpec>;
#[doc = "Wake Up Input Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen0 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen0> for bool {
    #[inline(always)]
    fn from(variant: Wkupen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN0` reader - Wake Up Input Enable 0"]
pub type Wkupen0R = crate::BitReader<Wkupen0>;
impl Wkupen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen0 {
        match self.bits {
            false => Wkupen0::Disable,
            true => Wkupen0::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen0::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen0::Enable
    }
}
#[doc = "Field `WKUPEN0` writer - Wake Up Input Enable 0"]
pub type Wkupen0W<'a, REG> = crate::BitWriter<'a, REG, Wkupen0>;
impl<'a, REG> Wkupen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen0::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen0::Enable)
    }
}
#[doc = "Wake Up Input Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen1 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen1> for bool {
    #[inline(always)]
    fn from(variant: Wkupen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN1` reader - Wake Up Input Enable 1"]
pub type Wkupen1R = crate::BitReader<Wkupen1>;
impl Wkupen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen1 {
        match self.bits {
            false => Wkupen1::Disable,
            true => Wkupen1::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen1::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen1::Enable
    }
}
#[doc = "Field `WKUPEN1` writer - Wake Up Input Enable 1"]
pub type Wkupen1W<'a, REG> = crate::BitWriter<'a, REG, Wkupen1>;
impl<'a, REG> Wkupen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen1::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen1::Enable)
    }
}
#[doc = "Wake Up Input Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen2 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen2> for bool {
    #[inline(always)]
    fn from(variant: Wkupen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN2` reader - Wake Up Input Enable 2"]
pub type Wkupen2R = crate::BitReader<Wkupen2>;
impl Wkupen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen2 {
        match self.bits {
            false => Wkupen2::Disable,
            true => Wkupen2::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen2::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen2::Enable
    }
}
#[doc = "Field `WKUPEN2` writer - Wake Up Input Enable 2"]
pub type Wkupen2W<'a, REG> = crate::BitWriter<'a, REG, Wkupen2>;
impl<'a, REG> Wkupen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen2::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen2::Enable)
    }
}
#[doc = "Wake Up Input Enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen3 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen3> for bool {
    #[inline(always)]
    fn from(variant: Wkupen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN3` reader - Wake Up Input Enable 3"]
pub type Wkupen3R = crate::BitReader<Wkupen3>;
impl Wkupen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen3 {
        match self.bits {
            false => Wkupen3::Disable,
            true => Wkupen3::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen3::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen3::Enable
    }
}
#[doc = "Field `WKUPEN3` writer - Wake Up Input Enable 3"]
pub type Wkupen3W<'a, REG> = crate::BitWriter<'a, REG, Wkupen3>;
impl<'a, REG> Wkupen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen3::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen3::Enable)
    }
}
#[doc = "Wake Up Input Enable 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen4 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen4> for bool {
    #[inline(always)]
    fn from(variant: Wkupen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN4` reader - Wake Up Input Enable 4"]
pub type Wkupen4R = crate::BitReader<Wkupen4>;
impl Wkupen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen4 {
        match self.bits {
            false => Wkupen4::Disable,
            true => Wkupen4::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen4::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen4::Enable
    }
}
#[doc = "Field `WKUPEN4` writer - Wake Up Input Enable 4"]
pub type Wkupen4W<'a, REG> = crate::BitWriter<'a, REG, Wkupen4>;
impl<'a, REG> Wkupen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen4::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen4::Enable)
    }
}
#[doc = "Wake Up Input Enable 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen5 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen5> for bool {
    #[inline(always)]
    fn from(variant: Wkupen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN5` reader - Wake Up Input Enable 5"]
pub type Wkupen5R = crate::BitReader<Wkupen5>;
impl Wkupen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen5 {
        match self.bits {
            false => Wkupen5::Disable,
            true => Wkupen5::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen5::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen5::Enable
    }
}
#[doc = "Field `WKUPEN5` writer - Wake Up Input Enable 5"]
pub type Wkupen5W<'a, REG> = crate::BitWriter<'a, REG, Wkupen5>;
impl<'a, REG> Wkupen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen5::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen5::Enable)
    }
}
#[doc = "Wake Up Input Enable 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen6 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen6> for bool {
    #[inline(always)]
    fn from(variant: Wkupen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN6` reader - Wake Up Input Enable 6"]
pub type Wkupen6R = crate::BitReader<Wkupen6>;
impl Wkupen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen6 {
        match self.bits {
            false => Wkupen6::Disable,
            true => Wkupen6::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen6::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen6::Enable
    }
}
#[doc = "Field `WKUPEN6` writer - Wake Up Input Enable 6"]
pub type Wkupen6W<'a, REG> = crate::BitWriter<'a, REG, Wkupen6>;
impl<'a, REG> Wkupen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen6::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen6::Enable)
    }
}
#[doc = "Wake Up Input Enable 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen7 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen7> for bool {
    #[inline(always)]
    fn from(variant: Wkupen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN7` reader - Wake Up Input Enable 7"]
pub type Wkupen7R = crate::BitReader<Wkupen7>;
impl Wkupen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen7 {
        match self.bits {
            false => Wkupen7::Disable,
            true => Wkupen7::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen7::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen7::Enable
    }
}
#[doc = "Field `WKUPEN7` writer - Wake Up Input Enable 7"]
pub type Wkupen7W<'a, REG> = crate::BitWriter<'a, REG, Wkupen7>;
impl<'a, REG> Wkupen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen7::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen7::Enable)
    }
}
#[doc = "Wake Up Input Enable 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen8 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen8> for bool {
    #[inline(always)]
    fn from(variant: Wkupen8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN8` reader - Wake Up Input Enable 8"]
pub type Wkupen8R = crate::BitReader<Wkupen8>;
impl Wkupen8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen8 {
        match self.bits {
            false => Wkupen8::Disable,
            true => Wkupen8::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen8::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen8::Enable
    }
}
#[doc = "Field `WKUPEN8` writer - Wake Up Input Enable 8"]
pub type Wkupen8W<'a, REG> = crate::BitWriter<'a, REG, Wkupen8>;
impl<'a, REG> Wkupen8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen8::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen8::Enable)
    }
}
#[doc = "Wake Up Input Enable 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen9 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen9> for bool {
    #[inline(always)]
    fn from(variant: Wkupen9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN9` reader - Wake Up Input Enable 9"]
pub type Wkupen9R = crate::BitReader<Wkupen9>;
impl Wkupen9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen9 {
        match self.bits {
            false => Wkupen9::Disable,
            true => Wkupen9::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen9::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen9::Enable
    }
}
#[doc = "Field `WKUPEN9` writer - Wake Up Input Enable 9"]
pub type Wkupen9W<'a, REG> = crate::BitWriter<'a, REG, Wkupen9>;
impl<'a, REG> Wkupen9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen9::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen9::Enable)
    }
}
#[doc = "Wake Up Input Enable 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen10 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen10> for bool {
    #[inline(always)]
    fn from(variant: Wkupen10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN10` reader - Wake Up Input Enable 10"]
pub type Wkupen10R = crate::BitReader<Wkupen10>;
impl Wkupen10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen10 {
        match self.bits {
            false => Wkupen10::Disable,
            true => Wkupen10::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen10::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen10::Enable
    }
}
#[doc = "Field `WKUPEN10` writer - Wake Up Input Enable 10"]
pub type Wkupen10W<'a, REG> = crate::BitWriter<'a, REG, Wkupen10>;
impl<'a, REG> Wkupen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen10::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen10::Enable)
    }
}
#[doc = "Wake Up Input Enable 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen11 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen11> for bool {
    #[inline(always)]
    fn from(variant: Wkupen11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN11` reader - Wake Up Input Enable 11"]
pub type Wkupen11R = crate::BitReader<Wkupen11>;
impl Wkupen11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen11 {
        match self.bits {
            false => Wkupen11::Disable,
            true => Wkupen11::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen11::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen11::Enable
    }
}
#[doc = "Field `WKUPEN11` writer - Wake Up Input Enable 11"]
pub type Wkupen11W<'a, REG> = crate::BitWriter<'a, REG, Wkupen11>;
impl<'a, REG> Wkupen11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen11::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen11::Enable)
    }
}
#[doc = "Wake Up Input Enable 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen12 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen12> for bool {
    #[inline(always)]
    fn from(variant: Wkupen12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN12` reader - Wake Up Input Enable 12"]
pub type Wkupen12R = crate::BitReader<Wkupen12>;
impl Wkupen12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen12 {
        match self.bits {
            false => Wkupen12::Disable,
            true => Wkupen12::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen12::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen12::Enable
    }
}
#[doc = "Field `WKUPEN12` writer - Wake Up Input Enable 12"]
pub type Wkupen12W<'a, REG> = crate::BitWriter<'a, REG, Wkupen12>;
impl<'a, REG> Wkupen12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen12::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen12::Enable)
    }
}
#[doc = "Wake Up Input Enable 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen13 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen13> for bool {
    #[inline(always)]
    fn from(variant: Wkupen13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN13` reader - Wake Up Input Enable 13"]
pub type Wkupen13R = crate::BitReader<Wkupen13>;
impl Wkupen13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen13 {
        match self.bits {
            false => Wkupen13::Disable,
            true => Wkupen13::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen13::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen13::Enable
    }
}
#[doc = "Field `WKUPEN13` writer - Wake Up Input Enable 13"]
pub type Wkupen13W<'a, REG> = crate::BitWriter<'a, REG, Wkupen13>;
impl<'a, REG> Wkupen13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen13::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen13::Enable)
    }
}
#[doc = "Wake Up Input Enable 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen14 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen14> for bool {
    #[inline(always)]
    fn from(variant: Wkupen14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN14` reader - Wake Up Input Enable 14"]
pub type Wkupen14R = crate::BitReader<Wkupen14>;
impl Wkupen14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen14 {
        match self.bits {
            false => Wkupen14::Disable,
            true => Wkupen14::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen14::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen14::Enable
    }
}
#[doc = "Field `WKUPEN14` writer - Wake Up Input Enable 14"]
pub type Wkupen14W<'a, REG> = crate::BitWriter<'a, REG, Wkupen14>;
impl<'a, REG> Wkupen14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen14::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen14::Enable)
    }
}
#[doc = "Wake Up Input Enable 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupen15 {
    #[doc = "0: the corresponding wake-up input has no wake up effect."]
    Disable = 0,
    #[doc = "1: the corresponding wake-up input forces the wake up of the core power supply."]
    Enable = 1,
}
impl From<Wkupen15> for bool {
    #[inline(always)]
    fn from(variant: Wkupen15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN15` reader - Wake Up Input Enable 15"]
pub type Wkupen15R = crate::BitReader<Wkupen15>;
impl Wkupen15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupen15 {
        match self.bits {
            false => Wkupen15::Disable,
            true => Wkupen15::Enable,
        }
    }
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkupen15::Disable
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkupen15::Enable
    }
}
#[doc = "Field `WKUPEN15` writer - Wake Up Input Enable 15"]
pub type Wkupen15W<'a, REG> = crate::BitWriter<'a, REG, Wkupen15>;
impl<'a, REG> Wkupen15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the corresponding wake-up input has no wake up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen15::Disable)
    }
    #[doc = "the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupen15::Enable)
    }
}
#[doc = "Wake Up Input Type 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt0 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt0> for bool {
    #[inline(always)]
    fn from(variant: Wkupt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT0` reader - Wake Up Input Type 0"]
pub type Wkupt0R = crate::BitReader<Wkupt0>;
impl Wkupt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt0 {
        match self.bits {
            false => Wkupt0::Low,
            true => Wkupt0::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt0::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt0::High
    }
}
#[doc = "Field `WKUPT0` writer - Wake Up Input Type 0"]
pub type Wkupt0W<'a, REG> = crate::BitWriter<'a, REG, Wkupt0>;
impl<'a, REG> Wkupt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt0::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt0::High)
    }
}
#[doc = "Wake Up Input Type 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt1 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt1> for bool {
    #[inline(always)]
    fn from(variant: Wkupt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT1` reader - Wake Up Input Type 1"]
pub type Wkupt1R = crate::BitReader<Wkupt1>;
impl Wkupt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt1 {
        match self.bits {
            false => Wkupt1::Low,
            true => Wkupt1::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt1::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt1::High
    }
}
#[doc = "Field `WKUPT1` writer - Wake Up Input Type 1"]
pub type Wkupt1W<'a, REG> = crate::BitWriter<'a, REG, Wkupt1>;
impl<'a, REG> Wkupt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt1::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt1::High)
    }
}
#[doc = "Wake Up Input Type 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt2 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt2> for bool {
    #[inline(always)]
    fn from(variant: Wkupt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT2` reader - Wake Up Input Type 2"]
pub type Wkupt2R = crate::BitReader<Wkupt2>;
impl Wkupt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt2 {
        match self.bits {
            false => Wkupt2::Low,
            true => Wkupt2::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt2::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt2::High
    }
}
#[doc = "Field `WKUPT2` writer - Wake Up Input Type 2"]
pub type Wkupt2W<'a, REG> = crate::BitWriter<'a, REG, Wkupt2>;
impl<'a, REG> Wkupt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt2::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt2::High)
    }
}
#[doc = "Wake Up Input Type 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt3 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt3> for bool {
    #[inline(always)]
    fn from(variant: Wkupt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT3` reader - Wake Up Input Type 3"]
pub type Wkupt3R = crate::BitReader<Wkupt3>;
impl Wkupt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt3 {
        match self.bits {
            false => Wkupt3::Low,
            true => Wkupt3::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt3::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt3::High
    }
}
#[doc = "Field `WKUPT3` writer - Wake Up Input Type 3"]
pub type Wkupt3W<'a, REG> = crate::BitWriter<'a, REG, Wkupt3>;
impl<'a, REG> Wkupt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt3::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt3::High)
    }
}
#[doc = "Wake Up Input Type 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt4 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt4> for bool {
    #[inline(always)]
    fn from(variant: Wkupt4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT4` reader - Wake Up Input Type 4"]
pub type Wkupt4R = crate::BitReader<Wkupt4>;
impl Wkupt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt4 {
        match self.bits {
            false => Wkupt4::Low,
            true => Wkupt4::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt4::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt4::High
    }
}
#[doc = "Field `WKUPT4` writer - Wake Up Input Type 4"]
pub type Wkupt4W<'a, REG> = crate::BitWriter<'a, REG, Wkupt4>;
impl<'a, REG> Wkupt4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt4::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt4::High)
    }
}
#[doc = "Wake Up Input Type 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt5 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt5> for bool {
    #[inline(always)]
    fn from(variant: Wkupt5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT5` reader - Wake Up Input Type 5"]
pub type Wkupt5R = crate::BitReader<Wkupt5>;
impl Wkupt5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt5 {
        match self.bits {
            false => Wkupt5::Low,
            true => Wkupt5::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt5::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt5::High
    }
}
#[doc = "Field `WKUPT5` writer - Wake Up Input Type 5"]
pub type Wkupt5W<'a, REG> = crate::BitWriter<'a, REG, Wkupt5>;
impl<'a, REG> Wkupt5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt5::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt5::High)
    }
}
#[doc = "Wake Up Input Type 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt6 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt6> for bool {
    #[inline(always)]
    fn from(variant: Wkupt6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT6` reader - Wake Up Input Type 6"]
pub type Wkupt6R = crate::BitReader<Wkupt6>;
impl Wkupt6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt6 {
        match self.bits {
            false => Wkupt6::Low,
            true => Wkupt6::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt6::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt6::High
    }
}
#[doc = "Field `WKUPT6` writer - Wake Up Input Type 6"]
pub type Wkupt6W<'a, REG> = crate::BitWriter<'a, REG, Wkupt6>;
impl<'a, REG> Wkupt6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt6::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt6::High)
    }
}
#[doc = "Wake Up Input Type 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt7 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt7> for bool {
    #[inline(always)]
    fn from(variant: Wkupt7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT7` reader - Wake Up Input Type 7"]
pub type Wkupt7R = crate::BitReader<Wkupt7>;
impl Wkupt7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt7 {
        match self.bits {
            false => Wkupt7::Low,
            true => Wkupt7::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt7::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt7::High
    }
}
#[doc = "Field `WKUPT7` writer - Wake Up Input Type 7"]
pub type Wkupt7W<'a, REG> = crate::BitWriter<'a, REG, Wkupt7>;
impl<'a, REG> Wkupt7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt7::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt7::High)
    }
}
#[doc = "Wake Up Input Type 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt8 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt8> for bool {
    #[inline(always)]
    fn from(variant: Wkupt8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT8` reader - Wake Up Input Type 8"]
pub type Wkupt8R = crate::BitReader<Wkupt8>;
impl Wkupt8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt8 {
        match self.bits {
            false => Wkupt8::Low,
            true => Wkupt8::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt8::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt8::High
    }
}
#[doc = "Field `WKUPT8` writer - Wake Up Input Type 8"]
pub type Wkupt8W<'a, REG> = crate::BitWriter<'a, REG, Wkupt8>;
impl<'a, REG> Wkupt8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt8::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt8::High)
    }
}
#[doc = "Wake Up Input Type 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt9 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt9> for bool {
    #[inline(always)]
    fn from(variant: Wkupt9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT9` reader - Wake Up Input Type 9"]
pub type Wkupt9R = crate::BitReader<Wkupt9>;
impl Wkupt9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt9 {
        match self.bits {
            false => Wkupt9::Low,
            true => Wkupt9::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt9::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt9::High
    }
}
#[doc = "Field `WKUPT9` writer - Wake Up Input Type 9"]
pub type Wkupt9W<'a, REG> = crate::BitWriter<'a, REG, Wkupt9>;
impl<'a, REG> Wkupt9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt9::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt9::High)
    }
}
#[doc = "Wake Up Input Type 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt10 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt10> for bool {
    #[inline(always)]
    fn from(variant: Wkupt10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT10` reader - Wake Up Input Type 10"]
pub type Wkupt10R = crate::BitReader<Wkupt10>;
impl Wkupt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt10 {
        match self.bits {
            false => Wkupt10::Low,
            true => Wkupt10::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt10::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt10::High
    }
}
#[doc = "Field `WKUPT10` writer - Wake Up Input Type 10"]
pub type Wkupt10W<'a, REG> = crate::BitWriter<'a, REG, Wkupt10>;
impl<'a, REG> Wkupt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt10::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt10::High)
    }
}
#[doc = "Wake Up Input Type 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt11 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt11> for bool {
    #[inline(always)]
    fn from(variant: Wkupt11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT11` reader - Wake Up Input Type 11"]
pub type Wkupt11R = crate::BitReader<Wkupt11>;
impl Wkupt11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt11 {
        match self.bits {
            false => Wkupt11::Low,
            true => Wkupt11::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt11::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt11::High
    }
}
#[doc = "Field `WKUPT11` writer - Wake Up Input Type 11"]
pub type Wkupt11W<'a, REG> = crate::BitWriter<'a, REG, Wkupt11>;
impl<'a, REG> Wkupt11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt11::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt11::High)
    }
}
#[doc = "Wake Up Input Type 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt12 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt12> for bool {
    #[inline(always)]
    fn from(variant: Wkupt12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT12` reader - Wake Up Input Type 12"]
pub type Wkupt12R = crate::BitReader<Wkupt12>;
impl Wkupt12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt12 {
        match self.bits {
            false => Wkupt12::Low,
            true => Wkupt12::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt12::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt12::High
    }
}
#[doc = "Field `WKUPT12` writer - Wake Up Input Type 12"]
pub type Wkupt12W<'a, REG> = crate::BitWriter<'a, REG, Wkupt12>;
impl<'a, REG> Wkupt12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt12::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt12::High)
    }
}
#[doc = "Wake Up Input Type 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt13 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt13> for bool {
    #[inline(always)]
    fn from(variant: Wkupt13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT13` reader - Wake Up Input Type 13"]
pub type Wkupt13R = crate::BitReader<Wkupt13>;
impl Wkupt13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt13 {
        match self.bits {
            false => Wkupt13::Low,
            true => Wkupt13::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt13::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt13::High
    }
}
#[doc = "Field `WKUPT13` writer - Wake Up Input Type 13"]
pub type Wkupt13W<'a, REG> = crate::BitWriter<'a, REG, Wkupt13>;
impl<'a, REG> Wkupt13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt13::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt13::High)
    }
}
#[doc = "Wake Up Input Type 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt14 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt14> for bool {
    #[inline(always)]
    fn from(variant: Wkupt14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT14` reader - Wake Up Input Type 14"]
pub type Wkupt14R = crate::BitReader<Wkupt14>;
impl Wkupt14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt14 {
        match self.bits {
            false => Wkupt14::Low,
            true => Wkupt14::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt14::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt14::High
    }
}
#[doc = "Field `WKUPT14` writer - Wake Up Input Type 14"]
pub type Wkupt14W<'a, REG> = crate::BitWriter<'a, REG, Wkupt14>;
impl<'a, REG> Wkupt14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt14::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt14::High)
    }
}
#[doc = "Wake Up Input Type 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupt15 {
    #[doc = "0: a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    Low = 0,
    #[doc = "1: a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    High = 1,
}
impl From<Wkupt15> for bool {
    #[inline(always)]
    fn from(variant: Wkupt15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT15` reader - Wake Up Input Type 15"]
pub type Wkupt15R = crate::BitReader<Wkupt15>;
impl Wkupt15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupt15 {
        match self.bits {
            false => Wkupt15::Low,
            true => Wkupt15::High,
        }
    }
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Wkupt15::Low
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Wkupt15::High
    }
}
#[doc = "Field `WKUPT15` writer - Wake Up Input Type 15"]
pub type Wkupt15W<'a, REG> = crate::BitWriter<'a, REG, Wkupt15>;
impl<'a, REG> Wkupt15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a low level for a period defined by WKUPDBC on the corresponding wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt15::Low)
    }
    #[doc = "a high levelfor a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupt15::High)
    }
}
impl R {
    #[doc = "Bit 0 - Wake Up Input Enable 0"]
    #[inline(always)]
    pub fn wkupen0(&self) -> Wkupen0R {
        Wkupen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake Up Input Enable 1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> Wkupen1R {
        Wkupen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake Up Input Enable 2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> Wkupen2R {
        Wkupen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake Up Input Enable 3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> Wkupen3R {
        Wkupen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake Up Input Enable 4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> Wkupen4R {
        Wkupen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake Up Input Enable 5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> Wkupen5R {
        Wkupen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake Up Input Enable 6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> Wkupen6R {
        Wkupen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake Up Input Enable 7"]
    #[inline(always)]
    pub fn wkupen7(&self) -> Wkupen7R {
        Wkupen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake Up Input Enable 8"]
    #[inline(always)]
    pub fn wkupen8(&self) -> Wkupen8R {
        Wkupen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake Up Input Enable 9"]
    #[inline(always)]
    pub fn wkupen9(&self) -> Wkupen9R {
        Wkupen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake Up Input Enable 10"]
    #[inline(always)]
    pub fn wkupen10(&self) -> Wkupen10R {
        Wkupen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake Up Input Enable 11"]
    #[inline(always)]
    pub fn wkupen11(&self) -> Wkupen11R {
        Wkupen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake Up Input Enable 12"]
    #[inline(always)]
    pub fn wkupen12(&self) -> Wkupen12R {
        Wkupen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake Up Input Enable 13"]
    #[inline(always)]
    pub fn wkupen13(&self) -> Wkupen13R {
        Wkupen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wake Up Input Enable 14"]
    #[inline(always)]
    pub fn wkupen14(&self) -> Wkupen14R {
        Wkupen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wake Up Input Enable 15"]
    #[inline(always)]
    pub fn wkupen15(&self) -> Wkupen15R {
        Wkupen15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wake Up Input Type 0"]
    #[inline(always)]
    pub fn wkupt0(&self) -> Wkupt0R {
        Wkupt0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake Up Input Type 1"]
    #[inline(always)]
    pub fn wkupt1(&self) -> Wkupt1R {
        Wkupt1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake Up Input Type 2"]
    #[inline(always)]
    pub fn wkupt2(&self) -> Wkupt2R {
        Wkupt2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake Up Input Type 3"]
    #[inline(always)]
    pub fn wkupt3(&self) -> Wkupt3R {
        Wkupt3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake Up Input Type 4"]
    #[inline(always)]
    pub fn wkupt4(&self) -> Wkupt4R {
        Wkupt4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake Up Input Type 5"]
    #[inline(always)]
    pub fn wkupt5(&self) -> Wkupt5R {
        Wkupt5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake Up Input Type 6"]
    #[inline(always)]
    pub fn wkupt6(&self) -> Wkupt6R {
        Wkupt6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wake Up Input Type 7"]
    #[inline(always)]
    pub fn wkupt7(&self) -> Wkupt7R {
        Wkupt7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake Up Input Type 8"]
    #[inline(always)]
    pub fn wkupt8(&self) -> Wkupt8R {
        Wkupt8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake Up Input Type 9"]
    #[inline(always)]
    pub fn wkupt9(&self) -> Wkupt9R {
        Wkupt9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake Up Input Type 10"]
    #[inline(always)]
    pub fn wkupt10(&self) -> Wkupt10R {
        Wkupt10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake Up Input Type 11"]
    #[inline(always)]
    pub fn wkupt11(&self) -> Wkupt11R {
        Wkupt11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake Up Input Type 12"]
    #[inline(always)]
    pub fn wkupt12(&self) -> Wkupt12R {
        Wkupt12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake Up Input Type 13"]
    #[inline(always)]
    pub fn wkupt13(&self) -> Wkupt13R {
        Wkupt13R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wake Up Input Type 14"]
    #[inline(always)]
    pub fn wkupt14(&self) -> Wkupt14R {
        Wkupt14R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wake Up Input Type 15"]
    #[inline(always)]
    pub fn wkupt15(&self) -> Wkupt15R {
        Wkupt15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake Up Input Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen0(&mut self) -> Wkupen0W<WuirSpec> {
        Wkupen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Wake Up Input Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen1(&mut self) -> Wkupen1W<WuirSpec> {
        Wkupen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Wake Up Input Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen2(&mut self) -> Wkupen2W<WuirSpec> {
        Wkupen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Wake Up Input Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen3(&mut self) -> Wkupen3W<WuirSpec> {
        Wkupen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Wake Up Input Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen4(&mut self) -> Wkupen4W<WuirSpec> {
        Wkupen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Wake Up Input Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen5(&mut self) -> Wkupen5W<WuirSpec> {
        Wkupen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Wake Up Input Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen6(&mut self) -> Wkupen6W<WuirSpec> {
        Wkupen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Wake Up Input Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen7(&mut self) -> Wkupen7W<WuirSpec> {
        Wkupen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Wake Up Input Enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen8(&mut self) -> Wkupen8W<WuirSpec> {
        Wkupen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Wake Up Input Enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen9(&mut self) -> Wkupen9W<WuirSpec> {
        Wkupen9W::new(self, 9)
    }
    #[doc = "Bit 10 - Wake Up Input Enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen10(&mut self) -> Wkupen10W<WuirSpec> {
        Wkupen10W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake Up Input Enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen11(&mut self) -> Wkupen11W<WuirSpec> {
        Wkupen11W::new(self, 11)
    }
    #[doc = "Bit 12 - Wake Up Input Enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen12(&mut self) -> Wkupen12W<WuirSpec> {
        Wkupen12W::new(self, 12)
    }
    #[doc = "Bit 13 - Wake Up Input Enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen13(&mut self) -> Wkupen13W<WuirSpec> {
        Wkupen13W::new(self, 13)
    }
    #[doc = "Bit 14 - Wake Up Input Enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen14(&mut self) -> Wkupen14W<WuirSpec> {
        Wkupen14W::new(self, 14)
    }
    #[doc = "Bit 15 - Wake Up Input Enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen15(&mut self) -> Wkupen15W<WuirSpec> {
        Wkupen15W::new(self, 15)
    }
    #[doc = "Bit 16 - Wake Up Input Type 0"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt0(&mut self) -> Wkupt0W<WuirSpec> {
        Wkupt0W::new(self, 16)
    }
    #[doc = "Bit 17 - Wake Up Input Type 1"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt1(&mut self) -> Wkupt1W<WuirSpec> {
        Wkupt1W::new(self, 17)
    }
    #[doc = "Bit 18 - Wake Up Input Type 2"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt2(&mut self) -> Wkupt2W<WuirSpec> {
        Wkupt2W::new(self, 18)
    }
    #[doc = "Bit 19 - Wake Up Input Type 3"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt3(&mut self) -> Wkupt3W<WuirSpec> {
        Wkupt3W::new(self, 19)
    }
    #[doc = "Bit 20 - Wake Up Input Type 4"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt4(&mut self) -> Wkupt4W<WuirSpec> {
        Wkupt4W::new(self, 20)
    }
    #[doc = "Bit 21 - Wake Up Input Type 5"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt5(&mut self) -> Wkupt5W<WuirSpec> {
        Wkupt5W::new(self, 21)
    }
    #[doc = "Bit 22 - Wake Up Input Type 6"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt6(&mut self) -> Wkupt6W<WuirSpec> {
        Wkupt6W::new(self, 22)
    }
    #[doc = "Bit 23 - Wake Up Input Type 7"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt7(&mut self) -> Wkupt7W<WuirSpec> {
        Wkupt7W::new(self, 23)
    }
    #[doc = "Bit 24 - Wake Up Input Type 8"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt8(&mut self) -> Wkupt8W<WuirSpec> {
        Wkupt8W::new(self, 24)
    }
    #[doc = "Bit 25 - Wake Up Input Type 9"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt9(&mut self) -> Wkupt9W<WuirSpec> {
        Wkupt9W::new(self, 25)
    }
    #[doc = "Bit 26 - Wake Up Input Type 10"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt10(&mut self) -> Wkupt10W<WuirSpec> {
        Wkupt10W::new(self, 26)
    }
    #[doc = "Bit 27 - Wake Up Input Type 11"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt11(&mut self) -> Wkupt11W<WuirSpec> {
        Wkupt11W::new(self, 27)
    }
    #[doc = "Bit 28 - Wake Up Input Type 12"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt12(&mut self) -> Wkupt12W<WuirSpec> {
        Wkupt12W::new(self, 28)
    }
    #[doc = "Bit 29 - Wake Up Input Type 13"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt13(&mut self) -> Wkupt13W<WuirSpec> {
        Wkupt13W::new(self, 29)
    }
    #[doc = "Bit 30 - Wake Up Input Type 14"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt14(&mut self) -> Wkupt14W<WuirSpec> {
        Wkupt14W::new(self, 30)
    }
    #[doc = "Bit 31 - Wake Up Input Type 15"]
    #[inline(always)]
    #[must_use]
    pub fn wkupt15(&mut self) -> Wkupt15W<WuirSpec> {
        Wkupt15W::new(self, 31)
    }
}
#[doc = "Supply Controller Wake Up Inputs Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wuir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WuirSpec;
impl crate::RegisterSpec for WuirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wuir::R`](R) reader structure"]
impl crate::Readable for WuirSpec {}
#[doc = "`write(|w| ..)` method takes [`wuir::W`](W) writer structure"]
impl crate::Writable for WuirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUIR to value 0"]
impl crate::Resettable for WuirSpec {
    const RESET_VALUE: u32 = 0;
}
