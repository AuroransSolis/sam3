#[doc = "Register `EPTRST` writer"]
pub struct W(crate::W<EPTRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPTRST_SPEC>;
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
impl From<crate::W<EPTRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPTRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPT_0` writer - Endpoint 0 Reset"]
pub struct EPT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_0_W<'a> {
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
#[doc = "Field `EPT_1` writer - Endpoint 1 Reset"]
pub struct EPT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_1_W<'a> {
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
#[doc = "Field `EPT_2` writer - Endpoint 2 Reset"]
pub struct EPT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_2_W<'a> {
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
#[doc = "Field `EPT_3` writer - Endpoint 3 Reset"]
pub struct EPT_3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_3_W<'a> {
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
#[doc = "Field `EPT_4` writer - Endpoint 4 Reset"]
pub struct EPT_4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_4_W<'a> {
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
#[doc = "Field `EPT_5` writer - Endpoint 5 Reset"]
pub struct EPT_5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_5_W<'a> {
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
#[doc = "Field `EPT_6` writer - Endpoint 6 Reset"]
pub struct EPT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPT_6_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn ept_0(&mut self) -> EPT_0_W {
        EPT_0_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn ept_1(&mut self) -> EPT_1_W {
        EPT_1_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn ept_2(&mut self) -> EPT_2_W {
        EPT_2_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn ept_3(&mut self) -> EPT_3_W {
        EPT_3_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn ept_4(&mut self) -> EPT_4_W {
        EPT_4_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn ept_5(&mut self) -> EPT_5_W {
        EPT_5_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn ept_6(&mut self) -> EPT_6_W {
        EPT_6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Endpoints Reset Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptrst](index.html) module"]
pub struct EPTRST_SPEC;
impl crate::RegisterSpec for EPTRST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eptrst::W](W) writer structure"]
impl crate::Writable for EPTRST_SPEC {
    type Writer = W;
}
