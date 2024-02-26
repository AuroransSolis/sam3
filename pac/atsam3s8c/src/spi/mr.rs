#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub type MstrR = crate::BitReader;
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Peripheral Select"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Peripheral Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub type PcsdecR = crate::BitReader;
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub type PcsdecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub type ModfdisR = crate::BitReader;
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub type ModfdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WdrbtR = crate::BitReader;
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WdrbtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LlbR = crate::BitReader;
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LlbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PcsR = crate::FieldReader;
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub type DlybcsR = crate::FieldReader;
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub type DlybcsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PcsdecR {
        PcsdecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> ModfdisR {
        ModfdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WdrbtR {
        WdrbtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LlbR {
        LlbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DlybcsR {
        DlybcsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MstrW<MrSpec> {
        MstrW::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<MrSpec> {
        PsW::new(self, 1)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    #[must_use]
    pub fn pcsdec(&mut self) -> PcsdecW<MrSpec> {
        PcsdecW::new(self, 2)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    #[must_use]
    pub fn modfdis(&mut self) -> ModfdisW<MrSpec> {
        ModfdisW::new(self, 4)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wdrbt(&mut self) -> WdrbtW<MrSpec> {
        WdrbtW::new(self, 5)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn llb(&mut self) -> LlbW<MrSpec> {
        LlbW::new(self, 7)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PcsW<MrSpec> {
        PcsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    #[must_use]
    pub fn dlybcs(&mut self) -> DlybcsW<MrSpec> {
        DlybcsW::new(self, 24)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
