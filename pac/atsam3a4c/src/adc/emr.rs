#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpmode {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    Low = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    High = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    In = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    Out = 3,
}
impl From<Cmpmode> for u8 {
    #[inline(always)]
    fn from(variant: Cmpmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpmode {
    type Ux = u8;
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CmpmodeR = crate::FieldReader<Cmpmode>;
impl CmpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmode {
        match self.bits {
            0 => Cmpmode::Low,
            1 => Cmpmode::High,
            2 => Cmpmode::In,
            3 => Cmpmode::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cmpmode::Low
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cmpmode::High
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Cmpmode::In
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Cmpmode::Out
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CmpmodeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cmpmode>;
impl<'a, REG> CmpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::Low)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::High)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::In)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmode::Out)
    }
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub type CmpselR = crate::FieldReader;
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub type CmpselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub type CmpallR = crate::BitReader;
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub type CmpallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub type CmpfilterR = crate::FieldReader;
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub type CmpfilterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAG` reader - TAG of the ADC_LDCR register"]
pub type TagR = crate::BitReader;
#[doc = "Field `TAG` writer - TAG of the ADC_LDCR register"]
pub type TagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CmpmodeR {
        CmpmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CmpselR {
        CmpselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CmpallR {
        CmpallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CmpfilterR {
        CmpfilterR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 24 - TAG of the ADC_LDCR register"]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmode(&mut self) -> CmpmodeW<EmrSpec> {
        CmpmodeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel(&mut self) -> CmpselW<EmrSpec> {
        CmpselW::new(self, 4)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    #[must_use]
    pub fn cmpall(&mut self) -> CmpallW<EmrSpec> {
        CmpallW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfilter(&mut self) -> CmpfilterW<EmrSpec> {
        CmpfilterW::new(self, 12)
    }
    #[doc = "Bit 24 - TAG of the ADC_LDCR register"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<EmrSpec> {
        TagW::new(self, 24)
    }
}
#[doc = "Extended Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {
    const RESET_VALUE: u32 = 0;
}
