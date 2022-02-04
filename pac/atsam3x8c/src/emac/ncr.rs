#[doc = "Register `NCR` reader"]
pub struct R(crate::R<NCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NCR` writer"]
pub struct W(crate::W<NCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NCR_SPEC>;
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
impl From<crate::W<NCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LB` reader - LoopBack"]
pub struct LB_R(crate::FieldReader<bool, bool>);
impl LB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LB` writer - LoopBack"]
pub struct LB_W<'a> {
    w: &'a mut W,
}
impl<'a> LB_W<'a> {
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
#[doc = "Field `LLB` reader - Loopback local"]
pub struct LLB_R(crate::FieldReader<bool, bool>);
impl LLB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLB` writer - Loopback local"]
pub struct LLB_W<'a> {
    w: &'a mut W,
}
impl<'a> LLB_W<'a> {
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
#[doc = "Field `RE` reader - Receive enable"]
pub struct RE_R(crate::FieldReader<bool, bool>);
impl RE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE` writer - Receive enable"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
#[doc = "Field `TE` reader - Transmit enable"]
pub struct TE_R(crate::FieldReader<bool, bool>);
impl TE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE` writer - Transmit enable"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
#[doc = "Field `MPE` reader - Management port enable"]
pub struct MPE_R(crate::FieldReader<bool, bool>);
impl MPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPE` writer - Management port enable"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
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
#[doc = "Field `CLRSTAT` reader - Clear statistics registers"]
pub struct CLRSTAT_R(crate::FieldReader<bool, bool>);
impl CLRSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRSTAT` writer - Clear statistics registers"]
pub struct CLRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSTAT_W<'a> {
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
#[doc = "Field `INCSTAT` reader - Increment statistics registers"]
pub struct INCSTAT_R(crate::FieldReader<bool, bool>);
impl INCSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INCSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCSTAT` writer - Increment statistics registers"]
pub struct INCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCSTAT_W<'a> {
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
#[doc = "Field `WESTAT` reader - Write enable for statistics registers"]
pub struct WESTAT_R(crate::FieldReader<bool, bool>);
impl WESTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WESTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WESTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WESTAT` writer - Write enable for statistics registers"]
pub struct WESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WESTAT_W<'a> {
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
#[doc = "Field `BP` reader - Back pressure"]
pub struct BP_R(crate::FieldReader<bool, bool>);
impl BP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BP` writer - Back pressure"]
pub struct BP_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_W<'a> {
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
#[doc = "Field `TSTART` reader - Start transmission"]
pub struct TSTART_R(crate::FieldReader<bool, bool>);
impl TSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTART` writer - Start transmission"]
pub struct TSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTART_W<'a> {
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
#[doc = "Field `THALT` reader - Transmit halt"]
pub struct THALT_R(crate::FieldReader<bool, bool>);
impl THALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THALT` writer - Transmit halt"]
pub struct THALT_W<'a> {
    w: &'a mut W,
}
impl<'a> THALT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LoopBack"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrstat(&self) -> CLRSTAT_R {
        CLRSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Increment statistics registers"]
    #[inline(always)]
    pub fn incstat(&self) -> INCSTAT_R {
        INCSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn westat(&self) -> WESTAT_R {
        WESTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn thalt(&self) -> THALT_R {
        THALT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LoopBack"]
    #[inline(always)]
    pub fn lb(&mut self) -> LB_W {
        LB_W { w: self }
    }
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W {
        LLB_W { w: self }
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W { w: self }
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrstat(&mut self) -> CLRSTAT_W {
        CLRSTAT_W { w: self }
    }
    #[doc = "Bit 6 - Increment statistics registers"]
    #[inline(always)]
    pub fn incstat(&mut self) -> INCSTAT_W {
        INCSTAT_W { w: self }
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn westat(&mut self) -> WESTAT_W {
        WESTAT_W { w: self }
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&mut self) -> BP_W {
        BP_W { w: self }
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W {
        TSTART_W { w: self }
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn thalt(&mut self) -> THALT_W {
        THALT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr](index.html) module"]
pub struct NCR_SPEC;
impl crate::RegisterSpec for NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ncr::R](R) reader structure"]
impl crate::Readable for NCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ncr::W](W) writer structure"]
impl crate::Writable for NCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
