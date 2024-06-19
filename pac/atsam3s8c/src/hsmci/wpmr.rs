#[doc = "Register `WPMR` reader"]
pub type R = crate::R<WpmrSpec>;
#[doc = "Register `WPMR` writer"]
pub type W = crate::W<WpmrSpec>;
#[doc = "Field `WP_EN` reader - Write Protection Enable"]
pub type WpEnR = crate::BitReader;
#[doc = "Field `WP_EN` writer - Write Protection Enable"]
pub type WpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP_KEY` reader - Write Protection Key password"]
pub type WpKeyR = crate::FieldReader<u32>;
#[doc = "Field `WP_KEY` writer - Write Protection Key password"]
pub type WpKeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wp_en(&self) -> WpEnR {
        WpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key password"]
    #[inline(always)]
    pub fn wp_key(&self) -> WpKeyR {
        WpKeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wp_en(&mut self) -> WpEnW<WpmrSpec> {
        WpEnW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key password"]
    #[inline(always)]
    #[must_use]
    pub fn wp_key(&mut self) -> WpKeyW<WpmrSpec> {
        WpKeyW::new(self, 8)
    }
}
#[doc = "Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpmrSpec;
impl crate::RegisterSpec for WpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpmr::R`](R) reader structure"]
impl crate::Readable for WpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`wpmr::W`](W) writer structure"]
impl crate::Writable for WpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
