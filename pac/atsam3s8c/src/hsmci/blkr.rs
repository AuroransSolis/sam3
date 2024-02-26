#[doc = "Register `BLKR` reader"]
pub type R = crate::R<BlkrSpec>;
#[doc = "Register `BLKR` writer"]
pub type W = crate::W<BlkrSpec>;
#[doc = "MMC/SDIO Block Count - SDIO Byte Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Bcnt {
    #[doc = "0: MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    Multiple = 0,
    #[doc = "4: SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    Byte = 4,
    #[doc = "5: SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    Block = 5,
}
impl From<Bcnt> for u16 {
    #[inline(always)]
    fn from(variant: Bcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcnt {
    type Ux = u16;
}
#[doc = "Field `BCNT` reader - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BcntR = crate::FieldReader<Bcnt>;
impl BcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bcnt> {
        match self.bits {
            0 => Some(Bcnt::Multiple),
            4 => Some(Bcnt::Byte),
            5 => Some(Bcnt::Block),
            _ => None,
        }
    }
    #[doc = "MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == Bcnt::Multiple
    }
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Bcnt::Byte
    }
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Bcnt::Block
    }
}
#[doc = "Field `BCNT` writer - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, Bcnt>;
impl<'a, REG> BcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Bcnt::Multiple)
    }
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Bcnt::Byte)
    }
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Bcnt::Block)
    }
}
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
