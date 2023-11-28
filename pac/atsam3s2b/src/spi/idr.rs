#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Disable"]
pub type RDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - SPI Transmit Data Register Empty Interrupt Disable"]
pub type TDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` writer - Mode Fault Error Interrupt Disable"]
pub type MODF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Disable"]
pub type OVRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Disable"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Disable"]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSR` writer - NSS Rising Interrupt Disable"]
pub type NSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Disable"]
pub type TXEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDES` writer - Underrun Error Interrupt Disable"]
pub type UNDES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<IDR_SPEC> {
        RDRF_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<IDR_SPEC> {
        TDRE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> MODF_W<IDR_SPEC> {
        MODF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovres(&mut self) -> OVRES_W<IDR_SPEC> {
        OVRES_W::new(self, 3)
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IDR_SPEC> {
        ENDRX_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IDR_SPEC> {
        ENDTX_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IDR_SPEC> {
        RXBUFF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<IDR_SPEC> {
        TXBUFE_W::new(self, 7)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nssr(&mut self) -> NSSR_W<IDR_SPEC> {
        NSSR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<IDR_SPEC> {
        TXEMPTY_W::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn undes(&mut self) -> UNDES_W<IDR_SPEC> {
        UNDES_W::new(self, 10)
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
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
