#[doc = "Register `QIER` writer"]
pub type W = crate::W<QIER_SPEC>;
#[doc = "Field `IDX` writer - Index"]
pub type IDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - Direction Change"]
pub type DIRCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QERR` writer - Quadrature Error"]
pub type QERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IDX_W<QIER_SPEC> {
        IDX_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<QIER_SPEC> {
        DIRCHG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QERR_W<QIER_SPEC> {
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
#[doc = "QDEC Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QIER_SPEC;
impl crate::RegisterSpec for QIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qier::W`](W) writer structure"]
impl crate::Writable for QIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
