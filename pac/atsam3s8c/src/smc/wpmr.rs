#[doc = "Register `WPMR` reader"]
pub type R = crate::R<WPMR_SPEC>;
#[doc = "Register `WPMR` writer"]
pub type W = crate::W<WPMR_SPEC>;
#[doc = "Field `WPEN` reader - Write Protect Enable"]
pub type WPEN_R = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protect Enable"]
pub type WPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` reader - Write Protect KEY"]
pub type WPKEY_R = crate::FieldReader<u32>;
#[doc = "Field `WPKEY` writer - Write Protect KEY"]
pub type WPKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<WPMR_SPEC> {
        WPEN_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<WPMR_SPEC> {
        WPKEY_W::new(self, 8)
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
#[doc = "SMC Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPMR_SPEC;
impl crate::RegisterSpec for WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpmr::R`](R) reader structure"]
impl crate::Readable for WPMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wpmr::W`](W) writer structure"]
impl crate::Writable for WPMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPMR to value 0"]
impl crate::Resettable for WPMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
