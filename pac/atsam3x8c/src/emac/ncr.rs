#[doc = "Register `NCR` reader"]
pub type R = crate::R<NCR_SPEC>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NCR_SPEC>;
#[doc = "Field `LB` reader - LoopBack"]
pub type LB_R = crate::BitReader;
#[doc = "Field `LB` writer - LoopBack"]
pub type LB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LLB` reader - Loopback local"]
pub type LLB_R = crate::BitReader;
#[doc = "Field `LLB` writer - Loopback local"]
pub type LLB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RE` reader - Receive enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receive enable"]
pub type RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TE` reader - Transmit enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmit enable"]
pub type TE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPE` reader - Management port enable"]
pub type MPE_R = crate::BitReader;
#[doc = "Field `MPE` writer - Management port enable"]
pub type MPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLRSTAT` reader - Clear statistics registers"]
pub type CLRSTAT_R = crate::BitReader;
#[doc = "Field `CLRSTAT` writer - Clear statistics registers"]
pub type CLRSTAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCSTAT` reader - Increment statistics registers"]
pub type INCSTAT_R = crate::BitReader;
#[doc = "Field `INCSTAT` writer - Increment statistics registers"]
pub type INCSTAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WESTAT` reader - Write enable for statistics registers"]
pub type WESTAT_R = crate::BitReader;
#[doc = "Field `WESTAT` writer - Write enable for statistics registers"]
pub type WESTAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BP` reader - Back pressure"]
pub type BP_R = crate::BitReader;
#[doc = "Field `BP` writer - Back pressure"]
pub type BP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSTART` reader - Start transmission"]
pub type TSTART_R = crate::BitReader;
#[doc = "Field `TSTART` writer - Start transmission"]
pub type TSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THALT` reader - Transmit halt"]
pub type THALT_R = crate::BitReader;
#[doc = "Field `THALT` writer - Transmit halt"]
pub type THALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LoopBack"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrstat(&self) -> CLRSTAT_R {
        CLRSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Increment statistics registers"]
    #[inline(always)]
    pub fn incstat(&self) -> INCSTAT_R {
        INCSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn westat(&self) -> WESTAT_R {
        WESTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn thalt(&self) -> THALT_R {
        THALT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LoopBack"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LB_W<NCR_SPEC, 0> {
        LB_W::new(self)
    }
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    #[must_use]
    pub fn llb(&mut self) -> LLB_W<NCR_SPEC, 1> {
        LLB_W::new(self)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<NCR_SPEC, 2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<NCR_SPEC, 3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<NCR_SPEC, 4> {
        MPE_W::new(self)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn clrstat(&mut self) -> CLRSTAT_W<NCR_SPEC, 5> {
        CLRSTAT_W::new(self)
    }
    #[doc = "Bit 6 - Increment statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn incstat(&mut self) -> INCSTAT_W<NCR_SPEC, 6> {
        INCSTAT_W::new(self)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn westat(&mut self) -> WESTAT_W<NCR_SPEC, 7> {
        WESTAT_W::new(self)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<NCR_SPEC, 8> {
        BP_W::new(self)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<NCR_SPEC, 9> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    #[must_use]
    pub fn thalt(&mut self) -> THALT_W<NCR_SPEC, 10> {
        THALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Network Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCR_SPEC;
impl crate::RegisterSpec for NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
