#[doc = "Register `MAN` reader"]
pub type R = crate::R<ManSpec>;
#[doc = "Register `MAN` writer"]
pub type W = crate::W<ManSpec>;
#[doc = "Field `DATA` reader - "]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - "]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CODE` reader - "]
pub type CodeR = crate::FieldReader;
#[doc = "Field `CODE` writer - "]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGA` reader - Register Address"]
pub type RegaR = crate::FieldReader;
#[doc = "Field `REGA` writer - Register Address"]
pub type RegaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHYA` reader - PHY Address"]
pub type PhyaR = crate::FieldReader;
#[doc = "Field `PHYA` writer - PHY Address"]
pub type PhyaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RW` reader - Read-write"]
pub type RwR = crate::FieldReader;
#[doc = "Field `RW` writer - Read-write"]
pub type RwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SofR = crate::FieldReader;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SofW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&self) -> RegaR {
        RegaR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&self) -> PhyaR {
        PhyaR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Read-write"]
    #[inline(always)]
    pub fn rw(&self) -> RwR {
        RwR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<ManSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CodeW<ManSpec> {
        CodeW::new(self, 16)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn rega(&mut self) -> RegaW<ManSpec> {
        RegaW::new(self, 18)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn phya(&mut self) -> PhyaW<ManSpec> {
        PhyaW::new(self, 23)
    }
    #[doc = "Bits 28:29 - Read-write"]
    #[inline(always)]
    #[must_use]
    pub fn rw(&mut self) -> RwW<ManSpec> {
        RwW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<ManSpec> {
        SofW::new(self, 30)
    }
}
#[doc = "Phy Maintenance Register\n\nYou can [`read`](crate::Reg::read) this register and get [`man::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`man::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ManSpec;
impl crate::RegisterSpec for ManSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`man::R`](R) reader structure"]
impl crate::Readable for ManSpec {}
#[doc = "`write(|w| ..)` method takes [`man::W`](W) writer structure"]
impl crate::Writable for ManSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAN to value 0"]
impl crate::Resettable for ManSpec {
    const RESET_VALUE: u32 = 0;
}
