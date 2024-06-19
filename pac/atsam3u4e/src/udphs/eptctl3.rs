#[doc = "Register `EPTCTL3` reader"]
pub type R = crate::R<Eptctl3Spec>;
#[doc = "Field `EPT_ENABL` reader - Endpoint Enable"]
pub type EptEnablR = crate::BitReader;
#[doc = "Field `AUTO_VALID` reader - Packet Auto-Valid Enabled (Not for CONTROL Endpoints)"]
pub type AutoValidR = crate::BitReader;
#[doc = "Field `INTDIS_DMA` reader - Interrupt Disables DMA"]
pub type IntdisDmaR = crate::BitReader;
#[doc = "Field `NYET_DIS` reader - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
pub type NyetDisR = crate::BitReader;
#[doc = "Field `ERR_OVFLW` reader - Overflow Error Interrupt Enabled"]
pub type ErrOvflwR = crate::BitReader;
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data Interrupt Enabled"]
pub type RxrdyTxklR = crate::BitReader;
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete Interrupt Enabled"]
pub type TxCompltR = crate::BitReader;
#[doc = "Field `TXRDY` reader - TX Packet Ready Interrupt Enabled"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `RX_SETUP` reader - Received SETUP Interrupt Enabled"]
pub type RxSetupR = crate::BitReader;
#[doc = "Field `STALL_SNT` reader - Stall Sent Interrupt Enabled"]
pub type StallSntR = crate::BitReader;
#[doc = "Field `NAK_IN` reader - NAKIN Interrupt Enabled"]
pub type NakInR = crate::BitReader;
#[doc = "Field `NAK_OUT` reader - NAKOUT Interrupt Enabled"]
pub type NakOutR = crate::BitReader;
#[doc = "Field `BUSY_BANK` reader - Busy Bank Interrupt Enabled"]
pub type BusyBankR = crate::BitReader;
#[doc = "Field `SHRT_PCKT` reader - Short Packet Interrupt Enabled"]
pub type ShrtPcktR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    pub fn ept_enabl(&self) -> EptEnablR {
        EptEnablR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enabled (Not for CONTROL Endpoints)"]
    #[inline(always)]
    pub fn auto_valid(&self) -> AutoValidR {
        AutoValidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Disables DMA"]
    #[inline(always)]
    pub fn intdis_dma(&self) -> IntdisDmaR {
        IntdisDmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    pub fn nyet_dis(&self) -> NyetDisR {
        NyetDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enabled"]
    #[inline(always)]
    pub fn err_ovflw(&self) -> ErrOvflwR {
        ErrOvflwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enabled"]
    #[inline(always)]
    pub fn rxrdy_txkl(&self) -> RxrdyTxklR {
        RxrdyTxklR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enabled"]
    #[inline(always)]
    pub fn tx_complt(&self) -> TxCompltR {
        TxCompltR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX Packet Ready Interrupt Enabled"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Received SETUP Interrupt Enabled"]
    #[inline(always)]
    pub fn rx_setup(&self) -> RxSetupR {
        RxSetupR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Enabled"]
    #[inline(always)]
    pub fn stall_snt(&self) -> StallSntR {
        StallSntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Enabled"]
    #[inline(always)]
    pub fn nak_in(&self) -> NakInR {
        NakInR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Enabled"]
    #[inline(always)]
    pub fn nak_out(&self) -> NakOutR {
        NakOutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enabled"]
    #[inline(always)]
    pub fn busy_bank(&self) -> BusyBankR {
        BusyBankR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Enabled"]
    #[inline(always)]
    pub fn shrt_pckt(&self) -> ShrtPcktR {
        ShrtPcktR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UDPHS Endpoint Control Register (endpoint = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptctl3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptctl3Spec;
impl crate::RegisterSpec for Eptctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eptctl3::R`](R) reader structure"]
impl crate::Readable for Eptctl3Spec {}
#[doc = "`reset()` method sets EPTCTL3 to value 0"]
impl crate::Resettable for Eptctl3Spec {
    const RESET_VALUE: u32 = 0;
}
