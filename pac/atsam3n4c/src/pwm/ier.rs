#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `CHID0` writer - Channel ID."]
pub type Chid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID1` writer - Channel ID."]
pub type Chid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID2` writer - Channel ID."]
pub type Chid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID3` writer - Channel ID."]
pub type Chid3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> Chid0W<IerSpec> {
        Chid0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> Chid1W<IerSpec> {
        Chid1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> Chid2W<IerSpec> {
        Chid2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> Chid3W<IerSpec> {
        Chid3W::new(self, 3)
    }
}
#[doc = "PWM Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
