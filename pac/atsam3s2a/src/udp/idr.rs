#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `EP0INT` writer - Disable Endpoint 0 Interrupt"]
pub type Ep0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1INT` writer - Disable Endpoint 1 Interrupt"]
pub type Ep1intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2INT` writer - Disable Endpoint 2 Interrupt"]
pub type Ep2intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3INT` writer - Disable Endpoint 3 Interrupt"]
pub type Ep3intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4INT` writer - Disable Endpoint 4 Interrupt"]
pub type Ep4intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5INT` writer - Disable Endpoint 5 Interrupt"]
pub type Ep5intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6INT` writer - Disable Endpoint 6 Interrupt"]
pub type Ep6intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7INT` writer - Disable Endpoint 7 Interrupt"]
pub type Ep7intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSUSP` writer - Disable UDP Suspend Interrupt"]
pub type RxsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSM` writer - Disable UDP Resume Interrupt"]
pub type RxrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRSM` writer - "]
pub type ExtrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFINT` writer - Disable Start Of Frame Interrupt"]
pub type SofintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Disable USB Bus Interrupt"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable Endpoint 0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep0int(&mut self) -> Ep0intW<IdrSpec> {
        Ep0intW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Endpoint 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep1int(&mut self) -> Ep1intW<IdrSpec> {
        Ep1intW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Endpoint 2 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep2int(&mut self) -> Ep2intW<IdrSpec> {
        Ep2intW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable Endpoint 3 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep3int(&mut self) -> Ep3intW<IdrSpec> {
        Ep3intW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable Endpoint 4 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep4int(&mut self) -> Ep4intW<IdrSpec> {
        Ep4intW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Endpoint 5 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep5int(&mut self) -> Ep5intW<IdrSpec> {
        Ep5intW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Endpoint 6 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep6int(&mut self) -> Ep6intW<IdrSpec> {
        Ep6intW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable Endpoint 7 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ep7int(&mut self) -> Ep7intW<IdrSpec> {
        Ep7intW::new(self, 7)
    }
    #[doc = "Bit 8 - Disable UDP Suspend Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxsusp(&mut self) -> RxsuspW<IdrSpec> {
        RxsuspW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable UDP Resume Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsm(&mut self) -> RxrsmW<IdrSpec> {
        RxrsmW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn extrsm(&mut self) -> ExtrsmW<IdrSpec> {
        ExtrsmW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable Start Of Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sofint(&mut self) -> SofintW<IdrSpec> {
        SofintW::new(self, 11)
    }
    #[doc = "Bit 13 - Disable USB Bus Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IdrSpec> {
        WakeupW::new(self, 13)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
