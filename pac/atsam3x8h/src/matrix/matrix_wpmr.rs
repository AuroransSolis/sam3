#[doc = "Register `MATRIX_WPMR` reader"]
pub type R = crate::R<MATRIX_WPMR_SPEC>;
#[doc = "Register `MATRIX_WPMR` writer"]
pub type W = crate::W<MATRIX_WPMR_SPEC>;
#[doc = "Field `WPEN` reader - Write Protect ENable"]
pub type WPEN_R = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protect ENable"]
pub type WPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` reader - Write Protect KEY (Write-only)"]
pub type WPKEY_R = crate::FieldReader<u32>;
#[doc = "Field `WPKEY` writer - Write Protect KEY (Write-only)"]
pub type WPKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<MATRIX_WPMR_SPEC> {
        WPEN_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<MATRIX_WPMR_SPEC> {
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
#[doc = "Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_wpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MATRIX_WPMR_SPEC;
impl crate::RegisterSpec for MATRIX_WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_wpmr::R`](R) reader structure"]
impl crate::Readable for MATRIX_WPMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`matrix_wpmr::W`](W) writer structure"]
impl crate::Writable for MATRIX_WPMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATRIX_WPMR to value 0"]
impl crate::Resettable for MATRIX_WPMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
