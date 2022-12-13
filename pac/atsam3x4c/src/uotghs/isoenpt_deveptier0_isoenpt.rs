#[doc = "Register `DEVEPTIER0_ISOENPT` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC>);
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TXINES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RXOUTES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `UNDERFES` writer - Underflow Interrupt Enable"]
pub type UNDERFES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `HBISOINERRES` writer - High Bandwidth Isochronous IN Error Interrupt Enable"]
pub type HBISOINERRES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `HBISOFLUSHES` writer - High Bandwidth Isochronous IN Flush Interrupt Enable"]
pub type HBISOFLUSHES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OVERFES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `CRCERRES` writer - CRC Error Interrupt Enable"]
pub type CRCERRES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `MDATAES` writer - MData Interrupt Enable"]
pub type MDATAES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `DATAXES` writer - DataX Interrupt Enable"]
pub type DATAXES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `ERRORTRANSES` writer - Transaction Error Interrupt Enable"]
pub type ERRORTRANSES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NBUSYBKES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KILLBKS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FIFOCONS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EPDISHDMAS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
#[doc = "Field `STALLRQS` writer - STALL Request Enable"]
pub type STALLRQS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENPT_DEVEPTIER0_ISOENPT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txines(&mut self) -> TXINES_W<0> {
        TXINES_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutes(&mut self) -> RXOUTES_W<1> {
        RXOUTES_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underfes(&mut self) -> UNDERFES_W<2> {
        UNDERFES_W::new(self)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerres(&mut self) -> HBISOINERRES_W<3> {
        HBISOINERRES_W::new(self)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushes(&mut self) -> HBISOFLUSHES_W<4> {
        HBISOFLUSHES_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfes(&mut self) -> OVERFES_W<5> {
        OVERFES_W::new(self)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcerres(&mut self) -> CRCERRES_W<6> {
        CRCERRES_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketes(&mut self) -> SHORTPACKETES_W<7> {
        SHORTPACKETES_W::new(self)
    }
    #[doc = "Bit 8 - MData Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdataes(&mut self) -> MDATAES_W<8> {
        MDATAES_W::new(self)
    }
    #[doc = "Bit 9 - DataX Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataxes(&mut self) -> DATAXES_W<9> {
        DATAXES_W::new(self)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errortranses(&mut self) -> ERRORTRANSES_W<10> {
        ERRORTRANSES_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<12> {
        NBUSYBKES_W::new(self)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    #[must_use]
    pub fn killbks(&mut self) -> KILLBKS_W<13> {
        KILLBKS_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    #[must_use]
    pub fn fifocons(&mut self) -> FIFOCONS_W<14> {
        FIFOCONS_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmas(&mut self) -> EPDISHDMAS_W<16> {
        EPDISHDMAS_W::new(self)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<18> {
        RSTDTS_W::new(self)
    }
    #[doc = "Bit 19 - STALL Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqs(&mut self) -> STALLRQS_W<19> {
        STALLRQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Enable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoenpt_deveptier0_isoenpt](index.html) module"]
pub struct ISOENPT_DEVEPTIER0_ISOENPT_SPEC;
impl crate::RegisterSpec for ISOENPT_DEVEPTIER0_ISOENPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isoenpt_deveptier0_isoenpt::W](W) writer structure"]
impl crate::Writable for ISOENPT_DEVEPTIER0_ISOENPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
