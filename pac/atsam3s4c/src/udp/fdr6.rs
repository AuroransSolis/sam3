#[doc = "Register `FDR6` reader"]
pub type R = crate::R<FDR6_SPEC>;
#[doc = "Register `FDR6` writer"]
pub type W = crate::W<FDR6_SPEC>;
#[doc = "Field `FIFO_DATA` reader - FIFO Data Value"]
pub type FIFO_DATA_R = crate::FieldReader;
#[doc = "Field `FIFO_DATA` writer - FIFO Data Value"]
pub type FIFO_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn fifo_data(&mut self) -> FIFO_DATA_W<FDR6_SPEC> {
        FIFO_DATA_W::new(self, 0)
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
#[doc = "Endpoint FIFO Data Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr6::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDR6_SPEC;
impl crate::RegisterSpec for FDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr6::R`](R) reader structure"]
impl crate::Readable for FDR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdr6::W`](W) writer structure"]
impl crate::Writable for FDR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}