#[doc = "Register `BLKR` reader"]
pub type R = crate::R<BlkrSpec>;
#[doc = "Register `BLKR` writer"]
pub type W = crate::W<BlkrSpec>;
#[doc = "Field `BCNT` reader - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `BCNT` writer - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BLKLEN` reader - Data Block Length"]
pub type BlklenR = crate::FieldReader<u16>;
#[doc = "Field `BLKLEN` writer - Data Block Length"]
pub type BlklenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&self) -> BlklenR {
        BlklenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    #[must_use]
    pub fn bcnt(&mut self) -> BcntW<BlkrSpec> {
        BcntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn blklen(&mut self) -> BlklenW<BlkrSpec> {
        BlklenW::new(self, 16)
    }
}
#[doc = "Block Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkrSpec;
impl crate::RegisterSpec for BlkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blkr::R`](R) reader structure"]
impl crate::Readable for BlkrSpec {}
#[doc = "`write(|w| ..)` method takes [`blkr::W`](W) writer structure"]
impl crate::Writable for BlkrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLKR to value 0"]
impl crate::Resettable for BlkrSpec {
    const RESET_VALUE: u32 = 0;
}
