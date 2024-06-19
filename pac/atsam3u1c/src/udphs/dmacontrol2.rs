#[doc = "Register `DMACONTROL2` reader"]
pub type R = crate::R<Dmacontrol2Spec>;
#[doc = "Register `DMACONTROL2` writer"]
pub type W = crate::W<Dmacontrol2Spec>;
#[doc = "Field `CHANN_ENB` reader - (Channel Enable Command)"]
pub type ChannEnbR = crate::BitReader;
#[doc = "Field `CHANN_ENB` writer - (Channel Enable Command)"]
pub type ChannEnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDNXT_DSC` reader - Load Next Channel Transfer Descriptor Enable (Command)"]
pub type LdnxtDscR = crate::BitReader;
#[doc = "Field `LDNXT_DSC` writer - Load Next Channel Transfer Descriptor Enable (Command)"]
pub type LdnxtDscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_TR_EN` reader - End of Transfer Enable (Control)"]
pub type EndTrEnR = crate::BitReader;
#[doc = "Field `END_TR_EN` writer - End of Transfer Enable (Control)"]
pub type EndTrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_B_EN` reader - End of Buffer Enable (Control)"]
pub type EndBEnR = crate::BitReader;
#[doc = "Field `END_B_EN` writer - End of Buffer Enable (Control)"]
pub type EndBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_TR_IT` reader - End of Transfer Interrupt Enable"]
pub type EndTrItR = crate::BitReader;
#[doc = "Field `END_TR_IT` writer - End of Transfer Interrupt Enable"]
pub type EndTrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_BUFFIT` reader - End of Buffer Interrupt Enable"]
pub type EndBuffitR = crate::BitReader;
#[doc = "Field `END_BUFFIT` writer - End of Buffer Interrupt Enable"]
pub type EndBuffitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LD_IT` reader - Descriptor Loaded Interrupt Enable"]
pub type DescLdItR = crate::BitReader;
#[doc = "Field `DESC_LD_IT` writer - Descriptor Loaded Interrupt Enable"]
pub type DescLdItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_LCK` reader - Burst Lock Enable"]
pub type BurstLckR = crate::BitReader;
#[doc = "Field `BURST_LCK` writer - Burst Lock Enable"]
pub type BurstLckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_LENGTH` reader - Buffer Byte Length (Write-only)"]
pub type BuffLengthR = crate::FieldReader<u16>;
#[doc = "Field `BUFF_LENGTH` writer - Buffer Byte Length (Write-only)"]
pub type BuffLengthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - (Channel Enable Command)"]
    #[inline(always)]
    pub fn chann_enb(&self) -> ChannEnbR {
        ChannEnbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable (Command)"]
    #[inline(always)]
    pub fn ldnxt_dsc(&self) -> LdnxtDscR {
        LdnxtDscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Transfer Enable (Control)"]
    #[inline(always)]
    pub fn end_tr_en(&self) -> EndTrEnR {
        EndTrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Buffer Enable (Control)"]
    #[inline(always)]
    pub fn end_b_en(&self) -> EndBEnR {
        EndBEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn end_tr_it(&self) -> EndTrItR {
        EndTrItR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn end_buffit(&self) -> EndBuffitR {
        EndBuffitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    pub fn desc_ld_it(&self) -> DescLdItR {
        DescLdItR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    pub fn burst_lck(&self) -> BurstLckR {
        BurstLckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    pub fn buff_length(&self) -> BuffLengthR {
        BuffLengthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - (Channel Enable Command)"]
    #[inline(always)]
    #[must_use]
    pub fn chann_enb(&mut self) -> ChannEnbW<Dmacontrol2Spec> {
        ChannEnbW::new(self, 0)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable (Command)"]
    #[inline(always)]
    #[must_use]
    pub fn ldnxt_dsc(&mut self) -> LdnxtDscW<Dmacontrol2Spec> {
        LdnxtDscW::new(self, 1)
    }
    #[doc = "Bit 2 - End of Transfer Enable (Control)"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_en(&mut self) -> EndTrEnW<Dmacontrol2Spec> {
        EndTrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Buffer Enable (Control)"]
    #[inline(always)]
    #[must_use]
    pub fn end_b_en(&mut self) -> EndBEnW<Dmacontrol2Spec> {
        EndBEnW::new(self, 3)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_it(&mut self) -> EndTrItW<Dmacontrol2Spec> {
        EndTrItW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn end_buffit(&mut self) -> EndBuffitW<Dmacontrol2Spec> {
        EndBuffitW::new(self, 5)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn desc_ld_it(&mut self) -> DescLdItW<Dmacontrol2Spec> {
        DescLdItW::new(self, 6)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn burst_lck(&mut self) -> BurstLckW<Dmacontrol2Spec> {
        BurstLckW::new(self, 7)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn buff_length(&mut self) -> BuffLengthW<Dmacontrol2Spec> {
        BuffLengthW::new(self, 16)
    }
}
#[doc = "UDPHS DMA Channel Control Register (channel = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacontrol2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacontrol2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmacontrol2Spec;
impl crate::RegisterSpec for Dmacontrol2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacontrol2::R`](R) reader structure"]
impl crate::Readable for Dmacontrol2Spec {}
#[doc = "`write(|w| ..)` method takes [`dmacontrol2::W`](W) writer structure"]
impl crate::Writable for Dmacontrol2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACONTROL2 to value 0"]
impl crate::Resettable for Dmacontrol2Spec {
    const RESET_VALUE: u32 = 0;
}
