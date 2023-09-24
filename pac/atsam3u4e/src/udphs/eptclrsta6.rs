#[doc = "Register `EPTCLRSTA6` writer"]
pub type W = crate::W<EPTCLRSTA6_SPEC>;
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Clear"]
pub type FRCESTALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub type TOGGLESQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
pub type RXRDY_TXKL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
pub type TX_COMPLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_SETUP` writer - Received SETUP Clear"]
pub type RX_SETUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Clear"]
pub type STALL_SNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAK_IN` writer - NAKIN Clear"]
pub type NAK_IN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Clear"]
pub type NAK_OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn frcestall(&mut self) -> FRCESTALL_W<EPTCLRSTA6_SPEC, 5> {
        FRCESTALL_W::new(self)
    }
    #[doc = "Bit 6 - Data Toggle Clear"]
    #[inline(always)]
    #[must_use]
    pub fn togglesq(&mut self) -> TOGGLESQ_W<EPTCLRSTA6_SPEC, 6> {
        TOGGLESQ_W::new(self)
    }
    #[doc = "Bit 9 - Received OUT Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<EPTCLRSTA6_SPEC, 9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<EPTCLRSTA6_SPEC, 10> {
        TX_COMPLT_W::new(self)
    }
    #[doc = "Bit 12 - Received SETUP Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RX_SETUP_W<EPTCLRSTA6_SPEC, 12> {
        RX_SETUP_W::new(self)
    }
    #[doc = "Bit 13 - Stall Sent Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> STALL_SNT_W<EPTCLRSTA6_SPEC, 13> {
        STALL_SNT_W::new(self)
    }
    #[doc = "Bit 14 - NAKIN Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NAK_IN_W<EPTCLRSTA6_SPEC, 14> {
        NAK_IN_W::new(self)
    }
    #[doc = "Bit 15 - NAKOUT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NAK_OUT_W<EPTCLRSTA6_SPEC, 15> {
        NAK_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 6)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eptclrsta6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPTCLRSTA6_SPEC;
impl crate::RegisterSpec for EPTCLRSTA6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptclrsta6::W`](W) writer structure"]
impl crate::Writable for EPTCLRSTA6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
