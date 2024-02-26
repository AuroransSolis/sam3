#[doc = "Register `INTSTA` reader"]
pub type R = crate::R<IntstaSpec>;
#[doc = "Field `SPEED` reader - Speed Status"]
pub type SpeedR = crate::BitReader;
#[doc = "Field `DET_SUSPD` reader - Suspend Interrupt"]
pub type DetSuspdR = crate::BitReader;
#[doc = "Field `MICRO_SOF` reader - Micro Start Of Frame Interrupt"]
pub type MicroSofR = crate::BitReader;
#[doc = "Field `INT_SOF` reader - Start Of Frame Interrupt"]
pub type IntSofR = crate::BitReader;
#[doc = "Field `ENDRESET` reader - End Of Reset Interrupt"]
pub type EndresetR = crate::BitReader;
#[doc = "Field `WAKE_UP` reader - Wake Up CPU Interrupt"]
pub type WakeUpR = crate::BitReader;
#[doc = "Field `ENDOFRSM` reader - End Of Resume Interrupt"]
pub type EndofrsmR = crate::BitReader;
#[doc = "Field `UPSTR_RES` reader - Upstream Resume Interrupt"]
pub type UpstrResR = crate::BitReader;
#[doc = "Field `EPT_0` reader - Endpoint 0 Interrupt"]
pub type Ept0R = crate::BitReader;
#[doc = "Field `EPT_1` reader - Endpoint 1 Interrupt"]
pub type Ept1R = crate::BitReader;
#[doc = "Field `EPT_2` reader - Endpoint 2 Interrupt"]
pub type Ept2R = crate::BitReader;
#[doc = "Field `EPT_3` reader - Endpoint 3 Interrupt"]
pub type Ept3R = crate::BitReader;
#[doc = "Field `EPT_4` reader - Endpoint 4 Interrupt"]
pub type Ept4R = crate::BitReader;
#[doc = "Field `EPT_5` reader - Endpoint 5 Interrupt"]
pub type Ept5R = crate::BitReader;
#[doc = "Field `EPT_6` reader - Endpoint 6 Interrupt"]
pub type Ept6R = crate::BitReader;
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt"]
pub type Dma3R = crate::BitReader;
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt"]
pub type Dma4R = crate::BitReader;
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt"]
pub type Dma5R = crate::BitReader;
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt"]
pub type Dma6R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend Interrupt"]
    #[inline(always)]
    pub fn det_suspd(&self) -> DetSuspdR {
        DetSuspdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Micro Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn micro_sof(&self) -> MicroSofR {
        MicroSofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn int_sof(&self) -> IntSofR {
        IntSofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt"]
    #[inline(always)]
    pub fn endreset(&self) -> EndresetR {
        EndresetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt"]
    #[inline(always)]
    pub fn wake_up(&self) -> WakeUpR {
        WakeUpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt"]
    #[inline(always)]
    pub fn endofrsm(&self) -> EndofrsmR {
        EndofrsmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt"]
    #[inline(always)]
    pub fn upstr_res(&self) -> UpstrResR {
        UpstrResR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ept_0(&self) -> Ept0R {
        Ept0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ept_1(&self) -> Ept1R {
        Ept1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ept_2(&self) -> Ept2R {
        Ept2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ept_3(&self) -> Ept3R {
        Ept3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ept_4(&self) -> Ept4R {
        Ept4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ept_5(&self) -> Ept5R {
        Ept5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ept_6(&self) -> Ept6R {
        Ept6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> Dma4R {
        Dma4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> Dma5R {
        Dma5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> Dma6R {
        Dma6R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "UDPHS Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstaSpec;
impl crate::RegisterSpec for IntstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsta::R`](R) reader structure"]
impl crate::Readable for IntstaSpec {}
#[doc = "`reset()` method sets INTSTA to value 0"]
impl crate::Resettable for IntstaSpec {
    const RESET_VALUE: u32 = 0;
}
