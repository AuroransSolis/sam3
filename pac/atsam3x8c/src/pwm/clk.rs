#[doc = "Register `CLK` reader"]
pub type R = crate::R<ClkSpec>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<ClkSpec>;
#[doc = "Field `DIVA` reader - CLKA, CLKB Divide Factor"]
pub type DivaR = crate::FieldReader;
#[doc = "Field `DIVA` writer - CLKA, CLKB Divide Factor"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PREA` reader - CLKA, CLKB Source Clock Selection"]
pub type PreaR = crate::FieldReader;
#[doc = "Field `PREA` writer - CLKA, CLKB Source Clock Selection"]
pub type PreaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVB` reader - CLKA, CLKB Divide Factor"]
pub type DivbR = crate::FieldReader;
#[doc = "Field `DIVB` writer - CLKA, CLKB Divide Factor"]
pub type DivbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PREB` reader - CLKA, CLKB Source Clock Selection"]
pub type PrebR = crate::FieldReader;
#[doc = "Field `PREB` writer - CLKA, CLKB Source Clock Selection"]
pub type PrebW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&self) -> PreaR {
        PreaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DivbR {
        DivbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&self) -> PrebR {
        PrebR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DivaW<ClkSpec> {
        DivaW::new(self, 0)
    }
    #[doc = "Bits 8:11 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prea(&mut self) -> PreaW<ClkSpec> {
        PreaW::new(self, 8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DivbW<ClkSpec> {
        DivbW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CLKA, CLKB Source Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn preb(&mut self) -> PrebW<ClkSpec> {
        PrebW::new(self, 24)
    }
}
#[doc = "PWM Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSpec;
impl crate::RegisterSpec for ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for ClkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for ClkSpec {
    const RESET_VALUE: u32 = 0;
}
