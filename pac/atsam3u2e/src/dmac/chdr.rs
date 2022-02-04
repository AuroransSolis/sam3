#[doc = "Register `CHDR` writer"]
pub struct W(crate::W<CHDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDR_SPEC>;
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
impl From<crate::W<CHDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS0` writer - Disable \\[3:0\\]"]
pub struct DIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS0_W<'a> {
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
#[doc = "Field `DIS1` writer - Disable \\[3:0\\]"]
pub struct DIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS1_W<'a> {
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
#[doc = "Field `DIS2` writer - Disable \\[3:0\\]"]
pub struct DIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS2_W<'a> {
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
#[doc = "Field `DIS3` writer - Disable \\[3:0\\]"]
pub struct DIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS3_W<'a> {
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
#[doc = "Field `RES0` writer - Resume \\[3:0\\]"]
pub struct RES0_W<'a> {
    w: &'a mut W,
}
impl<'a> RES0_W<'a> {
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
#[doc = "Field `RES1` writer - Resume \\[3:0\\]"]
pub struct RES1_W<'a> {
    w: &'a mut W,
}
impl<'a> RES1_W<'a> {
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
#[doc = "Field `RES2` writer - Resume \\[3:0\\]"]
pub struct RES2_W<'a> {
    w: &'a mut W,
}
impl<'a> RES2_W<'a> {
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
#[doc = "Field `RES3` writer - Resume \\[3:0\\]"]
pub struct RES3_W<'a> {
    w: &'a mut W,
}
impl<'a> RES3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Disable \\[3:0\\]"]
    #[inline(always)]
    pub fn dis0(&mut self) -> DIS0_W {
        DIS0_W { w: self }
    }
    #[doc = "Bit 1 - Disable \\[3:0\\]"]
    #[inline(always)]
    pub fn dis1(&mut self) -> DIS1_W {
        DIS1_W { w: self }
    }
    #[doc = "Bit 2 - Disable \\[3:0\\]"]
    #[inline(always)]
    pub fn dis2(&mut self) -> DIS2_W {
        DIS2_W { w: self }
    }
    #[doc = "Bit 3 - Disable \\[3:0\\]"]
    #[inline(always)]
    pub fn dis3(&mut self) -> DIS3_W {
        DIS3_W { w: self }
    }
    #[doc = "Bit 8 - Resume \\[3:0\\]"]
    #[inline(always)]
    pub fn res0(&mut self) -> RES0_W {
        RES0_W { w: self }
    }
    #[doc = "Bit 9 - Resume \\[3:0\\]"]
    #[inline(always)]
    pub fn res1(&mut self) -> RES1_W {
        RES1_W { w: self }
    }
    #[doc = "Bit 10 - Resume \\[3:0\\]"]
    #[inline(always)]
    pub fn res2(&mut self) -> RES2_W {
        RES2_W { w: self }
    }
    #[doc = "Bit 11 - Resume \\[3:0\\]"]
    #[inline(always)]
    pub fn res3(&mut self) -> RES3_W {
        RES3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Handler Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdr](index.html) module"]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chdr::W](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    type Writer = W;
}
