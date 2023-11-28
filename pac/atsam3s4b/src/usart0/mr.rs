#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type USART_MODE_R = crate::FieldReader<USART_MODE_A>;
#[doc = "USART Mode of Operation"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART_MODE_A {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: RS485"]
    Rs485 = 1,
    #[doc = "2: Hardware Handshaking"]
    HwHandshaking = 2,
    #[doc = "3: Modem"]
    Modem = 3,
    #[doc = "4: IS07816 Protocol: T = 0"]
    Is07816T0 = 4,
    #[doc = "6: IS07816 Protocol: T = 1"]
    Is07816T1 = 6,
    #[doc = "8: IrDA"]
    Irda = 8,
    #[doc = "14: SPI Master"]
    SpiMaster = 14,
    #[doc = "15: SPI Slave"]
    SpiSlave = 15,
}
impl From<USART_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: USART_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART_MODE_A {
    type Ux = u8;
}
impl USART_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART_MODE_A> {
        match self.bits {
            0 => Some(USART_MODE_A::Normal),
            1 => Some(USART_MODE_A::Rs485),
            2 => Some(USART_MODE_A::HwHandshaking),
            3 => Some(USART_MODE_A::Modem),
            4 => Some(USART_MODE_A::Is07816T0),
            6 => Some(USART_MODE_A::Is07816T1),
            8 => Some(USART_MODE_A::Irda),
            14 => Some(USART_MODE_A::SpiMaster),
            15 => Some(USART_MODE_A::SpiSlave),
            _ => None,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == USART_MODE_A::Normal
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == USART_MODE_A::Rs485
    }
    #[doc = "Hardware Handshaking"]
    #[inline(always)]
    pub fn is_hw_handshaking(&self) -> bool {
        *self == USART_MODE_A::HwHandshaking
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == USART_MODE_A::Modem
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is_is07816_t_0(&self) -> bool {
        *self == USART_MODE_A::Is07816T0
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is_is07816_t_1(&self) -> bool {
        *self == USART_MODE_A::Is07816T1
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == USART_MODE_A::Irda
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODE_A::SpiMaster
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODE_A::SpiSlave
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type USART_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, USART_MODE_A>;
impl<'a, REG> USART_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::Normal)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::Rs485)
    }
    #[doc = "Hardware Handshaking"]
    #[inline(always)]
    pub fn hw_handshaking(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::HwHandshaking)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::Modem)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is07816_t_0(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::Is07816T0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is07816_t_1(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::Is07816T1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::Irda)
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::SpiMaster)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(USART_MODE_A::SpiSlave)
    }
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type USCLKS_R = crate::FieldReader<USCLKS_A>;
#[doc = "Clock Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USCLKS_A {
    #[doc = "0: Master Clock MCK is selected"]
    Mck = 0,
    #[doc = "1: Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    Div = 1,
    #[doc = "3: Serial Clock SLK is selected"]
    Sck = 3,
}
impl From<USCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USCLKS_A {
    type Ux = u8;
}
impl USCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USCLKS_A> {
        match self.bits {
            0 => Some(USCLKS_A::Mck),
            1 => Some(USCLKS_A::Div),
            3 => Some(USCLKS_A::Sck),
            _ => None,
        }
    }
    #[doc = "Master Clock MCK is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKS_A::Mck
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == USCLKS_A::Div
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKS_A::Sck
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type USCLKS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USCLKS_A>;
impl<'a, REG> USCLKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(USCLKS_A::Mck)
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(USCLKS_A::Div)
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut crate::W<REG> {
        self.variant(USCLKS_A::Sck)
    }
}
#[doc = "Field `CHRL` reader - Character Length."]
pub type CHRL_R = crate::FieldReader<CHRL_A>;
#[doc = "Character Length."]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHRL_A {
    #[doc = "0: Character length is 5 bits"]
    _5Bit = 0,
    #[doc = "1: Character length is 6 bits"]
    _6Bit = 1,
    #[doc = "2: Character length is 7 bits"]
    _7Bit = 2,
    #[doc = "3: Character length is 8 bits"]
    _8Bit = 3,
}
impl From<CHRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHRL_A {
    type Ux = u8;
}
impl CHRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHRL_A {
        match self.bits {
            0 => CHRL_A::_5Bit,
            1 => CHRL_A::_6Bit,
            2 => CHRL_A::_7Bit,
            3 => CHRL_A::_8Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == CHRL_A::_5Bit
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == CHRL_A::_6Bit
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == CHRL_A::_7Bit
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRL_A::_8Bit
    }
}
#[doc = "Field `CHRL` writer - Character Length."]
pub type CHRL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CHRL_A>;
impl<'a, REG> CHRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHRL_A::_5Bit)
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHRL_A::_6Bit)
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHRL_A::_7Bit)
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(CHRL_A::_8Bit)
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode Select"]
pub type SYNC_R = crate::BitReader;
#[doc = "Field `SYNC` writer - Synchronous Mode Select"]
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAR` reader - Parity Type"]
pub type PAR_R = crate::FieldReader<PAR_A>;
#[doc = "Parity Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAR_A {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
    #[doc = "2: Parity forced to 0 (Space)"]
    Space = 2,
    #[doc = "3: Parity forced to 1 (Mark)"]
    Mark = 3,
    #[doc = "4: No parity"]
    No = 4,
    #[doc = "6: Multidrop mode"]
    Multidrop = 6,
}
impl From<PAR_A> for u8 {
    #[inline(always)]
    fn from(variant: PAR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAR_A {
    type Ux = u8;
}
impl PAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PAR_A> {
        match self.bits {
            0 => Some(PAR_A::Even),
            1 => Some(PAR_A::Odd),
            2 => Some(PAR_A::Space),
            3 => Some(PAR_A::Mark),
            4 => Some(PAR_A::No),
            6 => Some(PAR_A::Multidrop),
            _ => None,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PAR_A::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PAR_A::Odd
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PAR_A::Space
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PAR_A::Mark
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PAR_A::No
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn is_multidrop(&self) -> bool {
        *self == PAR_A::Multidrop
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type PAR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PAR_A>;
impl<'a, REG> PAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Odd)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Space)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Mark)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::No)
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn multidrop(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Multidrop)
    }
}
#[doc = "Field `NBSTOP` reader - Number of Stop Bits"]
pub type NBSTOP_R = crate::FieldReader<NBSTOP_A>;
#[doc = "Number of Stop Bits"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBSTOP_A {
    #[doc = "0: 1 stop bit"]
    _1Bit = 0,
    #[doc = "1: 1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    _15Bit = 1,
    #[doc = "2: 2 stop bits"]
    _2Bit = 2,
}
impl From<NBSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: NBSTOP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NBSTOP_A {
    type Ux = u8;
}
impl NBSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NBSTOP_A> {
        match self.bits {
            0 => Some(NBSTOP_A::_1Bit),
            1 => Some(NBSTOP_A::_15Bit),
            2 => Some(NBSTOP_A::_2Bit),
            _ => None,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == NBSTOP_A::_1Bit
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn is_1_5_bit(&self) -> bool {
        *self == NBSTOP_A::_15Bit
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == NBSTOP_A::_2Bit
    }
}
#[doc = "Field `NBSTOP` writer - Number of Stop Bits"]
pub type NBSTOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NBSTOP_A>;
impl<'a, REG> NBSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut crate::W<REG> {
        self.variant(NBSTOP_A::_1Bit)
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn _1_5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(NBSTOP_A::_15Bit)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(NBSTOP_A::_2Bit)
    }
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type CHMODE_R = crate::FieldReader<CHMODE_A>;
#[doc = "Channel Mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMODE_A {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Automatic Echo. Receiver input is connected to the TXD pin."]
    Automatic = 1,
    #[doc = "2: Local Loopback. Transmitter output is connected to the Receiver Input."]
    LocalLoopback = 2,
    #[doc = "3: Remote Loopback. RXD pin is internally connected to the TXD pin."]
    RemoteLoopback = 3,
}
impl From<CHMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHMODE_A {
    type Ux = u8;
}
impl CHMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHMODE_A {
        match self.bits {
            0 => CHMODE_A::Normal,
            1 => CHMODE_A::Automatic,
            2 => CHMODE_A::LocalLoopback,
            3 => CHMODE_A::RemoteLoopback,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHMODE_A::Normal
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == CHMODE_A::Automatic
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == CHMODE_A::LocalLoopback
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == CHMODE_A::RemoteLoopback
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type CHMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CHMODE_A>;
impl<'a, REG> CHMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::Normal)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::Automatic)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::LocalLoopback)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::RemoteLoopback)
    }
}
#[doc = "Field `MSBF` reader - Bit Order"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Bit Order"]
pub type MSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE9` reader - 9-bit Character Length"]
pub type MODE9_R = crate::BitReader;
#[doc = "Field `MODE9` writer - 9-bit Character Length"]
pub type MODE9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub type CLKO_R = crate::BitReader;
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub type CLKO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER` reader - Oversampling Mode"]
pub type OVER_R = crate::BitReader;
#[doc = "Field `OVER` writer - Oversampling Mode"]
pub type OVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACK` reader - Inhibit Non Acknowledge"]
pub type INACK_R = crate::BitReader;
#[doc = "Field `INACK` writer - Inhibit Non Acknowledge"]
pub type INACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DSNACK_R = crate::BitReader;
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DSNACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAR_SYNC` reader - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VAR_SYNC_R = crate::BitReader;
#[doc = "Field `VAR_SYNC` writer - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VAR_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVDATA` reader - Inverted Data"]
pub type INVDATA_R = crate::BitReader;
#[doc = "Field `INVDATA` writer - Inverted Data"]
pub type INVDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_ITERATION` reader - Maximum Number of Automatic Iteration"]
pub type MAX_ITERATION_R = crate::FieldReader;
#[doc = "Field `MAX_ITERATION` writer - Maximum Number of Automatic Iteration"]
pub type MAX_ITERATION_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FILTER` reader - Infrared Receive Line Filter"]
pub type FILTER_R = crate::BitReader;
#[doc = "Field `FILTER` writer - Infrared Receive Line Filter"]
pub type FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAN` reader - Manchester Encoder/Decoder Enable"]
pub type MAN_R = crate::BitReader;
#[doc = "Field `MAN` writer - Manchester Encoder/Decoder Enable"]
pub type MAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSYNC` reader - Manchester Synchronization Mode"]
pub type MODSYNC_R = crate::BitReader;
#[doc = "Field `MODSYNC` writer - Manchester Synchronization Mode"]
pub type MODSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEBIT` reader - Start Frame Delimiter Selector"]
pub type ONEBIT_R = crate::BitReader;
#[doc = "Field `ONEBIT` writer - Start Frame Delimiter Selector"]
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> USART_MODE_R {
        USART_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> USCLKS_R {
        USCLKS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&self) -> NBSTOP_R {
        NBSTOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    pub fn var_sync(&self) -> VAR_SYNC_R {
        VAR_SYNC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    pub fn invdata(&self) -> INVDATA_R {
        INVDATA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    pub fn max_iteration(&self) -> MAX_ITERATION_R {
        MAX_ITERATION_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    pub fn man(&self) -> MAN_R {
        MAN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    pub fn modsync(&self) -> MODSYNC_R {
        MODSYNC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    #[must_use]
    pub fn usart_mode(&mut self) -> USART_MODE_W<MR_SPEC> {
        USART_MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usclks(&mut self) -> USCLKS_W<MR_SPEC> {
        USCLKS_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    #[must_use]
    pub fn chrl(&mut self) -> CHRL_W<MR_SPEC> {
        CHRL_W::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<MR_SPEC> {
        SYNC_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> PAR_W<MR_SPEC> {
        PAR_W::new(self, 9)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    #[must_use]
    pub fn nbstop(&mut self) -> NBSTOP_W<MR_SPEC> {
        NBSTOP_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmode(&mut self) -> CHMODE_W<MR_SPEC> {
        CHMODE_W::new(self, 14)
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<MR_SPEC> {
        MSBF_W::new(self, 16)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<MR_SPEC> {
        MODE9_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn clko(&mut self) -> CLKO_W<MR_SPEC> {
        CLKO_W::new(self, 18)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OVER_W<MR_SPEC> {
        OVER_W::new(self, 19)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn inack(&mut self) -> INACK_W<MR_SPEC> {
        INACK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    #[must_use]
    pub fn dsnack(&mut self) -> DSNACK_W<MR_SPEC> {
        DSNACK_W::new(self, 21)
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    #[must_use]
    pub fn var_sync(&mut self) -> VAR_SYNC_W<MR_SPEC> {
        VAR_SYNC_W::new(self, 22)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    #[must_use]
    pub fn invdata(&mut self) -> INVDATA_W<MR_SPEC> {
        INVDATA_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    #[must_use]
    pub fn max_iteration(&mut self) -> MAX_ITERATION_W<MR_SPEC> {
        MAX_ITERATION_W::new(self, 24)
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<MR_SPEC> {
        FILTER_W::new(self, 28)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    #[must_use]
    pub fn man(&mut self) -> MAN_W<MR_SPEC> {
        MAN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn modsync(&mut self) -> MODSYNC_W<MR_SPEC> {
        MODSYNC_W::new(self, 30)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<MR_SPEC> {
        ONEBIT_W::new(self, 31)
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
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
