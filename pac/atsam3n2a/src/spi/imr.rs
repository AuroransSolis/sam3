#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RDRF` reader - Receive Data Register Full Interrupt Mask"]
pub type RdrfR = crate::BitReader;
#[doc = "Field `TDRE` reader - SPI Transmit Data Register Empty Interrupt Mask"]
pub type TdreR = crate::BitReader;
#[doc = "Field `MODF` reader - Mode Fault Error Interrupt Mask"]
pub type ModfR = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Interrupt Mask"]
pub type OvresR = crate::BitReader;
#[doc = "Field `NSSR` reader - NSS Rising Interrupt Mask"]
pub type NssrR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNDES` reader - Underrun Error Interrupt Mask"]
pub type UndesR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Mask"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OvresR {
        OvresR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Mask"]
    #[inline(always)]
    pub fn nssr(&self) -> NssrR {
        NssrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn undes(&self) -> UndesR {
        UndesR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
