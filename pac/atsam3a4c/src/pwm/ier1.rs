#[doc = "Register `IER1` writer"]
pub type W = crate::W<Ier1Spec>;
#[doc = "Field `CHID0` writer - Counter Event on Channel 0 Interrupt Enable"]
pub type Chid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID1` writer - Counter Event on Channel 1 Interrupt Enable"]
pub type Chid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID2` writer - Counter Event on Channel 2 Interrupt Enable"]
pub type Chid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID3` writer - Counter Event on Channel 3 Interrupt Enable"]
pub type Chid3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID4` writer - Counter Event on Channel 4 Interrupt Enable"]
pub type Chid4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID5` writer - Counter Event on Channel 5 Interrupt Enable"]
pub type Chid5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID6` writer - Counter Event on Channel 6 Interrupt Enable"]
pub type Chid6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID7` writer - Counter Event on Channel 7 Interrupt Enable"]
pub type Chid7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID0` writer - Fault Protection Trigger on Channel 0 Interrupt Enable"]
pub type Fchid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID1` writer - Fault Protection Trigger on Channel 1 Interrupt Enable"]
pub type Fchid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID2` writer - Fault Protection Trigger on Channel 2 Interrupt Enable"]
pub type Fchid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID3` writer - Fault Protection Trigger on Channel 3 Interrupt Enable"]
pub type Fchid3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID4` writer - Fault Protection Trigger on Channel 4 Interrupt Enable"]
pub type Fchid4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID5` writer - Fault Protection Trigger on Channel 5 Interrupt Enable"]
pub type Fchid5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID6` writer - Fault Protection Trigger on Channel 6 Interrupt Enable"]
pub type Fchid6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCHID7` writer - Fault Protection Trigger on Channel 7 Interrupt Enable"]
pub type Fchid7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> Chid0W<Ier1Spec> {
        Chid0W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> Chid1W<Ier1Spec> {
        Chid1W::new(self, 1)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> Chid2W<Ier1Spec> {
        Chid2W::new(self, 2)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> Chid3W<Ier1Spec> {
        Chid3W::new(self, 3)
    }
    #[doc = "Bit 4 - Counter Event on Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid4(&mut self) -> Chid4W<Ier1Spec> {
        Chid4W::new(self, 4)
    }
    #[doc = "Bit 5 - Counter Event on Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid5(&mut self) -> Chid5W<Ier1Spec> {
        Chid5W::new(self, 5)
    }
    #[doc = "Bit 6 - Counter Event on Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid6(&mut self) -> Chid6W<Ier1Spec> {
        Chid6W::new(self, 6)
    }
    #[doc = "Bit 7 - Counter Event on Channel 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chid7(&mut self) -> Chid7W<Ier1Spec> {
        Chid7W::new(self, 7)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid0(&mut self) -> Fchid0W<Ier1Spec> {
        Fchid0W::new(self, 16)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid1(&mut self) -> Fchid1W<Ier1Spec> {
        Fchid1W::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid2(&mut self) -> Fchid2W<Ier1Spec> {
        Fchid2W::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid3(&mut self) -> Fchid3W<Ier1Spec> {
        Fchid3W::new(self, 19)
    }
    #[doc = "Bit 20 - Fault Protection Trigger on Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid4(&mut self) -> Fchid4W<Ier1Spec> {
        Fchid4W::new(self, 20)
    }
    #[doc = "Bit 21 - Fault Protection Trigger on Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid5(&mut self) -> Fchid5W<Ier1Spec> {
        Fchid5W::new(self, 21)
    }
    #[doc = "Bit 22 - Fault Protection Trigger on Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid6(&mut self) -> Fchid6W<Ier1Spec> {
        Fchid6W::new(self, 22)
    }
    #[doc = "Bit 23 - Fault Protection Trigger on Channel 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fchid7(&mut self) -> Fchid7W<Ier1Spec> {
        Fchid7W::new(self, 23)
    }
}
#[doc = "PWM Interrupt Enable Register 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ier1Spec;
impl crate::RegisterSpec for Ier1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier1::W`](W) writer structure"]
impl crate::Writable for Ier1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
