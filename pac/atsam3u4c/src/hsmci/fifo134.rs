#[doc = "Register `FIFO134` reader"]
pub type R = crate::R<FIFO134_SPEC>;
#[doc = "Register `FIFO134` writer"]
pub type W = crate::W<FIFO134_SPEC>;
#[doc = "Field `DATA` reader - Data to Read or Data to Write"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data to Read or Data to Write"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to Read or Data to Write"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to Read or Data to Write"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FIFO134_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "FIFO Memory Aperture0 134\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo134::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo134::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO134_SPEC;
impl crate::RegisterSpec for FIFO134_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo134::R`](R) reader structure"]
impl crate::Readable for FIFO134_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo134::W`](W) writer structure"]
impl crate::Writable for FIFO134_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
