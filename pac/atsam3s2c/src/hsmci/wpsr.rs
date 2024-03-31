#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WpsrSpec>;
#[doc = "Write Protection Violation Status"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WpVs {
    #[doc = "0: No Write Protection Violation occurred since the last read of this register (WP_SR)"]
    None = 0,
    #[doc = "1: Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    Write = 1,
    #[doc = "2: Software reset had been performed while Write Protection was enabled (since the last read)."]
    Reset = 2,
    #[doc = "3: Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    Both = 3,
}
impl From<WpVs> for u8 {
    #[inline(always)]
    fn from(variant: WpVs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WpVs {
    type Ux = u8;
}
impl crate::IsEnum for WpVs {}
#[doc = "Field `WP_VS` reader - Write Protection Violation Status"]
pub type WpVsR = crate::FieldReader<WpVs>;
impl WpVsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WpVs> {
        match self.bits {
            0 => Some(WpVs::None),
            1 => Some(WpVs::Write),
            2 => Some(WpVs::Reset),
            3 => Some(WpVs::Both),
            _ => None,
        }
    }
    #[doc = "No Write Protection Violation occurred since the last read of this register (WP_SR)"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WpVs::None
    }
    #[doc = "Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == WpVs::Write
    }
    #[doc = "Software reset had been performed while Write Protection was enabled (since the last read)."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WpVs::Reset
    }
    #[doc = "Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == WpVs::Both
    }
}
#[doc = "Field `WP_VSRC` reader - Write Protection Violation SouRCe"]
pub type WpVsrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wp_vs(&self) -> WpVsR {
        WpVsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Write Protection Violation SouRCe"]
    #[inline(always)]
    pub fn wp_vsrc(&self) -> WpVsrcR {
        WpVsrcR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsrSpec;
impl crate::RegisterSpec for WpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WpsrSpec {}
