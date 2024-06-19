#[doc = "Register `DMASTATUS5` reader"]
pub type R = crate::R<Dmastatus5Spec>;
#[doc = "Register `DMASTATUS5` writer"]
pub type W = crate::W<Dmastatus5Spec>;
#[doc = "Field `CHANN_ENB` reader - Channel Enable Status"]
pub type ChannEnbR = crate::BitReader;
#[doc = "Field `CHANN_ENB` writer - Channel Enable Status"]
pub type ChannEnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANN_ACT` reader - Channel Active Status"]
pub type ChannActR = crate::BitReader;
#[doc = "Field `CHANN_ACT` writer - Channel Active Status"]
pub type ChannActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_TR_ST` reader - End of Channel Transfer Status"]
pub type EndTrStR = crate::BitReader;
#[doc = "Field `END_TR_ST` writer - End of Channel Transfer Status"]
pub type EndTrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_BF_ST` reader - End of Channel Buffer Status"]
pub type EndBfStR = crate::BitReader;
#[doc = "Field `END_BF_ST` writer - End of Channel Buffer Status"]
pub type EndBfStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LDST` reader - Descriptor Loaded Status"]
pub type DescLdstR = crate::BitReader;
#[doc = "Field `DESC_LDST` writer - Descriptor Loaded Status"]
pub type DescLdstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_COUNT` reader - Buffer Byte Count"]
pub type BuffCountR = crate::FieldReader<u16>;
#[doc = "Field `BUFF_COUNT` writer - Buffer Byte Count"]
pub type BuffCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&self) -> ChannEnbR {
        ChannEnbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&self) -> ChannActR {
        ChannActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&self) -> EndTrStR {
        EndTrStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&self) -> EndBfStR {
        EndBfStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&self) -> DescLdstR {
        DescLdstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&self) -> BuffCountR {
        BuffCountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    #[must_use]
    pub fn chann_enb(&mut self) -> ChannEnbW<Dmastatus5Spec> {
        ChannEnbW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    #[must_use]
    pub fn chann_act(&mut self) -> ChannActW<Dmastatus5Spec> {
        ChannActW::new(self, 1)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_st(&mut self) -> EndTrStW<Dmastatus5Spec> {
        EndTrStW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    #[must_use]
    pub fn end_bf_st(&mut self) -> EndBfStW<Dmastatus5Spec> {
        EndBfStW::new(self, 5)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    #[must_use]
    pub fn desc_ldst(&mut self) -> DescLdstW<Dmastatus5Spec> {
        DescLdstW::new(self, 6)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    #[must_use]
    pub fn buff_count(&mut self) -> BuffCountW<Dmastatus5Spec> {
        BuffCountW::new(self, 16)
    }
}
#[doc = "UDPHS DMA Channel Status Register (channel = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmastatus5Spec;
impl crate::RegisterSpec for Dmastatus5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmastatus5::R`](R) reader structure"]
impl crate::Readable for Dmastatus5Spec {}
#[doc = "`write(|w| ..)` method takes [`dmastatus5::W`](W) writer structure"]
impl crate::Writable for Dmastatus5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASTATUS5 to value 0"]
impl crate::Resettable for Dmastatus5Spec {
    const RESET_VALUE: u32 = 0;
}
