#[doc = "Register `TXVC` reader"]
pub type R = crate::R<TXVC_SPEC>;
#[doc = "Register `TXVC` writer"]
pub type W = crate::W<TXVC_SPEC>;
#[doc = "Field `TXVDIS` reader - Transceiver Disable"]
pub type TXVDIS_R = crate::BitReader;
#[doc = "Field `TXVDIS` writer - Transceiver Disable"]
pub type TXVDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUON` reader - Pull-up On"]
pub type PUON_R = crate::BitReader;
#[doc = "Field `PUON` writer - Pull-up On"]
pub type PUON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    pub fn txvdis(&self) -> TXVDIS_R {
        TXVDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    pub fn puon(&self) -> PUON_R {
        PUON_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txvdis(&mut self) -> TXVDIS_W<TXVC_SPEC> {
        TXVDIS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    #[must_use]
    pub fn puon(&mut self) -> PUON_W<TXVC_SPEC> {
        PUON_W::new(self, 9)
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
#[doc = "Transceiver Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txvc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXVC_SPEC;
impl crate::RegisterSpec for TXVC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txvc::R`](R) reader structure"]
impl crate::Readable for TXVC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txvc::W`](W) writer structure"]
impl crate::Writable for TXVC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXVC to value 0x0100"]
impl crate::Resettable for TXVC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
