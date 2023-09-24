#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `EP0INT` writer - Enable Endpoint 0 Interrupt"]
pub type EP0INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP1INT` writer - Enable Endpoint 1 Interrupt"]
pub type EP1INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP2INT` writer - Enable Endpoint 2Interrupt"]
pub type EP2INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP3INT` writer - Enable Endpoint 3 Interrupt"]
pub type EP3INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP4INT` writer - Enable Endpoint 4 Interrupt"]
pub type EP4INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP5INT` writer - Enable Endpoint 5 Interrupt"]
pub type EP5INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP6INT` writer - Enable Endpoint 6 Interrupt"]
pub type EP6INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP7INT` writer - Enable Endpoint 7 Interrupt"]
pub type EP7INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSUSP` writer - Enable UDP Suspend Interrupt"]
pub type RXSUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRSM` writer - Enable UDP Resume Interrupt"]
pub type RXRSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTRSM` writer - "]
pub type EXTRSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFINT` writer - Enable Start Of Frame Interrupt"]
pub type SOFINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUP` writer - Enable UDP bus Wakeup Interrupt"]
pub type WAKEUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Enable Endpoint 0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep0int(&mut self) -> EP0INT_W<IER_SPEC, 0> {
        EP0INT_W::new(self)
    }
    #[doc = "Bit 1 - Enable Endpoint 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep1int(&mut self) -> EP1INT_W<IER_SPEC, 1> {
        EP1INT_W::new(self)
    }
    #[doc = "Bit 2 - Enable Endpoint 2Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep2int(&mut self) -> EP2INT_W<IER_SPEC, 2> {
        EP2INT_W::new(self)
    }
    #[doc = "Bit 3 - Enable Endpoint 3 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep3int(&mut self) -> EP3INT_W<IER_SPEC, 3> {
        EP3INT_W::new(self)
    }
    #[doc = "Bit 4 - Enable Endpoint 4 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep4int(&mut self) -> EP4INT_W<IER_SPEC, 4> {
        EP4INT_W::new(self)
    }
    #[doc = "Bit 5 - Enable Endpoint 5 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep5int(&mut self) -> EP5INT_W<IER_SPEC, 5> {
        EP5INT_W::new(self)
    }
    #[doc = "Bit 6 - Enable Endpoint 6 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep6int(&mut self) -> EP6INT_W<IER_SPEC, 6> {
        EP6INT_W::new(self)
    }
    #[doc = "Bit 7 - Enable Endpoint 7 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep7int(&mut self) -> EP7INT_W<IER_SPEC, 7> {
        EP7INT_W::new(self)
    }
    #[doc = "Bit 8 - Enable UDP Suspend Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxsusp(&mut self) -> RXSUSP_W<IER_SPEC, 8> {
        RXSUSP_W::new(self)
    }
    #[doc = "Bit 9 - Enable UDP Resume Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsm(&mut self) -> RXRSM_W<IER_SPEC, 9> {
        RXRSM_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn extrsm(&mut self) -> EXTRSM_W<IER_SPEC, 10> {
        EXTRSM_W::new(self)
    }
    #[doc = "Bit 11 - Enable Start Of Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sofint(&mut self) -> SOFINT_W<IER_SPEC, 11> {
        SOFINT_W::new(self)
    }
    #[doc = "Bit 13 - Enable UDP bus Wakeup Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<IER_SPEC, 13> {
        WAKEUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
