#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WPSR_SPEC>;
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub type WPVS_R = crate::FieldReader<WPVS_A>;
#[doc = "Write Protection Violation Status"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WPVS_A {
    #[doc = "0: No Write Protection Violation occurred since the last read of this register (HSMCI_WPSR)"]
    None = 0,
    #[doc = "1: Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    Write = 1,
    #[doc = "2: Software reset had been performed while Write Protection was enabled (since the last read)."]
    Reset = 2,
    #[doc = "3: Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    Both = 3,
}
impl From<WPVS_A> for u8 {
    #[inline(always)]
    fn from(variant: WPVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPVS_A {
    type Ux = u8;
}
impl WPVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPVS_A> {
        match self.bits {
            0 => Some(WPVS_A::None),
            1 => Some(WPVS_A::Write),
            2 => Some(WPVS_A::Reset),
            3 => Some(WPVS_A::Both),
            _ => None,
        }
    }
    #[doc = "No Write Protection Violation occurred since the last read of this register (HSMCI_WPSR)"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WPVS_A::None
    }
    #[doc = "Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == WPVS_A::Write
    }
    #[doc = "Software reset had been performed while Write Protection was enabled (since the last read)."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WPVS_A::Reset
    }
    #[doc = "Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == WPVS_A::Both
    }
}
#[doc = "Field `WPVSRC` reader - Write Protection Violation SouRCe"]
pub type WPVSRC_R = crate::FieldReader<u16>;
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
#[doc = "Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WPSR_SPEC {}
