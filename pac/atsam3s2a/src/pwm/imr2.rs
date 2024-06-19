#[doc = "Register `IMR2` reader"]
pub type R = crate::R<Imr2Spec>;
#[doc = "Field `WRDY` reader - Write Ready for Synchronous Channels Update Interrupt Mask"]
pub type WrdyR = crate::BitReader;
#[doc = "Field `ENDTX` reader - PDC End of TX Buffer Interrupt Mask"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - PDC TX Buffer Empty Interrupt Mask"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `UNRE` reader - Synchronous Channels Update Underrun Error Interrupt Mask"]
pub type UnreR = crate::BitReader;
#[doc = "Field `CMPM0` reader - Comparison 0 Match Interrupt Mask"]
pub type Cmpm0R = crate::BitReader;
#[doc = "Field `CMPM1` reader - Comparison 1 Match Interrupt Mask"]
pub type Cmpm1R = crate::BitReader;
#[doc = "Field `CMPM2` reader - Comparison 2 Match Interrupt Mask"]
pub type Cmpm2R = crate::BitReader;
#[doc = "Field `CMPM3` reader - Comparison 3 Match Interrupt Mask"]
pub type Cmpm3R = crate::BitReader;
#[doc = "Field `CMPM4` reader - Comparison 4 Match Interrupt Mask"]
pub type Cmpm4R = crate::BitReader;
#[doc = "Field `CMPM5` reader - Comparison 5 Match Interrupt Mask"]
pub type Cmpm5R = crate::BitReader;
#[doc = "Field `CMPM6` reader - Comparison 6 Match Interrupt Mask"]
pub type Cmpm6R = crate::BitReader;
#[doc = "Field `CMPM7` reader - Comparison 7 Match Interrupt Mask"]
pub type Cmpm7R = crate::BitReader;
#[doc = "Field `CMPU0` reader - Comparison 0 Update Interrupt Mask"]
pub type Cmpu0R = crate::BitReader;
#[doc = "Field `CMPU1` reader - Comparison 1 Update Interrupt Mask"]
pub type Cmpu1R = crate::BitReader;
#[doc = "Field `CMPU2` reader - Comparison 2 Update Interrupt Mask"]
pub type Cmpu2R = crate::BitReader;
#[doc = "Field `CMPU3` reader - Comparison 3 Update Interrupt Mask"]
pub type Cmpu3R = crate::BitReader;
#[doc = "Field `CMPU4` reader - Comparison 4 Update Interrupt Mask"]
pub type Cmpu4R = crate::BitReader;
#[doc = "Field `CMPU5` reader - Comparison 5 Update Interrupt Mask"]
pub type Cmpu5R = crate::BitReader;
#[doc = "Field `CMPU6` reader - Comparison 6 Update Interrupt Mask"]
pub type Cmpu6R = crate::BitReader;
#[doc = "Field `CMPU7` reader - Comparison 7 Update Interrupt Mask"]
pub type Cmpu7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Mask"]
    #[inline(always)]
    pub fn wrdy(&self) -> WrdyR {
        WrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDC End of TX Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDC TX Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm0(&self) -> Cmpm0R {
        Cmpm0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm1(&self) -> Cmpm1R {
        Cmpm1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm2(&self) -> Cmpm2R {
        Cmpm2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm3(&self) -> Cmpm3R {
        Cmpm3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm4(&self) -> Cmpm4R {
        Cmpm4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm5(&self) -> Cmpm5R {
        Cmpm5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm6(&self) -> Cmpm6R {
        Cmpm6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm7(&self) -> Cmpm7R {
        Cmpm7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu0(&self) -> Cmpu0R {
        Cmpu0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu1(&self) -> Cmpu1R {
        Cmpu1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu2(&self) -> Cmpu2R {
        Cmpu2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu3(&self) -> Cmpu3R {
        Cmpu3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu4(&self) -> Cmpu4R {
        Cmpu4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu5(&self) -> Cmpu5R {
        Cmpu5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu6(&self) -> Cmpu6R {
        Cmpu6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu7(&self) -> Cmpu7R {
        Cmpu7R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Mask Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr2Spec;
impl crate::RegisterSpec for Imr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for Imr2Spec {}
#[doc = "`reset()` method sets IMR2 to value 0"]
impl crate::Resettable for Imr2Spec {
    const RESET_VALUE: u32 = 0;
}
