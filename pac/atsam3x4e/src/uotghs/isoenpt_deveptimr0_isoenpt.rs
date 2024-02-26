#[doc = "Register `DEVEPTIMR0_ISOENPT` reader"]
pub type R = crate::R<IsoenptDeveptimr0IsoenptSpec>;
#[doc = "Field `TXINE` reader - Transmitted IN Data Interrupt"]
pub type TxineR = crate::BitReader;
#[doc = "Field `RXOUTE` reader - Received OUT Data Interrupt"]
pub type RxouteR = crate::BitReader;
#[doc = "Field `UNDERFE` reader - Underflow Interrupt"]
pub type UnderfeR = crate::BitReader;
#[doc = "Field `HBISOINERRE` reader - High Bandwidth Isochronous IN Error Interrupt"]
pub type HbisoinerreR = crate::BitReader;
#[doc = "Field `HBISOFLUSHE` reader - High Bandwidth Isochronous IN Flush Interrupt"]
pub type HbisoflusheR = crate::BitReader;
#[doc = "Field `OVERFE` reader - Overflow Interrupt"]
pub type OverfeR = crate::BitReader;
#[doc = "Field `CRCERRE` reader - CRC Error Interrupt"]
pub type CrcerreR = crate::BitReader;
#[doc = "Field `SHORTPACKETE` reader - Short Packet Interrupt"]
pub type ShortpacketeR = crate::BitReader;
#[doc = "Field `MDATAE` reader - MData Interrupt"]
pub type MdataeR = crate::BitReader;
#[doc = "Field `DATAXE` reader - DataX Interrupt"]
pub type DataxeR = crate::BitReader;
#[doc = "Field `ERRORTRANSE` reader - Transaction Error Interrupt"]
pub type ErrortranseR = crate::BitReader;
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt"]
pub type NbusybkeR = crate::BitReader;
#[doc = "Field `KILLBK` reader - Kill IN Bank"]
pub type KillbkR = crate::BitReader;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FifoconR = crate::BitReader;
#[doc = "Field `EPDISHDMA` reader - Endpoint Interrupts Disable HDMA Request"]
pub type EpdishdmaR = crate::BitReader;
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub type RstdtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txine(&self) -> TxineR {
        TxineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxoute(&self) -> RxouteR {
        RxouteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt"]
    #[inline(always)]
    pub fn underfe(&self) -> UnderfeR {
        UnderfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt"]
    #[inline(always)]
    pub fn hbisoinerre(&self) -> HbisoinerreR {
        HbisoinerreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt"]
    #[inline(always)]
    pub fn hbisoflushe(&self) -> HbisoflusheR {
        HbisoflusheR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfe(&self) -> OverfeR {
        OverfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn crcerre(&self) -> CrcerreR {
        CrcerreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpackete(&self) -> ShortpacketeR {
        ShortpacketeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MData Interrupt"]
    #[inline(always)]
    pub fn mdatae(&self) -> MdataeR {
        MdataeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DataX Interrupt"]
    #[inline(always)]
    pub fn dataxe(&self) -> DataxeR {
        DataxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt"]
    #[inline(always)]
    pub fn errortranse(&self) -> ErrortranseR {
        ErrortranseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NbusybkeR {
        NbusybkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KillbkR {
        KillbkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FifoconR {
        FifoconR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request"]
    #[inline(always)]
    pub fn epdishdma(&self) -> EpdishdmaR {
        EpdishdmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RstdtR {
        RstdtR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Device Endpoint Mask Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isoenpt_deveptimr0_isoenpt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoenptDeveptimr0IsoenptSpec;
impl crate::RegisterSpec for IsoenptDeveptimr0IsoenptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoenpt_deveptimr0_isoenpt::R`](R) reader structure"]
impl crate::Readable for IsoenptDeveptimr0IsoenptSpec {}
