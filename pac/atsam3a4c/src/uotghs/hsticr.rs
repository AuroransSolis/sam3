#[doc = "Register `HSTICR` writer"]
pub type W = crate::W<HSTICR_SPEC>;
#[doc = "Field `DCONNIC` writer - Device Connection Interrupt Clear"]
pub type DCONNIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISCIC` writer - Device Disconnection Interrupt Clear"]
pub type DDISCIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIC` writer - USB Reset Sent Interrupt Clear"]
pub type RSTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMEDIC` writer - Downstream Resume Sent Interrupt Clear"]
pub type RSMEDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSMIC` writer - Upstream Resume Received Interrupt Clear"]
pub type RXRSMIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSOFIC` writer - Host Start of Frame Interrupt Clear"]
pub type HSOFIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWUPIC` writer - Host Wake-Up Interrupt Clear"]
pub type HWUPIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dconnic(&mut self) -> DCONNIC_W<HSTICR_SPEC> {
        DCONNIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscic(&mut self) -> DDISCIC_W<HSTICR_SPEC> {
        DDISCIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstic(&mut self) -> RSTIC_W<HSTICR_SPEC> {
        RSTIC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedic(&mut self) -> RSMEDIC_W<HSTICR_SPEC> {
        RSMEDIC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmic(&mut self) -> RXRSMIC_W<HSTICR_SPEC> {
        RXRSMIC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsofic(&mut self) -> HSOFIC_W<HSTICR_SPEC> {
        HSOFIC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hwupic(&mut self) -> HWUPIC_W<HSTICR_SPEC> {
        HWUPIC_W::new(self, 6)
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
#[doc = "Host Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsticr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTICR_SPEC;
impl crate::RegisterSpec for HSTICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hsticr::W`](W) writer structure"]
impl crate::Writable for HSTICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
