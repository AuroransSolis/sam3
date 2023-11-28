#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Enable"]
pub type RDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - SPI Transmit Data Register Empty Interrupt Enable"]
pub type TDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` writer - Mode Fault Error Interrupt Enable"]
pub type MODF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Enable"]
pub type OVRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSR` writer - NSS Rising Interrupt Enable"]
pub type NSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Enable"]
pub type TXEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDES` writer - Underrun Error Interrupt Enable"]
pub type UNDES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<IER_SPEC> {
        RDRF_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<IER_SPEC> {
        TDRE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> MODF_W<IER_SPEC> {
        MODF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovres(&mut self) -> OVRES_W<IER_SPEC> {
        OVRES_W::new(self, 3)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nssr(&mut self) -> NSSR_W<IER_SPEC> {
        NSSR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<IER_SPEC> {
        TXEMPTY_W::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn undes(&mut self) -> UNDES_W<IER_SPEC> {
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
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
