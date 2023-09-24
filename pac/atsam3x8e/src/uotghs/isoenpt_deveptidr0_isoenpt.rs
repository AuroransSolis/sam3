#[doc = "Register `DEVEPTIDR0_ISOENPT` writer"]
pub type W = crate::W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TXINEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNDERFEC` writer - Underflow Interrupt Clear"]
pub type UNDERFEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOINERREC` writer - High Bandwidth Isochronous IN Error Interrupt Clear"]
pub type HBISOINERREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOFLUSHEC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HBISOFLUSHEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OVERFEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCERREC` writer - CRC Error Interrupt Clear"]
pub type CRCERREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type SHORTPACKETEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MDATEC` writer - MData Interrupt Clear"]
pub type MDATEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAXEC` writer - DataX Interrupt Clear"]
pub type DATAXEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRORTRANSEC` writer - Transaction Error Interrupt Clear"]
pub type ERRORTRANSEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NBUSYBKEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FIFOCONC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EPDISHDMAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TXINEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 0> {
        TXINEC_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RXOUTEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 1> {
        RXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfec(&mut self) -> UNDERFEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 2> {
        UNDERFEC_W::new(self)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerrec(&mut self) -> HBISOINERREC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 3> {
        HBISOINERREC_W::new(self)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushec(&mut self) -> HBISOFLUSHEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 4> {
        HBISOFLUSHEC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OVERFEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 5> {
        OVERFEC_W::new(self)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcerrec(&mut self) -> CRCERREC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 6> {
        CRCERREC_W::new(self)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> SHORTPACKETEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 7> {
        SHORTPACKETEC_W::new(self)
    }
    #[doc = "Bit 8 - MData Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mdatec(&mut self) -> MDATEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 8> {
        MDATEC_W::new(self)
    }
    #[doc = "Bit 9 - DataX Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dataxec(&mut self) -> DATAXEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 9> {
        DATAXEC_W::new(self)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errortransec(&mut self) -> ERRORTRANSEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 10> {
        ERRORTRANSEC_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 12> {
        NBUSYBKEC_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 14> {
        FIFOCONC_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EPDISHDMAC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC, 16> {
        EPDISHDMAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Endpoint Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptidr0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENPT_DEVEPTIDR0_ISOENPT_SPEC;
impl crate::RegisterSpec for ISOENPT_DEVEPTIDR0_ISOENPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_deveptidr0_isoenpt::W`](W) writer structure"]
impl crate::Writable for ISOENPT_DEVEPTIDR0_ISOENPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
