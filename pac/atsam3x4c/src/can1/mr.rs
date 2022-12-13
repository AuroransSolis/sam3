#[doc = "Register `MR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MR_SPEC>);
#[doc = "Register `MR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MR_SPEC>);
#[doc = "Field `CANEN` reader - CAN Controller Enable"]
pub type CANEN_R = crate::BitReader<bool>;
#[doc = "Field `CANEN` writer - CAN Controller Enable"]
pub type CANEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `LPM` reader - Disable/Enable Low Power Mode"]
pub type LPM_R = crate::BitReader<bool>;
#[doc = "Field `LPM` writer - Disable/Enable Low Power Mode"]
pub type LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `ABM` reader - Disable/Enable Autobaud/Listen mode"]
pub type ABM_R = crate::BitReader<bool>;
#[doc = "Field `ABM` writer - Disable/Enable Autobaud/Listen mode"]
pub type ABM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `OVL` reader - Disable/Enable Overload Frame"]
pub type OVL_R = crate::BitReader<bool>;
#[doc = "Field `OVL` writer - Disable/Enable Overload Frame"]
pub type OVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `TEOF` reader - Timestamp messages at each end of Frame"]
pub type TEOF_R = crate::BitReader<bool>;
#[doc = "Field `TEOF` writer - Timestamp messages at each end of Frame"]
pub type TEOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `TTM` reader - Disable/Enable Time Triggered Mode"]
pub type TTM_R = crate::BitReader<bool>;
#[doc = "Field `TTM` writer - Disable/Enable Time Triggered Mode"]
pub type TTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `TIMFRZ` reader - Enable Timer Freeze"]
pub type TIMFRZ_R = crate::BitReader<bool>;
#[doc = "Field `TIMFRZ` writer - Enable Timer Freeze"]
pub type TIMFRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `DRPT` reader - Disable Repeat"]
pub type DRPT_R = crate::BitReader<bool>;
#[doc = "Field `DRPT` writer - Disable Repeat"]
pub type DRPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `RXSYNC` reader - Reception Synchronization Stage (not readable)"]
pub type RXSYNC_R = crate::FieldReader<u8, RXSYNC_A>;
#[doc = "Reception Synchronization Stage (not readable)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXSYNC_A {
    #[doc = "0: Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    DoublePp = 0,
    #[doc = "1: Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    DoublePn = 1,
    #[doc = "2: Rx Signal with Single Synchro Stage (Positive Edge)"]
    SingleP = 2,
    #[doc = "3: Rx Signal with No Synchro Stage"]
    None = 3,
}
impl From<RXSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSYNC_A) -> Self {
        variant as _
    }
}
impl RXSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXSYNC_A> {
        match self.bits {
            0 => Some(RXSYNC_A::DoublePp),
            1 => Some(RXSYNC_A::DoublePn),
            2 => Some(RXSYNC_A::SingleP),
            3 => Some(RXSYNC_A::None),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DoublePp`"]
    #[inline(always)]
    pub fn is_double_pp(&self) -> bool {
        *self == RXSYNC_A::DoublePp
    }
    #[doc = "Checks if the value of the field is `DoublePn`"]
    #[inline(always)]
    pub fn is_double_pn(&self) -> bool {
        *self == RXSYNC_A::DoublePn
    }
    #[doc = "Checks if the value of the field is `SingleP`"]
    #[inline(always)]
    pub fn is_single_p(&self) -> bool {
        *self == RXSYNC_A::SingleP
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RXSYNC_A::None
    }
}
#[doc = "Field `RXSYNC` writer - Reception Synchronization Stage (not readable)"]
pub type RXSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, RXSYNC_A, 3, O>;
impl<'a, const O: u8> RXSYNC_W<'a, O> {
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline(always)]
    pub fn double_pp(self) -> &'a mut W {
        self.variant(RXSYNC_A::DoublePp)
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline(always)]
    pub fn double_pn(self) -> &'a mut W {
        self.variant(RXSYNC_A::DoublePn)
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline(always)]
    pub fn single_p(self) -> &'a mut W {
        self.variant(RXSYNC_A::SingleP)
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RXSYNC_A::None)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&self) -> ABM_R {
        ABM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&self) -> OVL_R {
        OVL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&self) -> TEOF_R {
        TEOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&self) -> TTM_R {
        TTM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&self) -> TIMFRZ_R {
        TIMFRZ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&self) -> DRPT_R {
        DRPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    pub fn rxsync(&self) -> RXSYNC_R {
        RXSYNC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CANEN_W<0> {
        CANEN_W::new(self)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<1> {
        LPM_W::new(self)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn abm(&mut self) -> ABM_W<2> {
        ABM_W::new(self)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ovl(&mut self) -> OVL_W<3> {
        OVL_W::new(self)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn teof(&mut self) -> TEOF_W<4> {
        TEOF_W::new(self)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ttm(&mut self) -> TTM_W<5> {
        TTM_W::new(self)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn timfrz(&mut self) -> TIMFRZ_W<6> {
        TIMFRZ_W::new(self)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    #[must_use]
    pub fn drpt(&mut self) -> DRPT_W<7> {
        DRPT_W::new(self)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    #[must_use]
    pub fn rxsync(&mut self) -> RXSYNC_W<24> {
        RXSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
