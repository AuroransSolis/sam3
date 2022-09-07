#[doc = "Register `CSR_LIN_MODE` reader"]
pub struct R(crate::R<LIN_MODE_CSR_LIN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIN_MODE_CSR_LIN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIN_MODE_CSR_LIN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIN_MODE_CSR_LIN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `ENDRX` reader - "]
pub type ENDRX_R = crate::BitReader<bool>;
#[doc = "Field `ENDTX` reader - "]
pub type ENDTX_R = crate::BitReader<bool>;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OVRE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME` reader - Framing Error"]
pub type FRAME_R = crate::BitReader<bool>;
#[doc = "Field `PARE` reader - Parity Error"]
pub type PARE_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` reader - Receiver Time-out"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TXBUFE` reader - "]
pub type TXBUFE_R = crate::BitReader<bool>;
#[doc = "Field `RXBUFF` reader - "]
pub type RXBUFF_R = crate::BitReader<bool>;
#[doc = "Field `LINBK` reader - LIN Break Sent or LIN Break Received"]
pub type LINBK_R = crate::BitReader<bool>;
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Identifier Received"]
pub type LINID_R = crate::BitReader<bool>;
#[doc = "Field `LINTC` reader - LIN Transfer Completed"]
pub type LINTC_R = crate::BitReader<bool>;
#[doc = "Field `LINBLS` reader - LIN Bus Line Status"]
pub type LINBLS_R = crate::BitReader<bool>;
#[doc = "Field `LINBE` reader - LIN Bit Error"]
pub type LINBE_R = crate::BitReader<bool>;
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error"]
pub type LINISFE_R = crate::BitReader<bool>;
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Error"]
pub type LINIPE_R = crate::BitReader<bool>;
#[doc = "Field `LINCE` reader - LIN Checksum Error"]
pub type LINCE_R = crate::BitReader<bool>;
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error"]
pub type LINSNRE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receiver Time-out"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received"]
    #[inline(always)]
    pub fn linbk(&self) -> LINBK_R {
        LINBK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R {
        LINID_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Completed"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R {
        LINTC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - LIN Bus Line Status"]
    #[inline(always)]
    pub fn linbls(&self) -> LINBLS_R {
        LINBLS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Bit Error"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R {
        LINBE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R {
        LINISFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Error"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R {
        LINIPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R {
        LINCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R {
        LINSNRE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lin_mode_csr_lin_mode](index.html) module"]
pub struct LIN_MODE_CSR_LIN_MODE_SPEC;
impl crate::RegisterSpec for LIN_MODE_CSR_LIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lin_mode_csr_lin_mode::R](R) reader structure"]
impl crate::Readable for LIN_MODE_CSR_LIN_MODE_SPEC {
    type Reader = R;
}
