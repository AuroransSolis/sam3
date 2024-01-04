#[doc = "Register `DMASTATUS3` reader"]
pub type R = crate::R<DMASTATUS3_SPEC>;
#[doc = "Register `DMASTATUS3` writer"]
pub type W = crate::W<DMASTATUS3_SPEC>;
#[doc = "Field `CHANN_ENB` reader - Channel Enable Status"]
pub type CHANN_ENB_R = crate::BitReader;
#[doc = "Field `CHANN_ENB` writer - Channel Enable Status"]
pub type CHANN_ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANN_ACT` reader - Channel Active Status"]
pub type CHANN_ACT_R = crate::BitReader;
#[doc = "Field `CHANN_ACT` writer - Channel Active Status"]
pub type CHANN_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_TR_ST` reader - End of Channel Transfer Status"]
pub type END_TR_ST_R = crate::BitReader;
#[doc = "Field `END_TR_ST` writer - End of Channel Transfer Status"]
pub type END_TR_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_BF_ST` reader - End of Channel Buffer Status"]
pub type END_BF_ST_R = crate::BitReader;
#[doc = "Field `END_BF_ST` writer - End of Channel Buffer Status"]
pub type END_BF_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LDST` reader - Descriptor Loaded Status"]
pub type DESC_LDST_R = crate::BitReader;
#[doc = "Field `DESC_LDST` writer - Descriptor Loaded Status"]
pub type DESC_LDST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_COUNT` reader - Buffer Byte Count"]
pub type BUFF_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `BUFF_COUNT` writer - Buffer Byte Count"]
pub type BUFF_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&self) -> CHANN_ACT_R {
        CHANN_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&self) -> END_TR_ST_R {
        END_TR_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&self) -> END_BF_ST_R {
        END_BF_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&self) -> DESC_LDST_R {
        DESC_LDST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&self) -> BUFF_COUNT_R {
        BUFF_COUNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    #[must_use]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W<DMASTATUS3_SPEC> {
        CHANN_ENB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    #[must_use]
    pub fn chann_act(&mut self) -> CHANN_ACT_W<DMASTATUS3_SPEC> {
        CHANN_ACT_W::new(self, 1)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_st(&mut self) -> END_TR_ST_W<DMASTATUS3_SPEC> {
        END_TR_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    #[must_use]
    pub fn end_bf_st(&mut self) -> END_BF_ST_W<DMASTATUS3_SPEC> {
        END_BF_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    #[must_use]
    pub fn desc_ldst(&mut self) -> DESC_LDST_W<DMASTATUS3_SPEC> {
        DESC_LDST_W::new(self, 6)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    #[must_use]
    pub fn buff_count(&mut self) -> BUFF_COUNT_W<DMASTATUS3_SPEC> {
        BUFF_COUNT_W::new(self, 16)
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
#[doc = "UDPHS DMA Channel Status Register (channel = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASTATUS3_SPEC;
impl crate::RegisterSpec for DMASTATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmastatus3::R`](R) reader structure"]
impl crate::Readable for DMASTATUS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmastatus3::W`](W) writer structure"]
impl crate::Writable for DMASTATUS3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASTATUS3 to value 0"]
impl crate::Resettable for DMASTATUS3_SPEC {
    const RESET_VALUE: u32 = 0;
}
