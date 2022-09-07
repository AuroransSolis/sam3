#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDTIC` writer - ID Transition Interrupt Clear"]
pub type IDTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `VBUSTIC` writer - VBus Transition Interrupt Clear"]
pub type VBUSTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SRPIC` writer - SRP Interrupt Clear"]
pub type SRPIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `VBERRIC` writer - VBus Error Interrupt Clear"]
pub type VBERRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `BCERRIC` writer - B-Connection Error Interrupt Clear"]
pub type BCERRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `ROLEEXIC` writer - Role Exchange Interrupt Clear"]
pub type ROLEEXIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `HNPERRIC` writer - HNP Error Interrupt Clear"]
pub type HNPERRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `STOIC` writer - Suspend Time-Out Interrupt Clear"]
pub type STOIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `VBUSRQC` writer - VBus Request Clear"]
pub type VBUSRQC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Clear"]
    #[inline(always)]
    pub fn idtic(&mut self) -> IDTIC_W<0> {
        IDTIC_W::new(self)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Clear"]
    #[inline(always)]
    pub fn vbustic(&mut self) -> VBUSTIC_W<1> {
        VBUSTIC_W::new(self)
    }
    #[doc = "Bit 2 - SRP Interrupt Clear"]
    #[inline(always)]
    pub fn srpic(&mut self) -> SRPIC_W<2> {
        SRPIC_W::new(self)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Clear"]
    #[inline(always)]
    pub fn vberric(&mut self) -> VBERRIC_W<3> {
        VBERRIC_W::new(self)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Clear"]
    #[inline(always)]
    pub fn bcerric(&mut self) -> BCERRIC_W<4> {
        BCERRIC_W::new(self)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Clear"]
    #[inline(always)]
    pub fn roleexic(&mut self) -> ROLEEXIC_W<5> {
        ROLEEXIC_W::new(self)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Clear"]
    #[inline(always)]
    pub fn hnperric(&mut self) -> HNPERRIC_W<6> {
        HNPERRIC_W::new(self)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn stoic(&mut self) -> STOIC_W<7> {
        STOIC_W::new(self)
    }
    #[doc = "Bit 9 - VBus Request Clear"]
    #[inline(always)]
    pub fn vbusrqc(&mut self) -> VBUSRQC_W<9> {
        VBUSRQC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
