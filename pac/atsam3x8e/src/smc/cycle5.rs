#[doc = "Register `CYCLE5` reader"]
pub type R = crate::R<CYCLE5_SPEC>;
#[doc = "Register `CYCLE5` writer"]
pub type W = crate::W<CYCLE5_SPEC>;
#[doc = "Field `NWE_CYCLE` reader - Total Write Cycle Length"]
pub type NWE_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `NWE_CYCLE` writer - Total Write Cycle Length"]
pub type NWE_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NRD_CYCLE` reader - Total Read Cycle Length"]
pub type NRD_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `NRD_CYCLE` writer - Total Read Cycle Length"]
pub type NRD_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&self) -> NWE_CYCLE_R {
        NWE_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&self) -> NRD_CYCLE_R {
        NRD_CYCLE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_cycle(&mut self) -> NWE_CYCLE_W<CYCLE5_SPEC> {
        NWE_CYCLE_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_cycle(&mut self) -> NRD_CYCLE_W<CYCLE5_SPEC> {
        NRD_CYCLE_W::new(self, 16)
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
#[doc = "SMC Cycle Register (CS_number = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CYCLE5_SPEC;
impl crate::RegisterSpec for CYCLE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycle5::R`](R) reader structure"]
impl crate::Readable for CYCLE5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cycle5::W`](W) writer structure"]
impl crate::Writable for CYCLE5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CYCLE5 to value 0x0003_0003"]
impl crate::Resettable for CYCLE5_SPEC {
    const RESET_VALUE: u32 = 0x0003_0003;
}
