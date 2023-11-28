#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDIS` writer - SPI Disable"]
pub type SPIDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - SPI Software Reset"]
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LASTXFER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<CR_SPEC> {
        SPIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Disable"]
    #[inline(always)]
    #[must_use]
    pub fn spidis(&mut self) -> SPIDIS_W<CR_SPEC> {
        SPIDIS_W::new(self, 1)
    }
    #[doc = "Bit 7 - SPI Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR_SPEC> {
        SWRST_W::new(self, 7)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn lastxfer(&mut self) -> LASTXFER_W<CR_SPEC> {
        LASTXFER_W::new(self, 24)
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
