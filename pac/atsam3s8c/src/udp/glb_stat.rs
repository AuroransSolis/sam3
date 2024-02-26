#[doc = "Register `GLB_STAT` reader"]
pub type R = crate::R<GlbStatSpec>;
#[doc = "Register `GLB_STAT` writer"]
pub type W = crate::W<GlbStatSpec>;
#[doc = "Field `FADDEN` reader - Function Address Enable"]
pub type FaddenR = crate::BitReader;
#[doc = "Field `FADDEN` writer - Function Address Enable"]
pub type FaddenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFG` reader - Configured"]
pub type ConfgR = crate::BitReader;
#[doc = "Field `CONFG` writer - Configured"]
pub type ConfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESR` reader - Enable Send Resume"]
pub type EsrR = crate::BitReader;
#[doc = "Field `ESR` writer - Enable Send Resume"]
pub type EsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMINPR` reader - "]
pub type RsminprR = crate::BitReader;
#[doc = "Field `RSMINPR` writer - "]
pub type RsminprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMWUPE` reader - Remote Wake Up Enable"]
pub type RmwupeR = crate::BitReader;
#[doc = "Field `RMWUPE` writer - Remote Wake Up Enable"]
pub type RmwupeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    pub fn fadden(&self) -> FaddenR {
        FaddenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    pub fn confg(&self) -> ConfgR {
        ConfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    pub fn esr(&self) -> EsrR {
        EsrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsminpr(&self) -> RsminprR {
        RsminprR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    pub fn rmwupe(&self) -> RmwupeR {
        RmwupeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fadden(&mut self) -> FaddenW<GlbStatSpec> {
        FaddenW::new(self, 0)
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    #[must_use]
    pub fn confg(&mut self) -> ConfgW<GlbStatSpec> {
        ConfgW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    #[must_use]
    pub fn esr(&mut self) -> EsrW<GlbStatSpec> {
        EsrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rsminpr(&mut self) -> RsminprW<GlbStatSpec> {
        RsminprW::new(self, 3)
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmwupe(&mut self) -> RmwupeW<GlbStatSpec> {
        RmwupeW::new(self, 4)
    }
}
#[doc = "Global State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbStatSpec;
impl crate::RegisterSpec for GlbStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_stat::R`](R) reader structure"]
impl crate::Readable for GlbStatSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_stat::W`](W) writer structure"]
impl crate::Writable for GlbStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_STAT to value 0x10"]
impl crate::Resettable for GlbStatSpec {
    const RESET_VALUE: u32 = 0x10;
}
