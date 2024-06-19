#[doc = "Register `MATRIX_SCFG8` reader"]
pub type R = crate::R<MatrixScfg8Spec>;
#[doc = "Register `MATRIX_SCFG8` writer"]
pub type W = crate::W<MatrixScfg8Spec>;
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub type SlotCycleR = crate::FieldReader;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub type SlotCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DefmstrTypeR = crate::FieldReader;
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DefmstrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub type FixedDefmstrR = crate::FieldReader;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub type FixedDefmstrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ARBT` reader - Arbitration Type"]
pub type ArbtR = crate::FieldReader;
#[doc = "Field `ARBT` writer - Arbitration Type"]
pub type ArbtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SlotCycleR {
        SlotCycleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DefmstrTypeR {
        DefmstrTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FixedDefmstrR {
        FixedDefmstrR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&self) -> ArbtR {
        ArbtR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    #[must_use]
    pub fn slot_cycle(&mut self) -> SlotCycleW<MatrixScfg8Spec> {
        SlotCycleW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    #[must_use]
    pub fn defmstr_type(&mut self) -> DefmstrTypeW<MatrixScfg8Spec> {
        DefmstrTypeW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    #[must_use]
    pub fn fixed_defmstr(&mut self) -> FixedDefmstrW<MatrixScfg8Spec> {
        FixedDefmstrW::new(self, 18)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    #[must_use]
    pub fn arbt(&mut self) -> ArbtW<MatrixScfg8Spec> {
        ArbtW::new(self, 24)
    }
}
#[doc = "Slave Configuration Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_scfg8::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_scfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixScfg8Spec;
impl crate::RegisterSpec for MatrixScfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_scfg8::R`](R) reader structure"]
impl crate::Readable for MatrixScfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`matrix_scfg8::W`](W) writer structure"]
impl crate::Writable for MatrixScfg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
