#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CMDR_SPEC>;
#[doc = "Field `CMDNB` writer - Command Number"]
pub type CMDNB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Response Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPTYP_AW {
    #[doc = "0: No response"]
    Noresp = 0,
    #[doc = "1: 48-bit response"]
    _48Bit = 1,
    #[doc = "2: 136-bit response"]
    _136Bit = 2,
    #[doc = "3: R1b response type"]
    R1b = 3,
}
impl From<RSPTYP_AW> for u8 {
    #[inline(always)]
    fn from(variant: RSPTYP_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSPTYP_AW {
    type Ux = u8;
}
#[doc = "Field `RSPTYP` writer - Response Type"]
pub type RSPTYP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RSPTYP_AW>;
impl<'a, REG> RSPTYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No response"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYP_AW::Noresp)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYP_AW::_48Bit)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYP_AW::_136Bit)
    }
    #[doc = "R1b response type"]
    #[inline(always)]
    pub fn r1b(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYP_AW::R1b)
    }
}
#[doc = "Special Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPCMD_AW {
    #[doc = "0: Not a special CMD."]
    Std = 0,
    #[doc = "1: Initialization CMD: 74 clock cycles for initialization sequence."]
    Init = 1,
    #[doc = "2: Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    Sync = 2,
    #[doc = "3: CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    CeAta = 3,
    #[doc = "4: Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    ItCmd = 4,
    #[doc = "5: Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    ItResp = 5,
    #[doc = "6: Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    Bor = 6,
    #[doc = "7: End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    Ebo = 7,
}
impl From<SPCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: SPCMD_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPCMD_AW {
    type Ux = u8;
}
#[doc = "Field `SPCMD` writer - Special Command"]
pub type SPCMD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SPCMD_AW>;
impl<'a, REG> SPCMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not a special CMD."]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::Std)
    }
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::Init)
    }
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::Sync)
    }
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    #[inline(always)]
    pub fn ce_ata(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::CeAta)
    }
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::ItCmd)
    }
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_resp(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::ItResp)
    }
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    #[inline(always)]
    pub fn bor(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::Bor)
    }
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    #[inline(always)]
    pub fn ebo(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMD_AW::Ebo)
    }
}
#[doc = "Open Drain Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPDCMD_AW {
    #[doc = "0: Push pull command."]
    Pushpull = 0,
    #[doc = "1: Open drain command."]
    Opendrain = 1,
}
impl From<OPDCMD_AW> for bool {
    #[inline(always)]
    fn from(variant: OPDCMD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPDCMD` writer - Open Drain Command"]
pub type OPDCMD_W<'a, REG> = crate::BitWriter<'a, REG, OPDCMD_AW>;
impl<'a, REG> OPDCMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push pull command."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(OPDCMD_AW::Pushpull)
    }
    #[doc = "Open drain command."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut crate::W<REG> {
        self.variant(OPDCMD_AW::Opendrain)
    }
}
#[doc = "Max Latency for Command to Response"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAXLAT_AW {
    #[doc = "0: 5-cycle max latency."]
    _5 = 0,
    #[doc = "1: 64-cycle max latency."]
    _64 = 1,
}
impl From<MAXLAT_AW> for bool {
    #[inline(always)]
    fn from(variant: MAXLAT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXLAT` writer - Max Latency for Command to Response"]
pub type MAXLAT_W<'a, REG> = crate::BitWriter<'a, REG, MAXLAT_AW>;
impl<'a, REG> MAXLAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "5-cycle max latency."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(MAXLAT_AW::_5)
    }
    #[doc = "64-cycle max latency."]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(MAXLAT_AW::_64)
    }
}
#[doc = "Transfer Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRCMD_AW {
    #[doc = "0: No data transfer"]
    NoData = 0,
    #[doc = "1: Start data transfer"]
    StartData = 1,
    #[doc = "2: Stop data transfer"]
    StopData = 2,
}
impl From<TRCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: TRCMD_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRCMD_AW {
    type Ux = u8;
}
#[doc = "Field `TRCMD` writer - Transfer Command"]
pub type TRCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRCMD_AW>;
impl<'a, REG> TRCMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(TRCMD_AW::NoData)
    }
    #[doc = "Start data transfer"]
    #[inline(always)]
    pub fn start_data(self) -> &'a mut crate::W<REG> {
        self.variant(TRCMD_AW::StartData)
    }
    #[doc = "Stop data transfer"]
    #[inline(always)]
    pub fn stop_data(self) -> &'a mut crate::W<REG> {
        self.variant(TRCMD_AW::StopData)
    }
}
#[doc = "Transfer Direction"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRDIR_AW {
    #[doc = "0: Write."]
    Write = 0,
    #[doc = "1: Read."]
    Read = 1,
}
impl From<TRDIR_AW> for bool {
    #[inline(always)]
    fn from(variant: TRDIR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRDIR` writer - Transfer Direction"]
pub type TRDIR_W<'a, REG> = crate::BitWriter<'a, REG, TRDIR_AW>;
impl<'a, REG> TRDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write."]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(TRDIR_AW::Write)
    }
    #[doc = "Read."]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(TRDIR_AW::Read)
    }
}
#[doc = "Transfer Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRTYP_AW {
    #[doc = "0: MMC/SD Card Single Block"]
    Single = 0,
    #[doc = "1: MMC/SD Card Multiple Block"]
    Multiple = 1,
    #[doc = "2: MMC Stream"]
    Stream = 2,
    #[doc = "4: SDIO Byte"]
    Byte = 4,
    #[doc = "5: SDIO Block"]
    Block = 5,
}
impl From<TRTYP_AW> for u8 {
    #[inline(always)]
    fn from(variant: TRTYP_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRTYP_AW {
    type Ux = u8;
}
#[doc = "Field `TRTYP` writer - Transfer Type"]
pub type TRTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRTYP_AW>;
impl<'a, REG> TRTYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MMC/SD Card Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYP_AW::Single)
    }
    #[doc = "MMC/SD Card Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYP_AW::Multiple)
    }
    #[doc = "MMC Stream"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYP_AW::Stream)
    }
    #[doc = "SDIO Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYP_AW::Byte)
    }
    #[doc = "SDIO Block"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYP_AW::Block)
    }
}
#[doc = "SDIO Special Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOSPCMD_AW {
    #[doc = "0: Not an SDIO Special Command"]
    Std = 0,
    #[doc = "1: SDIO Suspend Command"]
    Suspend = 1,
    #[doc = "2: SDIO Resume Command"]
    Resume = 2,
}
impl From<IOSPCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: IOSPCMD_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IOSPCMD_AW {
    type Ux = u8;
}
#[doc = "Field `IOSPCMD` writer - SDIO Special Command"]
pub type IOSPCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IOSPCMD_AW>;
impl<'a, REG> IOSPCMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not an SDIO Special Command"]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(IOSPCMD_AW::Std)
    }
    #[doc = "SDIO Suspend Command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(IOSPCMD_AW::Suspend)
    }
    #[doc = "SDIO Resume Command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(IOSPCMD_AW::Resume)
    }
}
#[doc = "ATA with Command Completion Signal"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATACS_AW {
    #[doc = "0: Normal operation mode."]
    Normal = 0,
    #[doc = "1: This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    Completion = 1,
}
impl From<ATACS_AW> for bool {
    #[inline(always)]
    fn from(variant: ATACS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATACS` writer - ATA with Command Completion Signal"]
pub type ATACS_W<'a, REG> = crate::BitWriter<'a, REG, ATACS_AW>;
impl<'a, REG> ATACS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ATACS_AW::Normal)
    }
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    #[inline(always)]
    pub fn completion(self) -> &'a mut crate::W<REG> {
        self.variant(ATACS_AW::Completion)
    }
}
#[doc = "Field `BOOT_ACK` writer - Boot Operation Acknowledge"]
pub type BOOT_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:5 - Command Number"]
    #[inline(always)]
    #[must_use]
    pub fn cmdnb(&mut self) -> CMDNB_W<CMDR_SPEC> {
        CMDNB_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Response Type"]
    #[inline(always)]
    #[must_use]
    pub fn rsptyp(&mut self) -> RSPTYP_W<CMDR_SPEC> {
        RSPTYP_W::new(self, 6)
    }
    #[doc = "Bits 8:10 - Special Command"]
    #[inline(always)]
    #[must_use]
    pub fn spcmd(&mut self) -> SPCMD_W<CMDR_SPEC> {
        SPCMD_W::new(self, 8)
    }
    #[doc = "Bit 11 - Open Drain Command"]
    #[inline(always)]
    #[must_use]
    pub fn opdcmd(&mut self) -> OPDCMD_W<CMDR_SPEC> {
        OPDCMD_W::new(self, 11)
    }
    #[doc = "Bit 12 - Max Latency for Command to Response"]
    #[inline(always)]
    #[must_use]
    pub fn maxlat(&mut self) -> MAXLAT_W<CMDR_SPEC> {
        MAXLAT_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn trcmd(&mut self) -> TRCMD_W<CMDR_SPEC> {
        TRCMD_W::new(self, 16)
    }
    #[doc = "Bit 18 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn trdir(&mut self) -> TRDIR_W<CMDR_SPEC> {
        TRDIR_W::new(self, 18)
    }
    #[doc = "Bits 19:21 - Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn trtyp(&mut self) -> TRTYP_W<CMDR_SPEC> {
        TRTYP_W::new(self, 19)
    }
    #[doc = "Bits 24:25 - SDIO Special Command"]
    #[inline(always)]
    #[must_use]
    pub fn iospcmd(&mut self) -> IOSPCMD_W<CMDR_SPEC> {
        IOSPCMD_W::new(self, 24)
    }
    #[doc = "Bit 26 - ATA with Command Completion Signal"]
    #[inline(always)]
    #[must_use]
    pub fn atacs(&mut self) -> ATACS_W<CMDR_SPEC> {
        ATACS_W::new(self, 26)
    }
    #[doc = "Bit 27 - Boot Operation Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack(&mut self) -> BOOT_ACK_W<CMDR_SPEC> {
        BOOT_ACK_W::new(self, 27)
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
#[doc = "Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDR_SPEC;
impl crate::RegisterSpec for CMDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CMDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
