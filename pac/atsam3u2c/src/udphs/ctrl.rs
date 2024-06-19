#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `DEV_ADDR` reader - UDPHS Address"]
pub type DevAddrR = crate::FieldReader;
#[doc = "Field `DEV_ADDR` writer - UDPHS Address"]
pub type DevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FADDR_EN` reader - Function Address Enable"]
pub type FaddrEnR = crate::BitReader;
#[doc = "Field `FADDR_EN` writer - Function Address Enable"]
pub type FaddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_UDPHS` reader - UDPHS Enable"]
pub type EnUdphsR = crate::BitReader;
#[doc = "Field `EN_UDPHS` writer - UDPHS Enable"]
pub type EnUdphsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETACH` reader - Detach Command"]
pub type DetachR = crate::BitReader;
#[doc = "Field `DETACH` writer - Detach Command"]
pub type DetachW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REWAKEUP` reader - Send Remote Wake Up"]
pub type RewakeupR = crate::BitReader;
#[doc = "Field `REWAKEUP` writer - Send Remote Wake Up"]
pub type RewakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLD_DIS` reader - Pull-Down Disable"]
pub type PulldDisR = crate::BitReader;
#[doc = "Field `PULLD_DIS` writer - Pull-Down Disable"]
pub type PulldDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - UDPHS Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DevAddrR {
        DevAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Function Address Enable"]
    #[inline(always)]
    pub fn faddr_en(&self) -> FaddrEnR {
        FaddrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDPHS Enable"]
    #[inline(always)]
    pub fn en_udphs(&self) -> EnUdphsR {
        EnUdphsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Detach Command"]
    #[inline(always)]
    pub fn detach(&self) -> DetachR {
        DetachR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send Remote Wake Up"]
    #[inline(always)]
    pub fn rewakeup(&self) -> RewakeupR {
        RewakeupR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pull-Down Disable"]
    #[inline(always)]
    pub fn pulld_dis(&self) -> PulldDisR {
        PulldDisR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - UDPHS Address"]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DevAddrW<CtrlSpec> {
        DevAddrW::new(self, 0)
    }
    #[doc = "Bit 7 - Function Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn faddr_en(&mut self) -> FaddrEnW<CtrlSpec> {
        FaddrEnW::new(self, 7)
    }
    #[doc = "Bit 8 - UDPHS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en_udphs(&mut self) -> EnUdphsW<CtrlSpec> {
        EnUdphsW::new(self, 8)
    }
    #[doc = "Bit 9 - Detach Command"]
    #[inline(always)]
    #[must_use]
    pub fn detach(&mut self) -> DetachW<CtrlSpec> {
        DetachW::new(self, 9)
    }
    #[doc = "Bit 10 - Send Remote Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn rewakeup(&mut self) -> RewakeupW<CtrlSpec> {
        RewakeupW::new(self, 10)
    }
    #[doc = "Bit 11 - Pull-Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pulld_dis(&mut self) -> PulldDisW<CtrlSpec> {
        PulldDisW::new(self, 11)
    }
}
#[doc = "UDPHS Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0200"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0200;
}
