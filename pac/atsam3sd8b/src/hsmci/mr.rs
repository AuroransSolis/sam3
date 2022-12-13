#[doc = "Register `MR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MR_SPEC>);
#[doc = "Register `MR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MR_SPEC>);
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PWSDIV` reader - Power Saving Divider"]
pub type PWSDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWSDIV` writer - Power Saving Divider"]
pub type PWSDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RDPROOF` reader - "]
pub type RDPROOF_R = crate::BitReader<bool>;
#[doc = "Field `RDPROOF` writer - "]
pub type RDPROOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `WRPROOF` reader - "]
pub type WRPROOF_R = crate::BitReader<bool>;
#[doc = "Field `WRPROOF` writer - "]
pub type WRPROOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `FBYTE` reader - Force Byte Transfer"]
pub type FBYTE_R = crate::BitReader<bool>;
#[doc = "Field `FBYTE` writer - Force Byte Transfer"]
pub type FBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PADV` reader - Padding Value"]
pub type PADV_R = crate::BitReader<bool>;
#[doc = "Field `PADV` writer - Padding Value"]
pub type PADV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PDCMODE` reader - PDC-oriented Mode"]
pub type PDCMODE_R = crate::BitReader<bool>;
#[doc = "Field `PDCMODE` writer - PDC-oriented Mode"]
pub type PDCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&self) -> PWSDIV_R {
        PWSDIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rdproof(&self) -> RDPROOF_R {
        RDPROOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wrproof(&self) -> WRPROOF_R {
        WRPROOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&self) -> FBYTE_R {
        FBYTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&self) -> PADV_R {
        PADV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDC-oriented Mode"]
    #[inline(always)]
    pub fn pdcmode(&self) -> PDCMODE_R {
        PDCMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    #[must_use]
    pub fn pwsdiv(&mut self) -> PWSDIV_W<8> {
        PWSDIV_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rdproof(&mut self) -> RDPROOF_W<11> {
        RDPROOF_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wrproof(&mut self) -> WRPROOF_W<12> {
        WRPROOF_W::new(self)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn fbyte(&mut self) -> FBYTE_W<13> {
        FBYTE_W::new(self)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    #[must_use]
    pub fn padv(&mut self) -> PADV_W<14> {
        PADV_W::new(self)
    }
    #[doc = "Bit 15 - PDC-oriented Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdcmode(&mut self) -> PDCMODE_W<15> {
        PDCMODE_W::new(self)
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
