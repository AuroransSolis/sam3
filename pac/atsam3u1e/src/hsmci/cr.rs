#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `MCIEN` writer - Multi-Media Interface Enable"]
pub type MCIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCIDIS` writer - Multi-Media Interface Disable"]
pub type MCIDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWSEN` writer - Power Save Mode Enable"]
pub type PWSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWSDIS` writer - Power Save Mode Disable"]
pub type PWSDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Multi-Media Interface Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcien(&mut self) -> MCIEN_W<CR_SPEC> {
        MCIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Multi-Media Interface Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mcidis(&mut self) -> MCIDIS_W<CR_SPEC> {
        MCIDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power Save Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwsen(&mut self) -> PWSEN_W<CR_SPEC> {
        PWSEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Power Save Mode Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pwsdis(&mut self) -> PWSDIS_W<CR_SPEC> {
        PWSDIS_W::new(self, 3)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR_SPEC> {
        SWRST_W::new(self, 7)
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
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
