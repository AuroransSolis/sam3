#[doc = "Register `CHDR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CHDR_SPEC>);
#[doc = "Field `CH0` writer - "]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH1` writer - "]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH2` writer - "]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH3` writer - "]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH4` writer - "]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH5` writer - "]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH6` writer - "]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH7` writer - "]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdr](index.html) module"]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chdr::W](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
