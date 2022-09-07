#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXSUSP` writer - Clear UDP Suspend Interrupt"]
pub type RXSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXRSM` writer - Clear UDP Resume Interrupt"]
pub type RXRSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `EXTRSM` writer - "]
pub type EXTRSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `SOFINT` writer - Clear Start Of Frame Interrupt"]
pub type SOFINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `ENDBUSRES` writer - Clear End of Bus Reset Interrupt"]
pub type ENDBUSRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `WAKEUP` writer - Clear Wakeup Interrupt"]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 8 - Clear UDP Suspend Interrupt"]
    #[inline(always)]
    pub fn rxsusp(&mut self) -> RXSUSP_W<8> {
        RXSUSP_W::new(self)
    }
    #[doc = "Bit 9 - Clear UDP Resume Interrupt"]
    #[inline(always)]
    pub fn rxrsm(&mut self) -> RXRSM_W<9> {
        RXRSM_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&mut self) -> EXTRSM_W<10> {
        EXTRSM_W::new(self)
    }
    #[doc = "Bit 11 - Clear Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn sofint(&mut self) -> SOFINT_W<11> {
        SOFINT_W::new(self)
    }
    #[doc = "Bit 12 - Clear End of Bus Reset Interrupt"]
    #[inline(always)]
    pub fn endbusres(&mut self) -> ENDBUSRES_W<12> {
        ENDBUSRES_W::new(self)
    }
    #[doc = "Bit 13 - Clear Wakeup Interrupt"]
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
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
