#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `MFD` writer - Management Frame sent"]
pub type MFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RCOMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUBR` writer - Receive Used Bit Read"]
pub type RXUBR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUBR` writer - Transmit Used Bit Read"]
pub type TXUBR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUND` writer - Ethernet Transmit Buffer Underrun"]
pub type TUND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLE` writer - Retry Limit Exceeded"]
pub type RLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` writer - "]
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TCOMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` writer - Hresp not OK"]
pub type HRESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFR` writer - Pause Frame Received"]
pub type PFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PTZ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Management Frame sent"]
    #[inline(always)]
    #[must_use]
    pub fn mfd(&mut self) -> MFD_W<IDR_SPEC> {
        MFD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RCOMP_W<IDR_SPEC> {
        RCOMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RXUBR_W<IDR_SPEC> {
        RXUBR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn txubr(&mut self) -> TXUBR_W<IDR_SPEC> {
        TXUBR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tund(&mut self) -> TUND_W<IDR_SPEC> {
        TUND_W::new(self, 4)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rle(&mut self) -> RLE_W<IDR_SPEC> {
        RLE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<IDR_SPEC> {
        TXERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TCOMP_W<IDR_SPEC> {
        TCOMP_W::new(self, 7)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<IDR_SPEC> {
        ROVR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HRESP_W<IDR_SPEC> {
        HRESP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pfr(&mut self) -> PFR_W<IDR_SPEC> {
        PFR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    #[must_use]
    pub fn ptz(&mut self) -> PTZ_W<IDR_SPEC> {
        PTZ_W::new(self, 13)
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
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
