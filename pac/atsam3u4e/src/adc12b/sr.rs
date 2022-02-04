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
#[doc = "Field `EOC0` reader - End of Conversion 0"]
pub struct EOC0_R(crate::FieldReader<bool, bool>);
impl EOC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC1` reader - End of Conversion 1"]
pub struct EOC1_R(crate::FieldReader<bool, bool>);
impl EOC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC2` reader - End of Conversion 2"]
pub struct EOC2_R(crate::FieldReader<bool, bool>);
impl EOC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC3` reader - End of Conversion 3"]
pub struct EOC3_R(crate::FieldReader<bool, bool>);
impl EOC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC4` reader - End of Conversion 4"]
pub struct EOC4_R(crate::FieldReader<bool, bool>);
impl EOC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC5` reader - End of Conversion 5"]
pub struct EOC5_R(crate::FieldReader<bool, bool>);
impl EOC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC6` reader - End of Conversion 6"]
pub struct EOC6_R(crate::FieldReader<bool, bool>);
impl EOC6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC7` reader - End of Conversion 7"]
pub struct EOC7_R(crate::FieldReader<bool, bool>);
impl EOC7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE0` reader - Overrun Error 0"]
pub struct OVRE0_R(crate::FieldReader<bool, bool>);
impl OVRE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE1` reader - Overrun Error 1"]
pub struct OVRE1_R(crate::FieldReader<bool, bool>);
impl OVRE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE2` reader - Overrun Error 2"]
pub struct OVRE2_R(crate::FieldReader<bool, bool>);
impl OVRE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE3` reader - Overrun Error 3"]
pub struct OVRE3_R(crate::FieldReader<bool, bool>);
impl OVRE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE4` reader - Overrun Error 4"]
pub struct OVRE4_R(crate::FieldReader<bool, bool>);
impl OVRE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE5` reader - Overrun Error 5"]
pub struct OVRE5_R(crate::FieldReader<bool, bool>);
impl OVRE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE6` reader - Overrun Error 6"]
pub struct OVRE6_R(crate::FieldReader<bool, bool>);
impl OVRE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE7` reader - Overrun Error 7"]
pub struct OVRE7_R(crate::FieldReader<bool, bool>);
impl OVRE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRDY` reader - Data Ready"]
pub struct DRDY_R(crate::FieldReader<bool, bool>);
impl DRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOVRE` reader - General Overrun Error"]
pub struct GOVRE_R(crate::FieldReader<bool, bool>);
impl GOVRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GOVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDRX` reader - End of RX Buffer"]
pub struct ENDRX_R(crate::FieldReader<bool, bool>);
impl ENDRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub struct RXBUFF_R(crate::FieldReader<bool, bool>);
impl RXBUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of Conversion 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Conversion 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Conversion 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Conversion 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Conversion 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> EOC4_R {
        EOC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> EOC5_R {
        EOC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End of Conversion 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> EOC6_R {
        EOC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - End of Conversion 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> EOC7_R {
        EOC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> OVRE0_R {
        OVRE0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> OVRE1_R {
        OVRE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> OVRE2_R {
        OVRE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> OVRE3_R {
        OVRE3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> OVRE4_R {
        OVRE4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> OVRE5_R {
        OVRE5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> OVRE6_R {
        OVRE6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> OVRE7_R {
        OVRE7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - General Overrun Error"]
    #[inline(always)]
    pub fn govre(&self) -> GOVRE_R {
        GOVRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - End of RX Buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x000c_0000"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_0000
    }
}
