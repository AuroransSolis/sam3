#[doc = "Register `EPTCTLENB5` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<EPTCTLENB5_SPEC>);
#[doc = "Field `EPT_ENABL` writer - Endpoint Enable"]
pub type EPT_ENABL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Enable"]
pub type AUTO_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type INTDIS_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `NYET_DIS` writer - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
pub type NYET_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Enable"]
pub type ERR_OVFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Enable"]
pub type RXRDY_TXKL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Enable"]
pub type TX_COMPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Interrupt Enable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `RX_SETUP` writer - Received SETUP"]
pub type RX_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Interrupt Enable"]
pub type STALL_SNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `NAK_IN` writer - NAKIN Interrupt Enable"]
pub type NAK_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Interrupt Enable"]
pub type NAK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Enable"]
pub type BUSY_BANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Send/Short Packet Interrupt Enable"]
pub type SHRT_PCKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLENB5_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_enabl(&mut self) -> EPT_ENABL_W<0> {
        EPT_ENABL_W::new(self)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auto_valid(&mut self) -> AUTO_VALID_W<1> {
        AUTO_VALID_W::new(self)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn intdis_dma(&mut self) -> INTDIS_DMA_W<3> {
        INTDIS_DMA_W::new(self)
    }
    #[doc = "Bit 4 - NYET Disable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_dis(&mut self) -> NYET_DIS_W<4> {
        NYET_DIS_W::new(self)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ovflw(&mut self) -> ERR_OVFLW_W<8> {
        ERR_OVFLW_W::new(self)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<10> {
        TX_COMPLT_W::new(self)
    }
    #[doc = "Bit 11 - TX Packet Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<11> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 12 - Received SETUP"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RX_SETUP_W<12> {
        RX_SETUP_W::new(self)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> STALL_SNT_W<13> {
        STALL_SNT_W::new(self)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NAK_IN_W<14> {
        NAK_IN_W::new(self)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NAK_OUT_W<15> {
        NAK_OUT_W::new(self)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn busy_bank(&mut self) -> BUSY_BANK_W<18> {
        BUSY_BANK_W::new(self)
    }
    #[doc = "Bit 31 - Short Packet Send/Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shrt_pckt(&mut self) -> SHRT_PCKT_W<31> {
        SHRT_PCKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Control Enable Register (endpoint = 5)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptctlenb5](index.html) module"]
pub struct EPTCTLENB5_SPEC;
impl crate::RegisterSpec for EPTCTLENB5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eptctlenb5::W](W) writer structure"]
impl crate::Writable for EPTCTLENB5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
