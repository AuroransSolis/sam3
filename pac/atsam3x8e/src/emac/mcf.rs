#[doc = "Register `MCF` reader"]
pub type R = crate::R<MCF_SPEC>;
#[doc = "Register `MCF` writer"]
pub type W = crate::W<MCF_SPEC>;
#[doc = "Field `MCF` reader - Multicollision Frames"]
pub type MCF_R = crate::FieldReader<u16>;
#[doc = "Field `MCF` writer - Multicollision Frames"]
pub type MCF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<MCF_SPEC> {
        MCF_W::new(self, 0)
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
#[doc = "Multiple Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCF_SPEC;
impl crate::RegisterSpec for MCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcf::R`](R) reader structure"]
impl crate::Readable for MCF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcf::W`](W) writer structure"]
impl crate::Writable for MCF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCF to value 0"]
impl crate::Resettable for MCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
