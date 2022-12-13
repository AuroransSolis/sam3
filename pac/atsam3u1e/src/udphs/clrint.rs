#[doc = "Register `CLRINT` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CLRINT_SPEC>);
#[doc = "Field `DET_SUSPD` writer - Suspend Interrupt Clear"]
pub type DET_SUSPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRINT_SPEC, bool, O>;
#[doc = "Field `MICRO_SOF` writer - Micro Start Of Frame Interrupt Clear"]
pub type MICRO_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRINT_SPEC, bool, O>;
#[doc = "Field `INT_SOF` writer - Start Of Frame Interrupt Clear"]
pub type INT_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRINT_SPEC, bool, O>;
#[doc = "Field `ENDRESET` writer - End Of Reset Interrupt Clear"]
pub type ENDRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRINT_SPEC, bool, O>;
#[doc = "Field `WAKE_UP` writer - Wake Up CPU Interrupt Clear"]
pub type WAKE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRINT_SPEC, bool, O>;
#[doc = "Field `ENDOFRSM` writer - End Of Resume Interrupt Clear"]
pub type ENDOFRSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRINT_SPEC, bool, O>;
#[doc = "Field `UPSTR_RES` writer - Upstream Resume Interrupt Clear"]
pub type UPSTR_RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRINT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - Suspend Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn det_suspd(&mut self) -> DET_SUSPD_W<1> {
        DET_SUSPD_W::new(self)
    }
    #[doc = "Bit 2 - Micro Start Of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn micro_sof(&mut self) -> MICRO_SOF_W<2> {
        MICRO_SOF_W::new(self)
    }
    #[doc = "Bit 3 - Start Of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn int_sof(&mut self) -> INT_SOF_W<3> {
        INT_SOF_W::new(self)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn endreset(&mut self) -> ENDRESET_W<4> {
        ENDRESET_W::new(self)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wake_up(&mut self) -> WAKE_UP_W<5> {
        WAKE_UP_W::new(self)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn endofrsm(&mut self) -> ENDOFRSM_W<6> {
        ENDOFRSM_W::new(self)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn upstr_res(&mut self) -> UPSTR_RES_W<7> {
        UPSTR_RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Clear Interrupt Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrint](index.html) module"]
pub struct CLRINT_SPEC;
impl crate::RegisterSpec for CLRINT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrint::W](W) writer structure"]
impl crate::Writable for CLRINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
