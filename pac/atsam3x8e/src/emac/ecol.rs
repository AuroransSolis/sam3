#[doc = "Register `ECOL` reader"]
pub type R = crate::R<ECOL_SPEC>;
#[doc = "Register `ECOL` writer"]
pub type W = crate::W<ECOL_SPEC>;
#[doc = "Field `EXCOL` reader - Excessive Collisions"]
pub type EXCOL_R = crate::FieldReader;
#[doc = "Field `EXCOL` writer - Excessive Collisions"]
pub type EXCOL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    #[must_use]
    pub fn excol(&mut self) -> EXCOL_W<ECOL_SPEC, 0> {
        EXCOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Excessive Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECOL_SPEC;
impl crate::RegisterSpec for ECOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecol::R`](R) reader structure"]
impl crate::Readable for ECOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecol::W`](W) writer structure"]
impl crate::Writable for ECOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECOL to value 0"]
impl crate::Resettable for ECOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
