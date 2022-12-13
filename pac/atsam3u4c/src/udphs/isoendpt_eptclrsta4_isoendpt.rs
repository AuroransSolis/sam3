#[doc = "Register `EPTCLRSTA4_ISOENDPT` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC>);
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub type TOGGLESQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
pub type RXRDY_TXKL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
pub type TX_COMPLT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Clear"]
pub type ERR_FL_ISO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_CRC_NTR` writer - Number of Transaction Error Clear"]
pub type ERR_CRC_NTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Clear"]
pub type ERR_FLUSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC, bool, O>;
impl W {
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
    #[doc = "Bit 12 - Error Flow Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_fl_iso(&mut self) -> ERR_FL_ISO_W<12> {
        ERR_FL_ISO_W::new(self)
    }
    #[doc = "Bit 13 - Number of Transaction Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_crc_ntr(&mut self) -> ERR_CRC_NTR_W<13> {
        ERR_CRC_NTR_W::new(self)
    }
    #[doc = "Bit 14 - Bank Flush Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_flush(&mut self) -> ERR_FLUSH_W<14> {
        ERR_FLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_eptclrsta4_isoendpt](index.html) module"]
pub struct ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isoendpt_eptclrsta4_isoendpt::W](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTCLRSTA4_ISOENDPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
