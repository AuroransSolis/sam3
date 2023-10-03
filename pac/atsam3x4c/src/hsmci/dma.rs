#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `OFFSET` reader - DMA Write Buffer Offset"]
pub type OFFSET_R = crate::FieldReader;
#[doc = "Field `OFFSET` writer - DMA Write Buffer Offset"]
pub type OFFSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CHKSIZE` reader - DMA Channel Read and Write Chunk Size"]
pub type CHKSIZE_R = crate::BitReader<CHKSIZE_A>;
#[doc = "DMA Channel Read and Write Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHKSIZE_A {
    #[doc = "0: 1 data available"]
    _1 = 0,
    #[doc = "1: 4 data available"]
    _4 = 1,
}
impl From<CHKSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: CHKSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl CHKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHKSIZE_A {
        match self.bits {
            false => CHKSIZE_A::_1,
            true => CHKSIZE_A::_4,
        }
    }
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHKSIZE_A::_1
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == CHKSIZE_A::_4
    }
}
#[doc = "Field `CHKSIZE` writer - DMA Channel Read and Write Chunk Size"]
pub type CHKSIZE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHKSIZE_A>;
impl<'a, REG, const O: u8> CHKSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CHKSIZE_A::_1)
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(CHKSIZE_A::_4)
    }
}
#[doc = "Field `DMAEN` reader - DMA Hardware Handshaking Enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Hardware Handshaking Enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROPT` reader - Read Optimization with padding"]
pub type ROPT_R = crate::BitReader;
#[doc = "Field `ROPT` writer - Read Optimization with padding"]
pub type ROPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - DMA Write Buffer Offset"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> CHKSIZE_R {
        CHKSIZE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Read Optimization with padding"]
    #[inline(always)]
    pub fn ropt(&self) -> ROPT_R {
        ROPT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA Write Buffer Offset"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<DMA_SPEC, 0> {
        OFFSET_W::new(self)
    }
    #[doc = "Bit 4 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    #[must_use]
    pub fn chksize(&mut self) -> CHKSIZE_W<DMA_SPEC, 4> {
        CHKSIZE_W::new(self)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DMA_SPEC, 8> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 12 - Read Optimization with padding"]
    #[inline(always)]
    #[must_use]
    pub fn ropt(&mut self) -> ROPT_W<DMA_SPEC, 12> {
        ROPT_W::new(self)
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
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
