#[doc = "Register `MCR6` writer"]
pub struct W(crate::W<MCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR6_SPEC>;
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
impl From<crate::W<MCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDLC` writer - Mailbox Data Length Code"]
pub struct MDLC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `MRTR` writer - Mailbox Remote Transmission Request"]
pub struct MRTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MRTR_W<'a> {
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
#[doc = "Field `MACR` writer - Abort Request for Mailbox x"]
pub struct MACR_W<'a> {
    w: &'a mut W,
}
impl<'a> MACR_W<'a> {
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
#[doc = "Field `MTCR` writer - Mailbox Transfer Command"]
pub struct MTCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MTCR_W<'a> {
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
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    pub fn mdlc(&mut self) -> MDLC_W {
        MDLC_W { w: self }
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    pub fn mrtr(&mut self) -> MRTR_W {
        MRTR_W { w: self }
    }
    #[doc = "Bit 22 - Abort Request for Mailbox x"]
    #[inline(always)]
    pub fn macr(&mut self) -> MACR_W {
        MACR_W { w: self }
    }
    #[doc = "Bit 23 - Mailbox Transfer Command"]
    #[inline(always)]
    pub fn mtcr(&mut self) -> MTCR_W {
        MTCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Control Register (MB = 6)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr6](index.html) module"]
pub struct MCR6_SPEC;
impl crate::RegisterSpec for MCR6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mcr6::W](W) writer structure"]
impl crate::Writable for MCR6_SPEC {
    type Writer = W;
}
