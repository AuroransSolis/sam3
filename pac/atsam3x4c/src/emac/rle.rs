#[doc = "Register `RLE` reader"]
pub type R = crate::R<RLE_SPEC>;
#[doc = "Register `RLE` writer"]
pub type W = crate::W<RLE_SPEC>;
#[doc = "Field `RLFM` reader - Receive Length Field Mismatch"]
pub type RLFM_R = crate::FieldReader;
#[doc = "Field `RLFM` writer - Receive Length Field Mismatch"]
pub type RLFM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    pub fn rlfm(&self) -> RLFM_R {
        RLFM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn rlfm(&mut self) -> RLFM_W<RLE_SPEC, 0> {
        RLFM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Received Length Field Mismatch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLE_SPEC;
impl crate::RegisterSpec for RLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rle::R`](R) reader structure"]
impl crate::Readable for RLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rle::W`](W) writer structure"]
impl crate::Writable for RLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLE to value 0"]
impl crate::Resettable for RLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
