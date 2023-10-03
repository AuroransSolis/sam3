#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISR_SPEC>;
#[doc = "Field `MFD` reader - Management Frame Done"]
pub type MFD_R = crate::BitReader;
#[doc = "Field `MFD` writer - Management Frame Done"]
pub type MFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RCOMP_R = crate::BitReader;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUBR` reader - Receive Used Bit Read"]
pub type RXUBR_R = crate::BitReader;
#[doc = "Field `RXUBR` writer - Receive Used Bit Read"]
pub type RXUBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUBR` reader - Transmit Used Bit Read"]
pub type TXUBR_R = crate::BitReader;
#[doc = "Field `TXUBR` writer - Transmit Used Bit Read"]
pub type TXUBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TUND` reader - Ethernet Transmit Buffer Underrun"]
pub type TUND_R = crate::BitReader;
#[doc = "Field `TUND` writer - Ethernet Transmit Buffer Underrun"]
pub type TUND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded"]
pub type RLEX_R = crate::BitReader;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded"]
pub type RLEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXERR` reader - Transmit Error"]
pub type TXERR_R = crate::BitReader;
#[doc = "Field `TXERR` writer - Transmit Error"]
pub type TXERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TCOMP_R = crate::BitReader;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type ROVR_R = crate::BitReader;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HRESP` reader - Hresp not OK"]
pub type HRESP_R = crate::BitReader;
#[doc = "Field `HRESP` writer - Hresp not OK"]
pub type HRESP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFRE` reader - Pause Frame Received"]
pub type PFRE_R = crate::BitReader;
#[doc = "Field `PFRE` writer - Pause Frame Received"]
pub type PFRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub type PTZ_R = crate::BitReader;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PTZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Management Frame Done"]
    #[inline(always)]
    pub fn mfd(&self) -> MFD_R {
        MFD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TXUBR_R {
        TXUBR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Error"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    pub fn pfre(&self) -> PFRE_R {
        PFRE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PTZ_R {
        PTZ_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management Frame Done"]
    #[inline(always)]
    #[must_use]
    pub fn mfd(&mut self) -> MFD_W<ISR_SPEC, 0> {
        MFD_W::new(self)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RCOMP_W<ISR_SPEC, 1> {
        RCOMP_W::new(self)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RXUBR_W<ISR_SPEC, 2> {
        RXUBR_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn txubr(&mut self) -> TXUBR_W<ISR_SPEC, 3> {
        TXUBR_W::new(self)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tund(&mut self) -> TUND_W<ISR_SPEC, 4> {
        TUND_W::new(self)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rlex(&mut self) -> RLEX_W<ISR_SPEC, 5> {
        RLEX_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Error"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<ISR_SPEC, 6> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TCOMP_W<ISR_SPEC, 7> {
        TCOMP_W::new(self)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<ISR_SPEC, 10> {
        ROVR_W::new(self)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HRESP_W<ISR_SPEC, 11> {
        HRESP_W::new(self)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pfre(&mut self) -> PFRE_W<ISR_SPEC, 12> {
        PFRE_W::new(self)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    #[must_use]
    pub fn ptz(&mut self) -> PTZ_W<ISR_SPEC, 13> {
        PTZ_W::new(self)
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
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
