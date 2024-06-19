#[doc = "Register `WPCR` writer"]
pub type W = crate::W<WpcrSpec>;
#[doc = "Field `WP_EN` writer - Write Protection Enable"]
pub type WpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP_KEY` writer - Write Protection KEY password"]
pub type WpKeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wp_en(&mut self) -> WpEnW<WpcrSpec> {
        WpEnW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protection KEY password"]
    #[inline(always)]
    #[must_use]
    pub fn wp_key(&mut self) -> WpKeyW<WpcrSpec> {
        WpKeyW::new(self, 8)
    }
}
#[doc = "Write Protection Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpcrSpec;
impl crate::RegisterSpec for WpcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpcr::W`](W) writer structure"]
impl crate::Writable for WpcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR to value 0"]
impl crate::Resettable for WpcrSpec {
    const RESET_VALUE: u32 = 0;
}
