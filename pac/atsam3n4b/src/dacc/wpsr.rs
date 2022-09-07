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
#[doc = "Field `WPROTERR` reader - Write protection error"]
pub type WPROTERR_R = crate::BitReader<bool>;
#[doc = "Field `WPROTADDR` reader - Write protection error address"]
pub type WPROTADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Write protection error"]
    #[inline(always)]
    pub fn wproterr(&self) -> WPROTERR_R {
        WPROTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Write protection error address"]
    #[inline(always)]
    pub fn wprotaddr(&self) -> WPROTADDR_R {
        WPROTADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsr::R](R) reader structure"]
impl crate::Readable for WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
