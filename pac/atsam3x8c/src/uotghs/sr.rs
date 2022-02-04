#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDTI` reader - ID Transition Interrupt"]
pub struct IDTI_R(crate::FieldReader<bool, bool>);
impl IDTI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSTI` reader - VBus Transition Interrupt"]
pub struct VBUSTI_R(crate::FieldReader<bool, bool>);
impl VBUSTI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPI` reader - SRP Interrupt"]
pub struct SRPI_R(crate::FieldReader<bool, bool>);
impl SRPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBERRI` reader - VBus Error Interrupt"]
pub struct VBERRI_R(crate::FieldReader<bool, bool>);
impl VBERRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCERRI` reader - B-Connection Error Interrupt"]
pub struct BCERRI_R(crate::FieldReader<bool, bool>);
impl BCERRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROLEEXI` reader - Role Exchange Interrupt"]
pub struct ROLEEXI_R(crate::FieldReader<bool, bool>);
impl ROLEEXI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROLEEXI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROLEEXI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNPERRI` reader - HNP Error Interrupt"]
pub struct HNPERRI_R(crate::FieldReader<bool, bool>);
impl HNPERRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HNPERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNPERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOI` reader - Suspend Time-Out Interrupt"]
pub struct STOI_R(crate::FieldReader<bool, bool>);
impl STOI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSRQ` reader - VBus Request"]
pub struct VBUSRQ_R(crate::FieldReader<bool, bool>);
impl VBUSRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` reader - UOTGID Pin State"]
pub struct ID_R(crate::FieldReader<bool, bool>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS` reader - VBus Level"]
pub struct VBUS_R(crate::FieldReader<bool, bool>);
impl VBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Full-Speed mode"]
    FULL_SPEED = 0,
    #[doc = "1: High-Speed mode"]
    HIGH_SPEED = 1,
    #[doc = "2: Low-Speed mode"]
    LOW_SPEED = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED` reader - Speed Status"]
pub struct SPEED_R(crate::FieldReader<u8, SPEED_A>);
impl SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            0 => Some(SPEED_A::FULL_SPEED),
            1 => Some(SPEED_A::HIGH_SPEED),
            2 => Some(SPEED_A::LOW_SPEED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        **self == SPEED_A::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == SPEED_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        **self == SPEED_A::LOW_SPEED
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<u8, SPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKUSABLE` reader - UTMI Clock Usable"]
pub struct CLKUSABLE_R(crate::FieldReader<bool, bool>);
impl CLKUSABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKUSABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKUSABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt"]
    #[inline(always)]
    pub fn idti(&self) -> IDTI_R {
        IDTI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt"]
    #[inline(always)]
    pub fn vbusti(&self) -> VBUSTI_R {
        VBUSTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt"]
    #[inline(always)]
    pub fn srpi(&self) -> SRPI_R {
        SRPI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt"]
    #[inline(always)]
    pub fn vberri(&self) -> VBERRI_R {
        VBERRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt"]
    #[inline(always)]
    pub fn bcerri(&self) -> BCERRI_R {
        BCERRI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt"]
    #[inline(always)]
    pub fn roleexi(&self) -> ROLEEXI_R {
        ROLEEXI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt"]
    #[inline(always)]
    pub fn hnperri(&self) -> HNPERRI_R {
        HNPERRI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt"]
    #[inline(always)]
    pub fn stoi(&self) -> STOI_R {
        STOI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VBus Request"]
    #[inline(always)]
    pub fn vbusrq(&self) -> VBUSRQ_R {
        VBUSRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UOTGID Pin State"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VBus Level"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x0400"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
