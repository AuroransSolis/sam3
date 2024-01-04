#[doc = "Register `PTR` reader"]
pub type R = crate::R<PTR_SPEC>;
#[doc = "Register `PTR` writer"]
pub type W = crate::W<PTR_SPEC>;
#[doc = "Field `PTIME` reader - Pause Time"]
pub type PTIME_R = crate::FieldReader<u16>;
#[doc = "Field `PTIME` writer - Pause Time"]
pub type PTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Time"]
    #[inline(always)]
    pub fn ptime(&self) -> PTIME_R {
        PTIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Time"]
    #[inline(always)]
    #[must_use]
    pub fn ptime(&mut self) -> PTIME_W<PTR_SPEC> {
        PTIME_W::new(self, 0)
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
#[doc = "Pause Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTR_SPEC;
impl crate::RegisterSpec for PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptr::R`](R) reader structure"]
impl crate::Readable for PTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptr::W`](W) writer structure"]
impl crate::Writable for PTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
