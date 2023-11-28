#[doc = "Register `EPTCTLENB4` writer"]
pub type W = crate::W<EPTCTLENB4_SPEC>;
#[doc = "Field `EPT_ENABL` writer - Endpoint Enable"]
pub type EPT_ENABL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Enable"]
pub type AUTO_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type INTDIS_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET_DIS` writer - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
pub type NYET_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Enable"]
pub type ERR_OVFLW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Enable"]
pub type RXRDY_TXKL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Enable"]
pub type TX_COMPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Interrupt Enable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SETUP` writer - Received SETUP"]
pub type RX_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Interrupt Enable"]
pub type STALL_SNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_IN` writer - NAKIN Interrupt Enable"]
pub type NAK_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Interrupt Enable"]
pub type NAK_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Enable"]
pub type BUSY_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Send/Short Packet Interrupt Enable"]
pub type SHRT_PCKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_enabl(&mut self) -> EPT_ENABL_W<EPTCTLENB4_SPEC> {
        EPT_ENABL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AUTO_VALID_W<EPTCTLENB4_SPEC> {
        AUTO_VALID_W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> INTDIS_DMA_W<EPTCTLENB4_SPEC> {
        INTDIS_DMA_W::new(self, 3)
    }
    #[doc = "Bit 4 - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_dis(&mut self) -> NYET_DIS_W<EPTCTLENB4_SPEC> {
        NYET_DIS_W::new(self, 4)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ERR_OVFLW_W<EPTCTLENB4_SPEC> {
        ERR_OVFLW_W::new(self, 8)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<EPTCTLENB4_SPEC> {
        RXRDY_TXKL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<EPTCTLENB4_SPEC> {
        TX_COMPLT_W::new(self, 10)
    }
    #[doc = "Bit 11 - TX Packet Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<EPTCTLENB4_SPEC> {
        TXRDY_W::new(self, 11)
    }
    #[doc = "Bit 12 - Received SETUP"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RX_SETUP_W<EPTCTLENB4_SPEC> {
        RX_SETUP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> STALL_SNT_W<EPTCTLENB4_SPEC> {
        STALL_SNT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NAK_IN_W<EPTCTLENB4_SPEC> {
        NAK_IN_W::new(self, 14)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NAK_OUT_W<EPTCTLENB4_SPEC> {
        NAK_OUT_W::new(self, 15)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BUSY_BANK_W<EPTCTLENB4_SPEC> {
        BUSY_BANK_W::new(self, 18)
    }
    #[doc = "Bit 31 - Short Packet Send/Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> SHRT_PCKT_W<EPTCTLENB4_SPEC> {
        SHRT_PCKT_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 4)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptctlenb4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPTCTLENB4_SPEC;
impl crate::RegisterSpec for EPTCTLENB4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptctlenb4::W`](W) writer structure"]
impl crate::Writable for EPTCTLENB4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
