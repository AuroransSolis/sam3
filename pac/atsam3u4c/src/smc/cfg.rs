#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pagesize {
    #[doc = "0: Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    Ps512_16 = 0,
    #[doc = "1: Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    Ps1024_32 = 1,
    #[doc = "2: Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    Ps2048_64 = 2,
    #[doc = "3: Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    Ps4096_128 = 3,
}
impl From<Pagesize> for u8 {
    #[inline(always)]
    fn from(variant: Pagesize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pagesize {
    type Ux = u8;
}
impl crate::IsEnum for Pagesize {}
#[doc = "Field `PAGESIZE` reader - "]
pub type PagesizeR = crate::FieldReader<Pagesize>;
impl PagesizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pagesize {
        match self.bits {
            0 => Pagesize::Ps512_16,
            1 => Pagesize::Ps1024_32,
            2 => Pagesize::Ps2048_64,
            3 => Pagesize::Ps4096_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    #[inline(always)]
    pub fn is_ps512_16(&self) -> bool {
        *self == Pagesize::Ps512_16
    }
    #[doc = "Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    #[inline(always)]
    pub fn is_ps1024_32(&self) -> bool {
        *self == Pagesize::Ps1024_32
    }
    #[doc = "Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    #[inline(always)]
    pub fn is_ps2048_64(&self) -> bool {
        *self == Pagesize::Ps2048_64
    }
    #[doc = "Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    #[inline(always)]
    pub fn is_ps4096_128(&self) -> bool {
        *self == Pagesize::Ps4096_128
    }
}
#[doc = "Field `PAGESIZE` writer - "]
pub type PagesizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pagesize, crate::Safe>;
impl<'a, REG> PagesizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    #[inline(always)]
    pub fn ps512_16(self) -> &'a mut crate::W<REG> {
        self.variant(Pagesize::Ps512_16)
    }
    #[doc = "Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    #[inline(always)]
    pub fn ps1024_32(self) -> &'a mut crate::W<REG> {
        self.variant(Pagesize::Ps1024_32)
    }
    #[doc = "Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    #[inline(always)]
    pub fn ps2048_64(self) -> &'a mut crate::W<REG> {
        self.variant(Pagesize::Ps2048_64)
    }
    #[doc = "Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    #[inline(always)]
    pub fn ps4096_128(self) -> &'a mut crate::W<REG> {
        self.variant(Pagesize::Ps4096_128)
    }
}
#[doc = "Field `WSPARE` reader - Write Spare Area"]
pub type WspareR = crate::BitReader;
#[doc = "Field `WSPARE` writer - Write Spare Area"]
pub type WspareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSPARE` reader - Read Spare Area"]
pub type RspareR = crate::BitReader;
#[doc = "Field `RSPARE` writer - Read Spare Area"]
pub type RspareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGECTRL` reader - Rising/Falling Edge Detection Control"]
pub type EdgectrlR = crate::BitReader;
#[doc = "Field `EDGECTRL` writer - Rising/Falling Edge Detection Control"]
pub type EdgectrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBEDGE` reader - Ready/Busy Signal Edge Detection"]
pub type RbedgeR = crate::BitReader;
#[doc = "Field `RBEDGE` writer - Ready/Busy Signal Edge Detection"]
pub type RbedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOCYC` reader - Data Timeout Cycle Number"]
pub type DtocycR = crate::FieldReader;
#[doc = "Field `DTOCYC` writer - Data Timeout Cycle Number"]
pub type DtocycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Data Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtomul {
    #[doc = "0: DTOCYC"]
    X1 = 0,
    #[doc = "1: DTOCYC x 16"]
    X16 = 1,
    #[doc = "2: DTOCYC x 128"]
    X128 = 2,
    #[doc = "3: DTOCYC x 256"]
    X256 = 3,
    #[doc = "4: DTOCYC x 1024"]
    X1024 = 4,
    #[doc = "5: DTOCYC x 4096"]
    X4096 = 5,
    #[doc = "6: DTOCYC x 65536"]
    X65536 = 6,
    #[doc = "7: DTOCYC x 1048576"]
    X1048576 = 7,
}
impl From<Dtomul> for u8 {
    #[inline(always)]
    fn from(variant: Dtomul) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtomul {
    type Ux = u8;
}
impl crate::IsEnum for Dtomul {}
#[doc = "Field `DTOMUL` reader - Data Timeout Multiplier"]
pub type DtomulR = crate::FieldReader<Dtomul>;
impl DtomulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtomul {
        match self.bits {
            0 => Dtomul::X1,
            1 => Dtomul::X16,
            2 => Dtomul::X128,
            3 => Dtomul::X256,
            4 => Dtomul::X1024,
            5 => Dtomul::X4096,
            6 => Dtomul::X65536,
            7 => Dtomul::X1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == Dtomul::X1
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == Dtomul::X16
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == Dtomul::X128
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == Dtomul::X256
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == Dtomul::X1024
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == Dtomul::X4096
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn is_x65536(&self) -> bool {
        *self == Dtomul::X65536
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn is_x1048576(&self) -> bool {
        *self == Dtomul::X1048576
    }
}
#[doc = "Field `DTOMUL` writer - Data Timeout Multiplier"]
pub type DtomulW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtomul, crate::Safe>;
impl<'a, REG> DtomulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn x65536(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn x1048576(self) -> &'a mut crate::W<REG> {
        self.variant(Dtomul::X1048576)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pagesize(&self) -> PagesizeR {
        PagesizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    pub fn wspare(&self) -> WspareR {
        WspareR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    pub fn rspare(&self) -> RspareR {
        RspareR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    pub fn edgectrl(&self) -> EdgectrlR {
        EdgectrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    pub fn rbedge(&self) -> RbedgeR {
        RbedgeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DtocycR {
        DtocycR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DtomulR {
        DtomulR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pagesize(&mut self) -> PagesizeW<CfgSpec> {
        PagesizeW::new(self, 0)
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    #[must_use]
    pub fn wspare(&mut self) -> WspareW<CfgSpec> {
        WspareW::new(self, 8)
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    #[must_use]
    pub fn rspare(&mut self) -> RspareW<CfgSpec> {
        RspareW::new(self, 9)
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    #[must_use]
    pub fn edgectrl(&mut self) -> EdgectrlW<CfgSpec> {
        EdgectrlW::new(self, 12)
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    #[must_use]
    pub fn rbedge(&mut self) -> RbedgeW<CfgSpec> {
        RbedgeW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dtocyc(&mut self) -> DtocycW<CfgSpec> {
        DtocycW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn dtomul(&mut self) -> DtomulW<CfgSpec> {
        DtomulW::new(self, 20)
    }
}
#[doc = "SMC NFC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
