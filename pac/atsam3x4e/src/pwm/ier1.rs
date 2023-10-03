#[doc = "Register `IER1` writer"]
pub type W = crate::W<IER1_SPEC>;
#[doc = "Field `CHID0` writer - Counter Event on Channel 0 Interrupt Enable"]
pub type CHID0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHID1` writer - Counter Event on Channel 1 Interrupt Enable"]
pub type CHID1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHID2` writer - Counter Event on Channel 2 Interrupt Enable"]
pub type CHID2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHID3` writer - Counter Event on Channel 3 Interrupt Enable"]
pub type CHID3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHID4` writer - Counter Event on Channel 4 Interrupt Enable"]
pub type CHID4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHID5` writer - Counter Event on Channel 5 Interrupt Enable"]
pub type CHID5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHID6` writer - Counter Event on Channel 6 Interrupt Enable"]
pub type CHID6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHID7` writer - Counter Event on Channel 7 Interrupt Enable"]
pub type CHID7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID0` writer - Fault Protection Trigger on Channel 0 Interrupt Enable"]
pub type FCHID0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID1` writer - Fault Protection Trigger on Channel 1 Interrupt Enable"]
pub type FCHID1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID2` writer - Fault Protection Trigger on Channel 2 Interrupt Enable"]
pub type FCHID2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID3` writer - Fault Protection Trigger on Channel 3 Interrupt Enable"]
pub type FCHID3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID4` writer - Fault Protection Trigger on Channel 4 Interrupt Enable"]
pub type FCHID4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID5` writer - Fault Protection Trigger on Channel 5 Interrupt Enable"]
pub type FCHID5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID6` writer - Fault Protection Trigger on Channel 6 Interrupt Enable"]
pub type FCHID6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCHID7` writer - Fault Protection Trigger on Channel 7 Interrupt Enable"]
pub type FCHID7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> CHID0_W<IER1_SPEC, 0> {
        CHID0_W::new(self)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> CHID1_W<IER1_SPEC, 1> {
        CHID1_W::new(self)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> CHID2_W<IER1_SPEC, 2> {
        CHID2_W::new(self)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> CHID3_W<IER1_SPEC, 3> {
        CHID3_W::new(self)
    }
    #[doc = "Bit 4 - Counter Event on Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid4(&mut self) -> CHID4_W<IER1_SPEC, 4> {
        CHID4_W::new(self)
    }
    #[doc = "Bit 5 - Counter Event on Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid5(&mut self) -> CHID5_W<IER1_SPEC, 5> {
        CHID5_W::new(self)
    }
    #[doc = "Bit 6 - Counter Event on Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid6(&mut self) -> CHID6_W<IER1_SPEC, 6> {
        CHID6_W::new(self)
    }
    #[doc = "Bit 7 - Counter Event on Channel 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid7(&mut self) -> CHID7_W<IER1_SPEC, 7> {
        CHID7_W::new(self)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid0(&mut self) -> FCHID0_W<IER1_SPEC, 16> {
        FCHID0_W::new(self)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid1(&mut self) -> FCHID1_W<IER1_SPEC, 17> {
        FCHID1_W::new(self)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid2(&mut self) -> FCHID2_W<IER1_SPEC, 18> {
        FCHID2_W::new(self)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid3(&mut self) -> FCHID3_W<IER1_SPEC, 19> {
        FCHID3_W::new(self)
    }
    #[doc = "Bit 20 - Fault Protection Trigger on Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid4(&mut self) -> FCHID4_W<IER1_SPEC, 20> {
        FCHID4_W::new(self)
    }
    #[doc = "Bit 21 - Fault Protection Trigger on Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid5(&mut self) -> FCHID5_W<IER1_SPEC, 21> {
        FCHID5_W::new(self)
    }
    #[doc = "Bit 22 - Fault Protection Trigger on Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid6(&mut self) -> FCHID6_W<IER1_SPEC, 22> {
        FCHID6_W::new(self)
    }
    #[doc = "Bit 23 - Fault Protection Trigger on Channel 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid7(&mut self) -> FCHID7_W<IER1_SPEC, 23> {
        FCHID7_W::new(self)
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
#[doc = "PWM Interrupt Enable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier1::W`](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
