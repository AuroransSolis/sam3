#[doc = "Register `MAN` reader"]
pub struct R(crate::R<MAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN` writer"]
pub struct W(crate::W<MAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_SPEC>;
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
impl From<crate::W<MAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u16, u16, 16, O>;
#[doc = "Field `CODE` reader - "]
pub type CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE` writer - "]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `REGA` reader - Register Address"]
pub type REGA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGA` writer - Register Address"]
pub type REGA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u8, u8, 5, O>;
#[doc = "Field `PHYA` reader - PHY Address"]
pub type PHYA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYA` writer - PHY Address"]
pub type PHYA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u8, u8, 5, O>;
#[doc = "Field `RW` reader - Read-write"]
pub type RW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RW` writer - Read-write"]
pub type RW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u8, u8, 2, O>;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SOF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SOF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&self) -> REGA_R {
        REGA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&self) -> PHYA_R {
        PHYA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Read-write"]
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W<16> {
        CODE_W::new(self)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&mut self) -> REGA_W<18> {
        REGA_W::new(self)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&mut self) -> PHYA_W<23> {
        PHYA_W::new(self)
    }
    #[doc = "Bits 28:29 - Read-write"]
    #[inline(always)]
    pub fn rw(&mut self) -> RW_W<28> {
        RW_W::new(self)
    }
    #[doc = "Bits 30:31 - Start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<30> {
        SOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phy Maintenance Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](index.html) module"]
pub struct MAN_SPEC;
impl crate::RegisterSpec for MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [man::R](R) reader structure"]
impl crate::Readable for MAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man::W](W) writer structure"]
impl crate::Writable for MAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAN to value 0"]
impl crate::Resettable for MAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
