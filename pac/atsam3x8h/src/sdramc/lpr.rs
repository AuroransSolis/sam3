#[doc = "Register `LPR` reader"]
pub struct R(crate::R<LPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPR` writer"]
pub struct W(crate::W<LPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPR_SPEC>;
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
impl From<crate::W<LPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-power Configuration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCB_A {
    #[doc = "0: Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    DISABLED = 0,
    #[doc = "1: The SDRAM Controller issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    SELF_REFRESH = 1,
    #[doc = "2: The SDRAM Controller issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    POWER_DOWN = 2,
    #[doc = "3: The SDRAM Controller issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    DEEP_POWER_DOWN = 3,
}
impl From<LPCB_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPCB` reader - Low-power Configuration Bits"]
pub struct LPCB_R(crate::FieldReader<u8, LPCB_A>);
impl LPCB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPCB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCB_A {
        match self.bits {
            0 => LPCB_A::DISABLED,
            1 => LPCB_A::SELF_REFRESH,
            2 => LPCB_A::POWER_DOWN,
            3 => LPCB_A::DEEP_POWER_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LPCB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SELF_REFRESH`"]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        **self == LPCB_A::SELF_REFRESH
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == LPCB_A::POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        **self == LPCB_A::DEEP_POWER_DOWN
    }
}
impl core::ops::Deref for LPCB_R {
    type Target = crate::FieldReader<u8, LPCB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCB` writer - Low-power Configuration Bits"]
pub struct LPCB_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCB_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPCB_A::DISABLED)
    }
    #[doc = "The SDRAM Controller issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn self_refresh(self) -> &'a mut W {
        self.variant(LPCB_A::SELF_REFRESH)
    }
    #[doc = "The SDRAM Controller issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(LPCB_A::POWER_DOWN)
    }
    #[doc = "The SDRAM Controller issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(LPCB_A::DEEP_POWER_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PASR` reader - Partial Array Self-refresh (only for low-power SDRAM)"]
pub struct PASR_R(crate::FieldReader<u8, u8>);
impl PASR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PASR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PASR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASR` writer - Partial Array Self-refresh (only for low-power SDRAM)"]
pub struct PASR_W<'a> {
    w: &'a mut W,
}
impl<'a> PASR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `TCSR` reader - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub struct TCSR_R(crate::FieldReader<u8, u8>);
impl TCSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCSR` writer - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub struct TCSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DS` reader - Drive Strength (only for low-power SDRAM)"]
pub struct DS_R(crate::FieldReader<u8, u8>);
impl DS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS` writer - Drive Strength (only for low-power SDRAM)"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Time to define when low-power mode is enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: The SDRAM controller activates the SDRAM low-power mode immediately after the end of the last transfer."]
    LP_LAST_XFER = 0,
    #[doc = "1: The SDRAM controller activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_64 = 1,
    #[doc = "2: The SDRAM controller activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_128 = 2,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMEOUT` reader - Time to define when low-power mode is enable"]
pub struct TIMEOUT_R(crate::FieldReader<u8, TIMEOUT_A>);
impl TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUT_A> {
        match self.bits {
            0 => Some(TIMEOUT_A::LP_LAST_XFER),
            1 => Some(TIMEOUT_A::LP_LAST_XFER_64),
            2 => Some(TIMEOUT_A::LP_LAST_XFER_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER`"]
    #[inline(always)]
    pub fn is_lp_last_xfer(&self) -> bool {
        **self == TIMEOUT_A::LP_LAST_XFER
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER_64`"]
    #[inline(always)]
    pub fn is_lp_last_xfer_64(&self) -> bool {
        **self == TIMEOUT_A::LP_LAST_XFER_64
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER_128`"]
    #[inline(always)]
    pub fn is_lp_last_xfer_128(&self) -> bool {
        **self == TIMEOUT_A::LP_LAST_XFER_128
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<u8, TIMEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - Time to define when low-power mode is enable"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode immediately after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER)
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_64(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER_64)
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_128(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&self) -> LPCB_R {
        LPCB_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&self) -> TCSR_R {
        TCSR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Time to define when low-power mode is enable"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&mut self) -> LPCB_W {
        LPCB_W { w: self }
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&mut self) -> PASR_W {
        PASR_W { w: self }
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&mut self) -> TCSR_W {
        TCSR_W { w: self }
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bits 12:13 - Time to define when low-power mode is enable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Low Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpr](index.html) module"]
pub struct LPR_SPEC;
impl crate::RegisterSpec for LPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpr::R](R) reader structure"]
impl crate::Readable for LPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpr::W](W) writer structure"]
impl crate::Writable for LPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPR to value 0"]
impl crate::Resettable for LPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
