#[doc = "Register `CHER` writer"]
pub struct W(crate::W<CHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHER_SPEC>;
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
impl From<crate::W<CHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA0` writer - Enable \\[3:0\\]"]
pub struct ENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA0_W<'a> {
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
#[doc = "Field `ENA1` writer - Enable \\[3:0\\]"]
pub struct ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA1_W<'a> {
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
#[doc = "Field `ENA2` writer - Enable \\[3:0\\]"]
pub struct ENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA2_W<'a> {
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
#[doc = "Field `ENA3` writer - Enable \\[3:0\\]"]
pub struct ENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA3_W<'a> {
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
#[doc = "Field `SUSP0` writer - Suspend \\[3:0\\]"]
pub struct SUSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP0_W<'a> {
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
#[doc = "Field `SUSP1` writer - Suspend \\[3:0\\]"]
pub struct SUSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP1_W<'a> {
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
#[doc = "Field `SUSP2` writer - Suspend \\[3:0\\]"]
pub struct SUSP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP2_W<'a> {
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
#[doc = "Field `SUSP3` writer - Suspend \\[3:0\\]"]
pub struct SUSP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP3_W<'a> {
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
#[doc = "Field `KEEP0` writer - Keep on \\[3:0\\]"]
pub struct KEEP0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `KEEP1` writer - Keep on \\[3:0\\]"]
pub struct KEEP1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `KEEP2` writer - Keep on \\[3:0\\]"]
pub struct KEEP2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `KEEP3` writer - Keep on \\[3:0\\]"]
pub struct KEEP3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena0(&mut self) -> ENA0_W {
        ENA0_W { w: self }
    }
    #[doc = "Bit 1 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena1(&mut self) -> ENA1_W {
        ENA1_W { w: self }
    }
    #[doc = "Bit 2 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena2(&mut self) -> ENA2_W {
        ENA2_W { w: self }
    }
    #[doc = "Bit 3 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena3(&mut self) -> ENA3_W {
        ENA3_W { w: self }
    }
    #[doc = "Bit 8 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp0(&mut self) -> SUSP0_W {
        SUSP0_W { w: self }
    }
    #[doc = "Bit 9 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp1(&mut self) -> SUSP1_W {
        SUSP1_W { w: self }
    }
    #[doc = "Bit 10 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp2(&mut self) -> SUSP2_W {
        SUSP2_W { w: self }
    }
    #[doc = "Bit 11 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp3(&mut self) -> SUSP3_W {
        SUSP3_W { w: self }
    }
    #[doc = "Bit 24 - Keep on \\[3:0\\]"]
    #[inline(always)]
    pub fn keep0(&mut self) -> KEEP0_W {
        KEEP0_W { w: self }
    }
    #[doc = "Bit 25 - Keep on \\[3:0\\]"]
    #[inline(always)]
    pub fn keep1(&mut self) -> KEEP1_W {
        KEEP1_W { w: self }
    }
    #[doc = "Bit 26 - Keep on \\[3:0\\]"]
    #[inline(always)]
    pub fn keep2(&mut self) -> KEEP2_W {
        KEEP2_W { w: self }
    }
    #[doc = "Bit 27 - Keep on \\[3:0\\]"]
    #[inline(always)]
    pub fn keep3(&mut self) -> KEEP3_W {
        KEEP3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Handler Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cher](index.html) module"]
pub struct CHER_SPEC;
impl crate::RegisterSpec for CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cher::W](W) writer structure"]
impl crate::Writable for CHER_SPEC {
    type Writer = W;
}
