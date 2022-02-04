#[doc = "Register `WPSR` reader"]
pub struct R(crate::R<WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Write Protection Violation Status"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WP_VS_A {
    #[doc = "0: No Write Protection Violation occurred since the last read of this register (WP_SR)"]
    NONE = 0,
    #[doc = "1: Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    WRITE = 1,
    #[doc = "2: Software reset had been performed while Write Protection was enabled (since the last read)."]
    RESET = 2,
    #[doc = "3: Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    BOTH = 3,
}
impl From<WP_VS_A> for u8 {
    #[inline(always)]
    fn from(variant: WP_VS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WP_VS` reader - Write Protection Violation Status"]
pub struct WP_VS_R(crate::FieldReader<u8, WP_VS_A>);
impl WP_VS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WP_VS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WP_VS_A> {
        match self.bits {
            0 => Some(WP_VS_A::NONE),
            1 => Some(WP_VS_A::WRITE),
            2 => Some(WP_VS_A::RESET),
            3 => Some(WP_VS_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == WP_VS_A::NONE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == WP_VS_A::WRITE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == WP_VS_A::RESET
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == WP_VS_A::BOTH
    }
}
impl core::ops::Deref for WP_VS_R {
    type Target = crate::FieldReader<u8, WP_VS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP_VSRC` reader - Write Protection Violation SouRCe"]
pub struct WP_VSRC_R(crate::FieldReader<u16, u16>);
impl WP_VSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WP_VSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_VSRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wp_vs(&self) -> WP_VS_R {
        WP_VS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Write Protection Violation SouRCe"]
    #[inline(always)]
    pub fn wp_vsrc(&self) -> WP_VSRC_R {
        WP_VSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsr::R](R) reader structure"]
impl crate::Readable for WPSR_SPEC {
    type Reader = R;
}
