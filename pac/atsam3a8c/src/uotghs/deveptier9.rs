#[doc = "Register `DEVEPTIER9` writer"]
pub type W = crate::W<DEVEPTIER9_SPEC>;
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TXINES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RXOUTES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPES` writer - Received SETUP Interrupt Enable"]
pub type RXSTPES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTES` writer - NAKed OUT Interrupt Enable"]
pub type NAKOUTES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINES` writer - NAKed IN Interrupt Enable"]
pub type NAKINES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OVERFES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDES` writer - STALLed Interrupt Enable"]
pub type STALLEDES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NBUSYBKES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KILLBKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FIFOCONS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EPDISHDMAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETDISS` writer - NYET Token Disable Enable"]
pub type NYETDISS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQS` writer - STALL Request Enable"]
pub type STALLRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txines(&mut self) -> TXINES_W<DEVEPTIER9_SPEC> {
        TXINES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutes(&mut self) -> RXOUTES_W<DEVEPTIER9_SPEC> {
        RXOUTES_W::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpes(&mut self) -> RXSTPES_W<DEVEPTIER9_SPEC> {
        RXSTPES_W::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutes(&mut self) -> NAKOUTES_W<DEVEPTIER9_SPEC> {
        NAKOUTES_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakines(&mut self) -> NAKINES_W<DEVEPTIER9_SPEC> {
        NAKINES_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfes(&mut self) -> OVERFES_W<DEVEPTIER9_SPEC> {
        OVERFES_W::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stalledes(&mut self) -> STALLEDES_W<DEVEPTIER9_SPEC> {
        STALLEDES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketes(&mut self) -> SHORTPACKETES_W<DEVEPTIER9_SPEC> {
        SHORTPACKETES_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<DEVEPTIER9_SPEC> {
        NBUSYBKES_W::new(self, 12)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    #[must_use]
    pub fn killbks(&mut self) -> KILLBKS_W<DEVEPTIER9_SPEC> {
        KILLBKS_W::new(self, 13)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    #[must_use]
    pub fn fifocons(&mut self) -> FIFOCONS_W<DEVEPTIER9_SPEC> {
        FIFOCONS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmas(&mut self) -> EPDISHDMAS_W<DEVEPTIER9_SPEC> {
        EPDISHDMAS_W::new(self, 16)
    }
    #[doc = "Bit 17 - NYET Token Disable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdiss(&mut self) -> NYETDISS_W<DEVEPTIER9_SPEC> {
        NYETDISS_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<DEVEPTIER9_SPEC> {
        RSTDTS_W::new(self, 18)
    }
    #[doc = "Bit 19 - STALL Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqs(&mut self) -> STALLRQS_W<DEVEPTIER9_SPEC> {
        STALLRQS_W::new(self, 19)
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
#[doc = "Device Endpoint Enable Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier9::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTIER9_SPEC;
impl crate::RegisterSpec for DEVEPTIER9_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptier9::W`](W) writer structure"]
impl crate::Writable for DEVEPTIER9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
