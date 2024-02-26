#[doc = "Register `CWR` reader"]
pub type R = crate::R<CwrSpec>;
#[doc = "Register `CWR` writer"]
pub type W = crate::W<CwrSpec>;
#[doc = "Field `LOWTHRES` reader - Low Threshold"]
pub type LowthresR = crate::FieldReader<u16>;
#[doc = "Field `LOWTHRES` writer - Low Threshold"]
pub type LowthresW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HIGHTHRES` reader - High Threshold"]
pub type HighthresR = crate::FieldReader<u16>;
#[doc = "Field `HIGHTHRES` writer - High Threshold"]
pub type HighthresW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&self) -> LowthresR {
        LowthresR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&self) -> HighthresR {
        HighthresR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lowthres(&mut self) -> LowthresW<CwrSpec> {
        LowthresW::new(self, 0)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn highthres(&mut self) -> HighthresW<CwrSpec> {
        HighthresW::new(self, 16)
    }
}
#[doc = "Compare Window Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwrSpec;
impl crate::RegisterSpec for CwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwr::R`](R) reader structure"]
impl crate::Readable for CwrSpec {}
#[doc = "`write(|w| ..)` method takes [`cwr::W`](W) writer structure"]
impl crate::Writable for CwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWR to value 0"]
impl crate::Resettable for CwrSpec {
    const RESET_VALUE: u32 = 0;
}
