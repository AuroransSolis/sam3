#[doc = "Register `DEVEPTIDR0` writer"]
pub type W = crate::W<DEVEPTIDR0_SPEC>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TXINEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTPEC` writer - Received SETUP Interrupt Clear"]
pub type RXSTPEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKOUTEC` writer - NAKed OUT Interrupt Clear"]
pub type NAKOUTEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKINEC` writer - NAKed IN Interrupt Clear"]
pub type NAKINEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OVERFEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLEDEC` writer - STALLed Interrupt Clear"]
pub type STALLEDEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type SHORTPACKETEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NBUSYBKEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FIFOCONC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EPDISHDMAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYETDISC` writer - NYET Token Disable Clear"]
pub type NYETDISC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLRQC` writer - STALL Request Clear"]
pub type STALLRQC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TXINEC_W<DEVEPTIDR0_SPEC, 0> {
        TXINEC_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RXOUTEC_W<DEVEPTIDR0_SPEC, 1> {
        RXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpec(&mut self) -> RXSTPEC_W<DEVEPTIDR0_SPEC, 2> {
        RXSTPEC_W::new(self)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutec(&mut self) -> NAKOUTEC_W<DEVEPTIDR0_SPEC, 3> {
        NAKOUTEC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinec(&mut self) -> NAKINEC_W<DEVEPTIDR0_SPEC, 4> {
        NAKINEC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OVERFEC_W<DEVEPTIDR0_SPEC, 5> {
        OVERFEC_W::new(self)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledec(&mut self) -> STALLEDEC_W<DEVEPTIDR0_SPEC, 6> {
        STALLEDEC_W::new(self)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> SHORTPACKETEC_W<DEVEPTIDR0_SPEC, 7> {
        SHORTPACKETEC_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<DEVEPTIDR0_SPEC, 12> {
        NBUSYBKEC_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<DEVEPTIDR0_SPEC, 14> {
        FIFOCONC_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EPDISHDMAC_W<DEVEPTIDR0_SPEC, 16> {
        EPDISHDMAC_W::new(self)
    }
    #[doc = "Bit 17 - NYET Token Disable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdisc(&mut self) -> NYETDISC_W<DEVEPTIDR0_SPEC, 17> {
        NYETDISC_W::new(self)
    }
    #[doc = "Bit 19 - STALL Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqc(&mut self) -> STALLRQC_W<DEVEPTIDR0_SPEC, 19> {
        STALLRQC_W::new(self)
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
#[doc = "Device Endpoint Disable Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTIDR0_SPEC;
impl crate::RegisterSpec for DEVEPTIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptidr0::W`](W) writer structure"]
impl crate::Writable for DEVEPTIDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
