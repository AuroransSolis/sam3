#[doc = "Register `CHDR` writer"]
pub type W = crate::W<CHDR_SPEC>;
#[doc = "Field `DIS0` writer - Disable \\[3:0\\]"]
pub type DIS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS1` writer - Disable \\[3:0\\]"]
pub type DIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS2` writer - Disable \\[3:0\\]"]
pub type DIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS3` writer - Disable \\[3:0\\]"]
pub type DIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0` writer - Resume \\[3:0\\]"]
pub type RES0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES1` writer - Resume \\[3:0\\]"]
pub type RES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES2` writer - Resume \\[3:0\\]"]
pub type RES2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES3` writer - Resume \\[3:0\\]"]
pub type RES3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis0(&mut self) -> DIS0_W<CHDR_SPEC> {
        DIS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis1(&mut self) -> DIS1_W<CHDR_SPEC> {
        DIS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis2(&mut self) -> DIS2_W<CHDR_SPEC> {
        DIS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis3(&mut self) -> DIS3_W<CHDR_SPEC> {
        DIS3_W::new(self, 3)
    }
    #[doc = "Bit 8 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> RES0_W<CHDR_SPEC> {
        RES0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> RES1_W<CHDR_SPEC> {
        RES1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> RES2_W<CHDR_SPEC> {
        RES2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> RES3_W<CHDR_SPEC> {
        RES3_W::new(self, 11)
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
#[doc = "DMAC Channel Handler Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdr::W`](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
