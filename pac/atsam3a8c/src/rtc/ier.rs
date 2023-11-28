#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `ACKEN` writer - Acknowledge Update Interrupt Enable"]
pub type ACKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALREN` writer - Alarm Interrupt Enable"]
pub type ALREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECEN` writer - Second Event Interrupt Enable"]
pub type SECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEN` writer - Time Event Interrupt Enable"]
pub type TIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALEN` writer - Calendar Event Interrupt Enable"]
pub type CALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acken(&mut self) -> ACKEN_W<IER_SPEC> {
        ACKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alren(&mut self) -> ALREN_W<IER_SPEC> {
        ALREN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Second Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn secen(&mut self) -> SECEN_W<IER_SPEC> {
        SECEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Time Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timen(&mut self) -> TIMEN_W<IER_SPEC> {
        TIMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<IER_SPEC> {
        CALEN_W::new(self, 4)
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
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
