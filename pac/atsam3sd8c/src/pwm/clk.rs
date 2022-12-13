#[doc = "Register `CLK` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CLK_SPEC>);
#[doc = "Register `CLK` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CLK_SPEC>);
#[doc = "Field `DIVA` reader - CLKA, CLKB Divide Factor"]
pub type DIVA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVA` writer - CLKA, CLKB Divide Factor"]
pub type DIVA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `PREA` reader - CLKA, CLKB Source Clock Selection"]
pub type PREA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREA` writer - CLKA, CLKB Source Clock Selection"]
pub type PREA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIVB` reader - CLKA, CLKB Divide Factor"]
pub type DIVB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVB` writer - CLKA, CLKB Divide Factor"]
pub type DIVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `PREB` reader - CLKA, CLKB Source Clock Selection"]
pub type PREB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREB` writer - CLKA, CLKB Source Clock Selection"]
pub type PREB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&self) -> PREA_R {
        PREA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&self) -> PREB_R {
        PREB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<0> {
        DIVA_W::new(self)
    }
    #[doc = "Bits 8:11 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prea(&mut self) -> PREA_W<8> {
        PREA_W::new(self)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DIVB_W<16> {
        DIVB_W::new(self)
    }
    #[doc = "Bits 24:27 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn preb(&mut self) -> PREB_W<24> {
        PREB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](index.html) module"]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk::R](R) reader structure"]
impl crate::Readable for CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk::W](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
