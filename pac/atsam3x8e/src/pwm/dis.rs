#[doc = "Register `DIS` writer"]
pub struct W(crate::W<DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIS_SPEC>;
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
impl From<crate::W<DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHID0` writer - Channel ID"]
pub type CHID0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `CHID1` writer - Channel ID"]
pub type CHID1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `CHID2` writer - Channel ID"]
pub type CHID2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `CHID3` writer - Channel ID"]
pub type CHID3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `CHID4` writer - Channel ID"]
pub type CHID4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `CHID5` writer - Channel ID"]
pub type CHID5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `CHID6` writer - Channel ID"]
pub type CHID6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `CHID7` writer - Channel ID"]
pub type CHID7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&mut self) -> CHID0_W<0> {
        CHID0_W::new(self)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&mut self) -> CHID1_W<1> {
        CHID1_W::new(self)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&mut self) -> CHID2_W<2> {
        CHID2_W::new(self)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&mut self) -> CHID3_W<3> {
        CHID3_W::new(self)
    }
    #[doc = "Bit 4 - Channel ID"]
    #[inline(always)]
    pub fn chid4(&mut self) -> CHID4_W<4> {
        CHID4_W::new(self)
    }
    #[doc = "Bit 5 - Channel ID"]
    #[inline(always)]
    pub fn chid5(&mut self) -> CHID5_W<5> {
        CHID5_W::new(self)
    }
    #[doc = "Bit 6 - Channel ID"]
    #[inline(always)]
    pub fn chid6(&mut self) -> CHID6_W<6> {
        CHID6_W::new(self)
    }
    #[doc = "Bit 7 - Channel ID"]
    #[inline(always)]
    pub fn chid7(&mut self) -> CHID7_W<7> {
        CHID7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](index.html) module"]
pub struct DIS_SPEC;
impl crate::RegisterSpec for DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dis::W](W) writer structure"]
impl crate::Writable for DIS_SPEC {
    type Writer = W;
}
