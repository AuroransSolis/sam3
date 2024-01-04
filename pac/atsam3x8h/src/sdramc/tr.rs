#[doc = "Register `TR` reader"]
pub type R = crate::R<TR_SPEC>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TR_SPEC>;
#[doc = "Field `COUNT` reader - SDRAMC Refresh Timer Count"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - SDRAMC Refresh Timer Count"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - SDRAMC Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - SDRAMC Refresh Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<TR_SPEC> {
        COUNT_W::new(self, 0)
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
#[doc = "SDRAMC Refresh Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    const RESET_VALUE: u32 = 0;
}
