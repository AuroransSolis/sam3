#[doc = "Register `HSTPIPIFR3` writer"]
pub type W = crate::W<HSTPIPIFR3_SPEC>;
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RXINIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TXOUTIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXSTPIS` writer - Transmitted SETUP Interrupt Set"]
pub type TXSTPIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PERRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NAKEDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTALLDIS` writer - Received STALLed Interrupt Set"]
pub type RXSTALLDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NBUSYBKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxinis(&mut self) -> RXINIS_W<HSTPIPIFR3_SPEC, 0> {
        RXINIS_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutis(&mut self) -> TXOUTIS_W<HSTPIPIFR3_SPEC, 1> {
        TXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txstpis(&mut self) -> TXSTPIS_W<HSTPIPIFR3_SPEC, 2> {
        TXSTPIS_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn perris(&mut self) -> PERRIS_W<HSTPIPIFR3_SPEC, 3> {
        PERRIS_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedis(&mut self) -> NAKEDIS_W<HSTPIPIFR3_SPEC, 4> {
        NAKEDIS_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<HSTPIPIFR3_SPEC, 5> {
        OVERFIS_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldis(&mut self) -> RXSTALLDIS_W<HSTPIPIFR3_SPEC, 6> {
        RXSTALLDIS_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketis(&mut self) -> SHORTPACKETIS_W<HSTPIPIFR3_SPEC, 7> {
        SHORTPACKETIS_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<HSTPIPIFR3_SPEC, 12> {
        NBUSYBKS_W::new(self)
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
#[doc = "Host Pipe Set Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPIFR3_SPEC;
impl crate::RegisterSpec for HSTPIPIFR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipifr3::W`](W) writer structure"]
impl crate::Writable for HSTPIPIFR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
