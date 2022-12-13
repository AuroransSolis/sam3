#[doc = "Register `RST_EP` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RST_EP_SPEC>);
#[doc = "Register `RST_EP` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RST_EP_SPEC>);
#[doc = "Field `EP0` reader - Reset Endpoint 0"]
pub type EP0_R = crate::BitReader<bool>;
#[doc = "Field `EP0` writer - Reset Endpoint 0"]
pub type EP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
#[doc = "Field `EP1` reader - Reset Endpoint 1"]
pub type EP1_R = crate::BitReader<bool>;
#[doc = "Field `EP1` writer - Reset Endpoint 1"]
pub type EP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
#[doc = "Field `EP2` reader - Reset Endpoint 2"]
pub type EP2_R = crate::BitReader<bool>;
#[doc = "Field `EP2` writer - Reset Endpoint 2"]
pub type EP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
#[doc = "Field `EP3` reader - Reset Endpoint 3"]
pub type EP3_R = crate::BitReader<bool>;
#[doc = "Field `EP3` writer - Reset Endpoint 3"]
pub type EP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
#[doc = "Field `EP4` reader - Reset Endpoint 4"]
pub type EP4_R = crate::BitReader<bool>;
#[doc = "Field `EP4` writer - Reset Endpoint 4"]
pub type EP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
#[doc = "Field `EP5` reader - Reset Endpoint 5"]
pub type EP5_R = crate::BitReader<bool>;
#[doc = "Field `EP5` writer - Reset Endpoint 5"]
pub type EP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
#[doc = "Field `EP6` reader - Reset Endpoint 6"]
pub type EP6_R = crate::BitReader<bool>;
#[doc = "Field `EP6` writer - Reset Endpoint 6"]
pub type EP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
#[doc = "Field `EP7` reader - Reset Endpoint 7"]
pub type EP7_R = crate::BitReader<bool>;
#[doc = "Field `EP7` writer - Reset Endpoint 7"]
pub type EP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reset Endpoint 0"]
    #[inline(always)]
    pub fn ep0(&self) -> EP0_R {
        EP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset Endpoint 1"]
    #[inline(always)]
    pub fn ep1(&self) -> EP1_R {
        EP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset Endpoint 2"]
    #[inline(always)]
    pub fn ep2(&self) -> EP2_R {
        EP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Endpoint 3"]
    #[inline(always)]
    pub fn ep3(&self) -> EP3_R {
        EP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Endpoint 4"]
    #[inline(always)]
    pub fn ep4(&self) -> EP4_R {
        EP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset Endpoint 5"]
    #[inline(always)]
    pub fn ep5(&self) -> EP5_R {
        EP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Endpoint 6"]
    #[inline(always)]
    pub fn ep6(&self) -> EP6_R {
        EP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset Endpoint 7"]
    #[inline(always)]
    pub fn ep7(&self) -> EP7_R {
        EP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Endpoint 0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0(&mut self) -> EP0_W<0> {
        EP0_W::new(self)
    }
    #[doc = "Bit 1 - Reset Endpoint 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep1(&mut self) -> EP1_W<1> {
        EP1_W::new(self)
    }
    #[doc = "Bit 2 - Reset Endpoint 2"]
    #[inline(always)]
    #[must_use]
    pub fn ep2(&mut self) -> EP2_W<2> {
        EP2_W::new(self)
    }
    #[doc = "Bit 3 - Reset Endpoint 3"]
    #[inline(always)]
    #[must_use]
    pub fn ep3(&mut self) -> EP3_W<3> {
        EP3_W::new(self)
    }
    #[doc = "Bit 4 - Reset Endpoint 4"]
    #[inline(always)]
    #[must_use]
    pub fn ep4(&mut self) -> EP4_W<4> {
        EP4_W::new(self)
    }
    #[doc = "Bit 5 - Reset Endpoint 5"]
    #[inline(always)]
    #[must_use]
    pub fn ep5(&mut self) -> EP5_W<5> {
        EP5_W::new(self)
    }
    #[doc = "Bit 6 - Reset Endpoint 6"]
    #[inline(always)]
    #[must_use]
    pub fn ep6(&mut self) -> EP6_W<6> {
        EP6_W::new(self)
    }
    #[doc = "Bit 7 - Reset Endpoint 7"]
    #[inline(always)]
    #[must_use]
    pub fn ep7(&mut self) -> EP7_W<7> {
        EP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_ep](index.html) module"]
pub struct RST_EP_SPEC;
impl crate::RegisterSpec for RST_EP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_ep::R](R) reader structure"]
impl crate::Readable for RST_EP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_ep::W](W) writer structure"]
impl crate::Writable for RST_EP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST_EP to value 0"]
impl crate::Resettable for RST_EP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
