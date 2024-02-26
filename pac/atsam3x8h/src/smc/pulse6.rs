#[doc = "Register `PULSE6` reader"]
pub type R = crate::R<Pulse6Spec>;
#[doc = "Register `PULSE6` writer"]
pub type W = crate::W<Pulse6Spec>;
#[doc = "Field `NWE_PULSE` reader - NWE Pulse Length"]
pub type NwePulseR = crate::FieldReader;
#[doc = "Field `NWE_PULSE` writer - NWE Pulse Length"]
pub type NwePulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NCS_WR_PULSE` reader - NCS Pulse Length in WRITE Access"]
pub type NcsWrPulseR = crate::FieldReader;
#[doc = "Field `NCS_WR_PULSE` writer - NCS Pulse Length in WRITE Access"]
pub type NcsWrPulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NRD_PULSE` reader - NRD Pulse Length"]
pub type NrdPulseR = crate::FieldReader;
#[doc = "Field `NRD_PULSE` writer - NRD Pulse Length"]
pub type NrdPulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NCS_RD_PULSE` reader - NCS Pulse Length in READ Access"]
pub type NcsRdPulseR = crate::FieldReader;
#[doc = "Field `NCS_RD_PULSE` writer - NCS Pulse Length in READ Access"]
pub type NcsRdPulseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    pub fn nwe_pulse(&self) -> NwePulseR {
        NwePulseR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_pulse(&self) -> NcsWrPulseR {
        NcsWrPulseR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    pub fn nrd_pulse(&self) -> NrdPulseR {
        NrdPulseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_pulse(&self) -> NcsRdPulseR {
        NcsRdPulseR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_pulse(&mut self) -> NwePulseW<Pulse6Spec> {
        NwePulseW::new(self, 0)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_wr_pulse(&mut self) -> NcsWrPulseW<Pulse6Spec> {
        NcsWrPulseW::new(self, 8)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_pulse(&mut self) -> NrdPulseW<Pulse6Spec> {
        NrdPulseW::new(self, 16)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_rd_pulse(&mut self) -> NcsRdPulseW<Pulse6Spec> {
        NcsRdPulseW::new(self, 24)
    }
}
#[doc = "SMC Pulse Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pulse6Spec;
impl crate::RegisterSpec for Pulse6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulse6::R`](R) reader structure"]
impl crate::Readable for Pulse6Spec {}
#[doc = "`write(|w| ..)` method takes [`pulse6::W`](W) writer structure"]
impl crate::Writable for Pulse6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PULSE6 to value 0x0101_0101"]
impl crate::Resettable for Pulse6Spec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
