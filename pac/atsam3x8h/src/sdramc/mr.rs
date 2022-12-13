#[doc = "Register `MR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MR_SPEC>);
#[doc = "Register `MR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MR_SPEC>);
#[doc = "Field `MODE` reader - SDRAMC Command Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "SDRAMC Command Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, command must be followed by a write to the SDRAM."]
    Normal = 0,
    #[doc = "1: The SDRAM Controller issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    Nop = 1,
    #[doc = "2: The SDRAM Controller issues an \"All Banks Precharge\" command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    AllbanksPrecharge = 2,
    #[doc = "3: The SDRAM Controller issues a \"Load Mode Register\" command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    LoadModereg = 3,
    #[doc = "4: The SDRAM Controller issues an \"Auto-Refresh\" Command when the SDRAM device is accessed regardless of the cycle. Previously, an \"All Banks Precharge\" command must be issued. To activate this mode, command must be followed by a write to the SDRAM."]
    AutoRefresh = 4,
    #[doc = "5: The SDRAM Controller issues an \"Extended Load Mode Register\" command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the \"Extended Load Mode Register\" command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    ExtLoadModereg = 5,
    #[doc = "6: Deep power-down mode. Enters deep power-down mode."]
    DeepPowerdown = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::Normal),
            1 => Some(MODE_A::Nop),
            2 => Some(MODE_A::AllbanksPrecharge),
            3 => Some(MODE_A::LoadModereg),
            4 => Some(MODE_A::AutoRefresh),
            5 => Some(MODE_A::ExtLoadModereg),
            6 => Some(MODE_A::DeepPowerdown),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE_A::Normal
    }
    #[doc = "Checks if the value of the field is `Nop`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == MODE_A::Nop
    }
    #[doc = "Checks if the value of the field is `AllbanksPrecharge`"]
    #[inline(always)]
    pub fn is_allbanks_precharge(&self) -> bool {
        *self == MODE_A::AllbanksPrecharge
    }
    #[doc = "Checks if the value of the field is `LoadModereg`"]
    #[inline(always)]
    pub fn is_load_modereg(&self) -> bool {
        *self == MODE_A::LoadModereg
    }
    #[doc = "Checks if the value of the field is `AutoRefresh`"]
    #[inline(always)]
    pub fn is_auto_refresh(&self) -> bool {
        *self == MODE_A::AutoRefresh
    }
    #[doc = "Checks if the value of the field is `ExtLoadModereg`"]
    #[inline(always)]
    pub fn is_ext_load_modereg(&self) -> bool {
        *self == MODE_A::ExtLoadModereg
    }
    #[doc = "Checks if the value of the field is `DeepPowerdown`"]
    #[inline(always)]
    pub fn is_deep_powerdown(&self) -> bool {
        *self == MODE_A::DeepPowerdown
    }
}
#[doc = "Field `MODE` writer - SDRAMC Command Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_A::Normal)
    }
    #[doc = "The SDRAM Controller issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(MODE_A::Nop)
    }
    #[doc = "The SDRAM Controller issues an \"All Banks Precharge\" command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn allbanks_precharge(self) -> &'a mut W {
        self.variant(MODE_A::AllbanksPrecharge)
    }
    #[doc = "The SDRAM Controller issues a \"Load Mode Register\" command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn load_modereg(self) -> &'a mut W {
        self.variant(MODE_A::LoadModereg)
    }
    #[doc = "The SDRAM Controller issues an \"Auto-Refresh\" Command when the SDRAM device is accessed regardless of the cycle. Previously, an \"All Banks Precharge\" command must be issued. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn auto_refresh(self) -> &'a mut W {
        self.variant(MODE_A::AutoRefresh)
    }
    #[doc = "The SDRAM Controller issues an \"Extended Load Mode Register\" command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the \"Extended Load Mode Register\" command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    #[inline(always)]
    pub fn ext_load_modereg(self) -> &'a mut W {
        self.variant(MODE_A::ExtLoadModereg)
    }
    #[doc = "Deep power-down mode. Enters deep power-down mode."]
    #[inline(always)]
    pub fn deep_powerdown(self) -> &'a mut W {
        self.variant(MODE_A::DeepPowerdown)
    }
}
impl R {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
