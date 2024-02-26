#[doc = "Register `HSTPIPIMR0` reader"]
pub type R = crate::R<Hstpipimr0Spec>;
#[doc = "Field `RXINE` reader - Received IN Data Interrupt Enable"]
pub type RxineR = crate::BitReader;
#[doc = "Field `TXOUTE` reader - Transmitted OUT Data Interrupt Enable"]
pub type TxouteR = crate::BitReader;
#[doc = "Field `TXSTPE` reader - Transmitted SETUP Interrupt Enable"]
pub type TxstpeR = crate::BitReader;
#[doc = "Field `PERRE` reader - Pipe Error Interrupt Enable"]
pub type PerreR = crate::BitReader;
#[doc = "Field `NAKEDE` reader - NAKed Interrupt Enable"]
pub type NakedeR = crate::BitReader;
#[doc = "Field `OVERFIE` reader - Overflow Interrupt Enable"]
pub type OverfieR = crate::BitReader;
#[doc = "Field `RXSTALLDE` reader - Received STALLed Interrupt Enable"]
pub type RxstalldeR = crate::BitReader;
#[doc = "Field `SHORTPACKETIE` reader - Short Packet Interrupt Enable"]
pub type ShortpacketieR = crate::BitReader;
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt Enable"]
pub type NbusybkeR = crate::BitReader;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FifoconR = crate::BitReader;
#[doc = "Field `PDISHDMA` reader - Pipe Interrupts Disable HDMA Request Enable"]
pub type PdishdmaR = crate::BitReader;
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub type PfreezeR = crate::BitReader;
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub type RstdtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxine(&self) -> RxineR {
        RxineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn txoute(&self) -> TxouteR {
        TxouteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Enable"]
    #[inline(always)]
    pub fn txstpe(&self) -> TxstpeR {
        TxstpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perre(&self) -> PerreR {
        PerreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    pub fn nakede(&self) -> NakedeR {
        NakedeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfie(&self) -> OverfieR {
        OverfieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    pub fn rxstallde(&self) -> RxstalldeR {
        RxstalldeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketie(&self) -> ShortpacketieR {
        ShortpacketieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NbusybkeR {
        NbusybkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FifoconR {
        FifoconR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn pdishdma(&self) -> PdishdmaR {
        PdishdmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PfreezeR {
        PfreezeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RstdtR {
        RstdtR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Host Pipe Mask Register (n = 0) 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpipimr0Spec;
impl crate::RegisterSpec for Hstpipimr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipimr0::R`](R) reader structure"]
impl crate::Readable for Hstpipimr0Spec {}
