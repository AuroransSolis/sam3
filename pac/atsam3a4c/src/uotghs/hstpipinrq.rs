#[doc = "Register `HSTPIPINRQ[%s]` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<HSTPIPINRQ_SPEC>);
#[doc = "Register `HSTPIPINRQ[%s]` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<HSTPIPINRQ_SPEC>);
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub type INRQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub type INRQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSTPIPINRQ_SPEC, u8, u8, 8, O>;
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub type INMODE_R = crate::BitReader<bool>;
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub type INMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPINRQ_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&self) -> INMODE_R {
        INMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> INRQ_W<0> {
        INRQ_W::new(self)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inmode(&mut self) -> INMODE_W<8> {
        INMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe IN Request Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipinrq](index.html) module"]
pub struct HSTPIPINRQ_SPEC;
impl crate::RegisterSpec for HSTPIPINRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpipinrq::R](R) reader structure"]
impl crate::Readable for HSTPIPINRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstpipinrq::W](W) writer structure"]
impl crate::Writable for HSTPIPINRQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
