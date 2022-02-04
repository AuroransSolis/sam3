#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP0INT` reader - Endpoint 0 Interrupt Status"]
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
#[doc = "Field `EP1INT` reader - Endpoint 1 Interrupt Status"]
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
#[doc = "Field `EP2INT` reader - Endpoint 2 Interrupt Status"]
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
#[doc = "Field `EP3INT` reader - Endpoint 3 Interrupt Status"]
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
#[doc = "Field `EP4INT` reader - Endpoint 4 Interrupt Status"]
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
#[doc = "Field `EP5INT` reader - Endpoint 5 Interrupt Status"]
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
#[doc = "Field `EP6INT` reader - Endpoint 6 Interrupt Status"]
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
#[doc = "Field `EP7INT` reader - Endpoint 7Interrupt Status"]
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
#[doc = "Field `RXSUSP` reader - UDP Suspend Interrupt Status"]
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
#[doc = "Field `RXRSM` reader - UDP Resume Interrupt Status"]
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
#[doc = "Field `SOFINT` reader - Start of Frame Interrupt Status"]
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
#[doc = "Field `ENDBUSRES` reader - End of BUS Reset Interrupt Status"]
pub struct ENDBUSRES_R(crate::FieldReader<bool, bool>);
impl ENDBUSRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDBUSRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDBUSRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` reader - UDP Resume Interrupt Status"]
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
    #[doc = "Bit 0 - Endpoint 0 Interrupt Status"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Interrupt Status"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Interrupt Status"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Interrupt Status"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Interrupt Status"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Interrupt Status"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Interrupt Status"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7Interrupt Status"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UDP Suspend Interrupt Status"]
    #[inline(always)]
    pub fn rxsusp(&self) -> RXSUSP_R {
        RXSUSP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UDP Resume Interrupt Status"]
    #[inline(always)]
    pub fn rxrsm(&self) -> RXRSM_R {
        RXRSM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&self) -> EXTRSM_R {
        EXTRSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Start of Frame Interrupt Status"]
    #[inline(always)]
    pub fn sofint(&self) -> SOFINT_R {
        SOFINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - End of BUS Reset Interrupt Status"]
    #[inline(always)]
    pub fn endbusres(&self) -> ENDBUSRES_R {
        ENDBUSRES_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UDP Resume Interrupt Status"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
