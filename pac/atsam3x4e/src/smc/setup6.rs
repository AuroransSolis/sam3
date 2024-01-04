#[doc = "Register `SETUP6` reader"]
pub type R = crate::R<SETUP6_SPEC>;
#[doc = "Register `SETUP6` writer"]
pub type W = crate::W<SETUP6_SPEC>;
#[doc = "Field `NWE_SETUP` reader - NWE Setup Length"]
pub type NWE_SETUP_R = crate::FieldReader;
#[doc = "Field `NWE_SETUP` writer - NWE Setup Length"]
pub type NWE_SETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NCS_WR_SETUP` reader - NCS Setup Length in Write Access"]
pub type NCS_WR_SETUP_R = crate::FieldReader;
#[doc = "Field `NCS_WR_SETUP` writer - NCS Setup Length in Write Access"]
pub type NCS_WR_SETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NRD_SETUP` reader - NRD Setup Length"]
pub type NRD_SETUP_R = crate::FieldReader;
#[doc = "Field `NRD_SETUP` writer - NRD Setup Length"]
pub type NRD_SETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NCS_RD_SETUP` reader - NCS Setup Length in Read Access"]
pub type NCS_RD_SETUP_R = crate::FieldReader;
#[doc = "Field `NCS_RD_SETUP` writer - NCS Setup Length in Read Access"]
pub type NCS_RD_SETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    pub fn nwe_setup(&self) -> NWE_SETUP_R {
        NWE_SETUP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in Write Access"]
    #[inline(always)]
    pub fn ncs_wr_setup(&self) -> NCS_WR_SETUP_R {
        NCS_WR_SETUP_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    pub fn nrd_setup(&self) -> NRD_SETUP_R {
        NRD_SETUP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in Read Access"]
    #[inline(always)]
    pub fn ncs_rd_setup(&self) -> NCS_RD_SETUP_R {
        NCS_RD_SETUP_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_setup(&mut self) -> NWE_SETUP_W<SETUP6_SPEC> {
        NWE_SETUP_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in Write Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_wr_setup(&mut self) -> NCS_WR_SETUP_W<SETUP6_SPEC> {
        NCS_WR_SETUP_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_setup(&mut self) -> NRD_SETUP_W<SETUP6_SPEC> {
        NRD_SETUP_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in Read Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_rd_setup(&mut self) -> NCS_RD_SETUP_W<SETUP6_SPEC> {
        NCS_RD_SETUP_W::new(self, 24)
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
#[doc = "SMC Setup Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SETUP6_SPEC;
impl crate::RegisterSpec for SETUP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setup6::R`](R) reader structure"]
impl crate::Readable for SETUP6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`setup6::W`](W) writer structure"]
impl crate::Writable for SETUP6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETUP6 to value 0x0101_0101"]
impl crate::Resettable for SETUP6_SPEC {
    const RESET_VALUE: u32 = 0x0101_0101;
}
