#[doc = "Register `HSTICR` writer"]
pub type W = crate::W<HsticrSpec>;
#[doc = "Field `DCONNIC` writer - Device Connection Interrupt Clear"]
pub type DconnicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISCIC` writer - Device Disconnection Interrupt Clear"]
pub type DdiscicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIC` writer - USB Reset Sent Interrupt Clear"]
pub type RsticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMEDIC` writer - Downstream Resume Sent Interrupt Clear"]
pub type RsmedicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSMIC` writer - Upstream Resume Received Interrupt Clear"]
pub type RxrsmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSOFIC` writer - Host Start of Frame Interrupt Clear"]
pub type HsoficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWUPIC` writer - Host Wake-Up Interrupt Clear"]
pub type HwupicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dconnic(&mut self) -> DconnicW<HsticrSpec> {
        DconnicW::new(self, 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscic(&mut self) -> DdiscicW<HsticrSpec> {
        DdiscicW::new(self, 1)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstic(&mut self) -> RsticW<HsticrSpec> {
        RsticW::new(self, 2)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedic(&mut self) -> RsmedicW<HsticrSpec> {
        RsmedicW::new(self, 3)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmic(&mut self) -> RxrsmicW<HsticrSpec> {
        RxrsmicW::new(self, 4)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsofic(&mut self) -> HsoficW<HsticrSpec> {
        HsoficW::new(self, 5)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hwupic(&mut self) -> HwupicW<HsticrSpec> {
        HwupicW::new(self, 6)
    }
}
#[doc = "Host Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsticr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsticrSpec;
impl crate::RegisterSpec for HsticrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hsticr::W`](W) writer structure"]
impl crate::Writable for HsticrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
