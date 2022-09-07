#[doc = "Register `NCR` reader"]
pub struct R(crate::R<NCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NCR` writer"]
pub struct W(crate::W<NCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LB` reader - LoopBack"]
pub type LB_R = crate::BitReader<bool>;
#[doc = "Field `LB` writer - LoopBack"]
pub type LB_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `LLB` reader - Loopback local"]
pub type LLB_R = crate::BitReader<bool>;
#[doc = "Field `LLB` writer - Loopback local"]
pub type LLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `RE` reader - Receive enable"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receive enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TE` reader - Transmit enable"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmit enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `MPE` reader - Management port enable"]
pub type MPE_R = crate::BitReader<bool>;
#[doc = "Field `MPE` writer - Management port enable"]
pub type MPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `CLRSTAT` reader - Clear statistics registers"]
pub type CLRSTAT_R = crate::BitReader<bool>;
#[doc = "Field `CLRSTAT` writer - Clear statistics registers"]
pub type CLRSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `INCSTAT` reader - Increment statistics registers"]
pub type INCSTAT_R = crate::BitReader<bool>;
#[doc = "Field `INCSTAT` writer - Increment statistics registers"]
pub type INCSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `WESTAT` reader - Write enable for statistics registers"]
pub type WESTAT_R = crate::BitReader<bool>;
#[doc = "Field `WESTAT` writer - Write enable for statistics registers"]
pub type WESTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `BP` reader - Back pressure"]
pub type BP_R = crate::BitReader<bool>;
#[doc = "Field `BP` writer - Back pressure"]
pub type BP_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `TSTART` reader - Start transmission"]
pub type TSTART_R = crate::BitReader<bool>;
#[doc = "Field `TSTART` writer - Start transmission"]
pub type TSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
#[doc = "Field `THALT` reader - Transmit halt"]
pub type THALT_R = crate::BitReader<bool>;
#[doc = "Field `THALT` writer - Transmit halt"]
pub type THALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCR_SPEC, bool, O>;
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
    pub fn lb(&mut self) -> LB_W<0> {
        LB_W::new(self)
    }
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W<1> {
        LLB_W::new(self)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W<4> {
        MPE_W::new(self)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrstat(&mut self) -> CLRSTAT_W<5> {
        CLRSTAT_W::new(self)
    }
    #[doc = "Bit 6 - Increment statistics registers"]
    #[inline(always)]
    pub fn incstat(&mut self) -> INCSTAT_W<6> {
        INCSTAT_W::new(self)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn westat(&mut self) -> WESTAT_W<7> {
        WESTAT_W::new(self)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&mut self) -> BP_W<8> {
        BP_W::new(self)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W<9> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn thalt(&mut self) -> THALT_W<10> {
        THALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr](index.html) module"]
pub struct NCR_SPEC;
impl crate::RegisterSpec for NCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ncr::R](R) reader structure"]
impl crate::Readable for NCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ncr::W](W) writer structure"]
impl crate::Writable for NCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NCR to value 0"]
impl crate::Resettable for NCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
