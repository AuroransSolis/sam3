#[doc = "Register `SEQR1` reader"]
pub type R = crate::R<SEQR1_SPEC>;
#[doc = "Register `SEQR1` writer"]
pub type W = crate::W<SEQR1_SPEC>;
#[doc = "Field `USCH1` reader - User Sequence Number 1"]
pub type USCH1_R = crate::FieldReader;
#[doc = "Field `USCH1` writer - User Sequence Number 1"]
pub type USCH1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH2` reader - User Sequence Number 2"]
pub type USCH2_R = crate::FieldReader;
#[doc = "Field `USCH2` writer - User Sequence Number 2"]
pub type USCH2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH3` reader - User Sequence Number 3"]
pub type USCH3_R = crate::FieldReader;
#[doc = "Field `USCH3` writer - User Sequence Number 3"]
pub type USCH3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH4` reader - User Sequence Number 4"]
pub type USCH4_R = crate::FieldReader;
#[doc = "Field `USCH4` writer - User Sequence Number 4"]
pub type USCH4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH5` reader - User Sequence Number 5"]
pub type USCH5_R = crate::FieldReader;
#[doc = "Field `USCH5` writer - User Sequence Number 5"]
pub type USCH5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH6` reader - User Sequence Number 6"]
pub type USCH6_R = crate::FieldReader;
#[doc = "Field `USCH6` writer - User Sequence Number 6"]
pub type USCH6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH7` reader - User Sequence Number 7"]
pub type USCH7_R = crate::FieldReader;
#[doc = "Field `USCH7` writer - User Sequence Number 7"]
pub type USCH7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USCH8` reader - User Sequence Number 8"]
pub type USCH8_R = crate::FieldReader;
#[doc = "Field `USCH8` writer - User Sequence Number 8"]
pub type USCH8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&self) -> USCH1_R {
        USCH1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&self) -> USCH2_R {
        USCH2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&self) -> USCH3_R {
        USCH3_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&self) -> USCH4_R {
        USCH4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&self) -> USCH5_R {
        USCH5_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&self) -> USCH6_R {
        USCH6_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&self) -> USCH7_R {
        USCH7_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> USCH8_R {
        USCH8_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - User Sequence Number 1"]
    #[inline(always)]
    #[must_use]
    pub fn usch1(&mut self) -> USCH1_W<SEQR1_SPEC> {
        USCH1_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - User Sequence Number 2"]
    #[inline(always)]
    #[must_use]
    pub fn usch2(&mut self) -> USCH2_W<SEQR1_SPEC> {
        USCH2_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - User Sequence Number 3"]
    #[inline(always)]
    #[must_use]
    pub fn usch3(&mut self) -> USCH3_W<SEQR1_SPEC> {
        USCH3_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - User Sequence Number 4"]
    #[inline(always)]
    #[must_use]
    pub fn usch4(&mut self) -> USCH4_W<SEQR1_SPEC> {
        USCH4_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - User Sequence Number 5"]
    #[inline(always)]
    #[must_use]
    pub fn usch5(&mut self) -> USCH5_W<SEQR1_SPEC> {
        USCH5_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - User Sequence Number 6"]
    #[inline(always)]
    #[must_use]
    pub fn usch6(&mut self) -> USCH6_W<SEQR1_SPEC> {
        USCH6_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - User Sequence Number 7"]
    #[inline(always)]
    #[must_use]
    pub fn usch7(&mut self) -> USCH7_W<SEQR1_SPEC> {
        USCH7_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - User Sequence Number 8"]
    #[inline(always)]
    #[must_use]
    pub fn usch8(&mut self) -> USCH8_W<SEQR1_SPEC> {
        USCH8_W::new(self, 28)
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
#[doc = "Channel Sequence Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQR1_SPEC;
impl crate::RegisterSpec for SEQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr1::R`](R) reader structure"]
impl crate::Readable for SEQR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seqr1::W`](W) writer structure"]
impl crate::Writable for SEQR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQR1 to value 0"]
impl crate::Resettable for SEQR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
