#[doc = "Register `QIDR` writer"]
pub type W = crate::W<QIDR_SPEC>;
#[doc = "Field `IDX` writer - InDeX"]
pub type IDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - DIRection CHanGe"]
pub type DIRCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QERR` writer - Quadrature ERRor"]
pub type QERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - InDeX"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IDX_W<QIDR_SPEC> {
        IDX_W::new(self, 0)
    }
    #[doc = "Bit 1 - DIRection CHanGe"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<QIDR_SPEC> {
        DIRCHG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Quadrature ERRor"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QERR_W<QIDR_SPEC> {
        QERR_W::new(self, 2)
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
#[doc = "QDEC Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QIDR_SPEC;
impl crate::RegisterSpec for QIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qidr::W`](W) writer structure"]
impl crate::Writable for QIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
