#[doc = "Register `LCOL` reader"]
pub type R = crate::R<LCOL_SPEC>;
#[doc = "Register `LCOL` writer"]
pub type W = crate::W<LCOL_SPEC>;
#[doc = "Field `LCOL` reader - Late Collisions"]
pub type LCOL_R = crate::FieldReader;
#[doc = "Field `LCOL` writer - Late Collisions"]
pub type LCOL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    #[must_use]
    pub fn lcol(&mut self) -> LCOL_W<LCOL_SPEC> {
        LCOL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Late Collisions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCOL_SPEC;
impl crate::RegisterSpec for LCOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcol::R`](R) reader structure"]
impl crate::Readable for LCOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcol::W`](W) writer structure"]
impl crate::Writable for LCOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCOL to value 0"]
impl crate::Resettable for LCOL_SPEC {
    const RESET_VALUE: u32 = 0;
}
