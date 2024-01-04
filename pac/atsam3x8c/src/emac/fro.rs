#[doc = "Register `FRO` reader"]
pub type R = crate::R<FRO_SPEC>;
#[doc = "Register `FRO` writer"]
pub type W = crate::W<FRO_SPEC>;
#[doc = "Field `FROK` reader - Frames Received OK"]
pub type FROK_R = crate::FieldReader<u32>;
#[doc = "Field `FROK` writer - Frames Received OK"]
pub type FROK_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FROK_R {
        FROK_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    #[must_use]
    pub fn frok(&mut self) -> FROK_W<FRO_SPEC> {
        FROK_W::new(self, 0)
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
#[doc = "Frames Received Ok Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fro::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fro::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRO_SPEC;
impl crate::RegisterSpec for FRO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fro::R`](R) reader structure"]
impl crate::Readable for FRO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fro::W`](W) writer structure"]
impl crate::Writable for FRO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRO to value 0"]
impl crate::Resettable for FRO_SPEC {
    const RESET_VALUE: u32 = 0;
}
