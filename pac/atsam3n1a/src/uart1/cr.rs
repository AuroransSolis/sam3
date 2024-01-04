#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RSTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RSTTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RSTSTA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RSTRX_W<CR_SPEC> {
        RSTRX_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RSTTX_W<CR_SPEC> {
        RSTTX_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CR_SPEC> {
        RXEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<CR_SPEC> {
        RXDIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<CR_SPEC> {
        TXEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<CR_SPEC> {
        TXDIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RSTSTA_W<CR_SPEC> {
        RSTSTA_W::new(self, 8)
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
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
