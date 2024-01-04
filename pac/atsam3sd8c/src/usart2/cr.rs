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
#[doc = "Field `STTBRK` writer - Start Break"]
pub type STTBRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPBRK` writer - Stop Break"]
pub type STPBRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTTO` writer - Start Time-out"]
pub type STTTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDA` writer - Send Address"]
pub type SENDA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIT` writer - Reset Iterations"]
pub type RSTIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTNACK` writer - Reset Non Acknowledge"]
pub type RSTNACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETTO` writer - Rearm Time-out"]
pub type RETTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTREN` writer - Data Terminal Ready Enable"]
pub type DTREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTRDIS` writer - Data Terminal Ready Disable"]
pub type DTRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` writer - Request to Send Enable"]
pub type RTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSDIS` writer - Request to Send Disable"]
pub type RTSDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 9 - Start Break"]
    #[inline(always)]
    #[must_use]
    pub fn sttbrk(&mut self) -> STTBRK_W<CR_SPEC> {
        STTBRK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline(always)]
    #[must_use]
    pub fn stpbrk(&mut self) -> STPBRK_W<CR_SPEC> {
        STPBRK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Start Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn sttto(&mut self) -> STTTO_W<CR_SPEC> {
        STTTO_W::new(self, 11)
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline(always)]
    #[must_use]
    pub fn senda(&mut self) -> SENDA_W<CR_SPEC> {
        SENDA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline(always)]
    #[must_use]
    pub fn rstit(&mut self) -> RSTIT_W<CR_SPEC> {
        RSTIT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn rstnack(&mut self) -> RSTNACK_W<CR_SPEC> {
        RSTNACK_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rearm Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn retto(&mut self) -> RETTO_W<CR_SPEC> {
        RETTO_W::new(self, 15)
    }
    #[doc = "Bit 16 - Data Terminal Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtren(&mut self) -> DTREN_W<CR_SPEC> {
        DTREN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Data Terminal Ready Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtrdis(&mut self) -> DTRDIS_W<CR_SPEC> {
        DTRDIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - Request to Send Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<CR_SPEC> {
        RTSEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Request to Send Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsdis(&mut self) -> RTSDIS_W<CR_SPEC> {
        RTSDIS_W::new(self, 19)
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
