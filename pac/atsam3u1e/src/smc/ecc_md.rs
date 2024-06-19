#[doc = "Register `ECC_MD` reader"]
pub type R = crate::R<EccMdSpec>;
#[doc = "Register `ECC_MD` writer"]
pub type W = crate::W<EccMdSpec>;
#[doc = "ECC Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EccPagesize {
    #[doc = "0: Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    Ps512_16 = 0,
    #[doc = "1: Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    Ps1024_32 = 1,
    #[doc = "2: Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    Ps2048_64 = 2,
    #[doc = "3: Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    Ps4096_128 = 3,
}
impl From<EccPagesize> for u8 {
    #[inline(always)]
    fn from(variant: EccPagesize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EccPagesize {
    type Ux = u8;
}
impl crate::IsEnum for EccPagesize {}
#[doc = "Field `ECC_PAGESIZE` reader - ECC Page Size"]
pub type EccPagesizeR = crate::FieldReader<EccPagesize>;
impl EccPagesizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccPagesize {
        match self.bits {
            0 => EccPagesize::Ps512_16,
            1 => EccPagesize::Ps1024_32,
            2 => EccPagesize::Ps2048_64,
            3 => EccPagesize::Ps4096_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    #[inline(always)]
    pub fn is_ps512_16(&self) -> bool {
        *self == EccPagesize::Ps512_16
    }
    #[doc = "Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    #[inline(always)]
    pub fn is_ps1024_32(&self) -> bool {
        *self == EccPagesize::Ps1024_32
    }
    #[doc = "Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    #[inline(always)]
    pub fn is_ps2048_64(&self) -> bool {
        *self == EccPagesize::Ps2048_64
    }
    #[doc = "Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    #[inline(always)]
    pub fn is_ps4096_128(&self) -> bool {
        *self == EccPagesize::Ps4096_128
    }
}
#[doc = "Field `ECC_PAGESIZE` writer - ECC Page Size"]
pub type EccPagesizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, EccPagesize, crate::Safe>;
impl<'a, REG> EccPagesizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    #[inline(always)]
    pub fn ps512_16(self) -> &'a mut crate::W<REG> {
        self.variant(EccPagesize::Ps512_16)
    }
    #[doc = "Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    #[inline(always)]
    pub fn ps1024_32(self) -> &'a mut crate::W<REG> {
        self.variant(EccPagesize::Ps1024_32)
    }
    #[doc = "Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    #[inline(always)]
    pub fn ps2048_64(self) -> &'a mut crate::W<REG> {
        self.variant(EccPagesize::Ps2048_64)
    }
    #[doc = "Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    #[inline(always)]
    pub fn ps4096_128(self) -> &'a mut crate::W<REG> {
        self.variant(EccPagesize::Ps4096_128)
    }
}
#[doc = "Type of Correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Typcorrec {
    #[doc = "0: 1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    Cpage = 0,
    #[doc = "1: 1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C256b = 1,
    #[doc = "2: 1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C512b = 2,
}
impl From<Typcorrec> for u8 {
    #[inline(always)]
    fn from(variant: Typcorrec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Typcorrec {
    type Ux = u8;
}
impl crate::IsEnum for Typcorrec {}
#[doc = "Field `TYPCORREC` reader - Type of Correction"]
pub type TypcorrecR = crate::FieldReader<Typcorrec>;
impl TypcorrecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Typcorrec> {
        match self.bits {
            0 => Some(Typcorrec::Cpage),
            1 => Some(Typcorrec::C256b),
            2 => Some(Typcorrec::C512b),
            _ => None,
        }
    }
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    #[inline(always)]
    pub fn is_cpage(&self) -> bool {
        *self == Typcorrec::Cpage
    }
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn is_c256b(&self) -> bool {
        *self == Typcorrec::C256b
    }
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn is_c512b(&self) -> bool {
        *self == Typcorrec::C512b
    }
}
#[doc = "Field `TYPCORREC` writer - Type of Correction"]
pub type TypcorrecW<'a, REG> = crate::FieldWriter<'a, REG, 2, Typcorrec>;
impl<'a, REG> TypcorrecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    #[inline(always)]
    pub fn cpage(self) -> &'a mut crate::W<REG> {
        self.variant(Typcorrec::Cpage)
    }
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c256b(self) -> &'a mut crate::W<REG> {
        self.variant(Typcorrec::C256b)
    }
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c512b(self) -> &'a mut crate::W<REG> {
        self.variant(Typcorrec::C512b)
    }
}
impl R {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    pub fn ecc_pagesize(&self) -> EccPagesizeR {
        EccPagesizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    pub fn typcorrec(&self) -> TypcorrecR {
        TypcorrecR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_pagesize(&mut self) -> EccPagesizeW<EccMdSpec> {
        EccPagesizeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    #[must_use]
    pub fn typcorrec(&mut self) -> TypcorrecW<EccMdSpec> {
        TypcorrecW::new(self, 4)
    }
}
#[doc = "SMC ECC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_md::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_md::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccMdSpec;
impl crate::RegisterSpec for EccMdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_md::R`](R) reader structure"]
impl crate::Readable for EccMdSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_md::W`](W) writer structure"]
impl crate::Writable for EccMdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_MD to value 0"]
impl crate::Resettable for EccMdSpec {
    const RESET_VALUE: u32 = 0;
}
