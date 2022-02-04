#[doc = "Register `PMC_IER` writer"]
pub struct W(crate::W<PMC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_IER_SPEC>;
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
impl From<crate::W<PMC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Enable"]
pub struct MOSCXTS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCXTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Enable"]
pub struct LOCKA_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LOCKB` writer - PLLB Lock Interrupt Enable"]
pub struct LOCKB_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Enable"]
pub struct MCKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Enable"]
pub struct PCKRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Enable"]
pub struct PCKRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Enable"]
pub struct PCKRDY2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKRDY2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `MOSCSELS` writer - Main Oscillator Selection Status Interrupt Enable"]
pub struct MOSCSELS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCSELS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `MOSCRCS` writer - Main On-Chip RC Status Interrupt Enable"]
pub struct MOSCRCS_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSCRCS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Enable"]
pub struct CFDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Enable"]
    #[inline(always)]
    pub fn moscxts(&mut self) -> MOSCXTS_W {
        MOSCXTS_W { w: self }
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Enable"]
    #[inline(always)]
    pub fn locka(&mut self) -> LOCKA_W {
        LOCKA_W { w: self }
    }
    #[doc = "Bit 2 - PLLB Lock Interrupt Enable"]
    #[inline(always)]
    pub fn lockb(&mut self) -> LOCKB_W {
        LOCKB_W { w: self }
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Enable"]
    #[inline(always)]
    pub fn mckrdy(&mut self) -> MCKRDY_W {
        MCKRDY_W { w: self }
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy0(&mut self) -> PCKRDY0_W {
        PCKRDY0_W { w: self }
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy1(&mut self) -> PCKRDY1_W {
        PCKRDY1_W { w: self }
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pckrdy2(&mut self) -> PCKRDY2_W {
        PCKRDY2_W { w: self }
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status Interrupt Enable"]
    #[inline(always)]
    pub fn moscsels(&mut self) -> MOSCSELS_W {
        MOSCSELS_W { w: self }
    }
    #[doc = "Bit 17 - Main On-Chip RC Status Interrupt Enable"]
    #[inline(always)]
    pub fn moscrcs(&mut self) -> MOSCRCS_W {
        MOSCRCS_W { w: self }
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Enable"]
    #[inline(always)]
    pub fn cfdev(&mut self) -> CFDEV_W {
        CFDEV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_ier](index.html) module"]
pub struct PMC_IER_SPEC;
impl crate::RegisterSpec for PMC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_ier::W](W) writer structure"]
impl crate::Writable for PMC_IER_SPEC {
    type Writer = W;
}
