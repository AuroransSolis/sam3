#[doc = "Register `DEVEPTIDR1` writer"]
pub type W = crate::W<DEVEPTIDR1_SPEC>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TXINEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPEC` writer - Received SETUP Interrupt Clear"]
pub type RXSTPEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTEC` writer - NAKed OUT Interrupt Clear"]
pub type NAKOUTEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINEC` writer - NAKed IN Interrupt Clear"]
pub type NAKINEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OVERFEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDEC` writer - STALLed Interrupt Clear"]
pub type STALLEDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type SHORTPACKETEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NBUSYBKEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FIFOCONC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EPDISHDMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETDISC` writer - NYET Token Disable Clear"]
pub type NYETDISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQC` writer - STALL Request Clear"]
pub type STALLRQC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TXINEC_W<DEVEPTIDR1_SPEC> {
        TXINEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RXOUTEC_W<DEVEPTIDR1_SPEC> {
        RXOUTEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpec(&mut self) -> RXSTPEC_W<DEVEPTIDR1_SPEC> {
        RXSTPEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutec(&mut self) -> NAKOUTEC_W<DEVEPTIDR1_SPEC> {
        NAKOUTEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinec(&mut self) -> NAKINEC_W<DEVEPTIDR1_SPEC> {
        NAKINEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OVERFEC_W<DEVEPTIDR1_SPEC> {
        OVERFEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledec(&mut self) -> STALLEDEC_W<DEVEPTIDR1_SPEC> {
        STALLEDEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> SHORTPACKETEC_W<DEVEPTIDR1_SPEC> {
        SHORTPACKETEC_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<DEVEPTIDR1_SPEC> {
        NBUSYBKEC_W::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<DEVEPTIDR1_SPEC> {
        FIFOCONC_W::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EPDISHDMAC_W<DEVEPTIDR1_SPEC> {
        EPDISHDMAC_W::new(self, 16)
    }
    #[doc = "Bit 17 - NYET Token Disable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdisc(&mut self) -> NYETDISC_W<DEVEPTIDR1_SPEC> {
        NYETDISC_W::new(self, 17)
    }
    #[doc = "Bit 19 - STALL Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqc(&mut self) -> STALLRQC_W<DEVEPTIDR1_SPEC> {
        STALLRQC_W::new(self, 19)
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
#[doc = "Device Endpoint Disable Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTIDR1_SPEC;
impl crate::RegisterSpec for DEVEPTIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptidr1::W`](W) writer structure"]
impl crate::Writable for DEVEPTIDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
