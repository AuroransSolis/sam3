#[doc = "Register `DSCR5` reader"]
pub type R = crate::R<DSCR5_SPEC>;
#[doc = "Register `DSCR5` writer"]
pub type W = crate::W<DSCR5_SPEC>;
#[doc = "Field `DSCR` reader - Buffer Transfer Descriptor Address"]
pub type DSCR_R = crate::FieldReader<u32>;
#[doc = "Field `DSCR` writer - Buffer Transfer Descriptor Address"]
pub type DSCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn dscr(&mut self) -> DSCR_W<DSCR5_SPEC, 2> {
        DSCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSCR5_SPEC;
impl crate::RegisterSpec for DSCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr5::R`](R) reader structure"]
impl crate::Readable for DSCR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dscr5::W`](W) writer structure"]
impl crate::Writable for DSCR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSCR5 to value 0"]
impl crate::Resettable for DSCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
