#[doc = "Register `CYCLE0` reader"]
pub type R = crate::R<Cycle0Spec>;
#[doc = "Register `CYCLE0` writer"]
pub type W = crate::W<Cycle0Spec>;
#[doc = "Field `NWE_CYCLE` reader - Total Write Cycle Length"]
pub type NweCycleR = crate::FieldReader<u16>;
#[doc = "Field `NWE_CYCLE` writer - Total Write Cycle Length"]
pub type NweCycleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NRD_CYCLE` reader - Total Read Cycle Length"]
pub type NrdCycleR = crate::FieldReader<u16>;
#[doc = "Field `NRD_CYCLE` writer - Total Read Cycle Length"]
pub type NrdCycleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&self) -> NweCycleR {
        NweCycleR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&self) -> NrdCycleR {
        NrdCycleR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_cycle(&mut self) -> NweCycleW<Cycle0Spec> {
        NweCycleW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_cycle(&mut self) -> NrdCycleW<Cycle0Spec> {
        NrdCycleW::new(self, 16)
    }
}
#[doc = "SMC Cycle Register (CS_number = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cycle0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycle0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cycle0Spec;
impl crate::RegisterSpec for Cycle0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycle0::R`](R) reader structure"]
impl crate::Readable for Cycle0Spec {}
#[doc = "`write(|w| ..)` method takes [`cycle0::W`](W) writer structure"]
impl crate::Writable for Cycle0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CYCLE0 to value 0x0003_0003"]
impl crate::Resettable for Cycle0Spec {
    const RESET_VALUE: u32 = 0x0003_0003;
}
