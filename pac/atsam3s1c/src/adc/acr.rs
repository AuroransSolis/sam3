#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `TSON` reader - Temperature Sensor On"]
pub type TsonR = crate::BitReader;
#[doc = "Field `TSON` writer - Temperature Sensor On"]
pub type TsonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBCTL` reader - ADC Bias Current Control"]
pub type IbctlR = crate::FieldReader;
#[doc = "Field `IBCTL` writer - ADC Bias Current Control"]
pub type IbctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 4 - Temperature Sensor On"]
    #[inline(always)]
    pub fn tson(&self) -> TsonR {
        TsonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ADC Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IbctlR {
        IbctlR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Temperature Sensor On"]
    #[inline(always)]
    #[must_use]
    pub fn tson(&mut self) -> TsonW<AcrSpec> {
        TsonW::new(self, 4)
    }
    #[doc = "Bits 8:9 - ADC Bias Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctl(&mut self) -> IbctlW<AcrSpec> {
        IbctlW::new(self, 8)
    }
}
#[doc = "Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0x0100"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
