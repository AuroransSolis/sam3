#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "SELection for MINUS comparator input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selminus {
    #[doc = "0: SelectTS"]
    Ts = 0,
    #[doc = "1: Select ADVREF"]
    Advref = 1,
    #[doc = "2: Select DAC0"]
    Dac0 = 2,
    #[doc = "3: Select DAC1"]
    Dac1 = 3,
    #[doc = "4: Select AD0"]
    Ad0 = 4,
    #[doc = "5: Select AD1"]
    Ad1 = 5,
    #[doc = "6: Select AD2"]
    Ad2 = 6,
    #[doc = "7: Select AD3"]
    Ad3 = 7,
}
impl From<Selminus> for u8 {
    #[inline(always)]
    fn from(variant: Selminus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selminus {
    type Ux = u8;
}
#[doc = "Field `SELMINUS` reader - SELection for MINUS comparator input"]
pub type SelminusR = crate::FieldReader<Selminus>;
impl SelminusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selminus {
        match self.bits {
            0 => Selminus::Ts,
            1 => Selminus::Advref,
            2 => Selminus::Dac0,
            3 => Selminus::Dac1,
            4 => Selminus::Ad0,
            5 => Selminus::Ad1,
            6 => Selminus::Ad2,
            7 => Selminus::Ad3,
            _ => unreachable!(),
        }
    }
    #[doc = "SelectTS"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == Selminus::Ts
    }
    #[doc = "Select ADVREF"]
    #[inline(always)]
    pub fn is_advref(&self) -> bool {
        *self == Selminus::Advref
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == Selminus::Dac0
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == Selminus::Dac1
    }
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == Selminus::Ad0
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn is_ad1(&self) -> bool {
        *self == Selminus::Ad1
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        *self == Selminus::Ad2
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn is_ad3(&self) -> bool {
        *self == Selminus::Ad3
    }
}
#[doc = "Field `SELMINUS` writer - SELection for MINUS comparator input"]
pub type SelminusW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Selminus>;
impl<'a, REG> SelminusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SelectTS"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Ts)
    }
    #[doc = "Select ADVREF"]
    #[inline(always)]
    pub fn advref(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Advref)
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Dac0)
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Dac1)
    }
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Ad0)
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn ad1(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Ad1)
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Ad2)
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn ad3(self) -> &'a mut crate::W<REG> {
        self.variant(Selminus::Ad3)
    }
}
#[doc = "SELection for PLUS comparator input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selplus {
    #[doc = "0: Select AD0"]
    Ad0 = 0,
    #[doc = "1: Select AD1"]
    Ad1 = 1,
    #[doc = "2: Select AD2"]
    Ad2 = 2,
    #[doc = "3: Select AD3"]
    Ad3 = 3,
    #[doc = "4: Select AD4"]
    Ad4 = 4,
    #[doc = "5: Select AD5"]
    Ad5 = 5,
    #[doc = "6: Select AD6"]
    Ad6 = 6,
    #[doc = "7: Select AD7"]
    Ad7 = 7,
}
impl From<Selplus> for u8 {
    #[inline(always)]
    fn from(variant: Selplus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selplus {
    type Ux = u8;
}
#[doc = "Field `SELPLUS` reader - SELection for PLUS comparator input"]
pub type SelplusR = crate::FieldReader<Selplus>;
impl SelplusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selplus {
        match self.bits {
            0 => Selplus::Ad0,
            1 => Selplus::Ad1,
            2 => Selplus::Ad2,
            3 => Selplus::Ad3,
            4 => Selplus::Ad4,
            5 => Selplus::Ad5,
            6 => Selplus::Ad6,
            7 => Selplus::Ad7,
            _ => unreachable!(),
        }
    }
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == Selplus::Ad0
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn is_ad1(&self) -> bool {
        *self == Selplus::Ad1
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        *self == Selplus::Ad2
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn is_ad3(&self) -> bool {
        *self == Selplus::Ad3
    }
    #[doc = "Select AD4"]
    #[inline(always)]
    pub fn is_ad4(&self) -> bool {
        *self == Selplus::Ad4
    }
    #[doc = "Select AD5"]
    #[inline(always)]
    pub fn is_ad5(&self) -> bool {
        *self == Selplus::Ad5
    }
    #[doc = "Select AD6"]
    #[inline(always)]
    pub fn is_ad6(&self) -> bool {
        *self == Selplus::Ad6
    }
    #[doc = "Select AD7"]
    #[inline(always)]
    pub fn is_ad7(&self) -> bool {
        *self == Selplus::Ad7
    }
}
#[doc = "Field `SELPLUS` writer - SELection for PLUS comparator input"]
pub type SelplusW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Selplus>;
impl<'a, REG> SelplusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad0)
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn ad1(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad1)
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad2)
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn ad3(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad3)
    }
    #[doc = "Select AD4"]
    #[inline(always)]
    pub fn ad4(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad4)
    }
    #[doc = "Select AD5"]
    #[inline(always)]
    pub fn ad5(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad5)
    }
    #[doc = "Select AD6"]
    #[inline(always)]
    pub fn ad6(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad6)
    }
    #[doc = "Select AD7"]
    #[inline(always)]
    pub fn ad7(self) -> &'a mut crate::W<REG> {
        self.variant(Selplus::Ad7)
    }
}
#[doc = "Analog Comparator ENable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acen {
    #[doc = "0: Analog Comparator Disabled."]
    Dis = 0,
    #[doc = "1: Analog Comparator Enabled."]
    En = 1,
}
impl From<Acen> for bool {
    #[inline(always)]
    fn from(variant: Acen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACEN` reader - Analog Comparator ENable"]
pub type AcenR = crate::BitReader<Acen>;
impl AcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acen {
        match self.bits {
            false => Acen::Dis,
            true => Acen::En,
        }
    }
    #[doc = "Analog Comparator Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Acen::Dis
    }
    #[doc = "Analog Comparator Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Acen::En
    }
}
#[doc = "Field `ACEN` writer - Analog Comparator ENable"]
pub type AcenW<'a, REG> = crate::BitWriter<'a, REG, Acen>;
impl<'a, REG> AcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog Comparator Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Acen::Dis)
    }
    #[doc = "Analog Comparator Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Acen::En)
    }
}
#[doc = "EDGE TYPe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edgetyp {
    #[doc = "0: only rising edge of comparator output"]
    Rising = 0,
    #[doc = "1: falling edge of comparator output"]
    Falling = 1,
    #[doc = "2: any edge of comparator output"]
    Any = 2,
}
impl From<Edgetyp> for u8 {
    #[inline(always)]
    fn from(variant: Edgetyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edgetyp {
    type Ux = u8;
}
#[doc = "Field `EDGETYP` reader - EDGE TYPe"]
pub type EdgetypR = crate::FieldReader<Edgetyp>;
impl EdgetypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Edgetyp> {
        match self.bits {
            0 => Some(Edgetyp::Rising),
            1 => Some(Edgetyp::Falling),
            2 => Some(Edgetyp::Any),
            _ => None,
        }
    }
    #[doc = "only rising edge of comparator output"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Edgetyp::Rising
    }
    #[doc = "falling edge of comparator output"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Edgetyp::Falling
    }
    #[doc = "any edge of comparator output"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == Edgetyp::Any
    }
}
#[doc = "Field `EDGETYP` writer - EDGE TYPe"]
pub type EdgetypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edgetyp>;
impl<'a, REG> EdgetypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "only rising edge of comparator output"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Edgetyp::Rising)
    }
    #[doc = "falling edge of comparator output"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Edgetyp::Falling)
    }
    #[doc = "any edge of comparator output"]
    #[inline(always)]
    pub fn any(self) -> &'a mut crate::W<REG> {
        self.variant(Edgetyp::Any)
    }
}
#[doc = "INVert comparator output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Analog Comparator output is directly processed."]
    Dis = 0,
    #[doc = "1: Analog Comparator output is inverted prior to being processed."]
    En = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - INVert comparator output"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::Dis,
            true => Inv::En,
        }
    }
    #[doc = "Analog Comparator output is directly processed."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Inv::Dis
    }
    #[doc = "Analog Comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Inv::En
    }
}
#[doc = "Field `INV` writer - INVert comparator output"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog Comparator output is directly processed."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::Dis)
    }
    #[doc = "Analog Comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::En)
    }
}
#[doc = "SELection of Fault Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selfs {
    #[doc = "0: the CF flag is used to drive the FAULT output."]
    Cf = 0,
    #[doc = "1: the output of the Analog Comparator flag is used to drive the FAULT output."]
    Output = 1,
}
impl From<Selfs> for bool {
    #[inline(always)]
    fn from(variant: Selfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELFS` reader - SELection of Fault Source"]
pub type SelfsR = crate::BitReader<Selfs>;
impl SelfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selfs {
        match self.bits {
            false => Selfs::Cf,
            true => Selfs::Output,
        }
    }
    #[doc = "the CF flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn is_cf(&self) -> bool {
        *self == Selfs::Cf
    }
    #[doc = "the output of the Analog Comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Selfs::Output
    }
}
#[doc = "Field `SELFS` writer - SELection of Fault Source"]
pub type SelfsW<'a, REG> = crate::BitWriter<'a, REG, Selfs>;
impl<'a, REG> SelfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the CF flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn cf(self) -> &'a mut crate::W<REG> {
        self.variant(Selfs::Cf)
    }
    #[doc = "the output of the Analog Comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Selfs::Output)
    }
}
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: the FAULT output is tied to 0."]
    Dis = 0,
    #[doc = "1: the FAULT output is driven by the signal defined by SELFS."]
    En = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Fault Enable"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::Dis,
            true => Fe::En,
        }
    }
    #[doc = "the FAULT output is tied to 0."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Fe::Dis
    }
    #[doc = "the FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Fe::En
    }
}
#[doc = "Field `FE` writer - Fault Enable"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG, Fe>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the FAULT output is tied to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::Dis)
    }
    #[doc = "the FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::En)
    }
}
impl R {
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline(always)]
    pub fn selminus(&self) -> SelminusR {
        SelminusR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline(always)]
    pub fn selplus(&self) -> SelplusR {
        SelplusR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline(always)]
    pub fn acen(&self) -> AcenR {
        AcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline(always)]
    pub fn edgetyp(&self) -> EdgetypR {
        EdgetypR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline(always)]
    pub fn selfs(&self) -> SelfsR {
        SelfsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline(always)]
    #[must_use]
    pub fn selminus(&mut self) -> SelminusW<MrSpec> {
        SelminusW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline(always)]
    #[must_use]
    pub fn selplus(&mut self) -> SelplusW<MrSpec> {
        SelplusW::new(self, 4)
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline(always)]
    #[must_use]
    pub fn acen(&mut self) -> AcenW<MrSpec> {
        AcenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline(always)]
    #[must_use]
    pub fn edgetyp(&mut self) -> EdgetypW<MrSpec> {
        EdgetypW::new(self, 9)
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<MrSpec> {
        InvW::new(self, 12)
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline(always)]
    #[must_use]
    pub fn selfs(&mut self) -> SelfsW<MrSpec> {
        SelfsW::new(self, 13)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<MrSpec> {
        FeW::new(self, 14)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
