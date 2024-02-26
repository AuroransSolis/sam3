#[doc = "Register `IER2` writer"]
pub type W = crate::W<Ier2Spec>;
#[doc = "Field `WRDY` writer - Write Ready for Synchronous Channels Update Interrupt Enable"]
pub type WrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - PDC End of TX Buffer Interrupt Enable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - PDC TX Buffer Empty Interrupt Enable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Synchronous Channels Update Underrun Error Interrupt Enable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM0` writer - Comparison 0 Match Interrupt Enable"]
pub type Cmpm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM1` writer - Comparison 1 Match Interrupt Enable"]
pub type Cmpm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM2` writer - Comparison 2 Match Interrupt Enable"]
pub type Cmpm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM3` writer - Comparison 3 Match Interrupt Enable"]
pub type Cmpm3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM4` writer - Comparison 4 Match Interrupt Enable"]
pub type Cmpm4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM5` writer - Comparison 5 Match Interrupt Enable"]
pub type Cmpm5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM6` writer - Comparison 6 Match Interrupt Enable"]
pub type Cmpm6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPM7` writer - Comparison 7 Match Interrupt Enable"]
pub type Cmpm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU0` writer - Comparison 0 Update Interrupt Enable"]
pub type Cmpu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU1` writer - Comparison 1 Update Interrupt Enable"]
pub type Cmpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU2` writer - Comparison 2 Update Interrupt Enable"]
pub type Cmpu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU3` writer - Comparison 3 Update Interrupt Enable"]
pub type Cmpu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU4` writer - Comparison 4 Update Interrupt Enable"]
pub type Cmpu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU5` writer - Comparison 5 Update Interrupt Enable"]
pub type Cmpu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU6` writer - Comparison 6 Update Interrupt Enable"]
pub type Cmpu6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPU7` writer - Comparison 7 Update Interrupt Enable"]
pub type Cmpu7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrdy(&mut self) -> WrdyW<Ier2Spec> {
        WrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - PDC End of TX Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<Ier2Spec> {
        EndtxW::new(self, 1)
    }
    #[doc = "Bit 2 - PDC TX Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<Ier2Spec> {
        TxbufeW::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UnreW<Ier2Spec> {
        UnreW::new(self, 3)
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm0(&mut self) -> Cmpm0W<Ier2Spec> {
        Cmpm0W::new(self, 8)
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm1(&mut self) -> Cmpm1W<Ier2Spec> {
        Cmpm1W::new(self, 9)
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm2(&mut self) -> Cmpm2W<Ier2Spec> {
        Cmpm2W::new(self, 10)
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm3(&mut self) -> Cmpm3W<Ier2Spec> {
        Cmpm3W::new(self, 11)
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm4(&mut self) -> Cmpm4W<Ier2Spec> {
        Cmpm4W::new(self, 12)
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm5(&mut self) -> Cmpm5W<Ier2Spec> {
        Cmpm5W::new(self, 13)
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm6(&mut self) -> Cmpm6W<Ier2Spec> {
        Cmpm6W::new(self, 14)
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpm7(&mut self) -> Cmpm7W<Ier2Spec> {
        Cmpm7W::new(self, 15)
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu0(&mut self) -> Cmpu0W<Ier2Spec> {
        Cmpu0W::new(self, 16)
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu1(&mut self) -> Cmpu1W<Ier2Spec> {
        Cmpu1W::new(self, 17)
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu2(&mut self) -> Cmpu2W<Ier2Spec> {
        Cmpu2W::new(self, 18)
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu3(&mut self) -> Cmpu3W<Ier2Spec> {
        Cmpu3W::new(self, 19)
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu4(&mut self) -> Cmpu4W<Ier2Spec> {
        Cmpu4W::new(self, 20)
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu5(&mut self) -> Cmpu5W<Ier2Spec> {
        Cmpu5W::new(self, 21)
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu6(&mut self) -> Cmpu6W<Ier2Spec> {
        Cmpu6W::new(self, 22)
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpu7(&mut self) -> Cmpu7W<Ier2Spec> {
        Cmpu7W::new(self, 23)
    }
}
#[doc = "PWM Interrupt Enable Register 2\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ier2Spec;
impl crate::RegisterSpec for Ier2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier2::W`](W) writer structure"]
impl crate::Writable for Ier2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
