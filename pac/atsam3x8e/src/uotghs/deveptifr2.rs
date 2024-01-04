#[doc = "Register `DEVEPTIFR2` writer"]
pub type W = crate::W<DEVEPTIFR2_SPEC>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TXINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RXOUTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPIS` writer - Received SETUP Interrupt Set"]
pub type RXSTPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTIS` writer - NAKed OUT Interrupt Set"]
pub type NAKOUTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINIS` writer - NAKed IN Interrupt Set"]
pub type NAKINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDIS` writer - STALLed Interrupt Set"]
pub type STALLEDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NBUSYBKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TXINIS_W<DEVEPTIFR2_SPEC> {
        TXINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RXOUTIS_W<DEVEPTIFR2_SPEC> {
        RXOUTIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpis(&mut self) -> RXSTPIS_W<DEVEPTIFR2_SPEC> {
        RXSTPIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutis(&mut self) -> NAKOUTIS_W<DEVEPTIFR2_SPEC> {
        NAKOUTIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakinis(&mut self) -> NAKINIS_W<DEVEPTIFR2_SPEC> {
        NAKINIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<DEVEPTIFR2_SPEC> {
        OVERFIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn stalledis(&mut self) -> STALLEDIS_W<DEVEPTIFR2_SPEC> {
        STALLEDIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpackets(&mut self) -> SHORTPACKETS_W<DEVEPTIFR2_SPEC> {
        SHORTPACKETS_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<DEVEPTIFR2_SPEC> {
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
#[doc = "Device Endpoint Set Register (n = 0) 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTIFR2_SPEC;
impl crate::RegisterSpec for DEVEPTIFR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptifr2::W`](W) writer structure"]
impl crate::Writable for DEVEPTIFR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
