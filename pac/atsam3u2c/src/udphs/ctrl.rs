#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_ADDR` reader - UDPHS Address"]
pub struct DEV_ADDR_R(crate::FieldReader<u8, u8>);
impl DEV_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEV_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_ADDR` writer - UDPHS Address"]
pub struct DEV_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `FADDR_EN` reader - Function Address Enable"]
pub struct FADDR_EN_R(crate::FieldReader<bool, bool>);
impl FADDR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FADDR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR_EN` writer - Function Address Enable"]
pub struct FADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_EN_W<'a> {
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
#[doc = "Field `EN_UDPHS` reader - UDPHS Enable"]
pub struct EN_UDPHS_R(crate::FieldReader<bool, bool>);
impl EN_UDPHS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_UDPHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_UDPHS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_UDPHS` writer - UDPHS Enable"]
pub struct EN_UDPHS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_UDPHS_W<'a> {
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
#[doc = "Field `DETACH` reader - Detach Command"]
pub struct DETACH_R(crate::FieldReader<bool, bool>);
impl DETACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DETACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DETACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DETACH` writer - Detach Command"]
pub struct DETACH_W<'a> {
    w: &'a mut W,
}
impl<'a> DETACH_W<'a> {
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
#[doc = "Field `REWAKEUP` reader - Send Remote Wake Up"]
pub struct REWAKEUP_R(crate::FieldReader<bool, bool>);
impl REWAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REWAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REWAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REWAKEUP` writer - Send Remote Wake Up"]
pub struct REWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> REWAKEUP_W<'a> {
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
#[doc = "Field `PULLD_DIS` reader - Pull-Down Disable"]
pub struct PULLD_DIS_R(crate::FieldReader<bool, bool>);
impl PULLD_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULLD_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULLD_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULLD_DIS` writer - Pull-Down Disable"]
pub struct PULLD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLD_DIS_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - UDPHS Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Function Address Enable"]
    #[inline(always)]
    pub fn faddr_en(&self) -> FADDR_EN_R {
        FADDR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UDPHS Enable"]
    #[inline(always)]
    pub fn en_udphs(&self) -> EN_UDPHS_R {
        EN_UDPHS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Detach Command"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Send Remote Wake Up"]
    #[inline(always)]
    pub fn rewakeup(&self) -> REWAKEUP_R {
        REWAKEUP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pull-Down Disable"]
    #[inline(always)]
    pub fn pulld_dis(&self) -> PULLD_DIS_R {
        PULLD_DIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - UDPHS Address"]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W {
        DEV_ADDR_W { w: self }
    }
    #[doc = "Bit 7 - Function Address Enable"]
    #[inline(always)]
    pub fn faddr_en(&mut self) -> FADDR_EN_W {
        FADDR_EN_W { w: self }
    }
    #[doc = "Bit 8 - UDPHS Enable"]
    #[inline(always)]
    pub fn en_udphs(&mut self) -> EN_UDPHS_W {
        EN_UDPHS_W { w: self }
    }
    #[doc = "Bit 9 - Detach Command"]
    #[inline(always)]
    pub fn detach(&mut self) -> DETACH_W {
        DETACH_W { w: self }
    }
    #[doc = "Bit 10 - Send Remote Wake Up"]
    #[inline(always)]
    pub fn rewakeup(&mut self) -> REWAKEUP_W {
        REWAKEUP_W { w: self }
    }
    #[doc = "Bit 11 - Pull-Down Disable"]
    #[inline(always)]
    pub fn pulld_dis(&mut self) -> PULLD_DIS_W {
        PULLD_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0200"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
