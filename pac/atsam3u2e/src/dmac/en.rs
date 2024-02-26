#[doc = "Register `EN` reader"]
pub type R = crate::R<EnSpec>;
#[doc = "Register `EN` writer"]
pub type W = crate::W<EnSpec>;
#[doc = "Field `ENABLE` reader - General Enable of DMA"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - General Enable of DMA"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - General Enable of DMA"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Enable of DMA"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<EnSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "DMAC Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnSpec;
impl crate::RegisterSpec for EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EnSpec {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EnSpec {
    const RESET_VALUE: u32 = 0;
}
