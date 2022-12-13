#[doc = "Register `OCMS` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<OCMS_SPEC>);
#[doc = "Register `OCMS` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<OCMS_SPEC>);
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub type SMSE_R = crate::BitReader<bool>;
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub type SMSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
#[doc = "Field `CS0SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS0SE_R = crate::BitReader<bool>;
#[doc = "Field `CS0SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS0SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
#[doc = "Field `CS1SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS1SE_R = crate::BitReader<bool>;
#[doc = "Field `CS1SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS1SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
#[doc = "Field `CS2SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS2SE_R = crate::BitReader<bool>;
#[doc = "Field `CS2SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS2SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
#[doc = "Field `CS3SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS3SE_R = crate::BitReader<bool>;
#[doc = "Field `CS3SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS3SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&self) -> CS0SE_R {
        CS0SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&self) -> CS1SE_R {
        CS1SE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&self) -> CS2SE_R {
        CS2SE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&self) -> CS3SE_R {
        CS3SE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smse(&mut self) -> SMSE_W<0> {
        SMSE_W::new(self)
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs0se(&mut self) -> CS0SE_W<16> {
        CS0SE_W::new(self)
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs1se(&mut self) -> CS1SE_W<17> {
        CS1SE_W::new(self)
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs2se(&mut self) -> CS2SE_W<18> {
        CS2SE_W::new(self)
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs3se(&mut self) -> CS3SE_W<19> {
        CS3SE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC OCMS MODE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](index.html) module"]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocms::R](R) reader structure"]
impl crate::Readable for OCMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocms::W](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
