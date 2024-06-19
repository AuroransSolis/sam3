#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WpsrSpec>;
#[doc = "Write Protection Violation Status"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wpvs {
    #[doc = "0: No Write Protection Violation occurred since the last read of this register (HSMCI_WPSR)"]
    None = 0,
    #[doc = "1: Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    Write = 1,
    #[doc = "2: Software reset had been performed while Write Protection was enabled (since the last read)."]
    Reset = 2,
    #[doc = "3: Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    Both = 3,
}
impl From<Wpvs> for u8 {
    #[inline(always)]
    fn from(variant: Wpvs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpvs {
    type Ux = u8;
}
impl crate::IsEnum for Wpvs {}
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub type WpvsR = crate::FieldReader<Wpvs>;
impl WpvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wpvs> {
        match self.bits {
            0 => Some(Wpvs::None),
            1 => Some(Wpvs::Write),
            2 => Some(Wpvs::Reset),
            3 => Some(Wpvs::Both),
            _ => None,
        }
    }
    #[doc = "No Write Protection Violation occurred since the last read of this register (HSMCI_WPSR)"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wpvs::None
    }
    #[doc = "Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Wpvs::Write
    }
    #[doc = "Software reset had been performed while Write Protection was enabled (since the last read)."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Wpvs::Reset
    }
    #[doc = "Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Wpvs::Both
    }
}
#[doc = "Field `WPVSRC` reader - Write Protection Violation SouRCe"]
pub type WpvsrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WpvsR {
        WpvsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Write Protection Violation SouRCe"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WpvsrcR {
        WpvsrcR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsrSpec;
impl crate::RegisterSpec for WpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WpsrSpec {}
