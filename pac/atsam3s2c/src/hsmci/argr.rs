#[doc = "Register `ARGR` reader"]
pub type R = crate::R<ARGR_SPEC>;
#[doc = "Register `ARGR` writer"]
pub type W = crate::W<ARGR_SPEC>;
#[doc = "Field `ARG` reader - Command Argument"]
pub type ARG_R = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - Command Argument"]
pub type ARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<ARGR_SPEC> {
        ARG_W::new(self, 0)
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
#[doc = "Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARGR_SPEC;
impl crate::RegisterSpec for ARGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argr::R`](R) reader structure"]
impl crate::Readable for ARGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`argr::W`](W) writer structure"]
impl crate::Writable for ARGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARGR to value 0"]
impl crate::Resettable for ARGR_SPEC {
    const RESET_VALUE: u32 = 0;
}
