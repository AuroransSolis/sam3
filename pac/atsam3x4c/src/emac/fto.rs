#[doc = "Register `FTO` reader"]
pub type R = crate::R<FTO_SPEC>;
#[doc = "Register `FTO` writer"]
pub type W = crate::W<FTO_SPEC>;
#[doc = "Field `FTOK` reader - Frames Transmitted OK"]
pub type FTOK_R = crate::FieldReader<u32>;
#[doc = "Field `FTOK` writer - Frames Transmitted OK"]
pub type FTOK_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    pub fn ftok(&self) -> FTOK_R {
        FTOK_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    #[must_use]
    pub fn ftok(&mut self) -> FTOK_W<FTO_SPEC> {
        FTOK_W::new(self, 0)
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
#[doc = "Frames Transmitted Ok Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fto::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fto::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTO_SPEC;
impl crate::RegisterSpec for FTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fto::R`](R) reader structure"]
impl crate::Readable for FTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fto::W`](W) writer structure"]
impl crate::Writable for FTO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTO to value 0"]
impl crate::Resettable for FTO_SPEC {
    const RESET_VALUE: u32 = 0;
}
