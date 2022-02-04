#[doc = "Register `CLRINT` writer"]
pub struct W(crate::W<CLRINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRINT_SPEC>;
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
impl From<crate::W<CLRINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DET_SUSPD` writer - Suspend Interrupt Clear"]
pub struct DET_SUSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DET_SUSPD_W<'a> {
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
#[doc = "Field `MICRO_SOF` writer - Micro Start Of Frame Interrupt Clear"]
pub struct MICRO_SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> MICRO_SOF_W<'a> {
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
#[doc = "Field `INT_SOF` writer - Start Of Frame Interrupt Clear"]
pub struct INT_SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_SOF_W<'a> {
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
#[doc = "Field `ENDRESET` writer - End Of Reset Interrupt Clear"]
pub struct ENDRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WAKE_UP` writer - Wake Up CPU Interrupt Clear"]
pub struct WAKE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ENDOFRSM` writer - End Of Resume Interrupt Clear"]
pub struct ENDOFRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDOFRSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UPSTR_RES` writer - Upstream Resume Interrupt Clear"]
pub struct UPSTR_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> UPSTR_RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 1 - Suspend Interrupt Clear"]
    #[inline(always)]
    pub fn det_suspd(&mut self) -> DET_SUSPD_W {
        DET_SUSPD_W { w: self }
    }
    #[doc = "Bit 2 - Micro Start Of Frame Interrupt Clear"]
    #[inline(always)]
    pub fn micro_sof(&mut self) -> MICRO_SOF_W {
        MICRO_SOF_W { w: self }
    }
    #[doc = "Bit 3 - Start Of Frame Interrupt Clear"]
    #[inline(always)]
    pub fn int_sof(&mut self) -> INT_SOF_W {
        INT_SOF_W { w: self }
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Clear"]
    #[inline(always)]
    pub fn endreset(&mut self) -> ENDRESET_W {
        ENDRESET_W { w: self }
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Clear"]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W {
        WAKE_UP_W { w: self }
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Clear"]
    #[inline(always)]
    pub fn endofrsm(&mut self) -> ENDOFRSM_W {
        ENDOFRSM_W { w: self }
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Clear"]
    #[inline(always)]
    pub fn upstr_res(&mut self) -> UPSTR_RES_W {
        UPSTR_RES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Clear Interrupt Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrint](index.html) module"]
pub struct CLRINT_SPEC;
impl crate::RegisterSpec for CLRINT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrint::W](W) writer structure"]
impl crate::Writable for CLRINT_SPEC {
    type Writer = W;
}
