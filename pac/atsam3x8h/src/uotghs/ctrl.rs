#[doc = "Register `CTRL` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CTRL_SPEC>);
#[doc = "Register `CTRL` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CTRL_SPEC>);
#[doc = "Field `IDTE` reader - ID Transition Interrupt Enable"]
pub type IDTE_R = crate::BitReader<bool>;
#[doc = "Field `IDTE` writer - ID Transition Interrupt Enable"]
pub type IDTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VBUSTE` reader - VBus Transition Interrupt Enable"]
pub type VBUSTE_R = crate::BitReader<bool>;
#[doc = "Field `VBUSTE` writer - VBus Transition Interrupt Enable"]
pub type VBUSTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SRPE` reader - SRP Interrupt Enable"]
pub type SRPE_R = crate::BitReader<bool>;
#[doc = "Field `SRPE` writer - SRP Interrupt Enable"]
pub type SRPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VBERRE` reader - VBus Error Interrupt Enable"]
pub type VBERRE_R = crate::BitReader<bool>;
#[doc = "Field `VBERRE` writer - VBus Error Interrupt Enable"]
pub type VBERRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BCERRE` reader - B-Connection Error Interrupt Enable"]
pub type BCERRE_R = crate::BitReader<bool>;
#[doc = "Field `BCERRE` writer - B-Connection Error Interrupt Enable"]
pub type BCERRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ROLEEXE` reader - Role Exchange Interrupt Enable"]
pub type ROLEEXE_R = crate::BitReader<bool>;
#[doc = "Field `ROLEEXE` writer - Role Exchange Interrupt Enable"]
pub type ROLEEXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HNPERRE` reader - HNP Error Interrupt Enable"]
pub type HNPERRE_R = crate::BitReader<bool>;
#[doc = "Field `HNPERRE` writer - HNP Error Interrupt Enable"]
pub type HNPERRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STOE` reader - Suspend Time-Out Interrupt Enable"]
pub type STOE_R = crate::BitReader<bool>;
#[doc = "Field `STOE` writer - Suspend Time-Out Interrupt Enable"]
pub type STOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VBUSHWC` reader - VBus Hardware Control"]
pub type VBUSHWC_R = crate::BitReader<bool>;
#[doc = "Field `VBUSHWC` writer - VBus Hardware Control"]
pub type VBUSHWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SRPSEL` reader - SRP Selection"]
pub type SRPSEL_R = crate::BitReader<bool>;
#[doc = "Field `SRPSEL` writer - SRP Selection"]
pub type SRPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SRPREQ` reader - SRP Request"]
pub type SRPREQ_R = crate::BitReader<bool>;
#[doc = "Field `SRPREQ` writer - SRP Request"]
pub type SRPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HNPREQ` reader - HNP Request"]
pub type HNPREQ_R = crate::BitReader<bool>;
#[doc = "Field `HNPREQ` writer - HNP Request"]
pub type HNPREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OTGPADE` reader - OTG Pad Enable"]
pub type OTGPADE_R = crate::BitReader<bool>;
#[doc = "Field `OTGPADE` writer - OTG Pad Enable"]
pub type OTGPADE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VBUSPO` reader - VBus Polarity Off"]
pub type VBUSPO_R = crate::BitReader<bool>;
#[doc = "Field `VBUSPO` writer - VBus Polarity Off"]
pub type VBUSPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FRZCLK` reader - Freeze USB Clock"]
pub type FRZCLK_R = crate::BitReader<bool>;
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub type FRZCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `USBE` reader - UOTGHS Enable"]
pub type USBE_R = crate::BitReader<bool>;
#[doc = "Field `USBE` writer - UOTGHS Enable"]
pub type USBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TIMVALUE` reader - Timer Value"]
pub type TIMVALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMVALUE` writer - Timer Value"]
pub type TIMVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIMPAGE` reader - Timer Page"]
pub type TIMPAGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMPAGE` writer - Timer Page"]
pub type TIMPAGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `UNLOCK` reader - Timer Access Unlock"]
pub type UNLOCK_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCK` writer - Timer Access Unlock"]
pub type UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `UIDE` reader - UOTGID Pin Enable"]
pub type UIDE_R = crate::BitReader<UIDE_A>;
#[doc = "UOTGID Pin Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIDE_A {
    #[doc = "0: The USB mode (device/host) is selected from the UIMOD bit."]
    Uimod = 0,
    #[doc = "1: The USB mode (device/host) is selected from the UOTGID input pin."]
    Uotgid = 1,
}
impl From<UIDE_A> for bool {
    #[inline(always)]
    fn from(variant: UIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl UIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIDE_A {
        match self.bits {
            false => UIDE_A::Uimod,
            true => UIDE_A::Uotgid,
        }
    }
    #[doc = "Checks if the value of the field is `Uimod`"]
    #[inline(always)]
    pub fn is_uimod(&self) -> bool {
        *self == UIDE_A::Uimod
    }
    #[doc = "Checks if the value of the field is `Uotgid`"]
    #[inline(always)]
    pub fn is_uotgid(&self) -> bool {
        *self == UIDE_A::Uotgid
    }
}
#[doc = "Field `UIDE` writer - UOTGID Pin Enable"]
pub type UIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, UIDE_A, O>;
impl<'a, const O: u8> UIDE_W<'a, O> {
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    #[inline(always)]
    pub fn uimod(self) -> &'a mut W {
        self.variant(UIDE_A::Uimod)
    }
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    #[inline(always)]
    pub fn uotgid(self) -> &'a mut W {
        self.variant(UIDE_A::Uotgid)
    }
}
#[doc = "Field `UIMOD` reader - UOTGHS Mode"]
pub type UIMOD_R = crate::BitReader<UIMOD_A>;
#[doc = "UOTGHS Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIMOD_A {
    #[doc = "0: The module is in USB host mode."]
    Host = 0,
    #[doc = "1: The module is in USB device mode."]
    Device = 1,
}
impl From<UIMOD_A> for bool {
    #[inline(always)]
    fn from(variant: UIMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl UIMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIMOD_A {
        match self.bits {
            false => UIMOD_A::Host,
            true => UIMOD_A::Device,
        }
    }
    #[doc = "Checks if the value of the field is `Host`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == UIMOD_A::Host
    }
    #[doc = "Checks if the value of the field is `Device`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == UIMOD_A::Device
    }
}
#[doc = "Field `UIMOD` writer - UOTGHS Mode"]
pub type UIMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, UIMOD_A, O>;
impl<'a, const O: u8> UIMOD_W<'a, O> {
    #[doc = "The module is in USB host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMOD_A::Host)
    }
    #[doc = "The module is in USB device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMOD_A::Device)
    }
}
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    pub fn idte(&self) -> IDTE_R {
        IDTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    pub fn vbuste(&self) -> VBUSTE_R {
        VBUSTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    pub fn srpe(&self) -> SRPE_R {
        SRPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vberre(&self) -> VBERRE_R {
        VBERRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn bcerre(&self) -> BCERRE_R {
        BCERRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    pub fn roleexe(&self) -> ROLEEXE_R {
        ROLEEXE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    pub fn hnperre(&self) -> HNPERRE_R {
        HNPERRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    pub fn stoe(&self) -> STOE_R {
        STOE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VBUSHWC_R {
        VBUSHWC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    pub fn srpsel(&self) -> SRPSEL_R {
        SRPSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SRPREQ_R {
        SRPREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    pub fn otgpade(&self) -> OTGPADE_R {
        OTGPADE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    pub fn vbuspo(&self) -> VBUSPO_R {
        VBUSPO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    pub fn timvalue(&self) -> TIMVALUE_R {
        TIMVALUE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    pub fn timpage(&self) -> TIMPAGE_R {
        TIMPAGE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    pub fn uide(&self) -> UIDE_R {
        UIDE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idte(&mut self) -> IDTE_W<0> {
        IDTE_W::new(self)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbuste(&mut self) -> VBUSTE_W<1> {
        VBUSTE_W::new(self)
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srpe(&mut self) -> SRPE_W<2> {
        SRPE_W::new(self)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vberre(&mut self) -> VBERRE_W<3> {
        VBERRE_W::new(self)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcerre(&mut self) -> BCERRE_W<4> {
        BCERRE_W::new(self)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roleexe(&mut self) -> ROLEEXE_W<5> {
        ROLEEXE_W::new(self)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hnperre(&mut self) -> HNPERRE_W<6> {
        HNPERRE_W::new(self)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stoe(&mut self) -> STOE_W<7> {
        STOE_W::new(self)
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    #[must_use]
    pub fn vbushwc(&mut self) -> VBUSHWC_W<8> {
        VBUSHWC_W::new(self)
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    #[must_use]
    pub fn srpsel(&mut self) -> SRPSEL_W<9> {
        SRPSEL_W::new(self)
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    #[must_use]
    pub fn srpreq(&mut self) -> SRPREQ_W<10> {
        SRPREQ_W::new(self)
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HNPREQ_W<11> {
        HNPREQ_W::new(self)
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgpade(&mut self) -> OTGPADE_W<12> {
        OTGPADE_W::new(self)
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    #[must_use]
    pub fn vbuspo(&mut self) -> VBUSPO_W<13> {
        VBUSPO_W::new(self)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    #[must_use]
    pub fn frzclk(&mut self) -> FRZCLK_W<14> {
        FRZCLK_W::new(self)
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbe(&mut self) -> USBE_W<15> {
        USBE_W::new(self)
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn timvalue(&mut self) -> TIMVALUE_W<16> {
        TIMVALUE_W::new(self)
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    #[must_use]
    pub fn timpage(&mut self) -> TIMPAGE_W<20> {
        TIMPAGE_W::new(self)
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<22> {
        UNLOCK_W::new(self)
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uide(&mut self) -> UIDE_W<24> {
        UIDE_W::new(self)
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uimod(&mut self) -> UIMOD_W<25> {
        UIMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_4000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_4000;
}
