#[doc = "Register `DEVDMACONTROL4` reader"]
pub type R = crate::R<DEVDMACONTROL4_SPEC>;
#[doc = "Register `DEVDMACONTROL4` writer"]
pub type W = crate::W<DEVDMACONTROL4_SPEC>;
#[doc = "Field `CHANN_ENB` reader - Channel Enable Command"]
pub type CHANN_ENB_R = crate::BitReader;
#[doc = "Field `CHANN_ENB` writer - Channel Enable Command"]
pub type CHANN_ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDNXT_DSC` reader - Load Next Channel Transfer Descriptor Enable Command"]
pub type LDNXT_DSC_R = crate::BitReader;
#[doc = "Field `LDNXT_DSC` writer - Load Next Channel Transfer Descriptor Enable Command"]
pub type LDNXT_DSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_TR_EN` reader - End of Transfer Enable Control"]
pub type END_TR_EN_R = crate::BitReader;
#[doc = "Field `END_TR_EN` writer - End of Transfer Enable Control"]
pub type END_TR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_B_EN` reader - End of Buffer Enable Control"]
pub type END_B_EN_R = crate::BitReader;
#[doc = "Field `END_B_EN` writer - End of Buffer Enable Control"]
pub type END_B_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_TR_IT` reader - End of Transfer Interrupt Enable"]
pub type END_TR_IT_R = crate::BitReader;
#[doc = "Field `END_TR_IT` writer - End of Transfer Interrupt Enable"]
pub type END_TR_IT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_BUFFIT` reader - End of Buffer Interrupt Enable"]
pub type END_BUFFIT_R = crate::BitReader;
#[doc = "Field `END_BUFFIT` writer - End of Buffer Interrupt Enable"]
pub type END_BUFFIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LD_IT` reader - Descriptor Loaded Interrupt Enable"]
pub type DESC_LD_IT_R = crate::BitReader;
#[doc = "Field `DESC_LD_IT` writer - Descriptor Loaded Interrupt Enable"]
pub type DESC_LD_IT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_LCK` reader - Burst Lock Enable"]
pub type BURST_LCK_R = crate::BitReader;
#[doc = "Field `BURST_LCK` writer - Burst Lock Enable"]
pub type BURST_LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_LENGTH` reader - Buffer Byte Length (Write-only)"]
pub type BUFF_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `BUFF_LENGTH` writer - Buffer Byte Length (Write-only)"]
pub type BUFF_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    pub fn ldnxt_dsc(&self) -> LDNXT_DSC_R {
        LDNXT_DSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Transfer Enable Control"]
    #[inline(always)]
    pub fn end_tr_en(&self) -> END_TR_EN_R {
        END_TR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    pub fn end_b_en(&self) -> END_B_EN_R {
        END_B_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn end_tr_it(&self) -> END_TR_IT_R {
        END_TR_IT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn end_buffit(&self) -> END_BUFFIT_R {
        END_BUFFIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    pub fn desc_ld_it(&self) -> DESC_LD_IT_R {
        DESC_LD_IT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    pub fn burst_lck(&self) -> BURST_LCK_R {
        BURST_LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    pub fn buff_length(&self) -> BUFF_LENGTH_R {
        BUFF_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W<DEVDMACONTROL4_SPEC> {
        CHANN_ENB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn ldnxt_dsc(&mut self) -> LDNXT_DSC_W<DEVDMACONTROL4_SPEC> {
        LDNXT_DSC_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of Transfer Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_en(&mut self) -> END_TR_EN_W<DEVDMACONTROL4_SPEC> {
        END_TR_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn end_b_en(&mut self) -> END_B_EN_W<DEVDMACONTROL4_SPEC> {
        END_B_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_it(&mut self) -> END_TR_IT_W<DEVDMACONTROL4_SPEC> {
        END_TR_IT_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn end_buffit(&mut self) -> END_BUFFIT_W<DEVDMACONTROL4_SPEC> {
        END_BUFFIT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn desc_ld_it(&mut self) -> DESC_LD_IT_W<DEVDMACONTROL4_SPEC> {
        DESC_LD_IT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn burst_lck(&mut self) -> BURST_LCK_W<DEVDMACONTROL4_SPEC> {
        BURST_LCK_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn buff_length(&mut self) -> BUFF_LENGTH_W<DEVDMACONTROL4_SPEC> {
        BUFF_LENGTH_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device DMA Channel Control Register (n = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVDMACONTROL4_SPEC;
impl crate::RegisterSpec for DEVDMACONTROL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmacontrol4::R`](R) reader structure"]
impl crate::Readable for DEVDMACONTROL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devdmacontrol4::W`](W) writer structure"]
impl crate::Writable for DEVDMACONTROL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVDMACONTROL4 to value 0"]
impl crate::Resettable for DEVDMACONTROL4_SPEC {
    const RESET_VALUE: u32 = 0;
}
