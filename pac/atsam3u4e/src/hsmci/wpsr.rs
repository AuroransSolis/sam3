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
pub enum WPVS_A {
    #[doc = "0: No Write Protection Violation occurred since the last read of this register (HSMCI_WPSR)"]
    NONE = 0,
    #[doc = "1: Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    WRITE = 1,
    #[doc = "2: Software reset had been performed while Write Protection was enabled (since the last read)."]
    RESET = 2,
    #[doc = "3: Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    BOTH = 3,
}
impl From<WPVS_A> for u8 {
    #[inline(always)]
    fn from(variant: WPVS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub struct WPVS_R(crate::FieldReader<u8, WPVS_A>);
impl WPVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WPVS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPVS_A> {
        match self.bits {
            0 => Some(WPVS_A::NONE),
            1 => Some(WPVS_A::WRITE),
            2 => Some(WPVS_A::RESET),
            3 => Some(WPVS_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == WPVS_A::NONE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == WPVS_A::WRITE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == WPVS_A::RESET
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == WPVS_A::BOTH
    }
}
impl core::ops::Deref for WPVS_R {
    type Target = crate::FieldReader<u8, WPVS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPVSRC` reader - Write Protection Violation SouRCe"]
pub struct WPVSRC_R(crate::FieldReader<u16, u16>);
impl WPVSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WPVSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPVSRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Write Protection Violation SouRCe"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
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
