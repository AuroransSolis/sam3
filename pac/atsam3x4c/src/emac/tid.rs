#[doc = "Register `TID` reader"]
pub type R = crate::R<TID_SPEC>;
#[doc = "Register `TID` writer"]
pub type W = crate::W<TID_SPEC>;
#[doc = "Field `TID` reader - Type ID checking"]
pub type TID_R = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID checking"]
pub type TID_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Type ID checking"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID checking"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<TID_SPEC> {
        TID_W::new(self, 0)
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
#[doc = "Type ID Checking Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TID_SPEC;
impl crate::RegisterSpec for TID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tid::R`](R) reader structure"]
impl crate::Readable for TID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tid::W`](W) writer structure"]
impl crate::Writable for TID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TID to value 0"]
impl crate::Resettable for TID_SPEC {
    const RESET_VALUE: u32 = 0;
}
