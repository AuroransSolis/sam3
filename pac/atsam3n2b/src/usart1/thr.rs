#[doc = "Register `THR` writer"]
pub type W = crate::W<THR_SPEC>;
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TXCHR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXSYNH` writer - Sync Field to be transmitted"]
pub type TXSYNH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txchr(&mut self) -> TXCHR_W<THR_SPEC> {
        TXCHR_W::new(self, 0)
    }
    #[doc = "Bit 15 - Sync Field to be transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txsynh(&mut self) -> TXSYNH_W<THR_SPEC> {
        TXSYNH_W::new(self, 15)
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
#[doc = "Transmitter Holding Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THR_SPEC;
impl crate::RegisterSpec for THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for THR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
