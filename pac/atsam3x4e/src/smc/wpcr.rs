#[doc = "Register `WPCR` writer"]
pub type W = crate::W<WpcrSpec>;
#[doc = "Field `WP_EN` writer - Write Protection Enable"]
pub type WpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protection KEY Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WpKey {
    #[doc = "5459267: Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    Passwd = 5459267,
}
impl From<WpKey> for u32 {
    #[inline(always)]
    fn from(variant: WpKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WpKey {
    type Ux = u32;
}
impl crate::IsEnum for WpKey {}
#[doc = "Field `WP_KEY` writer - Write Protection KEY Password"]
pub type WpKeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, WpKey>;
impl<'a, REG> WpKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(WpKey::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wp_en(&mut self) -> WpEnW<WpcrSpec> {
        WpEnW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protection KEY Password"]
    #[inline(always)]
    #[must_use]
    pub fn wp_key(&mut self) -> WpKeyW<WpcrSpec> {
        WpKeyW::new(self, 8)
    }
}
#[doc = "Write Protection Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
