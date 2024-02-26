#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `DET_SUSPD` reader - Suspend Interrupt Enable"]
pub type DetSuspdR = crate::BitReader;
#[doc = "Field `DET_SUSPD` writer - Suspend Interrupt Enable"]
pub type DetSuspdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICRO_SOF` reader - Micro-SOF Interrupt Enable"]
pub type MicroSofR = crate::BitReader;
#[doc = "Field `MICRO_SOF` writer - Micro-SOF Interrupt Enable"]
pub type MicroSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_SOF` reader - SOF Interrupt Enable"]
pub type IntSofR = crate::BitReader;
#[doc = "Field `INT_SOF` writer - SOF Interrupt Enable"]
pub type IntSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRESET` reader - End Of Reset Interrupt Enable"]
pub type EndresetR = crate::BitReader;
#[doc = "Field `ENDRESET` writer - End Of Reset Interrupt Enable"]
pub type EndresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE_UP` reader - Wake Up CPU Interrupt Enable"]
pub type WakeUpR = crate::BitReader;
#[doc = "Field `WAKE_UP` writer - Wake Up CPU Interrupt Enable"]
pub type WakeUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDOFRSM` reader - End Of Resume Interrupt Enable"]
pub type EndofrsmR = crate::BitReader;
#[doc = "Field `ENDOFRSM` writer - End Of Resume Interrupt Enable"]
pub type EndofrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPSTR_RES` reader - Upstream Resume Interrupt Enable"]
pub type UpstrResR = crate::BitReader;
#[doc = "Field `UPSTR_RES` writer - Upstream Resume Interrupt Enable"]
pub type UpstrResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_0` reader - Endpoint 0 Interrupt Enable"]
pub type Ept0R = crate::BitReader;
#[doc = "Field `EPT_0` writer - Endpoint 0 Interrupt Enable"]
pub type Ept0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_1` reader - Endpoint 1 Interrupt Enable"]
pub type Ept1R = crate::BitReader;
#[doc = "Field `EPT_1` writer - Endpoint 1 Interrupt Enable"]
pub type Ept1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_2` reader - Endpoint 2 Interrupt Enable"]
pub type Ept2R = crate::BitReader;
#[doc = "Field `EPT_2` writer - Endpoint 2 Interrupt Enable"]
pub type Ept2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_3` reader - Endpoint 3 Interrupt Enable"]
pub type Ept3R = crate::BitReader;
#[doc = "Field `EPT_3` writer - Endpoint 3 Interrupt Enable"]
pub type Ept3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_4` reader - Endpoint 4 Interrupt Enable"]
pub type Ept4R = crate::BitReader;
#[doc = "Field `EPT_4` writer - Endpoint 4 Interrupt Enable"]
pub type Ept4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_5` reader - Endpoint 5 Interrupt Enable"]
pub type Ept5R = crate::BitReader;
#[doc = "Field `EPT_5` writer - Endpoint 5 Interrupt Enable"]
pub type Ept5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_6` reader - Endpoint 6 Interrupt Enable"]
pub type Ept6R = crate::BitReader;
#[doc = "Field `EPT_6` writer - Endpoint 6 Interrupt Enable"]
pub type Ept6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt Enable"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Enable"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt Enable"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Enable"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt Enable"]
pub type Dma3R = crate::BitReader;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Enable"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt Enable"]
pub type Dma4R = crate::BitReader;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Enable"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt Enable"]
pub type Dma5R = crate::BitReader;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Enable"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt Enable"]
pub type Dma6R = crate::BitReader;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Enable"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn det_suspd(&self) -> DetSuspdR {
        DetSuspdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Micro-SOF Interrupt Enable"]
    #[inline(always)]
    pub fn micro_sof(&self) -> MicroSofR {
        MicroSofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn int_sof(&self) -> IntSofR {
        IntSofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn endreset(&self) -> EndresetR {
        EndresetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Enable"]
    #[inline(always)]
    pub fn wake_up(&self) -> WakeUpR {
        WakeUpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn endofrsm(&self) -> EndofrsmR {
        EndofrsmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn upstr_res(&self) -> UpstrResR {
        UpstrResR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_0(&self) -> Ept0R {
        Ept0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_1(&self) -> Ept1R {
        Ept1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_2(&self) -> Ept2R {
        Ept2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_3(&self) -> Ept3R {
        Ept3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_4(&self) -> Ept4R {
        Ept4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_5(&self) -> Ept5R {
        Ept5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_6(&self) -> Ept6R {
        Ept6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&self) -> Dma4R {
        Dma4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&self) -> Dma5R {
        Dma5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&self) -> Dma6R {
        Dma6R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Suspend Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn det_suspd(&mut self) -> DetSuspdW<IenSpec> {
        DetSuspdW::new(self, 1)
    }
    #[doc = "Bit 2 - Micro-SOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn micro_sof(&mut self) -> MicroSofW<IenSpec> {
        MicroSofW::new(self, 2)
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int_sof(&mut self) -> IntSofW<IenSpec> {
        IntSofW::new(self, 3)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endreset(&mut self) -> EndresetW<IenSpec> {
        EndresetW::new(self, 4)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wake_up(&mut self) -> WakeUpW<IenSpec> {
        WakeUpW::new(self, 5)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endofrsm(&mut self) -> EndofrsmW<IenSpec> {
        EndofrsmW::new(self, 6)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upstr_res(&mut self) -> UpstrResW<IenSpec> {
        UpstrResW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_0(&mut self) -> Ept0W<IenSpec> {
        Ept0W::new(self, 8)
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_1(&mut self) -> Ept1W<IenSpec> {
        Ept1W::new(self, 9)
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_2(&mut self) -> Ept2W<IenSpec> {
        Ept2W::new(self, 10)
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_3(&mut self) -> Ept3W<IenSpec> {
        Ept3W::new(self, 11)
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_4(&mut self) -> Ept4W<IenSpec> {
        Ept4W::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_5(&mut self) -> Ept5W<IenSpec> {
        Ept5W::new(self, 13)
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ept_6(&mut self) -> Ept6W<IenSpec> {
        Ept6W::new(self, 14)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<IenSpec> {
        Dma1W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<IenSpec> {
        Dma2W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<IenSpec> {
        Dma3W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<IenSpec> {
        Dma4W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<IenSpec> {
        Dma5W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<IenSpec> {
        Dma6W::new(self, 30)
    }
}
#[doc = "UDPHS Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0x10"]
impl crate::Resettable for IenSpec {
    const RESET_VALUE: u32 = 0x10;
}
