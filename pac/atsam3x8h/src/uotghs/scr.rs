#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `IDTIC` writer - ID Transition Interrupt Clear"]
pub type IDTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBUSTIC` writer - VBus Transition Interrupt Clear"]
pub type VBUSTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRPIC` writer - SRP Interrupt Clear"]
pub type SRPIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBERRIC` writer - VBus Error Interrupt Clear"]
pub type VBERRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BCERRIC` writer - B-Connection Error Interrupt Clear"]
pub type BCERRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROLEEXIC` writer - Role Exchange Interrupt Clear"]
pub type ROLEEXIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNPERRIC` writer - HNP Error Interrupt Clear"]
pub type HNPERRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOIC` writer - Suspend Time-Out Interrupt Clear"]
pub type STOIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBUSRQC` writer - VBus Request Clear"]
pub type VBUSRQC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn idtic(&mut self) -> IDTIC_W<SCR_SPEC, 0> {
        IDTIC_W::new(self)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbustic(&mut self) -> VBUSTIC_W<SCR_SPEC, 1> {
        VBUSTIC_W::new(self)
    }
    #[doc = "Bit 2 - SRP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn srpic(&mut self) -> SRPIC_W<SCR_SPEC, 2> {
        SRPIC_W::new(self)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vberric(&mut self) -> VBERRIC_W<SCR_SPEC, 3> {
        VBERRIC_W::new(self)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bcerric(&mut self) -> BCERRIC_W<SCR_SPEC, 4> {
        BCERRIC_W::new(self)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn roleexic(&mut self) -> ROLEEXIC_W<SCR_SPEC, 5> {
        ROLEEXIC_W::new(self)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hnperric(&mut self) -> HNPERRIC_W<SCR_SPEC, 6> {
        HNPERRIC_W::new(self)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stoic(&mut self) -> STOIC_W<SCR_SPEC, 7> {
        STOIC_W::new(self)
    }
    #[doc = "Bit 9 - VBus Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqc(&mut self) -> VBUSRQC_W<SCR_SPEC, 9> {
        VBUSRQC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "General Status Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
