#[doc = "Register `HSTPIPIFR7` writer"]
pub type W = crate::W<HSTPIPIFR7_SPEC>;
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RXINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TXOUTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTPIS` writer - Transmitted SETUP Interrupt Set"]
pub type TXSTPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NAKEDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDIS` writer - Received STALLed Interrupt Set"]
pub type RXSTALLDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NBUSYBKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxinis(&mut self) -> RXINIS_W<HSTPIPIFR7_SPEC> {
        RXINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutis(&mut self) -> TXOUTIS_W<HSTPIPIFR7_SPEC> {
        TXOUTIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txstpis(&mut self) -> TXSTPIS_W<HSTPIPIFR7_SPEC> {
        TXSTPIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn perris(&mut self) -> PERRIS_W<HSTPIPIFR7_SPEC> {
        PERRIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedis(&mut self) -> NAKEDIS_W<HSTPIPIFR7_SPEC> {
        NAKEDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<HSTPIPIFR7_SPEC> {
        OVERFIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldis(&mut self) -> RXSTALLDIS_W<HSTPIPIFR7_SPEC> {
        RXSTALLDIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketis(&mut self) -> SHORTPACKETIS_W<HSTPIPIFR7_SPEC> {
        SHORTPACKETIS_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<HSTPIPIFR7_SPEC> {
        NBUSYBKS_W::new(self, 12)
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
#[doc = "Host Pipe Set Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPIFR7_SPEC;
impl crate::RegisterSpec for HSTPIPIFR7_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipifr7::W`](W) writer structure"]
impl crate::Writable for HSTPIPIFR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
