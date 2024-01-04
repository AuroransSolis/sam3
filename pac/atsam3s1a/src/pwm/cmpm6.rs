#[doc = "Register `CMPM6` reader"]
pub type R = crate::R<CMPM6_SPEC>;
#[doc = "Register `CMPM6` writer"]
pub type W = crate::W<CMPM6_SPEC>;
#[doc = "Field `CEN` reader - Comparison x Enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Comparison x Enable"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - Comparison x Trigger"]
pub type CTR_R = crate::FieldReader;
#[doc = "Field `CTR` writer - Comparison x Trigger"]
pub type CTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPR` reader - Comparison x Period"]
pub type CPR_R = crate::FieldReader;
#[doc = "Field `CPR` writer - Comparison x Period"]
pub type CPR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPRCNT` reader - Comparison x Period Counter"]
pub type CPRCNT_R = crate::FieldReader;
#[doc = "Field `CPRCNT` writer - Comparison x Period Counter"]
pub type CPRCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUPR` reader - Comparison x Update Period"]
pub type CUPR_R = crate::FieldReader;
#[doc = "Field `CUPR` writer - Comparison x Update Period"]
pub type CUPR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUPRCNT` reader - Comparison x Update Period Counter"]
pub type CUPRCNT_R = crate::FieldReader;
#[doc = "Field `CUPRCNT` writer - Comparison x Update Period Counter"]
pub type CUPRCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&self) -> CPRCNT_R {
        CPRCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&self) -> CUPR_R {
        CUPR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&self) -> CUPRCNT_R {
        CUPRCNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CMPM6_SPEC> {
        CEN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<CMPM6_SPEC> {
        CTR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<CMPM6_SPEC> {
        CPR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cprcnt(&mut self) -> CPRCNT_W<CMPM6_SPEC> {
        CPRCNT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn cupr(&mut self) -> CUPR_W<CMPM6_SPEC> {
        CUPR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cuprcnt(&mut self) -> CUPRCNT_W<CMPM6_SPEC> {
        CUPRCNT_W::new(self, 20)
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
#[doc = "PWM Comparison 6 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPM6_SPEC;
impl crate::RegisterSpec for CMPM6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpm6::R`](R) reader structure"]
impl crate::Readable for CMPM6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpm6::W`](W) writer structure"]
impl crate::Writable for CMPM6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPM6 to value 0"]
impl crate::Resettable for CMPM6_SPEC {
    const RESET_VALUE: u32 = 0;
}
