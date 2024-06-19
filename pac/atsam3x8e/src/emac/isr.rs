#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `MFD` reader - Management Frame Done"]
pub type MfdR = crate::BitReader;
#[doc = "Field `MFD` writer - Management Frame Done"]
pub type MfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RcompR = crate::BitReader;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUBR` reader - Receive Used Bit Read"]
pub type RxubrR = crate::BitReader;
#[doc = "Field `RXUBR` writer - Receive Used Bit Read"]
pub type RxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUBR` reader - Transmit Used Bit Read"]
pub type TxubrR = crate::BitReader;
#[doc = "Field `TXUBR` writer - Transmit Used Bit Read"]
pub type TxubrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUND` reader - Ethernet Transmit Buffer Underrun"]
pub type TundR = crate::BitReader;
#[doc = "Field `TUND` writer - Ethernet Transmit Buffer Underrun"]
pub type TundW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded"]
pub type RlexR = crate::BitReader;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded"]
pub type RlexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - Transmit Error"]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - Transmit Error"]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TcompR = crate::BitReader;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type RovrR = crate::BitReader;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type RovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESP` reader - Hresp not OK"]
pub type HrespR = crate::BitReader;
#[doc = "Field `HRESP` writer - Hresp not OK"]
pub type HrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRE` reader - Pause Frame Received"]
pub type PfreR = crate::BitReader;
#[doc = "Field `PFRE` writer - Pause Frame Received"]
pub type PfreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub type PtzR = crate::BitReader;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PtzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Management Frame Done"]
    #[inline(always)]
    pub fn mfd(&self) -> MfdR {
        MfdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RcompR {
        RcompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RxubrR {
        RxubrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TxubrR {
        TxubrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    pub fn tund(&self) -> TundR {
        TundR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rlex(&self) -> RlexR {
        RlexR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Error"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TcompR {
        TcompR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HrespR {
        HrespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    pub fn pfre(&self) -> PfreR {
        PfreR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PtzR {
        PtzR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management Frame Done"]
    #[inline(always)]
    #[must_use]
    pub fn mfd(&mut self) -> MfdW<IsrSpec> {
        MfdW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    #[must_use]
    pub fn rcomp(&mut self) -> RcompW<IsrSpec> {
        RcompW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn rxubr(&mut self) -> RxubrW<IsrSpec> {
        RxubrW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn txubr(&mut self) -> TxubrW<IsrSpec> {
        TxubrW::new(self, 3)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn tund(&mut self) -> TundW<IsrSpec> {
        TundW::new(self, 4)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rlex(&mut self) -> RlexW<IsrSpec> {
        RlexW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Error"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TxerrW<IsrSpec> {
        TxerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp(&mut self) -> TcompW<IsrSpec> {
        TcompW::new(self, 7)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> RovrW<IsrSpec> {
        RovrW::new(self, 10)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    #[must_use]
    pub fn hresp(&mut self) -> HrespW<IsrSpec> {
        HrespW::new(self, 11)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    #[must_use]
    pub fn pfre(&mut self) -> PfreW<IsrSpec> {
        PfreW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    #[must_use]
    pub fn ptz(&mut self) -> PtzW<IsrSpec> {
        PtzW::new(self, 13)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
