#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RDRF` reader - Receive Data Register Full"]
pub type RdrfR = crate::BitReader;
#[doc = "Field `TDRE` reader - Transmit Data Register Empty"]
pub type TdreR = crate::BitReader;
#[doc = "Field `MODF` reader - Mode Fault Error"]
pub type ModfR = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Status"]
pub type OvresR = crate::BitReader;
#[doc = "Field `NSSR` reader - NSS Rising"]
pub type NssrR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNDES` reader - Underrun Error Status (Slave Mode Only)"]
pub type UndesR = crate::BitReader;
#[doc = "Field `SPIENS` reader - SPI Enable Status"]
pub type SpiensR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status"]
    #[inline(always)]
    pub fn ovres(&self) -> OvresR {
        OvresR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising"]
    #[inline(always)]
    pub fn nssr(&self) -> NssrR {
        NssrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Status (Slave Mode Only)"]
    #[inline(always)]
    pub fn undes(&self) -> UndesR {
        UndesR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Enable Status"]
    #[inline(always)]
    pub fn spiens(&self) -> SpiensR {
        SpiensR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0xf0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xf0;
}
