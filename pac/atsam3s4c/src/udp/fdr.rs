#[doc = "Register `FDR[%s]` reader"]
pub type R = crate::R<FDR_SPEC>;
#[doc = "Register `FDR[%s]` writer"]
pub type W = crate::W<FDR_SPEC>;
#[doc = "Field `FIFO_DATA` reader - FIFO Data Value"]
pub type FIFO_DATA_R = crate::FieldReader;
#[doc = "Field `FIFO_DATA` writer - FIFO Data Value"]
pub type FIFO_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    pub fn fifo_data(&self) -> FIFO_DATA_R {
        FIFO_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data(&mut self) -> FIFO_DATA_W<FDR_SPEC, 0> {
        FIFO_DATA_W::new(self)
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
#[doc = "Endpoint FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
