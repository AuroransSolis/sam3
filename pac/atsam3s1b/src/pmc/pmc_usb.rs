#[doc = "Register `PMC_USB` reader"]
pub type R = crate::R<PmcUsbSpec>;
#[doc = "Register `PMC_USB` writer"]
pub type W = crate::W<PmcUsbSpec>;
#[doc = "Field `USBS` reader - USB Input Clock Selection"]
pub type UsbsR = crate::BitReader;
#[doc = "Field `USBS` writer - USB Input Clock Selection"]
pub type UsbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBDIV` reader - Divider for USB Clock."]
pub type UsbdivR = crate::FieldReader;
#[doc = "Field `USBDIV` writer - Divider for USB Clock."]
pub type UsbdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    pub fn usbs(&self) -> UsbsR {
        UsbsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Divider for USB Clock."]
    #[inline(always)]
    pub fn usbdiv(&self) -> UsbdivR {
        UsbdivR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbs(&mut self) -> UsbsW<PmcUsbSpec> {
        UsbsW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Divider for USB Clock."]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv(&mut self) -> UsbdivW<PmcUsbSpec> {
        UsbdivW::new(self, 8)
    }
}
#[doc = "USB Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_usb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_usb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcUsbSpec;
impl crate::RegisterSpec for PmcUsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_usb::R`](R) reader structure"]
impl crate::Readable for PmcUsbSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_usb::W`](W) writer structure"]
impl crate::Writable for PmcUsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_USB to value 0"]
impl crate::Resettable for PmcUsbSpec {
    const RESET_VALUE: u32 = 0;
}
