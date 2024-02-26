#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EP0INT` writer - Enable Endpoint 0 Interrupt"]
pub type Ep0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1INT` writer - Enable Endpoint 1 Interrupt"]
pub type Ep1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2INT` writer - Enable Endpoint 2Interrupt"]
pub type Ep2intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3INT` writer - Enable Endpoint 3 Interrupt"]
pub type Ep3intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4INT` writer - Enable Endpoint 4 Interrupt"]
pub type Ep4intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5INT` writer - Enable Endpoint 5 Interrupt"]
pub type Ep5intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6INT` writer - Enable Endpoint 6 Interrupt"]
pub type Ep6intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7INT` writer - Enable Endpoint 7 Interrupt"]
pub type Ep7intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSUSP` writer - Enable UDP Suspend Interrupt"]
pub type RxsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSM` writer - Enable UDP Resume Interrupt"]
pub type RxrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRSM` writer - "]
pub type ExtrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFINT` writer - Enable Start Of Frame Interrupt"]
pub type SofintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Enable UDP bus Wakeup Interrupt"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable Endpoint 0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep0int(&mut self) -> Ep0intW<IerSpec> {
        Ep0intW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Endpoint 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep1int(&mut self) -> Ep1intW<IerSpec> {
        Ep1intW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Endpoint 2Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep2int(&mut self) -> Ep2intW<IerSpec> {
        Ep2intW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Endpoint 3 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep3int(&mut self) -> Ep3intW<IerSpec> {
        Ep3intW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Endpoint 4 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep4int(&mut self) -> Ep4intW<IerSpec> {
        Ep4intW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Endpoint 5 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep5int(&mut self) -> Ep5intW<IerSpec> {
        Ep5intW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Endpoint 6 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep6int(&mut self) -> Ep6intW<IerSpec> {
        Ep6intW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Endpoint 7 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep7int(&mut self) -> Ep7intW<IerSpec> {
        Ep7intW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable UDP Suspend Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxsusp(&mut self) -> RxsuspW<IerSpec> {
        RxsuspW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable UDP Resume Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsm(&mut self) -> RxrsmW<IerSpec> {
        RxrsmW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn extrsm(&mut self) -> ExtrsmW<IerSpec> {
        ExtrsmW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Start Of Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sofint(&mut self) -> SofintW<IerSpec> {
        SofintW::new(self, 11)
    }
    #[doc = "Bit 13 - Enable UDP bus Wakeup Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IerSpec> {
        WakeupW::new(self, 13)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
