#[doc = "Register `DSCR` reader"]
pub type R = crate::R<DSCR_SPEC>;
#[doc = "Register `DSCR` writer"]
pub type W = crate::W<DSCR_SPEC>;
#[doc = "Field `DSCR` reader - Descriptor Base Address"]
pub type DSCR_R = crate::FieldReader<u32>;
#[doc = "Field `DSCR` writer - Descriptor Base Address"]
pub type DSCR_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 9:31 - Descriptor Base Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 9:31 - Descriptor Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn dscr(&mut self) -> DSCR_W<DSCR_SPEC> {
        DSCR_W::new(self, 9)
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
#[doc = "CRCCU Descriptor Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSCR_SPEC;
impl crate::RegisterSpec for DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr::R`](R) reader structure"]
impl crate::Readable for DSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dscr::W`](W) writer structure"]
impl crate::Writable for DSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSCR to value 0"]
impl crate::Resettable for DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
