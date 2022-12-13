#[doc = "Register `WPCR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<WPCR_SPEC>);
#[doc = "Field `WPCMD` writer - Write Protect Command"]
pub type WPCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WPRG0` writer - Write Protect Register Group 0"]
pub type WPRG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG1` writer - Write Protect Register Group 1"]
pub type WPRG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG2` writer - Write Protect Register Group 2"]
pub type WPRG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG3` writer - Write Protect Register Group 3"]
pub type WPRG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG4` writer - Write Protect Register Group 4"]
pub type WPRG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG5` writer - Write Protect Register Group 5"]
pub type WPRG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPKEY` writer - Write Protect Key"]
pub type WPKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR_SPEC, u32, u32, 24, O>;
impl W {
    #[doc = "Bits 0:1 - Write Protect Command"]
    #[inline(always)]
    #[must_use]
    pub fn wpcmd(&mut self) -> WPCMD_W<0> {
        WPCMD_W::new(self)
    }
    #[doc = "Bit 2 - Write Protect Register Group 0"]
    #[inline(always)]
    #[must_use]
    pub fn wprg0(&mut self) -> WPRG0_W<2> {
        WPRG0_W::new(self)
    }
    #[doc = "Bit 3 - Write Protect Register Group 1"]
    #[inline(always)]
    #[must_use]
    pub fn wprg1(&mut self) -> WPRG1_W<3> {
        WPRG1_W::new(self)
    }
    #[doc = "Bit 4 - Write Protect Register Group 2"]
    #[inline(always)]
    #[must_use]
    pub fn wprg2(&mut self) -> WPRG2_W<4> {
        WPRG2_W::new(self)
    }
    #[doc = "Bit 5 - Write Protect Register Group 3"]
    #[inline(always)]
    #[must_use]
    pub fn wprg3(&mut self) -> WPRG3_W<5> {
        WPRG3_W::new(self)
    }
    #[doc = "Bit 6 - Write Protect Register Group 4"]
    #[inline(always)]
    #[must_use]
    pub fn wprg4(&mut self) -> WPRG4_W<6> {
        WPRG4_W::new(self)
    }
    #[doc = "Bit 7 - Write Protect Register Group 5"]
    #[inline(always)]
    #[must_use]
    pub fn wprg5(&mut self) -> WPRG5_W<7> {
        WPRG5_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protect Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Write Protect Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](index.html) module"]
pub struct WPCR_SPEC;
impl crate::RegisterSpec for WPCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wpcr::W](W) writer structure"]
impl crate::Writable for WPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
