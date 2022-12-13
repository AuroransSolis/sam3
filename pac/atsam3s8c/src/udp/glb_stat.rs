#[doc = "Register `GLB_STAT` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<GLB_STAT_SPEC>);
#[doc = "Register `GLB_STAT` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<GLB_STAT_SPEC>);
#[doc = "Field `FADDEN` reader - Function Address Enable"]
pub type FADDEN_R = crate::BitReader<bool>;
#[doc = "Field `FADDEN` writer - Function Address Enable"]
pub type FADDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_STAT_SPEC, bool, O>;
#[doc = "Field `CONFG` reader - Configured"]
pub type CONFG_R = crate::BitReader<bool>;
#[doc = "Field `CONFG` writer - Configured"]
pub type CONFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_STAT_SPEC, bool, O>;
#[doc = "Field `ESR` reader - Enable Send Resume"]
pub type ESR_R = crate::BitReader<bool>;
#[doc = "Field `ESR` writer - Enable Send Resume"]
pub type ESR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_STAT_SPEC, bool, O>;
#[doc = "Field `RSMINPR` reader - "]
pub type RSMINPR_R = crate::BitReader<bool>;
#[doc = "Field `RSMINPR` writer - "]
pub type RSMINPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_STAT_SPEC, bool, O>;
#[doc = "Field `RMWUPE` reader - Remote Wake Up Enable"]
pub type RMWUPE_R = crate::BitReader<bool>;
#[doc = "Field `RMWUPE` writer - Remote Wake Up Enable"]
pub type RMWUPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLB_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    pub fn fadden(&self) -> FADDEN_R {
        FADDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    pub fn confg(&self) -> CONFG_R {
        CONFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    pub fn esr(&self) -> ESR_R {
        ESR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsminpr(&self) -> RSMINPR_R {
        RSMINPR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    pub fn rmwupe(&self) -> RMWUPE_R {
        RMWUPE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fadden(&mut self) -> FADDEN_W<0> {
        FADDEN_W::new(self)
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    #[must_use]
    pub fn confg(&mut self) -> CONFG_W<1> {
        CONFG_W::new(self)
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    #[must_use]
    pub fn esr(&mut self) -> ESR_W<2> {
        ESR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rsminpr(&mut self) -> RSMINPR_W<3> {
        RSMINPR_W::new(self)
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmwupe(&mut self) -> RMWUPE_W<4> {
        RMWUPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_stat](index.html) module"]
pub struct GLB_STAT_SPEC;
impl crate::RegisterSpec for GLB_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glb_stat::R](R) reader structure"]
impl crate::Readable for GLB_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glb_stat::W](W) writer structure"]
impl crate::Writable for GLB_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLB_STAT to value 0x10"]
impl crate::Resettable for GLB_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
