#[doc = "Register `TST` reader"]
pub struct R(crate::R<TST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST` writer"]
pub struct W(crate::W<TST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_SPEC>;
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
impl From<crate::W<TST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_CFG_A {
    #[doc = "0: Normal Mode: The macro is in Full Speed mode, ready to make a High Speed identification, if the host supports it and then to automatically switch to High Speed mode"]
    NORMAL = 0,
    #[doc = "2: Force High Speed: Set this value to force the hardware to work in High Speed mode. Only for debug or test purpose."]
    HIGH_SPEED = 2,
    #[doc = "3: Force Full Speed: Set this value to force the hardware to work only in Full Speed mode. In this configuration, the macro will not respond to a High Speed reset handshake."]
    FULL_SPEED = 3,
}
impl From<SPEED_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED_CFG` reader - Speed Configuration"]
pub struct SPEED_CFG_R(crate::FieldReader<u8, SPEED_CFG_A>);
impl SPEED_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_CFG_A> {
        match self.bits {
            0 => Some(SPEED_CFG_A::NORMAL),
            2 => Some(SPEED_CFG_A::HIGH_SPEED),
            3 => Some(SPEED_CFG_A::FULL_SPEED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SPEED_CFG_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == SPEED_CFG_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        **self == SPEED_CFG_A::FULL_SPEED
    }
}
impl core::ops::Deref for SPEED_CFG_R {
    type Target = crate::FieldReader<u8, SPEED_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED_CFG` writer - Speed Configuration"]
pub struct SPEED_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal Mode: The macro is in Full Speed mode, ready to make a High Speed identification, if the host supports it and then to automatically switch to High Speed mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPEED_CFG_A::NORMAL)
    }
    #[doc = "Force High Speed: Set this value to force the hardware to work in High Speed mode. Only for debug or test purpose."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(SPEED_CFG_A::HIGH_SPEED)
    }
    #[doc = "Force Full Speed: Set this value to force the hardware to work only in Full Speed mode. In this configuration, the macro will not respond to a High Speed reset handshake."]
    #[inline(always)]
    pub fn full_speed(self) -> &'a mut W {
        self.variant(SPEED_CFG_A::FULL_SPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TST_J` reader - Test J Mode"]
pub struct TST_J_R(crate::FieldReader<bool, bool>);
impl TST_J_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TST_J_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TST_J_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TST_J` writer - Test J Mode"]
pub struct TST_J_W<'a> {
    w: &'a mut W,
}
impl<'a> TST_J_W<'a> {
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
#[doc = "Field `TST_K` reader - Test K Mode"]
pub struct TST_K_R(crate::FieldReader<bool, bool>);
impl TST_K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TST_K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TST_K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TST_K` writer - Test K Mode"]
pub struct TST_K_W<'a> {
    w: &'a mut W,
}
impl<'a> TST_K_W<'a> {
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
#[doc = "Field `TST_PKT` reader - Test Packet Mode"]
pub struct TST_PKT_R(crate::FieldReader<bool, bool>);
impl TST_PKT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TST_PKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TST_PKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TST_PKT` writer - Test Packet Mode"]
pub struct TST_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TST_PKT_W<'a> {
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
#[doc = "Field `OPMODE2` reader - OpMode2"]
pub struct OPMODE2_R(crate::FieldReader<bool, bool>);
impl OPMODE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPMODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPMODE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE2` writer - OpMode2"]
pub struct OPMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE2_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Speed Configuration"]
    #[inline(always)]
    pub fn speed_cfg(&self) -> SPEED_CFG_R {
        SPEED_CFG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Test J Mode"]
    #[inline(always)]
    pub fn tst_j(&self) -> TST_J_R {
        TST_J_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Test K Mode"]
    #[inline(always)]
    pub fn tst_k(&self) -> TST_K_R {
        TST_K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Test Packet Mode"]
    #[inline(always)]
    pub fn tst_pkt(&self) -> TST_PKT_R {
        TST_PKT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OpMode2"]
    #[inline(always)]
    pub fn opmode2(&self) -> OPMODE2_R {
        OPMODE2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed Configuration"]
    #[inline(always)]
    pub fn speed_cfg(&mut self) -> SPEED_CFG_W {
        SPEED_CFG_W { w: self }
    }
    #[doc = "Bit 2 - Test J Mode"]
    #[inline(always)]
    pub fn tst_j(&mut self) -> TST_J_W {
        TST_J_W { w: self }
    }
    #[doc = "Bit 3 - Test K Mode"]
    #[inline(always)]
    pub fn tst_k(&mut self) -> TST_K_W {
        TST_K_W { w: self }
    }
    #[doc = "Bit 4 - Test Packet Mode"]
    #[inline(always)]
    pub fn tst_pkt(&mut self) -> TST_PKT_W {
        TST_PKT_W { w: self }
    }
    #[doc = "Bit 5 - OpMode2"]
    #[inline(always)]
    pub fn opmode2(&mut self) -> OPMODE2_W {
        OPMODE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst](index.html) module"]
pub struct TST_SPEC;
impl crate::RegisterSpec for TST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst::R](R) reader structure"]
impl crate::Readable for TST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst::W](W) writer structure"]
impl crate::Writable for TST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TST to value 0"]
impl crate::Resettable for TST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
