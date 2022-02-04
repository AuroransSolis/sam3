#[doc = "Register `CHSR` reader"]
pub struct R(crate::R<CHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENA0` reader - Enable \\[5:0\\]"]
pub struct ENA0_R(crate::FieldReader<bool, bool>);
impl ENA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA1` reader - Enable \\[5:0\\]"]
pub struct ENA1_R(crate::FieldReader<bool, bool>);
impl ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA2` reader - Enable \\[5:0\\]"]
pub struct ENA2_R(crate::FieldReader<bool, bool>);
impl ENA2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA3` reader - Enable \\[5:0\\]"]
pub struct ENA3_R(crate::FieldReader<bool, bool>);
impl ENA3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA4` reader - Enable \\[5:0\\]"]
pub struct ENA4_R(crate::FieldReader<bool, bool>);
impl ENA4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA5` reader - Enable \\[5:0\\]"]
pub struct ENA5_R(crate::FieldReader<bool, bool>);
impl ENA5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP0` reader - Suspend \\[5:0\\]"]
pub struct SUSP0_R(crate::FieldReader<bool, bool>);
impl SUSP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP1` reader - Suspend \\[5:0\\]"]
pub struct SUSP1_R(crate::FieldReader<bool, bool>);
impl SUSP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP2` reader - Suspend \\[5:0\\]"]
pub struct SUSP2_R(crate::FieldReader<bool, bool>);
impl SUSP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP3` reader - Suspend \\[5:0\\]"]
pub struct SUSP3_R(crate::FieldReader<bool, bool>);
impl SUSP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP4` reader - Suspend \\[5:0\\]"]
pub struct SUSP4_R(crate::FieldReader<bool, bool>);
impl SUSP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP5` reader - Suspend \\[5:0\\]"]
pub struct SUSP5_R(crate::FieldReader<bool, bool>);
impl SUSP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPT0` reader - Empty \\[5:0\\]"]
pub struct EMPT0_R(crate::FieldReader<bool, bool>);
impl EMPT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPT1` reader - Empty \\[5:0\\]"]
pub struct EMPT1_R(crate::FieldReader<bool, bool>);
impl EMPT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPT2` reader - Empty \\[5:0\\]"]
pub struct EMPT2_R(crate::FieldReader<bool, bool>);
impl EMPT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPT3` reader - Empty \\[5:0\\]"]
pub struct EMPT3_R(crate::FieldReader<bool, bool>);
impl EMPT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPT4` reader - Empty \\[5:0\\]"]
pub struct EMPT4_R(crate::FieldReader<bool, bool>);
impl EMPT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPT5` reader - Empty \\[5:0\\]"]
pub struct EMPT5_R(crate::FieldReader<bool, bool>);
impl EMPT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAL0` reader - Stalled \\[5:0\\]"]
pub struct STAL0_R(crate::FieldReader<bool, bool>);
impl STAL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STAL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAL1` reader - Stalled \\[5:0\\]"]
pub struct STAL1_R(crate::FieldReader<bool, bool>);
impl STAL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAL2` reader - Stalled \\[5:0\\]"]
pub struct STAL2_R(crate::FieldReader<bool, bool>);
impl STAL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STAL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAL3` reader - Stalled \\[5:0\\]"]
pub struct STAL3_R(crate::FieldReader<bool, bool>);
impl STAL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STAL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAL4` reader - Stalled \\[5:0\\]"]
pub struct STAL4_R(crate::FieldReader<bool, bool>);
impl STAL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STAL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAL5` reader - Stalled \\[5:0\\]"]
pub struct STAL5_R(crate::FieldReader<bool, bool>);
impl STAL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STAL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena0(&self) -> ENA0_R {
        ENA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena1(&self) -> ENA1_R {
        ENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena2(&self) -> ENA2_R {
        ENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena3(&self) -> ENA3_R {
        ENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena4(&self) -> ENA4_R {
        ENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena5(&self) -> ENA5_R {
        ENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp0(&self) -> SUSP0_R {
        SUSP0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp1(&self) -> SUSP1_R {
        SUSP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp2(&self) -> SUSP2_R {
        SUSP2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp3(&self) -> SUSP3_R {
        SUSP3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp4(&self) -> SUSP4_R {
        SUSP4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp5(&self) -> SUSP5_R {
        SUSP5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt0(&self) -> EMPT0_R {
        EMPT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt1(&self) -> EMPT1_R {
        EMPT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt2(&self) -> EMPT2_R {
        EMPT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt3(&self) -> EMPT3_R {
        EMPT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt4(&self) -> EMPT4_R {
        EMPT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt5(&self) -> EMPT5_R {
        EMPT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal0(&self) -> STAL0_R {
        STAL0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal1(&self) -> STAL1_R {
        STAL1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal2(&self) -> STAL2_R {
        STAL2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal3(&self) -> STAL3_R {
        STAL3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal4(&self) -> STAL4_R {
        STAL4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal5(&self) -> STAL5_R {
        STAL5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "DMAC Channel Handler Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsr](index.html) module"]
pub struct CHSR_SPEC;
impl crate::RegisterSpec for CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsr::R](R) reader structure"]
impl crate::Readable for CHSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSR to value 0x00ff_0000"]
impl crate::Resettable for CHSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_0000
    }
}
