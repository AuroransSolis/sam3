#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Enable"]
pub type TXRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` writer - End of Conversion Interrupt Enable"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Enable"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IER_SPEC> {
        TXRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<IER_SPEC> {
        EOC_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IER_SPEC> {
        ENDTX_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<IER_SPEC> {
        TXBUFE_W::new(self, 3)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
