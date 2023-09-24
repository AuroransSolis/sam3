#[doc = "Register `PMC_USB` reader"]
pub type R = crate::R<PMC_USB_SPEC>;
#[doc = "Register `PMC_USB` writer"]
pub type W = crate::W<PMC_USB_SPEC>;
#[doc = "Field `USBS` reader - USB Input Clock Selection"]
pub type USBS_R = crate::BitReader;
#[doc = "Field `USBS` writer - USB Input Clock Selection"]
pub type USBS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBDIV` reader - Divider for USB Clock."]
pub type USBDIV_R = crate::FieldReader;
#[doc = "Field `USBDIV` writer - Divider for USB Clock."]
pub type USBDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    pub fn usbs(&self) -> USBS_R {
        USBS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Divider for USB Clock."]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB Input Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbs(&mut self) -> USBS_W<PMC_USB_SPEC, 0> {
        USBS_W::new(self)
    }
    #[doc = "Bits 8:11 - Divider for USB Clock."]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv(&mut self) -> USBDIV_W<PMC_USB_SPEC, 8> {
        USBDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_usb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_usb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_USB_SPEC;
impl crate::RegisterSpec for PMC_USB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_usb::R`](R) reader structure"]
impl crate::Readable for PMC_USB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmc_usb::W`](W) writer structure"]
impl crate::Writable for PMC_USB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC_USB to value 0"]
impl crate::Resettable for PMC_USB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
