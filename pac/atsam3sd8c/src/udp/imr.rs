#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Field `EP0INT` reader - Mask Endpoint 0 Interrupt"]
pub type EP0INT_R = crate::BitReader;
#[doc = "Field `EP1INT` reader - Mask Endpoint 1 Interrupt"]
pub type EP1INT_R = crate::BitReader;
#[doc = "Field `EP2INT` reader - Mask Endpoint 2 Interrupt"]
pub type EP2INT_R = crate::BitReader;
#[doc = "Field `EP3INT` reader - Mask Endpoint 3 Interrupt"]
pub type EP3INT_R = crate::BitReader;
#[doc = "Field `EP4INT` reader - Mask Endpoint 4 Interrupt"]
pub type EP4INT_R = crate::BitReader;
#[doc = "Field `EP5INT` reader - Mask Endpoint 5 Interrupt"]
pub type EP5INT_R = crate::BitReader;
#[doc = "Field `EP6INT` reader - Mask Endpoint 6 Interrupt"]
pub type EP6INT_R = crate::BitReader;
#[doc = "Field `EP7INT` reader - Mask Endpoint 7 Interrupt"]
pub type EP7INT_R = crate::BitReader;
#[doc = "Field `RXSUSP` reader - Mask UDP Suspend Interrupt"]
pub type RXSUSP_R = crate::BitReader;
#[doc = "Field `RXRSM` reader - Mask UDP Resume Interrupt."]
pub type RXRSM_R = crate::BitReader;
#[doc = "Field `EXTRSM` reader - "]
pub type EXTRSM_R = crate::BitReader;
#[doc = "Field `SOFINT` reader - Mask Start Of Frame Interrupt"]
pub type SOFINT_R = crate::BitReader;
#[doc = "Field `BIT12` reader - UDP_IMR Bit 12"]
pub type BIT12_R = crate::BitReader;
#[doc = "Field `WAKEUP` reader - USB Bus WAKEUP Interrupt"]
pub type WAKEUP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mask Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask UDP Suspend Interrupt"]
    #[inline(always)]
    pub fn rxsusp(&self) -> RXSUSP_R {
        RXSUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask UDP Resume Interrupt."]
    #[inline(always)]
    pub fn rxrsm(&self) -> RXRSM_R {
        RXRSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&self) -> EXTRSM_R {
        EXTRSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn sofint(&self) -> SOFINT_R {
        SOFINT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UDP_IMR Bit 12"]
    #[inline(always)]
    pub fn bit12(&self) -> BIT12_R {
        BIT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USB Bus WAKEUP Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`reset()` method sets IMR to value 0x1200"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: u32 = 0x1200;
}
