#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0INT` writer - Enable Endpoint 0 Interrupt"]
pub type EP0INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP1INT` writer - Enable Endpoint 1 Interrupt"]
pub type EP1INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP2INT` writer - Enable Endpoint 2Interrupt"]
pub type EP2INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP3INT` writer - Enable Endpoint 3 Interrupt"]
pub type EP3INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP4INT` writer - Enable Endpoint 4 Interrupt"]
pub type EP4INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP5INT` writer - Enable Endpoint 5 Interrupt"]
pub type EP5INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP6INT` writer - Enable Endpoint 6 Interrupt"]
pub type EP6INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EP7INT` writer - Enable Endpoint 7 Interrupt"]
pub type EP7INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RXSUSP` writer - Enable UDP Suspend Interrupt"]
pub type RXSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RXRSM` writer - Enable UDP Resume Interrupt"]
pub type RXRSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EXTRSM` writer - "]
pub type EXTRSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SOFINT` writer - Enable Start Of Frame Interrupt"]
pub type SOFINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `WAKEUP` writer - Enable UDP bus Wakeup Interrupt"]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Enable Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&mut self) -> EP0INT_W<0> {
        EP0INT_W::new(self)
    }
    #[doc = "Bit 1 - Enable Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&mut self) -> EP1INT_W<1> {
        EP1INT_W::new(self)
    }
    #[doc = "Bit 2 - Enable Endpoint 2Interrupt"]
    #[inline(always)]
    pub fn ep2int(&mut self) -> EP2INT_W<2> {
        EP2INT_W::new(self)
    }
    #[doc = "Bit 3 - Enable Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&mut self) -> EP3INT_W<3> {
        EP3INT_W::new(self)
    }
    #[doc = "Bit 4 - Enable Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&mut self) -> EP4INT_W<4> {
        EP4INT_W::new(self)
    }
    #[doc = "Bit 5 - Enable Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&mut self) -> EP5INT_W<5> {
        EP5INT_W::new(self)
    }
    #[doc = "Bit 6 - Enable Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&mut self) -> EP6INT_W<6> {
        EP6INT_W::new(self)
    }
    #[doc = "Bit 7 - Enable Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&mut self) -> EP7INT_W<7> {
        EP7INT_W::new(self)
    }
    #[doc = "Bit 8 - Enable UDP Suspend Interrupt"]
    #[inline(always)]
    pub fn rxsusp(&mut self) -> RXSUSP_W<8> {
        RXSUSP_W::new(self)
    }
    #[doc = "Bit 9 - Enable UDP Resume Interrupt"]
    #[inline(always)]
    pub fn rxrsm(&mut self) -> RXRSM_W<9> {
        RXRSM_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&mut self) -> EXTRSM_W<10> {
        EXTRSM_W::new(self)
    }
    #[doc = "Bit 11 - Enable Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn sofint(&mut self) -> SOFINT_W<11> {
        SOFINT_W::new(self)
    }
    #[doc = "Bit 13 - Enable UDP bus Wakeup Interrupt"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W<13> {
        WAKEUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
