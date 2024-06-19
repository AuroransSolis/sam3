#[doc = "Register `CMPM7` reader"]
pub type R = crate::R<Cmpm7Spec>;
#[doc = "Register `CMPM7` writer"]
pub type W = crate::W<Cmpm7Spec>;
#[doc = "Field `CEN` reader - Comparison x Enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - Comparison x Enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - Comparison x Trigger"]
pub type CtrR = crate::FieldReader;
#[doc = "Field `CTR` writer - Comparison x Trigger"]
pub type CtrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPR` reader - Comparison x Period"]
pub type CprR = crate::FieldReader;
#[doc = "Field `CPR` writer - Comparison x Period"]
pub type CprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPRCNT` reader - Comparison x Period Counter"]
pub type CprcntR = crate::FieldReader;
#[doc = "Field `CPRCNT` writer - Comparison x Period Counter"]
pub type CprcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUPR` reader - Comparison x Update Period"]
pub type CuprR = crate::FieldReader;
#[doc = "Field `CUPR` writer - Comparison x Update Period"]
pub type CuprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUPRCNT` reader - Comparison x Update Period Counter"]
pub type CuprcntR = crate::FieldReader;
#[doc = "Field `CUPRCNT` writer - Comparison x Update Period Counter"]
pub type CuprcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&self) -> CprR {
        CprR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&self) -> CprcntR {
        CprcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&self) -> CuprR {
        CuprR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&self) -> CuprcntR {
        CuprcntR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CenW<Cmpm7Spec> {
        CenW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CtrW<Cmpm7Spec> {
        CtrW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CprW<Cmpm7Spec> {
        CprW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cprcnt(&mut self) -> CprcntW<Cmpm7Spec> {
        CprcntW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn cupr(&mut self) -> CuprW<Cmpm7Spec> {
        CuprW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cuprcnt(&mut self) -> CuprcntW<Cmpm7Spec> {
        CuprcntW::new(self, 20)
    }
}
#[doc = "PWM Comparison 7 Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpm7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpm7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpm7Spec;
impl crate::RegisterSpec for Cmpm7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpm7::R`](R) reader structure"]
impl crate::Readable for Cmpm7Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpm7::W`](W) writer structure"]
impl crate::Writable for Cmpm7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPM7 to value 0"]
impl crate::Resettable for Cmpm7Spec {
    const RESET_VALUE: u32 = 0;
}
