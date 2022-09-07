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
pub type EP0INT_R = crate::BitReader<bool>;
#[doc = "Field `EP1INT` reader - Endpoint 1 Interrupt Status"]
pub type EP1INT_R = crate::BitReader<bool>;
#[doc = "Field `EP2INT` reader - Endpoint 2 Interrupt Status"]
pub type EP2INT_R = crate::BitReader<bool>;
#[doc = "Field `EP3INT` reader - Endpoint 3 Interrupt Status"]
pub type EP3INT_R = crate::BitReader<bool>;
#[doc = "Field `EP4INT` reader - Endpoint 4 Interrupt Status"]
pub type EP4INT_R = crate::BitReader<bool>;
#[doc = "Field `EP5INT` reader - Endpoint 5 Interrupt Status"]
pub type EP5INT_R = crate::BitReader<bool>;
#[doc = "Field `EP6INT` reader - Endpoint 6 Interrupt Status"]
pub type EP6INT_R = crate::BitReader<bool>;
#[doc = "Field `EP7INT` reader - Endpoint 7Interrupt Status"]
pub type EP7INT_R = crate::BitReader<bool>;
#[doc = "Field `RXSUSP` reader - UDP Suspend Interrupt Status"]
pub type RXSUSP_R = crate::BitReader<bool>;
#[doc = "Field `RXRSM` reader - UDP Resume Interrupt Status"]
pub type RXRSM_R = crate::BitReader<bool>;
#[doc = "Field `EXTRSM` reader - "]
pub type EXTRSM_R = crate::BitReader<bool>;
#[doc = "Field `SOFINT` reader - Start of Frame Interrupt Status"]
pub type SOFINT_R = crate::BitReader<bool>;
#[doc = "Field `ENDBUSRES` reader - End of BUS Reset Interrupt Status"]
pub type ENDBUSRES_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` reader - UDP Resume Interrupt Status"]
pub type WAKEUP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 Interrupt Status"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Interrupt Status"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Interrupt Status"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Interrupt Status"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Interrupt Status"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Interrupt Status"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Interrupt Status"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7Interrupt Status"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDP Suspend Interrupt Status"]
    #[inline(always)]
    pub fn rxsusp(&self) -> RXSUSP_R {
        RXSUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UDP Resume Interrupt Status"]
    #[inline(always)]
    pub fn rxrsm(&self) -> RXRSM_R {
        RXRSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&self) -> EXTRSM_R {
        EXTRSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start of Frame Interrupt Status"]
    #[inline(always)]
    pub fn sofint(&self) -> SOFINT_R {
        SOFINT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of BUS Reset Interrupt Status"]
    #[inline(always)]
    pub fn endbusres(&self) -> ENDBUSRES_R {
        ENDBUSRES_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UDP Resume Interrupt Status"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 13) & 1) != 0)
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
