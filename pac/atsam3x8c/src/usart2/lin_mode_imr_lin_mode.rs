#[doc = "Register `IMR_LIN_MODE` reader"]
pub type R = crate::R<LIN_MODE_IMR_LIN_MODE_SPEC>;
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `ENDRX` reader - "]
pub type ENDRX_R = crate::BitReader;
#[doc = "Field `ENDTX` reader - "]
pub type ENDTX_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error Interrupt Mask"]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error Interrupt Mask"]
pub type PARE_R = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Time-out Interrupt Mask"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `TXBUFE` reader - "]
pub type TXBUFE_R = crate::BitReader;
#[doc = "Field `RXBUFF` reader - "]
pub type RXBUFF_R = crate::BitReader;
#[doc = "Field `LINBK` reader - LIN Break Sent or LIN Break Received Interrupt Mask"]
pub type LINBK_R = crate::BitReader;
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Identifier Received Interrupt Mask"]
pub type LINID_R = crate::BitReader;
#[doc = "Field `LINTC` reader - LIN Transfer Completed Interrupt Mask"]
pub type LINTC_R = crate::BitReader;
#[doc = "Field `LINBE` reader - LIN Bus Error Interrupt Mask"]
pub type LINBE_R = crate::BitReader;
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error Interrupt Mask"]
pub type LINISFE_R = crate::BitReader;
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Interrupt Mask"]
pub type LINIPE_R = crate::BitReader;
#[doc = "Field `LINCE` reader - LIN Checksum Error Interrupt Mask"]
pub type LINCE_R = crate::BitReader;
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error Interrupt Mask"]
pub type LINSNRE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
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
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time-out Interrupt Mask"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
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
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Mask"]
    #[inline(always)]
    pub fn linbk(&self) -> LINBK_R {
        LINBK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Mask"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R {
        LINID_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R {
        LINTC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R {
        LINBE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Mask"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R {
        LINISFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Mask"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R {
        LINIPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Mask"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R {
        LINCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Mask"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R {
        LINSNRE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_mode_imr_lin_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIN_MODE_IMR_LIN_MODE_SPEC;
impl crate::RegisterSpec for LIN_MODE_IMR_LIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lin_mode_imr_lin_mode::R`](R) reader structure"]
impl crate::Readable for LIN_MODE_IMR_LIN_MODE_SPEC {}
#[doc = "`reset()` method sets IMR_LIN_MODE to value 0"]
impl crate::Resettable for LIN_MODE_IMR_LIN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
