#[doc = "Register `MDR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MDR_SPEC>);
#[doc = "Register `MDR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MDR_SPEC>);
#[doc = "Field `MD` reader - Memory Device Type"]
pub type MD_R = crate::FieldReader<u8, MD_A>;
#[doc = "Memory Device Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: SDRAM"]
    Sdram = 0,
    #[doc = "1: Low-power SDRAM"]
    Lpsdram = 1,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
impl MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MD_A> {
        match self.bits {
            0 => Some(MD_A::Sdram),
            1 => Some(MD_A::Lpsdram),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Sdram`"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == MD_A::Sdram
    }
    #[doc = "Checks if the value of the field is `Lpsdram`"]
    #[inline(always)]
    pub fn is_lpsdram(&self) -> bool {
        *self == MD_A::Lpsdram
    }
}
#[doc = "Field `MD` writer - Memory Device Type"]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDR_SPEC, u8, MD_A, 2, O>;
impl<'a, const O: u8> MD_W<'a, O> {
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MD_A::Sdram)
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn lpsdram(self) -> &'a mut W {
        self.variant(MD_A::Lpsdram)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Memory Device Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](index.html) module"]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdr::R](R) reader structure"]
impl crate::Readable for MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdr::W](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
