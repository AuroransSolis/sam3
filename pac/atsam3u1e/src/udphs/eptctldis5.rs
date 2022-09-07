#[doc = "Register `EPTCTLDIS5` writer"]
pub struct W(crate::W<EPTCTLDIS5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTCTLDIS5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EPTCTLDIS5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTCTLDIS5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPT_DISABL` writer - Endpoint Disable"]
pub type EPT_DISABL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `AUTO_VALID` writer - Packet Auto-Valid Disable"]
pub type AUTO_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `INTDIS_DMA` writer - Interrupts Disable DMA"]
pub type INTDIS_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `NYET_DIS` writer - NYET Enable (Only for High Speed Bulk OUT endpoints)"]
pub type NYET_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `ERR_OVFLW` writer - Overflow Error Interrupt Disable"]
pub type ERR_OVFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Interrupt Disable"]
pub type RXRDY_TXKL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Interrupt Disable"]
pub type TX_COMPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `TXRDY` writer - TX Packet Ready Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `RX_SETUP` writer - Received SETUP Interrupt Disable"]
pub type RX_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Interrupt Disable"]
pub type STALL_SNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `NAK_IN` writer - NAKIN Interrupt Disable"]
pub type NAK_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Interrupt Disable"]
pub type NAK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `BUSY_BANK` writer - Busy Bank Interrupt Disable"]
pub type BUSY_BANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
#[doc = "Field `SHRT_PCKT` writer - Short Packet Interrupt Disable"]
pub type SHRT_PCKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCTLDIS5_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Endpoint Disable"]
    #[inline(always)]
    pub fn ept_disabl(&mut self) -> EPT_DISABL_W<0> {
        EPT_DISABL_W::new(self)
    }
    #[doc = "Bit 1 - Packet Auto-Valid Disable"]
    #[inline(always)]
    pub fn auto_valid(&mut self) -> AUTO_VALID_W<1> {
        AUTO_VALID_W::new(self)
    }
    #[doc = "Bit 3 - Interrupts Disable DMA"]
    #[inline(always)]
    pub fn intdis_dma(&mut self) -> INTDIS_DMA_W<3> {
        INTDIS_DMA_W::new(self)
    }
    #[doc = "Bit 4 - NYET Enable (Only for High Speed Bulk OUT endpoints)"]
    #[inline(always)]
    pub fn nyet_dis(&mut self) -> NYET_DIS_W<4> {
        NYET_DIS_W::new(self)
    }
    #[doc = "Bit 8 - Overflow Error Interrupt Disable"]
    #[inline(always)]
    pub fn err_ovflw(&mut self) -> ERR_OVFLW_W<8> {
        ERR_OVFLW_W::new(self)
    }
    #[doc = "Bit 9 - Received OUT Data Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Interrupt Disable"]
    #[inline(always)]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<10> {
        TX_COMPLT_W::new(self)
    }
    #[doc = "Bit 11 - TX Packet Ready Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W<11> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 12 - Received SETUP Interrupt Disable"]
    #[inline(always)]
    pub fn rx_setup(&mut self) -> RX_SETUP_W<12> {
        RX_SETUP_W::new(self)
    }
    #[doc = "Bit 13 - Stall Sent Interrupt Disable"]
    #[inline(always)]
    pub fn stall_snt(&mut self) -> STALL_SNT_W<13> {
        STALL_SNT_W::new(self)
    }
    #[doc = "Bit 14 - NAKIN Interrupt Disable"]
    #[inline(always)]
    pub fn nak_in(&mut self) -> NAK_IN_W<14> {
        NAK_IN_W::new(self)
    }
    #[doc = "Bit 15 - NAKOUT Interrupt Disable"]
    #[inline(always)]
    pub fn nak_out(&mut self) -> NAK_OUT_W<15> {
        NAK_OUT_W::new(self)
    }
    #[doc = "Bit 18 - Busy Bank Interrupt Disable"]
    #[inline(always)]
    pub fn busy_bank(&mut self) -> BUSY_BANK_W<18> {
        BUSY_BANK_W::new(self)
    }
    #[doc = "Bit 31 - Short Packet Interrupt Disable"]
    #[inline(always)]
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
#[doc = "UDPHS Endpoint Control Disable Register (endpoint = 5)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptctldis5](index.html) module"]
pub struct EPTCTLDIS5_SPEC;
impl crate::RegisterSpec for EPTCTLDIS5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eptctldis5::W](W) writer structure"]
impl crate::Writable for EPTCTLDIS5_SPEC {
    type Writer = W;
}
