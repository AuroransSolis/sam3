#[doc = "Register `CKGR_PLLBR` reader"]
pub type R = crate::R<CKGR_PLLBR_SPEC>;
#[doc = "Register `CKGR_PLLBR` writer"]
pub type W = crate::W<CKGR_PLLBR_SPEC>;
#[doc = "Field `DIVB` reader - Divider"]
pub type DIVB_R = crate::FieldReader;
#[doc = "Field `DIVB` writer - Divider"]
pub type DIVB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PLLBCOUNT` reader - PLLB Counter"]
pub type PLLBCOUNT_R = crate::FieldReader;
#[doc = "Field `PLLBCOUNT` writer - PLLB Counter"]
pub type PLLBCOUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `MULB` reader - PLLB Multiplier"]
pub type MULB_R = crate::FieldReader<u16>;
#[doc = "Field `MULB` writer - PLLB Multiplier"]
pub type MULB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&self) -> PLLBCOUNT_R {
        PLLBCOUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&self) -> MULB_R {
        MULB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DIVB_W<CKGR_PLLBR_SPEC, 0> {
        DIVB_W::new(self)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pllbcount(&mut self) -> PLLBCOUNT_W<CKGR_PLLBR_SPEC, 8> {
        PLLBCOUNT_W::new(self)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn mulb(&mut self) -> MULB_W<CKGR_PLLBR_SPEC, 16> {
        MULB_W::new(self)
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
#[doc = "PLLB Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_pllbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_pllbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKGR_PLLBR_SPEC;
impl crate::RegisterSpec for CKGR_PLLBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_pllbr::R`](R) reader structure"]
impl crate::Readable for CKGR_PLLBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckgr_pllbr::W`](W) writer structure"]
impl crate::Writable for CKGR_PLLBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKGR_PLLBR to value 0x3f00"]
impl crate::Resettable for CKGR_PLLBR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f00;
}
