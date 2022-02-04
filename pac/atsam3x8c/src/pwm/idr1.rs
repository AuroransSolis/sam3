#[doc = "Register `IDR1` writer"]
pub struct W(crate::W<IDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR1_SPEC>;
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
impl From<crate::W<IDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHID0` writer - Counter Event on Channel 0 Interrupt Disable"]
pub struct CHID0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID0_W<'a> {
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
#[doc = "Field `CHID1` writer - Counter Event on Channel 1 Interrupt Disable"]
pub struct CHID1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID1_W<'a> {
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
#[doc = "Field `CHID2` writer - Counter Event on Channel 2 Interrupt Disable"]
pub struct CHID2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID2_W<'a> {
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
#[doc = "Field `CHID3` writer - Counter Event on Channel 3 Interrupt Disable"]
pub struct CHID3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID3_W<'a> {
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
#[doc = "Field `CHID4` writer - Counter Event on Channel 4 Interrupt Disable"]
pub struct CHID4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID4_W<'a> {
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
#[doc = "Field `CHID5` writer - Counter Event on Channel 5 Interrupt Disable"]
pub struct CHID5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID5_W<'a> {
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
#[doc = "Field `CHID6` writer - Counter Event on Channel 6 Interrupt Disable"]
pub struct CHID6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID6_W<'a> {
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
#[doc = "Field `CHID7` writer - Counter Event on Channel 7 Interrupt Disable"]
pub struct CHID7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHID7_W<'a> {
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
#[doc = "Field `FCHID0` writer - Fault Protection Trigger on Channel 0 Interrupt Disable"]
pub struct FCHID0_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID0_W<'a> {
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
#[doc = "Field `FCHID1` writer - Fault Protection Trigger on Channel 1 Interrupt Disable"]
pub struct FCHID1_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID1_W<'a> {
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
#[doc = "Field `FCHID2` writer - Fault Protection Trigger on Channel 2 Interrupt Disable"]
pub struct FCHID2_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID2_W<'a> {
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
#[doc = "Field `FCHID3` writer - Fault Protection Trigger on Channel 3 Interrupt Disable"]
pub struct FCHID3_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `FCHID4` writer - Fault Protection Trigger on Channel 4 Interrupt Disable"]
pub struct FCHID4_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `FCHID5` writer - Fault Protection Trigger on Channel 5 Interrupt Disable"]
pub struct FCHID5_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `FCHID6` writer - Fault Protection Trigger on Channel 6 Interrupt Disable"]
pub struct FCHID6_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `FCHID7` writer - Fault Protection Trigger on Channel 7 Interrupt Disable"]
pub struct FCHID7_W<'a> {
    w: &'a mut W,
}
impl<'a> FCHID7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Disable"]
    #[inline(always)]
    pub fn chid0(&mut self) -> CHID0_W {
        CHID0_W { w: self }
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Disable"]
    #[inline(always)]
    pub fn chid1(&mut self) -> CHID1_W {
        CHID1_W { w: self }
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Disable"]
    #[inline(always)]
    pub fn chid2(&mut self) -> CHID2_W {
        CHID2_W { w: self }
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Disable"]
    #[inline(always)]
    pub fn chid3(&mut self) -> CHID3_W {
        CHID3_W { w: self }
    }
    #[doc = "Bit 4 - Counter Event on Channel 4 Interrupt Disable"]
    #[inline(always)]
    pub fn chid4(&mut self) -> CHID4_W {
        CHID4_W { w: self }
    }
    #[doc = "Bit 5 - Counter Event on Channel 5 Interrupt Disable"]
    #[inline(always)]
    pub fn chid5(&mut self) -> CHID5_W {
        CHID5_W { w: self }
    }
    #[doc = "Bit 6 - Counter Event on Channel 6 Interrupt Disable"]
    #[inline(always)]
    pub fn chid6(&mut self) -> CHID6_W {
        CHID6_W { w: self }
    }
    #[doc = "Bit 7 - Counter Event on Channel 7 Interrupt Disable"]
    #[inline(always)]
    pub fn chid7(&mut self) -> CHID7_W {
        CHID7_W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid0(&mut self) -> FCHID0_W {
        FCHID0_W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid1(&mut self) -> FCHID1_W {
        FCHID1_W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid2(&mut self) -> FCHID2_W {
        FCHID2_W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid3(&mut self) -> FCHID3_W {
        FCHID3_W { w: self }
    }
    #[doc = "Bit 20 - Fault Protection Trigger on Channel 4 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid4(&mut self) -> FCHID4_W {
        FCHID4_W { w: self }
    }
    #[doc = "Bit 21 - Fault Protection Trigger on Channel 5 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid5(&mut self) -> FCHID5_W {
        FCHID5_W { w: self }
    }
    #[doc = "Bit 22 - Fault Protection Trigger on Channel 6 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid6(&mut self) -> FCHID6_W {
        FCHID6_W { w: self }
    }
    #[doc = "Bit 23 - Fault Protection Trigger on Channel 7 Interrupt Disable"]
    #[inline(always)]
    pub fn fchid7(&mut self) -> FCHID7_W {
        FCHID7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Disable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr1](index.html) module"]
pub struct IDR1_SPEC;
impl crate::RegisterSpec for IDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr1::W](W) writer structure"]
impl crate::Writable for IDR1_SPEC {
    type Writer = W;
}
