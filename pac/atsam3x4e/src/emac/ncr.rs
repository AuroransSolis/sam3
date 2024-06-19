#[doc = "Register `NCR` reader"]
pub type R = crate::R<NcrSpec>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NcrSpec>;
#[doc = "Field `LB` reader - LoopBack"]
pub type LbR = crate::BitReader;
#[doc = "Field `LB` writer - LoopBack"]
pub type LbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLB` reader - Loopback local"]
pub type LlbR = crate::BitReader;
#[doc = "Field `LLB` writer - Loopback local"]
pub type LlbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Receive enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receive enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmit enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmit enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPE` reader - Management port enable"]
pub type MpeR = crate::BitReader;
#[doc = "Field `MPE` writer - Management port enable"]
pub type MpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRSTAT` reader - Clear statistics registers"]
pub type ClrstatR = crate::BitReader;
#[doc = "Field `CLRSTAT` writer - Clear statistics registers"]
pub type ClrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCSTAT` reader - Increment statistics registers"]
pub type IncstatR = crate::BitReader;
#[doc = "Field `INCSTAT` writer - Increment statistics registers"]
pub type IncstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WESTAT` reader - Write enable for statistics registers"]
pub type WestatR = crate::BitReader;
#[doc = "Field `WESTAT` writer - Write enable for statistics registers"]
pub type WestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BP` reader - Back pressure"]
pub type BpR = crate::BitReader;
#[doc = "Field `BP` writer - Back pressure"]
pub type BpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTART` reader - Start transmission"]
pub type TstartR = crate::BitReader;
#[doc = "Field `TSTART` writer - Start transmission"]
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THALT` reader - Transmit halt"]
pub type ThaltR = crate::BitReader;
#[doc = "Field `THALT` writer - Transmit halt"]
pub type ThaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LoopBack"]
    #[inline(always)]
    pub fn lb(&self) -> LbR {
        LbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn llb(&self) -> LlbR {
        LlbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MpeR {
        MpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrstat(&self) -> ClrstatR {
        ClrstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Increment statistics registers"]
    #[inline(always)]
    pub fn incstat(&self) -> IncstatR {
        IncstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn westat(&self) -> WestatR {
        WestatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&self) -> BpR {
        BpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn thalt(&self) -> ThaltR {
        ThaltR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LoopBack"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LbW<NcrSpec> {
        LbW::new(self, 0)
    }
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    #[must_use]
    pub fn llb(&mut self) -> LlbW<NcrSpec> {
        LlbW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<NcrSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<NcrSpec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MpeW<NcrSpec> {
        MpeW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn clrstat(&mut self) -> ClrstatW<NcrSpec> {
        ClrstatW::new(self, 5)
    }
    #[doc = "Bit 6 - Increment statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn incstat(&mut self) -> IncstatW<NcrSpec> {
        IncstatW::new(self, 6)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn westat(&mut self) -> WestatW<NcrSpec> {
        WestatW::new(self, 7)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BpW<NcrSpec> {
        BpW::new(self, 8)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TstartW<NcrSpec> {
        TstartW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    #[must_use]
    pub fn thalt(&mut self) -> ThaltW<NcrSpec> {
        ThaltW::new(self, 10)
    }
}
#[doc = "Network Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcrSpec;
impl crate::RegisterSpec for NcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NcrSpec {
    const RESET_VALUE: u32 = 0;
}
