#[doc = "Register `IDR1` writer"]
pub type W = crate::W<IDR1_SPEC>;
#[doc = "Field `CHID0` writer - Counter Event on Channel 0 Interrupt Disable"]
pub type CHID0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID1` writer - Counter Event on Channel 1 Interrupt Disable"]
pub type CHID1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID2` writer - Counter Event on Channel 2 Interrupt Disable"]
pub type CHID2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID3` writer - Counter Event on Channel 3 Interrupt Disable"]
pub type CHID3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID0` writer - Fault Protection Trigger on Channel 0 Interrupt Disable"]
pub type FCHID0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID1` writer - Fault Protection Trigger on Channel 1 Interrupt Disable"]
pub type FCHID1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID2` writer - Fault Protection Trigger on Channel 2 Interrupt Disable"]
pub type FCHID2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID3` writer - Fault Protection Trigger on Channel 3 Interrupt Disable"]
pub type FCHID3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> CHID0_W<IDR1_SPEC> {
        CHID0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> CHID1_W<IDR1_SPEC> {
        CHID1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> CHID2_W<IDR1_SPEC> {
        CHID2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> CHID3_W<IDR1_SPEC> {
        CHID3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid0(&mut self) -> FCHID0_W<IDR1_SPEC> {
        FCHID0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid1(&mut self) -> FCHID1_W<IDR1_SPEC> {
        FCHID1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid2(&mut self) -> FCHID2_W<IDR1_SPEC> {
        FCHID2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid3(&mut self) -> FCHID3_W<IDR1_SPEC> {
        FCHID3_W::new(self, 19)
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
#[doc = "PWM Interrupt Disable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR1_SPEC;
impl crate::RegisterSpec for IDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr1::W`](W) writer structure"]
impl crate::Writable for IDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
