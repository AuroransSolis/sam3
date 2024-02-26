#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `EP0INT` reader - Mask Endpoint 0 Interrupt"]
pub type Ep0intR = crate::BitReader;
#[doc = "Field `EP1INT` reader - Mask Endpoint 1 Interrupt"]
pub type Ep1intR = crate::BitReader;
#[doc = "Field `EP2INT` reader - Mask Endpoint 2 Interrupt"]
pub type Ep2intR = crate::BitReader;
#[doc = "Field `EP3INT` reader - Mask Endpoint 3 Interrupt"]
pub type Ep3intR = crate::BitReader;
#[doc = "Field `EP4INT` reader - Mask Endpoint 4 Interrupt"]
pub type Ep4intR = crate::BitReader;
#[doc = "Field `EP5INT` reader - Mask Endpoint 5 Interrupt"]
pub type Ep5intR = crate::BitReader;
#[doc = "Field `EP6INT` reader - Mask Endpoint 6 Interrupt"]
pub type Ep6intR = crate::BitReader;
#[doc = "Field `EP7INT` reader - Mask Endpoint 7 Interrupt"]
pub type Ep7intR = crate::BitReader;
#[doc = "Field `RXSUSP` reader - Mask UDP Suspend Interrupt"]
pub type RxsuspR = crate::BitReader;
#[doc = "Field `RXRSM` reader - Mask UDP Resume Interrupt."]
pub type RxrsmR = crate::BitReader;
#[doc = "Field `EXTRSM` reader - "]
pub type ExtrsmR = crate::BitReader;
#[doc = "Field `SOFINT` reader - Mask Start Of Frame Interrupt"]
pub type SofintR = crate::BitReader;
#[doc = "Field `BIT12` reader - UDP_IMR Bit 12"]
pub type Bit12R = crate::BitReader;
#[doc = "Field `WAKEUP` reader - USB Bus WAKEUP Interrupt"]
pub type WakeupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mask Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&self) -> Ep0intR {
        Ep0intR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&self) -> Ep1intR {
        Ep1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&self) -> Ep2intR {
        Ep2intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&self) -> Ep3intR {
        Ep3intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&self) -> Ep4intR {
        Ep4intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&self) -> Ep5intR {
        Ep5intR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&self) -> Ep6intR {
        Ep6intR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&self) -> Ep7intR {
        Ep7intR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask UDP Suspend Interrupt"]
    #[inline(always)]
    pub fn rxsusp(&self) -> RxsuspR {
        RxsuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask UDP Resume Interrupt."]
    #[inline(always)]
    pub fn rxrsm(&self) -> RxrsmR {
        RxrsmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&self) -> ExtrsmR {
        ExtrsmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn sofint(&self) -> SofintR {
        SofintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UDP_IMR Bit 12"]
    #[inline(always)]
    pub fn bit12(&self) -> Bit12R {
        Bit12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USB Bus WAKEUP Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0x1200"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0x1200;
}
