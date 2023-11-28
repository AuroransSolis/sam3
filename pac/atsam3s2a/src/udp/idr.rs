#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `EP0INT` writer - Disable Endpoint 0 Interrupt"]
pub type EP0INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1INT` writer - Disable Endpoint 1 Interrupt"]
pub type EP1INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2INT` writer - Disable Endpoint 2 Interrupt"]
pub type EP2INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3INT` writer - Disable Endpoint 3 Interrupt"]
pub type EP3INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4INT` writer - Disable Endpoint 4 Interrupt"]
pub type EP4INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5INT` writer - Disable Endpoint 5 Interrupt"]
pub type EP5INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6INT` writer - Disable Endpoint 6 Interrupt"]
pub type EP6INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7INT` writer - Disable Endpoint 7 Interrupt"]
pub type EP7INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSUSP` writer - Disable UDP Suspend Interrupt"]
pub type RXSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSM` writer - Disable UDP Resume Interrupt"]
pub type RXRSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRSM` writer - "]
pub type EXTRSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFINT` writer - Disable Start Of Frame Interrupt"]
pub type SOFINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Disable USB Bus Interrupt"]
pub type WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable Endpoint 0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep0int(&mut self) -> EP0INT_W<IDR_SPEC> {
        EP0INT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Endpoint 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep1int(&mut self) -> EP1INT_W<IDR_SPEC> {
        EP1INT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Endpoint 2 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep2int(&mut self) -> EP2INT_W<IDR_SPEC> {
        EP2INT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Disable Endpoint 3 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep3int(&mut self) -> EP3INT_W<IDR_SPEC> {
        EP3INT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Disable Endpoint 4 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep4int(&mut self) -> EP4INT_W<IDR_SPEC> {
        EP4INT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Endpoint 5 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep5int(&mut self) -> EP5INT_W<IDR_SPEC> {
        EP5INT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Endpoint 6 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep6int(&mut self) -> EP6INT_W<IDR_SPEC> {
        EP6INT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Disable Endpoint 7 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep7int(&mut self) -> EP7INT_W<IDR_SPEC> {
        EP7INT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Disable UDP Suspend Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxsusp(&mut self) -> RXSUSP_W<IDR_SPEC> {
        RXSUSP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Disable UDP Resume Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsm(&mut self) -> RXRSM_W<IDR_SPEC> {
        RXRSM_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn extrsm(&mut self) -> EXTRSM_W<IDR_SPEC> {
        EXTRSM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Disable Start Of Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sofint(&mut self) -> SOFINT_W<IDR_SPEC> {
        SOFINT_W::new(self, 11)
    }
    #[doc = "Bit 13 - Disable USB Bus Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<IDR_SPEC> {
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
