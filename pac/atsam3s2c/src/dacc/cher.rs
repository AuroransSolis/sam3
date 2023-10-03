#[doc = "Register `CHER` writer"]
pub type W = crate::W<CHER_SPEC>;
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type CH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type CH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<CHER_SPEC, 0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<CHER_SPEC, 1> {
        CH1_W::new(self)
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
#[doc = "Channel Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHER_SPEC;
impl crate::RegisterSpec for CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cher::W`](W) writer structure"]
impl crate::Writable for CHER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
