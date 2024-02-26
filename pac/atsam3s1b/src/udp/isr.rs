#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EP0INT` reader - Endpoint 0 Interrupt Status"]
pub type Ep0intR = crate::BitReader;
#[doc = "Field `EP1INT` reader - Endpoint 1 Interrupt Status"]
pub type Ep1intR = crate::BitReader;
#[doc = "Field `EP2INT` reader - Endpoint 2 Interrupt Status"]
pub type Ep2intR = crate::BitReader;
#[doc = "Field `EP3INT` reader - Endpoint 3 Interrupt Status"]
pub type Ep3intR = crate::BitReader;
#[doc = "Field `EP4INT` reader - Endpoint 4 Interrupt Status"]
pub type Ep4intR = crate::BitReader;
#[doc = "Field `EP5INT` reader - Endpoint 5 Interrupt Status"]
pub type Ep5intR = crate::BitReader;
#[doc = "Field `EP6INT` reader - Endpoint 6 Interrupt Status"]
pub type Ep6intR = crate::BitReader;
#[doc = "Field `EP7INT` reader - Endpoint 7Interrupt Status"]
pub type Ep7intR = crate::BitReader;
#[doc = "Field `RXSUSP` reader - UDP Suspend Interrupt Status"]
pub type RxsuspR = crate::BitReader;
#[doc = "Field `RXRSM` reader - UDP Resume Interrupt Status"]
pub type RxrsmR = crate::BitReader;
#[doc = "Field `EXTRSM` reader - "]
pub type ExtrsmR = crate::BitReader;
#[doc = "Field `SOFINT` reader - Start of Frame Interrupt Status"]
pub type SofintR = crate::BitReader;
#[doc = "Field `ENDBUSRES` reader - End of BUS Reset Interrupt Status"]
pub type EndbusresR = crate::BitReader;
#[doc = "Field `WAKEUP` reader - UDP Resume Interrupt Status"]
pub type WakeupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint 0 Interrupt Status"]
    #[inline(always)]
    pub fn ep0int(&self) -> Ep0intR {
        Ep0intR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Interrupt Status"]
    #[inline(always)]
    pub fn ep1int(&self) -> Ep1intR {
        Ep1intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Interrupt Status"]
    #[inline(always)]
    pub fn ep2int(&self) -> Ep2intR {
        Ep2intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Interrupt Status"]
    #[inline(always)]
    pub fn ep3int(&self) -> Ep3intR {
        Ep3intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Interrupt Status"]
    #[inline(always)]
    pub fn ep4int(&self) -> Ep4intR {
        Ep4intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Interrupt Status"]
    #[inline(always)]
    pub fn ep5int(&self) -> Ep5intR {
        Ep5intR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Interrupt Status"]
    #[inline(always)]
    pub fn ep6int(&self) -> Ep6intR {
        Ep6intR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7Interrupt Status"]
    #[inline(always)]
    pub fn ep7int(&self) -> Ep7intR {
        Ep7intR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDP Suspend Interrupt Status"]
    #[inline(always)]
    pub fn rxsusp(&self) -> RxsuspR {
        RxsuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UDP Resume Interrupt Status"]
    #[inline(always)]
    pub fn rxrsm(&self) -> RxrsmR {
        RxrsmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&self) -> ExtrsmR {
        ExtrsmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start of Frame Interrupt Status"]
    #[inline(always)]
    pub fn sofint(&self) -> SofintR {
        SofintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of BUS Reset Interrupt Status"]
    #[inline(always)]
    pub fn endbusres(&self) -> EndbusresR {
        EndbusresR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UDP Resume Interrupt Status"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
