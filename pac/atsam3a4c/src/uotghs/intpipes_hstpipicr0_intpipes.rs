#[doc = "Register `HSTPIPICR0_INTPIPES` writer"]
pub type W = crate::W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC>;
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RXINIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TXOUTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UNDERFIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NAKEDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDIC` writer - Received STALLed Interrupt Clear"]
pub type RXSTALLDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxinic(&mut self) -> RXINIC_W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        RXINIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutic(&mut self) -> TXOUTIC_W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        TXOUTIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfic(&mut self) -> UNDERFIC_W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        UNDERFIC_W::new(self, 2)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedic(&mut self) -> NAKEDIC_W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        NAKEDIC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OVERFIC_W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        OVERFIC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldic(&mut self) -> RXSTALLDIC_W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        RXSTALLDIC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketic(&mut self) -> SHORTPACKETIC_W<INTPIPES_HSTPIPICR0_INTPIPES_SPEC> {
        SHORTPACKETIC_W::new(self, 7)
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
#[doc = "Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipicr0_intpipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTPIPES_HSTPIPICR0_INTPIPES_SPEC;
impl crate::RegisterSpec for INTPIPES_HSTPIPICR0_INTPIPES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intpipes_hstpipicr0_intpipes::W`](W) writer structure"]
impl crate::Writable for INTPIPES_HSTPIPICR0_INTPIPES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
