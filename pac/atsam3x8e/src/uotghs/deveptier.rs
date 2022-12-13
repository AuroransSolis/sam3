#[doc = "Register `DEVEPTIER[%s]` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<DEVEPTIER_SPEC>);
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TXINES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RXOUTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `RXSTPES` writer - Received SETUP Interrupt Enable"]
pub type RXSTPES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `NAKOUTES` writer - NAKed OUT Interrupt Enable"]
pub type NAKOUTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `NAKINES` writer - NAKed IN Interrupt Enable"]
pub type NAKINES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OVERFES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `STALLEDES` writer - STALLed Interrupt Enable"]
pub type STALLEDES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NBUSYBKES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KILLBKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FIFOCONS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EPDISHDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `NYETDISS` writer - NYET Token Disable Enable"]
pub type NYETDISS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
#[doc = "Field `STALLRQS` writer - STALL Request Enable"]
pub type STALLRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIER_SPEC, bool, O>;
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
    #[doc = "Bit 2 - Received SETUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpes(&mut self) -> RXSTPES_W<2> {
        RXSTPES_W::new(self)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutes(&mut self) -> NAKOUTES_W<3> {
        NAKOUTES_W::new(self)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakines(&mut self) -> NAKINES_W<4> {
        NAKINES_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfes(&mut self) -> OVERFES_W<5> {
        OVERFES_W::new(self)
    }
    #[doc = "Bit 6 - STALLed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stalledes(&mut self) -> STALLEDES_W<6> {
        STALLEDES_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketes(&mut self) -> SHORTPACKETES_W<7> {
        SHORTPACKETES_W::new(self)
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
    #[doc = "Bit 17 - NYET Token Disable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdiss(&mut self) -> NYETDISS_W<17> {
        NYETDISS_W::new(self)
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
#[doc = "Device Endpoint Enable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptier](index.html) module"]
pub struct DEVEPTIER_SPEC;
impl crate::RegisterSpec for DEVEPTIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [deveptier::W](W) writer structure"]
impl crate::Writable for DEVEPTIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
