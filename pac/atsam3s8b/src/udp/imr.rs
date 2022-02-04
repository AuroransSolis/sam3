#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP0INT` reader - Mask Endpoint 0 Interrupt"]
pub struct EP0INT_R(crate::FieldReader<bool, bool>);
impl EP0INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1INT` reader - Mask Endpoint 1 Interrupt"]
pub struct EP1INT_R(crate::FieldReader<bool, bool>);
impl EP1INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2INT` reader - Mask Endpoint 2 Interrupt"]
pub struct EP2INT_R(crate::FieldReader<bool, bool>);
impl EP2INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP2INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3INT` reader - Mask Endpoint 3 Interrupt"]
pub struct EP3INT_R(crate::FieldReader<bool, bool>);
impl EP3INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP3INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4INT` reader - Mask Endpoint 4 Interrupt"]
pub struct EP4INT_R(crate::FieldReader<bool, bool>);
impl EP4INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5INT` reader - Mask Endpoint 5 Interrupt"]
pub struct EP5INT_R(crate::FieldReader<bool, bool>);
impl EP5INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP5INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6INT` reader - Mask Endpoint 6 Interrupt"]
pub struct EP6INT_R(crate::FieldReader<bool, bool>);
impl EP6INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP6INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7INT` reader - Mask Endpoint 7 Interrupt"]
pub struct EP7INT_R(crate::FieldReader<bool, bool>);
impl EP7INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP7INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSUSP` reader - Mask UDP Suspend Interrupt"]
pub struct RXSUSP_R(crate::FieldReader<bool, bool>);
impl RXSUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRSM` reader - Mask UDP Resume Interrupt."]
pub struct RXRSM_R(crate::FieldReader<bool, bool>);
impl RXRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTRSM` reader - "]
pub struct EXTRSM_R(crate::FieldReader<bool, bool>);
impl EXTRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFINT` reader - Mask Start Of Frame Interrupt"]
pub struct SOFINT_R(crate::FieldReader<bool, bool>);
impl SOFINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT12` reader - UDP_IMR Bit 12"]
pub struct BIT12_R(crate::FieldReader<bool, bool>);
impl BIT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` reader - USB Bus WAKEUP Interrupt"]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Mask Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask UDP Suspend Interrupt"]
    #[inline(always)]
    pub fn rxsusp(&self) -> RXSUSP_R {
        RXSUSP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask UDP Resume Interrupt."]
    #[inline(always)]
    pub fn rxrsm(&self) -> RXRSM_R {
        RXRSM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&self) -> EXTRSM_R {
        EXTRSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn sofint(&self) -> SOFINT_R {
        SOFINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UDP_IMR Bit 12"]
    #[inline(always)]
    pub fn bit12(&self) -> BIT12_R {
        BIT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USB Bus WAKEUP Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0x1200"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1200
    }
}
