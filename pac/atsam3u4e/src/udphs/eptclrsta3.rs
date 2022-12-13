#[doc = "Register `EPTCLRSTA3` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<EPTCLRSTA3_SPEC>);
#[doc = "Field `FRCESTALL` writer - Stall Handshake Request Clear"]
pub type FRCESTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub type TOGGLESQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
pub type RXRDY_TXKL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
pub type TX_COMPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
#[doc = "Field `RX_SETUP` writer - Received SETUP Clear"]
pub type RX_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
#[doc = "Field `STALL_SNT` writer - Stall Sent Clear"]
pub type STALL_SNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
#[doc = "Field `NAK_IN` writer - NAKIN Clear"]
pub type NAK_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
#[doc = "Field `NAK_OUT` writer - NAKOUT Clear"]
pub type NAK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPTCLRSTA3_SPEC, bool, O>;
impl W {
    #[doc = "Bit 5 - Stall Handshake Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn frcestall(&mut self) -> FRCESTALL_W<5> {
        FRCESTALL_W::new(self)
    }
    #[doc = "Bit 6 - Data Toggle Clear"]
    #[inline(always)]
    #[must_use]
    pub fn togglesq(&mut self) -> TOGGLESQ_W<6> {
        TOGGLESQ_W::new(self)
    }
    #[doc = "Bit 9 - Received OUT Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<9> {
        RXRDY_TXKL_W::new(self)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<10> {
        TX_COMPLT_W::new(self)
    }
    #[doc = "Bit 12 - Received SETUP Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rx_setup(&mut self) -> RX_SETUP_W<12> {
        RX_SETUP_W::new(self)
    }
    #[doc = "Bit 13 - Stall Sent Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stall_snt(&mut self) -> STALL_SNT_W<13> {
        STALL_SNT_W::new(self)
    }
    #[doc = "Bit 14 - NAKIN Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_in(&mut self) -> NAK_IN_W<14> {
        NAK_IN_W::new(self)
    }
    #[doc = "Bit 15 - NAKOUT Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nak_out(&mut self) -> NAK_OUT_W<15> {
        NAK_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptclrsta3](index.html) module"]
pub struct EPTCLRSTA3_SPEC;
impl crate::RegisterSpec for EPTCLRSTA3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eptclrsta3::W](W) writer structure"]
impl crate::Writable for EPTCLRSTA3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
