#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `RXSUSP` writer - Clear UDP Suspend Interrupt"]
pub type RXSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSM` writer - Clear UDP Resume Interrupt"]
pub type RXRSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRSM` writer - "]
pub type EXTRSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFINT` writer - Clear Start Of Frame Interrupt"]
pub type SOFINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDBUSRES` writer - Clear End of Bus Reset Interrupt"]
pub type ENDBUSRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Clear Wakeup Interrupt"]
pub type WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Clear UDP Suspend Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxsusp(&mut self) -> RXSUSP_W<ICR_SPEC> {
        RXSUSP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear UDP Resume Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsm(&mut self) -> RXRSM_W<ICR_SPEC> {
        RXRSM_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn extrsm(&mut self) -> EXTRSM_W<ICR_SPEC> {
        EXTRSM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Start Of Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sofint(&mut self) -> SOFINT_W<ICR_SPEC> {
        SOFINT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear End of Bus Reset Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn endbusres(&mut self) -> ENDBUSRES_W<ICR_SPEC> {
        ENDBUSRES_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Wakeup Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<ICR_SPEC> {
        WAKEUP_W::new(self, 13)
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
#[doc = "Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
