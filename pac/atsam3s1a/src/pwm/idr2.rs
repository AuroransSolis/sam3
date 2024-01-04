#[doc = "Register `IDR2` writer"]
pub type W = crate::W<IDR2_SPEC>;
#[doc = "Field `WRDY` writer - Write Ready for Synchronous Channels Update Interrupt Disable"]
pub type WRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - PDC End of TX Buffer Interrupt Disable"]
pub type ENDTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - PDC TX Buffer Empty Interrupt Disable"]
pub type TXBUFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Synchronous Channels Update Underrun Error Interrupt Disable"]
pub type UNRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM0` writer - Comparison 0 Match Interrupt Disable"]
pub type CMPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM1` writer - Comparison 1 Match Interrupt Disable"]
pub type CMPM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM2` writer - Comparison 2 Match Interrupt Disable"]
pub type CMPM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM3` writer - Comparison 3 Match Interrupt Disable"]
pub type CMPM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM4` writer - Comparison 4 Match Interrupt Disable"]
pub type CMPM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM5` writer - Comparison 5 Match Interrupt Disable"]
pub type CMPM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM6` writer - Comparison 6 Match Interrupt Disable"]
pub type CMPM6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM7` writer - Comparison 7 Match Interrupt Disable"]
pub type CMPM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU0` writer - Comparison 0 Update Interrupt Disable"]
pub type CMPU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU1` writer - Comparison 1 Update Interrupt Disable"]
pub type CMPU1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU2` writer - Comparison 2 Update Interrupt Disable"]
pub type CMPU2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU3` writer - Comparison 3 Update Interrupt Disable"]
pub type CMPU3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU4` writer - Comparison 4 Update Interrupt Disable"]
pub type CMPU4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU5` writer - Comparison 5 Update Interrupt Disable"]
pub type CMPU5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU6` writer - Comparison 6 Update Interrupt Disable"]
pub type CMPU6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU7` writer - Comparison 7 Update Interrupt Disable"]
pub type CMPU7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wrdy(&mut self) -> WRDY_W<IDR2_SPEC> {
        WRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - PDC End of TX Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<IDR2_SPEC> {
        ENDTX_W::new(self, 1)
    }
    #[doc = "Bit 2 - PDC TX Buffer Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<IDR2_SPEC> {
        TXBUFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<IDR2_SPEC> {
        UNRE_W::new(self, 3)
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm0(&mut self) -> CMPM0_W<IDR2_SPEC> {
        CMPM0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm1(&mut self) -> CMPM1_W<IDR2_SPEC> {
        CMPM1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm2(&mut self) -> CMPM2_W<IDR2_SPEC> {
        CMPM2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm3(&mut self) -> CMPM3_W<IDR2_SPEC> {
        CMPM3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm4(&mut self) -> CMPM4_W<IDR2_SPEC> {
        CMPM4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm5(&mut self) -> CMPM5_W<IDR2_SPEC> {
        CMPM5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm6(&mut self) -> CMPM6_W<IDR2_SPEC> {
        CMPM6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm7(&mut self) -> CMPM7_W<IDR2_SPEC> {
        CMPM7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu0(&mut self) -> CMPU0_W<IDR2_SPEC> {
        CMPU0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu1(&mut self) -> CMPU1_W<IDR2_SPEC> {
        CMPU1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu2(&mut self) -> CMPU2_W<IDR2_SPEC> {
        CMPU2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu3(&mut self) -> CMPU3_W<IDR2_SPEC> {
        CMPU3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu4(&mut self) -> CMPU4_W<IDR2_SPEC> {
        CMPU4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu5(&mut self) -> CMPU5_W<IDR2_SPEC> {
        CMPU5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu6(&mut self) -> CMPU6_W<IDR2_SPEC> {
        CMPU6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu7(&mut self) -> CMPU7_W<IDR2_SPEC> {
        CMPU7_W::new(self, 23)
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
#[doc = "PWM Interrupt Disable Register 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR2_SPEC;
impl crate::RegisterSpec for IDR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr2::W`](W) writer structure"]
impl crate::Writable for IDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
