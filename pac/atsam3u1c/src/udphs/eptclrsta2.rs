#[doc = "Register `EPTCLRSTA2` writer"]
pub type W = crate::W<EPTCLRSTA2_SPEC>;
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Clear"]
pub type FRCESTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub type TOGGLESQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
pub type RXRDY_TXKL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
pub type TX_COMPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SETUP` writer - Received SETUP Clear"]
pub type RX_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Clear"]
pub type STALL_SNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_IN` writer - NAKIN Clear"]
pub type NAK_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Clear"]
pub type NAK_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn frcestall(&mut self) -> FRCESTALL_W<EPTCLRSTA2_SPEC> {
        FRCESTALL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Data Toggle Clear"]
    #[inline(always)]
    #[must_use]
    pub fn togglesq(&mut self) -> TOGGLESQ_W<EPTCLRSTA2_SPEC> {
        TOGGLESQ_W::new(self, 6)
    }
    #[doc = "Bit 9 - Received OUT Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<EPTCLRSTA2_SPEC> {
        RXRDY_TXKL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<EPTCLRSTA2_SPEC> {
        TX_COMPLT_W::new(self, 10)
    }
    #[doc = "Bit 12 - Received SETUP Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RX_SETUP_W<EPTCLRSTA2_SPEC> {
        RX_SETUP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Stall Sent Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> STALL_SNT_W<EPTCLRSTA2_SPEC> {
        STALL_SNT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NAKIN Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NAK_IN_W<EPTCLRSTA2_SPEC> {
        NAK_IN_W::new(self, 14)
    }
    #[doc = "Bit 15 - NAKOUT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NAK_OUT_W<EPTCLRSTA2_SPEC> {
        NAK_OUT_W::new(self, 15)
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
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPTCLRSTA2_SPEC;
impl crate::RegisterSpec for EPTCLRSTA2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptclrsta2::W`](W) writer structure"]
impl crate::Writable for EPTCLRSTA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
